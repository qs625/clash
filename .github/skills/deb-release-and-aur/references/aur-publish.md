# AUR Publish Steps

Use these commands after the GitHub release already contains the `.deb` asset referenced by the package recipe.

## 1. Create The AUR Repository

Register on AUR, add your SSH public key, then clone the package repository:

```bash
git clone ssh://aur@aur.archlinux.org/myclash.git
cd myclash
```

If the package does not exist yet, first create it from the AUR web UI with the exact package name.

## 2. Copy The Recipe Files

Copy these files from the repo into the AUR clone:

- `packaging/aur/myclash/PKGBUILD`
- `packaging/aur/myclash/.SRCINFO`

If the release was built from a `v*` tag after the sync workflow was added, these files on `main` should already match the GitHub release asset.

## 3. Validate Locally

Run:

```bash
bash -n PKGBUILD
makepkg --printsrcinfo > .SRCINFO
```

If you changed the release asset or version, recompute the checksum before publishing:

```bash
sha256sum 'Clash Nyanpasu_<version>_amd64.deb'
```

Then update both `PKGBUILD` and `.SRCINFO`.

## 4. Push To AUR

Run:

```bash
git add PKGBUILD .SRCINFO
git commit -m "Initial import"
git push
```

## 5. Expected Release Alignment

- `pkgver` must match the GitHub tag version without the leading `v`.
- `source` must point to the actual release asset URL.
- `sha256sums` must match the exact uploaded `.deb`.
