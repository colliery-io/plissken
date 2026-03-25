# Tutorial: Document a Rust Project

This tutorial walks you through generating documentation for a pure Rust
crate. By the end, you'll have a browsable documentation site with
auto-generated API reference pages for your structs, enums, functions,
and traits.

## Prerequisites

- [plissken installed](../how-to/install.md)
- Rust toolchain installed (for building)
- pip installed (for MkDocs) or cargo (for mdBook)

## What You'll Build

You'll create a small Rust library and generate documentation that includes:

- Module overview pages with `//!` doc comments
- Struct and enum documentation with field details
- Function signatures with parameter types
- Visibility badges (`pub`, `pub(crate)`, private)

## Step 1: Create a Rust Crate

```bash
cargo new --lib filterkit && cd filterkit
```

Replace the contents of `src/lib.rs`:

```rust
//! # filterkit
//!
//! A composable data filtering library.
//!
//! Provides a pipeline-based approach to filtering collections
//! with reusable, chainable filter stages.

pub mod filter;
pub mod pipeline;
```

Create `src/filter.rs`:

```rust
//! Individual filter definitions.

/// A filter that can accept or reject items.
///
/// Implement this trait to create custom filters that can be
/// composed into pipelines.
///
/// # Examples
///
/// ```rust
/// use filterkit::filter::Filter;
///
/// struct PositiveFilter;
///
/// impl Filter<i32> for PositiveFilter {
///     fn apply(&self, item: &i32) -> bool {
///         *item > 0
///     }
///
///     fn name(&self) -> &str {
///         "positive"
///     }
/// }
/// ```
pub trait Filter<T> {
    /// Test whether an item passes this filter.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to test
    ///
    /// # Returns
    ///
    /// `true` if the item passes the filter, `false` otherwise.
    fn apply(&self, item: &T) -> bool;

    /// Human-readable name for this filter.
    fn name(&self) -> &str;
}

/// A filter that passes items within a numeric range.
///
/// Both bounds are inclusive.
///
/// # Examples
///
/// ```rust
/// use filterkit::filter::{Filter, RangeFilter};
///
/// let f = RangeFilter::new(1, 10);
/// assert!(f.apply(&5));
/// assert!(!f.apply(&11));
/// ```
#[derive(Debug, Clone)]
pub struct RangeFilter {
    /// Minimum value (inclusive).
    pub min: i64,
    /// Maximum value (inclusive).
    pub max: i64,
}

impl RangeFilter {
    /// Create a new range filter.
    ///
    /// # Arguments
    ///
    /// * `min` - Lower bound (inclusive)
    /// * `max` - Upper bound (inclusive)
    ///
    /// # Panics
    ///
    /// Panics if `min > max`.
    pub fn new(min: i64, max: i64) -> Self {
        assert!(min <= max, "min ({min}) must be <= max ({max})");
        Self { min, max }
    }
}

impl Filter<i64> for RangeFilter {
    fn apply(&self, item: &i64) -> bool {
        *item >= self.min && *item <= self.max
    }

    fn name(&self) -> &str {
        "range"
    }
}

/// Result of applying a filter to a collection.
#[derive(Debug, Clone)]
pub enum FilterResult {
    /// All items passed the filter.
    AllPassed,
    /// Some items were rejected.
    ///
    /// Contains the count of rejected items.
    Partial {
        /// Number of items that were rejected.
        rejected: usize,
    },
    /// All items were rejected.
    AllRejected,
}
```

Create `src/pipeline.rs`:

```rust
//! Filter pipeline for composing multiple filters.

use crate::filter::{Filter, FilterResult};

/// A pipeline that applies filters in sequence.
///
/// Items must pass all filters to be included in the output.
///
/// # Examples
///
/// ```rust
/// use filterkit::pipeline::Pipeline;
/// use filterkit::filter::RangeFilter;
///
/// let mut pipeline = Pipeline::new();
/// pipeline.add_filter(Box::new(RangeFilter::new(0, 100)));
///
/// let data = vec![1, 50, 200, -5, 99];
/// let filtered = pipeline.run(&data);
/// assert_eq!(filtered, vec![&1, &50, &99]);
/// ```
pub struct Pipeline<T> {
    filters: Vec<Box<dyn Filter<T>>>,
}

impl<T> Pipeline<T> {
    /// Create an empty pipeline.
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    /// Add a filter to the pipeline.
    ///
    /// Filters are applied in the order they are added.
    ///
    /// # Arguments
    ///
    /// * `filter` - The filter to append
    pub fn add_filter(&mut self, filter: Box<dyn Filter<T>>) {
        self.filters.push(filter);
    }

    /// Run the pipeline on a collection.
    ///
    /// Returns references to items that pass all filters.
    ///
    /// # Arguments
    ///
    /// * `items` - The collection to filter
    ///
    /// # Returns
    ///
    /// A vector of references to items that passed all filters.
    pub fn run<'a>(&self, items: &'a [T]) -> Vec<&'a T> {
        items
            .iter()
            .filter(|item| self.filters.iter().all(|f| f.apply(item)))
            .collect()
    }

    /// Run the pipeline and report the result.
    ///
    /// # Arguments
    ///
    /// * `items` - The collection to filter
    pub fn run_with_report(&self, items: &[T]) -> (Vec<&T>, FilterResult) {
        let passed: Vec<&T> = self.run(items);
        let rejected = items.len() - passed.len();

        let result = if rejected == 0 {
            FilterResult::AllPassed
        } else if passed.is_empty() {
            FilterResult::AllRejected
        } else {
            FilterResult::Partial { rejected }
        };

        (passed, result)
    }

    /// Number of filters in the pipeline.
    pub fn len(&self) -> usize {
        self.filters.len()
    }

    /// Whether the pipeline has no filters.
    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }
}

impl<T> Default for Pipeline<T> {
    fn default() -> Self {
        Self::new()
    }
}
```

## Step 2: Initialize plissken

```bash
plissken init
```

The generated `plissken.toml`:

```toml
[project]
name = "filterkit"
version_from = "cargo"

[output]
format = "markdown"
path = "docs/api"
template = "mkdocs-material"

[rust]
crates = ["."]
```

## Step 3: Generate Documentation

```bash
plissken render
```

This produces:

```
docs/api/
  rust/
    filterkit.md
    filterkit/filter.md
    filterkit/pipeline.md
  _nav.yml
```

## Step 4: Set Up MkDocs

Install MkDocs Material:

```bash
pip install mkdocs-material
```

Create `mkdocs.yml`:

```yaml
site_name: filterkit
theme:
  name: material

nav:
  - Home: index.md
  - API Reference:
    - filterkit: api/rust/filterkit.md
    - filterkit::filter: api/rust/filterkit/filter.md
    - filterkit::pipeline: api/rust/filterkit/pipeline.md
```

Create `docs/index.md`:

```markdown
# filterkit

A composable data filtering library for Rust.

## API Reference

- [filterkit](api/rust/filterkit.md) — Crate overview
- [filterkit::filter](api/rust/filterkit/filter.md) — Filter trait and implementations
- [filterkit::pipeline](api/rust/filterkit/pipeline.md) — Filter pipeline
```

## Step 5: Serve and Browse

```bash
mkdocs serve
```

Open [http://localhost:8000](http://localhost:8000). You'll see:

- Module pages with `//!` doc comments rendered as the overview
- The `Filter` trait with its methods documented
- `RangeFilter` struct with fields and their types
- `Pipeline` with its full method list
- `FilterResult` enum with variant documentation
- Visibility badges on each item (`pub`)
- `# Examples` sections rendered as code blocks

## Using mdBook Instead

If you prefer Rust-style documentation, change the template:

```toml
[output]
template = "mdbook"
path = "src"
```

Regenerate and serve:

```bash
plissken render
mdbook serve
```

See [How-To: Use mdBook](../how-to/use-mdbook.md) for details.

## What You Learned

- How to initialize plissken for a Rust crate
- How to write doc comments that plissken extracts (module-level `//!`, item-level `///`)
- How the `# Arguments`, `# Returns`, `# Examples` sections are parsed
- How visibility badges appear in the output
- How to switch between MkDocs Material and mdBook

## Next Steps

- [Tutorial: Document a Hybrid Project](hybrid-project.md) — Add Python bindings
- [Reference: CLI](../reference/cli.md) — All command-line options
- [Explanation: How Parsing Works](../explanation/parsing.md) — Details of Rust parsing
