# Set Just options.

set positional-arguments := true
set dotenv-load := true

# Define variables.

entites := "entity/src/entities"

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

# Meta task running all the linters at once.
lint: lint-md lint-spellcheck

# Lint markown files.
lint-md:
    npx --yes markdownlint-cli2 "**/*.md" "#.venv" "#docs/themes" "#target"

# Check spelling.
lint-spellcheck:
    npx --yes cspell --no-progress --show-suggestions --show-context "**/*.md"

# Generate models
db-generate-models:
    rm -fr  {{ entites }}
    sea-orm-cli generate entity -o {{ entites }} --with-serde both --date-time-crate time


# Apply migrations and seed the database.
db-init: db-migrate db-seed

# Drop all tables and re-apply the migrations.
db-fresh:
    sea-orm-cli migrate fresh

# Run the migrations.
db-migrate:
    sea-orm-cli migrate up

# Drop the tables, apply the migrations, generate the models and seed the database.
db-reset: db-fresh db-generate-models db-seed

# Drop the table and seed the database.
db-reseed: db-fresh db-seed

# Seed the database from a City Ratings CSV file.
db-seed:
    cargo run --example seeder

# Generate PostgreSQL dump from dbml.
dbml-sql:
  npx -y --package=@dbml/cli dbml2sql --postgres docs/database.dbml -o docs/database.sql

# Generate the SVG diagram from dbml.
dbml-svg:
  npx -y --package=@softwaretechnik/dbml-renderer dbml-renderer -i docs/database.dbml -o docs/database.svg

# Spins up Docker Compose.
compose-up:
  docker compose up -d

# Tears down Docker Compose.
compose-down:
  docker compose down
  docker compose rm -sfv
  docker volume rm -f bna-api_postgres
