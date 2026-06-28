# Dir2Txt

Simple tool to print the contents of all provided files or directories
(recursively) to stdout, to feed into an LLM's web interface for example.

Directory or filenames are passed via CLI args.

Dotfiles, and certain directories such as `.git`, or `node_modules` are ignored
unless passed deliberately.

## Usage Examples

```bash
# Fuzzy-find and pick files interactively
d2t -- $(fzf)

# Auto-copy output
d2t -- <files...> | wl-copy

# Auto-copy output of interactively selected files
d2t -- $(fzf) | wl-copy
```

## Future Work

- [x] Add `--names-only` flag
- [x] Incorporate `camino` & `anyhow` for better QoL
- [x] FIXME: verify that paths being printed even exist
- [x] FIXME: a file can appear in the output list multiple times
- [x] Ignore empty files by default. Let them pass via `--accept-empty`
- [x] Let hidden files pass via `--accept-hidden`
- [ ] FIXME: allow ignored dirs and files into the output list if passed
      deliberately
- [ ] Add `--ignore`/`-i` flag to remove a given file from the output
- [ ] Respect .ignore files
  - See: <https://docs.rs/ignore/latest/ignore/>
