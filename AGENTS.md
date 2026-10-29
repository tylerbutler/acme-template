# Project agent map

## Purpose and inputs

This repository is a minimal Rust CLI starter, not the acme implementation.
Read `factory/brief.md`, `factory/factory.toml` and `docs/README.md` first.
The brief records project intent. The config stub records an inactive
baseline; it does not approve a copied project's settings.

Use only `profile = "cli"` and `languages = ["rust"]` for this baseline.
Use Rust 2024 and the pinned toolchain. Keep errors explicit and add focused
tests for changed behavior.

## Repository map

- `src/`: project CLI source.
- `tests/`: CLI behavior tests.
- `factory/brief.md`: authored project brief.
- `factory/factory.toml`: complete config stub for owner review.
- `schema/factory.schema.json`: exact pinned acme schema; do not edit by hand.
- `docs/specs/`: reviewed feature specifications.
- `docs/adr/`: architecture decisions.
- `.agents/skills/init-interview/`: read-only config interview and proposal.
- `.agents/skills/spec-writer/`: read-only feature spec and backlog proposal.

For project setup or feature planning, read the matching `SKILL.md` before
starting. Both skills return proposals in the conversation. They do not
write files, run acme, publish issues or approve builds. See the root
README for inputs, outputs and scenario checks.

## Approval boundaries

Use L3 autonomy. Build one owner-approved issue with a merged spec in one
draft PR. Use a Conventional Commit PR title. A human reviews and merges;
never approve, merge or enable auto-merge.

All modules start off. Do not enable them, activate workflows, assign a
builder, upload secrets, or run paid tests without separate owner approval.
The interview may propose CI only after an explicit owner answer selects it.
All other modules and agent workflow/assignment flags remain off. A proposal
does not authorize generation or publication of a workflow.
Ask for secret names and requirements, never secret values. Never commit
credentials or private holdout scenarios.

The owner must review the complete config and diff before local generation.
Do not run init, sync, doctor repairs or provisioning without separate
approval. Local render checks use a temporary output directory only.
Acme `0.2.0` supports local `acme init --config factory/factory.toml`.
The human saves the reviewed config and explicitly invokes init. Do not
invent flags or claim that generation completes provisioning.

Do not change paths in `risk.never_agent`. The risk map is an instruction,
not an enforced author/path gate. Unsupported modules stay disabled.
Do not copy acme's source, module implementation or agent workflows here.

## Checks and completion

Run `just check` and any checks required by the changed spec.
Use `just build` to build the CLI. Keep CI commands limited to inputs
available in this project. Report missing tools and unverified acceptance
checks explicitly.
