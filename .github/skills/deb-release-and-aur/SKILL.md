---
name: deb-release-and-aur
description: 'Build a Debian package with GitHub Actions, create and push a release tag, and prepare or publish an AUR package for this repository. Use when asked to build a .deb, trigger a GitHub release build, update PKGBUILD/.SRCINFO, or create an Arch Linux AUR package.'
argument-hint: 'Describe whether you want to trigger a tag build, verify release artifacts, update the AUR recipe, or publish to AUR.'
user-invocable: true
---

# Debian Release And AUR

Use this skill for the repo workflow that turns the Tauri app into a GitHub-built `.deb` and then packages that release for Arch Linux AUR.

## When To Use

- Build or rebuild the Linux `.deb` package through GitHub Actions.
- Create and push a `v*` tag to trigger the release build.
- Verify where the GitHub workflow uploads the `.deb` artifact.
- Update the repo's AUR package recipe and `.SRCINFO`.
- Prepare the commands needed to publish a package to AUR.

## Repo Facts

- The GitHub workflow is [build-deb.yml](../../workflows/build-deb.yml).
- The workflow is intentionally deb-only and runs `pnpm tauri build --bundles deb --ci`.
- The local output path for the Debian package is `backend/target/release/bundle/deb/*.deb`.
- The repo contains the published AUR template at `packaging/aur/myclash/`.
- The Debian workflow also syncs `packaging/aur/myclash/` after a successful `v*` release build.

## Procedure

1. Confirm the app version from `backend/tauri/tauri.conf.json` and inspect remote tags before creating a new one.
2. If a release build is requested, create an annotated `v*` tag and push it to `origin`.
3. Treat the GitHub Actions workflow as the source of truth for remote `.deb` builds; do not switch it back to a full `pnpm build` unless the user explicitly wants rpm/AppImage too.
4. When preparing AUR metadata, point `source` at the GitHub Release asset URL and update `pkgver`, `pkgrel`, and `sha256sums` together.
5. Keep a distinct AUR package name if the upstream AUR package name is already taken; this repo now publishes `myclash`.
6. After a successful tag build, treat `packaging/aur/myclash/` on `main` as the source of truth for later AUR pushes.
7. For AUR publication, follow the command sequence in [aur-publish.md](./references/aur-publish.md).

## Guardrails

- Do not overwrite user edits in existing workflow or packaging files without first reading their current contents.
- If local Tauri build output exists, prefer hashing the actual `.deb` file rather than guessing the checksum.
- If the user wants a new personal AUR package, avoid reusing an existing package name such as `clash-nyanpasu-bin`.
- If release asset names change, update both `PKGBUILD` and `.SRCINFO` in the same edit.

## References

- [AUR publish steps](./references/aur-publish.md)