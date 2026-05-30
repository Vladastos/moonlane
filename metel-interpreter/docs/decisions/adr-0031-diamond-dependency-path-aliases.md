---
id: adr-0031
title: "Diamond Dependency Resolved via Path Aliases in the Module Loader"
date: '2026-05-30'
status: active
---

## Context

Module paths in Metel are computed relative to the importing parent module (ADR-0023). This means the same physical `.mtl` file can be reached via multiple logical paths in a diamond dependency graph:

```
main.mtl
├── import left::...     → left.mtl (path: ["left"])
│   └── import base::...  → base.mtl (path: ["left", "base"])  ← first visit
└── import right::...    → right.mtl (path: ["right"])
    └── import base::...  → base.mtl (path: ["right", "base"]) ← second visit, same file
```

Before v0.6.3, `module_loader::Loader` silently skipped already-visited files (`if self.visited.contains(&file_path) { return Ok(()); }`). This left `base.mtl` registered only under `["left", "base"]` in `ModuleGraph::modules`. When the name resolver processed `right.mtl`'s imports, it computed `source_module = ["right", "base"]` — a path with no entry in `pub_surface` or the type registry — causing T0003 on any name imported from `base` through `right`.

## Decision

The loader tracks two additional maps:

- `file_to_path: HashMap<PathBuf, Vec<String>>` — records the canonical (first-assigned) logical path for each physical file.
- `path_aliases: HashMap<Vec<String>, Vec<String>>` — maps alias paths to their canonical path. Populated when a visited file is reached via a new logical path.

Both maps are exposed on `ModuleGraph` (only `path_aliases` is used downstream; `file_to_path` is loader-internal).

The name resolver threads `path_aliases` through `resolve → resolve_module → collect_re_exports → process_tree → process_export_tree`. A `canonical_path` helper performs prefix-aware dereferencing: for any module path `p`, it finds the longest prefix that has an alias and substitutes the canonical prefix, preserving any suffix. This handles direct aliases (`["right", "base"] → ["left", "base"]`) and nested sub-module aliases (`["right", "base", "sub"] → ["left", "base", "sub"]`).

## Alternatives considered

1. **Load the module again under the new path** — rejected. This would add a duplicate `LoadedModule` to the graph, causing the typechecker and evaluator to process `base.mtl` twice, producing duplicate declarations and possible double-registration errors.

2. **Add aliased paths to `known_modules`** — rejected. `known_modules` is used to distinguish module-handle imports from item imports. Adding alias paths corrupts this check without fixing the `source_module` field in name bindings.

3. **Post-process pass to canonicalize all `source_module` fields after resolution** — functionally equivalent, but inline dereferencing in `process_tree`/`process_export_tree` is simpler and avoids a second traversal of `ResolvedNames`.

4. **Store modules by file path instead of logical path** — rejected. Logical paths are the identity used throughout the type checker, evaluator, and import system. Changing the identity would require broad changes to the pipeline.

## Consequences

- Diamond dependencies are transparent to the type checker and evaluator: all references to `base` resolve to `["left", "base"]` regardless of which importer introduced them.
- The canonical path is determined by the order modules are visited (depth-first, left-to-right), which is stable for a given module graph.
- Alias dereferencing is not recursive (an alias cannot point to another alias) because `file_to_path` maps to the path recorded on first visit, which is always canonical.

## Constraints for future contributors

- `path_aliases` must be consulted whenever a `source_module` or glob base is derived from an import path in the name resolver. The `canonical_path` helper in `name_resolver.rs` centralises this. Do not inline alias lookups elsewhere.
- The canonical path for a module is the path assigned on first visit by the loader. If the load order changes (e.g. breadth-first traversal), the canonical path may change, which is fine as long as it is consistent within a single compilation.
- `path_aliases` is intentionally not extended to `known_modules`. A path alias is not a real module — adding it to `known_modules` would make it appear as a loadable module, which it is not.
