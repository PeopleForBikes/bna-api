# Spike: API Request Collection Beyond REST Client

## Status

Concluded — Conditional Go (2026-11-14; confirmed after Posting was reopened and
rejected, §1.3, §5.6): explore the API with Slumber, keep testing with Hurl and
add a small wrapper to send one Hurl entry from Zed or VS Code, provided a
hands-on trial of the Slumber TUI by a Zed user and a VS Code user confirms the
exploration loop. See §6.

## 1. Intent

Which tool, or method, should the team use to explore the BNA API and keep a
collection of requests, now that the VS Code "REST Client" extension by Huachao
Mao is unmaintained and part of the team works in Zed?

The team standardized on REST Client when everyone used VS Code, and keeps its
collection in `lambdas/requests.rest`. Two things changed: the extension has had
no release for more than four years, and the team now also uses other editors,
Zed first. Meanwhile the team adopted [Hurl] to test and validate the API
(`specs/0001-hurl-api-coverage`, `lambdas/tests/`). This spike informs the
choice of a replacement: an editor extension, a standalone tool, or a way of
working; ideally one that builds on the Hurl work.

**Constraints that shaped scope**:

- Works for Zed and VS Code users. A terminal tool used beside the editor is
  acceptable.
- Open source, usable without an account, cloud sync or sign-in.
- The collection is plain-text files committed to git, diffable and reviewable
  like code.
- Actively maintained.
- Installable through mise (`mise.toml`, `specs/0003-tool-manager`): a strong
  bonus, not a hard requirement.
- Experiments: CLI tools only, installed outside the repository; editor
  extensions and GUI applications are assessed from their documentation, not
  installed. Requests run against the local server only; the Cognito login is
  checked structurally or against a local stand-in, never against AWS.

**Must keep** the four capabilities the current collection relies on (§2):

1. Send one request of the collection and inspect the response (status, headers,
   body).
2. Chaining: capture a value from a response and reuse it in a later request.
3. The Cognito login with a software MFA code, to obtain the Bearer token the
   write endpoints need.
4. Environments: switch between localhost and staging without editing requests.

**Success bar** (agreed as a shortlist, not a full port): a ranked shortlist of
candidates that meet the constraints, each with evidence for the four
capabilities, from documentation and from quick smoke runs of the CLI tools
against the local server. Bonus: a candidate that reuses the Hurl format,
variables files or tooling.

### 1.1 Reframe: Hurl first, Cognito as a bonus

_Agreed with the user before the first experiment; supersedes must-have 3 and
the order of the candidates._ The Cognito login is not needed to explore the
API: authentication is disabled on the local server (`lambdas/tests/README.md`).
Replaying some requests of the collection against staging or production, with a
token, is a bonus. Hurl may already be the right tool, with only the ways of
using it for exploration missing, so Hurl is evaluated first and in depth
(§5.1), and the other candidates are measured against it.

The question becomes: **what does the team need, beyond Hurl 8.1 as already
pinned, to explore the API from Zed and VS Code, and does any other tool do it
better?** The must-haves are now 1, 2 and 4: send one request and inspect the
response, chaining, and environments.

### 1.2 Reframe: exploration ergonomics, not capabilities

_After §5.1, from the user; refines §1.1._ Hurl is excellent for testing the
API, but the team struggled to use it for exploring the API or building new
endpoints: saying "send `GET X`" quickly, looking through the response body and
headers, or running request NN of `smoke/public.hurl` on its own (they did not
manage the last one; §5.1 H6 shows why). Capabilities alone do not settle the
question: Hurl has them (§5.1). The yardstick for every candidate becomes the
**exploration loop**: how fast an ad-hoc or saved request is sent, how the
response and headers can be inspected, and whether any saved request can be sent
on its own. Reusing Hurl stays a bonus, not a requirement.

### 1.3 Reopened: Posting, a teammate's preference

_After §6 and the Slumber TUI trial, which passed; reopens the comparison of
§5.3 and §5.5._ A teammate likes [Posting], which §5.5 had assessed from its
documentation only, because PyPI is blocked in the sandbox. Posting is evaluated
against the same yardstick as Slumber (§1.2), with the experiments run by the
user outside the sandbox, and a hands-on TUI trial (§5.6). The specification in
`requirements.md` waits for the outcome. _Outcome_: rejected from its
documentation and the user's own install, without the experiments or the trial;
the verdict of §6 is unchanged.

## 2. Current Baseline

**The tool**: [REST Client] (`humao.rest-client`), VS Code only. Latest release
`v0.25.0`, published 2022-06-21; the repository was last pushed 2026-04-23
(`gh api repos/Huachao/vscode-restclient`, 2026-11-14). MIT.

**The collection**: `lambdas/requests.rest`, 236 lines, 24 requests, last
changed 2026-05-04 (`4dfa20f`). CI ignores it
(`.github/workflows/lambdas.yml:12`), so nothing checks that it still matches
the API. It relies on these REST Client features:

| Feature                         | Where                                                                                         |
| ------------------------------- | --------------------------------------------------------------------------------------------- |
| File variables                  | `lambdas/requests.rest:3-7`, `:143`                                                           |
| Host from the editor settings   | `lambdas/requests.rest:1` (commented out: `{{host}}` comes from each user's VS Code settings) |
| Secrets from `.env`             | `{{$dotenv USERNAME}}`, `PASSWORD`, `CLIENT_ID`, `lambdas/requests.rest:93-97`, `:111-114`    |
| Named requests and chaining     | `# @name cognito_auth` / `cognito_mfa`, `{{cognito_auth.response.body.Session}}`, `:84-121`   |
| Interactive prompt              | `# @prompt mfa_code software MFA code`, `lambdas/requests.rest:103`                           |
| Bearer token on write endpoints | `Authorization: Bearer {{cognito_access}}`, `lambdas/requests.rest:127`, `:148`, `:161-236`   |

Environments are not in the repository: REST Client reads them from each user's
VS Code settings, so the switch between localhost and staging is per user and
undocumented.

The collection has already drifted from the API: the `POST /cities/` body at
`lambdas/requests.rest:221-230` is invalid JSON (`"latitude": 39.46975 9` and a
trailing comma).

**Hurl today**: Hurl 8.1.0 is pinned in `mise.toml`. Thirteen `.hurl` files in
`lambdas/tests/{endpoints,scenario,smoke}` run with
`hurl --test --variables-file <env>.vars` (`lambdas/tests/justfile`), with one
variables file per environment (`localhost.vars`, `staging.vars`) and the token
passed as `HURL_VARIABLE_token` (`lambdas/tests/README.md`).

## 3. Candidates Considered

| Candidate                                             | Verdict                                       | Why                                                                                                                                                    |
| ----------------------------------------------------- | --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Hurl 8.1 + `hurl-at` wrapper + editor tasks           | Tested — see §5.1                             | Covers the three must-haves from Zed and VS Code; reuses the tests, variables files and `mise.toml`; needs a small wrapper                             |
| vscode-hurl-runner (VS Code)                          | Documentation only — see §5.1                 | Maintained (`v0.18.0`, 2026-10-25), runs the entry at the cursor and keeps captures; VS Code only                                                      |
| Zed extensions `http`, `hurl`, `kulala-http`, `bruno` | Documentation only — see §5.0                 | Syntax and LSP only; none sends requests, Zed tasks do                                                                                                 |
| httpyac (CLI + VS Code)                               | Rejected — see §5.2                           | Closest to REST Client, but no release since 2025-03-30; the sandbox blocked the smoke runs                                                            |
| Slumber 5.4.4 (TUI + CLI)                             | Tested — see §5.3                             | Best exploration loop: chaining that sends upstream requests itself, profiles, history, response queries, editor-agnostic; a second format beside Hurl |
| Bruno 4.3 (desktop app + CLI)                         | CLI tested, app from documentation — see §5.4 | Generates the collection from `openapi-3.1.yaml`; offline, files in git; the app is a GUI outside both editors and outside mise                        |
| Posting 2.11.2 / 3.0 beta (Python TUI)                | Rejected — see §5.6 (supersedes §5.5)         | Reopened at a teammate's request (§1.3); TUI only, chains through session scripts, one file per request, Python from PyPI                              |
| ATAC (Rust TUI)                                       | Documentation only — see §5.5                 | Same niche as Slumber; chains through JavaScript scripts rather than declared dependencies                                                             |
| Yaak (desktop app)                                    | Rejected before testing — see §5.5            | Prebuilt binaries need a paid licence for commercial use                                                                                               |
| kulala.nvim                                           | Rejected before testing                       | Neovim only; its Zed port sends requests through httpyac                                                                                               |
| Postman, Insomnia                                     | Rejected before testing                       | Need an account or cloud sync                                                                                                                          |
| Hoppscotch                                            | Rejected before testing                       | Collections live in its database, not in files                                                                                                         |
| JetBrains HTTP Client (`ijhttp`)                      | Rejected before testing                       | Not open source                                                                                                                                        |

## 4. Evaluation Method

Tool versions and maintenance data were read from GitHub
(`gh api repos/<owner>/<repo>` and `.../releases?per_page=1`) and npm
(`npm view`) on 2026-11-14. Zed extensions were found in the official registry,
`zed-industries/extensions` (`extensions.toml`, 6,492 lines, same date), by
name: `bruno`, `http`, `hurl`, `kulala-http`.

Smoke runs: macOS arm64, inside the nono sandbox (`pfb-claude` profile), against
the local API (`just compose-up`, `just db-seed`, `just debug-axum`, started by
the user outside the sandbox because the profile denies Docker's socket) loaded
with `lambdas/tests/database-with-test-data.sql`. Every tool was installed in
the agent's scratch directory, never in the repository:

| Tool      | Version | Installed with                                                   |
| --------- | ------- | ---------------------------------------------------------------- |
| Hurl      | 8.1.0   | already pinned in `mise.toml`                                    |
| httpyac   | 6.16.7  | `npm install --prefix <scratch> httpyac@6.16.7`                  |
| Bruno CLI | 4.3.0   | `npm install --prefix <scratch> @usebruno/cli@4.3.0`             |
| Slumber   | 5.4.4   | `cargo install --locked --root <scratch> slumber@5.4.4` (47.7 s) |

GitHub release downloads (`release-assets.githubusercontent.com`) and PyPI are
not reachable from the sandbox, so Slumber was built from crates.io and Posting
was not installed. Editor extensions and desktop applications were assessed from
their documentation only, as agreed. httpyac, Bruno and Slumber ran with `HOME`
set to a scratch folder, so they wrote no state under the user's home.

## 5. Findings

### 5.0 Editors — no Zed extension sends requests; a task runs a CLI

The Zed registry has four HTTP-related extensions (2026-11-14):

| Extension     | Source                           | Last push  | Provides                                           |
| ------------- | -------------------------------- | ---------- | -------------------------------------------------- |
| `http`        | `tie304/zed-http`                | 2025-11-11 | `.http` syntax                                     |
| `hurl`        | `tommy/zed-hurl`                 | 2024-11-07 | Hurl syntax, folds, indents                        |
| `kulala-http` | `HusuSama/zed-kulala-http`       | 2026-03-02 | `.http` syntax, completion (kulala-ls), formatting |
| `bruno`       | `ebina4yaka/bruno-zed-extension` | 2026-04-29 | `.bru` syntax, LSP, formatting                     |

None of them sends requests. The `kulala-http` README documents "send requests"
as a project task that runs a CLI at the cursor:
`httpyac send --line $ZED_ROW $ZED_FILE`. None ships a `runnables.scm`, the file
a Zed language extension uses to show a run button in the gutter.

So in Zed every candidate reduces to **a CLI run by a task** (`.zed/tasks.json`
in the repository, bound to a key). Zed's task documentation
(`zed-industries/zed`, `docs/src/tasks.md:87-98`) provides `ZED_FILE`, `ZED_ROW`
and `ZED_WORKTREE_ROOT`; the output goes to a terminal tab. VS Code tasks
provide `${file}` and `${lineNumber}`, and `pickString`/`promptString` inputs
(`microsoft/vscode-docs`, `docs/reference/variables-reference.md:19-29`,
`:177-189`), so one task can ask for the environment. The question for each
candidate is therefore: does its CLI run one request of a file, chosen by line,
with readable output?

### 5.1 Hurl 8.1 — has the capabilities, given a 29-line wrapper and tasks

All runs: Hurl 8.1.0 (`mise.toml`), macOS, against the local server
(`just compose-up`, `just db-seed`, `just debug-axum`, test data from
`lambdas/tests/database-with-test-data.sql`), with
`lambdas/tests/localhost.vars`. Scratch files only; the scratch collection is a
6-entry port of `lambdas/requests.rest` (GETs, a chain, a POST).

**H1 — Single entry, chaining, headers.** Hurl has no "run the request at line
N" option, but has `--from-entry N --to-entry N` (`hurl --help`).

| Run                                                           | Result                                                                  |
| ------------------------------------------------------------- | ----------------------------------------------------------------------- |
| `--from-entry 1 --to-entry 1`                                 | exit 0, body of entry 1 only                                            |
| `--from-entry 3 --to-entry 3`, entry 3 uses a capture of 2    | `error: Undefined variable ... you must set the variable first_country` |
| `--to-entry 3`                                                | exit 0, runs 1–3, prints only the body of entry 3 (the chained city)    |
| `-i --from-entry 1 --to-entry 1`                              | `HTTP/1.1 200`, the `link` and `x-*` pagination headers, then the body  |
| `-i --from-entry 5 --to-entry 5` (`POST /cities/submissions`) | `HTTP/1.1 201` and the created submission                               |

The first chain failed with `URL rejected: Malformed input` because the captured
country holds a space; captures need the `urlEncode` filter
(`first_country: jsonpath "$[0].country" urlEncode`), as the variables files
already store `United%20States`.

**H2 — Entry at the cursor.** `hurlfmt --out json` gives the entries but no line
numbers, so a wrapper maps a line to an entry by counting request lines
(`METHOD url`, skipping `HTTP` status lines and ` ``` ` bodies). The wrapper,
`hurl-at [--upto] FILE LINE [hurl options]`, is 29 lines of Bash and awk and
runs `hurl --from-entry E --to-entry E` (`--from-entry 1` with `--upto`). A
cursor on the comments or blank lines directly above a request selects that
request, since Hurl attaches those comments to it.

- Entry detection against Hurl's own parser: for the 13 files of `lambdas/tests`
  (260 entries), the method and URL sequence the wrapper sees equals the one
  `hurlfmt --out json` gives, for every file.
- Line mapping: for each of the 2,650 lines of those files, the entry the
  wrapper picks equals an independent Python model of the same rule; 0
  mismatches. The first version picked the previous entry from a comment line
  (and none at line 1) and none inside ` ``` ` bodies; both were fixed before
  this run.

**H3 — Output a task terminal would show.** Run in a pseudo-terminal
(`script -q /dev/null hurl-at ...`), Hurl pretty-prints and colours the JSON
body; with `-i`, status and headers come first. Run on the test files directly
(`hurl-at endpoints/cities.hurl 13 --variables-file localhost.vars`), it sends
that test request, asserts included: the test suites double as the collection.

**H4 — Editor tasks.** Drafts of `.zed/tasks.json` (send entry with localhost,
send entry with staging, send up to entry) and `.vscode/tasks.json` (one task
with a `pickString` environment input) are valid JSON. Running the exact command
a Zed task would run
(`hurl-at $ZED_FILE $ZED_ROW --variables-file lambdas/tests/localhost.vars` from
the worktree root, with `ZED_FILE` set to
`lambdas/tests/endpoints/usstates.hurl` and `ZED_ROW=2`) prints the coloured
list of states. Not run inside Zed or VS Code: installing or driving the editors
was outside the agreed scope.

**H5 — Captures that survive between runs.** REST Client remembers named
responses between sends; separate `hurl` processes do not. Hurl provides what a
wrapper needs to do the same:

- `--report-json DIR` writes every capture of the run
  (`entries[].captures[{name, value}]`) while the body still goes to stdout.
  Turned into `NAME=VALUE` lines (9-line Python script), they make a variables
  file.
- `--variables-file` can be given twice; the later file wins (`host` from the
  second file was used).
- With the saved captures as a second variables file, entry 3 ran on its own:
  exit 0, the chained city.
- A capture marked `redact` is refused with `-v` or `--very-verbose`
  (`redacted secret not authorized in verbose`) and appears as `fake_token=***`
  in the report. A token for the staging bonus therefore cannot travel through
  the report: a login step would have to write it to a git-ignored file passed
  with `--secrets-file` (Hurl redacts those values in logs).

**Environments** are the existing variables files:
`--variables-file lambdas/tests/localhost.vars` or `staging.vars`, already used
by `just test` (`lambdas/tests/justfile`). Write requests already send
`Authorization: Bearer {{token}}`, with the token from `HURL_VARIABLE_token`
(`lambdas/tests/README.md`), which also covers replaying requests against
staging once a token is at hand.

**VS Code extension (from its documentation, not installed)**:
[vscode-hurl-runner] (`jellydn/vscode-hurl-runner`, MIT, `v0.18.0` released
2026-10-25, 21 stars) runs the entry at the cursor, from the beginning to the
current entry, a selection or the file; selects an environment file; keeps
"global variables" from captures between runs; and shows the response in a
formatted view. Its author also maintains `hurl.nvim`. It removes the need for
the task in VS Code; Zed still needs the wrapper.

**H6 — Request NN of `smoke/public.hurl` alone** (the team's failed attempt,
§1.2). Each of its 16 entries run with
`hurl --variables-file localhost.vars --from-entry N --to-entry N smoke/public.hurl`:
12 exit 0; entries 3, 4, 11 and 13 exit 3 with `error: Undefined variable`,
because they use `rating_id`, `submission_id` or `fargate_price_id`, captured by
an earlier entry (`smoke/public.hurl:9-10`, `:42-43`, `:59-60`). A test file
chains its requests on purpose; running one of them alone needs either the
earlier entries (`--upto`, which prints only the last response) or saved
captures (H5). Hurl prints only the body by default; headers need `-i` and the
request itself `-v`.

**What it means**: Hurl as already pinned covers the three must-haves. Sending
one request and seeing its response needs the line-to-entry wrapper and a task
per editor (or vscode-hurl-runner in VS Code). Chaining works within a run, and
across runs with a wrapper that saves captures. Environments are the existing
variables files. It reuses the Hurl format, the test suites and the variables
files, and needs nothing beyond `mise.toml`. Missing compared with REST Client:
no response pane or run button in Zed (the output is a terminal tab), no
interactive prompt (values come from variables files, `--variable`, or a VS Code
`promptString` input), and the wrapper itself has to be written and maintained.

### 5.2 httpyac 6.16.7 — the drop-in for `.rest` files, but stalled

httpyac reads REST Client's `.rest` and JetBrains' `.http` formats, runs a
request by line (`httpyac send FILE --line N`), prints the exchange
(`-o exchange`) and offers an interactive picker (`--interactive`). It is what
the Zed `kulala-http` extension runs (§5.0), and has a VS Code extension.

**Smoke run, not completed**:
`httpyac send requests.rest --line 11 --var host=http://localhost:3000` stopped
with `EPERM: operation not permitted, scandir '/Users/rgreinhofer'`: httpyac
lists every parent folder of the `.http` file looking for environment files, and
the sandbox denies listing the home folder. A `.httpyac.json` root marker and a
scratch `HOME` did not avoid it. The user chose to assess it from documentation
rather than widen the sandbox.

**Maintenance**: no release since `6.16.7` on 2025-03-30 (npm `time.modified`
agrees); 5 commits to `AnWeber/httpyac` and 1 to `AnWeber/vscode-httpyac` since
2025-04-01; 86 and 43 open issues (`gh api`, 2026-11-14).

**What it means**: replacing an extension unmaintained for four years with one
that has had no release for 19 months repeats the problem. Rejected.

### 5.3 Slumber 5.4.4 — the best exploration loop, as a terminal app

[Slumber] is a Rust HTTP client with a TUI and a CLI; its collection is one YAML
file (`slumber.yml`), its request history a per-user SQLite database
(`~/Library/Application Support/slumber/state.sqlite`), never in the repository.
MIT; `v5.4.4` released 2026-10-27, last push 2026-11-13.

**S1 — Import.** `slumber import openapi openapi-3.0.yaml` (exit 0) gives 26
recipes, grouped by tag, and one profile per server (localhost, staging,
production). `slumber import rest requests.rest` fails with
`There is no path for this request!`; bisecting with one-line files showed it
needs the HTTP version on the request line (`GET url HTTP/1.1`), optional in
REST Client. With the versions added by `sed`, the import gives 24 recipes and a
profile holding the file variables, but names them `Request_0`…, flattens the
JSON bodies (`\r\n`, no indentation) and leaves the REST Client references
(`$dotenv`, `cognito_auth.response.body.Session`, `mfa_code`) as escaped text
(`{_{…}}`).

**S2 — Sending the imported OpenAPI recipes as they are**
(`slumber -f from-openapi.yml request <id> -p http://localhost:3000`):

| Recipe                   | Request sent                                               | Response                                                                |
| ------------------------ | ---------------------------------------------------------- | ----------------------------------------------------------------------- |
| `get_cities`             | `GET /cities?page=&page_size=&sort_by=&order_direction=`   | `400`, `page: cannot parse integer from empty string`                   |
| `get_us_states` (`-v`)   | `GET /usstates?page_size=&page=`                           | `400`; `-v` prints the request line, status and headers before the body |
| `post_cities_submission` | body from the spec examples, `consent: 'true'` as a string | `422`, `consent: invalid type: string "true", expected a boolean`       |

The OpenAPI import is a starting point that needs cleaning (empty query
parameters, numbers and booleans as strings), not a collection to use as is.

**S3 — Hand-written collection** (4 recipes, profiles `local` and `staging`):

- `get_first_city`, whose URL takes country, state and name from
  `response('list_cities', trigger='no_history') | jsonpath(...)`, run with an
  empty history: Slumber sent `list_cities` first, then
  `GET /cities/United%20States/Oklahoma/Bartlesville` → `200`. It URL-encodes
  the values itself (Hurl needed `urlEncode`, §5.1 H1).
- `trigger='always'` re-sends the upstream request each time; a duration
  (`'1d'`) re-sends it when the stored response is older, and `select()` turns a
  JSONPath array into a picker (`docs/src/user_guide/templates/examples.md` at
  `v5.4.4`).
- `-o host=http://127.0.0.1:3000` overrides a profile value from the CLI
  (`--dry-run` showed the new host).

This answers "run request NN alone" (§1.2) without a wrapper: a recipe declares
what it depends on, and Slumber sends the dependencies when needed.

**Staging bonus (structural only, no AWS call)**: the template functions of
`v5.4.4` include `env`, `prompt`, `sensitive`, `command`, `file`, `response`,
`response_header`, `jq` and `jsonpath` (`crates/core/src/render/functions.rs`).
The Cognito MFA flow of `lambdas/requests.rest:82-121` maps onto them: username
and client ID from `env()`, password and MFA code from `prompt()`, the session
and the token from `response(...) | jsonpath(...)`, with a duration trigger
(`'1h'`) so the login is only replayed when the token is old.

**TUI (from its documentation, `docs/src/user_guide/tui/` at `v5.4.4`)**: the
"primary use case", akin to Insomnia or Postman in a terminal. It reloads the
collection when the file is saved; the response pane has a query box that pipes
the body through any shell command (`jq`, `grep`); "Edit Recipe" opens the
recipe in the user's editor (`code --wait`; Zed accepts the `file:line:column`
form the docs rely on); several sessions can share a collection. Not run: a TUI
cannot be driven from the agent.

**Editors**: none needed. Slumber runs in the terminal panel of Zed or VS Code,
or beside them, and both editors validate and complete `slumber.yml` through
`yaml-language-server` with the collection's published JSON schema
(`# yaml-language-server: $schema=...`, `docs/src/user_guide/json_schema.md`).

**mise**: `cargo:slumber` builds from crates.io like `cargo:bacon` and
`cargo:sea-orm-cli` already in `mise.toml` (47.7 s here,
`cargo.binstall = false`). The GitHub releases publish SHA-256 checksums but no
attestations (`gh api repos/LucasPickering/slumber/attestations/<digest>` →
404).

**S4 — Bridge to Hurl.** `slumber generate curl <recipe> --execute-triggers`
prints the request as `curl`, with templates resolved; `hurlfmt --in curl` turns
a curl command into a Hurl entry:

- `GET` (one line): converted to
  `GET http://localhost:3000/cities/United%20States/Oklahoma/Bartlesville`.
- `POST` with a JSON body: Slumber prints several lines, which `hurlfmt` does
  not read (`Missing delimiter '`); joined on one line, it still fails because
  Hurl 8.1's curl import does not know curl's `--json` option
  (`unexpected argument '--json' found`).
- Without `--execute-triggers`, `generate` refuses to send upstream requests;
  CLI requests are only stored in the history with `--persist`.

So an explored request can seed a Hurl test for GETs, with the host and values
written literally; it is not a two-way bridge.

**What it means**: Slumber meets the three must-haves and is the only tested
candidate whose exploration loop matches §1.2: pick a recipe, send it, browse
and query the response and headers, with dependencies sent on demand, from a
terminal next to any editor. It costs a second format beside Hurl, its own
collection to keep up to date, and a TUI to learn.

### 5.4 Bruno 4.3 — collection generated from the spec, explored in an app

[Bruno] is an MIT desktop app (Electron) with a CLI. Its README: collections are
plain-text files in a folder, for git; "Bruno is offline-only. There are no
plans to add cloud-sync to Bruno, ever." Release `v4.3.0`, 2026-10-22; 48,397
stars.

**B1 — Import.**
`bru import openapi -s openapi-3.1.yaml -o <scratch> -n bna-api` (exit 0) wrote
26 request files (one per `operationId`, 26 in the spec), grouped by tag, with
one environment per server, in the YAML OpenCollection format.
`city/get_city.yml` is 4,228 lines: the `Country` enum became 812 disabled
parameters. The `POST /cities/submissions` body is a skeleton from the schema
with the right types (`"consent": false`).

**B2 — CLI.** `bru run usstate/get_us_states.yml --env "Local development API"`
→ `usstate/get_us_states (200 OK) - 18 ms` and a summary table; the body and
headers are not printed, but `-o result.json` holds them (status 200, `link` and
`x-*` headers, the body). The CLI is a runner for tests and CI, not an
exploration tool.

**App (from its documentation)**: not installed (GUI applications were out of
scope). It is where Bruno is explored: request tabs, response and headers views,
environments, scripting. It sits outside both editors, and mise cannot pin it
(the CLI could be `npm:@usebruno/cli`). No official VS Code extension was found
under the `usebruno` organisation; the Zed `bruno` extension gives syntax only
(§5.0).

**What it means**: Bruno's strength is generating the collection from the
OpenAPI specification, so it cannot drift: regenerate after `just generate`. It
suits people who prefer a GUI. It adds a desktop app per contributor that the
repository cannot pin.

### 5.5 Other candidates — from documentation

- _Superseded by §5.6._ **Posting** (`darrenburns/posting`, Apache-2.0, `2.11.2`
  on 2026-10-05, `v3.0.0-beta.1` on 2026-10-15): a Python TUI, YAML collections,
  environments, imports OpenAPI, Postman and Bruno, exports curl. Not installed:
  PyPI is blocked in the sandbox. It covers the same ground as Slumber; mise
  could pin it through `pipx:`/`uv`.
- **ATAC** (`Julien-cpsn/ATAC`, MIT, `v0.24.0` on 2026-10-11): a Rust TUI with
  JSON/YAML collections, committable environment files, Postman/cURL import, and
  chaining through pre- and post-request JavaScript scripts (its README's
  feature table), where Slumber declares dependencies in templates. Not
  installed.
- **Yaak** (`mountain-loop/yaak`, MIT source, `v2026.9.1` on 2026-11-01): a
  desktop app that can mirror workspaces to files for git. Its README: "The
  prebuilt binaries follow Yaak's pricing terms: free for personal use, license
  required for commercial use." Rejected: a paid licence per contributor, or
  building it from source.

### 5.6 Posting — a nice TUI, rejected on chaining, CLI and packaging

[Posting] is a Python TUI built on Textual (Apache-2.0,
`requires-python >= 3.11`). Stable release `2.11.2` on 2026-10-05;
`v3.0.0-beta.0` and `v3.0.0-beta.1` on 2026-10-15
(`gh api repos/darrenburns/posting/releases`). 12,625 stars. Evaluated:
`2.11.2`, the latest stable release.

**From its documentation and source at tag `2.11.2`** (`docs/guide/*.md`,
`src/posting/__main__.py`):

- **TUI only.** The command line has four commands: `posting` (the TUI, with
  `--collection` and `--env`), `locate`, `import` and `sponsors`
  (`src/posting/__main__.py:45-178`). No command sends a request outside the
  TUI, so a task in Zed or VS Code cannot send one either, and agents cannot use
  it. The source at tag `v3.0.0-beta.1` has the same four commands and still
  declares `version = "2.11.2"` in `pyproject.toml`, yet the build the user
  installed with Homebrew answers `posting version` with `Posting 3.0.0-beta.1`;
  Homebrew's core formula ships `2.11.2` (`Formula/p/posting.rb`). That build
  therefore differs from the tag, and whether it can send a request outside the
  TUI was not checked.
- **Collection**: a directory of `.posting.yaml` files, one per request, loaded
  recursively with `--collection DIR`; sub-directories are sub-collections
  (`docs/guide/collections.md`).
- **History**: SQLite under `$XDG_DATA_HOME/posting/history/`, per collection,
  at most 100 exchanges; loading an entry restores its request and response.
- **Environments**: `.env` files loaded with `--env` (several, the last one
  wins), or `posting.env` from the current folder by default; variables written
  `$NAME` or `${NAME}`. The host's environment variables, such as
  `HURL_VARIABLE_token`, are used only with `use_host_environment: true` or
  `POSTING_USE_HOST_ENVIRONMENT=true` (`docs/guide/environments.md`).
- **Chaining**: Python scripts attached to a request (`setup`, `on_request`,
  `on_response`) set session variables with `posting.set_variable(...)`; they
  last until Posting is closed (`docs/guide/scripting.md`). A request that needs
  another's response therefore needs that request sent earlier in the session,
  or a setup script that fetches the value itself; nothing like Slumber's
  `trigger='no_history'` (§5.3 S3).
- **Import**: OpenAPI 3.x (`posting import spec.yaml -o DIR`), Postman, and a
  curl command pasted in the URL bar, all marked experimental
  (`docs/guide/importing.md`).
- **Editors and pagers**: `$POSTING_EDITOR` / `$EDITOR` for bodies and scripts
  (`code -w` for VS Code), `$POSTING_PAGER` for responses, with a separate pager
  for JSON (`docs/guide/external_tools.md`).
- **mise**: no cargo or GitHub release route; a Python package, so
  `pipx:posting` (installed through uv when present) from PyPI, with a Python
  interpreter to manage.

**User's assessment**: the user installed Posting with Homebrew
(`posting version` → `Posting 3.0.0-beta.1`) and found its TUI "really nice".
The planned experiments (install and command line, OpenAPI import, install
through mise) and the teammate's hands-on trial were not run: the user judged
the limitations above enough to reject Posting.

**What it means**: rejected, for four reasons set against §1 and §1.2, from its
documentation and source:

1. **TUI only**: no request can be sent from the command line, so no editor task
   and no agent use (not checked on the user's 3.0 build, see above).
2. **Chaining through session scripts**: a request that needs another's response
   only works after that one was sent in the same session, or with a Python
   script that fetches the value; it does not answer "run request NN on its own"
   (§1.2) the way Slumber's triggers do (§5.3 S3).
3. **One file per request** in a folder, where Slumber keeps one collection
   file.
4. **A Python package from PyPI**: pinned through `pipx:` with a Python
   interpreter beside the repository's Rust tools, and environment variables
   such as `HURL_VARIABLE_token` visible only with
   `POSTING_USE_HOST_ENVIRONMENT=true`.

The verdict of §6 is unchanged.

## 6. Conclusion & Recommendation

**Conditional Go** (agreed with the user, 2026-11-14): replace REST Client with
**Slumber** to explore the API, keep **Hurl** to test it, and add the
**`hurl-at` wrapper** with editor tasks so anyone can send a single Hurl entry
from Zed or VS Code while working on the tests. `lambdas/requests.rest` is
retired.

**Condition**: before the specification is finalized, one Zed user and one VS
Code user try the Slumber TUI on a seed collection and confirm the exploration
loop of §1.2: send a saved request, read its body and headers, query the body,
and send a chained request on its own. The agent could only run the CLI (§5.3);
the TUI was assessed from its documentation.

**Why Slumber for exploring** (§5.3):

- It is the only tested candidate whose loop matches what the team asked for
  (§1.2): recipes are picked by name, a chained recipe sends its dependencies
  itself (S3: `trigger='no_history'` sent `list_cities` before
  `get_first_city`), and the response pane shows status, headers and a body that
  can be queried with `jq`.
- The three must-haves without glue code: send and inspect, chaining (with URL
  encoding done for us), environments as profiles (`-p local|staging`).
- Editor-agnostic: it runs in the terminal panel of Zed or VS Code, reloads the
  collection on save, and both editors validate `slumber.yml` with its JSON
  schema. No editor extension to wait for: none in Zed sends requests anyway
  (§5.0).
- Fits the constraints: MIT, no account, one YAML file in git and history kept
  per user, released 2026-10-27, and `cargo:slumber` in `mise.toml` like
  `bacon`.
- The staging bonus maps onto its `env`, `prompt` and `response` functions
  (structural check only).

**Why keep the Hurl wrapper** (§5.1): Hurl stays the source of truth for
testing, and the team could not run request NN of a test file on its own (H6).
The wrapper is small (29 lines; it picked the right entry on all 2,650 lines of
the 13 test files), needs nothing beyond `mise.toml`, and turns a test entry
into a one-key action in both editors (`.zed/tasks.json`, `.vscode/tasks.json`;
`vscode-hurl-runner` as an optional alternative in VS Code). Entries that depend
on captures run with `--upto`, or with captures saved from an earlier run (H5).

**What it costs**: a second request format beside Hurl, with its own collection
to keep in step with the API. The bridge between them is partial:
`slumber generate curl` → `hurlfmt --in curl` works for `GET`, not for JSON
bodies (§5.3 S4). Imports only seed the collection: the OpenAPI import sends
empty query parameters and stringified values (`400`, `422`), and the `.rest`
import needs `HTTP/1.1` on every request line (§5.3 S1, S2).

**Not chosen**: Hurl alone (it has the capabilities, but the exploration loop is
what the team struggles with, §1.2); Bruno (best at generating the collection
from `openapi-3.1.yaml`, but a desktop app outside both editors and outside
mise, §5.4); httpyac (no release for 19 months, §5.2); Yaak (paid licence for
commercial use, §5.5); Posting (a nice TUI, but no command line, chaining only
within a session, one file per request, and Python from PyPI, §5.6).

## 7. Still Open

- **The TUI itself**: not run by the agent. The condition of §6 settles it; if
  the trial fails, Bruno (§5.4) is the next candidate.
- **Keeping the collection in step with the API**: nothing checks `slumber.yml`
  against `openapi-3.1.yaml`, the problem `requests.rest` already has (§2). A
  check in the spirit of `hurl-coverage` (every recipe path matches an
  operation) is possible but not explored.
- **Where the collection lives and how it is organized**: one `slumber.yml` at
  the root or in `lambdas/`, grouped by tag as the OpenAPI import does, or split
  with `$ref` (not explored).
- **Cognito login against staging or production**: mapped onto `env`, `prompt`
  and `response` on paper only; never run against AWS.
- **Hurl's curl import and `--json`**: Hurl 8.1 rejects curl's `--json`, so
  explored `POST`s cannot be turned into Hurl entries automatically; a later
  Hurl release may accept it (not checked).
- **Captures saved between `hurl-at` runs** (§5.1 H5): useful, but whether the
  wrapper should do it, and where the git-ignored file lives, is a design
  choice; `redact` captures cannot travel this way.
- **Editors**: the Zed and VS Code tasks were run as the command they would
  execute, not from inside the editors.
- **Agents in the nono sandbox**: Slumber writes its history to
  `~/Library/Application Support/slumber/`, which the `pfb-claude` profile does
  not grant; an agent would need that path or `HOME` pointed elsewhere, as in
  these runs.

## 8. Remaining Work

In priority order:

1. **TUI trial** (the condition of §6): a seed collection of a few recipes,
   including a chained one; one Zed user and one VS Code user; record what they
   found.
2. **The collection**: seed it with `slumber import openapi openapi-3.0.yaml`,
   remove the empty query parameters, fix the value types, name the recipes, and
   port the useful requests of `lambdas/requests.rest` (ignoring its invalid
   `POST /cities/` body, §2); profiles `local`, `staging` and `production` from
   the existing hosts; add the JSON schema comment. Decide its location (§7).
3. **Pin Slumber**: `"cargo:slumber" = "5.4.4"` in `mise.toml`, `mise lock`, and
   a line in `.github/CONTRIBUTING.md`.
4. **`hurl-at`**: add the wrapper (e.g. `lambdas/tests/hurl-at`), keep the check
   that its entries match `hurlfmt --out json` for every test file (§5.1 H2),
   commit `.zed/tasks.json` and `.vscode/tasks.json`, and document them with
   `vscode-hurl-runner` as an option in `lambdas/tests/README.md`. Decide on
   saved captures (§7).
5. **Retire `lambdas/requests.rest`** and its exclusion in
   `.github/workflows/lambdas.yml:12`; point to the new collection.
6. **Optional**: Cognito login recipes for staging and production; a check that
   keeps the collection in step with the specification (§7).

## Appendix A: `hurl-at` as tested

The wrapper of §5.1 H2–H4, kept here because the experiments ran in a scratch
folder. It is the version that passed both checks of H2.

````bash
#!/usr/bin/env bash
# Run the Hurl entry at LINE, or every entry up to it with --upto. A cursor on
# the comments or blank lines just above a request selects that request.
# Usage: hurl-at [--upto] FILE LINE [hurl options...]
set -euo pipefail
upto=false
if [ "${1:-}" = "--upto" ]; then upto=true; shift; fi
file=$1 line=$2; shift 2
entry=$(awk -v l="$line" '
  function is_request(s) { return s ~ /^[[:space:]]*[A-Z]+[[:space:]]+[^:[:space:]]/ && s !~ /^[[:space:]]*HTTP/ }
  function is_gap(s) { return s ~ /^[[:space:]]*(#.*)?$/ }
  # Lines of a ``` multiline body, fences included, are never requests.
  { fence = ($0 ~ /^[[:space:]]*```/); code = body || fence; if (fence) body = !body }
  !code && is_request($0) { n++ }
  # Remember the entry count at the cursor, and whether only comments and
  # blank lines separate the cursor from the next request.
  NR == l { at = n; gap = !code && is_gap($0); next }
  NR > l && gap {
    if (!code && is_request($0)) { at = n; exit }
    if (code || !is_gap($0)) gap = 0
  }
  END { print at + 0 }' "$file")
if [ "$entry" -lt 1 ]; then
  echo "hurl-at: no request at or above line $line of $file" >&2
  exit 2
fi
if $upto; then from=1; else from=$entry; fi
echo "hurl-at: $file:$line -> entry $entry (from $from)" >&2
exec hurl --from-entry "$from" --to-entry "$entry" "$@" "$file"
````

[Bruno]: https://github.com/usebruno/bruno
[Hurl]: https://hurl.dev
[Posting]: https://github.com/darrenburns/posting
[REST Client]: https://github.com/Huachao/vscode-restclient
[Slumber]: https://github.com/LucasPickering/slumber
[vscode-hurl-runner]: https://github.com/jellydn/vscode-hurl-runner
