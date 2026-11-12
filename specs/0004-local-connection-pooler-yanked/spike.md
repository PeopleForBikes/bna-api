# Spike: Local Connection Pooler

## Status

Concluded — No-Go: a local PgBouncer that behaves like Neon's works, but it
gives contributors nothing new and guards against a bug class the API does not
risk today, so it is not worth a permanent service. See §6.

## 1. Intent

Staging and production reach their database through Neon's connection pooler, a
PgBouncer instance that Neon runs in transaction mode. Locally, and in the
integration job of CI, the API connects straight to PostgreSQL
(`docker-compose.yml`, `.github/workflows/integration.yml`). So a change that
only breaks behind a transaction-mode pooler (prepared statements, session
state, `SET`, advisory locks, `LISTEN`) passes every local check and fails only
once deployed.

The spike answers: **can a pooler that behaves like Neon's be added to the local
Docker Compose setup, and does the API's integration suite run through it?** It
also compares PgBouncer with two alternatives (PgDog and PgCat) and shows what
the setup would look like. The decision it informs is whether to specify a
pooler service for `docker-compose.yml` (and possibly for CI).

**Constraints that shaped scope**:

- The local pooler should behave like Neon's. Neon documents its PgBouncer
  settings, which users cannot change
  ([Neon connection pooling](https://neon.com/docs/connect/connection-pooling),
  read on 2026-11-12): `pool_mode=transaction`, `max_client_conn=10000`,
  `default_pool_size=0.9 * max_connections`, `max_prepared_statements=1000`,
  `query_wait_timeout=120`. Protocol-level prepared statements are supported "as
  of PgBouncer 1.22.0"; `SET`/`RESET`, `LISTEN`/`NOTIFY`, SQL-level `PREPARE`,
  `WITH HOLD` cursors, temporary tables, `LOAD` and session-level advisory locks
  are not. Neon routes schema migrations to the direct connection, not the
  pooled one (`-pooler` host).
- It must run as a container next to the existing `postgres` service, without
  changing the API's code.
- Experiments run outside the repository, in the agent's scratch directory, with
  a Docker Compose stack of their own (§4).

**Success bar**: `just test localhost` (the Hurl endpoint and scenario suites)
passes with the API connected through the pooler, with the same result as the
direct connection, over three runs in a row.

## 2. Current Baseline

- The API connects with `Database::connect(database_url)`
  (`lambdas/src/lib.rs:47`) and keeps the connection in the global `DB_CONN`
  (`lambdas/src/lib.rs:31`, `lambdas/src/lib.rs:58`). No `ConnectOptions` are
  set, so the sqlx 0.9.0 defaults apply:
  - a pool of up to 10 connections per process (`max_connections: 10`,
    `sqlx-core-0.9.0/src/pool/options.rs:151`); SeaORM 2.0.4 leaves it unset
    (`sea-orm-2.0.4/src/database/mod.rs:264`);
  - a cache of 100 prepared statements per connection
    (`statement_cache_capacity: 100`,
    `sqlx-postgres-0.9.0/src/options/mod.rs:95`);
  - named, protocol-level prepared statements (`sqlx_s_<id>`,
    `sqlx-postgres-0.9.0/src/io/mod.rs:34`). Behind a transaction-mode pooler
    they only work if the pooler tracks prepared statements (PgBouncer's
    `max_prepared_statements`, available since PgBouncer 1.21).
- Local database: `postgres:17.9` in `docker-compose.yml`. The integration job
  uses `ghcr.io/gui/pgrouting:18.1-trixie-postgis-3.6.1-pgrouting-4.0.0`, and
  `lambdas/tests/database-with-test-data.sql` needs PostgreSQL 18 (it sets
  `transaction_timeout`). The experiments use the CI image.
- The API uses none of the features Neon's pooler does not support: a search of
  `lambdas/src`, `entity/src`, `migration/src` and `examples` finds no `SET`,
  `set_config`, advisory lock, `LISTEN`/`NOTIFY`, temporary table, `WITH HOLD`
  cursor or SQL-level `PREPARE`. Its only explicit transaction
  (`lambdas/src/core/resource/ratings/adaptor.rs:68`) is compatible with
  transaction pooling.
- On a direct connection, the Hurl suite passes 12 of 12 files in 3 runs out of
  3, each in under a second (§5.1).

## 3. Candidates Considered

| Candidate                                                       | Verdict                             | Why                                                                                                                   |
| --------------------------------------------------------------- | ----------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| PgBouncer 1.26.1, transaction mode, Neon settings               | Tested, passes, see §5.2            | 12/12 files in 3 of 3 runs; migrations pass too. It is what Neon runs.                                                |
| PgBouncer 1.26.1, transaction mode, `max_prepared_statements=0` | Tested, fails as intended, see §5.3 | Control: 11 of 12 files fail on `sqlx_s_*` errors, so the suite detects a pooler incompatibility.                     |
| PgBouncer 1.26.1, session mode                                  | Tested, passes, see §5.4            | Passes, but Neon does not use session mode, so it would hide transaction-mode problems.                               |
| PgDog, transaction mode                                         | Tested, passes, see §5.5            | 12/12 in 3 of 3 runs, migrations pass; not what Neon runs.                                                            |
| PgCat 1.2.0, transaction mode                                   | Tested, rejected, see §5.6          | Passes only after a config fix and a restart before each run: it does not recover from terminated server connections. |

## 4. Evaluation Method

All runs were made on 2026-11-12 on the agent's machine (macOS, Docker 29.9.3,
Compose 5.7.0), from repository commit `6e42b5b`, with a Compose stack of their
own outside the repository. The runbook (`run.sh`) and the stack were kept in
the agent's scratch directory, not committed; this section records what they
did.

- **Database**:
  `ghcr.io/gui/pgrouting@sha256:8944935dc9f3d13374b4ec9ceb7cbacb075a050668f878df3caf908337481e55`
  (PostgreSQL 18.1, the image of the integration job), `max_connections=100`.
- **Poolers**, each its own service in front of the same database:
  - PgBouncer 1.26.1,
    `edoburu/pgbouncer@sha256:8638aa563607256d3c7440437534195f1066a92b68015d89ae9d6b935b444a6d`,
    run as `pgbouncer /cfg/pgbouncer.ini` with the configuration of §6.1 (Neon's
    settings, `default_pool_size = 90` for `0.9 * max_connections`). Three
    services differ only in `pool_mode` and `max_prepared_statements`.
  - PgDog `va913c73`,
    `ghcr.io/pgdogdev/pgdog@sha256:5a29642187528f64301638891acd7b295058e64145dd9e32b3c88098fbad6fbd`,
    `pooler_mode = "transaction"`, `prepared_statements = "extended"`,
    `pool_size = 90`.
  - PgCat 1.2.0,
    `ghcr.io/postgresml/pgcat@sha256:627761f6dcbc8c0d17809445d7441518e2452c6d4fbdcc2a9986871d9e3e1176`,
    `pool_mode = "transaction"`, `pool_size = 90`,
    `prepared_statements_cache_size = 1000` in the pool section (§5.6).
- **API**: `cargo build --release -p lambdas --bin axumed` (rustc 1.99.1), run
  with `BNA_API_STANDALONE=1` and `DATABASE_URL` pointing at the database or at
  one pooler, with no other change.
- **Each run**: drop and re-create the `bnatest` database
  (`DROP DATABASE ... WITH (FORCE)`), load
  `lambdas/tests/database-with-test-data.sql` through the direct connection,
  start the API, run `just test localhost` from `lambdas/tests` (just 1.59.0,
  Hurl 8.1.0, files in parallel), then count the API's log lines mentioning
  `prepared statement` and the database's backends. Three runs per candidate.
- **Migrations**: `sea-orm-cli migrate fresh -d migration -u <url>` (sea-orm-cli
  2.0.4) on an empty database, through each endpoint, then a count of the tables
  in `public`.

Two earlier runs were discarded because they never reached a pooler: Docker's
credential helper could not run in the agent's sandbox, so the pooler images
were not pulled, and the API timed out on every query (`PoolTimedOut`). The
runbook then used a Docker configuration of its own (public images only), and
checks that every service runs before testing.

## 5. Findings

Results of `just test localhost`, 12 Hurl files per run:

| Candidate                              | Run 1 | Run 2 | Run 3 | `prepared statement` log lines | Migrations     |
| -------------------------------------- | ----- | ----- | ----- | ------------------------------ | -------------- |
| E0 direct                              | 12/12 | 12/12 | 12/12 | 0                              | 22 tables      |
| E1 PgBouncer, transaction              | 12/12 | 12/12 | 12/12 | 0                              | 22 tables      |
| E2 PgBouncer, transaction, no tracking | 1/12  | 1/12  | 1/12  | 14, 10, 13                     | fails, 1 table |
| E3 PgBouncer, session                  | 12/12 | 12/12 | 12/12 | 0                              | 22 tables      |
| E4 PgDog, transaction                  | 12/12 | 12/12 | 12/12 | 0                              | 22 tables      |
| E5 PgCat, transaction (fixed config)   | 12/12 | 1/12  | 4/12  | 0                              | not run        |
| E5 PgCat, restarted before each run    | 12/12 | 12/12 | 12/12 | 0                              | not run        |

Every run took 0 to 2 seconds; at this resolution the pooler adds no measurable
time to the suite.

### 5.1 Direct connection: the baseline passes

12/12 files in each of the 3 runs; the API's pool opened its 10 connections (10
backends after each run), so the suite exercises concurrent connections.

### 5.2 PgBouncer, transaction mode with Neon's settings: passes

12/12 files in each of the 3 runs, no prepared-statement error, 8 to 9 server
connections used. PgBouncer logged 18
`got packet 'E' from server when not linked` warnings: they are the idle server
connections that the forced database drop between runs terminated; PgBouncer
discarded them and connected again. `sea-orm-cli migrate fresh` through it
created the 22 tables, although Neon routes migrations to the direct connection.

### 5.3 PgBouncer without prepared-statement tracking: the suite catches it

With `max_prepared_statements=0`, 11 of 12 files fail in every run; only
`endpoints/system.hurl`, which does not query the database, passes. The API logs
`prepared statement "sqlx_s_1" already exists` (SQLSTATE `42P05`) on
`GET /ratings`, `POST /cities` and `POST /cities/submissions`: sqlx's named
statements land on server connections they were not prepared on. Migrations fail
the same way after creating one table. So the suite detects this class of
incompatibility as soon as the API runs through a transaction-mode pooler; on a
direct connection it cannot.

### 5.4 PgBouncer, session mode: passes, but is not what Neon runs

12/12 in each run, 10 backends, like the direct connection. Session mode keeps
one server connection per client, so it would not reveal any of the
transaction-mode limitations Neon has.

### 5.5 PgDog: passes

12/12 in each run, 8 backends, no error; migrations pass. Its only warning is
`admin password has been randomly generated`. PgDog is a working alternative,
but not the pooler Neon runs.

### 5.6 PgCat: rejected

The first configuration put `prepared_statements = true` and
`prepared_statements_cache_size` in `[general]`; PgCat 1.2.0 ignored them
silently (`Prepared statements server cache size: 0` in its log) and the suite
failed with `prepared statement "sqlx_s_1" does not exist` (SQLSTATE `26000`).
With `prepared_statements_cache_size = 1000` in the pool section, run 1 passed
but runs 2 and 3 failed (1/12, 4/12) with `error receiving data from server`:
PgCat handed out server connections that the forced drop of the database had
terminated. Restarting PgCat before each run, all 3 runs pass. PgCat does not
recover from server connections ended under it, which Neon also does when it
suspends or restarts a compute, and its prepared-statement setting is easy to
get silently wrong.

## 6. Conclusion & Recommendation

**No-Go.** Adding a pooler to the local Docker Compose setup is possible:
PgBouncer 1.26.1 with Neon's settings runs the whole Hurl suite with the same
result and timing as the direct connection (§5.2), and so do migrations. But it
would not benefit contributors enough to be kept as a permanent service:

- **Nothing new for contributors.** A transaction-mode pooler adds no
  capability; it only forbids session features (`SET`, advisory locks,
  `LISTEN`/`NOTIFY`, temporary tables, SQL-level `PREPARE`). The API uses none
  of them (§2), and Neon's pooler has been transparent to the team in staging
  and production since the start. Locally it would do the same thing in a
  slightly different way.
- **A narrow guard.** Its only value is to catch, before merge, a change that
  starts using a session feature or a driver upgrade that breaks protocol-level
  prepared statements. The suite would catch it (§5.3), but this has not
  happened in the project's history, SeaORM and sqlx keep such constructs out of
  everyday code, and the team already runs the suite against staging, behind
  Neon's real pooler, before every major release, so such a bug cannot reach
  production unnoticed.
- **A permanent cost.** One more service and port, two database URLs (pooled for
  the API, direct for migrations and seeding, as Neon recommends), settings to
  keep in line with a Neon PgBouncer whose version is not published, and, for
  the guard to work in CI, a `docker run` step in the integration job, because
  GitHub service containers cannot mount a configuration file from the
  repository.

If a pooler-specific problem ever shows up, the recipe below reproduces Neon's
pooler locally in a few minutes, without a permanent change.

Among poolers, PgBouncer is the one to use, since Neon runs it; PgDog works too
(§5.5); PgCat is not suitable (§5.6). Session mode must not be used to mimic
Neon (§5.4).

### 6.1 Recipe: a Neon-like pooler on demand

A service next to the `postgres` one, with a pinned image (1.22.0 or later for
prepared statements; 1.26.1 was tested):

```yaml
pgbouncer:
  image: edoburu/pgbouncer@sha256:8638aa563607256d3c7440437534195f1066a92b68015d89ae9d6b935b444a6d
  entrypoint: ["pgbouncer", "/cfg/pgbouncer.ini"]
  volumes: ["./pgbouncer:/cfg"]
  ports: ["6432:6432"]
  depends_on: [postgres]
```

`pgbouncer/userlist.txt` holds `"postgres" "postgres"`, and
`pgbouncer/pgbouncer.ini`:

```ini
[databases]
* = host=postgres port=5432

[pgbouncer]
listen_addr = 0.0.0.0
listen_port = 6432
auth_type = scram-sha-256
auth_file = /cfg/userlist.txt
; Neon's settings.
pool_mode = transaction
max_client_conn = 10000
default_pool_size = 90 ; 0.9 * max_connections (100)
max_prepared_statements = 1000
query_wait_timeout = 120
; sqlx sends extra_float_digits at startup.
ignore_startup_parameters = extra_float_digits
```

Then run the API with
`DATABASE_URL=postgresql://postgres:postgres@localhost:6432/<db>` and keep the
direct URL for migrations and seeding. Setting `max_prepared_statements = 0`
reproduces a pooler without prepared-statement support (§5.3).

## 7. Still Open

- **Running the suite against staging automatically.** The team runs it against
  staging before every major release. Running it after each staging deployment
  (no workflow does today) would test Neon's real pooler on every change, and
  catch the bug class of §6 sooner than a local copy would. Not investigated
  here; a separate feature.
- The PgBouncer version Neon runs: its documentation only says prepared
  statements need 1.22.0 or later.
- `docker-compose.yml` runs `postgres:17.9`, while the integration job runs
  PostgreSQL 18.1 and `lambdas/tests/database-with-test-data.sql` needs 18
  (`transaction_timeout`). Outside this spike's question, noticed while setting
  up the experiments.

## 8. Remaining Work

None: the verdict is No-Go.
