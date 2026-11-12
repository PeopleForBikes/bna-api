# Yanked

This spike was yanked because it concluded No-Go.

It asked whether a connection pooler that behaves like Neon's (PgBouncer in
transaction mode, in front of staging and production) should be added to the
local Docker Compose setup, and compared PgBouncer, PgDog and PgCat
([Intent](spike.md#1-intent)).

It is feasible: with Neon's settings, PgBouncer 1.26.1 ran the whole Hurl suite
with the same result and timing as the direct connection, and the suite caught a
pooler without prepared-statement support ([Findings](spike.md#5-findings)). The
team decided not to build it because:

- it gives contributors nothing new: a transaction-mode pooler only forbids
  session features, which the API does not use, and Neon's pooler has been
  transparent since the start ([Current Baseline](spike.md#2-current-baseline));
- the bug class it would guard against has never occurred, and the team already
  runs the suite against staging, behind Neon's real pooler, before every major
  release;
- it would add a permanent cost: an extra service, two database URLs, settings
  to keep in line with Neon, and an extra step in CI
  ([Conclusion & Recommendation](spike.md#6-conclusion--recommendation)).

To reproduce Neon's pooler locally when needed, use the tested recipe in
[§6.1](spike.md#61-recipe-a-neon-like-pooler-on-demand). Running the suite
against staging after every deployment is the open alternative
([Still Open](spike.md#7-still-open)).
