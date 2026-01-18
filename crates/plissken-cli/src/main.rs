//! plissken CLI - Documentation for the Rust-Python bridge

use anyhow::{Context, Result};
use clap::Parser;
use plissken_core::{
    Config, CrossRef, DocModel, ModuleRenderer, ProjectMetadata, PythonModule, Renderer, RustModule,
    build_cross_refs, synthesize_python_from_rust, synthesize_python_modules_from_rust,
    CARGO_MANIFEST, PYPROJECT_MANIFEST, PLISSKEN_CONFIG, TEMPLATE_MDBOOK, TEMPLATE_MKDOCS_MATERIAL,
    VERSION_SOURCE_CARGO, VERSION_SOURCE_PYPROJECT, DEFAULT_OUTPUT_FORMAT, DEFAULT_OUTPUT_PATH,
    DEFAULT_CRATES,
};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "plissken")]
#[command(about = "Documentation generator for Rust-Python hybrid projects")]
struct Cli {
    /// Increase output verbosity (-v for progress, -vv for debug)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

/// Print a message if verbosity level is met
macro_rules! verbose {
    ($level:expr, $verbosity:expr, $($arg:tt)*) => {
        if $verbosity >= $level {
            eprintln!($($arg)*);
        }
    };
}

/// Emit a structured warning for parser errors.
///
/// Parser errors are non-fatal by design - we continue processing other files.
/// This provides consistent formatting for all parser warnings across Python and Rust.
fn warn_parse_error(file_type: &str, file: &Path, error: &dyn std::fmt::Display) {
    eprintln!("warning: failed to parse {} file", file_type);
    eprintln!("  --> {}", file.display());
    eprintln!("  {}", error);
}

/// User-facing error with optional hint for recovery.
///
/// # Error Handling Patterns
///
/// This codebase uses three error handling patterns:
///
/// 1. **`CliError`** - For user-facing errors where a recovery hint helps.
///    Use when the user can take action to fix the problem (wrong config,
///    missing file, invalid arguments).
///    ```ignore
///    .map_err(|e| CliError::new("message").with_hint("try this"))?;
///    ```
///
/// 2. **`anyhow::with_context`** - For I/O and internal errors.
///    Use when the error is unexpected or the context alone is sufficient.
///    ```ignore
///    std::fs::write(&path, content).with_context(|| format!("..."))?;
///    ```
///
/// 3. **`warn_parse_error()`** - For non-fatal parser warnings.
///    Parser errors don't stop processing; we continue with other files.
///    ```ignore
///    warn_parse_error("Rust", &path, &error);
///    ```
struct CliError {
    message: String,
    hint: Option<String>,
    context: Option<String>,
}

impl CliError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            hint: None,
            context: None,
        }
    }

    fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(ctx) = &self.context {
            write!(f, "\n  --> {}", ctx)?;
        }
        if let Some(hint) = &self.hint {
            write!(f, "\nhint: {}", hint)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for CliError {}

#[derive(clap::Subcommand)]
enum Commands {
    /// Generate documentation model as JSON
    Generate {
        /// Path to plissken.toml or project root
        #[arg(default_value = ".")]
        path: String,

        /// Output file path (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Pretty-print JSON output
        #[arg(long)]
        pretty: bool,
    },
    /// Render documentation to Markdown files
    Render {
        /// Path to plissken.toml or project root
        #[arg(default_value = ".")]
        path: String,

        /// Output directory (overrides config)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Theme template (e.g., "mkdocs-material", "mdbook")
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Initialize a new plissken.toml configuration file
    Init {
        /// Overwrite existing plissken.toml
        #[arg(long)]
        force: bool,
    },
    /// Validate configuration without generating documentation
    Check {
        /// Path to plissken.toml or project root
        #[arg(default_value = ".")]
        path: String,

        /// Output format: text, json
        #[arg(long, default_value = "text")]
        format: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verbosity = cli.verbose;

    match cli.command {
        Commands::Generate {
            path,
            output,
            pretty,
        } => generate(&path, output, pretty, verbosity),
        Commands::Render {
            path,
            output,
            template,
        } => render(&path, output, template, verbosity),
        Commands::Init { force } => init(force, verbosity),
        Commands::Check { path, format } => check(&path, &format, verbosity),
    }
}

/// Generate documentation model from a project
fn generate(path: &str, output: Option<PathBuf>, pretty: bool, verbosity: u8) -> Result<()> {
    let project_path = Path::new(path);

    // Find and load config
    let config_path = find_config(project_path)?;
    let project_root = config_path
        .parent()
        .ok_or_else(|| CliError::new("Config path has no parent directory"))?;

    verbose!(1, verbosity, "Loading config from: {}", config_path.display());
    let config = load_config(&config_path)?;

    // Parse Rust sources
    let rust_modules = parse_rust_sources(&config, project_root, verbosity)?;
    verbose!(1, verbosity, "Parsed {} Rust module(s)", rust_modules.len());

    // Parse Python sources
    let python_modules = parse_python_sources(&config, project_root, verbosity)?;
    verbose!(1, verbosity, "Parsed {} Python module(s)", python_modules.len());

    // Build cross-references (synthesizing Python bindings if needed)
    let (python_modules, cross_refs) = synthesize_python_if_needed(&config, python_modules, &rust_modules);

    verbose!(1, verbosity, "Built {} cross-reference(s)", cross_refs.len());

    // Build the doc model
    let model = DocModel {
        metadata: ProjectMetadata {
            name: config.project.name.clone(),
            version: get_project_version(&config, project_root),
            description: None,
            git_ref: get_git_ref(project_root),
            git_commit: get_git_commit(project_root),
            generated_at: chrono_lite_now(),
        },
        rust_modules,
        python_modules,
        cross_refs,
    };

    // Output JSON
    let json = if pretty {
        serde_json::to_string_pretty(&model)?
    } else {
        serde_json::to_string(&model)?
    };

    if let Some(output_path) = output {
        std::fs::write(&output_path, &json)
            .with_context(|| format!("Failed to write to {}", output_path.display()))?;
        verbose!(1, verbosity, "Wrote output to: {}", output_path.display());
    } else {
        println!("{}", json);
    }

    Ok(())
}

/// Render documentation to Markdown files
fn render(
    path: &str,
    output_override: Option<PathBuf>,
    template_override: Option<String>,
    verbosity: u8,
) -> Result<()> {
    // Load configuration
    let (config, project_root) = load_project_config(path, verbosity)?;

    // Resolve output settings
    let output_dir = resolve_output_directory(&config, &project_root, output_override);
    let template = template_override.or_else(|| config.output.template.clone());
    log_output_settings(&output_dir, template.as_deref(), verbosity);

    // Parse and merge modules
    let (python_modules, rust_modules, cross_refs) = parse_and_merge_modules(
        &config,
        &project_root,
        verbosity,
    )?;

    // Create renderer with theme and cross-references
    let renderer = create_renderer(template.as_deref(), &project_root)?;
    let module_renderer = ModuleRenderer::with_cross_refs(&renderer, cross_refs);

    // Prepare output directory
    create_output_directory(&output_dir)?;
    let content_dir = resolve_content_directory(&output_dir, template.as_deref());

    // Write output
    let files_written = write_rendered_pages(
        &module_renderer,
        &python_modules,
        &rust_modules,
        &content_dir,
        verbosity,
    )?;

    let ssg_files = generate_ssg_files(
        &module_renderer,
        &python_modules,
        &rust_modules,
        &config,
        &output_dir,
        template.as_deref(),
        verbosity,
    )?;

    verbose!(1, verbosity, "\nRendered {} file(s) to {}", files_written + ssg_files, output_dir.display());
    Ok(())
}

/// Load project configuration from path
fn load_project_config(path: &str, verbosity: u8) -> Result<(Config, PathBuf)> {
    let project_path = Path::new(path);
    let config_path = find_config(project_path)?;
    let project_root = config_path
        .parent()
        .ok_or_else(|| CliError::new("Config path has no parent directory"))?
        .to_path_buf();

    verbose!(1, verbosity, "Loading config from: {}", config_path.display());
    let config = load_config(&config_path)?;

    Ok((config, project_root))
}

/// Resolve output directory from config and command-line override
fn resolve_output_directory(
    config: &Config,
    project_root: &Path,
    output_override: Option<PathBuf>,
) -> PathBuf {
    output_override
        .map(|p| {
            if p.is_relative() {
                project_root.join(p)
            } else {
                p
            }
        })
        .unwrap_or_else(|| project_root.join(&config.output.path))
}

/// Log output settings at appropriate verbosity level
fn log_output_settings(output_dir: &Path, template: Option<&str>, verbosity: u8) {
    verbose!(1, verbosity, "Output directory: {}", output_dir.display());
    if let Some(t) = template {
        verbose!(1, verbosity, "Using template: {}", t);
    }
}

/// Parse sources and merge synthesized modules
fn parse_and_merge_modules(
    config: &Config,
    project_root: &Path,
    verbosity: u8,
) -> Result<(Vec<PythonModule>, Vec<RustModule>, Vec<CrossRef>)> {
    // Parse sources
    let rust_modules = parse_rust_sources(config, project_root, verbosity)?;
    verbose!(1, verbosity, "Parsed {} Rust module(s)", rust_modules.len());

    let mut python_modules = parse_python_sources(config, project_root, verbosity)?;
    verbose!(1, verbosity, "Parsed {} Python module(s)", python_modules.len());

    // Normalize Python module paths
    for module in &mut python_modules {
        module.path = normalize_python_module_path(&module.path, config, project_root);
    }

    // Merge synthesized Python modules
    let (python_modules, initial_cross_refs) = merge_synthesized_python_modules(
        config,
        python_modules,
        &rust_modules,
        verbosity,
    );

    // Build cross-references
    let (python_modules, cross_refs) = build_cross_references(
        config,
        python_modules,
        &rust_modules,
        initial_cross_refs,
    );
    verbose!(1, verbosity, "Built {} cross-reference(s)", cross_refs.len());

    Ok((python_modules, rust_modules, cross_refs))
}

/// Create renderer with theme
fn create_renderer(template: Option<&str>, project_root: &Path) -> Result<Renderer> {
    Renderer::new(template, Some(project_root)).map_err(|e| {
        CliError::new(format!("failed to create renderer: {}", e))
            .with_hint(format!(
                "valid templates are '{}' and '{}'",
                TEMPLATE_MKDOCS_MATERIAL, TEMPLATE_MDBOOK
            ))
            .into()
    })
}

/// Create output directory with helpful error messages
fn create_output_directory(output_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(output_dir).map_err(|e| {
        CliError::new(format!(
            "failed to create output directory: {}",
            output_dir.display()
        ))
        .with_context(e.to_string())
        .with_hint("check that you have write permissions to this location")
    })?;
    Ok(())
}

/// Determine content directory based on SSG type
fn resolve_content_directory(output_dir: &Path, template: Option<&str>) -> PathBuf {
    if template == Some(TEMPLATE_MDBOOK) {
        output_dir.join("src")
    } else {
        output_dir.to_path_buf()
    }
}

/// Merge synthesized Python modules from Rust PyO3 bindings into existing modules
fn merge_synthesized_python_modules(
    config: &Config,
    mut python_modules: Vec<PythonModule>,
    rust_modules: &[RustModule],
    verbosity: u8,
) -> (Vec<PythonModule>, Vec<CrossRef>) {
    // Synthesize Python modules from Rust PyO3 bindings
    // Map Rust crate namespace to Python package namespace
    let python_package = config
        .python
        .as_ref()
        .map(|p| p.package.clone())
        .unwrap_or_else(|| config.project.name.clone());
    let rust_entry_point = config
        .rust
        .as_ref()
        .and_then(|r| r.entry_point.clone())
        .unwrap_or_else(|| config.project.name.clone());
    let synth_results = synthesize_python_modules_from_rust(rust_modules, &python_package, &rust_entry_point);
    let mut all_cross_refs: Vec<CrossRef> = Vec::new();

    // Add synthesized Python modules (merging with existing ones)
    for (synth_module, synth_refs) in synth_results {
        // Check if we already have a Python module with this path
        let existing = python_modules.iter_mut().find(|m| m.path == synth_module.path);
        if let Some(existing_module) = existing {
            // Merge: add synthesized items to existing module
            for synth_item in synth_module.items {
                let item_name = match &synth_item {
                    plissken_core::PythonItem::Class(c) => c.name.clone(),
                    plissken_core::PythonItem::Function(f) => f.name.clone(),
                    plissken_core::PythonItem::Variable(v) => v.name.clone(),
                };

                // Check if item already exists
                let exists = existing_module.items.iter().any(|item| {
                    match (item, &synth_item) {
                        (plissken_core::PythonItem::Class(a), plissken_core::PythonItem::Class(b)) => a.name == b.name,
                        (plissken_core::PythonItem::Function(a), plissken_core::PythonItem::Function(b)) => a.name == b.name,
                        (plissken_core::PythonItem::Variable(a), plissken_core::PythonItem::Variable(b)) => a.name == b.name,
                        _ => false,
                    }
                });

                if !exists {
                    verbose!(2, verbosity, "  Merging synthesized {} into {}", item_name, existing_module.path);
                    existing_module.items.push(synth_item);
                }
            }
        } else {
            // No existing module - add the synthesized one
            verbose!(2, verbosity, "  Synthesized Python module: {} (from Rust bindings)", synth_module.path);
            python_modules.push(synth_module);
        }
        all_cross_refs.extend(synth_refs);
    }

    (python_modules, all_cross_refs)
}

/// Synthesize Python bindings from Rust if needed, or build cross-references
///
/// This handles the case where a hybrid project has Rust code with PyO3 bindings
/// but no Python source files. In that case, we synthesize Python modules from
/// the Rust types.
fn synthesize_python_if_needed(
    config: &Config,
    python_modules: Vec<PythonModule>,
    rust_modules: &[RustModule],
) -> (Vec<PythonModule>, Vec<CrossRef>) {
    // Only synthesize Python bindings if [python] section exists in config (hybrid projects)
    if python_modules.is_empty() && !rust_modules.is_empty() && config.python.is_some() {
        // No Python sources but python config exists - synthesize bindings from Rust
        let module_name = config
            .python
            .as_ref()
            .map(|p| p.package.clone())
            .unwrap_or_else(|| config.project.name.clone());
        let (synth_module, synth_refs) = synthesize_python_from_rust(rust_modules, &module_name);
        (vec![synth_module], synth_refs)
    } else {
        build_cross_refs(config, rust_modules, python_modules)
    }
}

/// Build cross-references, synthesizing Python bindings if needed
fn build_cross_references(
    config: &Config,
    python_modules: Vec<PythonModule>,
    rust_modules: &[RustModule],
    initial_cross_refs: Vec<CrossRef>,
) -> (Vec<PythonModule>, Vec<CrossRef>) {
    let (python_modules, mut cross_refs) = synthesize_python_if_needed(config, python_modules, rust_modules);
    cross_refs.extend(initial_cross_refs);
    (python_modules, cross_refs)
}

/// Write rendered module pages to disk
fn write_rendered_pages(
    module_renderer: &ModuleRenderer,
    python_modules: &[PythonModule],
    rust_modules: &[RustModule],
    content_dir: &Path,
    verbosity: u8,
) -> Result<usize> {
    let mut files_written = 0;

    // Render Python modules
    for module in python_modules {
        let pages = module_renderer.render_python_module(module).map_err(|e| {
            CliError::new(format!("failed to render Python module '{}'", module.path))
                .with_context(e.to_string())
                .with_hint("this may indicate a bug in plissken - please report it")
        })?;

        for page in pages {
            let output_path = content_dir.join(&page.path);

            // Create parent directories if needed
            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
            }

            std::fs::write(&output_path, &page.content)
                .with_context(|| format!("Failed to write: {}", output_path.display()))?;

            verbose!(2, verbosity, "  Wrote: {}", output_path.display());
            files_written += 1;
        }
    }

    // Render Rust modules
    for module in rust_modules {
        let pages = module_renderer.render_rust_module(module).map_err(|e| {
            CliError::new(format!("failed to render Rust module '{}'", module.path))
                .with_context(e.to_string())
                .with_hint("this may indicate a bug in plissken - please report it")
        })?;

        for page in pages {
            let output_path = content_dir.join(&page.path);

            // Create parent directories if needed
            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
            }

            std::fs::write(&output_path, &page.content)
                .with_context(|| format!("Failed to write: {}", output_path.display()))?;

            verbose!(2, verbosity, "  Wrote: {}", output_path.display());
            files_written += 1;
        }
    }

    Ok(files_written)
}

/// Generate SSG-specific files (navigation, config, CSS)
fn generate_ssg_files(
    module_renderer: &ModuleRenderer,
    python_modules: &[PythonModule],
    rust_modules: &[RustModule],
    config: &Config,
    output_dir: &Path,
    template: Option<&str>,
    verbosity: u8,
) -> Result<usize> {
    let mut files_written = 0;
    let is_mdbook = template == Some(TEMPLATE_MDBOOK);

    if is_mdbook {
        // Generate mdBook SUMMARY.md
        let summary = module_renderer.generate_mdbook_summary(python_modules, rust_modules);
        let summary_path = output_dir.join("src/SUMMARY.md");
        if let Some(parent) = summary_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&summary_path, &summary)
            .with_context(|| format!("Failed to write SUMMARY.md: {}", summary_path.display()))?;
        verbose!(2, verbosity, "  Wrote: {}", summary_path.display());
        files_written += 1;

        // Generate book.toml
        let authors = vec![config.project.name.clone()];
        let book_config = module_renderer.generate_mdbook_config(&config.project.name, &authors);
        let config_path = output_dir.join("book.toml");
        std::fs::write(&config_path, &book_config)
            .with_context(|| format!("Failed to write book.toml: {}", config_path.display()))?;
        verbose!(2, verbosity, "  Wrote: {}", config_path.display());
        files_written += 1;

        // Generate custom CSS
        let css = module_renderer.generate_mdbook_css();
        let css_dir = output_dir.join("theme");
        std::fs::create_dir_all(&css_dir)?;
        let css_path = css_dir.join("custom.css");
        std::fs::write(&css_path, &css)
            .with_context(|| format!("Failed to write custom.css: {}", css_path.display()))?;
        verbose!(2, verbosity, "  Wrote: {}", css_path.display());
        files_written += 1;
    } else {
        // Generate MkDocs navigation YAML
        let nav_yaml = module_renderer.generate_nav_yaml(python_modules, rust_modules);
        let nav_path = output_dir.join("_nav.yml");
        std::fs::write(&nav_path, &nav_yaml)
            .with_context(|| format!("Failed to write nav file: {}", nav_path.display()))?;
        verbose!(2, verbosity, "  Wrote: {}", nav_path.display());
        files_written += 1;
    }

    Ok(files_written)
}

/// Initialize a new plissken.toml configuration file
fn init(force: bool, verbosity: u8) -> Result<()> {
    let config_path = PathBuf::from(PLISSKEN_CONFIG);

    // Check if config already exists
    if config_path.exists() && !force {
        return Err(CliError::new(format!("{} already exists", PLISSKEN_CONFIG))
            .with_hint("use --force to overwrite the existing configuration")
            .into());
    }

    verbose!(1, verbosity, "Detecting project type...");

    // Detect project type
    let project = detect_project()?;

    verbose!(1, verbosity, "Detected: {}", project.description());

    // Generate config content
    let config_content = generate_config(&project)?;

    // Write config file
    std::fs::write(&config_path, &config_content).map_err(|e| {
        CliError::new(format!("failed to write {}", PLISSKEN_CONFIG))
            .with_context(e.to_string())
            .with_hint("check that you have write permissions in this directory")
    })?;

    verbose!(1, verbosity, "Created {}", PLISSKEN_CONFIG);

    Ok(())
}

/// Validation issue found during check
#[derive(serde::Serialize)]
struct ValidationIssue {
    severity: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    hint: Option<String>,
}

/// Result of configuration validation (CLI output format)
#[derive(serde::Serialize)]
struct CliValidationResult {
    valid: bool,
    config_path: String,
    issues: Vec<ValidationIssue>,
}

impl CliValidationResult {
    fn new(config_path: &Path) -> Self {
        Self {
            valid: true,
            config_path: config_path.display().to_string(),
            issues: Vec::new(),
        }
    }

    fn add_error(&mut self, message: impl Into<String>, hint: Option<String>) {
        self.valid = false;
        self.issues.push(ValidationIssue {
            severity: "error".to_string(),
            message: message.into(),
            hint,
        });
    }

    fn add_warning(&mut self, message: impl Into<String>, hint: Option<String>) {
        self.issues.push(ValidationIssue {
            severity: "warning".to_string(),
            message: message.into(),
            hint,
        });
    }
}

/// Validate configuration without generating documentation
fn check(path: &str, format: &str, verbosity: u8) -> Result<()> {
    let project_path = Path::new(path);

    // Find config file
    let config_path = match find_config(project_path) {
        Ok(p) => p,
        Err(e) => {
            if format == "json" {
                let result = CliValidationResult {
                    valid: false,
                    config_path: project_path.join(PLISSKEN_CONFIG).display().to_string(),
                    issues: vec![ValidationIssue {
                        severity: "error".to_string(),
                        message: "configuration file not found".to_string(),
                        hint: Some("run 'plissken init' to create a configuration file".to_string()),
                    }],
                };
                println!("{}", serde_json::to_string_pretty(&result)?);
                std::process::exit(1);
            }
            return Err(e);
        }
    };

    let project_root = config_path
        .parent()
        .ok_or_else(|| CliError::new("Config path has no parent directory"))?;

    verbose!(1, verbosity, "Checking config: {}", config_path.display());

    let mut result = CliValidationResult::new(&config_path);

    // Try to load and parse config
    let config = match load_config(&config_path) {
        Ok(c) => c,
        Err(e) => {
            result.add_error(
                format!("failed to parse configuration: {}", e),
                Some("check TOML syntax and required fields".to_string()),
            );
            return output_result(&result, format, verbosity);
        }
    };

    verbose!(1, verbosity, "Config parsed successfully");

    // Use Config::validate() from plissken-core
    match config.validate(project_root) {
        Ok(core_result) => {
            // Convert core warnings to CLI format
            for warning in core_result.warnings {
                result.add_warning(&warning.message, warning.hint);
            }
        }
        Err(e) => {
            // Convert core error to CLI format with appropriate hints
            let hint = match &e {
                plissken_core::ConfigError::NoLanguageConfigured => {
                    Some("add at least one source section to generate documentation".to_string())
                }
                plissken_core::ConfigError::VersionSourceNotFound(_, _) => {
                    Some("create the file or change version_from to another source (cargo, pyproject, git)".to_string())
                }
                plissken_core::ConfigError::RustCrateNotFound(_) => {
                    Some("check the crates array in [rust] section".to_string())
                }
                plissken_core::ConfigError::PythonSourceNotFound(_) => {
                    Some("check the 'source' or 'package' field in [python] section".to_string())
                }
                plissken_core::ConfigError::GitRepoNotFound => {
                    Some("initialize git with 'git init' or change version_from".to_string())
                }
            };
            result.add_error(e.to_string(), hint);
        }
    }

    output_result(&result, format, verbosity)
}

/// Output validation result in requested format
fn output_result(result: &CliValidationResult, format: &str, verbosity: u8) -> Result<()> {
    if format == "json" {
        println!("{}", serde_json::to_string_pretty(result)?);
    } else {
        // Text format
        if result.valid {
            verbose!(1, verbosity, "Configuration is valid");
            if !result.issues.is_empty() {
                for issue in &result.issues {
                    eprintln!("warning: {}", issue.message);
                    if let Some(ref hint) = issue.hint {
                        eprintln!("hint: {}", hint);
                    }
                }
            }
        } else {
            for issue in &result.issues {
                if issue.severity == "error" {
                    eprintln!("error: {}", issue.message);
                } else {
                    eprintln!("warning: {}", issue.message);
                }
                if let Some(ref hint) = issue.hint {
                    eprintln!("hint: {}", hint);
                }
            }
        }
    }

    if result.valid {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

/// Detected project information
struct DetectedProject {
    name: String,
    has_rust: bool,
    has_python: bool,
    rust_crates: Vec<PathBuf>,
    rust_entry_point: Option<String>,
    python_package: Option<String>,
    python_source: Option<PathBuf>,
    is_hybrid: bool,
}

impl DetectedProject {
    fn description(&self) -> String {
        match (self.has_rust, self.has_python, self.is_hybrid) {
            (true, true, true) => format!("hybrid PyO3 project '{}'", self.name),
            (true, true, false) => format!("Rust + Python project '{}'", self.name),
            (true, false, _) => format!("Rust project '{}'", self.name),
            (false, true, _) => format!("Python project '{}'", self.name),
            (false, false, _) => format!("unknown project '{}'", self.name),
        }
    }
}

/// Detect project type from current directory
fn detect_project() -> Result<DetectedProject> {
    let cwd = std::env::current_dir()?;

    let mut project = DetectedProject {
        name: cwd
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("project")
            .to_string(),
        has_rust: false,
        has_python: false,
        rust_crates: Vec::new(),
        rust_entry_point: None,
        python_package: None,
        python_source: None,
        is_hybrid: false,
    };

    // Check for Cargo.toml
    let cargo_toml = cwd.join(CARGO_MANIFEST);
    if cargo_toml.exists() {
        project.has_rust = true;
        project.rust_crates.push(PathBuf::from("."));

        // Parse Cargo.toml for project info
        if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
            // Extract package name
            if let Some(name) = extract_cargo_name(&content) {
                project.name = name.clone();
                // Convert kebab-case to snake_case for entry point
                project.rust_entry_point = Some(name.replace('-', "_"));
            }

            // Check if it's a PyO3 project
            if content.contains("pyo3") || content.contains("maturin") {
                project.is_hybrid = true;
            }
        }
    }

    // Check for pyproject.toml
    let pyproject_toml = cwd.join(PYPROJECT_MANIFEST);
    if pyproject_toml.exists() {
        project.has_python = true;

        if let Ok(content) = std::fs::read_to_string(&pyproject_toml) {
            // Extract project name
            if let Some(name) = extract_pyproject_name(&content) {
                if !project.has_rust {
                    project.name = name.clone();
                }
                project.python_package = Some(name.replace('-', "_"));
            }

            // Extract Python source directory from pyproject.toml
            project.python_source = extract_python_source(&content);

            // Check for maturin build system (indicates hybrid)
            if content.contains("maturin") {
                project.is_hybrid = true;
            }
        }
    }

    // Check for setup.py as fallback
    let setup_py = cwd.join("setup.py");
    if setup_py.exists() && !project.has_python {
        project.has_python = true;
        // Try to infer package name from directory
        if project.python_package.is_none() {
            project.python_package = Some(project.name.replace('-', "_"));
        }
    }

    // If neither Rust nor Python detected, return error
    if !project.has_rust && !project.has_python {
        return Err(CliError::new("no Rust or Python project detected")
            .with_hint(format!(
                "run this command from a directory containing {} or {}",
                CARGO_MANIFEST, PYPROJECT_MANIFEST
            ))
            .into());
    }

    Ok(project)
}

/// Extract package name from Cargo.toml content
fn extract_cargo_name(content: &str) -> Option<String> {
    // Look for [package] section and name field
    let mut in_package = false;
    for line in content.lines() {
        let line = line.trim();
        if line == "[package]" {
            in_package = true;
            continue;
        }
        if line.starts_with('[') {
            in_package = false;
            continue;
        }
        if in_package && line.starts_with("name") {
            if let Some(val) = line.split('=').nth(1) {
                let name = val.trim().trim_matches('"').trim_matches('\'');
                return Some(name.to_string());
            }
        }
    }
    None
}

/// Extract project name from pyproject.toml content
fn extract_pyproject_name(content: &str) -> Option<String> {
    // Look for [project] section and name field, or [tool.poetry] name
    let mut in_project = false;
    let mut in_poetry = false;
    for line in content.lines() {
        let line = line.trim();
        if line == "[project]" {
            in_project = true;
            in_poetry = false;
            continue;
        }
        if line == "[tool.poetry]" {
            in_poetry = true;
            in_project = false;
            continue;
        }
        if line.starts_with('[') {
            in_project = false;
            in_poetry = false;
            continue;
        }
        if (in_project || in_poetry) && line.starts_with("name") {
            if let Some(val) = line.split('=').nth(1) {
                let name = val.trim().trim_matches('"').trim_matches('\'');
                return Some(name.to_string());
            }
        }
    }
    None
}

/// Extract Python source directory from pyproject.toml content
fn extract_python_source(content: &str) -> Option<PathBuf> {
    // Check for [tool.maturin] python-source (PyO3 projects)
    let mut in_maturin = false;
    for line in content.lines() {
        let line = line.trim();
        if line == "[tool.maturin]" {
            in_maturin = true;
            continue;
        }
        if line.starts_with('[') && in_maturin {
            in_maturin = false;
            continue;
        }
        if in_maturin && line.starts_with("python-source") {
            if let Some(val) = line.split('=').nth(1) {
                let source = val.trim().trim_matches('"').trim_matches('\'');
                return Some(PathBuf::from(source));
            }
        }
    }

    // Check for [tool.setuptools.packages.find] where (src layout)
    let mut in_find = false;
    for line in content.lines() {
        let line = line.trim();
        if line == "[tool.setuptools.packages.find]" {
            in_find = true;
            continue;
        }
        if line.starts_with('[') && in_find {
            in_find = false;
            continue;
        }
        if in_find && line.starts_with("where") {
            if let Some(val) = line.split('=').nth(1) {
                // where = ["src"] format
                let val = val.trim();
                if val.starts_with('[') {
                    // Parse array - take first element
                    let inner = val.trim_start_matches('[').trim_end_matches(']');
                    if let Some(first) = inner.split(',').next() {
                        let source = first.trim().trim_matches('"').trim_matches('\'');
                        if !source.is_empty() && source != "." {
                            return Some(PathBuf::from(source));
                        }
                    }
                }
            }
        }
    }

    // No explicit source configured - package is likely in root
    None
}

/// Generate plissken.toml content
fn generate_config(project: &DetectedProject) -> Result<String> {
    let mut config = String::new();

    // [project] section
    config.push_str("[project]\n");
    config.push_str(&format!("name = \"{}\"\n", project.name));

    // Version source - prefer Cargo for Rust projects
    if project.has_rust {
        config.push_str(&format!("version_from = \"{}\"\n", VERSION_SOURCE_CARGO));
    } else {
        config.push_str(&format!("version_from = \"{}\"\n", VERSION_SOURCE_PYPROJECT));
    }
    config.push('\n');

    // [output] section
    config.push_str("[output]\n");
    config.push_str(&format!("format = \"{}\"\n", DEFAULT_OUTPUT_FORMAT));
    config.push_str(&format!("path = \"{}\"\n", DEFAULT_OUTPUT_PATH));
    config.push_str(&format!("template = \"{}\"\n", TEMPLATE_MKDOCS_MATERIAL));
    config.push('\n');

    // [rust] section
    if project.has_rust {
        config.push_str("[rust]\n");
        config.push_str(&format!("crates = [\"{}\"]\n", DEFAULT_CRATES));
        if let Some(ref entry_point) = project.rust_entry_point {
            config.push_str(&format!("entry_point = \"{}\"\n", entry_point));
        }
        config.push('\n');
    }

    // [python] section
    if project.has_python || project.is_hybrid {
        config.push_str("[python]\n");
        let package = project
            .python_package
            .as_ref()
            .unwrap_or(&project.name)
            .replace('-', "_");
        config.push_str(&format!("package = \"{}\"\n", package));

        if let Some(ref source) = project.python_source {
            config.push_str(&format!("source = \"{}\"\n", source.display()));
        }
        config.push('\n');
    }

    Ok(config)
}

/// Find the plissken.toml config file
fn find_config(path: &Path) -> Result<PathBuf> {
    let path = if path.is_relative() {
        std::env::current_dir()?.join(path)
    } else {
        path.to_path_buf()
    };

    // If path is a file, use it directly
    if path.is_file()
        && path
            .file_name()
            .map(|f| f == PLISSKEN_CONFIG)
            .unwrap_or(false)
    {
        return Ok(path);
    }

    // Otherwise look for plissken.toml in directory
    let config_path = if path.is_dir() {
        path.join(PLISSKEN_CONFIG)
    } else {
        path
    };

    if config_path.exists() {
        Ok(config_path)
    } else {
        Err(CliError::new(format!(
            "{} not found at {}",
            PLISSKEN_CONFIG,
            config_path.display()
        ))
        .with_hint("run 'plissken init' to create a configuration file")
        .into())
    }
}

/// Load and parse configuration file with helpful error messages
fn load_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path).map_err(|e| {
        CliError::new(format!("failed to read {}", path.display()))
            .with_context(e.to_string())
            .with_hint("check that the file exists and is readable")
    })?;

    toml::from_str(&content).map_err(|e: toml::de::Error| {
        // Extract line/column info from toml error if available
        let mut err = CliError::new("invalid configuration file");

        // Parse the toml error message to extract location
        let err_str = e.to_string();
        if let Some(line_info) = extract_toml_location(&err_str) {
            err = err.with_context(format!("{}:{}", path.display(), line_info));
        } else {
            err = err.with_context(path.display().to_string());
        }

        // Add the actual error message and hint
        err = CliError::new(format!("invalid configuration: {}", summarize_toml_error(&err_str)))
            .with_context(err.context.unwrap_or_default())
            .with_hint(suggest_config_fix(&err_str));

        err.into()
    })
}

/// Extract line/column from toml error message
fn extract_toml_location(err: &str) -> Option<String> {
    // toml errors often contain "at line X column Y"
    if let Some(idx) = err.find("at line") {
        let rest = &err[idx..];
        if let Some(end) = rest.find(|c: char| c == '\n' || c == ',') {
            return Some(rest[..end].to_string());
        }
        return Some(rest.to_string());
    }
    None
}

/// Summarize a toml error message for users
fn summarize_toml_error(err: &str) -> String {
    // Extract the most useful part of the error
    if err.contains("missing field") {
        if let Some(start) = err.find('`') {
            if let Some(end) = err[start + 1..].find('`') {
                let field = &err[start + 1..start + 1 + end];
                return format!("missing required field '{}'", field);
            }
        }
    }
    if err.contains("unknown field") {
        if let Some(start) = err.find('`') {
            if let Some(end) = err[start + 1..].find('`') {
                let field = &err[start + 1..start + 1 + end];
                return format!("unknown field '{}'", field);
            }
        }
    }
    if err.contains("invalid type") {
        return "invalid value type".to_string();
    }
    // Fallback: return first line or truncated message
    err.lines().next().unwrap_or(err).to_string()
}

/// Suggest a fix based on the toml error
fn suggest_config_fix(err: &str) -> String {
    if err.contains("missing field `name`") || err.contains("missing field `project`") {
        return "ensure [project] section has a 'name' field".to_string();
    }
    if err.contains("missing field `path`") || err.contains("missing field `output`") {
        return "ensure [output] section has a 'path' field".to_string();
    }
    if err.contains("unknown field") {
        return "check spelling of field names in plissken.toml".to_string();
    }
    if err.contains("invalid type") {
        return "check that field values have the correct type (string, array, etc.)".to_string();
    }
    if err.contains("expected") {
        return "check TOML syntax - strings need quotes, arrays use brackets".to_string();
    }
    "check plissken.toml syntax and refer to documentation for config format".to_string()
}

/// Parse Rust source files based on config
fn parse_rust_sources(config: &Config, project_root: &Path, verbosity: u8) -> Result<Vec<RustModule>> {
    let Some(ref rust_config) = config.rust else {
        return Ok(Vec::new());
    };

    let parser = plissken_core::parser::RustParser::new();
    let mut modules = Vec::new();

    for crate_path in &rust_config.crates {
        let crate_dir = project_root.join(crate_path);

        // Read crate name from Cargo.toml
        let crate_name = read_crate_name(&crate_dir)?;
        verbose!(2, verbosity, "  Crate: {} ({})", crate_name, crate_dir.display());

        // Determine src directory (check common source directory names)
        let src_dir = if crate_dir.join("src").exists() {
            crate_dir.join("src")
        } else if crate_dir.join("rust").exists() && crate_dir.join("rust").join("lib.rs").exists() {
            crate_dir.join("rust")
        } else {
            crate_dir.clone()
        };

        // Find all .rs files in the crate
        let rs_files = find_rust_files(&crate_dir)?;

        for rs_file in rs_files {
            match parser.parse_file(&rs_file) {
                Ok(mut module) => {
                    // Convert filesystem path to logical module path
                    module.path = file_to_module_path(&rs_file, &crate_name, &src_dir);
                    verbose!(2, verbosity, "    {} -> {}", rs_file.display(), module.path);
                    modules.push(module);
                }
                Err(e) => {
                    warn_parse_error("Rust", &rs_file, &e);
                }
            }
        }
    }

    verbose!(2, verbosity, "  Found {} Rust files", modules.len());
    Ok(modules)
}

/// Read the crate name from a Cargo.toml file
fn read_crate_name(crate_dir: &Path) -> Result<String> {
    let cargo_toml = crate_dir.join(CARGO_MANIFEST);
    if !cargo_toml.exists() {
        // Fall back to directory name if no Cargo.toml
        return Ok(crate_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string());
    }

    let content = std::fs::read_to_string(&cargo_toml)
        .with_context(|| format!("failed to read {}", cargo_toml.display()))?;

    // Parse the TOML to extract package.name
    let parsed: toml::Value = toml::from_str(&content)
        .with_context(|| format!("failed to parse {}", cargo_toml.display()))?;

    // Check if this is a workspace manifest
    if parsed.get("workspace").is_some() && parsed.get("package").is_none() {
        return Err(CliError::new(format!(
            "{} is a workspace manifest, not a crate",
            cargo_toml.display()
        ))
        .with_hint("In plissken.toml [rust] section, specify individual crate paths instead of the workspace root, e.g.: crates = [\"crates/my-crate\"]")
        .into());
    }

    let name = parsed
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .ok_or_else(|| {
            CliError::new(format!(
                "missing package.name in {}",
                cargo_toml.display()
            ))
        })?;

    Ok(name.to_string())
}

/// Convert a file path to a logical Rust module path
///
/// Examples:
/// - `src/lib.rs` → `crate_name`
/// - `src/main.rs` → `crate_name`
/// - `src/foo.rs` → `crate_name::foo`
/// - `src/foo/mod.rs` → `crate_name::foo`
/// - `src/foo/bar.rs` → `crate_name::foo::bar`
fn file_to_module_path(file_path: &Path, crate_name: &str, src_dir: &Path) -> String {
    // Get path relative to src directory
    let relative = match file_path.strip_prefix(src_dir) {
        Ok(rel) => rel,
        Err(_) => return crate_name.to_string(),
    };

    let file_name = relative
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    // Handle crate root files
    if file_name == "lib.rs" || file_name == "main.rs" {
        return crate_name.to_string();
    }

    // Build module path from directory components
    let mut parts: Vec<&str> = Vec::new();

    for component in relative.parent().unwrap_or(Path::new("")).components() {
        if let std::path::Component::Normal(name) = component {
            if let Some(name_str) = name.to_str() {
                parts.push(name_str);
            }
        }
    }

    // Handle mod.rs - represents the parent directory module
    if file_name == "mod.rs" {
        if parts.is_empty() {
            return crate_name.to_string();
        }
        return format!("{}::{}", crate_name, parts.join("::"));
    }

    // Regular file - add the file stem as the final module component
    if let Some(stem) = relative.file_stem().and_then(|s| s.to_str()) {
        parts.push(stem);
    }

    if parts.is_empty() {
        crate_name.to_string()
    } else {
        format!("{}::{}", crate_name, parts.join("::"))
    }
}

/// Find all Rust source files in a directory
fn find_rust_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if !dir.exists() {
        return Ok(files);
    }

    // Check for common source directory names
    let search_dir = if dir.join("src").exists() {
        dir.join("src")
    } else if dir.join("rust").exists() && dir.join("rust").join("lib.rs").exists() {
        dir.join("rust")
    } else {
        dir.to_path_buf()
    };
    let search_dir = &search_dir;

    fn walk_dir(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                walk_dir(&path, files)?;
            } else if path.extension().map(|e| e == "rs").unwrap_or(false) {
                files.push(path);
            }
        }
        Ok(())
    }

    walk_dir(search_dir, &mut files)?;
    Ok(files)
}

/// Parse Python source files based on config
fn parse_python_sources(config: &Config, project_root: &Path, verbosity: u8) -> Result<Vec<PythonModule>> {
    let Some(ref python_config) = config.python else {
        return Ok(Vec::new());
    };

    let mut parser = plissken_core::parser::PythonParser::new();
    let mut modules = Vec::new();

    // Determine Python source directory
    let python_dir = if let Some(ref source) = python_config.source {
        project_root.join(source)
    } else {
        project_root.join(&python_config.package)
    };

    if !python_dir.exists() {
        return Ok(modules);
    }

    // Use auto-discovery or explicit file finding
    let py_files: Vec<PathBuf> = if python_config.auto_discover {
        verbose!(1, verbosity, "Auto-discovering Python modules in {}...", python_dir.display());
        let discovered = plissken_core::discover_python_modules(&python_dir, &python_config.package)
            .map_err(|e| CliError::new(format!("failed to discover Python modules: {}", e)))?;
        verbose!(2, verbosity, "  Discovered {} Python modules", discovered.len());
        discovered.into_iter().map(|m| m.path).collect()
    } else {
        find_python_files(&python_dir)?
    };

    for py_file in py_files {
        match parser.parse_file(&py_file) {
            Ok(module) => modules.push(module),
            Err(e) => {
                warn_parse_error("Python", &py_file, &e);
            }
        }
    }

    verbose!(2, verbosity, "  Found {} Python files", modules.len());
    Ok(modules)
}

/// Find all Python source files in a directory
fn find_python_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    fn walk_dir(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                walk_dir(&path, files)?;
            } else if path.extension().map(|e| e == "py").unwrap_or(false) {
                // Skip __pycache__ and other hidden directories
                if !path
                    .components()
                    .any(|c| c.as_os_str().to_string_lossy().starts_with("__pycache__"))
                {
                    files.push(path);
                }
            }
        }
        Ok(())
    }

    walk_dir(dir, &mut files)?;
    Ok(files)
}

/// Get project version from configured source
fn get_project_version(config: &Config, project_root: &Path) -> Option<String> {
    use plissken_core::config::VersionSource;

    match config.project.version_from {
        VersionSource::Cargo => {
            // Try to read version from Cargo.toml
            let cargo_toml = project_root.join(CARGO_MANIFEST);
            if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
                // Simple TOML parsing for version
                for line in content.lines() {
                    let line = line.trim();
                    if line.starts_with("version")
                        && let Some(val) = line.split('=').nth(1)
                    {
                        let version = val.trim().trim_matches('"').trim_matches('\'');
                        return Some(version.to_string());
                    }
                }
            }
            None
        }
        VersionSource::Pyproject => {
            // Try to read version from pyproject.toml
            let pyproject = project_root.join(PYPROJECT_MANIFEST);
            if let Ok(content) = std::fs::read_to_string(&pyproject) {
                for line in content.lines() {
                    let line = line.trim();
                    if line.starts_with("version")
                        && let Some(val) = line.split('=').nth(1)
                    {
                        let version = val.trim().trim_matches('"').trim_matches('\'');
                        return Some(version.to_string());
                    }
                }
            }
            None
        }
        VersionSource::Git => {
            // Use git describe
            std::process::Command::new("git")
                .args(["describe", "--tags", "--always"])
                .current_dir(project_root)
                .output()
                .ok()
                .and_then(|o| {
                    if o.status.success() {
                        String::from_utf8(o.stdout)
                            .ok()
                            .map(|s| s.trim().to_string())
                    } else {
                        None
                    }
                })
        }
    }
}

/// Get current git ref (branch or tag)
fn get_git_ref(project_root: &Path) -> Option<String> {
    std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(project_root)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

/// Get current git commit hash
fn get_git_commit(project_root: &Path) -> Option<String> {
    std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(project_root)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

/// Get current timestamp in ISO 8601 format (lightweight, no chrono dependency)
fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    let secs = duration.as_secs();

    // Calculate date components (simplified, assumes UTC)
    let days = secs / 86400;
    let remaining = secs % 86400;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;
    let seconds = remaining % 60;

    // Calculate year, month, day from days since epoch
    // This is a simplified calculation that works for dates from 1970-2100
    let mut year = 1970;
    let mut remaining_days = days as i64;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let days_in_months: [i64; 12] = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1;
    for &days_in_month in &days_in_months {
        if remaining_days < days_in_month {
            break;
        }
        remaining_days -= days_in_month;
        month += 1;
    }

    let day = remaining_days + 1;

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hours, minutes, seconds
    )
}

fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Normalize a Python file path to a logical module path.
///
/// Converts `/path/to/project/python/my_module/utils.py` to `my_module.utils`
fn normalize_python_module_path(file_path: &str, config: &Config, project_root: &Path) -> String {
    let path = Path::new(file_path);

    // Get relative path from project root
    let relative = path.strip_prefix(project_root).unwrap_or(path);

    // Get the package path from config
    let package_path = config
        .python
        .as_ref()
        .map(|p| Path::new(&p.package))
        .unwrap_or(Path::new(""));

    // Get the package name (last component of the package path)
    let package_name = package_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_else(|| &config.project.name);

    // Determine Python source directory (parent of the package)
    let python_source = config
        .python
        .as_ref()
        .and_then(|p| p.source.as_ref())
        .map(|s| s.as_path())
        .unwrap_or_else(|| package_path.parent().unwrap_or(Path::new("")));

    // Strip the Python source prefix
    let module_path = relative.strip_prefix(python_source).unwrap_or(relative);

    // Convert to dotted module path and remove .py extension
    let path_str = module_path
        .with_extension("")
        .to_string_lossy()
        .replace(['/', '\\'], ".");

    // Handle __init__.py -> package name
    if path_str.ends_with(".__init__") {
        path_str[..path_str.len() - 9].to_string()
    } else if path_str == "__init__" {
        package_name.to_string()
    } else {
        path_str
    }
}

// Note: normalize_rust_module_path was removed.
// Rust module paths are now computed in parse_rust_sources using file_to_module_path
// which reads crate names directly from Cargo.toml.
