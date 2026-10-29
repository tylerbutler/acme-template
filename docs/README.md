# Project documentation

Start with [`factory/brief.md`](../factory/brief.md). Keep durable feature
requirements in [`specs/`](specs/README.md) and architecture decisions in
[`adr/`](adr/README.md). Preserve these authored files during generation.

The factory config is an inactive stub for owner review. Its modules are
off. It names only the CLI/Rust preset and the adopted acme release.
The root [README](../README.md) records the release, schema and local checks.

Before future initialization, the owner reviews the complete config and
diff, then explicitly requests generation with a supported acme version.
Create a branch and one draft init PR. A human reviews and merges it.
Do not infer permission for paid work, provisioning or workflow activation.

Use the [proposal skills](../README.md#proposal-skills) to interview the owner
and propose config, then a small feature spec and backlog from the preserved
brief. The skills return drafts only; they do not write files or run acme.
Only the owner can approve each build after the spec is reviewed and merged.

Deterministic init is still separate work. Acme `0.1.0` does not implement
init. No sample-project milestone run has been performed.
