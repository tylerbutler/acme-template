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
   project identity, risks, engines and cost limits. Set the Cargo package
   name through a separate reviewed change and update the CLI tests.
3. Have the owner review the complete config and diff before saving the
   approved config or explicitly invoking a supported init. The supplied
   stub is inactive: all modules and workflow gates are off. Copying it
   does not approve a new project's config, paid work or remote writes.
4. Use `spec-writer` to propose the first feature and backlog from the preserved
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

The owner reviews the complete config and diff before explicitly invoking
`acme init` with the approved config when a release supports it. Acme `0.1.0`
does not. Do not use render or sync as an implicit fallback. A human reviews
and merges a feature spec; only the owner can approve its specific build with
`ready-for-agent` or direct Copilot assignment. Neither skill grants approval.

Replayable scenario prompts and expectations are in each skill's
`evals/evals.json`. Run them against a disposable local copy or read-only
fixture, not a live factory. Check the response and tool transcript, including
the absence of credential reads, writes and paid calls. These are behavioral
checks; `just check` covers the Rust starter, not model compliance.

## Pinned acme contract

The config uses acme `0.1.0`, `schema_version = 0`, and `bootstrap = false`.
The adopted stage0 release is
[`v0.1.0`](https://github.com/tylerbutler/acme/tree/v0.1.0), at commit
`997a365af1e2b167b2c209f84798006c77860feb`. The tag was retargeted with owner
approval; old clones can have its previous target. Use that exact commit
when you verify or build the release.

[`schema/factory.schema.json`](schema/factory.schema.json) is an unchanged
copy from
[that commit](https://github.com/tylerbutler/acme/blob/997a365af1e2b167b2c209f84798006c77860feb/schema/factory.schema.json).
Its SHA-256 is
`ed4ad9562a211a0fa2a61db237756d74508d388159d6017d11ded3817e032102`.
Use it to validate the complete TOML document as JSON. Do not remove fields
to make validation pass.

Acme is a separate tool. It is not a dependency of this crate, and this
template does not download or install it. The owner must explicitly install
the matching release binary before approved local generation. The required
`toolchain.gh_aw` field records the compatible compiler version; no agent
compiler is needed for this inactive baseline.

`acme init` is not implemented in `0.1.0`. Do not treat `render` or `sync`
as a completed init flow. With a reviewed config, the matching binary can
render into a new or empty temporary directory:

```sh
acme render --config factory/factory.toml --output /tmp/my-project-preview
```

With all modules off, the stub renders no files. There is no ownership
record yet. Generation does not provision labels or rulesets, upload
secrets, activate workflows, authorize inference, or approve a merge.
Those actions need separate owner approval. The later init change set
must use a branch and draft PR; a human reviews and merges it.

See the [M4 contract](https://github.com/tylerbutler/acme/blob/997a365af1e2b167b2c209f84798006c77860feb/docs/specs/0005-m4-init-template-contract.md)
and [baseline issue](https://github.com/tylerbutler/acme/issues/83) for scope.

## License

MIT OR Apache-2.0. See [LICENSE-MIT](LICENSE-MIT) and
[LICENSE-APACHE](LICENSE-APACHE).
