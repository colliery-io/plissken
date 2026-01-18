# python <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Python source code parser using tree-sitter

## Structs

### `struct PythonParser`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


#### Fields

| Name | Type | Description |
|------|------|-------------|
| `parser` | `Parser` |  |

#### Methods

##### `new` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn new () -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_python::LANGUAGE.into())
            .expect("Failed to load Python grammar");
        Self { parser }
    }
```

</details>



##### `parse_file` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn parse_file (& mut self , path : & Path) -> crate :: error :: Result < PythonModule >
```

Parse a Python source file.

**Raises:**

| Exception | Description |
|-----------|-------------|
| `Error` | Returns `PlisskenError::FileRead` if the file cannot be read, `PlisskenError::Parse` if the Python syntax is invalid. |


<details>
<summary>Source</summary>

```rust
    pub fn parse_file(&mut self, path: &Path) -> crate::error::Result<PythonModule> {
        use crate::error::PlisskenError;

        let content = std::fs::read_to_string(path)
            .map_err(|e| PlisskenError::file_read(path, e))?;
        self.parse_str(&content, path)
    }
```

</details>



##### `parse_str` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn parse_str (& mut self , content : & str , path : & Path) -> crate :: error :: Result < PythonModule >
```

Parse Python source from a string.

**Raises:**

| Exception | Description |
|-----------|-------------|
| `Error` | Returns `PlisskenError::Parse` if the Python syntax is invalid. |


<details>
<summary>Source</summary>

```rust
    pub fn parse_str(&mut self, content: &str, path: &Path) -> crate::error::Result<PythonModule> {
        use crate::error::PlisskenError;

        let tree = self
            .parser
            .parse(content, None)
            .ok_or_else(|| PlisskenError::python_parse(path, "failed to parse Python source"))?;

        let root = tree.root_node();

        // Extract module docstring
        let docstring = extract_module_docstring(&root, content);

        // Extract items
        let items = extract_module_items(&root, content, path);

        Ok(PythonModule {
            path: path.display().to_string(),
            docstring,
            items,
            source_type: SourceType::Python,
            source: SourceSpan::new(
                path.to_path_buf(),
                1,
                content.lines().count().max(1),
                content,
            ),
        })
    }
```

</details>





## Functions

### `fn extract_module_docstring`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_module_docstring (root : & Node , content : & str) -> Option < String >
```

<details>
<summary>Source</summary>

```rust
fn extract_module_docstring(root: &Node, content: &str) -> Option<String> {
    // Module docstring is the first expression_statement containing a string
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "expression_statement" {
            let mut child_cursor = child.walk();
            for expr in child.children(&mut child_cursor) {
                if expr.kind() == "string" {
                    return extract_string_content(&expr, content);
                }
            }
        } else if child.kind() != "comment" {
            // Stop at first non-comment, non-docstring
            break;
        }
    }
    None
}
```

</details>



### `fn extract_module_items`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_module_items (root : & Node , content : & str , path : & Path) -> Vec < PythonItem >
```

<details>
<summary>Source</summary>

```rust
fn extract_module_items(root: &Node, content: &str, path: &Path) -> Vec<PythonItem> {
    let mut items = Vec::new();

    // Collect children into a vector so we can peek ahead for variable docstrings
    let mut cursor = root.walk();
    let children: Vec<_> = root.children(&mut cursor).collect();

    let mut i = 0;
    while i < children.len() {
        let child = &children[i];
        match child.kind() {
            "class_definition" => {
                items.push(PythonItem::Class(extract_class(child, content, path)));
            }
            "function_definition" => {
                items.push(PythonItem::Function(extract_function(child, content, path)));
            }
            "decorated_definition" => {
                // Handle decorated classes and functions
                let decorators = extract_decorators(child, content);
                let mut inner_cursor = child.walk();
                for inner in child.children(&mut inner_cursor) {
                    match inner.kind() {
                        "class_definition" => {
                            let mut class = extract_class(&inner, content, path);
                            class.decorators = decorators.clone();
                            items.push(PythonItem::Class(class));
                        }
                        "function_definition" => {
                            let mut func = extract_function(&inner, content, path);
                            func.decorators = decorators.clone();
                            // Check for special decorators
                            for dec in &func.decorators {
                                if dec == "staticmethod" {
                                    func.is_staticmethod = true;
                                } else if dec == "classmethod" {
                                    func.is_classmethod = true;
                                } else if dec == "property" || dec.starts_with("property.") {
                                    func.is_property = true;
                                }
                            }
                            items.push(PythonItem::Function(func));
                        }
                        _ => {}
                    }
                }
            }
            "expression_statement" => {
                // Check for annotated assignments (module-level variables with types)
                let mut inner_cursor = child.walk();
                for inner in child.children(&mut inner_cursor) {
                    if inner.kind() == "assignment"
                        && let Some(mut var) = extract_variable(&inner, content)
                    {
                        // Look ahead for a docstring (PEP 224-style informal convention)
                        // A string literal immediately following a variable is its docstring
                        if i + 1 < children.len() {
                            let next = &children[i + 1];
                            if next.kind() == "expression_statement" {
                                if let Some(docstring) =
                                    extract_expression_string(next, content)
                                {
                                    var.docstring = Some(docstring);
                                    i += 1; // Skip the docstring node
                                }
                            }
                        }
                        items.push(PythonItem::Variable(var));
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }

    items
}
```

</details>



### `fn extract_expression_string`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_expression_string (node : & Node , content : & str) -> Option < String >
```

Extract a string from an expression_statement (used for variable docstrings)

<details>
<summary>Source</summary>

```rust
fn extract_expression_string(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "string" {
            return extract_string_content(&child, content);
        }
    }
    None
}
```

</details>



### `fn extract_class`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_class (node : & Node , content : & str , path : & Path) -> PythonClass
```

<details>
<summary>Source</summary>

```rust
fn extract_class(node: &Node, content: &str, path: &Path) -> PythonClass {
    let name = node
        .child_by_field_name("name")
        .map(|n| node_text(&n, content))
        .unwrap_or_default();

    // Extract base classes
    let bases = extract_bases(node, content);

    // Extract docstring and methods from body
    let body = node.child_by_field_name("body");
    let (docstring, methods, attributes) = if let Some(body) = body {
        extract_class_body(&body, content, path)
    } else {
        (None, vec![], vec![])
    };

    let start_line = node.start_position().row + 1;
    let end_line = node.end_position().row + 1;
    let source_text = extract_source_text(node, content);

    PythonClass {
        name,
        docstring,
        bases,
        methods,
        attributes,
        decorators: vec![],
        rust_impl: None,
        source: SourceSpan {
            location: SourceLocation {
                file: path.to_path_buf(),
                line_start: start_line,
                line_end: end_line,
            },
            source: source_text,
        },
    }
}
```

</details>



### `fn extract_bases`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_bases (node : & Node , content : & str) -> Vec < String >
```

<details>
<summary>Source</summary>

```rust
fn extract_bases(node: &Node, content: &str) -> Vec<String> {
    let mut bases = Vec::new();

    if let Some(args) = node.child_by_field_name("superclasses") {
        let mut cursor = args.walk();
        for child in args.children(&mut cursor) {
            match child.kind() {
                "identifier" | "attribute" => {
                    bases.push(node_text(&child, content));
                }
                "argument_list" => {
                    // Handle argument_list for bases
                    let mut inner_cursor = child.walk();
                    for inner in child.children(&mut inner_cursor) {
                        if inner.kind() == "identifier" || inner.kind() == "attribute" {
                            bases.push(node_text(&inner, content));
                        }
                    }
                }
                _ => {}
            }
        }
    }

    bases
}
```

</details>



### `fn extract_class_body`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_class_body (body : & Node , content : & str , path : & Path ,) -> (Option < String > , Vec < PythonFunction > , Vec < PythonVariable >)
```

<details>
<summary>Source</summary>

```rust
fn extract_class_body(
    body: &Node,
    content: &str,
    path: &Path,
) -> (Option<String>, Vec<PythonFunction>, Vec<PythonVariable>) {
    let mut docstring = None;
    let mut methods = Vec::new();
    let mut attributes = Vec::new();
    let mut first_item = true;

    let mut cursor = body.walk();
    for child in body.children(&mut cursor) {
        match child.kind() {
            "expression_statement" if first_item => {
                // Check for docstring
                let mut inner_cursor = child.walk();
                for inner in child.children(&mut inner_cursor) {
                    if inner.kind() == "string" {
                        docstring = extract_string_content(&inner, content);
                    }
                }
                first_item = false;
            }
            "function_definition" => {
                methods.push(extract_function(&child, content, path));
                first_item = false;
            }
            "decorated_definition" => {
                let decorators = extract_decorators(&child, content);
                let mut inner_cursor = child.walk();
                for inner in child.children(&mut inner_cursor) {
                    if inner.kind() == "function_definition" {
                        let mut func = extract_function(&inner, content, path);
                        func.decorators = decorators.clone();
                        // Check for special decorators
                        for dec in &func.decorators {
                            if dec == "staticmethod" {
                                func.is_staticmethod = true;
                            } else if dec == "classmethod" {
                                func.is_classmethod = true;
                            } else if dec == "property" || dec.starts_with("property.") {
                                func.is_property = true;
                            }
                        }
                        methods.push(func);
                    }
                }
                first_item = false;
            }
            "expression_statement" => {
                // Check for class attributes
                let mut inner_cursor = child.walk();
                for inner in child.children(&mut inner_cursor) {
                    if inner.kind() == "assignment"
                        && let Some(var) = extract_variable(&inner, content)
                    {
                        attributes.push(var);
                    }
                }
                first_item = false;
            }
            _ => {
                if child.kind() != "comment" && child.kind() != "pass_statement" {
                    first_item = false;
                }
            }
        }
    }

    (docstring, methods, attributes)
}
```

</details>



### `fn extract_function`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_function (node : & Node , content : & str , path : & Path) -> PythonFunction
```

<details>
<summary>Source</summary>

```rust
fn extract_function(node: &Node, content: &str, path: &Path) -> PythonFunction {
    let name = node
        .child_by_field_name("name")
        .map(|n| node_text(&n, content))
        .unwrap_or_default();

    let is_async = node.kind() == "function_definition"
        && node.child(0).map(|c| c.kind() == "async").unwrap_or(false);

    // Extract parameters
    let (params, signature_str) = extract_parameters(node, content);

    // Extract return type
    let return_type = node
        .child_by_field_name("return_type")
        .map(|n| node_text(&n, content));

    // Extract docstring from body
    let docstring = node
        .child_by_field_name("body")
        .and_then(|body| extract_function_docstring(&body, content));

    let start_line = node.start_position().row + 1;
    let end_line = node.end_position().row + 1;
    let source_text = extract_source_text(node, content);

    // Build full signature string
    let full_sig = if let Some(ret) = &return_type {
        format!("def {}({}) -> {}", name, signature_str, ret)
    } else {
        format!("def {}({})", name, signature_str)
    };

    PythonFunction {
        name,
        docstring,
        signature_str: full_sig,
        signature: PythonFunctionSig {
            params,
            return_type,
        },
        decorators: vec![],
        is_async,
        is_staticmethod: false,
        is_classmethod: false,
        is_property: false,
        parsed_doc: None,
        rust_impl: None,
        source: SourceSpan {
            location: SourceLocation {
                file: path.to_path_buf(),
                line_start: start_line,
                line_end: end_line,
            },
            source: source_text,
        },
    }
}
```

</details>



### `fn extract_parameters`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_parameters (node : & Node , content : & str) -> (Vec < PythonParam > , String)
```

<details>
<summary>Source</summary>

```rust
fn extract_parameters(node: &Node, content: &str) -> (Vec<PythonParam>, String) {
    let mut params = Vec::new();
    let mut param_strs = Vec::new();

    if let Some(params_node) = node.child_by_field_name("parameters") {
        let mut cursor = params_node.walk();
        for child in params_node.children(&mut cursor) {
            match child.kind() {
                "identifier" => {
                    let name = node_text(&child, content);
                    param_strs.push(name.clone());
                    params.push(PythonParam {
                        name,
                        ty: None,
                        default: None,
                    });
                }
                "typed_parameter" => {
                    let name = child
                        .child_by_field_name("name")
                        .or_else(|| child.child(0))
                        .map(|n| node_text(&n, content))
                        .unwrap_or_default();
                    let ty = child
                        .child_by_field_name("type")
                        .map(|n| node_text(&n, content));

                    let param_str = if let Some(ref t) = ty {
                        format!("{}: {}", name, t)
                    } else {
                        name.clone()
                    };
                    param_strs.push(param_str);

                    params.push(PythonParam {
                        name,
                        ty,
                        default: None,
                    });
                }
                "default_parameter" => {
                    let name = child
                        .child_by_field_name("name")
                        .or_else(|| child.child(0))
                        .map(|n| node_text(&n, content))
                        .unwrap_or_default();
                    let value = child
                        .child_by_field_name("value")
                        .map(|n| node_text(&n, content));

                    let param_str = if let Some(ref v) = value {
                        format!("{}={}", name, v)
                    } else {
                        name.clone()
                    };
                    param_strs.push(param_str);

                    params.push(PythonParam {
                        name,
                        ty: None,
                        default: value,
                    });
                }
                "typed_default_parameter" => {
                    let name = child
                        .child_by_field_name("name")
                        .or_else(|| child.child(0))
                        .map(|n| node_text(&n, content))
                        .unwrap_or_default();
                    let ty = child
                        .child_by_field_name("type")
                        .map(|n| node_text(&n, content));
                    let value = child
                        .child_by_field_name("value")
                        .map(|n| node_text(&n, content));

                    let param_str = match (&ty, &value) {
                        (Some(t), Some(v)) => format!("{}: {} = {}", name, t, v),
                        (Some(t), None) => format!("{}: {}", name, t),
                        (None, Some(v)) => format!("{} = {}", name, v),
                        (None, None) => name.clone(),
                    };
                    param_strs.push(param_str);

                    params.push(PythonParam {
                        name,
                        ty,
                        default: value,
                    });
                }
                "list_splat_pattern" | "dictionary_splat_pattern" => {
                    let text = node_text(&child, content);
                    param_strs.push(text.clone());
                    params.push(PythonParam {
                        name: text,
                        ty: None,
                        default: None,
                    });
                }
                "*" => {
                    param_strs.push("*".to_string());
                }
                "/" => {
                    param_strs.push("/".to_string());
                }
                _ => {}
            }
        }
    }

    (params, param_strs.join(", "))
}
```

</details>



### `fn extract_function_docstring`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_function_docstring (body : & Node , content : & str) -> Option < String >
```

<details>
<summary>Source</summary>

```rust
fn extract_function_docstring(body: &Node, content: &str) -> Option<String> {
    let mut cursor = body.walk();
    for child in body.children(&mut cursor) {
        if child.kind() == "expression_statement" {
            let mut inner_cursor = child.walk();
            for inner in child.children(&mut inner_cursor) {
                if inner.kind() == "string" {
                    return extract_string_content(&inner, content);
                }
            }
        }
        // Stop at first non-docstring statement
        if child.kind() != "comment" {
            break;
        }
    }
    None
}
```

</details>



### `fn extract_decorators`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_decorators (node : & Node , content : & str) -> Vec < String >
```

<details>
<summary>Source</summary>

```rust
fn extract_decorators(node: &Node, content: &str) -> Vec<String> {
    let mut decorators = Vec::new();
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if child.kind() == "decorator" {
            // Get the decorator content (skip the @)
            let text = node_text(&child, content);
            let decorator = text.strip_prefix('@').unwrap_or(&text).to_string();
            decorators.push(decorator);
        }
    }

    decorators
}
```

</details>



### `fn extract_variable`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_variable (node : & Node , content : & str) -> Option < PythonVariable >
```

<details>
<summary>Source</summary>

```rust
fn extract_variable(node: &Node, content: &str) -> Option<PythonVariable> {
    // Look for simple assignments like `x: int = 5` or `x = 5`
    let left = node.child_by_field_name("left")?;

    if left.kind() == "identifier" {
        let name = node_text(&left, content);
        let ty = node
            .child_by_field_name("type")
            .map(|n| node_text(&n, content));
        let value = node
            .child_by_field_name("right")
            .map(|n| node_text(&n, content));

        return Some(PythonVariable {
            name,
            ty,
            value,
            docstring: None,
        });
    }

    None
}
```

</details>



### `fn extract_string_content`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_string_content (node : & Node , content : & str) -> Option < String >
```

<details>
<summary>Source</summary>

```rust
fn extract_string_content(node: &Node, content: &str) -> Option<String> {
    let text = node_text(node, content);

    // Remove string delimiters (""", ''', ", ')
    let trimmed = if text.starts_with("\"\"\"") || text.starts_with("'''") {
        text.trim_start_matches("\"\"\"")
            .trim_start_matches("'''")
            .trim_end_matches("\"\"\"")
            .trim_end_matches("'''")
    } else if text.starts_with('"') || text.starts_with('\'') {
        text.trim_start_matches('"')
            .trim_start_matches('\'')
            .trim_end_matches('"')
            .trim_end_matches('\'')
    } else {
        &text
    };

    // Also handle raw strings and f-strings
    let trimmed = trimmed
        .trim_start_matches('r')
        .trim_start_matches('f')
        .trim_start_matches('b');

    if trimmed.is_empty() {
        None
    } else {
        // Dedent the docstring content
        Some(dedent(trimmed))
    }
}
```

</details>



### `fn dedent`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn dedent (text : & str) -> String
```

Remove common leading whitespace from all lines in a string. This is similar to Python's textwrap.dedent().

<details>
<summary>Source</summary>

```rust
fn dedent(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    if lines.is_empty() {
        return text.to_string();
    }

    // Find the minimum indentation (ignoring empty lines and the first line which may not be indented)
    let min_indent = lines
        .iter()
        .skip(1) // Skip first line which often has no indentation
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);

    if min_indent == 0 {
        return text.to_string();
    }

    // Remove that much indentation from each line (except the first)
    let mut result = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            result.push(line.trim().to_string());
        } else if line.len() >= min_indent {
            result.push(line[min_indent..].to_string());
        } else {
            result.push(line.trim().to_string());
        }
    }
    result.join("\n")
}
```

</details>



### `fn extract_source_text`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_source_text (node : & Node , content : & str) -> String
```

<details>
<summary>Source</summary>

```rust
fn extract_source_text(node: &Node, content: &str) -> String {
    let start = node.start_byte();
    let end = node.end_byte();
    content[start..end].to_string()
}
```

</details>



### `fn node_text`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn node_text (node : & Node , content : & str) -> String
```

<details>
<summary>Source</summary>

```rust
fn node_text(node: &Node, content: &str) -> String {
    let start = node.start_byte();
    let end = node.end_byte();
    content[start..end].to_string()
}
```

</details>



