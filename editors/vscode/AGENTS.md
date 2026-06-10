# VS Code Agent Guide

Use this file when editing `editors/vscode/**`.

## Ownership

- `src/extension.ts`: activation, commands, lifecycle, and view wiring.
- `src/client.ts`: LSP client setup and middleware.
- `src/commands.ts`: CLI subprocess calls and command arguments.
- `src/download.ts`: managed binary download, verification, and resolution.
- `src/generated/output-contract.d.ts`: generated from `docs/output-schema.json`, never hand-edit.
- `dist/extension.js` and `dist/extension.js.map`: committed bundle artifacts.

## Rules

- Source changes that affect runtime extension code need a bundle rebuild before commit.
- Keep generated output types in sync with the Rust schema. Run codegen rather than editing generated files.
- Binary resolution order is user path, workspace dependency, system path, managed binary, then auto-download.
- Do not silently change the VS Code engine floor. It affects Cursor, Windsurf, and VS Code compatibility.
- Health, audit, and sidebar analyses are intentionally separate spawns. Do not fold them together unless the UX and latency tradeoff is explicit.

## Validation

- Source edit: run `pnpm --dir editors/vscode run lint` and the focused tests.
- Generated type edit: run `pnpm --dir editors/vscode run check:codegen`.
- Bundle-affecting edit: run `pnpm --dir editors/vscode run build` and `pnpm --dir editors/vscode run check:dist`.
- Packaging or manifest edit: run the manifest or packaging tests that cover the touched surface.
