# acme-template

A thin Rust CLI starter for projects that will use
[acme](https://github.com/tylerbutler/acme). This is not the acme CLI.
The only preset is `profile = "cli"` with `languages = ["rust"]`.
It is a baseline, not a complete factory profile.

## Local checks

Install Rust with rustup and install [just](https://just.systems/).
Cargo uses the toolchain in `rust-toolchain.toml`.

```sh
just check
just build
cargo run --locked -- --help
cargo run --locked -- --version
```

The starter has no dependencies. It shows help or its version and rejects
unsupported arguments. Replace it with your project's CLI through reviewed
changes. `just check` runs formatting, Clippy and tests. No GitHub workflow
runs these checks yet.

## Start a project

Use GitHub's **Use this template** action to create your repository. Then:

1. Write the project's goal, users, acceptance checks and limits in
   [`factory/brief.md`](factory/brief.md).
2. Use the `init-interview` skill below to propose the complete
   [`factory/factory.toml`](factory/factory.toml) and its diff. Review the
   project identity, risks, engines, cost limits and CI selection. Set the
   Cargo package name through a separate reviewed change and update the CLI tests.
3. Have the owner review the complete config and diff before saving the
   approved config and explicitly invoking `acme init`. The supplied
   stub is inactive: all modules and workflow gates are off. Copying it
   does not approve a new project's config, paid work or remote writes.
4. Follow [Initialize approved config](#initialize-approved-config) to prepare
   one reviewed init PR. Publishing generated CI needs separate owner approval.
5. Use `spec-writer` to propose the first feature and backlog from the preserved
   brief. Keep authored instructions in [`AGENTS.md`](AGENTS.md), reviewed
   specs in [`docs/specs/`](docs/specs/README.md), and decisions in
   [`docs/adr/`](docs/adr/README.md).

The stub declares CI commands that use only this starter's Cargo inputs.
No acme-specific Python scripts, module implementations or workflows are
copied. Review and holdout are unavailable and remain off.

## Proposal skills

The skills live in `.agents/skills/`. If your agent does not discover that
directory, ask it to read the linked `SKILL.md` and follow it. No CLI command
or external skill dependency is needed.

| Skill | Input | Output |
| --- | --- | --- |
| [`init-interview`](.agents/skills/init-interview/SKILL.md) | Brief, current config, project files and unresolved owner answers | Fact table, complete unapproved config, proposed diff and human handoff |
| [`spec-writer`](.agents/skills/spec-writer/SKILL.md) | Preserved brief, relevant facts and existing public specs | Proposed small feature spec, acceptance scenarios, non-goals and dependency-ordered backlog |

For example: "Use init-interview to propose config from my brief; do not
write files." Then: "Use spec-writer to propose the first feature spec and
backlog from the preserved brief."

Both skills are read-only and ask only unresolved questions. Secret names
and requirements are allowed inputs; secret values and credentials are not.
Partial answers, silence and cancellation never authorize generation, remote
writes or paid work. The skills do not run acme, publish issues, launch agents
or change settings. They leave the brief and all authored files unchanged.

The interview may propose `modules.ci.enabled = true` only after an explicit
owner answer selects CI. Otherwise it keeps CI off. All other modules and
agent workflow/assignment flags remain off. This selection proposes local
CI files; it does not authorize generation, publication or paid work.

The owner reviews the complete config and diff before saving the approved
config and explicitly invoking `acme init` with the matching `0.2.0` binary.
Do not use render or sync as an implicit fallback. A human reviews
and merges a feature spec; only the owner can approve its specific build with
`ready-for-agent` or direct Copilot assignment. Neither skill grants approval.

Replayable scenario prompts and expectations are in each skill's
`evals/evals.json`. Run them against a disposable local copy or read-only
fixture, not a live factory. Check the response and tool transcript, including
the absence of credential reads, writes and paid calls. These are behavioral
checks; `just check` covers the Rust starter, not model compliance.

## Pinned acme contract

The config uses acme `0.2.0`, `schema_version = 0`, and `bootstrap = false`.
The released version is
[`v0.2.0`](https://github.com/tylerbutler/acme/tree/v0.2.0), at commit
`5f1a0df867912ddeb1c11d031e96817d6ca0de07`. Use that exact commit when you
verify or build the release.

[`schema/factory.schema.json`](schema/factory.schema.json) is an unchanged
copy from
[that commit](https://github.com/tylerbutler/acme/blob/5f1a0df867912ddeb1c11d031e96817d6ca0de07/schema/factory.schema.json).
Its SHA-256 is
`ed4ad9562a211a0fa2a61db237756d74508d388159d6017d11ded3817e032102`.
Use it to validate the complete TOML document as JSON. Do not remove fields
to make validation pass.

Acme is a separate tool. It is not a dependency of this crate, and this
template does not download or install it. The owner must explicitly install
the matching release binary before approved local generation. The required
`toolchain.gh_aw` field records the compatible compiler version; no agent
compiler is needed for this inactive baseline.

## Initialize approved config

The human reviews the complete proposal and diff, then saves the approved
config to `factory/factory.toml`. The existing config must match the approved
bytes; init does not replace a different config. With the matching binary
explicitly installed, the human creates a branch and invokes local generation:

```sh
git switch -c factory-init
acme init --config factory/factory.toml
just check
git status --short
git diff
```

Init supports this single-package Rust project and the stub's Cargo commands.
It rejects missing source inputs, incompatible pins, workspaces, `.cargo`
config overrides and conflicting generated files before writes. Commands
with `--locked` need a valid `Cargo.lock`. See the
[released init contract](https://github.com/tylerbutler/acme/blob/v0.2.0/README.md#local-init)
before proposing other CI commands.

With all modules off, init writes only the ownership record and preserves
the approved config and authored files. With CI selected, it also generates
`.github/workflows/ci.yml` and `.github/actions/setup-rust/action.yml`.
Stage only the reviewed config, ownership record and exact generated paths
listed by init. Review `git diff --cached`, commit the change, then explicitly
open a draft PR with `gh pr create --draft`. A human reviews and merges it.
Publishing CI files can start ordinary Actions jobs and needs separate
owner approval. It does not approve paid agent work.

An existing ownership record blocks init. Review the project and use the safe
sync path instead; do not delete the record to force initialization. Init does
not execute CI commands, contact GitHub, provision labels or rulesets, upload
secrets, activate agent workflows, create a branch/PR, or approve a merge.
Generation can leave partial changes after an I/O failure; inspect the result
and do not run it with concurrent checkout writers.

See the [M4 contract](https://github.com/tylerbutler/acme/blob/5f1a0df867912ddeb1c11d031e96817d6ca0de07/docs/specs/0005-m4-init-template-contract.md)
and [tylerbutler/acme#86](https://github.com/tylerbutler/acme/issues/86) for the
separately approved sample-project run. This template update does not complete it.

## License

MIT OR Apache-2.0. See [LICENSE-MIT](LICENSE-MIT) and
[LICENSE-APACHE](LICENSE-APACHE).
