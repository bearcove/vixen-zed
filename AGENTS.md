# AGENTS.md

When bumping the Vixen grammar in this repo:

1. Work in `/Users/amos/bearcove/tree-sitter-vixen`.
2. Make the grammar/query changes there first.
3. Run `npm run generate` and `npm test` in `tree-sitter-vixen`.
4. Commit and push `tree-sitter-vixen`.
5. Copy the updated query files from `tree-sitter-vixen/queries/` into `vixen-zed/languages/vixen/`:
   - `highlights.scm`
   - `brackets.scm`
   - `indents.scm`
   - `injections.scm`
6. Update `extension.toml` to the pushed `tree-sitter-vixen` commit SHA.
7. Commit and push `vixen-zed`.

Notes:

- Treat `/Users/amos/bearcove/vixen` as the language reference when the grammar change depends on current language syntax.
- If syntax changes affect highlighting or injections, update the language query files here to match the grammar repo rather than editing them independently.
