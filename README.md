# Dir2Txt

Simple tool to print the contents of all provided files or directories
(recursively) to stdout, to feed into an LLM's web interface for example.

Directory or filenames are passed via CLI args.

Dotfiles, and certain directories such as `.git`, or `node_modules` are ignored
unless passed deliberately.

## Usage Examples

```bash
# Fuzzy-find and pick files interactively
d2t $(fzf)

# Auto-copy output
d2t <files...> | wl-copy

# Auto-copy output of interactively selected files
d2t $(fzf) | wl-copy
```

## Future Work

- [ ] Ignore dot-directories; probably only need to canonicalize paths
- [ ] Respect .ignore files
