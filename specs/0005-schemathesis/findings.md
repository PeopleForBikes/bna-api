# Findings: Property-Based API Testing with Schemathesis

Everything non-obvious learned while implementing this feature: behavior that
looks like a bug unless you know it is deliberate, traps in the libraries and
tools, and what worked. Each entry says what the rule is, how it was caught, and
what it cost.

Status: in progress, task 1 done. Measured with bacon 3.26.0 on macOS arm64, in
the nono sandbox, on 2026-11-13.

## 1. Behavior that does not survive a naive implementation

## 2. Library and tooling traps

### 2.1 bacon restarts the server when a test run writes under `lambdas/tests/`

bacon watches every package's `src`, `tests`, `benches` and `examples`
directories, and `lambdas/tests/` is the `tests` directory of the `lambdas`
package, although it holds the Hurl suites, not Rust tests. With
`on_change_strategy = "kill_then_restart"`, the `axum-debug` job killed the
server whenever a file there changed: `just test` deletes and recreates
`reports/`, and Schemathesis 3 wrote `cassette.yaml` during its run. A
`.gitignore` rule does not help for `reports/`: recreating the ignored directory
itself still killed the server. The job now ignores the folder
(`bacon.toml:121`); a folder pattern covers its content since bacon 3.23.0.

**Caught by**: during the specification, the smoke tests passed and the next
`just test localhost` failed on all 12 files with "Failed to connect" in 0 ms. A
probe polling `/health` and the `axumed` PID every 100 ms showed the server
killed 100 ms after `reports/` was recreated, back after 1.6 s; a new file in
`lambdas/tests/` did the same.

**Cost**: the local Hurl suites looked broken, and Schemathesis looked like it
crashed the server: "Connection aborted. Remote end closed connection without
response" is what it reports when bacon kills the server during a request. That
message is why the CI step was commented out; whether the CI failure had the
same cause is unknown, since CI runs no bacon. With the ignore, task 1.2
measured the PID unchanged and `/health` at `200` for 8 s after each change
under `lambdas/tests/`.

### 2.2 `touch` does not make bacon restart; a content change does

Touching `lambdas/src/lib.rs` (modification time only) left the server untouched
for 8 s. Appending an empty line restarted it: down after 0.2 s, back with a new
PID after 5.9 s (debug build, incremental).

**Caught by**: task 1.2, whose first version of the probe used `touch` and
concluded no restart.

**Cost**: a probe built on `touch` would wrongly report that bacon no longer
watches `lambdas/src/`. Check a watcher with a real content change, then restore
the file (which restarts it once more).

## 3. Method — what actually worked

### 3.1 Poll the health endpoint and the server's PID

A loop printing `/health`'s status and the `axumed` PID every 100 ms, only when
either changes, tells a restart (PID changes) from a crash (no PID) from a slow
answer, and times each. It turned "the tests fail" into "the server is killed
100 ms after this file operation" in one run.
