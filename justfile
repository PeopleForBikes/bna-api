# Set Just options.

set positional-arguments := true
set dotenv-load := true

# Define variables.

dbml := "docs/database.dbml"
entites := "entity/src/entities"
sql_dump := "docs/database.sql"

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

# Dump database.
db-dump:
  pg_dump -d $DATABASE_URL --schema-only > {{ sql_dump }}

# Dump database and convert it to dbml.
db-to-dbml: db-dump dbml-from-sql dbml-svg

# Generate models
db-generate-models:
    rm -fr  {{ entites }}
    sea-orm-cli generate entity \
      -o {{ entites }} \
      --with-serde both \
      --date-time-crate chrono


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
  npx -y --package=@dbml/cli dbml2sql --postgres {{ dbml }} -o {{ sql_dump }}

# Convert PostgreSQL dump to dbml.
dbml-from-sql:
  sql2dbml {{ sql_dump }} --postgres -o {{ dbml }}

# Generate the SVG diagram from dbml.
dbml-svg:
  npx -y --package=@softwaretechnik/dbml-renderer dbml-renderer -i {{ dbml }} -o docs/database.svg

# Spin up Docker Compose.
compose-up:
  docker compose up -d

# Tear down Docker Compose.
compose-down:
  docker compose down
  docker compose rm -sfv
  docker volume rm -f bna-api_postgres

# Generate BNA API client.
generate-client:
    RUST_LOG="debug" \
      cargo progenitor \
      -i ./openapi.yaml \
      -o bnaclient \
      -n bnaclient \
      --interface builder \
      --license-name MIT \
      -v 1.0.0

# Start the Axum server locally in watch mode.
debug-axum:
  BNA_API_LOG_LEVEL=debug \
  BNA_API_STANDALONE=1 \
  DATABASE_URL=postgresql://postgres:postgres@localhost:5432/postgres \
  cargo watch -x \
    'run -p lambdas --bin axumed'
