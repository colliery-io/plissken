# Starlight Sites

plissken can write the API reference into an [Astro Starlight](https://starlight.astro.build)
site. The template is `starlight`. It works with any Starlight site, and with the
Colliery theme [starlight-aurora](https://github.com/colliery-io/starlight-aurora).

## What plissken writes

| Output | Description |
|--------|-------------|
| One `.md` page for each module | Python modules at the top of the output folder, Rust modules in `rust/` |
| `_sidebar.mjs` | A Starlight sidebar group of the Python and Rust modules |

Each page has front matter:

```yaml
---
title: "mycrate::config"
description: "The first line of the module docs."
sidebar:
  order: 3
  hidden: true
---
```

- Starlight shows `title` as the page heading.
- `sidebar.hidden` keeps the pages out of an `autogenerate` group. The pages are
  in the sidebar through `_sidebar.mjs`.

The pages are `.md`, not `.mdx`. They contain Markdown and a small amount of HTML
(badges, anchors and `<details>` for source code).

## Add the API reference to a starlight-aurora site

You need Node 22 or later, and a site made from the starlight-aurora starter
(`template/` in the starlight-aurora repository).

1. In `plissken.toml`, set the template and the output path:

    ```toml
    [output]
    format = "markdown"
    path = "docs/src/content/docs/reference/api"
    template = "starlight"
    ```

    The path must be a folder in `docs/src/content/docs/`. For a new project,
    `plissken init --template starlight` writes these values.

2. Render the pages:

    ```bash
    plissken render
    ```

3. In `docs/astro.config.mjs`, import the sidebar group:

    ```js
    import apiSidebar from './src/content/docs/reference/api/_sidebar.mjs';
    ```

4. Add the group to the sidebar. For example, put it in the Reference group of
   the starter:

    ```js
    sidebar: [
        // ...
        { label: 'Reference', items: [{ autogenerate: { directory: 'reference' } }, apiSidebar] },
        // ...
    ],
    ```

5. Build the site:

    ```bash
    cd docs && npm run build
    ```

To show the group with a different label, use `{ ...apiSidebar, label: 'API' }`.

Run `plissken render` again after each change to the code. Commit the generated
pages, or render them in the docs workflow before `npm run build`.

## Links and anchors

- Links between pages, and the links between Python and Rust items, are relative
  route links (for example `../../mypackage/#class.Task`). They work at `/` and
  under a GitHub Pages base path such as `/project/`.
- Each class, struct, enum, function and method has a fixed anchor id:

    | Item | Anchor |
    |------|--------|
    | Python class | `#class.Name` |
    | Rust struct | `#struct.Name` |
    | Rust enum | `#enum.Name` |
    | Function | `#fn.name` |
    | Method | `#Type.method` |

- Starlight skips a file whose name starts with `_`. plissken writes such a module
  (for example `mypackage._core`) as `mypackage/_core/index.md`. The route stays
  `mypackage/_core/`.

## Colors and code

- Badges use Starlight's color variables: a fill (`--sl-color-<hue>-low`) and text
  in the same hue (`--sl-color-<hue>-high`). starlight-aurora sets these from
  Aurora's tokens, so the badges follow the light, dark and hyper themes.
- Signatures, examples and source code are Markdown code fences. Starlight's code
  highlighter draws them in each theme.
- Code fences from Rust doc comments such as ```` ```ignore ```` or
  ```` ```rust,no_run ```` get the language `rust`.
