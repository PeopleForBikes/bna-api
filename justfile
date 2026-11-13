# Set Just options.

set positional-arguments
set dotenv-load

# Define variables.

dbml := "docs/database.dbml"
entities := "entity/src/entities"
sql_dump := "docs/database.sql"
sql_dump_data := "lambdas/tests/database-with-test-data.sql"

# mise's directories, for the nono recipes (specs/0003-tool-manager).
mise_data := env("MISE_DATA_DIR", env("XDG_DATA_HOME", home_directory() / ".local/share") / "mise")
mise_config := env("MISE_CONFIG_DIR", env("XDG_CONFIG_HOME", home_directory() / ".config") / "mise")
mise_state := env("MISE_STATE_DIR", env("XDG_STATE_HOME", home_directory() / ".local/state") / "mise")

# Meta task running ALL the CI tasks at once.
ci: lint

# Meta tasks running all formatters and automatic fixes at once.
fmt: fmt-spellcheck fmt-md fmt-just

# Format the justfile.
fmt-just:
    just --fmt --unstable

# typos exits 2 when misspellings with several suggestions remain; just lint reports them.
# Fix the spelling mistakes that have a single correction.
fmt-spellcheck:
    typos --write-changes || [ $? -eq 2 ]

# rumdl exits 1 when findings it cannot fix remain; just lint reports them.
# Fix the fixable Markdown lint findings, then format the Markdown files.
fmt-md:
    rumdl check --no-cache --config .markdownlint.yml --fix . || [ $? -eq 1 ]
    oxfmt "**/*.md"

# Meta task running all the linters at once.
lint: lint-md lint-spellcheck

# Check the Markdown formatting, then lint the Markdown files.
lint-md:
    oxfmt --check "**/*.md"
    rumdl check --no-cache --config .markdownlint.yml .

# Check spelling in the whole repository.
lint-spellcheck:
    typos

# Generate the entities from the schema of the database.
db-generate-models:
    rm -fr  {{ entities }}
    sea-orm-cli generate entity \
      -o {{ entities }} \
      --with-serde both \
      --date-time-crate chrono

# Apply the pending migrations.
db-migrate:
    sea-orm-cli migrate up

# The dump holds the tables, the test data and the applied migrations, and loads
# into an empty schema only: everything in the public schema is dropped first.
# Replace the database with the test data, then apply the newer migrations.
db-seed: && db-migrate
    psql "${DATABASE_URL}" -v ON_ERROR_STOP=1 -q \
      -c 'SET client_min_messages = warning' \
      -c 'DROP SCHEMA public CASCADE' \
      -c 'CREATE SCHEMA public'
    psql "${DATABASE_URL}" -v ON_ERROR_STOP=1 -q -f {{ sql_dump_data }} > /dev/null

# Replace the database with the sample of examples/sample.csv instead of the test data.
db-seed-from-csv:
    sea-orm-cli migrate fresh
    cargo run --example seeder

# Generate the PostgreSQL schema from dbml.
dbml-sql:
    #!/usr/bin/env bash
    set -euo pipefail
    sql=$({{ just_executable() }} dbml-sql-print)
    printf '%s\n' "$sql" > {{ sql_dump }}

# Print the PostgreSQL schema generated from dbml.
# The output has no generation date, so it only changes with the schema.
[private]
dbml-sql-print:
    #!/usr/bin/env bash
    set -euo pipefail
    sql=$(npx -y --package=@dbml/cli@12.0.0 dbml2sql --postgres {{ dbml }} | sed '/^-- Generated at:/d')
    printf -- '-- Generated from {{ dbml }} by `just dbml-sql`. Do not edit.\n\n%s\n' "$sql"

# Builds a database from each on the server of DATABASE_URL and compares their
# schemas, apart from the two differences DBML cannot avoid: descending index
# columns, and foreign keys that dbml2sql makes deferrable.
# Check that the dbml matches the migrations and that the SQL is up to date.
dbml-check:
    #!/usr/bin/env bash
    set -euo pipefail
    server="${DATABASE_URL%/*}"
    from_dbml="dbml_check_${$}_dbml"
    from_migrations="dbml_check_${$}_migrations"
    tmp=$(mktemp -d)
    cleanup() {
      psql "$DATABASE_URL" -q -c "SET client_min_messages = warning" \
        -c "DROP DATABASE IF EXISTS $from_dbml" \
        -c "DROP DATABASE IF EXISTS $from_migrations" || true
      rm -rf "$tmp"
    }
    trap cleanup EXIT
    dump() {
      pg_dump -d "$server/$1" --schema-only --no-owner -T seaql_migrations \
        | grep -v -e '^--' -e '^$' -e '^SET ' -e '^SELECT pg_catalog' \
          -e '^\\restrict' -e '^\\unrestrict'
    }

    {{ just_executable() }} dbml-sql-print > "$tmp/spec.sql"
    if ! diff -q {{ sql_dump }} "$tmp/spec.sql" > /dev/null; then
      echo "{{ sql_dump }} is out of date: run 'just dbml-sql'." >&2
      exit 1
    fi

    psql "$DATABASE_URL" -qc "CREATE DATABASE $from_dbml" \
      -c "CREATE DATABASE $from_migrations"
    psql "$server/$from_dbml" -v ON_ERROR_STOP=1 -q -f "$tmp/spec.sql"
    cargo run -q -p migration -- up -u "$server/$from_migrations" > /dev/null

    dump "$from_dbml" | sed 's/ DEFERRABLE;$/;/' > "$tmp/dbml.sql"
    dump "$from_migrations" | sed 's/ DESC\([,)]\)/\1/g' > "$tmp/migrations.sql"
    if diff -u --label migrations --label {{ dbml }} "$tmp/migrations.sql" "$tmp/dbml.sql"; then
      echo "{{ dbml }} matches the migrations."
    else
      echo "{{ dbml }} does not match the migrations." >&2
      exit 1
    fi

# Generate the SVG diagram from dbml.
dbml-svg:
    npx -y --package=@softwaretechnik/dbml-renderer@1.0.31 dbml-renderer -i {{ dbml }} -o docs/database.svg

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
      -i ./openapi-3.0.yaml \
      -o bnaclient \
      -n bnaclient \
      --interface builder \
      --license-name MIT \
      -v "$(cargo pkgid -p bnaclient | sed 's/.*[#@]//')"

# Start the Axum server locally in watch mode.
debug-axum:
    bacon axum-debug

# Generate the OAS 3.1.x from the Axum source code.
generate-oas-31:
    BNA_API_GENERATE_ONLY=1 \
      cargo run \
      -p lambdas \
      --bin axumed

# Generate the OAS 3.0.x from the OAS 3.1.x.
generate-oas-30:
    npx -y @apiture/openapi-down-convert@0.14.2 --input openapi-3.1.yaml --output openapi-3.0.yaml

# Regenerate the OpenAPI specifications and the client.
regenerate-all: generate-oas-31 generate-oas-30 generate-client

# Regenerate the OpenAPI specifications and the client without updating Cargo.toml.
regenerate-all-no-cargo: regenerate-all
    git restore bnaclient/Cargo.toml

# Use nono sandbox to login with Claude.
nono-claude-login:
    nono run --profile nolabs-ai/claude --allow-launch-services -- claude "/login"

# Use nono sandbox for Claude -- restricted internet, skip-permissions, with the tools installed by mise.
nono-claude-danger profile="pfb-claude":
    mkdir -p "{{ mise_data }}" "{{ mise_config }}" "{{ mise_state }}"
    mise_bin="$(command -v mise || true)"; PATH="{{ mise_data }}/shims:$PATH" nono run \
    --allow-cwd \
    --profile {{ profile }} \
    --read "{{ mise_data }}" \
    --read "{{ mise_config }}" \
    --allow "{{ mise_state }}" \
    ${mise_bin:+--read-file "$mise_bin" --read-file "$(realpath "$mise_bin")"} \
    -- claude --dangerously-skip-permissions
