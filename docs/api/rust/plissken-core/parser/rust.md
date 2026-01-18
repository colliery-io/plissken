# rust <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Rust source code parser using syn

## Structs

### `struct RustParser`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


#### Methods

##### `new` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn new () -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn new() -> Self {
        Self
    }
```

</details>



##### `parse_file` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn parse_file (& self , path : & Path) -> crate :: error :: Result < RustModule >
```

Parse a Rust source file.

**Raises:**

| Exception | Description |
|-----------|-------------|
| `Error` | Returns `PlisskenError::FileRead` if the file cannot be read, `PlisskenError::Parse` if the Rust syntax is invalid. |


<details>
<summary>Source</summary>

```rust
    pub fn parse_file(&self, path: &Path) -> crate::error::Result<RustModule> {
        use crate::error::PlisskenError;

        let content = std::fs::read_to_string(path)
            .map_err(|e| PlisskenError::file_read(path, e))?;
        self.parse_str(&content, path)
    }
```

</details>



##### `parse_str` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn parse_str (& self , content : & str , path : & Path) -> crate :: error :: Result < RustModule >
```

Parse Rust source from a string.

**Raises:**

| Exception | Description |
|-----------|-------------|
| `Error` | Returns `PlisskenError::Parse` if the Rust syntax is invalid. |


<details>
<summary>Source</summary>

```rust
    pub fn parse_str(&self, content: &str, path: &Path) -> crate::error::Result<RustModule> {
        use crate::error::PlisskenError;

        let syntax = syn::parse_file(content).map_err(|e| PlisskenError::Parse {
            language: "Rust".into(),
            path: path.to_path_buf(),
            line: Some(e.span().start().line),
            message: e.to_string(),
        })?;

        // Extract module doc comment from inner attributes
        let doc_comment = extract_inner_doc_comments(&syntax.attrs);

        // Extract items
        let items = syntax
            .items
            .iter()
            .filter_map(|item| self.extract_item(item, content, path))
            .collect();

        Ok(RustModule {
            path: path.display().to_string(),
            doc_comment,
            items,
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



##### `extract_item` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_item (& self , item : & Item , content : & str , path : & Path) -> Option < RustItem >
```

<details>
<summary>Source</summary>

```rust
    fn extract_item(&self, item: &Item, content: &str, path: &Path) -> Option<RustItem> {
        match item {
            Item::Struct(s) => Some(RustItem::Struct(self.extract_struct(s, content, path))),
            Item::Enum(e) => Some(RustItem::Enum(self.extract_enum(e, content, path))),
            Item::Fn(f) => Some(RustItem::Function(self.extract_function(f, content, path))),
            Item::Trait(t) => Some(RustItem::Trait(self.extract_trait(t, content, path))),
            Item::Impl(i) => Some(RustItem::Impl(self.extract_impl(i, content, path))),
            Item::Const(c) => Some(RustItem::Const(self.extract_const(c, content, path))),
            Item::Type(t) => Some(RustItem::TypeAlias(
                self.extract_type_alias(t, content, path),
            )),
            _ => None,
        }
    }
```

</details>



##### `extract_struct` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_struct (& self , s : & ItemStruct , content : & str , path : & Path) -> RustStruct
```

<details>
<summary>Source</summary>

```rust
    fn extract_struct(&self, s: &ItemStruct, content: &str, path: &Path) -> RustStruct {
        let span = get_source_span(
            &s.struct_token.span,
            &s.semi_token.map(|t| t.span).unwrap_or_else(|| {
                // For structs with braces, find the closing brace
                s.fields.span()
            }),
            content,
            path,
        );

        RustStruct {
            name: s.ident.to_string(),
            visibility: convert_visibility(&s.vis),
            doc_comment: extract_doc_comments(&s.attrs),
            generics: extract_generics(&s.generics),
            fields: extract_fields(&s.fields),
            derives: extract_derives(&s.attrs),
            pyclass: extract_pyclass(&s.attrs),
            source: span,
        }
    }
```

</details>



##### `extract_enum` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_enum (& self , e : & ItemEnum , content : & str , path : & Path) -> RustEnum
```

<details>
<summary>Source</summary>

```rust
    fn extract_enum(&self, e: &ItemEnum, content: &str, path: &Path) -> RustEnum {
        let span = get_source_span(
            &e.enum_token.span,
            &e.brace_token.span.close(),
            content,
            path,
        );

        RustEnum {
            name: e.ident.to_string(),
            visibility: convert_visibility(&e.vis),
            doc_comment: extract_doc_comments(&e.attrs),
            generics: extract_generics(&e.generics),
            variants: e
                .variants
                .iter()
                .map(|v| RustVariant {
                    name: v.ident.to_string(),
                    doc_comment: extract_doc_comments(&v.attrs),
                    fields: extract_fields(&v.fields),
                })
                .collect(),
            source: span,
        }
    }
```

</details>



##### `extract_function` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_function (& self , f : & ItemFn , content : & str , path : & Path) -> RustFunction
```

<details>
<summary>Source</summary>

```rust
    fn extract_function(&self, f: &ItemFn, content: &str, path: &Path) -> RustFunction {
        // Get the end of the function block
        let block_end = f.block.brace_token.span.close();
        extract_function_common(
            &f.sig.ident.to_string(),
            &f.vis,
            &f.attrs,
            &f.sig,
            Some(&block_end),
            content,
            path,
        )
    }
```

</details>



##### `extract_trait` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_trait (& self , t : & ItemTrait , content : & str , path : & Path) -> RustTrait
```

<details>
<summary>Source</summary>

```rust
    fn extract_trait(&self, t: &ItemTrait, content: &str, path: &Path) -> RustTrait {
        let span = get_source_span(
            &t.trait_token.span,
            &t.brace_token.span.close(),
            content,
            path,
        );

        let bounds = if t.supertraits.is_empty() {
            None
        } else {
            Some(
                t.supertraits
                    .iter()
                    .map(|b| b.to_token_stream().to_string())
                    .collect::<Vec<_>>()
                    .join(" + "),
            )
        };

        let associated_types = t
            .items
            .iter()
            .filter_map(|item| {
                if let TraitItem::Type(ty) = item {
                    Some(RustAssociatedType {
                        name: ty.ident.to_string(),
                        doc_comment: extract_doc_comments(&ty.attrs),
                        generics: extract_generics(&ty.generics),
                        bounds: if ty.bounds.is_empty() {
                            None
                        } else {
                            Some(
                                ty.bounds
                                    .iter()
                                    .map(|b| b.to_token_stream().to_string())
                                    .collect::<Vec<_>>()
                                    .join(" + "),
                            )
                        },
                    })
                } else {
                    None
                }
            })
            .collect();

        let methods = t
            .items
            .iter()
            .filter_map(|item| {
                if let TraitItem::Fn(f) = item {
                    // Trait methods may or may not have a default implementation
                    let block_end = f
                        .default
                        .as_ref()
                        .map(|block| block.brace_token.span.close());
                    Some(extract_function_common(
                        &f.sig.ident.to_string(),
                        &SynVisibility::Inherited,
                        &f.attrs,
                        &f.sig,
                        block_end.as_ref(),
                        content,
                        path,
                    ))
                } else {
                    None
                }
            })
            .collect();

        RustTrait {
            name: t.ident.to_string(),
            visibility: convert_visibility(&t.vis),
            doc_comment: extract_doc_comments(&t.attrs),
            generics: extract_generics(&t.generics),
            bounds,
            associated_types,
            methods,
            source: span,
        }
    }
```

</details>



##### `extract_impl` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_impl (& self , i : & ItemImpl , content : & str , path : & Path) -> RustImpl
```

<details>
<summary>Source</summary>

```rust
    fn extract_impl(&self, i: &ItemImpl, content: &str, path: &Path) -> RustImpl {
        let span = get_source_span(
            &i.impl_token.span,
            &i.brace_token.span.close(),
            content,
            path,
        );

        let trait_ = i
            .trait_
            .as_ref()
            .map(|(_, path, _)| path.to_token_stream().to_string());

        let where_clause = i
            .generics
            .where_clause
            .as_ref()
            .map(|w| w.to_token_stream().to_string());

        let pymethods = i.attrs.iter().any(|attr| attr.path().is_ident("pymethods"));

        let methods = i
            .items
            .iter()
            .filter_map(|item| {
                if let ImplItem::Fn(f) = item {
                    // Get the end of the method block
                    let block_end = f.block.brace_token.span.close();
                    Some(extract_function_common(
                        &f.sig.ident.to_string(),
                        &f.vis,
                        &f.attrs,
                        &f.sig,
                        Some(&block_end),
                        content,
                        path,
                    ))
                } else {
                    None
                }
            })
            .collect();

        RustImpl {
            generics: extract_generics(&i.generics),
            target: i.self_ty.to_token_stream().to_string(),
            trait_,
            where_clause,
            methods,
            pymethods,
            source: span,
        }
    }
```

</details>



##### `extract_const` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_const (& self , c : & ItemConst , content : & str , path : & Path) -> RustConst
```

<details>
<summary>Source</summary>

```rust
    fn extract_const(&self, c: &ItemConst, content: &str, path: &Path) -> RustConst {
        let span = get_source_span(&c.const_token.span, &c.semi_token.span, content, path);

        RustConst {
            name: c.ident.to_string(),
            visibility: convert_visibility(&c.vis),
            doc_comment: extract_doc_comments(&c.attrs),
            ty: c.ty.to_token_stream().to_string(),
            value: Some(c.expr.to_token_stream().to_string()),
            source: span,
        }
    }
```

</details>



##### `extract_type_alias` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_type_alias (& self , t : & ItemType , content : & str , path : & Path) -> RustTypeAlias
```

<details>
<summary>Source</summary>

```rust
    fn extract_type_alias(&self, t: &ItemType, content: &str, path: &Path) -> RustTypeAlias {
        let span = get_source_span(&t.type_token.span, &t.semi_token.span, content, path);

        RustTypeAlias {
            name: t.ident.to_string(),
            visibility: convert_visibility(&t.vis),
            doc_comment: extract_doc_comments(&t.attrs),
            generics: extract_generics(&t.generics),
            ty: t.ty.to_token_stream().to_string(),
            source: span,
        }
    }
```

</details>





## Functions

### `fn convert_visibility`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn convert_visibility (vis : & SynVisibility) -> Visibility
```

<details>
<summary>Source</summary>

```rust
fn convert_visibility(vis: &SynVisibility) -> Visibility {
    match vis {
        SynVisibility::Public(_) => Visibility::Public,
        SynVisibility::Restricted(r) => {
            let path = r.path.to_token_stream().to_string();
            if path == "crate" {
                Visibility::PubCrate
            } else if path == "super" {
                Visibility::PubSuper
            } else {
                Visibility::Private
            }
        }
        SynVisibility::Inherited => Visibility::Private,
    }
}
```

</details>



### `fn extract_doc_comments`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_doc_comments (attrs : & [Attribute]) -> Option < String >
```

<details>
<summary>Source</summary>

```rust
fn extract_doc_comments(attrs: &[Attribute]) -> Option<String> {
    let docs: Vec<String> = attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc")
                && let Meta::NameValue(nv) = &attr.meta
                && let syn::Expr::Lit(lit) = &nv.value
                && let syn::Lit::Str(s) = &lit.lit
            {
                return Some(s.value());
            }
            None
        })
        .collect();

    if docs.is_empty() {
        None
    } else {
        // Join doc lines and trim leading space from each line
        Some(
            docs.iter()
                .map(|s| s.strip_prefix(' ').unwrap_or(s))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}
```

</details>



### `fn extract_inner_doc_comments`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_inner_doc_comments (attrs : & [Attribute]) -> Option < String >
```

<details>
<summary>Source</summary>

```rust
fn extract_inner_doc_comments(attrs: &[Attribute]) -> Option<String> {
    let docs: Vec<String> = attrs
        .iter()
        .filter_map(|attr| {
            // Inner doc comments have style = Inner
            if attr.path().is_ident("doc")
                && let Meta::NameValue(nv) = &attr.meta
                && let syn::Expr::Lit(lit) = &nv.value
                && let syn::Lit::Str(s) = &lit.lit
            {
                return Some(s.value());
            }
            None
        })
        .collect();

    if docs.is_empty() {
        None
    } else {
        Some(
            docs.iter()
                .map(|s| s.strip_prefix(' ').unwrap_or(s))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}
```

</details>



### `fn extract_generics`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_generics (generics : & Generics) -> Option < String >
```

<details>
<summary>Source</summary>

```rust
fn extract_generics(generics: &Generics) -> Option<String> {
    if generics.params.is_empty() {
        return None;
    }

    let params: Vec<String> = generics
        .params
        .iter()
        .map(|p| match p {
            GenericParam::Type(t) => {
                let mut s = t.ident.to_string();
                if !t.bounds.is_empty() {
                    s.push_str(": ");
                    s.push_str(
                        &t.bounds
                            .iter()
                            .map(|b| b.to_token_stream().to_string())
                            .collect::<Vec<_>>()
                            .join(" + "),
                    );
                }
                s
            }
            GenericParam::Lifetime(l) => l.to_token_stream().to_string(),
            GenericParam::Const(c) => {
                format!("const {}: {}", c.ident, c.ty.to_token_stream())
            }
        })
        .collect();

    Some(format!("<{}>", params.join(", ")))
}
```

</details>



### `fn extract_fields`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_fields (fields : & Fields) -> Vec < RustField >
```

<details>
<summary>Source</summary>

```rust
fn extract_fields(fields: &Fields) -> Vec<RustField> {
    match fields {
        Fields::Named(named) => named
            .named
            .iter()
            .map(|f| RustField {
                name: f.ident.as_ref().map(|i| i.to_string()).unwrap_or_default(),
                ty: f.ty.to_token_stream().to_string(),
                visibility: convert_visibility(&f.vis),
                doc_comment: extract_doc_comments(&f.attrs),
            })
            .collect(),
        Fields::Unnamed(unnamed) => unnamed
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, f)| RustField {
                name: format!("{}", i),
                ty: f.ty.to_token_stream().to_string(),
                visibility: convert_visibility(&f.vis),
                doc_comment: extract_doc_comments(&f.attrs),
            })
            .collect(),
        Fields::Unit => vec![],
    }
}
```

</details>



### `fn extract_derives`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_derives (attrs : & [Attribute]) -> Vec < String >
```

<details>
<summary>Source</summary>

```rust
fn extract_derives(attrs: &[Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("derive")
                && let Meta::List(list) = &attr.meta
            {
                let tokens = list.tokens.to_string();
                return Some(
                    tokens
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<_>>(),
                );
            }
            None
        })
        .flatten()
        .collect()
}
```

</details>



### `fn extract_pyclass`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_pyclass (attrs : & [Attribute]) -> Option < PyClassMeta >
```

<details>
<summary>Source</summary>

```rust
fn extract_pyclass(attrs: &[Attribute]) -> Option<PyClassMeta> {
    for attr in attrs {
        if attr.path().is_ident("pyclass") {
            let mut meta = PyClassMeta::new();

            if let Meta::List(list) = &attr.meta {
                let tokens = list.tokens.to_string();
                for part in tokens.split(',') {
                    let part = part.trim();
                    if let Some(name) = part.strip_prefix("name") {
                        let name = name.trim_start_matches([' ', '=']);
                        let name = name.trim_matches('"');
                        meta.name = Some(name.to_string());
                    } else if let Some(module) = part.strip_prefix("module") {
                        let module = module.trim_start_matches([' ', '=']);
                        let module = module.trim_matches('"');
                        meta.module = Some(module.to_string());
                    }
                }
            }

            return Some(meta);
        }
    }
    None
}
```

</details>



### `fn extract_pyfunction`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_pyfunction (attrs : & [Attribute]) -> Option < PyFunctionMeta >
```

<details>
<summary>Source</summary>

```rust
fn extract_pyfunction(attrs: &[Attribute]) -> Option<PyFunctionMeta> {
    let mut meta = PyFunctionMeta::new();
    let mut found = false;

    for attr in attrs {
        if attr.path().is_ident("pyfunction") {
            found = true;
            if let Meta::List(list) = &attr.meta {
                let tokens = list.tokens.to_string();
                for part in tokens.split(',') {
                    let part = part.trim();
                    if let Some(name) = part.strip_prefix("name") {
                        let name = name.trim_start_matches([' ', '=']);
                        let name = name.trim_matches('"');
                        meta.name = Some(name.to_string());
                    }
                }
            }
        } else if attr.path().is_ident("pyo3")
            && let Meta::List(list) = &attr.meta
        {
            let tokens = list.tokens.to_string();
            if let Some(sig_start) = tokens.find("signature")
                && let Some(eq_pos) = tokens[sig_start..].find('=')
            {
                let sig = tokens[sig_start + eq_pos + 1..].trim();
                meta.signature = Some(sig.to_string());
            }
        }
    }

    if found || meta.signature.is_some() {
        Some(meta)
    } else {
        None
    }
}
```

</details>



### `fn extract_function_common`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_function_common (name : & str , vis : & SynVisibility , attrs : & [Attribute] , sig : & syn :: Signature , block_end : Option < & proc_macro2 :: Span > , content : & str , path : & Path ,) -> RustFunction
```

<details>
<summary>Source</summary>

```rust
fn extract_function_common(
    name: &str,
    vis: &SynVisibility,
    attrs: &[Attribute],
    sig: &syn::Signature,
    block_end: Option<&proc_macro2::Span>,
    content: &str,
    path: &Path,
) -> RustFunction {
    let signature_str = sig.to_token_stream().to_string();

    let params: Vec<RustParam> = sig
        .inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(r) => RustParam {
                name: "self".to_string(),
                ty: if r.mutability.is_some() {
                    "&mut self".to_string()
                } else if r.reference.is_some() {
                    "&self".to_string()
                } else {
                    "self".to_string()
                },
                default: None,
            },
            FnArg::Typed(t) => {
                let name = if let Pat::Ident(ident) = &*t.pat {
                    ident.ident.to_string()
                } else {
                    t.pat.to_token_stream().to_string()
                };
                RustParam {
                    name,
                    ty: t.ty.to_token_stream().to_string(),
                    default: None,
                }
            }
        })
        .collect();

    let return_type = match &sig.output {
        ReturnType::Default => None,
        ReturnType::Type(_, ty) => Some(ty.to_token_stream().to_string()),
    };

    // Get span from fn keyword to end of block (or signature if no block)
    let end_span = block_end.unwrap_or(&sig.fn_token.span);
    let span = get_source_span(&sig.fn_token.span, end_span, content, path);

    RustFunction {
        name: name.to_string(),
        visibility: convert_visibility(vis),
        doc_comment: extract_doc_comments(attrs),
        generics: extract_generics(&sig.generics),
        signature_str,
        signature: RustFunctionSig {
            params,
            return_type,
        },
        is_async: sig.asyncness.is_some(),
        is_unsafe: sig.unsafety.is_some(),
        is_const: sig.constness.is_some(),
        pyfunction: extract_pyfunction(attrs),
        source: span,
    }
}
```

</details>



### `fn get_source_span`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn get_source_span (start : & proc_macro2 :: Span , end : & proc_macro2 :: Span , content : & str , path : & Path ,) -> SourceSpan
```

<details>
<summary>Source</summary>

```rust
fn get_source_span(
    start: &proc_macro2::Span,
    end: &proc_macro2::Span,
    content: &str,
    path: &Path,
) -> SourceSpan {
    let start_line = start.start().line;
    let end_line = end.end().line;

    // Extract source text
    let lines: Vec<&str> = content.lines().collect();
    let source = if start_line > 0 && end_line <= lines.len() {
        lines[start_line - 1..end_line].join("\n")
    } else {
        String::new()
    };

    SourceSpan {
        location: SourceLocation {
            file: path.to_path_buf(),
            line_start: start_line,
            line_end: end_line,
        },
        source,
    }
}
```

</details>



