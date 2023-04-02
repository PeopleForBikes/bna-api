# Set Just options.
set positional-arguments := true
set dotenv-load := true

# Define variables.
activate := venv_bin / "activate"
src_dir := "."
venv := ".venv"
venv_bin := venv / "bin"

# Meta task running ALL the CI tasks at onces.
ci: lint

# Meta tasks running all formatters at once.
fmt: fmt-md fmt-just

# Format the justfile.
fmt-just:
    just --fmt --unstable

# Format markdown files.
fmt-md:
    npx --yes prettier --write --prose-wrap always "**/*.md"

# Format python files.
fmt-python:
    poetry run isort .
    poetry run black {{ src_dir }}
    poetry run ruff check --fix {{ src_dir }}

# Format SASS files.
fmt-sass:
  npx --yes prettier --write "**/*.scss"

# Rebuild Sphinx documentation on changes, with live-reload in the browser
docs-autobuild:
    poetry run sphinx-autobuild docs/source docs/build/html

# Clean the docs
docs-clean:
    rm -fr docs/build

# Build the Sphinx documentation
docs-sphinx:
    cd docs && poetry run make html
    @echo
    @echo "Click this link to open the documentation in the browser:"
    @echo "  file://${PWD}/docs/build/html/index.html"
    @echo

# Build the Zola documentation
docs-zola:
    cd docs && zola build

# Meta task running all the linters at once.
lint: lint-md lint-spellcheck

# Lint markown files.
lint-md:
    npx --yes markdownlint-cli2 "**/*.md" "#.venv" "#docs/themes" "#target"

# Lint python files.
lint-python:
    poetry run isort --check .
    poetry run black --check {{ src_dir }}
    poetry run ruff check {{ src_dir }}

# Lint sass files.
lint-sass:
  npx --yes prettier --check "**/*.scss"

# Check spelling.
lint-spellcheck:
    npx --yes cspell --no-progress --show-suggestions --show-context "**/*.md"
