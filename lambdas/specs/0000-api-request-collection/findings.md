# Findings: API Request Collection with Slumber

Everything non-obvious learned while implementing this feature: behavior that
looks like a bug unless you know it is deliberate, traps in the libraries and
tools, and what worked. Each entry says what the rule is, how it was caught, and
what it cost.

Status: complete for tasks 1-10. Open after merging: R3.3, to observe on the
first pull request that changes only `lambdas/slumber.yml` (`tasks.md`, Notes).
Keep it current as the files change.

## 1. Behavior that does not survive a naive implementation

### 1.1 Folders follow the operations' tags, not the spec's tag list

The README convention puts a recipe in a folder named after its operation's tag
(`lambdas/README.md`, "Adding a request to the collection"). The operations of
`openapi-3.1.yaml` are tagged `city`, `pipeline`, `price`, `rating`, `report`,
`system` and `usstate`, but the specification's top-level `tags` list
(`openapi-3.1.yaml:3129-3139`) names `reports`, not `report`, and has no
`system`. Following the list would give a `reports` folder and no home for
`get_health`. The README therefore says "the `tags` of its operation" and lists
the operations' tags.

**Caught by**: counting the `tags` of every operation while writing the README
(`grep` on `openapi-3.1.yaml`), against the list `design.md` had taken from the
OpenAPI import.

**Cost**: none in this feature; the mismatch itself is in the API's OpenAPI
annotations and is left as it is (out of scope).

### 1.2 The specification's examples quote booleans; that is why imports do

`slumber import openapi` and `bru import openapi` produced bodies with
`consent: 'true'`, which the API rejects with `422` (spike §5.3 S2). The
specification itself gives that example as a string: `consent` is
`type: boolean` with `examples: - 'true'` (`openapi-3.1.yaml:3027-3031`). The
importers copy the examples, so a recipe seeded from an import must have its
types checked against the schema, as the README says; the README's own example
sends `"consent":true` and was accepted (`201`, task 5.5).

**Caught by**: reading the `SubmissionPost` schema to check the README example's
types.

**Cost**: none here; fixing the example in the annotations would make imports
usable without that check (out of scope).

### 1.3 The health check's `timeout` above its `interval` is not an overlap

`docker-compose.yml` gives the `postgres` service `interval: 10s` and
`timeout: 60s`. The spec review read this as "a probe that hangs can overlap the
next one". It cannot: Docker runs a check `interval` seconds after the previous
one _completes_ (Dockerfile reference, `HEALTHCHECK`, in `docker/docs`,
`_vendor/github.com/moby/buildkit/frontend/dockerfile/docs/reference.md`), and a
check running longer than `timeout` counts as failed. During
`start_period: 10s`, checks run every start interval (5 s by default) and their
failures do not count, so `just compose-up` returns soon after PostgreSQL
accepts connections. The long `timeout` only slows the detection of a hung
check: up to six of them before the service is `unhealthy`. Measured: on a fresh
volume, `just compose db-seed` showed the service `Healthy 5.2s` after its
start, and `db-seed` connected at once (task 9.3). With a check that always
fails (task 9.4), `docker compose up -d --wait` stopped with
`container bna-api-postgres-1 is unhealthy`, exit 1, after 60.9 s: the start
period's checks end at 10 s, then the six counted failures run 10 s apart, so
the service is `unhealthy` at about 60 s. The task had predicted about 70 s,
counting one interval too many; corrected in `tasks.md`.

**Caught by**: the spec review's finding, checked against Docker's documentation
before recording the design.

**Cost**: none; the values were kept.

## 2. Library and tooling traps

### 2.1 A full `mise lock` fails inside nono; lock the new tool by name

Task 1.2 asks for the command of `.github/CONTRIBUTING.md` ("Bumping a tool"):
`mise lock --platform linux-x64,linux-arm64,macos-arm64,macos-x64`. Inside the
nono sandbox (mise 2026.11.8), it re-resolves every tool of `mise.toml`, and
stopped on the first GitHub tool whose attestation it re-verifies, typos for
`linux-arm64`, with `lock-time provenance verification failed`: exit 1,
`mise.lock` left unchanged. The same Sigstore verification failure inside nono
is recorded in `specs/0003-tool-manager/spike.md`.

Naming the tool limits the work to it:
`mise lock cargo:slumber --platform linux-x64,linux-arm64,macos-arm64,macos-x64`
checked the four targets, exit 0, and added only the expected entry
(`mise.lock:20-23`), shaped like the other cargo tools; the other entries were
not touched.

**Caught by**: running the documented command inside the sandbox.

**Cost**: none to the lockfile. Outside the sandbox the documented command is
still the one to use; inside it, lock the changed tool by name.

### 2.2 Overriding `HOME` breaks the mise shims; call the installed binary

Inside the sandbox, Slumber cannot write its history folder
(`~/Library/Application Support/slumber/`), so the checks run it with `HOME` set
to a scratch folder (`tasks.md`, Notes). Through the mise shim
(`~/.local/share/mise/shims/slumber`), that fails before Slumber starts:
`Config files in .../bna-api/mise.toml are not trusted`, exit 1. mise keeps its
trust records under the user's home, so a different `HOME` means an untrusted
`mise.toml`.

Calling the installed binary directly keeps the override for Slumber only:
`HOME=<scratch> "$(mise which slumber)" ...`, where `mise which slumber` gives
`~/.local/share/mise/installs/cargo-slumber/5.4.4/bin/slumber`. Tasks 2.2 and
2.3 passed this way. `tasks.md` (Notes) now says so.

**Caught by**: the first run of task 2.2.

**Cost**: one failed run; no effect on contributors, who run Slumber with their
own `HOME`.

### 2.3 oxfmt also reformats the YAML of Markdown code blocks

`just fmt` and `just lint` run oxfmt on every Markdown file, and oxfmt formats
fenced `yaml` blocks too. In the README example (`lambdas/README.md`, "Adding a
request to the collection"), it folded the one-line chained `url` of `get_city`
into a double-quoted scalar over five lines. YAML joins such lines with a single
space, and every break fell on an existing space, so the value is unchanged: the
URL Slumber parsed from the formatted block was compared with the original
one-line string and is identical, and `get_city` still answered `200`.

**Caught by**: reviewing the README diff after the formatter ran at
checkpoint 6.

**Cost**: none to the behaviour; the example is less readable than written. Any
example in a Markdown code block must be re-tested after `just fmt`, not before:
task 5.5 was re-run on the formatted block.

## 3. Method — what actually worked

### 3.1 Test the documented example by extracting it from the README

Task 5.5 did not retype the README example: a short script took the only `yaml`
block of `lambdas/README.md`, put it in place of the whole `requests:` mapping
of a scratch copy of `lambdas/slumber.yml`, and Slumber loaded and sent it.
_Corrected after the review_: the first version of this entry said "in place of
`requests: {}`", which no longer exists since task 7 added `city/get_cities`.
The example defines the same folder and recipe, so adding it next to the
collection's recipes instead of replacing them fails with
`Duplicate recipe/folder ID 'get_cities'`, exit 1 (checked); replacing the
mapping loads, and `get_city` answered `200`. What was tested is exactly what
readers copy, which is what caught the formatter's rewrite (§2.3) as harmless
rather than assumed. Reuse it for any configuration example kept in
documentation.
