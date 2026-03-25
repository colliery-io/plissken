# How To: Integrate with CI/CD

## GitHub Actions

### Validate and Build Docs

```yaml
name: Documentation
on:
  push:
    branches: [main]
  pull_request:

jobs:
  docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install plissken
        run: |
          curl -fsSL https://raw.githubusercontent.com/colliery-io/plissken/main/install.sh | bash
          echo "$HOME/.local/bin" >> $GITHUB_PATH

      - name: Validate configuration
        run: plissken check

      - name: Generate API docs
        run: plissken render

      - name: Install MkDocs
        run: pip install mkdocs-material

      - name: Build site
        run: mkdocs build --strict
```

### Deploy to GitHub Pages

```yaml
      - name: Deploy to Pages
        if: github.ref == 'refs/heads/main'
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./site
```

### Check Docs on Pull Requests

Add validation as a required check:

```yaml
      - name: Check for uncommitted doc changes
        run: |
          plissken render
          git diff --exit-code docs/api/ || {
            echo "::error::API docs are out of date. Run 'plissken render' and commit the changes."
            exit 1
          }
```

## GitLab CI

```yaml
docs:
  image: python:3.12
  before_script:
    - curl -fsSL https://raw.githubusercontent.com/colliery-io/plissken/main/install.sh | bash
    - export PATH="$HOME/.local/bin:$PATH"
    - pip install mkdocs-material
  script:
    - plissken check
    - plissken render
    - mkdocs build --strict
  artifacts:
    paths:
      - site/
```

## Using cargo install in CI

If you prefer building from source (useful for ensuring version consistency):

```yaml
      - name: Install plissken
        run: cargo install plissken
```

This is slower than downloading the binary but ensures you get a
reproducible build.

## Caching

Cache the plissken binary between runs:

```yaml
      - uses: actions/cache@v4
        with:
          path: ~/.local/bin/plissken
          key: plissken-${{ runner.os }}-${{ hashFiles('Cargo.lock') }}
```
