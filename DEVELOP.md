# For Developers

## Release Process

1. Update version in `Cargo.toml` on develop branch:

```toml
[package]
name = "jgze"
version = "1.1.0"  # Update this version(ex: 1.0.0 -> 1.1.0)
```

2. Update `CHANGELOG.md` on develop branch:

- Move changes from "Unreleased" to "Released"
- Add release date

```markdown
# Changelog

## [Unreleased]

## [Released]

### [1.1.0] - 2024-11-26

#### Feature
- xxx

### [1.0.0] - 2024-11-17
```

3. Update cargo.lock

```bash
cargo build
```

4. Commit and push changes to develop branch:

```bash
git add .
git commit -m "Prepare for v1.1.0 release"
git push origin develop
```

5. Merge develop branch into release branch:

```bash
git checkout release
git merge --no-ff develop
git push origin release
```

6. Create and push a new tag on release branch:

```bash
git tag v1.1.0
git push origin v1.1.0
```

This will trigger the GitHub Actions workflow that:

- Creates a new GitHub release
- Builds binaries
- Uploads the binaries to the release page

### Supported Platforms

The automated builds create binaries for:

| Platform | Architecture | Target Triple | File Name |
|----------|-------------|---------------|-----------|
| Linux | x86_64 | x86_64-unknown-linux-gnu | jgze-x86_64-linux.tar.gz |
| macOS | x86_64 | x86_64-apple-darwin | jgze-x86_64-darwin.tar.gz |
| macOS | aarch64 | aarch64-apple-darwin | jgze-aarch64-darwin.tar.gz |
| Windows | x86_64 | x86_64-pc-windows-msvc | jgze-x86_64-windows.zip |

7. Publish to crates.io:

```bash
cargo login
cargo publish
```

8. Merge release branch into main branch:

```bash
git checkout main
git merge --no-ff release
git push origin main
```
