# Nightingale

## Release notes

Generate draft release notes from git history:

```sh
bundle exec rake release:notes VERSION=0.1.1
```

Write the generated notes into `CHANGELOG.md` under `Unreleased`:

```sh
bundle exec rake release:changelog VERSION=0.1.1
```

Both tasks use the latest git tag as the starting point. If no tag exists yet, they use the full history. You can override the range with `FROM=<ref>` and `TO=<ref>`.
