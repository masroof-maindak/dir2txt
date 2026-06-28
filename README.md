# Dir2Txt

Simple tool to print the contents of all provided files or directories
(recursively) to stdout, to feed into an LLM's web interface for example.

Directory or filenames are passed via CLI args.

Hidden files, and certain directories such as `.git`, or `node_modules` are
ignored unless passed deliberately.

Yes, I am acutely aware that this entire repository could have been a handful of
`ripgrep` shell wrappers.

## Usage Examples

```bash
# Fuzzy-find and pick files interactively
d2t -- $(fzf)

# Auto-copy output
d2t -- <files...> | wl-copy

# Auto-copy output of interactively selected files
d2t -- $(fzf) | wl-copy
```

### Flags

```txt
$ d2t -h                                                                                                                         .
Convert a directory to text.

Usage: d2t [OPTIONS] [-- <PATHS>...]

Arguments:
  [PATHS]...  List of paths to print.

Options:
  -n, --names-only
  -E, --accept-empty   Accept empty files (disabled by default)
  -H, --accept-hidden  Accept hidden files (disabled by default)
  -h, --help           Print help
  -V, --version        Print version
```

## Future Work

- [x] Add `--names-only` flag
- [x] Incorporate `camino` & `anyhow` for better QoL
- [x] FIXME: verify that paths being printed even exist
- [x] FIXME: a file can appear in the output list multiple times
- [x] Ignore empty files by default. Let them pass via `--accept-empty`
- [x] Let hidden files pass via `--accept-hidden`
- [ ] Rework built-in ignoring
  - [ ] Add `--ignore`/`-i` flag to remove a given file from the output
  - [ ] Add `--no-ignore`/`u` flag to ALLOW an ignored directory/file that's in
        the 'ignore-list'
  - Ensure explicitly specified files do NOT get ignored
  - See <https://iepathos.github.io/ripgrep/automatic-filtering/#overview> for
    more info
- [ ] Respect .ignore files
  - See: <https://docs.rs/ignore/latest/ignore/>
