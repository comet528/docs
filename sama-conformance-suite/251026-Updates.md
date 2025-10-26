# 26 Oct 25 - sama conformance suite update goals

Today I would like to focus on 4 areas
- speeding testing via direct file injection
- updating the vue framework so it can use node 22
- ui testing via playwright framework
- preparation for next major piece of work that involves
    - updating the swagger for ksa
    - understanding how the tst case work
      - ensuring that all the tests work appropriately
      - suggesting ways to improve the test framework, test definitions etc
    - adding new tests to cover the new swagger
      - likely understanding better the business rules associated with the swagger
        - we need to figure out whats to enable this as the business rules are stored in private confluence pages
    get the schema validatator working
        - as part of the testcases for responses - ensuring adherence to the schema
        - as part of the test case checking for requests - i.e. a request should be allowed to break the schema but it should be flagged as doing such so that only testcases intended to break the request scheam actually do.

Go away and give me a proposal on how you deal with each of the above areas and the comparative amount of work required for each so that I can understand the size of effort for each task.
Keep the discussion at a relatively high level - which I decide to start working on each of the 4 we can deep dive implemenation of each at that point.




## Proposal (high-level)

### 1) Speeding testing via direct file injection

- Goal: Run suites rapidly by loading prepared inputs/fixtures directly, bypassing slow setup paths (e.g., network calls, multi-step UI flows).
- Approach:
  - Add a test-only injection interface:
    - Backend: an authenticated local-only endpoint (or CLI) to ingest test fixtures, tokens, and session state.
    - Frontend: dev flag to load fixture JSON from `fixtures/` and pre-populate store/localStorage.
  - Define a stable fixture format: inputs, expected API responses, and pre-baked auth artifacts per test case.
  - Wire a “fast path” in test runs: if `TEST_FAST=1`, skip setup flows and hydrate from fixture.
  - Cache heavy artifacts (e.g., discovery docs, JWKS) on disk between runs in a `.cache/` directory keyed by hash.
- Integration points relative to current code:
  - Vue app: add `fixtures/` with a loader utility and a `VITE_TEST_FAST` toggle.
  - API/test harness (if present in this repo): add `/__test__/inject` or a CLI `inject-fixture` command limited to `NODE_ENV=test` and loopback only.
- Effort estimate: Small → 1–2 days for MVP; +1–2 days to stabilize the fixture schema and caching.
- Risks/Notes: Keep the injection path disabled in prod builds; add a guard to fail if enabled outside test/dev.

### 2) Update Vue framework to work with Node 22

- Goal: Ensure local dev, build, and tests run on Node 22 (current LTS cadence).
- Approach:
  - Bump Node engine in `package.json` and CI images to 22.
  - Upgrade Vite/Vue CLI, Vue 3, and key plugins to Node-22-compatible versions.
  - Replace deprecated Node polyfills if present (Buffer, stream) with browser-friendly shims as needed.
  - Run a clean install and build; address type/eslint config drifts.
  - Update lockfile in a single controlled upgrade.
- Integration points:
  - `package.json`, build scripts, Vite config, ESLint/TS config.
- Effort estimate: Small/Medium → 0.5–1.5 days depending on plugin ecosystem drift.
- Risks/Notes: If legacy dependencies pin old Node APIs, plan for minor refactors or polyfills.

### 3) UI testing via Playwright

- Goal: Add reliable E2E/regression UI coverage with parallel, headless runs and trace/videos for flake triage.
- Approach:
  - Add Playwright with test project presets (chromium first; webkit/firefox optional).
  - Create auth helpers and fixture loaders (reusing the direct injection fast path above) to skip slow onboarding flows.
  - Define core smoke scenarios: load app, navigate key views, run representative test cases, assert pass/fail signals.
  - Enable `playwright/test` reporter with traces-on-failure and GitHub Actions integration (if CI present).
  - Tag tests: `@smoke` for PRs, `@full` nightly.
- Integration points:
  - `playwright.config.ts`, `tests/e2e/*.spec.ts`, npm scripts: `test:e2e`, `test:e2e:headed`.
- Effort estimate: Medium → 1–2 days to bootstrap + initial smoke; +2–4 days to cover critical paths.
- Risks/Notes: Flake control via network stubbing and fixture injection is key; keep auth deterministic.

### 4) Preparation for KSA swagger + test architecture improvements + schema validator

- Goals:
  - Align tests to the updated KSA OpenAPI (Swagger) spec.
  - Clarify business rules mapping into test assertions (public docs vs. private Confluence).
  - Enforce schema conformance on responses and “flag but allow” for negative-request tests.
- Approach:
  - Spec ingestion:
    - Store the KSA OpenAPI as versioned files under `openapi/ksa/` with a `SPEC_VERSION` switch.
    - Add a small generator utility to produce request/response validators and typed clients from the spec.
  - Test case model:
    - Define test metadata: positive/negative, required capabilities, expected status, expected error schema.
    - Map business rules to test IDs; capture rule provenance (Confluence link or summary) in metadata for traceability.
  - Schema validation:
    - Responses: validate against OpenAPI-derived JSON Schemas (e.g., via Ajv + openapi-types).
    - Requests: for negative tests, allow schema violations but explicitly assert the expected error/behavior.
    - Provide a per-test toggle: `enforceRequestSchema: true|false` with lint that ensures intent is explicit.
  - Gaps and improvements:
    - Add a linter that fails if a test targets an undefined/renamed spec operationId.
    - Add coverage report: operations covered vs. total per spec version.
- Integration points:
  - New `openapi/` folder, a `scripts/generate-schemas` step, test harness validators, and metadata schema for tests.
- Effort estimate: Large → 5–10 days depending on spec complexity and rule sourcing; +1–2 days to operationalize coverage and linting.
- Risks/Notes: Business rules locked in private Confluence delay precision; plan for a “stubbed rules” pass then refine with SMEs.

### Overall sequencing suggestion

1) Node 22 upgrade (unblocks tooling) — S/M
2) Direct file injection (speeds all further work) — S
3) Playwright bootstrap + smoke (leverages injection) — M
4) KSA spec alignment + schema validator + coverage/lint — L

### Quick wins to start immediately

- Add `VITE_TEST_FAST` toggle and a minimal fixture loader in the Vue app.
- Add Playwright with one smoke test and traces-on-failure.
- Bump Node to 22 and update build/test scripts.

If you want, I can start by drafting the Node 22 upgrade PR and the test fixture injection hooks; then wire Playwright smoke on top.
