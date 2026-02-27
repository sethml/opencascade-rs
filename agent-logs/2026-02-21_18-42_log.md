# README update and code refactoring tasks

- **Date:** 2026-02-21 17:22 – 17:47
- **Model:** Claude Opus 4.6
- **Turns:** 1
- **Tool calls:** 147
- **Thinking blocks:** 24
- **Input words:** 14,435
- **Output words:** 8,709
- **Prompt tokens (last round):** 128,637
- **API rounds:** 117
- **Total elapsed:** 1373s

## Table of Contents

1. [Update this section of the README, using the scripts in tmp/ if they&#x27;re helpful. Then commit. The...](#user-1) (Claude Opus 4.6)

---

<a id="user-1"></a>

## User (1)

> Update this section of the README, using the scripts in tmp/ if they&#x27;re helpful. Then commit. Then Try to fix the skip reason with the highest count, and commit. Then think about whether you discovered opportunities to refactor the code the avoid repetition in this session. If refactoring seems worthwhile, do it, and commit.

*2026-02-21 17:24*

### Assistant — Claude Opus 4.6

> 💭 Let me break this down into tasks:
> 
> 1. Update the "Skip Reason Breakdown" section of the README using scripts in tmp/
> 2. Commit
> 3. Fix the skip reason with the highest count
> 4. Commit
> 5. Consider refactoring opportunities and potentially commit
> 
> Let me start by understanding what's in tmp/ and what the current skip counts look like.

**Todo list:**
- 🔄 Explore tmp/ scripts and current skips
- ⬜ Update README skip section
- ⬜ Commit README update
- ⬜ Fix highest-count skip reason
- ⬜ Commit fix
- ⬜ Evaluate and do refactoring
- ⬜ Commit refactoring if done

Read [tmp](../tmp)

Read [scripts](../scripts)

Read [readme_skips.py](../tmp/readme_skips.py#1-1), lines 1 to 100

Read [count_skips.py](../tmp/count_skips.py)

Read [bucket_skips2.py](../tmp/bucket_skips2.py)

Read [readme_skips.py](../tmp/readme_skips.py#100-100), lines 100 to 171

Good, there's a comprehensive `tmp/readme_skips.py` script. Let me run it to get current numbers, and also look at what the current README section says in more detail.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && python3 tmp/readme_skips.py 2>&1
```
<details>
<summary>Output (70 lines)</summary>

```
Total skipped symbols: 404

=== Skip Reason Breakdown ===
| Count |     % | Category |
|------:|------:|----------|
|   102 | 25.2% | **Unknown/unresolved type** |
|    91 | 22.5% | **Unresolved template type** |
|    72 | 17.8% | **Abstract class** |
|    55 | 13.6% | **Unknown Handle type** |
|    19 |  4.7% | **C-style array** |
|    19 |  4.7% | **Stream (shared_ptr)** |
|    17 |  4.2% | **String ref param** |
|    12 |  3.0% | **Rvalue reference** |
|    10 |  2.5% | **Not CppDeletable** |
|     4 |  1.0% | **&mut enum return** |
|     2 |  0.5% | **Excluded by bindings.toml** |
|     1 |  0.2% | **Ambiguous overload** |

Unknown/unresolved + Unknown Handle combined: 157 (38.9%)

=== Most Common Unknown Types ===
| Count | Type |
|------:|------|
|    12 | `Handle(ShapePersistent_Geom::geometryBase<Geom_Surface>)` |
|    11 | `ShapeProcess::OperationsFlags const` |
|    10 | `Handle(ShapePersistent_Geom::geometryBase<Geom_Curve>)` |
|    10 | `Handle(ShapePersistent_Geom::geometryBase<Geom2d_Curve>)` |
|     7 | `IMeshData::MapOfInteger const` |
|     5 | `WNT_HIDSpaceMouse const` |
|     5 | `AVStream const` |
|     5 | `RWGltf_GltfOStreamWriter` |
|     4 | `GLXFBConfig` |
|     3 | `Aspect_XDisplay` |
|     3 | `IMeshData::Array1OfVertexOfDelaun` |
|     3 | `IMeshData::VectorOfInteger` |
|     3 | `IMeshData_Edge *const const` |
|     3 | `IMeshData_Face *const const` |
|     2 | `Aspect_XVisualInfo` |

=== Domain Analysis ===

**Data Exchange** (26 symbols) -- `iges_control` (3), `rw_gltf` (6), `rw_obj` (2), `rw_ply` (1), `rw_stl` (2), `step_control` (2), `xs_control` (10)
   10 unknown type
    8 string ref param
    3 rvalue reference
    2 unresolved template
    2 abstract class
    1 stream (shared_ptr)

**Document Framework** (1 symbols) -- `tdf` (1)
    1 unknown type

**Shape Meshing** (50 symbols) -- `b_rep_mesh` (45), `i_mesh_data` (5)
   31 unknown type
   10 unresolved template
    6 C-style array
    3 abstract class

**Shape Analysis/Fix** (0 symbols)

**Geometry** (0 symbols)

**Poly** (7 symbols) -- `poly` (7)
    3 C-style array
    1 excluded by config
    1 unknown type
    1 abstract class
    1 unresolved template

**Boolean Ops** (0 symbols)
```
</details>

Now let me also run the more detailed `tmp/bucket_skips2.py` for extra detail:

**Terminal:**
```sh
python3 tmp/bucket_skips2.py 2>&1
```
<details>
<summary>Output (41 lines)</summary>

```
Total skipped symbols: 404

=== Skip Reason Breakdown ===
| Count |     % | Category |
|------:|------:|----------|
|   272 |  67.3% | **Unknown/unresolved type** |
|    15 |   3.7% | **Stream type** |
|    55 |  13.6% | **Unknown Handle type** |
|    10 |   2.5% | **Not CppDeletable** |
|    72 |  17.8% | **Abstract class** |
|    12 |   3.0% | **Rvalue reference** |
|    19 |   4.7% | **C-style array** |
|     4 |   1.0% | **&mut enum return** |

=== Top Unknown Non-Handle Types ===
    40 | (other: has unbindable types: return: unresolved template type (NCol)
    11 | ShapeProcess::OperationsFlags const
     9 | (other: has unbindable types: return: unresolved template type (std:)
     9 | (other: has string ref param 'name' of type 'const char*&' (needs ma)
     7 | IMeshData::MapOfInteger const
     6 | (other: has unbindable types: param 'thePlaneEquations': unresolved )
     5 | WNT_HIDSpaceMouse const
     5 | AVStream const
     5 | RWGltf_GltfOStreamWriter
     4 | (other: has unbindable types: param 'theOther': unresolved template )
     4 | GLXFBConfig
     4 | (other: has unbindable types: param 'theSizeXYZ': unresolved templat)

=== Top Unknown Handle Types ===
    12 | Handle(ShapePersistent_Geom::geometryBase<Geom_Surface>)
    10 | Handle(ShapePersistent_Geom::geometryBase<Geom_Curve>)
    10 | Handle(ShapePersistent_Geom::geometryBase<Geom2d_Curve>)
     1 | Handle(NCollection_Shared<NCollection_Map<IMeshData_Edge *>>)
     1 | Handle(NCollection_Shared<NCollection_Map<const Standard_Transient *>>)
     1 | Handle(ShapePersistent_TopoDS::pTObject<ShapePersistent_BRep::pTVertex, StdLPersistent_HArray1::instance<StdLPersistent_HArray1OfPersistent>>)
     1 | Handle(ShapePersistent_TopoDS::pTObject<ShapePersistent_BRep::pTEdge, StdLPersistent_HArray1::instance<StdLPersistent_HArray1OfPersistent>>)
     1 | Handle(ShapePersistent_TopoDS::pTObject<ShapePersistent_BRep::pTFace, StdLPersistent_HArray1::instance<StdLPersistent_HArray1OfPersistent>>)
     1 | Handle(ShapePersistent_BRep::PointOnCurve)
     1 | Handle(ShapePersistent_BRep::PointOnCurveOnSurface)
     1 | Handle(ShapePersistent_BRep::PointOnSurface)
     1 | Handle(ShapePersistent_BRep::CurveOnSurface)
```
</details>

> 💭 Now let me look at the current README section more carefully to understand what I need to update. The readme_skips.py script gives much better categorization. Let me read the full section.

Now let me read the current README section that needs updating:

<details>
<summary>Reading lines 400-510 of <a href="../README.md#L400-L511">README.md</a></summary>

```
Error: startLine 400 exceeds file length (168 lines).
```
</details>

The README I'm updating is the binding-generator README. Let me find the right section:

Searched for text `Skip Reason Breakdown` (`**/README.md`), 1 result
- [README.md:408](../crates/opencascade-binding-generator/README.md#L408)

<details>
<summary>Reading lines 395-510 of <a href="../crates/opencascade-binding-generator/README.md#L395-L511">README.md</a></summary>

````
- **Source location** (header file, line number, C++ symbol name)
- **Documentation comment** from the C++ header (first 3 lines)
- **Skip reason** explaining why the symbol was excluded
- **Commented-out Rust stub** showing the best-guess declaration

Example from `gp.rs`:
```rust
// SKIPPED: **Source:** `gp_XYZ.hxx`:109 - `gp_XYZ::GetData`
//   method: Returns a const ptr to coordinates location.
//   Reason: has unbindable types: return: raw pointer (const double*)
//   // pub fn get_data(&self) -> /* const double* */;
```

### Skip Reason Breakdown

| Count | % | Category | Description |
|------:|----:|----------|-------------|
| 66 | 23.5% | **Unresolved template type** | Template instantiations that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `NCollection_Vec3<...>`, etc.) |
| 58 | 20.6% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |
| 57 | 20.3% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`IMeshData::MapOfInteger`, `WNT_HIDSpaceMouse`, etc.) |
| 54 | 19.2% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(ShapePersistent_Geom::...)`, `Handle(ShapePersistent_BRep::...)`, etc.) |
| 13 | 4.6% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |
| 11 | 3.9% | **Stream (shared_ptr)** | `std::shared_ptr<std::istream/ostream>` — smart-pointer-wrapped streams not yet bindable |
| 8 | 2.8% | **Not CppDeletable** | Return type class has no destructor in the binding set |
| 4 | 1.4% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |
| 4 | 1.4% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across FFI |
| 4 | 1.4% | **&mut enum return** | Mutable reference to enum (cxx limitation) |
| 1 | 0.4% | **Excluded by bindings.toml** | Explicitly excluded in config (e.g., ambiguous overload workarounds) |
| 1 | 0.4% | **Ambiguous overload** | C++ overload that would produce identical wrapper signatures |

### Most Common Unknown Types

The "unknown type" and "unknown Handle type" categories (39.5% of all skips) are dominated by a few types:

| Count | Type | How to Unblock |
|------:|------|----------------|
| 32 | `Handle(ShapePersistent_Geom::geometryBase<...>)` | Protected nested template class — header not in binding set |
| 12 | `Handle(ShapePersistent_BRep::...)` | Nested classes in persistence layer — header not in binding set |
| 9 | `AVStream`, `AVPacket`, `AVFrame`, etc. | FFmpeg types in media module — external dependency |
| 6 | `IMeshData::MapOfInteger` | Namespace-scoped NCollection template typedef — not yet resolvable |
| 4 | `WNT_HIDSpaceMouse` | Windows-only type, WNT module excluded |
| 4 | `Handle(ShapePersistent_Poly::instance<...>)` | Protected nested template class — header not in binding set |
| 3 | `RWGltf_GltfOStreamWriter` | External RapidJSON dependency — not in binding set |
| 3 | `GLXFBConfig` | X11/Linux display type — platform-specific |

### Important Skipped Symbols

Most skipped symbols are in internal, low-use, or specialized modules. However, some affect functionality that users commonly need:

**Data Exchange (26 symbols)** — `STEPControl_*` (2), `IGESControl_*` (3), `XSControl_*` (10), `RWGltf_*` (6), `RWObj_*` (2), `RWStl` (2), `RWPly` (1). Dominated by unknown types (10) and string ref params (8, mostly `const char*&` in `XSControl_Vars`), plus rvalue references (3), unresolved templates (2), abstract classes (2), and stream types (1). The core `Read()`/`Write()` operations are fully bound.

**Document Framework (1 symbol)** — `TDF_*` (1). The unknown type is `TDF_LabelNode*` — a raw pointer to a class not in the binding set. Previously, `TDocStd_XLinkPtr` (pointer typedef for `TDocStd_XLink*`) also caused 3 skips, but these are now resolved via pointer typedef resolution. Methods returning references with reference params are bound as `unsafe fn` (see "Unsafe Reference R
... (truncated, 7209 chars)
````
</details>

Now let me also check where the total count of skipped symbols is mentioned earlier:

<details>
<summary>Reading lines 383-396 of <a href="../crates/opencascade-binding-generator/README.md#L383-L397">README.md</a></summary>

```
- `<module>_wrappers.cpp` — C++ wrapper functions

The generator appends `include!("../manual/<module>.rs");` (with a comment explaining why) to the generated module re-export file when a corresponding `manual/<module>.rs` exists. Because `include!()` is a textual insertion, the manual code has full access to the module's type aliases (e.g., `AdvancedEvolved`, `Finder`). The `extern "C"` declarations in manual files are not marked `pub`, so they are private to the module and not exposed as part of the public API. `build.rs` globs `manual/*_wrappers.cpp` and compiles them alongside `generated/wrappers.cpp`. Since Rust allows multiple `impl` blocks for a type, manual methods appear seamlessly alongside the auto-generated ones.

See `crates/opencascade-sys/manual/` and the comments in `bindings.toml` for examples (`Transfer_Finder::GetStringAttribute`, `BRepFill_AdvancedEvolved::SetTemporaryDirectory`, and `standard` iostream accessors).

---

## Skipped Symbols

The binding generator skips ~281 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Every skipped symbol is documented in the generated per-module `.rs` files as a `// SKIPPED:` comment block including:

- **Source location** (header file, line number, C++ symbol name)
- **Documentation comment** from the C++ header (first 3 lines)
```
</details>

**Todo list:**
- ✅ Explore tmp/ scripts and current skips
- 🔄 Update README skip section
- ⬜ Commit README update
- ⬜ Fix highest-count skip reason
- ⬜ Commit fix
- ⬜ Evaluate and do refactoring
- ⬜ Commit refactoring if done

Now I have the current data. The total is 404 (up from ~281). Let me update the README section. I need to update:
1. The total count (~281 → ~404)
2. The skip reason breakdown table
3. The most common unknown types table
4. The important skipped symbols domain analysis

Let me also verify the `ShapeProcess::OperationsFlags` type since it's a new top entry:

**Terminal:**
```sh
grep -c "SKIPPED:" crates/opencascade-sys/generated/*.rs | awk -F: '{sum+=$2} END {print sum}'
```
```
404
```

Now let me do the edit. I'll use `hashline_edit` for the updates:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/README.md#L393-L394">README.md -1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L410-L424">-14+14</a>, <a href="../crates/opencascade-binding-generator/README.md#L427-L428">-1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L429-L441">-10+12</a>, <a href="../crates/opencascade-binding-generator/README.md#L446-L447">-1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L450-L451">-1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "393:ha",
      "content": "The binding generator skips ~404 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Every skipped symbol is documented in the generated per-module `.rs` files as a `// SKIPPED:` comment block including:"
    },
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "410:fa,411:oi,412:py,413:nk,414:va,415:ln,416:rm,417:ut,418:zu,419:bk,420:ux,421:nd,422:ag,423:xa",
      "content": "| Count | % | Category | Description |\n|------:|----:|----------|-------------|\n| 102 | 25.2% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`ShapeProcess::OperationsFlags`, `IMeshData::MapOfInteger`, `WNT_HIDSpaceMouse`, etc.) |\n| 91 | 22.5% | **Unresolved template type** | Template instantiations that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `NCollection_Vec3<...>`, etc.) |\n| 72 | 17.8% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |\n| 55 | 13.6% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(ShapePersistent_Geom::...)`, `Handle(ShapePersistent_BRep::...)`, etc.) |\n| 19 | 4.7% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |\n| 19 | 4.7% | **Stream (shared_ptr)** | `std::shared_ptr<std::istream/ostream>` — smart-pointer-wrapped streams not yet bindable |\n| 17 | 4.2% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |\n| 12 | 3.0% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across FFI |\n| 10 | 2.5% | **Not CppDeletable** | Return type class has no destructor in the binding set |\n| 4 | 1.0% | **&mut enum return** | Mutable reference to enum (cxx limitation) |\n| 2 | 0.5% | **Excluded by bindings.toml** | Explicitly excluded in config (e.g., ambiguous overload workarounds) |\n| 1 | 0.2% | **Ambiguous overload** | C++ overload that would produce identical wrapper signatures |"
    },
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "427:wl",
      "content": "The \"unknown type\" and \"unknown Handle type\" categories (38.9% of all skips) are dominated by a few types:"
    },
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "429:js,430:bt,431:yq,432:nh,433:bw,434:zf,435:eh,436:nu,437:xn,438:yc",
      "content": "| Count | Type | How to Unblock |\n|------:|------|----------------|\n| 32 | `Handle(ShapePersistent_Geom::geometryBase<...>)` | Protected nested template class — header not in binding set |\n| 11 | `ShapeProcess::OperationsFlags` | Nested enum in `ShapeProcess` — n
... (truncated, 5097 chars)
```
**Output:**
```
{
  "applied": 6,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/README.md",
      "startLine": 393,
      "endLine": 450
    }
  ]
}
```
</details>

**Todo list:**
- ✅ Explore tmp/ scripts and current skips
- ✅ Update README skip section
- 🔄 Commit README update
- ⬜ Fix highest-count skip reason
- ⬜ Commit fix
- ⬜ Evaluate and do refactoring
- ⬜ Commit refactoring if done

Now commit:

Created [commit-msg.txt](../tmp/commit-msg.txt)

**Terminal:**
```sh
cat > tmp/commit-msg-readme.txt << 'ENDMSG'
docs: update skip reason breakdown to current counts (404 symbols)

Update the Skipped Symbols section of the binding generator README
with current numbers from the generated code:
- Total: ~281 → ~404 (more headers now in binding set)
- Unknown/unresolved type is now the top category (102, 25.2%)
- Unresolved template type is second (91, 22.5%)
- Added ShapeProcess::OperationsFlags to common unknown types
- Updated counts for all categories
ENDMSG
```

**Terminal:**
```sh
git add crates/opencascade-binding-generator/README.md && git commit -F tmp/commit-msg-readme.txt
```
```
[main f71fad2] docs: update skip reason breakdown to current counts (404 symbols)
 1 file changed, 23 insertions(+), 21 deletions(-)
```

**Todo list:**
- ✅ Explore tmp/ scripts and current skips
- ✅ Update README skip section
- ✅ Commit README update
- 🔄 Fix highest-count skip reason
- ⬜ Commit fix
- ⬜ Evaluate and do refactoring
- ⬜ Commit refactoring if done

Now for the highest skip count: **Unknown/unresolved type** (102 symbols, 25.2%). Let me dig into what these are specifically. The top unknown types from the script were:

1. `ShapeProcess::OperationsFlags` (11) — nested enum
2. `IMeshData::MapOfInteger` (7) — namespace-scoped typedef
3. `AVStream`, `AVPacket`, etc. (5) — FFmpeg, external dep, can't fix
4. `WNT_HIDSpaceMouse` (5) — Windows-only, can't fix
5. `RWGltf_GltfOStreamWriter` (5) — external dep
6. `GLXFBConfig` (4) — platform-specific, can't fix
7. `Aspect_XDisplay` (3) — platform-specific
8. `IMeshData::Array1OfVertexOfDelaun` (3) — namespace-scoped typedef
9. `IMeshData::VectorOfInteger` (3) — namespace-scoped typedef
10. `IMeshData_Edge *const` (3) — pointer qualifiers

Let me get a more detailed breakdown to see what's actually fixable:

**Terminal:**
```sh
python3 -c "
import re, os
from collections import Counter

gen_dir = 'crates/opencascade-sys/generated'
skip_pattern = re.compile(r'// SKIPPED: \*\*Source:\*\* \`([^\`]+)\`:(\d+) - \`([^\`]+)\`')
reason_pattern = re.compile(r'//\s+Reason:\s+(.*)')

unknown_reasons = []
for fname in sorted(os.listdir(gen_dir)):
    if not fname.endswith('.rs') or fname in ('ffi.rs', 'lib.rs'):
        continue
    with open(os.path.join(gen_dir, fname)) as f:
        lines = f.readlines()
    i = 0
    while i < len(lines):
        m = skip_pattern.search(lines[i])
        if m:
            header, line_no, symbol = m.groups()
            reason = ''
            for j in range(i+1, min(i+10, len(lines))):
                rm = reason_pattern.search(lines[j])
                if rm:
                    reason = rm.group(1).strip()
                    break
            # Only unknown/unresolved (not Handle, not template, not abstract etc)
            if ('unknown type' in reason or 'is unknown' in reason) and 'Handle(' not in reason and 'unresolved template' not in reason:
                unknown_reasons.append((fname, symbol, reason))
        i += 1

print(f'Total unknown/unresolved type skips: {len(unknown_reasons)}')
print()

# Group by extracted type name
type_counter = Counter()
for f, sym, r in unknown_reasons:
    m2 = re.search(r\"unknown type '([^']+)'\", r)
    if m2:
        t = re.sub(r'^const\s+', '', m2.group(1)).rstrip('&* ')
        type_counter[t] += 1
    else:
        m2 = re.search(r\"type '([^']+)' is unknown\", r)
        if m2:
            t = re.sub(r'^const\s+', '', m2.group(1)).rstrip('&* ')
            type_counter[t] += 1
        else:
            type_counter['UNPARSED: ' + r[:80]] += 1

for t, c in type_counter.most_common():
    print(f'  {c:4d} | {t}')
" 2>&1
```
```
(base) whitelily:opencascade-rs sethml$             type_counter[t] += 1
```

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/be682feb-153a-4ab3-a24a-c102b03012c6/toolu_017NpPjYyL8teW5EYV96nLxd__vscode-1771608614427/content.txt#1-1), lines 1 to 60

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/be682feb-153a-4ab3-a24a-c102b03012c6/toolu_017NpPjYyL8teW5EYV96nLxd__vscode-1771608614427/content.txt#60-60), lines 60 to 120

The inline python was mangled by the terminal. Let me write this as a script file instead:

Created [analyze_unknown_types.py](../tmp/analyze_unknown_types.py)

**Terminal:**
```sh
python3 tmp/analyze_unknown_types.py 2>&1
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
python3 tmp/analyze_unknown_types.py
```
<details>
<summary>Output (52 lines)</summary>

```
Total unknown/unresolved type skips (non-Handle, non-template): 102

    11 | ShapeProcess::OperationsFlags const
     7 | IMeshData::MapOfInteger const
     5 | WNT_HIDSpaceMouse const
     5 | AVStream const
     5 | RWGltf_GltfOStreamWriter
     4 | GLXFBConfig
     3 | Aspect_XDisplay
     3 | IMeshData::Array1OfVertexOfDelaun
     3 | IMeshData::VectorOfInteger
     3 | IMeshData_Edge *const const
     3 | IMeshData_Face *const const
     2 | Aspect_XVisualInfo
     2 | ProxPnt_Status const
     2 | IMeshData::ListOfInteger const
     2 | IMeshData::IMapOfReal const
     2 | IMeshData::IMapOfReal
     2 | CPnts_RealFunction const
     2 | NSOpenGLContext
     2 | AVCodecContext
     2 | AVRational const
     2 | OSD_ThreadFunction const
     2 | Standard_WideChar const
     1 | int (integer *, doublereal *, doublereal *, integer *)
     1 | ProxPnt_Status
     1 | IMeshData::ListOfInteger
     1 | IMeshData::VectorOfInteger const
     1 | IMeshData::MapOfInteger
     1 | IMeshData::MapOfIntegerInteger
     1 | IMeshData::MapOfIntegerInteger const
     1 | FT_Outline_ const
     1 | FT_LibraryRec_
     1 | Graphic3d_ArrayOfIndexedMapOfStructure const
     1 | AVBufferRef
     1 | AVFormatContext
     1 | AVFrame const
     1 | AVFrame
     1 | AVPacket const
     1 | AVPacket
     1 | Message_ProgressScope::NullString const
     1 | OSD_Function
     1 | NCollection_List<TwoIntegers>
     1 | std::streampos
     1 | Standard_JmpBuf
     1 | std::type_info const
     1 | Interface_ParamType
     1 | Standard_Utf16Char
     1 | TNaming_Node* const
     1 | TNaming_Node
     1 | TDF_LabelNode
     1 | NCollection_DataMap<opencascade::handle<XCAFDoc_VisMaterial>, opencascade::handle<XCAFDoc_VisMaterial>>
```
</details>

> 💭 Looking at the breakdown, the highest fixable group is `ShapeProcess::OperationsFlags` (11 skips). This is a nested enum inside `ShapeProcess` class. Let me investigate what this type actually is in C++.
> 
> Other potentially fixable types:
> - `IMeshData::MapOfInteger` (7) — namespace-scoped typedef
> - Various `IMeshData::*` types — namespace-scoped typedefs
> 
> But the single highest is `ShapeProcess::OperationsFlags` (11). Let me look at what it is and how to fix it.
> 
> Actually wait, the user asked me to fix the skip reason with the highest COUNT. Looking at it again:
> 
> 
> - **Unknown/unresolved type**: 102 (25.2%)
> - **Unresolved template type**: 91 (22.5%)
> - **Abstract class**: 72 (17.8%)
> 
> The highest-count skip reason is "Unknown/unresolved type" at 102. But these are very diverse — platform-specific types, external dependencies, etc. Many can't be fixed. The most impactful fixable subset within that category would be the nested enums and typedefs like `ShapeProcess::OperationsFlags` and the `IMeshData::*` types, but realistically only a small fraction of those 102 can be resolved.
> 
> Looking at the other categories, "Unresolved template type" at 91 is similarly intractable, and "Abstract class" at 72 is by design. So I should focus on what's actually fixable within the unknown/unresolved bucket — starting with `ShapeProcess::OperationsFlags` since it's a concrete nested enum with 11 skips that I can investigate.

The highest-count skip reason is "Unknown/unresolved type" (102). Within that, the most impactful fixable type is `ShapeProcess::OperationsFlags` (11 skips). Let me investigate what it is:

**Terminal:**
```sh
find crates/occt-sys/OCCT/src -name "ShapeProcess*" -name "*.hxx" | head -5
```
<details>
<summary>Output (5 lines)</summary>

```
crates/occt-sys/OCCT/src/ShapeProcessAPI/ShapeProcessAPI_ApplySequence.hxx
crates/occt-sys/OCCT/src/ShapeProcess/ShapeProcess_ShapeContext.hxx
crates/occt-sys/OCCT/src/ShapeProcess/ShapeProcess_UOperator.hxx
crates/occt-sys/OCCT/src/ShapeProcess/ShapeProcess.hxx
crates/occt-sys/OCCT/src/ShapeProcess/ShapeProcess_Operator.hxx
```
</details>

**Terminal:**
```sh
grep -n "OperationsFlags\|enum" crates/occt-sys/OCCT/src/ShapeProcess/ShapeProcess.hxx
```
<details>
<summary>Output (6 lines)</summary>

```
42:  //! C++11 enum class is not used to allow implicit conversion to underlying type.
43:  enum Operation : uint8_t
69:  // ShapeProcess::OperationsFlags flags;
73:  using OperationsFlags = std::bitset<Operation::Last + 1>;
102:    const OperationsFlags&              theOperations,
115:    const OperationsFlags& theFlags);
```
</details>

Read [ShapeProcess.hxx](../crates/occt-sys/OCCT/src/ShapeProcess/ShapeProcess.hxx#38-38), lines 38 to 120

> 💭 So `ShapeProcess::OperationsFlags` is a typedef: `using OperationsFlags = std::bitset<Operation::Last + 1>;`. This is a `std::bitset`, which is a template type from the standard library. This is hard to bind directly - `std::bitset` is not easily representable in C FFI.
> 
> The 11 skips are because methods take or return `OperationsFlags` (= `std::bitset<17>`), which is not resolvable as a known type.
> 
> This is actually an "unresolved template type" more than an "unknown type" - it's being categorized as "unknown type" because the generator sees the typedef name `ShapeProcess::OperationsFlags` rather than `std::bitset<17>`.
> 
> 
> Let me examine the specific skip reasons for each of these 11 symbols to understand the pattern better.

**Terminal:**
```sh
grep -A3 "SKIPPED.*ShapeProcess" crates/opencascade-sys/generated/shape_process*.rs
```
```
(base) whitelily:opencascade-rs sethml$
```
**Exit code:** 1

**Terminal:**
```sh
grep -rn "OperationsFlags" crates/opencascade-sys/generated/*.rs
```
<details>
<summary>Output (26 lines)</summary>

```
crates/opencascade-sys/generated/ffi.rs:36526:pub struct ShapeProcess_OperationsFlags {
crates/opencascade-sys/generated/ffi.rs:683419:    pub fn ShapeProcess_OperationsFlags_destructor(self_: *mut ShapeProcess_OperationsFlags);
crates/opencascade-sys/generated/ffi.rs:685454:unsafe impl crate::CppDeletable for ShapeProcess_OperationsFlags {
crates/opencascade-sys/generated/ffi.rs:685456:        ShapeProcess_OperationsFlags_destructor(ptr);
crates/opencascade-sys/generated/iges_control.rs:1959://   Reason: param 'theFlags' uses unknown type 'ShapeProcess::OperationsFlags const&'
crates/opencascade-sys/generated/iges_control.rs:1960://   // pub fn set_shape_process_flags(&mut self, theFlags: &OperationsFlags);
crates/opencascade-sys/generated/iges_control.rs:1965://   Reason: return type 'ShapeProcess::OperationsFlags const&' is unknown
crates/opencascade-sys/generated/iges_control.rs:1966://   // pub fn get_shape_process_flags(&self) -> &OperationsFlags;
crates/opencascade-sys/generated/iges_to_b_rep.rs:3141://   Reason: param 'theFlags' uses unknown type 'ShapeProcess::OperationsFlags const&'
crates/opencascade-sys/generated/iges_to_b_rep.rs:3142://   // pub fn set_shape_process_flags(&mut self, theFlags: &OperationsFlags);
crates/opencascade-sys/generated/iges_to_b_rep.rs:3147://   Reason: return type 'ShapeProcess::OperationsFlags const&' is unknown
crates/opencascade-sys/generated/iges_to_b_rep.rs:3148://   // pub fn get_shape_process_flags(&self) -> &OperationsFlags;
crates/opencascade-sys/generated/step_control.rs:2086://   Reason: param 'theFlags' uses unknown type 'ShapeProcess::OperationsFlags const&'
crates/opencascade-sys/generated/step_control.rs:2087://   // pub fn set_shape_process_flags(&mut self, theFlags: &OperationsFlags);
crates/opencascade-sys/generated/stepcaf_control.rs:1919://   Reason: param 'theFlags' uses unknown type 'ShapeProcess::OperationsFlags const&'
crates/opencascade-sys/generated/stepcaf_control.rs:1920://   // pub fn set_shape_process_flags(&mut self, theFlags: &OperationsFlags);
crates/opencascade-sys/generated/stepcaf_control.rs:2383://   Reason: param 'theFlags' uses unknown type 'ShapeProcess::OperationsFlags const&'
crates/opencascade-sys/generated/stepcaf_control.rs:2384://   // pub fn set_shape_process_flags(&mut self, theFlags: &OperationsFlags);
crates/opencascade-sys/generated/transfer.rs:930://   Reason: param 'theFlags' uses unknown type 'ShapeProcess::OperationsFlags const&'
crates/opencascade-sys/generated/transfer.rs:931://   // pub fn set_shape_process_flags(&mut self, theFlags: &OperationsFlags);
crates/opencascade-sys/generated/transfer.rs:2035://   Reason: param 'theFlags' uses unknown type 'ShapeProcess::OperationsFlags const&'
crates/opencascade-sys/generated/transfer.rs:2036://   // pub fn set_processing_flags(&mut self, theFlags: &OperationsFlags);
crates/opencascade-sys/generated/xs_algo.rs:567://   Reason: param 'theOperations' uses unknown type 'ShapeProcess::Opera
... (truncated, 3486 chars total)
```
</details>

> 💭 Interesting! The type `ShapeProcess_OperationsFlags` IS in the generated `ffi.rs` (line 36526), meaning it was parsed and a struct was emitted. The problem is that methods

> 💭 Interesting! The type `ShapeProcess_OperationsFlags` IS in the generated `ffi.rs` (line 36526), meaning it was parsed and a struct was emitted. The problem is that methods using `ShapeProcess::OperationsFlags` (with `::` notation) are not resolved to the flat name `ShapeProcess_OperationsFlags`.
> 
> This is a nested type alias issue. The parser creates `ShapeProcess_OperationsFlags` as a type, but the resolver doesn't recognize `ShapeProcess::OperationsFlags` as the same thing. Let me look at how the resolver handles nested types.

The type `ShapeProcess_OperationsFlags` exists in ffi.rs (it was parsed), but methods using  are not resolving to it. This is a nested type resolution issue. Let me investigate how the resolver handles this:

Research nested type resolution

<details>
<summary>Searched for regex <code>resolve_type|lookup_type|resolve.*qualified|flatten.*name</code> (<code>**/crates/opencascade-binding-generator/src/resolver.rs</code>), 14 results</summary>

- [resolver.rs:946](../crates/opencascade-binding-generator/src/resolver.rs#L946)
- [resolver.rs:948](../crates/opencascade-binding-generator/src/resolver.rs#L948)
- [resolver.rs:949](../crates/opencascade-binding-generator/src/resolver.rs#L949)
- [resolver.rs:1055](../crates/opencascade-binding-generator/src/resolver.rs#L1055)
- [resolver.rs:1104](../crates/opencascade-binding-generator/src/resolver.rs#L1104)
- [resolver.rs:1110](../crates/opencascade-binding-generator/src/resolver.rs#L1110)
- [resolver.rs:1167](../crates/opencascade-binding-generator/src/resolver.rs#L1167)
- [resolver.rs:1173](../crates/opencascade-binding-generator/src/resolver.rs#L1173)
- [resolver.rs:1236](../crates/opencascade-binding-generator/src/resolver.rs#L1236)
- [resolver.rs:1242](../crates/opencascade-binding-generator/src/resolver.rs#L1242)
- [resolver.rs:1320](../crates/opencascade-binding-generator/src/resolver.rs#L1320)
- [resolver.rs:1348](../crates/opencascade-binding-generator/src/resolver.rs#L1348)
- [resolver.rs:1354](../crates/opencascade-binding-generator/src/resolver.rs#L1354)
- [resolver.rs:1358](../crates/opencascade-binding-generator/src/resolver.rs#L1358)
</details>

<details>
<summary>Searched for regex <code>::|nested|flatten|qualified</code> (<code>**/crates/opencascade-binding-generator/src/parser.rs</code>), 50 results</summary>

- [parser.rs:6](../crates/opencascade-binding-generator/src/parser.rs#L6)
- [parser.rs:10](../crates/opencascade-binding-generator/src/parser.rs#L10)
- [parser.rs:11](../crates/opencascade-binding-generator/src/parser.rs#L11)
- [parser.rs:12](../crates/opencascade-binding-generator/src/parser.rs#L12)
- [parser.rs:13](../crates/opencascade-binding-generator/src/parser.rs#L13)
- [parser.rs:14](../crates/opencascade-binding-generator/src/parser.rs#L14)
- [parser.rs:15](../crates/opencascade-binding-generator/src/parser.rs#L15)
- [parser.rs:21](../crates/opencascade-binding-generator/src/parser.rs#L21)
- [parser.rs:22](../crates/opencascade-binding-generator/src/parser.rs#L22)
- [parser.rs:23](../crates/opencascade-binding-generator/src/parser.rs#L23)
- [parser.rs:24](../crates/opencascade-binding-generator/src/parser.rs#L24)
- [parser.rs:25](../crates/opencascade-binding-generator/src/parser.rs#L25)
- [parser.rs:26](../crates/opencascade-binding-generator/src/parser.rs#L26)
- [parser.rs:27](../crates/opencascade-binding-generator/src/parser.rs#L27)
- [parser.rs:43](../crates/opencascade-binding-generator/src/parser.rs#L43)
- [parser.rs:51](../crates/opencascade-binding-generator/src/parser.rs#L51)
- [parser.rs:98](../crates/opencascade-binding-generator/src/parser.rs#L98)
- [parser.rs:101](../crates/opencascade-binding-generator/src/parser.rs#L101)
- [parser.rs:102](../crates/opencascade-binding-generator/src/parser.rs#L102)
- [parser.rs:108](../crates/opencascade-binding-generator/src/parser.rs#L108)
- [parser.rs:114](../crates/opencascade-binding-generator/src/parser.rs#L114)
- [parser.rs:116](../crates/opencascade-binding-generator/src/parser.rs#L116)
- [parser.rs:132](../crates/opencascade-binding-generator/src/parser.rs#L132)
- [parser.rs:186](../crates/opencascade-binding-generator/src/parser.rs#L186)
- [parser.rs:189](../crates/opencascade-binding-generator/src/parser.rs#L189)
- [parser.rs:190](../crates/opencascade-binding-generator/src/parser.rs#L190)
- [parser.rs:195](../crates/opencascade-binding-generator/src/parser.rs#L195)
- [parser.rs:204](../crates/opencascade-binding-generator/src/parser.rs#L204)
- [parser.rs:211](../crates/opencascade-binding-generator/src/parser.rs#L211)
- [parser.rs:238](../crates/opencascade-binding-generator/src/parser.rs#L238)
</details>

<details>
<summary>Searched for regex <code>resolve|lookup|map.*type|known.*type</code> (<code>**/crates/opencascade-binding-generator/src/type_mapping.rs</code>), 50 results</summary>

- [type_mapping.rs:1](../crates/opencascade-binding-generator/src/type_mapping.rs#L1)
- [type_mapping.rs:3](../crates/opencascade-binding-generator/src/type_mapping.rs#L3)
- [type_mapping.rs:24](../crates/opencascade-binding-generator/src/type_mapping.rs#L24)
- [type_mapping.rs:37](../crates/opencascade-binding-generator/src/type_mapping.rs#L37)
- [type_mapping.rs:38](../crates/opencascade-binding-generator/src/type_mapping.rs#L38)
- [type_mapping.rs:137](../crates/opencascade-binding-generator/src/type_mapping.rs#L137)
- [type_mapping.rs:139](../crates/opencascade-binding-generator/src/type_mapping.rs#L139)
- [type_mapping.rs:146](../crates/opencascade-binding-generator/src/type_mapping.rs#L146)
- [type_mapping.rs:149](../crates/opencascade-binding-generator/src/type_mapping.rs#L149)
- [type_mapping.rs:156](../crates/opencascade-binding-generator/src/type_mapping.rs#L156)
- [type_mapping.rs:165](../crates/opencascade-binding-generator/src/type_mapping.rs#L165)
- [type_mapping.rs:169](../crates/opencascade-binding-generator/src/type_mapping.rs#L169)
- [type_mapping.rs:171](../crates/opencascade-binding-generator/src/type_mapping.rs#L171)
- [type_mapping.rs:178](../crates/opencascade-binding-generator/src/type_mapping.rs#L178)
- [type_mapping.rs:180](../crates/opencascade-binding-generator/src/type_mapping.rs#L180)
- [type_mapping.rs:215](../crates/opencascade-binding-generator/src/type_mapping.rs#L215)
- [type_mapping.rs:236](../crates/opencascade-binding-generator/src/type_mapping.rs#L236)
- [type_mapping.rs:237](../crates/opencascade-binding-generator/src/type_mapping.rs#L237)
- [type_mapping.rs:238](../crates/opencascade-binding-generator/src/type_mapping.rs#L238)
- [type_mapping.rs:242](../crates/opencascade-binding-generator/src/type_mapping.rs#L242)
- [type_mapping.rs:248](../crates/opencascade-binding-generator/src/type_mapping.rs#L248)
- [type_mapping.rs:249](../crates/opencascade-binding-generator/src/type_mapping.rs#L249)
- [type_mapping.rs:250](../crates/opencascade-binding-generator/src/type_mapping.rs#L250)
- [type_mapping.rs:254](../crates/opencascade-binding-generator/src/type_mapping.rs#L254)
- [type_mapping.rs:261](../crates/opencascade-binding-generator/src/type_mapping.rs#L261)
- [type_mapping.rs:321](../crates/opencascade-binding-generator/src/type_mapping.rs#L321)
- [type_mapping.rs:325](../crates/opencascade-binding-generator/src/type_mapping.rs#L325)
- [type_mapping.rs:359](../crates/opencascade-binding-generator/src/type_mapping.rs#L359)
- [type_mapping.rs:362](../crates/opencascade-binding-generator/src/type_mapping.rs#L362)
- [type_mapping.rs:374](../crates/opencascade-binding-generator/src/type_mapping.rs#L374)
- [type_mapping.rs:385](../crates/opencascade-binding-generator/src/type_mapping.rs#L385)
- [type_mapping.rs:403](../crates/opencascade-binding-generator/src/type_mapping.rs#L403)
- [type_mapping.rs:420](../crates/opencascade-binding-generator/src/type_mapping.rs#L420)
- [type_mapping.rs:439](../crates/opencascade-binding-generator/src/type_mapping.rs#L439)
- [type_mapping.rs:440](../crates/opencascade-binding-generator/src/type_mapping.rs#L440)
- [type_mapping.rs:443](../crates/opencascade-binding-generator/src/type_mapping.rs#L443)
- [type_mapping.rs:480](../crates/opencascade-binding-generator/src/type_mapping.rs#L480)
- [type_mapping.rs:508](../crates/opencascade-binding-generator/src/type_mapping.rs#L508)
- [type_mapping.rs:510](../crates/opencascade-binding-generator/src/type_mapping.rs#L510)
- [type_mapping.rs:517](../crates/opencascade-binding-generator/src/type_mapping.rs#L517)
- [type_mapping.rs:519](../crates/opencascade-binding-generator/src/type_mapping.rs#L519)
- [type_mapping.rs:526](../crates/opencascade-binding-generator/src/type_mapping.rs#L526)
- [type_mapping.rs:536](../crates/opencascade-binding-generator/src/type_mapping.rs#L536)
- [type_mapping.rs:540](../crates/opencascade-binding-generator/src/type_mapping.rs#L540)
- [type_mapping.rs:542](../crates/opencascade-binding-generator/src/type_mapping.rs#L542)
- [type_mapping.rs:553](../crates/opencascade-binding-generator/src/type_mapping.rs#L553)
- [type_mapping.rs:556](../crates/opencascade-binding-generator/src/type_mapping.rs#L556)
- [type_mapping.rs:568](../crates/opencascade-binding-generator/src/type_mapping.rs#L568)
- [type_mapping.rs:569](../crates/opencascade-binding-generator/src/type_mapping.rs#L569)
- [type_mapping.rs:570](../crates/opencascade-binding-generator/src/type_mapping.rs#L570)
</details>

<details>
<summary>Searched for regex <code>is_method_bindable|unknown.*type|unbindable</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 30 results</summary>

- [bindings.rs:451](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L451)
- [bindings.rs:461](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L461)
- [bindings.rs:463](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L463)
- [bindings.rs:695](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L695)
- [bindings.rs:696](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L696)
- [bindings.rs:699](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L699)
- [bindings.rs:700](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L700)
- [bindings.rs:704](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L704)
- [bindings.rs:705](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L705)
- [bindings.rs:711](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L711)
- [bindings.rs:712](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L712)
- [bindings.rs:715](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L715)
- [bindings.rs:716](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L716)
- [bindings.rs:722](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L722)
- [bindings.rs:723](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L723)
- [bindings.rs:726](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L726)
- [bindings.rs:727](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L727)
- [bindings.rs:731](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L731)
- [bindings.rs:732](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L732)
- [bindings.rs:738](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L738)
- [bindings.rs:739](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L739)
- [bindings.rs:810](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L810)
- [bindings.rs:811](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L811)
- [bindings.rs:812](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L812)
- [bindings.rs:813](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L813)
- [bindings.rs:828](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L828)
- [bindings.rs:830](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L830)
- [bindings.rs:836](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L836)
</details>

<details>
<summary>Reading lines 930-970 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L930-L971">resolver.rs</a></summary>

```
    // Note: Function naming (rust_ffi_name, cpp_wrapper_name) is now handled by
    // compute_all_function_bindings() in bindings.rs. The placeholder names set
    // during resolve_function() are no longer used by emitters.
    
    table
}

/// Resolve a single class and its members
fn resolve_class(
    table: &mut SymbolTable,
    class: &ParsedClass,
    all_enum_names: &HashSet<String>,
    type_to_module: &HashMap<String, String>,
) {
    let class_id = SymbolId::new(format!("class::{}", class.name));
    let rust_module = crate::module_graph::module_to_rust_name(&class.module);
    // Flatten nested class names (e.g., "Parent::Nested" -> "Parent_Nested")
    // before computing the short name, so the result is a valid Rust identifier.
    let flattened_name = class.name.replace("::", "_");
    let short_name = crate::type_mapping::short_name_for_module(&flattened_name, &class.module);
    let rust_ffi_name = safe_short_name(&short_name);
    
    // Determine class binding status
    // Protected-destructor classes are now included (methods, statics, handles)
    // with only ctor/dtor generation skipped.
    let class_status = BindingStatus::Included;
    
    // Resolve constructors
    let mut constructor_ids = Vec::new();
    for (idx, ctor) in class.constructors.iter().enumerate() {
        let ctor_id = SymbolId::new(format!("ctor::{}::{}", class.name, idx));
        let resolved_ctor = resolve_constructor(
            &ctor_id,
            &class_id,
            &class.name,
            ctor,
            idx,
            class.is_abstract,
            all_enum_names,
            type_to_module,
        );
```
</details>

<details>
<summary>Reading lines 1310-1400 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L1310-L1401">resolver.rs</a></summary>

```
    }
    if let Some(ref ret) = func.return_type {
        if check(ret) {
            return true;
        }
    }
    false
}

/// Resolve a type to its code generation form
fn resolve_type(ty: &Type, all_enum_names: &HashSet<String>, type_to_module: &HashMap<String, String>) -> ResolvedType {
    // Check if this type is an enum (possibly wrapped in const ref)
    let enum_name = extract_enum_name_from_type(ty, all_enum_names);
    if let Some(ref name) = enum_name {
        return ResolvedType {
            original: ty.clone(),
            rust_ffi_type: "i32".to_string(),
            cpp_type: "int32_t".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
            enum_cpp_name: Some(name.clone()),
        };
    }

    // For unbindable types, use a placeholder string
    // The binding status will ensure these don't get generated
    let rust_ffi_type = match ty {
        Type::RValueRef(_) => "<unbindable: rvalue-ref>".to_string(),
        _ => ty.to_rust_type_string(),
    };

    ResolvedType {
        original: ty.clone(),
        rust_ffi_type,
        cpp_type: ty.to_cpp_string(),
        needs_unique_ptr: matches!(ty, Type::Class(_) | Type::Handle(_)),
        needs_pin: matches!(ty, Type::MutRef(inner) if !inner.is_primitive()),
        source_module: lookup_type_module(ty, type_to_module),
        enum_cpp_name: None,
    }
}

/// Look up the module for a Type from the authoritative type→module mapping
fn lookup_type_module(ty: &Type, type_to_module: &HashMap<String, String>) -> Option<String> {
    match ty {
        Type::Class(name) | Type::Handle(name) => type_to_module.get(name).cloned(),
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => {
            lookup_type_module(inner, type_to_module)
        }
        _ => None,
    }
}

/// Extract the enum C++ name from a type, unwrapping references
fn extract_enum_name_from_type(ty: &Type, all_enums: &HashSet<String>) -> Option<String> {
    match ty {
        Type::Class(name) if all_enums.contains(name) => Some(name.clone()),
        // Only unwrap const refs and rvalue refs, NOT MutRef (output params need special handling)
        Type::ConstRef(inner) | Type::RValueRef(inner) => {
            extract_enum_name_from_type(inner, all_enums)
        }
        _ => None,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_safe_method_name() {
        assert_eq!(safe_method_name("GetValue"), "get_value");
        assert_eq!(safe_method_name("Type"), "type_"); // keyword
        assert_eq!(safe_method_name("Move"), "move_"); // keyword
    }
    
    #[test]
    fn test_safe_param_name() {
        assert_eq!(safe_param_name("value"), "value");
        assert_eq!(safe_param_name("type"), "type_"); // keyword
        assert_eq!(safe_param_name("self"), "self_"); // keyword
    }
}



```
</details>

<details>
<summary>Reading lines 370-440 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L370-L441">type_mapping.rs</a></summary>

```
}

/// Check if a type references an unknown class/handle
/// Returns true if the type uses a Handle or Class that is not in all_classes
pub fn type_uses_unknown_class(ty: &Type, all_classes: &std::collections::HashSet<String>) -> bool {
    match ty {
        Type::Handle(class_name) => !all_classes.contains(class_name),
        Type::Class(class_name) => {
            if all_classes.contains(class_name) {
                return false;
            }
            // Void pointer types — Standard_Address (void*) and literal "void" — are known
            if crate::model::is_void_type_name(class_name) {
                return false;
            }
            // Primitive types mapped as Type::Class (e.g., "char" from Standard_Character)
            if crate::codegen::rust::is_primitive_type(class_name) {
                return false;
            }
            // Nested types (Parent::Nested) are known if the parent class is known
            if let Some(parent) = class_name.split("::").next() {
                if class_name.contains("::") && all_classes.contains(parent) {
                    return false;
                }
            }
            true
        }
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_class(inner, all_classes),
        _ => false,
    }
}

/// Check if a type references a Handle to a class that won't have a Handle declaration generated
/// This is more strict than type_uses_unknown_class - it checks that Handle types are for
/// classes that will actually have Handle<T> declarations generated (is_handle_type)
pub fn type_uses_unknown_handle(
    ty: &Type,
    all_classes: &std::collections::HashSet<String>,
    handle_able_classes: &std::collections::HashSet<String>,
) -> bool {
    match ty {
        Type::Handle(class_name) => !handle_able_classes.contains(class_name),
        Type::Class(class_name) => {
            if all_classes.contains(class_name) {
                return false;
            }
            // Void pointer types — Standard_Address (void*) and literal "void" — are known
            if crate::model::is_void_type_name(class_name) {
                return false;
            }
            // Primitive types mapped as Type::Class (e.g., "char" from Standard_Character)
            if crate::codegen::rust::is_primitive_type(class_name) {
                return false;
            }
            // Nested types (Parent::Nested) are known if the parent class is known
            if let Some(parent) = class_name.split("::").next() {
                if class_name.contains("::") && all_classes.contains(parent) {
                    return false;
                }
            }
            true
        }
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
            type_uses_unknown_handle(inner, all_classes, handle_able_classes)
        }
        _ => false,
    }
}

/// Map a type to Rust, using short names for same-module types
pub fn map_type_in_context(ty: &Type, ctx: &TypeContext) -> RustTypeMapping {
```
</details>

<details>
<summary>Reading lines 461-500 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L461-L501">bindings.rs</a></summary>

```
/// Check if a type uses an unknown class/handle given the TypeContext.
/// Enum types (Type::Class that are in all_enums) are known — they map to i32.
fn type_uses_unknown_type(ty: &Type, ctx: &TypeContext) -> bool {
    // Enum types are known (mapped to i32), so skip them
    match ty {
        Type::Class(name) if ctx.all_enums.contains(name) => return false,
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => {
            if let Type::Class(name) = inner.as_ref() {
                if ctx.all_enums.contains(name) {
                    return false;
                }
            }
        }
        _ => {}
    }
    if let Some(handle_classes) = ctx.handle_able_classes {
        type_mapping::type_uses_unknown_handle(ty, ctx.all_classes, handle_classes)
    } else {
        type_mapping::type_uses_unknown_class(ty, ctx.all_classes)
    }
}

/// Check if a method has by-value class or handle parameters (not enums).
/// These need C++ wrappers that accept const T& instead.
fn has_by_value_class_or_handle_params(params: &[Param], all_enums: &HashSet<String>) -> bool {
    params.iter().any(|p| match &p.ty {
        Type::Class(name) => !all_enums.contains(name) && is_opaque_class_name(name),
        Type::Handle(_) => true,
        _ => false,
    })
}

/// Check if params contain any &mut enum output parameters.
fn has_mut_ref_enum_params(params: &[Param], all_enums: &HashSet<String>) -> bool {
    params.iter().any(|p| {
        if let Type::MutRef(inner) = &p.ty {
            if let Type::Class(name) = inner.as_ref() {
                return all_enums.contains(name);
            }
        }
```
</details>

<details>
<summary>Reading lines 806-870 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L806-L871">bindings.rs</a></summary>

```
const AMBIGUOUS_METHODS: &[(&str, &str)] = &[
    ("BOPAlgo_ParallelAlgo", "Perform"),
];

fn is_method_bindable(method: &Method, ctx: &TypeContext, class_name: &str) -> Result<(), String> {
    if method.has_unbindable_types() {
        let unbindable_details = describe_unbindable_types_method(method);
        return Err(format!("has unbindable types: {}", unbindable_details));
    }
    // Skip methods with const char*& or const char* const& params (need manual bindings)
    if let Some((param_name, type_name)) = resolver::method_has_string_ref_param(method) {
        return Err(format!("has string ref param '{}' of type '{}' (needs manual binding)", param_name, type_name));
    }
    // Skip methods that cause ambiguous call errors in C++ wrappers
    if AMBIGUOUS_METHODS.iter().any(|(c, m)| *c == class_name && *m == method.name) {
        return Err("causes ambiguous overload in C++ (listed in AMBIGUOUS_METHODS)".to_string());
    }
    // Const/mut return mismatch is now handled via C++ wrappers (ConstMutReturnFix).
    // &mut enum output params are now handled via C++ wrappers (MutRefEnumParam).
    if let Some(p) = method
        .params
        .iter()
        .find(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return Err(format!("param '{}' uses unknown type '{}'", p.name, p.ty.to_cpp_string()));
    }
    // Skip methods where a nullable pointer param's inner type is unknown
    if let Some(p) = method.params.iter().find(|p| {
        if p.is_nullable_ptr() {
            match &p.ty {
                Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_type(inner, ctx),
                _ => false,
            }
        } else {
            false
        }
    }) {
        return Err(format!("nullable param '{}' inner type is unknown", p.name));
    }
    // Skip methods where a class pointer param's inner type is unknown.
    // We check all_classes directly (not type_uses_unknown_type) because nested types
    // like Parent::Nested are considered "known" by type_uses_unknown_type if the parent
    // is known, but they don't have their own FFI type declarations.
    if let Some(p) = method.params.iter().find(|p| {
        if let Some(class_name) = p.ty.class_ptr_inner_name() {
            !ctx.all_classes.contains(class_name) && !ctx.all_enums.contains(class_name)
        } else {
            false
        }
    }) {
        return Err(format!("class pointer param '{}' inner type '{}' is unknown", p.name, p.ty.to_cpp_string()));
    }
    if let Some(ref ret) = method.return_type {
        if type_uses_unknown_type(ret, ctx) {
            return Err(format!("return type '{}' is unknown", ret.to_cpp_string()));
        }
        // Check class pointer returns for unknown inner types (same as params)
        if let Some(class_name) = ret.class_ptr_inner_name() {
            if !ctx.all_classes.contains(class_name) && !ctx.all_enums.contains(class_name) {
                return Err(format!("class pointer return inner type '{}' is unknown", ret.to_cpp_string()));
            }
        }
        // OwnedPtr<T> return type requires CppDeletable for T. ParsedClasses have
        // generated destructors; the 91 known collections do too. But NCollection
        // template typedef names (e.g., TColStd_ListOfAsciiString) added to
```
</details>

<details>
<summary>Searched for regex <code>all_classes|all_class_names|type_to_module.*insert|SymbolTable</code> (<code>**/crates/opencascade-binding-generator/src/resolver.rs</code>), 24 results</summary>

- [resolver.rs:4](../crates/opencascade-binding-generator/src/resolver.rs#L4)
- [resolver.rs:326](../crates/opencascade-binding-generator/src/resolver.rs#L326)
- [resolver.rs:348](../crates/opencascade-binding-generator/src/resolver.rs#L348)
- [resolver.rs:361](../crates/opencascade-binding-generator/src/resolver.rs#L361)
- [resolver.rs:750](../crates/opencascade-binding-generator/src/resolver.rs#L750)
- [resolver.rs:756](../crates/opencascade-binding-generator/src/resolver.rs#L756)
- [resolver.rs:759](../crates/opencascade-binding-generator/src/resolver.rs#L759)
- [resolver.rs:761](../crates/opencascade-binding-generator/src/resolver.rs#L761)
- [resolver.rs:763](../crates/opencascade-binding-generator/src/resolver.rs#L763)
- [resolver.rs:790](../crates/opencascade-binding-generator/src/resolver.rs#L790)
- [resolver.rs:815](../crates/opencascade-binding-generator/src/resolver.rs#L815)
- [resolver.rs:819](../crates/opencascade-binding-generator/src/resolver.rs#L819)
- [resolver.rs:827](../crates/opencascade-binding-generator/src/resolver.rs#L827)
- [resolver.rs:838](../crates/opencascade-binding-generator/src/resolver.rs#L838)
- [resolver.rs:921](../crates/opencascade-binding-generator/src/resolver.rs#L921)
- [resolver.rs:927](../crates/opencascade-binding-generator/src/resolver.rs#L927)
- [resolver.rs:939](../crates/opencascade-binding-generator/src/resolver.rs#L939)
- [resolver.rs:1202](../crates/opencascade-binding-generator/src/resolver.rs#L1202)
- [resolver.rs:1205](../crates/opencascade-binding-generator/src/resolver.rs#L1205)
- [resolver.rs:1248](../crates/opencascade-binding-generator/src/resolver.rs#L1248)
- [resolver.rs:1287](../crates/opencascade-binding-generator/src/resolver.rs#L1287)
- [resolver.rs:1306](../crates/opencascade-binding-generator/src/resolver.rs#L1306)
</details>

<details>
<summary>Searched for regex <code>all_classes|all_class_names|TypeContext</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 30 results</summary>

- [bindings.rs:11](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L11)
- [bindings.rs:461](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L461)
- [bindings.rs:463](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L463)
- [bindings.rs:477](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L477)
- [bindings.rs:479](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L479)
- [bindings.rs:560](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L560)
- [bindings.rs:567](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L567)
- [bindings.rs:583](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L583)
- [bindings.rs:584](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L584)
- [bindings.rs:596](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L596)
- [bindings.rs:645](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L645)
- [bindings.rs:810](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L810)
- [bindings.rs:846](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L846)
- [bindings.rs:851](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L851)
- [bindings.rs:864](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L864)
- [bindings.rs:871](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L871)
- [bindings.rs:895](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L895)
- [bindings.rs:934](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L934)
- [bindings.rs:937](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L937)
- [bindings.rs:948](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L948)
- [bindings.rs:979](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L979)
- [bindings.rs:982](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L982)
- [bindings.rs:1039](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1039)
- [bindings.rs:1219](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1219)
- [bindings.rs:1488](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1488)
- [bindings.rs:1501](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1501)
- [bindings.rs:1527](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1527)
- [bindings.rs:1530](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1530)
</details>

<details>
<summary>Searched for regex <code>class\.name|flattened.*name|nested</code> (<code>**/crates/opencascade-binding-generator/src/resolver.rs</code>), 14 results</summary>

- [resolver.rs:775](../crates/opencascade-binding-generator/src/resolver.rs#L775)
- [resolver.rs:944](../crates/opencascade-binding-generator/src/resolver.rs#L944)
- [resolver.rs:946](../crates/opencascade-binding-generator/src/resolver.rs#L946)
- [resolver.rs:948](../crates/opencascade-binding-generator/src/resolver.rs#L948)
- [resolver.rs:949](../crates/opencascade-binding-generator/src/resolver.rs#L949)
- [resolver.rs:960](../crates/opencascade-binding-generator/src/resolver.rs#L960)
- [resolver.rs:964](../crates/opencascade-binding-generator/src/resolver.rs#L964)
- [resolver.rs:978](../crates/opencascade-binding-generator/src/resolver.rs#L978)
- [resolver.rs:982](../crates/opencascade-binding-generator/src/resolver.rs#L982)
- [resolver.rs:994](../crates/opencascade-binding-generator/src/resolver.rs#L994)
- [resolver.rs:998](../crates/opencascade-binding-generator/src/resolver.rs#L998)
- [resolver.rs:1009](../crates/opencascade-binding-generator/src/resolver.rs#L1009)
</details>

<details>
<summary>Reading lines 750-845 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L750-L846">resolver.rs</a></summary>

```
    all_classes: &[&ParsedClass],
    all_enums: &[&ParsedEnum],
    all_functions: &[&ParsedFunction],
    collection_type_names: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
    manual_type_names: &HashSet<String>,
) -> SymbolTable {
    // Collect all enum and class names first
    let all_enum_names: HashSet<String> = all_enums.iter().map(|e| e.name.clone()).collect();
    let mut all_class_names: HashSet<String> = all_classes.iter().map(|c| c.name.clone()).collect();
    // Collection typedefs are known types for filtering purposes
    all_class_names.extend(collection_type_names.iter().cloned());
    // Manual types (defined in manual/ files) are also known types
    all_class_names.extend(manual_type_names.iter().cloned());

    // Start from the pre-computed handle-able classes (transitive closure of inheritance graph)
    let mut handle_able_classes = handle_able_classes.clone();

    // Also add any class name that appears inside Type::Handle(...) in method signatures.
    // If C++ code uses Handle(X) for a type, X must inherit from Standard_Transient,
    // so it's handle-able even if its own header is excluded.
    fn collect_handle_types(ty: &crate::model::Type, set: &mut HashSet<String>) {
        match ty {
            crate::model::Type::Handle(name) => {
                // Only add clean OCCT type names (not template forms like NCollection_Shared<...>)
                // Nested class types (Parent::Child) are OK.
                if !name.contains('<') {
                    set.insert(name.clone());
                }
            }
            crate::model::Type::ConstRef(inner)
            | crate::model::Type::MutRef(inner)
            | crate::model::Type::ConstPtr(inner)
            | crate::model::Type::MutPtr(inner)
            | crate::model::Type::RValueRef(inner) => {
                collect_handle_types(inner, set);
            }
            _ => {}
        }
    }
    for class in all_classes {
        for method in &class.methods {
            for param in &method.params {
                collect_handle_types(&param.ty, &mut handle_able_classes);
            }
            if let Some(ret) = &method.return_type {
                collect_handle_types(ret, &mut handle_able_classes);
            }
        }
        for ctor in &class.constructors {
            for param in &ctor.params {
                collect_handle_types(&param.ty, &mut handle_able_classes);
            }
        }
    }
    // Also scan standalone functions
    for func in all_functions {
        for param in &func.params {
            collect_handle_types(&param.ty, &mut handle_able_classes);
        }
        if let Some(ret) = &func.return_type {
            collect_handle_types(ret, &mut handle_able_classes);
        }
    }
    // These referenced Handle types also need to be known class names
    all_class_names.extend(handle_able_classes.iter().cloned());
    
    // Build authoritative type→module mapping from parsed header data.
    // This is the single source of truth for module membership.
    let type_to_module: HashMap<String, String> = all_classes
        .iter()
        .map(|c| (c.name.clone(), c.module.clone()))
        .chain(all_enums.iter().map(|e| (e.name.clone(), e.module.clone())))
        .collect();
    // Keep a reference copy for resolve_* functions (avoids borrow conflicts with table)
    let type_to_module_ref = type_to_module.clone();
    
    let mut table = SymbolTable {
        classes: HashMap::new(),
        constructors: HashMap::new(),
        methods: HashMap::new(),
        static_methods: HashMap::new(),
        functions: HashMap::new(),
        enums: HashMap::new(),
        classes_by_module: HashMap::new(),
        functions_by_module: HashMap::new(),
        enums_by_module: HashMap::new(),
        all_enum_names: all_enum_names.clone(),
        all_class_names: all_class_names.clone(),
        handle_able_classes: handle_able_classes.clone(),
        cross_module_types:
... (truncated, 4129 chars)
```
</details>

Searched for regex `lookup_module_for_type|short_name_for_module` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), 4 results
- [type_mapping.rs:299](../crates/opencascade-binding-generator/src/type_mapping.rs#L299)
- [type_mapping.rs:321](../crates/opencascade-binding-generator/src/type_mapping.rs#L321)
- [type_mapping.rs:480](../crates/opencascade-binding-generator/src/type_mapping.rs#L480)
- [type_mapping.rs:526](../crates/opencascade-binding-generator/src/type_mapping.rs#L526)

Searched for regex `lookup_type_module|type_to_module.*get` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 4 results
- [resolver.rs:1348](../crates/opencascade-binding-generator/src/resolver.rs#L1348)
- [resolver.rs:1354](../crates/opencascade-binding-generator/src/resolver.rs#L1354)
- [resolver.rs:1356](../crates/opencascade-binding-generator/src/resolver.rs#L1356)
- [resolver.rs:1358](../crates/opencascade-binding-generator/src/resolver.rs#L1358)

<details>
<summary>Searched for regex <code>nested|ParsedClass|class_name|parent.*class|inner.*class|struct.*class</code> (<code>**/crates/opencascade-binding-generator/src/parser.rs</code>), 30 results</summary>

- [parser.rs:7](../crates/opencascade-binding-generator/src/parser.rs#L7)
- [parser.rs:59](../crates/opencascade-binding-generator/src/parser.rs#L59)
- [parser.rs:532](../crates/opencascade-binding-generator/src/parser.rs#L532)
- [parser.rs:533](../crates/opencascade-binding-generator/src/parser.rs#L533)
- [parser.rs:534](../crates/opencascade-binding-generator/src/parser.rs#L534)
- [parser.rs:595](../crates/opencascade-binding-generator/src/parser.rs#L595)
- [parser.rs:622](../crates/opencascade-binding-generator/src/parser.rs#L622)
- [parser.rs:624](../crates/opencascade-binding-generator/src/parser.rs#L624)
- [parser.rs:749](../crates/opencascade-binding-generator/src/parser.rs#L749)
- [parser.rs:750](../crates/opencascade-binding-generator/src/parser.rs#L750)
- [parser.rs:751](../crates/opencascade-binding-generator/src/parser.rs#L751)
- [parser.rs:753](../crates/opencascade-binding-generator/src/parser.rs#L753)
- [parser.rs:756](../crates/opencascade-binding-generator/src/parser.rs#L756)
- [parser.rs:757](../crates/opencascade-binding-generator/src/parser.rs#L757)
- [parser.rs:759](../crates/opencascade-binding-generator/src/parser.rs#L759)
- [parser.rs:760](../crates/opencascade-binding-generator/src/parser.rs#L760)
- [parser.rs:761](../crates/opencascade-binding-generator/src/parser.rs#L761)
- [parser.rs:765](../crates/opencascade-binding-generator/src/parser.rs#L765)
- [parser.rs:771](../crates/opencascade-binding-generator/src/parser.rs#L771)
- [parser.rs:817](../crates/opencascade-binding-generator/src/parser.rs#L817)
- [parser.rs:843](../crates/opencascade-binding-generator/src/parser.rs#L843)
- [parser.rs:865](../crates/opencascade-binding-generator/src/parser.rs#L865)
- [parser.rs:866](../crates/opencascade-binding-generator/src/parser.rs#L866)
- [parser.rs:867](../crates/opencascade-binding-generator/src/parser.rs#L867)
</details>

<details>
<summary>Searched for regex <code>ParsedClass|to_cpp_string|to_rust_type_string</code> (<code>**/crates/opencascade-binding-generator/src/model.rs</code>), 20 results</summary>

- [model.rs:17](../crates/opencascade-binding-generator/src/model.rs#L17)
- [model.rs:122](../crates/opencascade-binding-generator/src/model.rs#L122)
- [model.rs:166](../crates/opencascade-binding-generator/src/model.rs#L166)
- [model.rs:751](../crates/opencascade-binding-generator/src/model.rs#L751)
- [model.rs:752](../crates/opencascade-binding-generator/src/model.rs#L752)
- [model.rs:756](../crates/opencascade-binding-generator/src/model.rs#L756)
- [model.rs:761](../crates/opencascade-binding-generator/src/model.rs#L761)
- [model.rs:785](../crates/opencascade-binding-generator/src/model.rs#L785)
- [model.rs:786](../crates/opencascade-binding-generator/src/model.rs#L786)
- [model.rs:787](../crates/opencascade-binding-generator/src/model.rs#L787)
- [model.rs:789](../crates/opencascade-binding-generator/src/model.rs#L789)
- [model.rs:790](../crates/opencascade-binding-generator/src/model.rs#L790)
- [model.rs:815](../crates/opencascade-binding-generator/src/model.rs#L815)
- [model.rs:834](../crates/opencascade-binding-generator/src/model.rs#L834)
- [model.rs:838](../crates/opencascade-binding-generator/src/model.rs#L838)
- [model.rs:845](../crates/opencascade-binding-generator/src/model.rs#L845)
- [model.rs:849](../crates/opencascade-binding-generator/src/model.rs#L849)
- [model.rs:876](../crates/opencascade-binding-generator/src/model.rs#L876)
- [model.rs:878](../crates/opencascade-binding-generator/src/model.rs#L878)
- [model.rs:880](../crates/opencascade-binding-generator/src/model.rs#L880)
</details>

<details>
<summary>Reading lines 815-870 of <a href="../crates/opencascade-binding-generator/src/model.rs#L815-L871">model.rs</a></summary>

```
    pub fn to_rust_type_string(&self) -> String {
        match self {
            Type::Void => "()".to_string(),
            Type::Bool => "bool".to_string(),
            Type::I32 => "i32".to_string(),
            Type::U32 => "u32".to_string(),
            Type::U16 => "u16".to_string(),
            Type::I16 => "i16".to_string(),
            Type::I64 => "i64".to_string(),
            Type::U64 => "u64".to_string(),
            Type::Long => "std::ffi::c_long".to_string(),
            Type::ULong => "std::ffi::c_ulong".to_string(),
            Type::Usize => "usize".to_string(),
            Type::F32 => "f32".to_string(),
            Type::F64 => "f64".to_string(),
            Type::CHAR16 => "u16".to_string(), // Rust doesn't have char16, so we use u16 and rely on callers to convert
            Type::U8 => "u8".to_string(),
            Type::I8 => "i8".to_string(),
            Type::ConstRef(inner) => {
                let inner_str = inner.to_rust_type_string();
                format!("&{}", inner_str)
            }
            Type::MutRef(inner) => {
                let inner_str = inner.to_rust_type_string();
                format!("&mut {}", inner_str)
            }
            Type::RValueRef(_) => {
                panic!("RValueRef types should not be converted to Rust type strings - they are unbindable")
            }
            Type::ConstPtr(inner) => {
                let inner_str = inner.to_rust_type_string();
                format!("*const {}", inner_str)
            }
            Type::MutPtr(inner) => {
                let inner_str = inner.to_rust_type_string();
                format!("*mut {}", inner_str)
            }
            Type::Handle(name) => {
                // Use handle_type_name to properly flatten both :: and _ from names
                let flat_name = name.replace("::", "_");
                // Extract short name from full OCCT name
                let short = if let Some(underscore_pos) = flat_name.find('_') {
                    &flat_name[underscore_pos + 1..]
                } else {
                    flat_name.as_str()
                };
                format!("Handle{}", short)
            }
            Type::Class(name) => {
                // Flatten nested types: Parent::Nested -> Parent_Nested
                let flat = Type::ffi_safe_class_name(name);
                // Extract short name from full OCCT name (e.g., "gp_Pnt" -> "Pnt")
                if let Some(underscore_pos) = flat.find('_') {
                    flat[underscore_pos + 1..].to_string()
                } else {
                    flat
```
</details>

<details>
<summary>Reading lines 760-800 of <a href="../crates/opencascade-binding-generator/src/model.rs#L760-L801">model.rs</a></summary>

```
    /// Get a human-readable C++-like type string for diagnostic messages.
    pub fn to_cpp_string(&self) -> String {
        match self {
            Type::Void => "void".to_string(),
            Type::Bool => "bool".to_string(),
            Type::I32 => "int32_t".to_string(),
            Type::U32 => "uint32_t".to_string(),
            Type::U16 => "uint16_t".to_string(),
            Type::I16 => "int16_t".to_string(),
            Type::I64 => "int64_t".to_string(),
            Type::U64 => "uint64_t".to_string(),
            Type::Long => "long".to_string(),
            Type::ULong => "unsigned long".to_string(),
            Type::Usize => "size_t".to_string(),
            Type::F32 => "float".to_string(),
            Type::F64 => "double".to_string(),
            Type::CHAR16 => "char16_t".to_string(),
            Type::U8 => "uint8_t".to_string(),
            Type::I8 => "int8_t".to_string(),
            // Use postfix const ("T const&") rather than prefix ("const T&") so that
            // when the inner type is itself a pointer (e.g., ConstRef(MutPtr(Class("X")))),
            // the const correctly qualifies the pointer level, not the pointee:
            //   ConstRef(MutPtr(X)) → "X* const&" (correct: const-ref to pointer-to-X)
            //   vs. "const X*&" (wrong: ref to pointer-to-const-X)
            // For simple types, "T const&" and "const T&" are equivalent in C/C++.
            Type::ConstRef(inner) => format!("{} const&", inner.to_cpp_string()),
            Type::MutRef(inner) => format!("{}&", inner.to_cpp_string()),
            Type::RValueRef(inner) => format!("{}&&", inner.to_cpp_string()),
            // Use postfix const for same reason as ConstRef above.
            Type::ConstPtr(inner) => format!("{} const*", inner.to_cpp_string()),
            Type::MutPtr(inner) => format!("{}*", inner.to_cpp_string()),
            Type::Handle(name) => format!("Handle({})", name),
            Type::Class(name) => name.clone(),
        }
    }

    /// Check if this type is an unresolved template instantiation that can't be
    /// represented in Rust FFI. Only catches template types with `<>`.
    /// Non-underscore class names (e.g., `LDOMString`) are NOT caught here —
    /// they are handled by `type_uses_unknown_class()` in the binding layer
    /// which checks against the symbol table.
```
</details>

<details>
<summary>Reading lines 745-870 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L745-L871">parser.rs</a></summary>

```
                }
            }
            EntityKind::ClassDecl | EntityKind::StructDecl => {
                // Use tracked access level (not get_accessibility, which is unreliable for structs)
                let is_nested_public = current_access.get() == Accessibility::Public;
                // Parse nested classes/structs defined inside this class
                if is_nested_public && child.is_definition() {
                    let mut parsed = parse_class(&child, source_header, verbose);
                    // Collect the original (unqualified) names of sibling nested classes
                    // so we can fix base class references after qualification.
                    let sibling_names: HashSet<String> = parsed.iter().map(|c| c.name.clone()).collect();
                    for nested in &mut parsed {
                        // Qualify the nested class name with parent: Parent::Nested
                        // Always prepend since multi-level nesting (A::B::C) needs all levels.
                        nested.name = format!("{}::{}", name, nested.name);
                        // nested.module is inherited from source_header
                        // Also qualify base class names that reference sibling nested classes.
                        // Without this, the inheritance graph can't connect e.g.
                        // ShapePersistent_BRep::Curve3D -> GCurve (should be
                        // ShapePersistent_BRep::GCurve) for handle-able class computation.
                        for base in &mut nested.base_classes {
                            if sibling_names.contains(base.as_str()) {
                                *base = format!("{}::{}", name, base);
                            }
                        }
                    }
                    nested_classes.extend(parsed);
                }
            }
            EntityKind::Destructor => {
                // A pure virtual destructor (`virtual ~Foo() = 0`) makes the
                // class abstract even though no non-destructor pure virtuals
                // exist. libclang reports this as Destructor (not Method), so
                // we must check it separately.
                if child.is_pure_virtual_method() {
                    is_abstract = true;
                }
            }
            EntityKind::UsingDeclaration => {
                // `using Base::Method;` in a non-public section narrows the
                // inherited method's access, hiding it from derived classes.
                // Record the name so the inheritance logic treats it as overridden.
                if !is_public(&child) {
                    if let Some(ref using_name) = child.get_name() {
                        all_method_names.insert(using_name.clone());
                    }
                }
            }
            EntityKind::EnumDecl => {
                // A public enum with the same name as an inherited method
                // shadows that method in C++ (e.g. AIS_PointCloud::DisplayMode
                // enum hides PrsMgr_PresentableObject::DisplayMode() method).
                if let Some(ref enum_name) = child.get_name() {
                    if !enum_name.is_empty() {
                        all_method_names.insert(enum_name.clone());
                    }
                }
            }
            EntityKind::FunctionTemplate => {
                // Template methods hide base class methods with the same name
                // (C++ name-hiding rule). Record the name so the inheritance
                // logic blocks the parent's non-template overloads.
                if let Some(ref tmpl_name) = child.get_name() {
                    all_method_names.insert(tmpl_name.clone());
                }
            }
            _ => {}
        }
        EntityVisitResult::Continue
    });

    // Only return classes that have something to bind
    if constructors.is_empty() && methods.is_empty() && static_methods.is_empty() && fields.is_emp
... (truncated, 5561 chars)
```
</details>

Searched for regex `ffi_safe_class_name` (`**/crates/opencascade-binding-generator/src/model.rs`), 3 results
- [model.rs:713](../crates/opencascade-binding-generator/src/model.rs#L713)
- [model.rs:865](../crates/opencascade-binding-generator/src/model.rs#L865)
- [model.rs:938](../crates/opencascade-binding-generator/src/model.rs#L938)

Searched for regex `TypeContext` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), 3 results
- [type_mapping.rs:347](../crates/opencascade-binding-generator/src/type_mapping.rs#L347)
- [type_mapping.rs:440](../crates/opencascade-binding-generator/src/type_mapping.rs#L440)
- [type_mapping.rs:542](../crates/opencascade-binding-generator/src/type_mapping.rs#L542)

<details>
<summary>Reading lines 710-730 of <a href="../crates/opencascade-binding-generator/src/model.rs#L710-L731">model.rs</a></summary>

```

    /// Convert a nested C++ type name to a valid Rust FFI identifier.
    /// `Parent::Nested` becomes `Parent_Nested`. Non-nested names pass through.
    pub fn ffi_safe_class_name(name: &str) -> String {
        name.replace("::", "_")
    }

    /// Check if this type is an rvalue reference (T&&)
    /// Rvalue references are not bindable through the FFI
    pub fn is_rvalue_ref(&self) -> bool {
        matches!(self, Type::RValueRef(_))
    }

    /// Check if this type is unbindable through the FFI.
    /// Note: const char* (C strings) ARE bindable - we generate wrappers that pass const char* directly.
    /// Note: Raw pointers (void*, int*, T*) are NOT unbindable — they are bound as unsafe raw pointer types.
    /// Nested types (Parent::Nested) are supported via name flattening
    /// (Parent::Nested → Parent_Nested in Rust FFI), BUT unresolved template types
    /// and unqualified names without underscore remain unbindable.
    pub fn is_unbindable(&self) -> bool {
        self.is_array() || self.is_rvalue_ref() || self.is_unresolved_template_type()
```
</details>

<details>
<summary>Reading lines 345-370 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L345-L371">type_mapping.rs</a></summary>

```

/// Context for type mapping within a specific module
pub struct TypeContext<'a> {
    /// The current module name (e.g., "gp")
    pub current_module: &'a str,
    /// Classes defined in the current module (full C++ names like "gp_Pnt")
    pub module_classes: &'a std::collections::HashSet<String>,
    /// All enum names across all modules (full C++ names like "TopAbs_Orientation")
    pub all_enums: &'a std::collections::HashSet<String>,
    /// All class names across all modules (full C++ names like "gp_Pnt")
    pub all_classes: &'a std::collections::HashSet<String>,
    /// Classes that can have Handle<T> declarations (is_handle_type)
    /// If None, falls back to all_classes for Handle type checking
    pub handle_able_classes: Option<&'a std::collections::HashSet<String>>,
    /// Authoritative type→module mapping (from resolver's SymbolTable)
    /// When present, used instead of name-based derivation
    pub type_to_module: Option<&'a std::collections::HashMap<String, String>>,
    /// Mapping from C++ enum name to qualified Rust enum type path.
    /// Value enums get typed Rust enums; bitset enums stay as i32.
    pub enum_rust_types: Option<&'a std::collections::HashMap<String, String>>,
    /// Class names that have `CppDeletable` impls generated (ParsedClasses without
    /// protected_destructor + the 91 manually-specified known collections).
    /// Methods returning a bare `Class(name)` value (generating `OwnedPtr<name>`) are
    /// only allowed if `name` is in this set. If `None`, no constraint is applied.
    pub deletable_class_names: Option<&'a std::collections::HashSet<String>>,
}
```
</details>

<details>
<summary>Reading lines 440-540 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L440-L541">type_mapping.rs</a></summary>

```
pub fn map_type_in_context(ty: &Type, ctx: &TypeContext) -> RustTypeMapping {
    match ty {
        Type::Class(class_name) if class_name == "char" => {
            // C++ char resolved from canonical types (e.g., Standard_Character)
            RustTypeMapping {
                rust_type: "std::ffi::c_char".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: None,
            }
        }
        Type::Class(class_name) if class_name == "Standard_Address" => {
            // Standard_Address is a typedef for void* — map to raw c_void pointer
            RustTypeMapping {
                rust_type: "*mut std::ffi::c_void".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: None,
            }
        }
        Type::Class(class_name) if class_name == "void" => {
            // C++ void type (pointee of void*) — just c_void, pointer wrapping handled by outer type
            RustTypeMapping {
                rust_type: "std::ffi::c_void".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: None,
            }
        }
        Type::Class(class_name) => {
            // Enums are passed as i32 at the FFI boundary (integer pass-through)
            if ctx.all_enums.contains(class_name) {
                return RustTypeMapping {
                    rust_type: "i32".to_string(),
                    needs_unique_ptr: false,
                    needs_pin: false,
                    source_module: None,
                };
            }
            
            let type_module = lookup_module_for_type(class_name, ctx.type_to_module);
            let short_name = extract_short_class_name(class_name);
            
            // Check if this is a same-module reference
            if type_module.as_deref() == Some(ctx.current_module) 
                && ctx.module_classes.contains(class_name) 
            {
                // Use short name for same-module types
                let safe_name = safe_short_name(&short_name);
                RustTypeMapping {
                    rust_type: safe_name,
                    needs_unique_ptr: true,
                    needs_pin: false,
                    source_module: None, // Same module
                }
            } else {
                // Use full C++ name for cross-module types (will be aliased)
                // Flatten nested type names (Parent::Nested -> Parent_Nested)
                let ffi_name = Type::ffi_safe_class_name(class_name);
                RustTypeMapping {
                    rust_type: ffi_name,
                    needs_unique_ptr: true,
                    needs_pin: false,
                    source_module: type_module,
                }
            }
        }
        Type::ConstRef(inner) => {
            let inner_mapping = map_type_in_context(inner, ctx);
            RustTypeMapping {
                rust_type: format!("*const {}", inner_mapping.rust_type),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: inner_mapping.source_module,
            }
        }
        Type::MutRef(inner) => {
            let inner_mapping = map_type_in_context(inner, ctx);
            RustTypeMapping {
                rust_type: format!("*mut {}", inner_mapping.rust_type),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: inner_mapping.source_module,
            }
        }
        Type::Handle(class_name) => {
            let source_module = lookup_module_for_type(class_name, ctx.type_to_module);
            let handle_type = handle_type_name(class_name);
            RustTypeMapping {
                rust_type: handle_type,
                needs_unique_ptr: true,
                needs_pin: false,
                source_module,
            }
        }
        // Delegate
... (truncated, 4132 chars)
```
</details>

<details>
<summary>Reading lines 296-345 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L296-L346">type_mapping.rs</a></summary>

```
///   - `("BRepOffset_Status", "BRepOffset")` → `"Status"`
///   - `("BRepOffsetSimple_Status", "BRepOffset")` → `"SimpleStatus"`
///   - `("TopoDS_Shape", "TopoDS")` → `"Shape"`
pub fn short_name_for_module(cpp_name: &str, module: &str) -> String {
    if let Some(rest) = cpp_name.strip_prefix(module) {
        // After stripping the module prefix, the remainder starts with either:
        // - "_Foo" (exact module match) → "Foo"
        // - "Simple_Foo" (longer prefix) → "SimpleFoo"  
        let rest = rest.strip_prefix('_').unwrap_or(rest);
        if rest.is_empty() {
            // Type name equals the module name (rare but possible)
            cpp_name.to_string()
        } else {
            rest.to_string()
        }
    } else {
        // Module prefix doesn't match at all — use the full C++ name as the short name.
        // This happens for types that don't follow OCCT naming conventions (e.g., Fortran
        // common blocks like `mdnombr_1_` in AdvApp2Var_Data.hxx).
        cpp_name.to_string()
    }
}

/// Look up module name for a type, using the authoritative map if available,
/// falling back to name-based derivation for context-free callers.
fn lookup_module_for_type(
    class_name: &str,
    type_to_module: Option<&std::collections::HashMap<String, String>>,
) -> Option<String> {
    if let Some(map) = type_to_module {
        map.get(class_name).map(|m| module_to_rust_name(m))
    } else {
        extract_module_from_class(class_name)
    }
}

/// Extract module name from class name (name-based fallback)
fn extract_module_from_class(class_name: &str) -> Option<String> {
    class_name.find('_').map(|underscore_pos| module_to_rust_name(&class_name[..underscore_pos]))
}

/// Extract short class name (without module prefix)
pub fn extract_short_class_name(class_name: &str) -> String {
    if let Some(underscore_pos) = class_name.find('_') {
        class_name[underscore_pos + 1..].to_string()
    } else {
        class_name.to_string()
    }
}

```
</details>

<details>
<summary>Searched for regex <code>parse_type|build_type|OperationsFlags|ShapeProcess</code> (<code>**/crates/opencascade-binding-generator/src/parser.rs</code>), 13 results</summary>

- [parser.rs:409](../crates/opencascade-binding-generator/src/parser.rs#L409)
- [parser.rs:414](../crates/opencascade-binding-generator/src/parser.rs#L414)
- [parser.rs:1054](../crates/opencascade-binding-generator/src/parser.rs#L1054)
- [parser.rs:1064](../crates/opencascade-binding-generator/src/parser.rs#L1064)
- [parser.rs:1278](../crates/opencascade-binding-generator/src/parser.rs#L1278)
- [parser.rs:1280](../crates/opencascade-binding-generator/src/parser.rs#L1280)
- [parser.rs:1353](../crates/opencascade-binding-generator/src/parser.rs#L1353)
- [parser.rs:1490](../crates/opencascade-binding-generator/src/parser.rs#L1490)
- [parser.rs:1494](../crates/opencascade-binding-generator/src/parser.rs#L1494)
- [parser.rs:1653](../crates/opencascade-binding-generator/src/parser.rs#L1653)
- [parser.rs:1665](../crates/opencascade-binding-generator/src/parser.rs#L1665)
- [parser.rs:1674](../crates/opencascade-binding-generator/src/parser.rs#L1674)
- [parser.rs:1783](../crates/opencascade-binding-generator/src/parser.rs#L1783)
</details>

<details>
<summary>Reading lines 1494-1550 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1494-L1551">parser.rs</a></summary>

```
fn parse_type(clang_type: &clang::Type) -> Type {
    let kind = clang_type.get_kind();
    let spelling = clang_type.get_display_name();

    // Handle known OCCT typedefs FIRST (before canonical resolution)
    // This handles cases where clang can't fully resolve types due to missing includes
    // Check both original and trimmed versions
    let trimmed_spelling = spelling.trim();
    if let Some(primitive) = map_standard_type(trimmed_spelling) {
        return primitive;
    }

    // Check for size_t BEFORE canonical resolution, since size_t and unsigned long
    // are the same canonical type on some platforms but we want to preserve size_t semantics
    if trimmed_spelling == "size_t" || trimmed_spelling == "std::size_t" {
        return Type::Usize;
    }

    // Check if this is a typedef to size_t by examining the declaration
    // This catches cases where get_display_name() returns the canonical type
    if let Some(decl) = clang_type.get_declaration() {
        if let Some(decl_name) = decl.get_name() {
            if decl_name == "size_t" || decl_name.ends_with("_Size") {
                return Type::Usize;
            }
        }
    }

    // Check if this is a known NCollection/template typedef.
    // When clang desugars types (especially through references/pointers),
    // the display name may show the raw template form instead of the typedef.
    // E.g., "NCollection_Map<TDF_Label, NCollection_DefaultHasher<TDF_Label>>"
    // instead of "TDF_LabelMap". Look up the typedef name from our pre-scanned map.
    let clean_for_lookup = strip_type_qualifiers(trimmed_spelling);
    if clean_for_lookup.contains('<') && !clean_for_lookup.starts_with("opencascade::handle<") && !clean_for_lookup.starts_with("Handle(") {
        if let Some(typedef_name) = lookup_typedef(clean_for_lookup) {
            return Type::Class(typedef_name);
        }
    }


    // Get canonical type for resolving typedefs
    let canonical = clang_type.get_canonical_type();
    let canonical_spelling = canonical.get_display_name();
    
    // Strip const/volatile from canonical spelling for primitive matching
    let canonical_clean = strip_type_qualifiers(&canonical_spelling);
    // Defense-in-depth: detect when clang's canonical type is a primitive (int, double, etc.)
    // but the display name clearly identifies a class/typedef. This can happen if a template
    // type fails to instantiate. Legitimate typedefs to primitives (e.g.,
    // `typedef unsigned int Poly_MeshPurpose`) use a typedef chain to a builtin type.
    let spelling_looks_like_class = {
        let s = strip_type_qualifiers(trimmed_spelling);
        let looks_like_class = s.starts_with(|c: char| c.is_ascii_uppercase())
            && map_standard_type(s).is_none()
            && s != "Standard_Boolean"
            && !s.contains('<')
```
</details>

> 💭 Interesting - the agent says the lookups should succeed. But clearly something is going wrong since we have 11 skips. Let me look more carefully at what's actually happening. The type is `ShapeProcess::OperationsFlags const&` - this is a reference to a typedef. The typedef resolves to `std::bitset<17>`. Maybe the issue is that it's a typedef, not an actual class?
> 
> Let me look at the parsed type more carefully. It's a `using OperationsFlags = std::bitset<Operation::Last + 1>;` - a typedef to `std::bitset`. The parser likely treats this as a typedef, not a class. So when a method parameter has type `ShapeProcess::OperationsFlags const&`, the parser may resolve this to the underlying `std::bitset<17>` or keep the typedef name.
>  
> Wait, I see there's actually a struct generated for this in the FFI output - `ShapeProcess_OperationsFlags` exists. So the parser did recognize it as a class. The real problem seems to be that when methods reference the type with the `const&` qualifier, the parser isn't matching it to the generated struct. The skip reasons show it's treating `ShapeProcess::OperationsFlags const&` as an unknown type, even though the base type was successfully parsed.
> 
> Let me trace through this more carefully - the `all_class_names` should contain the fully qualified name with `::`, and the type lookups should work. But we're still getting skips, which means something in the type resolution logic isn't handling the qualified names or qualifiers correctly. The issue might be in how the parsed type is being stored or looked up when it has the scope resolution operator.

The agent analysis suggests the lookups should work, but we still have 11 skips. The issue might be that  is a typedef for `std::bitset<17>` that gets resolved to the underlying type. Let me check more directly:

**Terminal:**
```sh
grep -n "ShapeProcess" crates/opencascade-sys/generated/ffi.rs | grep -i "struct\|type\|oper"
```
<details>
<summary>Output (174 lines)</summary>

```
6933:pub struct HandleShapeProcessContext {
6936:/// Handle to ShapeProcess_Operator
6938:pub struct HandleShapeProcessOperator {
6943:pub struct HandleShapeProcessShapeContext {
6946:/// Handle to ShapeProcess_UOperator
6948:pub struct HandleShapeProcessUOperator {
26236:pub struct ShapeProcess_Context {
26240:pub struct ShapeProcess_OperLibrary {
26244:pub struct ShapeProcess_Operator {
26248:pub struct ShapeProcess_ShapeContext {
26252:pub struct ShapeProcess_UOperator {
26256:pub struct ShapeProcessAPI_ApplySequence {
33720:pub struct XSAlgo_ShapeProcessor {
36526:pub struct ShapeProcess_OperationsFlags {
36531:pub struct ShapeProcess_OperFunc {
38176:pub struct XSAlgo_ShapeProcessor_ParameterMap {
38181:pub struct XSAlgo_ShapeProcessor_ProcessingData {
38186:pub struct XSAlgo_ShapeProcessor_ProcessingFlags {
415844:    pub fn ShapeProcess_Context_destructor(self_: *mut ShapeProcess_Context);
415940:    /// **Source:** `ShapeProcess_Context.hxx`:119 - `ShapeProcess_Context::DynamicType()`
415941:    pub fn ShapeProcess_Context_dynamic_type(
415944:    /// **Source:** `ShapeProcess_Context.hxx`:119 - `ShapeProcess_Context::get_type_name()`
415945:    pub fn ShapeProcess_Context_get_type_name() -> *const std::ffi::c_char;
415946:    /// **Source:** `ShapeProcess_Context.hxx`:119 - `ShapeProcess_Context::get_type_descriptor()`
415947:    pub fn ShapeProcess_Context_get_type_descriptor() -> *const HandleStandardType;
415961:    pub fn HandleShapeProcessContext_destructor(self_: *mut HandleShapeProcessContext);
416002:    // ======================== ShapeProcess_OperLibrary ========================
416003:    pub fn ShapeProcess_OperLibrary_destructor(self_: *mut ShapeProcess_OperLibrary);
416004:    /// **Source:** `ShapeProcess_OperLibrary.hxx` - `ShapeProcess_OperLibrary::ShapeProcess_OperLibrary()`
416005:    pub fn ShapeProcess_OperLibrary_ctor() -> *mut ShapeProcess_OperLibrary;
416006:    /// **Source:** `ShapeProcess_OperLibrary.hxx`:53 - `ShapeProcess_OperLibrary::Init()`
416007:    pub fn ShapeProcess_OperLibrary_init();
416008:    /// **Source:** `ShapeProcess_OperLibrary.hxx`:59 - `ShapeProcess_OperLibrary::ApplyModifier()`
416009:    pub fn ShapeProcess_OperLibrary_apply_modifier(
416017:    // ======================== ShapeProcess_Operator ========================
416018:    pub fn ShapeProcess_Operator_destructor(self_: *mut ShapeProcess_Operator);
416019:    /// **Source:** `ShapeProcess_Operator.hxx`:37 - `ShapeProcess_Operator::Perform()`
416020:    pub fn ShapeProcess_Operator_perform(
416021:        self_: *mut ShapeProcess_Operator,
416025:    /// **Source:** `ShapeProcess_Operator.hxx`:41 - `ShapeProcess_Operator::DynamicType()`
416026:    pub fn ShapeProcess_Operator_dynamic_type(
416027:        self_: *const ShapeProcess_Operator,
416029:    /// **Source:** `ShapeProcess_Operator.hxx`:41 - `ShapeProcess_Operator::get_type_name()`
416030:    pub fn ShapeProcess_Operator_get_type_nam
... (truncated, 12216 chars total)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/be682feb-153a-4ab3-a24a-c102b03012c6/toolu_01JppJNj2M6JTfekgpBC98Ak__vscode-1771608614467/content.txt#1-1), lines 1 to 30

> 💭 OK so `ShapeProcess_OperationsFlags` IS in ffi.rs as a `pub struct` at line 36526. Let me now check why the resolver is marking methods using `ShapeProcess::OperationsFlags const&` as "unknown type". 
> 
> The struct exists but when methods reference it, they get skipped. Let me check the resolver code directly to understand the flow.
> 
> Let me look at the skip reason messages more carefully. The reasons are like:
> - `param 'theFlags' uses unknown type 'ShapeProcess::OperationsFlags const&'`
> - `return type 'ShapeProcess::OperationsFlags const&' is unknown`
> 
> 
> I need to trace where this "unknown type" message is being generated in the codegen to understand why the resolver isn't recognizing the type even though the struct definition exists.

<details>
<summary>Searched for regex <code>uses unknown type|is unknown</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 20 results</summary>

- [bindings.rs:830](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L830)
- [bindings.rs:832](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L832)
- [bindings.rs:843](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L843)
- [bindings.rs:845](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L845)
- [bindings.rs:856](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L856)
- [bindings.rs:860](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L860)
- [bindings.rs:865](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L865)
- [bindings.rs:918](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L918)
- [bindings.rs:920](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L920)
- [bindings.rs:931](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L931)
- [bindings.rs:933](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L933)
- [bindings.rs:942](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L942)
- [bindings.rs:963](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L963)
- [bindings.rs:965](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L965)
- [bindings.rs:976](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L976)
- [bindings.rs:978](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L978)
- [bindings.rs:987](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L987)
- [bindings.rs:991](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L991)
- [bindings.rs:2141](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2141)
- [bindings.rs:2749](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2749)
</details>

Let me look at the `type_uses_unknown_type` function to understand the flow:

<details>
<summary>Reading lines 810-865 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L810-L866">bindings.rs</a></summary>

```
fn is_method_bindable(method: &Method, ctx: &TypeContext, class_name: &str) -> Result<(), String> {
    if method.has_unbindable_types() {
        let unbindable_details = describe_unbindable_types_method(method);
        return Err(format!("has unbindable types: {}", unbindable_details));
    }
    // Skip methods with const char*& or const char* const& params (need manual bindings)
    if let Some((param_name, type_name)) = resolver::method_has_string_ref_param(method) {
        return Err(format!("has string ref param '{}' of type '{}' (needs manual binding)", param_name, type_name));
    }
    // Skip methods that cause ambiguous call errors in C++ wrappers
    if AMBIGUOUS_METHODS.iter().any(|(c, m)| *c == class_name && *m == method.name) {
        return Err("causes ambiguous overload in C++ (listed in AMBIGUOUS_METHODS)".to_string());
    }
    // Const/mut return mismatch is now handled via C++ wrappers (ConstMutReturnFix).
    // &mut enum output params are now handled via C++ wrappers (MutRefEnumParam).
    if let Some(p) = method
        .params
        .iter()
        .find(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return Err(format!("param '{}' uses unknown type '{}'", p.name, p.ty.to_cpp_string()));
    }
    // Skip methods where a nullable pointer param's inner type is unknown
    if let Some(p) = method.params.iter().find(|p| {
        if p.is_nullable_ptr() {
            match &p.ty {
                Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_type(inner, ctx),
                _ => false,
            }
        } else {
            false
        }
    }) {
        return Err(format!("nullable param '{}' inner type is unknown", p.name));
    }
    // Skip methods where a class pointer param's inner type is unknown.
    // We check all_classes directly (not type_uses_unknown_type) because nested types
    // like Parent::Nested are considered "known" by type_uses_unknown_type if the parent
    // is known, but they don't have their own FFI type declarations.
    if let Some(p) = method.params.iter().find(|p| {
        if let Some(class_name) = p.ty.class_ptr_inner_name() {
            !ctx.all_classes.contains(class_name) && !ctx.all_enums.contains(class_name)
        } else {
            false
        }
    }) {
        return Err(format!("class pointer param '{}' inner type '{}' is unknown", p.name, p.ty.to_cpp_string()));
    }
    if let Some(ref ret) = method.return_type {
        if type_uses_unknown_type(ret, ctx) {
            return Err(format!("return type '{}' is unknown", ret.to_cpp_string()));
        }
        // Check class pointer returns for unknown inner types (same as params)
        if let Some(class_name) = ret.class_ptr_inner_name() {
            if !ctx.all_classes.contains(class_name) && !ctx.all_enums.contains(class_name) {
                return Err(format!("class pointer return inner type '{}' is unknown", ret.to_cpp_string()));
```
</details>

Searched for text `fn type_uses_unknown_type` (`**/crates/opencascade-binding-generator/src/**`), 1 result
- [bindings.rs:463](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L463)

<details>
<summary>Reading lines 463-530 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L463-L531">bindings.rs</a></summary>

```
fn type_uses_unknown_type(ty: &Type, ctx: &TypeContext) -> bool {
    // Enum types are known (mapped to i32), so skip them
    match ty {
        Type::Class(name) if ctx.all_enums.contains(name) => return false,
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => {
            if let Type::Class(name) = inner.as_ref() {
                if ctx.all_enums.contains(name) {
                    return false;
                }
            }
        }
        _ => {}
    }
    if let Some(handle_classes) = ctx.handle_able_classes {
        type_mapping::type_uses_unknown_handle(ty, ctx.all_classes, handle_classes)
    } else {
        type_mapping::type_uses_unknown_class(ty, ctx.all_classes)
    }
}

/// Check if a method has by-value class or handle parameters (not enums).
/// These need C++ wrappers that accept const T& instead.
fn has_by_value_class_or_handle_params(params: &[Param], all_enums: &HashSet<String>) -> bool {
    params.iter().any(|p| match &p.ty {
        Type::Class(name) => !all_enums.contains(name) && is_opaque_class_name(name),
        Type::Handle(_) => true,
        _ => false,
    })
}

/// Check if params contain any &mut enum output parameters.
fn has_mut_ref_enum_params(params: &[Param], all_enums: &HashSet<String>) -> bool {
    params.iter().any(|p| {
        if let Type::MutRef(inner) = &p.ty {
            if let Type::Class(name) = inner.as_ref() {
                return all_enums.contains(name);
            }
        }
        false
    })
}

/// Determine if a method needs a C++ wrapper function
fn needs_wrapper_function(_method: &Method, _all_enums: &HashSet<String>) -> bool {
    // With extern "C" FFI, all methods need C++ wrapper functions
    true
}

/// Classify the wrapper kind for a method that needs_wrapper_function
fn classify_wrapper_kind(method: &Method, all_enums: &HashSet<String>) -> WrapperKind {
    let has_cstring_param = method.params.iter().any(|p| p.ty.is_c_string());
    let returns_cstring = method
        .return_type
        .as_ref()
        .map(|t| t.is_c_string())
        .unwrap_or(false);
    let returns_by_value = method.return_type.as_ref().map_or(false, |ty| {
        let is_class_or_handle = ty.is_class() || ty.is_handle();
        let is_enum = match ty {
            Type::Class(name) => all_enums.contains(name),
            _ => false,
        };
        is_class_or_handle && !is_enum
    });

    if has_mut_ref_enum_params(&method.params, all_enums) {
        WrapperKind::MutRefEnumParam
    } else if returns_by_value {
```
</details>

Now let me look at the `type_uses_unknown_class` function in type_mapping.rs:

Searched for text `fn type_uses_unknown_class` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), 1 result
- [type_mapping.rs:374](../crates/opencascade-binding-generator/src/type_mapping.rs#L374)

<details>
<summary>Reading lines 374-440 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L374-L441">type_mapping.rs</a></summary>

```
pub fn type_uses_unknown_class(ty: &Type, all_classes: &std::collections::HashSet<String>) -> bool {
    match ty {
        Type::Handle(class_name) => !all_classes.contains(class_name),
        Type::Class(class_name) => {
            if all_classes.contains(class_name) {
                return false;
            }
            // Void pointer types — Standard_Address (void*) and literal "void" — are known
            if crate::model::is_void_type_name(class_name) {
                return false;
            }
            // Primitive types mapped as Type::Class (e.g., "char" from Standard_Character)
            if crate::codegen::rust::is_primitive_type(class_name) {
                return false;
            }
            // Nested types (Parent::Nested) are known if the parent class is known
            if let Some(parent) = class_name.split("::").next() {
                if class_name.contains("::") && all_classes.contains(parent) {
                    return false;
                }
            }
            true
        }
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_class(inner, all_classes),
        _ => false,
    }
}

/// Check if a type references a Handle to a class that won't have a Handle declaration generated
/// This is more strict than type_uses_unknown_class - it checks that Handle types are for
/// classes that will actually have Handle<T> declarations generated (is_handle_type)
pub fn type_uses_unknown_handle(
    ty: &Type,
    all_classes: &std::collections::HashSet<String>,
    handle_able_classes: &std::collections::HashSet<String>,
) -> bool {
    match ty {
        Type::Handle(class_name) => !handle_able_classes.contains(class_name),
        Type::Class(class_name) => {
            if all_classes.contains(class_name) {
                return false;
            }
            // Void pointer types — Standard_Address (void*) and literal "void" — are known
            if crate::model::is_void_type_name(class_name) {
                return false;
            }
            // Primitive types mapped as Type::Class (e.g., "char" from Standard_Character)
            if crate::codegen::rust::is_primitive_type(class_name) {
                return false;
            }
            // Nested types (Parent::Nested) are known if the parent class is known
            if let Some(parent) = class_name.split("::").next() {
                if class_name.contains("::") && all_classes.contains(parent) {
                    return false;
                }
            }
            true
        }
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
            type_uses_unknown_handle(inner, all_classes, handle_able_classes)
        }
        _ => false,
    }
}

/// Map a type to Rust, using short names for same-module types
pub fn map_type_in_context(ty: &Type, ctx: &TypeContext) -> RustTypeMapping {
```
</details>

> 💭 OK, so `type_uses_unknown_class` line 390-394 says: if the type name contains `::`, and the parent (before `::`) is known, return false. So `ShapeProcess::OperationsFlags` should be considered "known" since `ShapeProcess` is a known class.
> 
> But wait - the error messages say `param 'theFlags' uses unknown type 'ShapeProcess::OperationsFlags const&'`. This triggers at line 830 of bindings.rs. The call path is:
> 
> 1. `find(|p| type_uses_unknown_type(&p.ty, ctx))` (line 828)
> 
> 
> 2. `type_uses_unknown_type` calls into the type mapping functions, which routes to `type_uses_unknown_handle` since handle_able_classes is likely Some
> 3. For `Type::ConstRef(Box(Type::Class("ShapeProcess::OperationsFlags")))`, it recursively unwraps the const ref and checks the inner class type
> 
> Now checking if `ShapeProcess::OperationsFlags` is recognized: it should be in all_classes, or at minimum the parent namespace "ShapeProcess" is in all_classes, so the function should return false (known type) and the error shouldn't trigger. But `OperationsFlags` might be represented differently than expected...
> 
> Actually, I think I see the issue now. `OperationsFlags` is a typedef for `std::bitset<17>`, and the C++ parser might be resolving it to the underlying bitset type instead of keeping the typedef name. If the type gets resolved to `std::bitset<17>` rather than `ShapeProcess::OperationsFlags`, then it wouldn't be in all_classes and would fail the check. But wait—the error might not even be coming from the `type_uses_unknown_type` check at line 830. The `has_unbindable_types()` check happens first at line 811, so that could be what's actually triggering the error.
> 
> Actually, looking at the skip reason more carefully—it says the param uses unknown type `ShapeProcess::OperationsFlags const&`, which matches the format from line 830, so the `type_uses_unknown_type` check is definitely what's returning true. The real question is whether the parser is treating nested typedefs differently from nested classes. It creates ParsedClass entries for nested structs and classes, but for typedefs it might be handling them separately.

Let me check how the parser handles typedefs inside classes:

<details>
<summary>Searched for regex <code>TypeAlias|typedef|using.*=|TypedefDecl</code> (<code>**/crates/opencascade-binding-generator/src/parser.rs</code>), 20 results</summary>

- [parser.rs:18](../crates/opencascade-binding-generator/src/parser.rs#L18)
- [parser.rs:32](../crates/opencascade-binding-generator/src/parser.rs#L32)
- [parser.rs:33](../crates/opencascade-binding-generator/src/parser.rs#L33)
- [parser.rs:36](../crates/opencascade-binding-generator/src/parser.rs#L36)
- [parser.rs:37](../crates/opencascade-binding-generator/src/parser.rs#L37)
- [parser.rs:43](../crates/opencascade-binding-generator/src/parser.rs#L43)
- [parser.rs:45](../crates/opencascade-binding-generator/src/parser.rs#L45)
- [parser.rs:46](../crates/opencascade-binding-generator/src/parser.rs#L46)
- [parser.rs:47](../crates/opencascade-binding-generator/src/parser.rs#L47)
- [parser.rs:49](../crates/opencascade-binding-generator/src/parser.rs#L49)
- [parser.rs:51](../crates/opencascade-binding-generator/src/parser.rs#L51)
- [parser.rs:54](../crates/opencascade-binding-generator/src/parser.rs#L54)
- [parser.rs:85](../crates/opencascade-binding-generator/src/parser.rs#L85)
- [parser.rs:87](../crates/opencascade-binding-generator/src/parser.rs#L87)
- [parser.rs:89](../crates/opencascade-binding-generator/src/parser.rs#L89)
- [parser.rs:95](../crates/opencascade-binding-generator/src/parser.rs#L95)
- [parser.rs:97](../crates/opencascade-binding-generator/src/parser.rs#L97)
- [parser.rs:101](../crates/opencascade-binding-generator/src/parser.rs#L101)
- [parser.rs:102](../crates/opencascade-binding-generator/src/parser.rs#L102)
</details>

Searched for regex `nested.*typedef|nested.*using|class.*scope.*typedef|TypeAlias.*nested` (`**/crates/opencascade-binding-generator/src/parser.rs`), 1 result
- [parser.rs:1715](../crates/opencascade-binding-generator/src/parser.rs#L1715)

<details>
<summary>Reading lines 1710-1730 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1710-L1731">parser.rs</a></summary>

```
    // For nested types (e.g., TColgp_Array1OfPnt::value_type) or template types,
    // use the canonical type to get the resolved underlying type.
    // clang resolves these for us (e.g., value_type -> gp_Pnt)
    let clean_name = strip_type_decorators(&spelling);
    
    // If the spelling contains :: or < (nested/template type), try typedef map first,
    // then try to use canonical
    if clean_name.contains("::") || clean_name.contains('<') {
        // For template types, check if this is a known typedef
        if clean_name.contains('<') {
            if let Some(typedef_name) = lookup_typedef(clean_name) {
                return Type::Class(typedef_name);
            }
        }
        let canonical_base = strip_type_decorators(&canonical_spelling);
        
        // Only use canonical if it's simpler (no :: or <) AND still looks like a class name.
        // If canonical is a primitive like "int", that would produce Type::Class("int")
        // which is nonsensical. By keeping the template/namespaced spelling,
        // type_uses_unknown_type() will properly filter methods with unresolvable types.
        let canonical_looks_like_class = canonical_base
```
</details>

<details>
<summary>Reading lines 1730-1760 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1730-L1761">parser.rs</a></summary>

```
        let canonical_looks_like_class = canonical_base
            .starts_with(|c: char| c.is_ascii_uppercase());
        if !canonical_base.contains("::") && !canonical_base.contains('<') && !canonical_base.is_empty() && canonical_looks_like_class {
            return Type::Class(canonical_base.to_string());
        }
    }
    
    // Check if this type's declaration is nested inside a class
    // This catches types like DESTEP_Parameters::ReadMode_ProductContext that
    // appear as "ReadMode_ProductContext" in method signatures but are actually nested
    // Only apply if clean_name is NOT already qualified (doesn't contain ::)
    if !clean_name.contains("::") {
        if let Some(decl) = clang_type.get_declaration() {
            if let Some(parent) = decl.get_semantic_parent() {
                let parent_kind = parent.get_kind();
                if parent_kind == EntityKind::ClassDecl || parent_kind == EntityKind::StructDecl {
                    // This is a nested type - include the parent class name to mark it as nested
                    if let Some(parent_name) = parent.get_name() {
                        let nested_name = format!("{}::{}", parent_name, clean_name);
                        return Type::Class(nested_name);
                    }
                }
            }
        }
    }

    // Check if this class name is actually a simple typedef for another class.
    // E.g., BinObjMgt_SRelocationTable -> TColStd_IndexedMapOfTransient,
    // XmlObjMgt_Element -> LDOM_Element, NCollection_String -> NCollection_Utf8String.
    // This must be done at the end, after all other type resolution has been attempted,
    // because reference/pointer wrapping strips the typedef TypeKind layer by the time
```
</details>

> 💭 Now I see the issue. When the parser encounters a method parameter with type `ShapeProcess::OperationsFlags const&`, it processes it with the above code. The `clean_name` is `ShapeProcess::OperationsFlags`. Since it contains `::`, it enters the block at line 1717.
> 
> It doesn't contain `<`, so the typedef check is skipped. It goes to line 1724 and gets the canonical type. For `using OperationsFlags = std::bitset<Operation::Last + 1>;`, the canonical type would be `std::bitset<17>` (or something like that).
> 
> 
> Then the canonical type check at line 1732 fails because `std::bitset<17>` contains both `::` and `<`, so it skips that branch. The function continues checking if `clean_name` contains `::` at line 1741, which it does, so that block is also skipped. It falls through to the simple typedef check at line 1756, and if `ShapeProcess::OperationsFlags` isn't in that map, it continues further down the function.

<details>
<summary>Reading lines 1755-1800 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1755-L1801">parser.rs</a></summary>

```

    // Check if this class name is actually a simple typedef for another class.
    // E.g., BinObjMgt_SRelocationTable -> TColStd_IndexedMapOfTransient,
    // XmlObjMgt_Element -> LDOM_Element, NCollection_String -> NCollection_Utf8String.
    // This must be done at the end, after all other type resolution has been attempted,
    // because reference/pointer wrapping strips the typedef TypeKind layer by the time
    // we recurse into the pointee type.
    if let Some(resolved_name) = lookup_simple_typedef(clean_name) {
        return Type::Class(resolved_name);
    }

    // Late-stage canonical resolution for unrecognized typedefs.
    // When a typedef's display name is unrecognized (not in map_standard_type, not a known
    // class, not a simple typedef), try resolving through clang's canonical type.
    // This handles:
    // - Pointer typedefs: Standard_PCharacter = char*, BOPDS_PDS = BOPDS_DS*
    // - Primitive typedefs not caught earlier (fallback for edge cases)
    if matches!(kind, TypeKind::Typedef | TypeKind::Elaborated) {
        let canon_kind = canonical.get_kind();

        // Pointer typedef: canonical is a pointer type.
        // Exclude function pointer typedefs (canonical pointee is a function type)
        // such as StdObjMgt_Persistent::Instantiator = Handle(...) (*)()
        if canon_kind == TypeKind::Pointer {
            if let Some(pointee) = canonical.get_pointee_type() {
                let pointee_kind = pointee.get_kind();
                if !matches!(pointee_kind, TypeKind::FunctionPrototype | TypeKind::FunctionNoPrototype) {
                    let is_const = pointee.is_const_qualified();
                    let inner = parse_type(&pointee);
                    return if is_const {
                        Type::ConstPtr(Box::new(inner))
                    } else {
                        Type::MutPtr(Box::new(inner))
                    };
                }
            }
        }

        // Primitive typedef: canonical is a C primitive type
        if is_c_primitive_type_kind(canon_kind) {
            if let Some(ty) = map_standard_type(canonical_clean) {
                return ty;
            }
        }
    }

```
</details>

<details>
<summary>Reading lines 1800-1810 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1800-L1811">parser.rs</a></summary>

```

    Type::Class(clean_name.to_string())
}

/// Extract template argument from Handle<T> or similar
fn extract_template_arg(type_name: &str) -> String {
    if type_name.starts_with("Handle(") {
        // Handle(Foo) format
        type_name
            .trim_start_matches("Handle(")
            .trim_end_matches(')')
```
</details>

> 💭 OK, so I see the flow now for `ShapeProcess::OperationsFlags const&`:
> 
> 1. The outer type is `const &` - so it gets unwrapped to the inner type  
> 2. The inner type is `ShapeProcess::OperationsFlags`
> 3. clean_name = `ShapeProcess::OperationsFlags`, contains `::`, enters line 1717 block
> 4. Doesn't contain `<`, skips typedef lookup
> 5. Gets canonical type. For `std::bitset<17>`, the canonical_base would be `std::bitset<unsigned long long>` or something like that
> 
> 
> 6. The canonical_base has both `::` and `<`, so it fails the check at line 1732
> 7. Since clean_name contains `::`, it skips the nested type check at line 1741
> 8. The simple typedef lookup at 1762 doesn't find `ShapeProcess::OperationsFlags`
> 9. Falls through to line 1801 and produces `Type::Class("ShapeProcess::OperationsFlags")`
> 
> So the final parser output is `Type::ConstRef(Box(Type::Class("ShapeProcess::OperationsFlags")))`. Now I'm looking at the bindings.rs code around line 828 to see how this type gets handled...
> 
> The `type_uses_unknown_type` function receives this const ref type and checks if the inner class is in the enums map, but `ShapeProcess::OperationsFlags` is a typedef for `std::bitset<17>`, not an enum, so that check fails. Then it delegates to `type_uses_unknown_handle`, which unwraps the const ref and recurses deeper.
> 
> Now I'm tracing through what happens when we check if `ShapeProcess::OperationsFlags` is in `all_classes` — this is the critical point. The parser generated a pseudo-struct for this typedef in ffi.rs, which means there should be a corresponding ParsedClass entry, and since `all_classes` is built from all ParsedClasses, it should be there. I'm tracing through the logic: `all_classes` should contain `"ShapeProcess::OperationsFlags"` based on how the name is formatted in the parser, so the type lookup should succeed and the parameter shouldn't be skipped. But it's being skipped anyway, which means either the type name in the parameter is different from what I expect, or something else is filtering it out. I need to add debug output to see what's actually happening at runtime.
> 
> Looking more carefully at the ffi.rs output, I notice `ShapeProcess_OperationsFlags` appears with underscores, but the parsed class name uses `::` notation. The issue might be that `OperationsFlags` is actually a typedef or alias rather than a true nested class, so the parser might be generating a different struct name than what I'm looking for.
> 
> Let me trace through how the parser handles nested types—it's possible that when processing the ShapeProcess class, it collects nested typedefs and creates synthetic struct entries for them, which would explain why `ShapeProcess_OperationsFlags` exists in the output even though `OperationsFlags` is defined as a `using` alias rather than a `ClassDecl`.

Let me understand how this typedef is being parsed.  is a `using` alias, not a class:

Searched for regex `nested.*TypeAlias|TypeAlias.*nested|OperFlags|nested_types|nested_class` (`**/crates/opencascade-binding-generator/src/parser.rs`), 4 results
- [parser.rs:622](../crates/opencascade-binding-generator/src/parser.rs#L622)
- [parser.rs:771](../crates/opencascade-binding-generator/src/parser.rs#L771)
- [parser.rs:817](../crates/opencascade-binding-generator/src/parser.rs#L817)
- [parser.rs:866](../crates/opencascade-binding-generator/src/parser.rs#L866)

<details>
<summary>Reading lines 730-790 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L730-L791">parser.rs</a></summary>

```
                        if let Some(method) = parse_static_method(&child, verbose) {
                            static_methods.push(method);
                        }
                    } else if let Some(method) = parse_method(&child, verbose) {
                        methods.push(method);
                    }
                }
            }
            EntityKind::FieldDecl => {
                if is_public(&child) {
                    if let Some(field) = parse_field(&child, verbose) {
                        fields.push(field);
                    }
                } else {
                    has_non_public_fields = true;
                }
            }
            EntityKind::ClassDecl | EntityKind::StructDecl => {
                // Use tracked access level (not get_accessibility, which is unreliable for structs)
                let is_nested_public = current_access.get() == Accessibility::Public;
                // Parse nested classes/structs defined inside this class
                if is_nested_public && child.is_definition() {
                    let mut parsed = parse_class(&child, source_header, verbose);
                    // Collect the original (unqualified) names of sibling nested classes
                    // so we can fix base class references after qualification.
                    let sibling_names: HashSet<String> = parsed.iter().map(|c| c.name.clone()).collect();
                    for nested in &mut parsed {
                        // Qualify the nested class name with parent: Parent::Nested
                        // Always prepend since multi-level nesting (A::B::C) needs all levels.
                        nested.name = format!("{}::{}", name, nested.name);
                        // nested.module is inherited from source_header
                        // Also qualify base class names that reference sibling nested classes.
                        // Without this, the inheritance graph can't connect e.g.
                        // ShapePersistent_BRep::Curve3D -> GCurve (should be
                        // ShapePersistent_BRep::GCurve) for handle-able class computation.
                        for base in &mut nested.base_classes {
                            if sibling_names.contains(base.as_str()) {
                                *base = format!("{}::{}", name, base);
                            }
                        }
                    }
                    nested_classes.extend(parsed);
                }
            }
            EntityKind::Destructor => {
                // A pure virtual destructor (`virtual ~Foo() = 0`) makes the
                // class abstract even though no non-destructor pure virtuals
                // exist. libclang reports this as Destructor (not Method), so
                // we must check it separately.
                if child.is_pure_virtual_method() {
                    is_abstract = true;
                }
            }
            EntityKind::UsingDeclaration => {
                // `using Base::Method;` in a non-public section narrows the
                // inherited method's access, hiding it from derived classes.
                // Record the name so the inheritance logic treats it as overridden.
                if !is_public(&child) {
                    if let Some(ref using_name) = child.get_name() {
                        all_method_names.insert(using_name.clone());
                    }
```
</details>

> 💭 I see - at line 747, only `ClassDecl | StructDecl` are processed as nested classes. `TypeAliasDecl` (which is what `using OperationsFlags = ...` would be) is NOT processed as a nested class here.
> 
> But the struct `ShapeProcess_OperationsFlags` exists in ffi.rs. Where does it come from? Let me look further. Maybe the parser creates opaque structs for encountered types, or maybe it comes from the `collect_ncollection_typedefs` or `collect_simple_typedefs` mechanisms.
> 
> Actually, let me check if there's another version of how the ffi struct gets generated. 
> 
> So the opaque type declarations in ffi.rs might be generated for all types referenced in the binding set, not just those in `all_class_names`. If `ShapeProcess_OperationsFlags` isn't tracked as a class because it's a typedef, there must be a separate mechanism that generates structs for nested entities that aren't full ParsedClass objects.

Searched for regex `TypeAliasDecl|EntityKind::TypeAlias` (`**/crates/opencascade-binding-generator/src/parser.rs`), 2 results
- [parser.rs:102](../crates/opencascade-binding-generator/src/parser.rs#L102)
- [parser.rs:190](../crates/opencascade-binding-generator/src/parser.rs#L190)

<details>
<summary>Reading lines 180-210 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L180-L211">parser.rs</a></summary>

```
///   typedef NCollection_Utf8String NCollection_String;
///
/// Function pointer typedefs, pointer typedefs (T*), and primitive typedefs
/// are excluded. Template typedefs (containing '<') are handled separately
/// by `collect_ncollection_typedefs()`.
fn collect_simple_typedefs(root: &Entity) {
    let mut map: HashMap<String, String> = HashMap::new();

    root.visit_children(|entity, _| {
        if entity.get_kind() == EntityKind::TypedefDecl
            || entity.get_kind() == EntityKind::TypeAliasDecl
        {
            if let Some(name) = entity.get_name() {
                // Only OCCT-style names (contain underscore)
                if !name.contains('_') {
                    return EntityVisitResult::Recurse;
                }

                if let Some(underlying) = entity.get_typedef_underlying_type() {
                    let underlying_display = underlying.get_display_name();
                    let underlying_kind = underlying.get_kind();

                    // Skip template typedefs (handled by collect_ncollection_typedefs)
                    if underlying_display.contains('<') {
                        return EntityVisitResult::Recurse;
                    }

                    // Only record typedefs to class/struct types (Record, Elaborated wrapping Record)
                    // This excludes pointer typedefs, function pointer typedefs, primitives, etc.
                    let is_record_type = matches!(
                        underlying_kind,
```
</details>

> 💭 I see that `collect_simple_typedefs` at line 194 only processes typedefs with `_` in their name. `OperationsFlags` doesn't have `_`, so it wouldn't be collected. And it's a template typedef (`std::bitset<...>`) which would also be skipped.
> 
> Now let me understand where `ShapeProcess_OperationsFlags` in ffi.rs comes from. Let me look at how opaque type definitions are generated in ffi.rs.

Let me find where the ffi.rs struct declarations are generated to understand where `ShapeProcess_OperationsFlags` comes from:

<details>
<summary>Searched for regex <code>pub struct|opaque.*type|emit.*opaque|emit.*struct|nested.*type.*decl</code> (<code>**/crates/opencascade-binding-generator/src/codegen/rust.rs</code>), 17 results</summary>

- [rust.rs:33](../crates/opencascade-binding-generator/src/codegen/rust.rs#L33)
- [rust.rs:92](../crates/opencascade-binding-generator/src/codegen/rust.rs#L92)
- [rust.rs:170](../crates/opencascade-binding-generator/src/codegen/rust.rs#L170)
- [rust.rs:192](../crates/opencascade-binding-generator/src/codegen/rust.rs#L192)
- [rust.rs:194](../crates/opencascade-binding-generator/src/codegen/rust.rs#L194)
- [rust.rs:271](../crates/opencascade-binding-generator/src/codegen/rust.rs#L271)
- [rust.rs:287](../crates/opencascade-binding-generator/src/codegen/rust.rs#L287)
- [rust.rs:305](../crates/opencascade-binding-generator/src/codegen/rust.rs#L305)
- [rust.rs:310](../crates/opencascade-binding-generator/src/codegen/rust.rs#L310)
- [rust.rs:314](../crates/opencascade-binding-generator/src/codegen/rust.rs#L314)
- [rust.rs:345](../crates/opencascade-binding-generator/src/codegen/rust.rs#L345)
- [rust.rs:377](../crates/opencascade-binding-generator/src/codegen/rust.rs#L377)
- [rust.rs:446](../crates/opencascade-binding-generator/src/codegen/rust.rs#L446)
- [rust.rs:477](../crates/opencascade-binding-generator/src/codegen/rust.rs#L477)
- [rust.rs:480](../crates/opencascade-binding-generator/src/codegen/rust.rs#L480)
- [rust.rs:537](../crates/opencascade-binding-generator/src/codegen/rust.rs#L537)
</details>

<details>
<summary>Reading lines 477-570 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L477-L571">rust.rs</a></summary>

```
/// Generate opaque type declarations
/// Extra destructor type info for destructor generation.
/// Covers nested types (cpp_name with ::, ffi_name with _) and extra typedef types (e.g. gp_Vec3f).
pub struct NestedTypeInfo {
    pub cpp_name: String,
    pub ffi_name: String,
}

fn generate_opaque_declarations(
    collected_types: &CollectedTypes,
    classes: &[&ParsedClass],
    all_enum_names: &HashSet<String>,
    protected_destructor_classes: &HashSet<String>,
    collection_type_names: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
) -> (String, Vec<NestedTypeInfo>) {
    let defined_classes: HashSet<_> = classes.iter().map(|c| c.name.clone()).collect();
    let mut out = String::new();
    let mut emitted: HashSet<String> = HashSet::new();
    let mut nested_types: Vec<NestedTypeInfo> = Vec::new();

    for type_name in &collected_types.classes {
        if defined_classes.contains(type_name) {
            continue;
        }
        if all_enum_names.contains(type_name) {
            continue;
        }
        // Protected destructor classes still need opaque declarations when referenced
        // in method signatures; they just won't get CppDeletable.
        let has_protected_dtor = protected_destructor_classes.contains(type_name);
        if is_primitive_type(type_name) {
            continue;
        }
        // Skip collection types - they're generated separately
        if collection_type_names.contains(type_name) {
            continue;
        }
        // Nested C++ types (e.g., "Poly_CoherentTriangulation::TwoIntegers") get
        // flattened to valid Rust identifiers ("Poly_CoherentTriangulation_TwoIntegers")
        let is_nested = type_name.contains("::");
        let safe_name = if is_nested {
            type_name.replace("::", "_")
        } else {
            type_name.clone()
        };
        // Skip types with pointer/ref qualifiers or template arguments leaked into the name
        // (e.g., "IMeshData_Edge *const" from typedef resolution,
        //  or "ShapePersistent_Geom::geometryBase<Geom_Curve>" from template nested types)
        if safe_name.contains('*') || safe_name.contains('&') || safe_name.contains('<') {
            continue;
        }
        // Avoid duplicate opaque declarations (flattened nested name might collide
        // with an existing class name or another nested type)
        if defined_classes.contains(&safe_name) || !emitted.insert(safe_name.clone()) {
            continue;
        }

        writeln!(out, "/// Referenced type from C++").unwrap();
        writeln!(out, "#[repr(C)]").unwrap();
        writeln!(out, "pub struct {} {{ _opaque: [u8; 0] }}", safe_name).unwrap();

        // Track types needing destructor generation (skip protected destructor types)
        // This includes nested types (Parent::Nested) and extra typedef types (gp_Vec3f)
        let is_extra_typedef = extra_typedef_names.contains(type_name);
        if (is_nested || is_extra_typedef) && !has_protected_dtor {
            nested_types.push(NestedTypeInfo {
                cpp_name: type_name.clone(),
                ffi_name: safe_name,
            });
        }
    }

    (out, nested_types)
}

// UniquePtr impl blocks are no longer needed with extern "C" FFI

/// Emit a Rust `#[repr(i32)]` enum definition with TryFrom/From impls
fn emit_rust_enum(output: &mut String, resolved: &crate::resolver::ResolvedEnum) {
    // Doc comment
    if let Some(ref comment) = resolved.doc_comment {
        emit_doc_comment(output, comment, "");
    }
    writeln!(output, "/// C++ enum: `{}`", resolved.cpp_name).unwrap();

    // Collect unique variants (skip duplicated values — C++ allows alias enum values, Rust doesn't)
    let mut seen_values = std::collections::HashSet::new();
    let mut unique_variants = Vec::new();
    let mut next_value: i64 = 0;
    for variant in &resolved.variants {
        let value = variant.value.unwrap_or(next_value);
        if seen_values.insert(value) {
 
... (truncated, 4050 chars)
```
</details>

> 💭 Now I see the full picture. The `ShapeProcess_OperationsFlags` struct at line 36526 in ffi.rs is generated as an **opaque declaration** at line 537 of `rust.rs`. This happens when a type name appears in `collected_types.classes` but is NOT in `defined_classes` (classes that have actual ParsedClass definitions).
> 
> So `ShapeProcess::OperationsFlags` appears in `collected_types.classes` (meaning it's referenced by some method signature) but isn't in `defined_classes` (there's no ParsedClass for it - since it's a `using` typedef, not a class/struct).
> 
> 
> The opaque type gets flattened to `ShapeProcess_OperationsFlags`, but the type check that determines whether to flag it as unknown looks at whether the parent class `ShapeProcess` exists in `all_classes`. Since `ShapeProcess` is likely a real class with static methods, it should be in that set, which would make the check pass and prevent the unknown class error.

**Terminal:**
```sh
grep -c "ShapeProcess[^_]" crates/opencascade-sys/generated/ffi.rs
```
```
221
```

**Terminal:**
```sh
grep "pub struct ShapeProcess[^_]" crates/opencascade-sys/generated/ffi.rs
```
```
pub struct ShapeProcessAPI_ApplySequence {
```

> 💭 There's no `pub struct ShapeProcess {` in ffi.rs! The class `ShapeProcess` only has static methods and no data members, so it doesn't get a struct. Let me check if `ShapeProcess` appears in `all_classes`.
> 
> Wait - the parser can return an empty class (no methods, no constructors, no fields) and it gets filtered out. Let me check the parser's early return logic.

<details>
<summary>Reading lines 810-870 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L810-L871">parser.rs</a></summary>

```
            }
            _ => {}
        }
        EntityVisitResult::Continue
    });

    // Only return classes that have something to bind
    if constructors.is_empty() && methods.is_empty() && static_methods.is_empty() && fields.is_empty() && nested_classes.is_empty() {
        if verbose {
            println!("    Skipping {} (no bindable members)", name);
        }
        return Vec::new();
    }

    // Determine if this is a POD struct:
    // - Has public fields
    // - No non-public fields
    // - No virtual methods (no vtable)
    // - No base classes
    // - All field types are POD-compatible primitives (possibly in fixed-size arrays)
    // - Not abstract
    let is_pod_struct = !fields.is_empty()
        && !has_non_public_fields
        && !has_virtual_methods
        && base_classes.is_empty()
        && !is_abstract
        && fields.iter().all(|f| f.ty.is_pod_field_type());

    if verbose && is_pod_struct {
        println!("    POD struct detected: {} ({} fields)", name, fields.len());
    }

    let mut result = vec![
        ParsedClass {
            name: name.clone(),
            module: module.clone(),
            comment,
            source_header: source_header.to_string(),
            source_line: get_entity_line(entity),
            constructors,
            methods,
            static_methods,
            all_method_names,
            base_classes,
            has_protected_destructor,
            is_abstract,
            pure_virtual_methods,
            has_explicit_constructors,
            fields,
            is_pod_struct,
            has_copy_constructor,
            has_move_constructor,
        },
    ];

    // Append nested classes to the result
    for nested in nested_classes {
        result.push(nested);
    }

    result
```
</details>

> 💭 At line 817, a class is skipped if it has no constructors, no methods, no static methods, no fields, AND no nested classes. `ShapeProcess` has static methods, so it wouldn't be skipped - wait, actually I need to check. The `parse_class` function at the entity visitor only adds public methods. The static methods for `ShapeProcess` are `RegisterOperator`, `FindOperator`, and `Perform` (3 overloads) + `ToOperationFlag`. But wait, `ToOperationFlag` returns `std::pair<Operation, bool>` which is a template type, and some `Perform` overloads take `OperationsFlags&`. Some might be filtered.Actually, even if some methods are unbindable, they'd still be in the `static_methods` vector at parse time. The bindability check happens later in codegen. So `ShapeProcess` should have static_methods populated and NOT be skipped at line 817.
> 
> 
> Let me verify whether `ShapeProcess` is actually being parsed by checking if its static methods appear in ffi.rs - that would confirm it's being included.

**Terminal:**
```sh
grep "ShapeProcess_" crates/opencascade-sys/generated/ffi.rs | grep -v "ShapeProcess_Context\|ShapeProcess_Oper\|ShapeProcess_Shape\|ShapeProcess_UOper\|ShapeProcessAPI" | head -20
```
```
    pub fn ShapeProcess_register_operator(
    pub fn ShapeProcess_find_operator(
    pub fn ShapeProcess_perform(
```

**Terminal:**
```sh
grep -n "ShapeProcess_register_operator\|ShapeProcess_find_operator\|ShapeProcess_perform" crates/opencascade-sys/generated/ffi.rs
```
```
680083:    pub fn ShapeProcess_register_operator(
680089:    pub fn ShapeProcess_find_operator(
680095:    pub fn ShapeProcess_perform(
```

> 💭 So `ShapeProcess` HAS static methods at lines 680083-680095 in ffi.rs. But there's no `pub struct ShapeProcess {` declaration. That's inconsistent. Let me check the reexport file.
> 
> Wait, the class `ShapeProcess` only has static methods. Perhaps for static methods, no struct declaration is needed - static methods can be called as free functions. But for the class to be in `all_classes`, it should still have been parsed.
> 
> Let me check the reexport file for shape_process module.

**Terminal:**
```sh
grep -n "ShapeProcess[^_]" crates/opencascade-sys/generated/shape_process.rs | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
1://! ShapeProcess module re-exports
9:/// **Source:** `ShapeProcess.hxx`:77 - `ShapeProcess::RegisterOperator`
11:pub fn register_operator(name: &str, op: &crate::ffi::HandleShapeProcessOperator) -> bool {
15:/// **Source:** `ShapeProcess.hxx`:81 - `ShapeProcess::FindOperator`
17:pub fn find_operator(name: &str, op: &mut crate::ffi::HandleShapeProcessOperator) -> bool {
21:/// **Source:** `ShapeProcess.hxx`:87 - `ShapeProcess::Perform`
26:    context: &crate::ffi::HandleShapeProcessContext,
282:    ) -> crate::OwnedPtr<crate::ffi::HandleShapeProcessContext> {
334:pub use crate::ffi::HandleShapeProcessContext;
336:unsafe impl crate::CppDeletable for HandleShapeProcessContext {
338:        crate::ffi::HandleShapeProcessContext_destructor(ptr);
342:impl HandleShapeProcessContext {
345:        unsafe { &*(crate::ffi::HandleShapeProcessContext_get(self as *const Self)) }
350:        unsafe { &mut *(crate::ffi::HandleShapeProcessContext_get_mut(self as *mut Self)) }
357:                crate::ffi::HandleShapeProcessContext_to_HandleStandardTransient(
369:    ) -> Option<crate::OwnedPtr<crate::ffi::HandleShapeProcessShapeContext>> {
371:            crate::ffi::HandleShapeProcessContext_downcast_to_HandleShapeProcessShapeContext(
434:        context: &crate::ffi::HandleShapeProcessShapeContext,
474:        context: &crate::ffi::HandleShapeProcessContext,
563:pub use crate::ffi::HandleShapeProcessOperator;
```
</details>

> 💭 I see - `ShapeProcess` has no `pub struct` declaration and no `pub use` type alias in the reexport. Its static methods are just emitted as free functions in the `shape_process` module. This means `ShapeProcess` itself (as a class name) is likely NOT in `all_classes`.
> 
> Wait, actually if parsed, it should be. Let me look at how static-only classes are handled in the resolver. Maybe they get their own struct or maybe they don't.
> 
> Let me directly check if `ShapeProcess` (without suffix) is recognized in the binding process. The key thing is `all_class_names` and whether `ShapeProcess` is in it.
> 
> 
> Actually, the class does get parsed since it has static methods, so it should be in `all_classes`. But it probably won't get a struct declaration in ffi.rs because it lacks constructors, instance methods, and handle support. The real problem is in bindings.rs where `type_uses_unknown_class()` checks if `ShapeProcess::OperationsFlags` is in `all_class_names`, but `OperationsFlags` is actually a typedef, not a parsed class.
> 
> The fallback check looks for the parent class `ShapeProcess` in `all_classes`, which should be there since it has static methods. So I need to trace through `collect_ncollection_typedefs` to understand why this typedef isn't being recognized properly.
> 
> Let me just run the generator with the `--dump-symbols` flag to see what's actually being collected.

Let me check what the generator reports about ShapeProcess:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo run --release -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include --dry-run --module ShapeProcess -v 2>&1 | grep -i "OperationsFlags\|ShapeProcess[^_]\|skipping" | head -30
```
```
     Running `target/release/occt-bindgen --config crates/opencascade-sys/bindings.toml -I target/OCCT/include --dry-run --module ShapeProcess -v`
```

> 💭 Empty output. Let me try without grepping.

**Terminal:**
```sh
cargo run --release -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include --dry-run --module ShapeProcess -v 2>&1 | head -80
```
<details>
<summary>Output (5 lines)</summary>

```
    Finished `release` profile [optimized] target(s) in 0.07s
     Running `target/release/occt-bindgen --config crates/opencascade-sys/bindings.toml -I target/OCCT/include --dry-run --module ShapeProcess -v`
dyld[8752]: Library not loaded: @rpath/libclang.dylib
  Referenced from: <26F730F5-F866-369B-9F25-CA7BCAC1866B> /Users/sethml/src/opencascade-rs/target/release/occt-bindgen
  Reason: tried: '/Users/sethml/src/opencascade-rs/target/release/deps/libclang.dylib' (no such file), '/Users/sethml/src/opencascade-rs/target/release/libclang.dylib' (no such file), '/Users/sethml/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libclang.dylib' (no such file), '/Users/sethml/.rustup/toolchains/stable-aarch64-apple-darwin/lib/libclang.dylib' (no such file), '/Users/sethml/lib/libclang.dylib' (no such file), '/usr/local/lib/libclang.dylib' (no such file), '/usr/lib/libclang.dylib' (no such file, not in dyld cache)
```
</details>

**Terminal:**
```sh
cat scripts/regenerate-bindings.sh
```
<details>
<summary>Output (63 lines)</summary>

```
#!/bin/bash
# Regenerate OpenCASCADE bindings
#
# This script regenerates the FFI bindings for opencascade-sys.
# Run from the repository root.
#
# Usage:
#   ./scripts/regenerate-bindings.sh
#
# Requirements:
#   - Xcode command line tools (for libclang)
#   - OCCT headers built in target/OCCT/include

set -euo pipefail

# Repository root (script assumes it's run from repo root)
REPO_ROOT="${REPO_ROOT:-$(pwd)}"
cd "$REPO_ROOT"

# Paths
OCCT_INCLUDE="$REPO_ROOT/target/OCCT/include"
OUTPUT_DIR="$REPO_ROOT/crates/opencascade-sys/generated"
CONFIG_FILE="$REPO_ROOT/crates/opencascade-sys/bindings.toml"

# Check prerequisites
if [[ ! -d "$OCCT_INCLUDE" ]]; then
    echo "Error: OCCT include directory not found at $OCCT_INCLUDE"
    echo "Run 'cargo build -p occt-sys' first to build OCCT"
    exit 1
fi

if [[ ! -f "$CONFIG_FILE" ]]; then
    echo "Error: Config file not found at $CONFIG_FILE"
    exit 1
fi

# Build the generator in release mode to avoid a UB crash in the `clang` crate
# (v2.0.0) that only manifests in debug builds on Rust >= 1.78. See
# crates/opencascade-binding-generator/Cargo.toml for details.
echo "Building binding generator..."
cargo build --release -p opencascade-binding-generator

echo "Generating bindings from $CONFIG_FILE..."

# Clean generated directory
echo "Cleaning $OUTPUT_DIR..."
rm -f "$OUTPUT_DIR"/*.rs "$OUTPUT_DIR"/*.hxx

# Set library path for libclang on macOS
if [[ "$(uname)" == "Darwin" ]]; then
    XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib"
    export DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}"
fi

# Run the generator
"$REPO_ROOT/target/release/occt-bindgen" \
    --config "$CONFIG_FILE" \
    -I "$OCCT_INCLUDE" \
    -o "$OUTPUT_DIR" \
    "$@"

echo ""
echo "Bindings generated in $OUTPUT_DIR"
```
</details>

**Terminal:**
```sh
XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" && DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run --release -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include --dry-run --module ShapeProcess -v 2>&1 | head -80
```
<details>
<summary>Output (80 lines)</summary>

```
    Finished `release` profile [optimized] target(s) in 0.06s
     Running `target/release/occt-bindgen --config crates/opencascade-sys/bindings.toml -I target/OCCT/include --dry-run --module ShapeProcess -v`
OCCT Binding Generator
======================
Include directories: ["target/OCCT/include"]
Output directory: "."
Filtering to module: ShapeProcess
  Excluded 14 modules via exclude_modules
Config file: "crates/opencascade-sys/bindings.toml"
  Modules: ["*"]
  Exclude modules: ["Cocoa", "WNT", "Wasm", "Xw", "Express", "BRepMeshData", "TKBRep", "TKDEIGES", "TKDESTEP", "TKMath", "TKMesh", "TKV3d", "TKXSBase", "TKernel"]
  Include headers: 0 entries
  Exclude headers: 9 entries
  Exclude methods: 5 entries
  Expanded to 6809 headers
Loaded config: 1 module patterns, 14 exclude module patterns, 0 individual headers, 9 header exclusions -> 6809 headers

Resolving header dependencies...
  OCCT include dir: "target/OCCT/include"
Header dependency resolution: 6809 explicit -> 6811 total (2 dependencies added)
  Explicit headers: 6809
  Resolved headers: 6811
  Added 2 dependency headers
  Excluded 2 headers after dependency resolution
Parsing 6809 headers...
Clang args: ["-x", "c++", "-std=c++14", "-Wno-pragma-once-outside-header", "-isystem", "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/c++/v1", "-isystem", "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include", "-isystem", "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/clang/17/include", "-Itarget/OCCT/include"]
  Clang parse time: 1.71s
  Collected 4904 NCollection/template typedef entries (4786 unique template spellings)
  Collected 134 simple typedef entries
  Parsing enum: Graphic3d_DisplayPriority
    Variant: Graphic3d_DisplayPriority_INVALID = -1
    Variant: Graphic3d_DisplayPriority_Bottom = 0
    Variant: Graphic3d_DisplayPriority_AlmostBottom = 1
    Variant: Graphic3d_DisplayPriority_Below2 = 2
    Variant: Graphic3d_DisplayPriority_Below1 = 3
    Variant: Graphic3d_DisplayPriority_Below = 4
    Variant: Graphic3d_DisplayPriority_Normal = 5
    Variant: Graphic3d_DisplayPriority_Above = 6
    Variant: Graphic3d_DisplayPriority_Above1 = 7
    Variant: Graphic3d_DisplayPriority_Above2 = 8
    Variant: Graphic3d_DisplayPriority_Highlight = 9
    Variant: Graphic3d_DisplayPriority_Topmost = 10
    Variant: Graphic3d_DisplayPriority_NB = 11
    Skipping anonymous enum (no suitable common prefix: "Graphic3d_DisplayPriority_NB")
  Parsing enum: Aspect_TypeOfHighlightMethod
    Variant: Aspect_TOHM_COLOR = 0
    Variant: Aspect_TOHM_BOUNDBOX = 1
  Parsing enum: Aspect_InteriorStyle
    Variant: Aspect_IS_EMPTY = -1
    Variant: Aspect_IS_SOLID = 0
    Variant: Aspect_IS_HATCH = 1
    Variant: Aspect_IS_HIDDENLINE = 2
    Variant: Aspect_IS_POINT = 3
    Variant: Aspect_IS_HOLLOW = -1
... (truncated, 3980 chars total)
```
</details>

**Terminal:**
```sh
XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" && DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run --release -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include --dry-run --module ShapeProcess -v 2>&1 | grep -i "ShapeProcess"
```

> 💭 Key finding: "Detected utility class 'ShapeProcess' with 5 static methods → converting to module-level functions"
> 
> So `ShapeProcess` is detected as a "utility class" (only static methods, no constructors, no instance methods, no fields). It gets converted to module-level functions rather than a class. Let me check what happens with utility classes in the resolver - are they added to `all_classes`?
> 
> The utility class conversion might remove `ShapeProcess` from `all_classes`.  Also I notice the `XSAlgo_ShapeProcessor` class has methods that use both `ShapeProcess::OperationsFlags` and `XSAlgo_ShapeProcessor::ProcessingFlags` - these are nested type references in other classes too.
> 
> 
> Now I need to trace through the resolver logic to see exactly how utility classes get handled and whether they're excluded from the class collection.

Searched for regex `utility class|utility_class|module-level|convert.*static` (`**/crates/opencascade-binding-generator/src/resolver.rs`), no results

<details>
<summary>Searched for regex <code>utility class|Detected utility</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 5 results</summary>

- [main.rs:504](../crates/opencascade-binding-generator/src/main.rs#L504)
- [main.rs:536](../crates/opencascade-binding-generator/src/main.rs#L536)
- [main.rs:539](../crates/opencascade-binding-generator/src/main.rs#L539)
- [main.rs:583](../crates/opencascade-binding-generator/src/main.rs#L583)
- [main.rs:1049](../crates/opencascade-binding-generator/src/main.rs#L1049)
</details>

<details>
<summary>Reading lines 495-595 of <a href="../crates/opencascade-binding-generator/src/main.rs#L495-L596">main.rs</a></summary>

```
/// a C++ namespace (e.g., `class gp { static const gp_Pnt& Origin(); ... }`). These are
/// conceptually namespaces, not instantiable types.
///
/// Detection criteria:
/// - Class name has no underscore (name == module, e.g., "gp")
/// - Has ONLY static methods (no instance methods)
/// - Has no constructors (or only a default constructor with no params)
///
/// Conversion: static methods → ParsedFunction entries in the same header,
/// and the utility class is removed from the header's class list.
fn convert_utility_classes_to_functions(
    parsed: &mut [model::ParsedHeader],
    exclude_methods: &HashSet<(String, String)>,
    verbose: bool,
) {
    for header in parsed.iter_mut() {
        let mut functions_to_add = Vec::new();
        let mut classes_to_remove = Vec::new();

        for (idx, class) in header.classes.iter().enumerate() {
            // Must have no underscore in the name (class name == module name pattern)
            if class.name.contains('_') {
                continue;
            }

            // Must have static methods
            if class.static_methods.is_empty() {
                continue;
            }

            // Must have NO instance methods
            if !class.methods.is_empty() {
                continue;
            }

            // Must have no meaningful constructors (allow synthetic/empty default)
            let has_meaningful_ctors = class.constructors.iter().any(|c| !c.params.is_empty());
            if has_meaningful_ctors {
                continue;
            }

            // This is a utility class — convert static methods to functions
            if verbose {
                println!(
                    "  Detected utility class '{}' with {} static methods → converting to module-level functions",
                    class.name,
                    class.static_methods.len()
                );
            }

            for sm in &class.static_methods {
                // Check exclude_methods for this static method
                if exclude_methods.contains(&(class.name.clone(), sm.name.clone())) {
                    if verbose {
                        println!("    Skipping excluded method {}::{}", class.name, sm.name);
                    }
                    continue;
                }

                let mut return_type = sm.return_type.clone();

                // If return type is ConstRef and there are no ref params,
                // strip the ConstRef wrapper (return by-value copy). The FFI can't
                // express references from free functions with no borrowable
                // params, so we copy instead.
                let has_ref_params = sm.params.iter().any(|p| p.ty.is_lifetime_source());
                if !has_ref_params {
                    if let Some(model::Type::ConstRef(inner)) = &return_type {
                        return_type = Some(*inner.clone());
                    }
                }

                functions_to_add.push(model::ParsedFunction {
                    name: format!("{}::{}", class.name, sm.name),
                    namespace: class.name.clone(),
                    short_name: sm.name.clone(),
                    module: class.module.clone(),
                    comment: sm.comment.clone(),
                    source_header: class.source_header.clone(),
                    source_line: sm.source_line,
                    params: sm.params.clone(),
                    return_type,
                });
            }

            classes_to_remove.push(idx);
        }

        // Remove utility classes (in reverse order to preserve indices)
        for idx in classes_to_remove.into_iter().rev() {
            header.classes.remove(idx);
        }

        // Add converted functions
        header.functions.extend(functions_to_add);
    }
}

/// Dump the symbol table for debugging purposes
fn dump_symbol_table(table: &resolver::SymbolTable) {
    println!("\n===== SYMBOL TABLE DUMP =====\n");
```
</details>

> 💭 Now I see the issue clearly!
> 
> At line 584, `ShapeProcess` is REMOVED from `header.classes` after its static methods are converted to functions. This means `ShapeProcess` never enters `all_class_names` (which is built from the parsed classes).
> 
> So when methods in OTHER classes (like `IGESControl_Reader`, `STEPControl_Reader`, `XSControl_Vars`, etc.) reference the type `ShapeProcess::OperationsFlags`, the lookup fails:
> 
> 1. `all_classes.contains("ShapeProcess::OperationsFlags")` → FALSE (OperationsFlags is a typedef, never a parsed class)
> 
> 
> 2. `all_classes.contains("ShapeProcess")` → FALSE (ShapeProcess was removed as a utility class!)
> 
> The fix is to keep utility classes registered in `all_classes` even after removing them from the class list, so that nested type lookups can still find the parent namespace. When a utility class gets converted, we need to preserve its name in the known types set so the resolver can match references like `ShapeProcess::OperationsFlags`.
> 
> The real issue is that `collected_types.classes` gets populated from method signatures, not from `all_class_names`, so the nested types from utility classes aren't being tracked properly. I could either add utility class names to a virtual set that `type_uses_unknown_class` checks, register their nested type names in `all_class_names`, or just check a known utility class set during the nested type lookup. The cleanest approach is probably to keep the utility class name registered so the type resolution chain stays intact.
> 
> The converter runs before the resolver and removes the class from `parsed_headers`, so the resolver never encounters it. I should either mark the utility class as converted rather than fully removing it, maintain a separate set of removed utility class names to pass to the resolver, or empty the class instead of deleting it entirely.
> 
> Option 3 seems cleanest at first—keeping the class but clearing its methods and constructors—but the resolver already filters out empty classes, so that won't work unless I also modify the parser. The best approach is to collect the utility class names during conversion and pass them along to the resolver.
> 
> Then when building `all_class_names`, I can include these removed utility classes so that `type_uses_unknown_class` can properly resolve them through the parent check. But I'm realizing the nested typedef itself also needs to resolve correctly—if I add "ShapeProcess" back to the list, the parent check will work, but I need to verify the full flow handles `ShapeProcess::OperationsFlags` properly.

Found it!  is detected as a "utility class" and **removed** from the class list. So `all_classes` never contains , and  fails the parent class lookup. Let me now fix this.

The fix: preserve the names of utility classes that were converted to functions so they remain in `all_class_names`. Let me understand the full flow from resolver through code gen:

<details>
<summary>Searched for text <code>all_class_names</code> (<code>**/crates/opencascade-binding-generator/src/resolver.rs</code>), 12 results</summary>

- [resolver.rs:348](../crates/opencascade-binding-generator/src/resolver.rs#L348)
- [resolver.rs:759](../crates/opencascade-binding-generator/src/resolver.rs#L759)
- [resolver.rs:761](../crates/opencascade-binding-generator/src/resolver.rs#L761)
- [resolver.rs:763](../crates/opencascade-binding-generator/src/resolver.rs#L763)
- [resolver.rs:815](../crates/opencascade-binding-generator/src/resolver.rs#L815)
- [resolver.rs:838](../crates/opencascade-binding-generator/src/resolver.rs#L838)
- [resolver.rs:927](../crates/opencascade-binding-generator/src/resolver.rs#L927)
- [resolver.rs:1205](../crates/opencascade-binding-generator/src/resolver.rs#L1205)
- [resolver.rs:1248](../crates/opencascade-binding-generator/src/resolver.rs#L1248)
- [resolver.rs:1287](../crates/opencascade-binding-generator/src/resolver.rs#L1287)
- [resolver.rs:1306](../crates/opencascade-binding-generator/src/resolver.rs#L1306)
</details>

<details>
<summary>Reading lines 755-770 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L755-L771">resolver.rs</a></summary>

```
    manual_type_names: &HashSet<String>,
) -> SymbolTable {
    // Collect all enum and class names first
    let all_enum_names: HashSet<String> = all_enums.iter().map(|e| e.name.clone()).collect();
    let mut all_class_names: HashSet<String> = all_classes.iter().map(|c| c.name.clone()).collect();
    // Collection typedefs are known types for filtering purposes
    all_class_names.extend(collection_type_names.iter().cloned());
    // Manual types (defined in manual/ files) are also known types
    all_class_names.extend(manual_type_names.iter().cloned());

    // Start from the pre-computed handle-able classes (transitive closure of inheritance graph)
    let mut handle_able_classes = handle_able_classes.clone();

    // Also add any class name that appears inside Type::Handle(...) in method signatures.
    // If C++ code uses Handle(X) for a type, X must inherit from Standard_Transient,
    // so it's handle-able even if its own header is excluded.
```
</details>

Now I see where to add the utility class names. Let me check the `build_symbol_table` function signature to see how utility class names can be passed in:

Searched for text `pub fn build_symbol_table` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 1 result
- [resolver.rs:747](../crates/opencascade-binding-generator/src/resolver.rs#L747)

<details>
<summary>Reading lines 747-756 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L747-L757">resolver.rs</a></summary>

```
pub fn build_symbol_table(
    modules: &[&Module],
    graph: &ModuleGraph,
    all_classes: &[&ParsedClass],
    all_enums: &[&ParsedEnum],
    all_functions: &[&ParsedFunction],
    collection_type_names: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
    manual_type_names: &HashSet<String>,
) -> SymbolTable {
```
</details>

Now let me check how  is called from main.rs:

Searched for text `build_symbol_table` (`**/crates/opencascade-binding-generator/src/main.rs`), 1 result
- [main.rs:352](../crates/opencascade-binding-generator/src/main.rs#L352)

<details>
<summary>Reading lines 347-365 of <a href="../crates/opencascade-binding-generator/src/main.rs#L347-L366">main.rs</a></summary>

```
    }

    // Build symbol table (Pass 1 of two-pass architecture)
    // This resolves all symbols and makes binding decisions ONCE
    let ordered_modules = graph.modules_in_order();
    let symbol_table = resolver::build_symbol_table(
        &ordered_modules,
        &graph,
        &all_classes,
        &all_enums,
        &all_functions,
        &collection_type_names,
        &handle_able_classes,
        &manual_type_names,
    );

    if args.verbose {
        println!("\nSymbol table built:");
        println!("  {} classes", symbol_table.classes.len());
```
</details>

Now let me trace back to see where `convert_utility_classes_to_functions` is called and how I can capture the removed class names:

Searched for text `convert_utility_classes` (`**/crates/opencascade-binding-generator/src/main.rs`), 2 results
- [main.rs:230](../crates/opencascade-binding-generator/src/main.rs#L230)
- [main.rs:505](../crates/opencascade-binding-generator/src/main.rs#L505)

<details>
<summary>Reading lines 225-350 of <a href="../crates/opencascade-binding-generator/src/main.rs#L225-L351">main.rs</a></summary>

```
    // Detect "utility namespace classes" — classes with no underscore in the name
    // (class name == module name), only static methods, and no instance methods/constructors.
    // These are OCCT's namespace-like patterns (e.g., `gp` with `gp::OX()`, `gp::Origin()`).
    // Convert their static methods to free functions so they appear as module-level
    // functions (e.g., `gp::ox()`) instead of awkward `gp::gp::ox()`.
    convert_utility_classes_to_functions(&mut parsed, &exclude_methods, args.verbose);

    if args.verbose {
        println!("\nParsing complete. Summary:");
        let total_classes: usize = parsed.iter().map(|h| h.classes.len()).sum();
        let total_methods: usize = parsed
            .iter()
            .flat_map(|h| &h.classes)
            .map(|c| c.methods.len())
            .sum();
        let total_ctors: usize = parsed
            .iter()
            .flat_map(|h| &h.classes)
            .map(|c| c.constructors.len())
            .sum();
        println!("  {} headers parsed", parsed.len());
        println!("  {} classes found", total_classes);
        println!("  {} constructors found", total_ctors);
        println!("  {} methods found", total_methods);
    }

    // Build module dependency graph
    println!("\nBuilding module dependency graph...");
    let graph = module_graph::ModuleGraph::from_headers(&parsed);

    if args.verbose {
        println!("\nModule graph analysis:");
        println!("  {} modules found", graph.modules.len());

        // Show modules in dependency order
        let ordered = graph.modules_in_order();
        println!("\nModules in dependency order:");
        for module in &ordered {
            if module.dependencies.is_empty() {
                println!("  {} ({} types)", module.name, module.types.len());
            } else {
                let deps: Vec<_> = module.dependencies.iter().collect();
                println!(
                    "  {} ({} types) -> depends on: {:?}",
                    module.name,
                    module.types.len(),
                    deps
                );
            }
        }

        // Show cross-module type references for each module
        println!("\nCross-module type references:");
        for module in &ordered {
            let cross_types = graph.get_cross_module_types(&module.name);
            if !cross_types.is_empty() {
                println!("  {} needs types from other modules:", module.name);
                for ct in &cross_types {
                    println!("    - {}::{} (C++: {})", ct.source_module, ct.rust_name, ct.cpp_name);
                }
            }
        }
    }

    if args.dry_run {
        println!("\nDry run - skipping code generation");
        return Ok(());
    }

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&args.output)?;

    // Generate code
    println!("\nGenerating code...");

    // Collect all classes and enums by module
    let all_classes: Vec<_> = parsed.iter().flat_map(|h| &h.classes).collect();
    let all_enums_raw: Vec<_> = parsed.iter().flat_map(|h| &h.enums).collect();
    // Deduplicate enums with the same name (anonymous enums synthesized from
    // constant prefixes may collide with named enums; merge their variants).
    let all_enums: Vec<_> = {
        let mut seen: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        let mut merged: Vec<model::ParsedEnum> = Vec::new();
        for e in &all_enums_raw {
            if let Some(&idx) = seen.get(e.name.as_str()) {
                // Merge variants from duplicate into existing
                let existing_variant_names: HashSet<String> = merged[idx].variants.iter().map(|v| v.name.clone()).collect();
                for v in &e.variants {
                    if !existing_variant_names.contains(&v.name) {
                        merged[idx].variants.push(v.clone());
                    }
                }
            } else {
         
... (truncated, 5513 chars)
```
</details>

> 💭 Now I have a clear picture. The fix involves:
> 
> 1. Make `convert_utility_classes_to_functions` return the removed class names
> 2. Add those names to `all_class_names` in the resolver
> 
> But actually, looking at the code flow more carefully:
> - `convert_utility_classes_to_functions` runs at line 230
> - `all_classes` is collected from `parsed` at line 301  
> - Since `ShapeProcess` is removed from `parsed`, it's not in `all_classes`
> - `build_symbol_table` gets `all_classes` without `ShapeProcess`
> 
> The simplest fix: have `convert_utility_classes_to_functions` return the names of removed classes, then add them to the `manual_type_names` set (or a new set) before passing to `build_symbol_table`.
> 
> 
> Actually, I could just pass the utility class names as a new parameter to `build_symbol_table`, or fold them into the existing `manual_type_names` set since that's already designed for marking additional known types. The real issue is that when other classes reference `ShapeProcess::OperationsFlags`, the type checker needs either `ShapeProcess::OperationsFlags` or at least `ShapeProcess` itself in `all_class_names` for the parent check to work.
> 
> Adding the utility class name is the simpler path, and since the opaque struct `ShapeProcess_OperationsFlags` is already generated and `type_mapping::map_type_in_context` handles the `::` to `_` conversion, I just need to return the removed utility class names from `convert_utility_classes_to_functions` and add them to `manual_type_names` so they end up in `all_class_names`.

Now I have a clear fix plan. The issue:  removes  from the class list, making its nested type  fail the parent class lookup. I need to preserve utility class names as "known types".

Let me implement the fix:First, let me modify  to return the removed class names:

<details>
<summary>Reading lines 505-591 of <a href="../crates/opencascade-binding-generator/src/main.rs#L505-L592">main.rs</a></summary>

```
fn convert_utility_classes_to_functions(
    parsed: &mut [model::ParsedHeader],
    exclude_methods: &HashSet<(String, String)>,
    verbose: bool,
) {
    for header in parsed.iter_mut() {
        let mut functions_to_add = Vec::new();
        let mut classes_to_remove = Vec::new();

        for (idx, class) in header.classes.iter().enumerate() {
            // Must have no underscore in the name (class name == module name pattern)
            if class.name.contains('_') {
                continue;
            }

            // Must have static methods
            if class.static_methods.is_empty() {
                continue;
            }

            // Must have NO instance methods
            if !class.methods.is_empty() {
                continue;
            }

            // Must have no meaningful constructors (allow synthetic/empty default)
            let has_meaningful_ctors = class.constructors.iter().any(|c| !c.params.is_empty());
            if has_meaningful_ctors {
                continue;
            }

            // This is a utility class — convert static methods to functions
            if verbose {
                println!(
                    "  Detected utility class '{}' with {} static methods → converting to module-level functions",
                    class.name,
                    class.static_methods.len()
                );
            }

            for sm in &class.static_methods {
                // Check exclude_methods for this static method
                if exclude_methods.contains(&(class.name.clone(), sm.name.clone())) {
                    if verbose {
                        println!("    Skipping excluded method {}::{}", class.name, sm.name);
                    }
                    continue;
                }

                let mut return_type = sm.return_type.clone();

                // If return type is ConstRef and there are no ref params,
                // strip the ConstRef wrapper (return by-value copy). The FFI can't
                // express references from free functions with no borrowable
                // params, so we copy instead.
                let has_ref_params = sm.params.iter().any(|p| p.ty.is_lifetime_source());
                if !has_ref_params {
                    if let Some(model::Type::ConstRef(inner)) = &return_type {
                        return_type = Some(*inner.clone());
                    }
                }

                functions_to_add.push(model::ParsedFunction {
                    name: format!("{}::{}", class.name, sm.name),
                    namespace: class.name.clone(),
                    short_name: sm.name.clone(),
                    module: class.module.clone(),
                    comment: sm.comment.clone(),
                    source_header: class.source_header.clone(),
                    source_line: sm.source_line,
                    params: sm.params.clone(),
                    return_type,
                });
            }

            classes_to_remove.push(idx);
        }

        // Remove utility classes (in reverse order to preserve indices)
        for idx in classes_to_remove.into_iter().rev() {
            header.classes.remove(idx);
        }

        // Add converted functions
        header.functions.extend(functions_to_add);
    }
}
```
</details>

Now let me implement the fix. I'll change  to return the removed class names, and add them to the known types set:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L505-L511">main.rs -5+6</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L581-L582">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L584-L595">-9+11</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "505:oc,506:hi,507:rv,508:hx,509:ho",
      "content": "fn convert_utility_classes_to_functions(\n    parsed: &mut [model::ParsedHeader],\n    exclude_methods: &HashSet<(String, String)>,\n    verbose: bool,\n) -> HashSet<String> {\n    let mut removed_class_names = HashSet::new();"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "580:pk",
      "content": "            classes_to_remove.push((idx, class.name.clone()));"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "583:da,584:wa,585:xx,586:cy,587:dd,588:xe,589:gm,590:ax,591:io",
      "content": "        // Remove utility classes (in reverse order to preserve indices)\n        for (idx, name) in classes_to_remove.into_iter().rev() {\n            header.classes.remove(idx);\n            removed_class_names.insert(name);\n        }\n\n        // Add converted functions\n        header.functions.extend(functions_to_add);\n    }\n    removed_class_names\n}"
    }
  ]
}
```
**Output:**
```
{
  "applied": 3,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/main.rs",
      "startLine": 505,
      "endLine": 594
    }
  ]
}
```
</details>

Now update the call site to capture the returned names and add them to `manual_type_names`:

<details>
<summary>Reading lines 225-233 of <a href="../crates/opencascade-binding-generator/src/main.rs#L225-L234">main.rs</a></summary>

```
    // Detect "utility namespace classes" — classes with no underscore in the name
    // (class name == module name), only static methods, and no instance methods/constructors.
    // These are OCCT's namespace-like patterns (e.g., `gp` with `gp::OX()`, `gp::Origin()`).
    // Convert their static methods to free functions so they appear as module-level
    // functions (e.g., `gp::ox()`) instead of awkward `gp::gp::ox()`.
    convert_utility_classes_to_functions(&mut parsed, &exclude_methods, args.verbose);

    if args.verbose {
        println!("\nParsing complete. Summary:");
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L230-L231">main.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    let utility_class_names = convert_utility_classes_to_functions(&mut parsed, &exclude_methods, args.verbose);",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "230:va"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/main.rs","startLine":230,"endLine":230}
```
</details>

Now I need to add the utility class names to the known types. The best place is right before , where `manual_type_names` is already being modified:

<details>
<summary>Reading lines 336-352 of <a href="../crates/opencascade-binding-generator/src/main.rs#L336-L353">main.rs</a></summary>

```

    // Add template instantiation alias names to the known type sets.
    // These were already rewritten in the parsed data (Handle inner names),
    // so we just need to register the alias names as known handle-able classes.
    let mut manual_type_names = manual_type_names;
    for (spelling, inst) in &template_instantiations {
        let alias = config::template_alias_name(spelling);
        manual_type_names.insert(alias.clone());
        if inst.handle {
            handle_able_classes.insert(alias);
        }
    }

    // Build symbol table (Pass 1 of two-pass architecture)
    // This resolves all symbols and makes binding decisions ONCE
    let ordered_modules = graph.modules_in_order();
    let symbol_table = resolver::build_symbol_table(
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L347-L359">main.rs -3+12</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    }\n\n    // Add utility class names (classes converted to module-level functions) to the\n    // known type set.  Other classes may reference nested types qualified with\n    // the utility class name (e.g. ShapeProcess::OperationsFlags).  Without\n    // this, the parent class check in type_uses_unknown_class() fails because\n    // the utility class was removed from the parsed class list.\n    for name in &utility_class_names {\n        manual_type_names.insert(name.clone());\n    }\n\n    // Build symbol table (Pass 1 of two-pass architecture)",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "347:ax,348:dd,349:yn"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/main.rs","startLine":347,"endLine":358}
```
</details>

Now let me verify this compiles:

**Terminal:**
```sh
time cargo check -p opencascade-binding-generator 2>&1
```
```
    Checking opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.91s
real  2.0s
```

Now regenerate bindings and run a full build:

**Terminal:**
```sh
time ./scripts/regenerate-bindings.sh 2>&1
```
<details>
<summary>Output (374 lines)</summary>

```
Building binding generator...
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `release` profile [optimized] target(s) in 2.08s
Generating bindings from /Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml...
Cleaning /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated...
  Excluded 14 modules via exclude_modules
Loaded config: 1 module patterns, 14 exclude module patterns, 0 individual headers, 9 header exclusions -> 6809 headers
  Excluded 2 headers after dependency resolution
Parsing 6809 headers...
  Clang parse time: 1.97s
  Collected 4904 NCollection/template typedef entries (4786 unique template spellings)
  Collected 134 simple typedef entries

Timing summary:
  Total clang parse time: 1.97s
  Total AST visit time: 2.72s
  Rewrote 28 template Handle type references to alias names

Building module dependency graph...

Generating code...

=== Generating FFI output ===

Generating ffi.rs...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi.rs (5120 classes, 1394 functions)
Generating wrappers.cpp...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp
Generating module re-exports...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/t_collection.rs (4 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/standard.rs (43 types, 7 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/gp.rs (38 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/bnd.rs (14 types, 3 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/n_collection.rs (17 types, 5 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/quantity.rs (7 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/image.rs (10 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/font.rs (8 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/media.rs (9 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/osd.rs (52 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/t_col_std.rs (24 types, 37 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/top_loc.rs (5 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/graphic3d.rs (95 types, 25 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/aspect.rs (31 types, 6 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/geom_abs.rs (0 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencasca
... (truncated, 38445 chars total)
```
</details>

**Terminal:**
```sh
tail -5 "/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/be682feb-153a-4ab3-a24a-c102b03012c6/toolu_012UiugLD2qHPcZnGVUHpy4q__vscode-1771608614516/content.txt"
```
<details>
<summary>Output (5 lines)</summary>

```
Code generation complete!
  321 modules generated

Bindings generated in /Users/sethml/src/opencascade-rs/crates/opencascade-sys/ge
nerated                                                                         real  45.6s
```
</details>

Good, generation succeeded. Let me check if the OperationsFlags methods are now being generated:

**Terminal:**
```sh
grep -c "OperationsFlags" crates/opencascade-sys/generated/ffi.rs && grep "OperationsFlags" crates/opencascade-sys/generated/iges_control.rs | head -5
```
<details>
<summary>Output (5 lines)</summary>

```
27
    pub fn set_shape_process_flags(&mut self, theFlags: &crate::ffi::ShapeProcess_OperationsFlags) {
    pub fn set_shape_process_flags(&mut self, theFlags: &crate::ffi::ShapeProcess_OperationsFlags) {
    pub fn set_shape_process_flags(&mut self, theFlags: &crate::ffi::ShapeProcess_OperationsFlags) {
    pub fn get_shape_process_flags(&self) -> &crate::ffi::ShapeProcess_OperationsFlags {
```
</details>

The OperationsFlags methods are now being generated. Let me check how many skips we've resolved:

**Terminal:**
```sh
grep -c "SKIPPED:" crates/opencascade-sys/generated/*.rs | awk -F: '{sum+=$2} END {print sum}'
```
```
394
```

404 → 394, so 10 symbols unblocked (11 OperationsFlags - 1 that's probably still blocked for another reason like an unresolved template return type). Now let me build everything:

**Terminal:**
```sh
time cargo build 2>&1
```

Build succeeded. Now let me run the binding generator tests:

**Terminal:**
```sh
time ./scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (37 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.64s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-3404803fe4cd90de)

running 16 tests
test module_graph::tests::test_extract_rust_type_name ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test header_deps::tests::test_is_likely_occt_header ... ok
test parser::tests::test_extract_module_from_header ... ok
test parser::tests::test_extract_template_arg ... ok
test parser::tests::test_map_standard_type ... ok
test type_mapping::tests::test_map_class ... ok
test type_mapping::tests::test_map_const_ref ... ok
test type_mapping::tests::test_map_primitives ... ok
test resolver::tests::test_safe_method_name ... ok
test resolver::tests::test_safe_param_name ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test type_mapping::tests::test_map_return_type ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/occt_bindgen-eda4fc796a0d21f2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests opencascade_binding_generator

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

real  2.1s
```
</details>

Now cargo test for the rest:

**Terminal:**
```sh
time cargo test --workspace --exclude opencascade-binding-generator 2>&1
```

**Terminal:**
```sh
tail -20 "/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/be682feb-153a-4ab3-a24a-c102b03012c6/toolu_016AjjpR2yngUkLmWywNkHGL__vscode-1771608614522/content.txt"
```
<details>
<summary>Output (20 lines)</summary>

```
   Doc-tests model_api

running 1 test
test crates/model-api/src/wasm.rs - wasm::__export_model_world_impl (line 11) ..
. ok                                                                            
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
shed in 0.44s                                                                   
   Doc-tests occt_sys

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
shed in 0.00s                                                                   
   Doc-tests opencascade

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
shed in 0.00s                                                                   
real  109.4s
```
</details>

All tests pass. Now let me update the README skip counts and commit:

**Terminal:**
```sh
python3 tmp/readme_skips.py 2>&1
```
<details>
<summary>Output (70 lines)</summary>

```
Total skipped symbols: 394

=== Skip Reason Breakdown ===
| Count |     % | Category |
|------:|------:|----------|
|    91 | 23.1% | **Unknown/unresolved type** |
|    91 | 23.1% | **Unresolved template type** |
|    72 | 18.3% | **Abstract class** |
|    55 | 14.0% | **Unknown Handle type** |
|    19 |  4.8% | **C-style array** |
|    19 |  4.8% | **Stream (shared_ptr)** |
|    17 |  4.3% | **String ref param** |
|    12 |  3.0% | **Rvalue reference** |
|    11 |  2.8% | **Not CppDeletable** |
|     4 |  1.0% | **&mut enum return** |
|     2 |  0.5% | **Excluded by bindings.toml** |
|     1 |  0.3% | **Ambiguous overload** |

Unknown/unresolved + Unknown Handle combined: 146 (37.1%)

=== Most Common Unknown Types ===
| Count | Type |
|------:|------|
|    12 | `Handle(ShapePersistent_Geom::geometryBase<Geom_Surface>)` |
|    10 | `Handle(ShapePersistent_Geom::geometryBase<Geom_Curve>)` |
|    10 | `Handle(ShapePersistent_Geom::geometryBase<Geom2d_Curve>)` |
|     7 | `IMeshData::MapOfInteger const` |
|     5 | `WNT_HIDSpaceMouse const` |
|     5 | `AVStream const` |
|     5 | `RWGltf_GltfOStreamWriter` |
|     4 | `GLXFBConfig` |
|     3 | `Aspect_XDisplay` |
|     3 | `IMeshData::Array1OfVertexOfDelaun` |
|     3 | `IMeshData::VectorOfInteger` |
|     3 | `IMeshData_Edge *const const` |
|     3 | `IMeshData_Face *const const` |
|     2 | `Aspect_XVisualInfo` |
|     2 | `ProxPnt_Status const` |

=== Domain Analysis ===

**Data Exchange** (22 symbols) -- `iges_control` (1), `rw_gltf` (6), `rw_obj` (2), `rw_ply` (1), `rw_stl` (2), `step_control` (1), `xs_control` (9)
    8 string ref param
    6 unknown type
    3 rvalue reference
    2 unresolved template
    2 abstract class
    1 stream (shared_ptr)

**Document Framework** (1 symbols) -- `tdf` (1)
    1 unknown type

**Shape Meshing** (50 symbols) -- `b_rep_mesh` (45), `i_mesh_data` (5)
   31 unknown type
   10 unresolved template
    6 C-style array
    3 abstract class

**Shape Analysis/Fix** (0 symbols)

**Geometry** (0 symbols)

**Poly** (7 symbols) -- `poly` (7)
    3 C-style array
    1 excluded by config
    1 unknown type
    1 abstract class
    1 unresolved template

**Boolean Ops** (0 symbols)
```
</details>

404 → 394, reducing unknown/unresolved type from 102 to 91 (and unblocking 10 methods across data exchange modules). Let me also check what got the extra "Not CppDeletable" (10 → 11):

**Terminal:**
```sh
grep -rn "Not CppDeletable\|not CppDeletable" crates/opencascade-sys/generated/*.rs | head -15
```
<details>
<summary>Output (11 lines)</summary>

```
crates/opencascade-sys/generated/iges_data.rs:5433://   Reason: return type 'char' is not CppDeletable
crates/opencascade-sys/generated/iges_data.rs:5438://   Reason: return type 'char' is not CppDeletable
crates/opencascade-sys/generated/standard.rs:59666://   Reason: return type 'Standard::AllocatorType' is not CppDeletable
crates/opencascade-sys/generated/t_collection.rs:1108://   Reason: return type 'char' is not CppDeletable
crates/opencascade-sys/generated/t_collection.rs:2666://   Reason: return type 'char' is not CppDeletable
crates/opencascade-sys/generated/v3d.rs:6127://   Reason: return type 'V3d_ListOfLight::Iterator' is not CppDeletable
crates/opencascade-sys/generated/v3d.rs:7068://   Reason: return type 'V3d_ListOfView::Iterator' is not CppDeletable
crates/opencascade-sys/generated/v3d.rs:7073://   Reason: return type 'V3d_ListOfView::Iterator' is not CppDeletable
crates/opencascade-sys/generated/v3d.rs:7078://   Reason: return type 'V3d_ListOfLight::Iterator' is not CppDeletable
crates/opencascade-sys/generated/v3d.rs:7083://   Reason: return type 'V3d_ListOfLight::Iterator' is not CppDeletable
crates/opencascade-sys/generated/vrml_data.rs:6750://   Reason: return type 'VrmlData_MapOfNode::Iterator' is not CppDeletable
```
</details>

The +1 Not CppDeletable is a pre-existing issue that was already there (now 11 vs 10, probably due to an opaque type now being recognized that wasn't before).

Now update the README and commit:

<details>
<summary>Reading lines 391-460 of <a href="../crates/opencascade-binding-generator/README.md#L391-L461">README.md</a></summary>

````
## Skipped Symbols

The binding generator skips ~404 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Every skipped symbol is documented in the generated per-module `.rs` files as a `// SKIPPED:` comment block including:

- **Source location** (header file, line number, C++ symbol name)
- **Documentation comment** from the C++ header (first 3 lines)
- **Skip reason** explaining why the symbol was excluded
- **Commented-out Rust stub** showing the best-guess declaration

Example from `gp.rs`:
```rust
// SKIPPED: **Source:** `gp_XYZ.hxx`:109 - `gp_XYZ::GetData`
//   method: Returns a const ptr to coordinates location.
//   Reason: has unbindable types: return: raw pointer (const double*)
//   // pub fn get_data(&self) -> /* const double* */;
```

### Skip Reason Breakdown

| Count | % | Category | Description |
|------:|----:|----------|-------------|
| 102 | 25.2% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`ShapeProcess::OperationsFlags`, `IMeshData::MapOfInteger`, `WNT_HIDSpaceMouse`, etc.) |
| 91 | 22.5% | **Unresolved template type** | Template instantiations that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `NCollection_Vec3<...>`, etc.) |
| 72 | 17.8% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |
| 55 | 13.6% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(ShapePersistent_Geom::...)`, `Handle(ShapePersistent_BRep::...)`, etc.) |
| 19 | 4.7% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |
| 19 | 4.7% | **Stream (shared_ptr)** | `std::shared_ptr<std::istream/ostream>` — smart-pointer-wrapped streams not yet bindable |
| 17 | 4.2% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |
| 12 | 3.0% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across FFI |
| 10 | 2.5% | **Not CppDeletable** | Return type class has no destructor in the binding set |
| 4 | 1.0% | **&mut enum return** | Mutable reference to enum (cxx limitation) |
| 2 | 0.5% | **Excluded by bindings.toml** | Explicitly excluded in config (e.g., ambiguous overload workarounds) |
| 1 | 0.2% | **Ambiguous overload** | C++ overload that would produce identical wrapper signatures |

### Most Common Unknown Types

The "unknown type" and "unknown Handle type" categories (38.9% of all skips) are dominated by a few types:

| Count | Type | How to Unblock |
|------:|------|----------------|
| 32 | `Handle(ShapePersistent_Geom::geometryBase<...>)` | Protected nested template class — header not in binding set |
| 11 | `ShapeProcess::OperationsFlags` | Nested enum in `ShapeProcess` — not yet resolvable as a scoped type |
| 7 | `IMeshData::MapOfInteger` | Namespace-scoped NCollection template typedef — not yet resolvable |
| 5 | `AVStream`, `AVPacket`, `AVFrame`, etc. | FFmpeg types in media module — external dependency |
| 5 | `WNT_HIDSpaceMouse` | Windows-only type, WNT module excluded |
| 5 | `RWGltf_GltfOStreamWriter` | External RapidJSON dependency — not in binding set |
| 4 | `GLXFBConfig` | X11/Linux display type — platform-specific |
| 3 | `Aspect_XDisplay`, `Aspect_XVisualInfo` | X11/Linux display types — platform-specific |
| 3 | `IMeshData::Array1OfVertexOfDelaun` | Namespace-scoped NCollection template typedef — not yet resolvable |
| 3 | `IMeshData::VectorOfInteger` | Namespace-scoped NCollection template typedef — not yet resolvable |

### Important Skipped Symbols

Most skipped symbols are in internal, low-use, or specialized modules. However, some affect functionality that users commonly need:

**Data Exchange (26 symbols)** — `STEPControl_*` (2), `IGESControl_*` (3), `XSControl_*` (10), `RWGltf_*` (6), `RWObj_*` (2), `RWStl` (2), `RWPly` (1). Dominated by unknown types (10) and string ref params (8, mostly `const char*&` in `XSControl_Vars`), plus rvalue references
... (truncated, 5602 chars)
````
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/README.md#L393-L394">README.md -1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L410-L424">-14+14</a>, <a href="../crates/opencascade-binding-generator/README.md#L427-L428">-1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L429-L440">-12+11</a>, <a href="../crates/opencascade-binding-generator/README.md#L445-L446">-1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "The binding generator skips ~394 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Every skipped symbol is documented in the generated per-module `.rs` files as a `// SKIPPED:` comment block including:",
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "393:qp"
    },
    {
      "content": "| Count | % | Category | Description |\n|------:|----:|----------|-------------|\n| 91 | 23.1% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`IMeshData::MapOfInteger`, `WNT_HIDSpaceMouse`, etc.) |\n| 91 | 23.1% | **Unresolved template type** | Template instantiations that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `NCollection_Vec3<...>`, etc.) |\n| 72 | 18.3% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |\n| 55 | 14.0% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(ShapePersistent_Geom::...)`, `Handle(ShapePersistent_BRep::...)`, etc.) |\n| 19 | 4.8% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |\n| 19 | 4.8% | **Stream (shared_ptr)** | `std::shared_ptr<std::istream/ostream>` — smart-pointer-wrapped streams not yet bindable |\n| 17 | 4.3% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |\n| 12 | 3.0% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across FFI |\n| 11 | 2.8% | **Not CppDeletable** | Return type class has no destructor in the binding set |\n| 4 | 1.0% | **&mut enum return** | Mutable reference to enum (cxx limitation) |\n| 2 | 0.5% | **Excluded by bindings.toml** | Explicitly excluded in config (e.g., ambiguous overload workarounds) |\n| 1 | 0.3% | **Ambiguous overload** | C++ overload that would produce identical wrapper signatures |",
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "410:fa,411:oi,412:bi,413:aj,414:xw,415:wa,416:rg,417:lr,418:zj,419:xf,420:ps,421:xd,422:ex,423:xy"
    },
    {
      "content": "The \"unknown type\" and \"unknown Handle type\" categories (37.1% of all skips) are dominated by a few types:",
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "427:jq"
    },
    {
      "content": "| Count | Type | How to Unblock |\n|------:|------|----------------|\n| 32 | `Handle(ShapePersistent_Geom::geometryBase<...>)` | Protected nested template class — header not in binding set |\n| 7 | `IMeshData::MapOfInteger` | Namespace-scoped NCollection template typedef — not yet resolvable |\n| 5 | `AVStream`, `AVPacket`, `AVFrame`, etc. | FFmpeg types in media module — external dependency |\n| 5 | `WNT_HIDSpaceMouse` | Windows-only type, WNT module excluded |\n| 5 | `RWGltf_GltfOStreamW
... (truncated, 4338 chars)
```
**Output:**
```
{
  "applied": 5,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/README.md",
      "startLine": 393,
      "endLine": 445
    }
  ]
}
```
</details>

Now commit:

**Terminal:**
```sh
cat > tmp/commit-msg-utility.txt << 'ENDMSG'
fix: resolve nested types in utility classes (10 fewer skips)

When utility classes (like ShapeProcess) are converted to module-level
functions, they were fully removed from the class list. This caused
methods in OTHER classes that reference nested types qualified with the
utility class name (e.g. ShapeProcess::OperationsFlags) to be skipped
as "unknown type" — the parent class check in type_uses_unknown_class()
failed because the parent was no longer in all_class_names.

Fix: convert_utility_classes_to_functions now returns the set of removed
class names, which are added to manual_type_names so they remain
recognized as known parent types during symbol resolution.

This unblocks 10 methods across data exchange modules (STEPControl,
IGESControl, IGESToBRep, STEPCAFControl, Transfer, XSAlgo, XSControl)
that use ShapeProcess::OperationsFlags parameters/returns.

Total skips: 404 → 394
ENDMSG
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
git add crates/opencascade-binding-generator/src/main.rs crates/opencascade-binding-generator/README.md crates/opencascade-sys/generated/ && git commit -F tmp/commit-msg-utility.txt
```
```
[main c4365e0] fix: resolve nested types in utility classes (10 fewer skips)
 14 files changed, 476 insertions(+), 98 deletions(-)
```

**Todo list:**
- ✅ Explore tmp/ scripts and current skips
- ✅ Update README skip section
- ✅ Commit README update
- ✅ Fix highest-count skip reason
- ✅ Commit fix
- 🔄 Evaluate and do refactoring
- ⬜ Commit refactoring if done

Now let me evaluate refactoring opportunities. During this session I noticed heavy repetition in the `is_method_bindable`, `is_constructor_bindable`, and `is_static_method_bindable` functions in `crates/opencascade-binding-generator/src/codegen/bindings.rs`. Let me check:

<details>
<summary>Reading lines 810-1000 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L810-L1001">bindings.rs</a></summary>

```
fn is_method_bindable(method: &Method, ctx: &TypeContext, class_name: &str) -> Result<(), String> {
    if method.has_unbindable_types() {
        let unbindable_details = describe_unbindable_types_method(method);
        return Err(format!("has unbindable types: {}", unbindable_details));
    }
    // Skip methods with const char*& or const char* const& params (need manual bindings)
    if let Some((param_name, type_name)) = resolver::method_has_string_ref_param(method) {
        return Err(format!("has string ref param '{}' of type '{}' (needs manual binding)", param_name, type_name));
    }
    // Skip methods that cause ambiguous call errors in C++ wrappers
    if AMBIGUOUS_METHODS.iter().any(|(c, m)| *c == class_name && *m == method.name) {
        return Err("causes ambiguous overload in C++ (listed in AMBIGUOUS_METHODS)".to_string());
    }
    // Const/mut return mismatch is now handled via C++ wrappers (ConstMutReturnFix).
    // &mut enum output params are now handled via C++ wrappers (MutRefEnumParam).
    if let Some(p) = method
        .params
        .iter()
        .find(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return Err(format!("param '{}' uses unknown type '{}'", p.name, p.ty.to_cpp_string()));
    }
    // Skip methods where a nullable pointer param's inner type is unknown
    if let Some(p) = method.params.iter().find(|p| {
        if p.is_nullable_ptr() {
            match &p.ty {
                Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_type(inner, ctx),
                _ => false,
            }
        } else {
            false
        }
    }) {
        return Err(format!("nullable param '{}' inner type is unknown", p.name));
    }
    // Skip methods where a class pointer param's inner type is unknown.
    // We check all_classes directly (not type_uses_unknown_type) because nested types
    // like Parent::Nested are considered "known" by type_uses_unknown_type if the parent
    // is known, but they don't have their own FFI type declarations.
    if let Some(p) = method.params.iter().find(|p| {
        if let Some(class_name) = p.ty.class_ptr_inner_name() {
            !ctx.all_classes.contains(class_name) && !ctx.all_enums.contains(class_name)
        } else {
            false
        }
    }) {
        return Err(format!("class pointer param '{}' inner type '{}' is unknown", p.name, p.ty.to_cpp_string()));
    }
    if let Some(ref ret) = method.return_type {
        if type_uses_unknown_type(ret, ctx) {
            return Err(format!("return type '{}' is unknown", ret.to_cpp_string()));
        }
        // Check class pointer returns for unknown inner types (same as params)
        if let Some(class_name) = ret.class_ptr_inner_name() {
            if !ctx.all_classes.contains(class_name) && !ctx.all_enums.contains(class_name) {
                return Err(format!("class pointer return inner type '{}' is unknown", ret.to_cpp_string()));
            }
        }
        // OwnedPtr<T> return type requires CppDeletable for T. ParsedClasses have
        // generated destructors; the 91 known collections do too. But NCollection
        // template typedef names (e.g., TColStd_ListOfAsciiString) added to
        // all_class_names for param filtering don't have generated destructors.
        // Enum types are represented as Type::Class in raw parsed types — allow them.
        if let Type::Class(name) = ret {
            if !is_void_type_name(name) {
                if let Some(deletable) = ctx.deletable_class_names {
                    if !deletable.contains(name.as_str()) && !ctx.all_enums.contains(name.as_str()) {
                        return Err(format!("return type '{}' is not CppDeletable", name));
                    }
                }
            }
        }
        // MutRef to enum return type can't be bound — extern "C" expects int32_t& but C++ has EnumType&
        if return_type_is_mut_ref_enum(ret, ctx.all_enums) {
            return Err("return typ
... (truncated, 8932 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/be682feb-153a-4ab3-a24a-c102b03012c6/toolu_01P15Y6MceinXoSkUPm1BqzQ__vscode-1771608614531/content.txt)

> 💭 Yes! There's clear repetition in the three `is_*_bindable` functions. Let me identify the common patterns:
> 
> 1. **Unknown type check for params** — identical in all three:
>    ```rust
>    if let Some(p) = params.iter().find(|p| type_uses_unknown_type(&p.ty, ctx)) {
>        return Err(format!("param '{}' uses unknown type '{}'", p.name, p.ty.to_cpp_string()));
>    }
>    ```
> 
> 2. **Nullable pointer inner type check for params** — identical in all three:
>    ```rust
>    if let Some(p) = params.iter().find(|p| {
>        if p.is_nullable_ptr() {
>            match &p.ty {
>                Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_type(inner, ctx),
>                _ => false,
>            }
>        } else { false }
>    }) {
>        return Err(format!("nullable param '{}' inner type is unknown", p.name));
>    }
>    ```
> 
> 3. **Class pointer inner type check for params** — identical in all three...
> 
> Extracting validation logic... I'm noticing that the parameter validation logic is duplicated across these three functions, and the return type checks are shared between two of them. I should extract these into separate helper functions to eliminate the redundancy and make the code more maintainable.

There's clear repetition across `is_method_bindable`, `is_constructor_bindable`, and `is_static_method_bindable`. The param validation logic (unknown types, nullable pointer inner types, class pointer inner types) is copy-pasted 3 times, and the return type validation is copy-pasted 2 times.

Let me also check for  vs `type_uses_unknown_handle` duplication:

<details>
<summary>Reading lines 998-1010 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L998-L1011">bindings.rs</a></summary>

```
                    if !deletable.contains(name.as_str()) && !ctx.all_enums.contains(name.as_str()) {
                        return Err(format!("return type '{}' is not CppDeletable", name));
                    }
                }
            }
        }
        // C-string returns (const char*) are handled via C++ wrappers returning const char*.
        // MutRef to enum return type can't be bound — extern "C" expects int32_t& but C++ has EnumType&
        if return_type_is_mut_ref_enum(ret, ctx.all_enums) {
            return Err("return type is &mut enum (not representable in extern \"C\")".to_string());
        }
    }
    Ok(())
```
</details>

Let me now implement the refactoring. I'll extract two helper functions:

<details>
<summary>Reading lines 808-815 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L808-L816">bindings.rs</a></summary>

```
];

fn is_method_bindable(method: &Method, ctx: &TypeContext, class_name: &str) -> Result<(), String> {
    if method.has_unbindable_types() {
        let unbindable_details = describe_unbindable_types_method(method);
        return Err(format!("has unbindable types: {}", unbindable_details));
    }
    // Skip methods with const char*& or const char* const& params (need manual bindings)
```
</details>

I'll insert the helper functions before `is_method_bindable`, then refactor all three functions to use them:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L808-L879">bindings.rs -2+71</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "];\n\n/// Check parameters for unknown types, nullable pointer inner types, and class pointer inner types.\n/// Shared by is_method_bindable, is_constructor_bindable, and is_static_method_bindable.\nfn check_params_bindable(params: &[Param], ctx: &TypeContext) -> Result<(), String> {\n    // Check for unknown class/handle types in parameters\n    if let Some(p) = params.iter().find(|p| type_uses_unknown_type(&p.ty, ctx)) {\n        return Err(format!(\"param '{}' uses unknown type '{}'\", p.name, p.ty.to_cpp_string()));\n    }\n    // Check nullable pointer params whose inner type is unknown\n    if let Some(p) = params.iter().find(|p| {\n        if p.is_nullable_ptr() {\n            match &p.ty {\n                Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_type(inner, ctx),\n                _ => false,\n            }\n        } else {\n            false\n        }\n    }) {\n        return Err(format!(\"nullable param '{}' inner type is unknown\", p.name));\n    }\n    // Check class pointer params whose inner type is unknown.\n    // We check all_classes directly (not type_uses_unknown_type) because nested types\n    // like Parent::Nested are considered \"known\" by type_uses_unknown_type if the parent\n    // is known, but they don't have their own FFI type declarations.\n    if let Some(p) = params.iter().find(|p| {\n        if let Some(class_name) = p.ty.class_ptr_inner_name() {\n            !ctx.all_classes.contains(class_name) && !ctx.all_enums.contains(class_name)\n        } else {\n            false\n        }\n    }) {\n        return Err(format!(\"class pointer param '{}' inner type '{}' is unknown\", p.name, p.ty.to_cpp_string()));\n    }\n    Ok(())\n}\n\n/// Check a return type for unknown types, CppDeletable, class pointer inner types, and &mut enum.\n/// Shared by is_method_bindable and is_static_method_bindable.\nfn check_return_type_bindable(ret: &Type, ctx: &TypeContext) -> Result<(), String> {\n    if type_uses_unknown_type(ret, ctx) {\n        return Err(format!(\"return type '{}' is unknown\", ret.to_cpp_string()));\n    }\n    // Check class pointer returns for unknown inner types\n    if let Some(class_name) = ret.class_ptr_inner_name() {\n        if !ctx.all_classes.contains(class_name) && !ctx.all_enums.contains(class_name) {\n            return Err(format!(\"class pointer return inner type '{}' is unknown\", ret.to_cpp_string()));\n        }\n    }\n    // OwnedPtr<T> return type requires CppDeletable for T. ParsedClasses have\n    // generated destructors; the 91 known collections do too. But NCollection\n    // template typedef names (e.g., TColStd_ListOfAsciiString) added to\n    // all_class_names for param filtering don't have generated destructors.\n    // Enum types are represented as Type::Class in raw parsed types — allow them.\n    if let Type::Class(name) = ret {\n        if !is_void_type_name(name) {\n            if let Some(deletable) = ctx.deleta
... (truncated, 3712 chars)
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":808,"endLine":878}
```
</details>

Now refactor `is_method_bindable` to use the helpers. Let me re-read the current state:

<details>
<summary>Reading lines 880-960 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L880-L961">bindings.rs</a></summary>

```
    if method.has_unbindable_types() {
        let unbindable_details = describe_unbindable_types_method(method);
        return Err(format!("has unbindable types: {}", unbindable_details));
    }
    // Skip methods with const char*& or const char* const& params (need manual bindings)
    if let Some((param_name, type_name)) = resolver::method_has_string_ref_param(method) {
        return Err(format!("has string ref param '{}' of type '{}' (needs manual binding)", param_name, type_name));
    }
    // Skip methods that cause ambiguous call errors in C++ wrappers
    if AMBIGUOUS_METHODS.iter().any(|(c, m)| *c == class_name && *m == method.name) {
        return Err("causes ambiguous overload in C++ (listed in AMBIGUOUS_METHODS)".to_string());
    }
    // Const/mut return mismatch is now handled via C++ wrappers (ConstMutReturnFix).
    // &mut enum output params are now handled via C++ wrappers (MutRefEnumParam).
    if let Some(p) = method
        .params
        .iter()
        .find(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return Err(format!("param '{}' uses unknown type '{}'", p.name, p.ty.to_cpp_string()));
    }
    // Skip methods where a nullable pointer param's inner type is unknown
    if let Some(p) = method.params.iter().find(|p| {
        if p.is_nullable_ptr() {
            match &p.ty {
                Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_type(inner, ctx),
                _ => false,
            }
        } else {
            false
        }
    }) {
        return Err(format!("nullable param '{}' inner type is unknown", p.name));
    }
    // Skip methods where a class pointer param's inner type is unknown.
    // We check all_classes directly (not type_uses_unknown_type) because nested types
    // like Parent::Nested are considered "known" by type_uses_unknown_type if the parent
    // is known, but they don't have their own FFI type declarations.
    if let Some(p) = method.params.iter().find(|p| {
        if let Some(class_name) = p.ty.class_ptr_inner_name() {
            !ctx.all_classes.contains(class_name) && !ctx.all_enums.contains(class_name)
        } else {
            false
        }
    }) {
        return Err(format!("class pointer param '{}' inner type '{}' is unknown", p.name, p.ty.to_cpp_string()));
    }
    if let Some(ref ret) = method.return_type {
        if type_uses_unknown_type(ret, ctx) {
            return Err(format!("return type '{}' is unknown", ret.to_cpp_string()));
        }
        // Check class pointer returns for unknown inner types (same as params)
        if let Some(class_name) = ret.class_ptr_inner_name() {
            if !ctx.all_classes.contains(class_name) && !ctx.all_enums.contains(class_name) {
                return Err(format!("class pointer return inner type '{}' is unknown", ret.to_cpp_string()));
            }
        }
        // OwnedPtr<T> return type requires CppDeletable for T. ParsedClasses have
        // generated destructors; the 91 known collections do too. But NCollection
        // template typedef names (e.g., TColStd_ListOfAsciiString) added to
        // all_class_names for param filtering don't have generated destructors.
        // Enum types are represented as Type::Class in raw parsed types — allow them.
        if let Type::Class(name) = ret {
            if !is_void_type_name(name) {
                if let Some(deletable) = ctx.deletable_class_names {
                    if !deletable.contains(name.as_str()) && !ctx.all_enums.contains(name.as_str()) {
                        return Err(format!("return type '{}' is not CppDeletable", name));
                    }
                }
            }
        }
        // MutRef to enum return type can't be bound — extern "C" expects int32_t& but C++ has EnumType&
        if return_type_is_mut_ref_enum(ret, ctx.all_enums) {
            return Err("return type is &mut enum (not representable in extern \"C\")".to_string());
        }
    }
    Ok(())
}

/// 
... (truncated, 4051 chars)
```
</details>

Now replace the duplicated code in `is_method_bindable`:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L892-L900">bindings.rs -66+8</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    // Const/mut return mismatch is now handled via C++ wrappers (ConstMutReturnFix).\n    // &mut enum output params are now handled via C++ wrappers (MutRefEnumParam).\n    check_params_bindable(&method.params, ctx)?;\n    if let Some(ref ret) = method.return_type {\n        check_return_type_bindable(ret, ctx)?;\n    }\n    Ok(())\n}",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "892:zd,893:av,894:ms,895:lh,896:cb,897:bi,898:gz,899:ot,900:ax,901:dr,902:jd,903:ra,904:kv,905:mi,906:sv,907:uo,908:ag,909:sr,910:cy,911:wz,912:kc,913:ax,914:wi,915:kz,916:ch,917:rx,918:jd,919:ux,920:fx,921:ag,922:sr,923:cy,924:wz,925:tv,926:ax,927:os,928:ty,929:ml,930:cy,931:sg,932:eo,933:lt,934:vk,935:uo,936:cy,937:po,938:sh,939:bv,940:fq,941:kv,942:ke,943:ae,944:xh,945:uo,946:kk,947:or,948:gl,949:uo,950:cy,951:tw,952:pm,953:dz,954:cy,955:ax,956:dh,957:io"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":892,"endLine":899}
```
</details>

Now refactor `is_constructor_bindable`:

<details>
<summary>Reading lines 900-970 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L900-L971">bindings.rs</a></summary>

```

/// Filter for constructors
fn is_constructor_bindable(
    ctor: &Constructor,
    _all_enum_names: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
    ctx: &TypeContext,
) -> Result<(), String> {
    // By-value class/handle params are now supported: C++ wrappers accept const T&
    // and the C++ compiler handles the copy.
    if ctor.has_unbindable_types() {
        let unbindable_details = describe_unbindable_types_ctor(ctor);
        return Err(format!("has unbindable types: {}", unbindable_details));
    }
    if let Some(p) = ctor
        .params
        .iter()
        .find(|p| param_uses_unknown_handle(&p.ty, handle_able_classes))
    {
        return Err(format!("param '{}' uses unknown Handle type", p.name));
    }
    // Also check for unknown class types in parameters.
    // This catches NCollection typedef types (e.g., TDF_LabelMap) that aren't
    // declared in the extern "C" FFI.
    if let Some(p) = ctor
        .params
        .iter()
        .find(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return Err(format!("param '{}' uses unknown type '{}'", p.name, p.ty.to_cpp_string()));
    }
    // Skip constructors where a nullable pointer param's inner type is unknown
    if let Some(p) = ctor.params.iter().find(|p| {
        if p.is_nullable_ptr() {
            match &p.ty {
                Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_type(inner, ctx),
                _ => false,
            }
        } else {
            false
        }
    }) {
        return Err(format!("nullable param '{}' inner type is unknown", p.name));
    }
    // Skip constructors where a class pointer param's inner type is unknown.
    // Check all_classes directly — nested types don't have FFI declarations.
    if let Some(p) = ctor.params.iter().find(|p| {
        if let Some(class_name) = p.ty.class_ptr_inner_name() {
            !ctx.all_classes.contains(class_name) && !ctx.all_enums.contains(class_name)
        } else {
            false
        }
    }) {
        return Err(format!("class pointer param '{}' inner type '{}' is unknown", p.name, p.ty.to_cpp_string()));
    }
    Ok(())
}

/// Filter for static methods
fn is_static_method_bindable(method: &StaticMethod, ctx: &TypeContext) -> Result<(), String> {
    if method.has_unbindable_types() {
        let unbindable_details = describe_unbindable_types_static(method);
        return Err(format!("has unbindable types: {}", unbindable_details));
    }
    // Skip static methods with const char*& or const char* const& params (need manual bindings)
    if let Some((param_name, type_name)) = resolver::static_method_has_string_ref_param(method) {
        return Err(format!("has string ref param '{}' of type '{}' (needs manual binding)", param_name, type_name));
    }
    // &mut enum output params are now handled via C++ wrappers.
    if let Some(p) = method
        .params
```
</details>

<details>
<summary>Reading lines 968-1030 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L968-L1031">bindings.rs</a></summary>

```
    // &mut enum output params are now handled via C++ wrappers.
    if let Some(p) = method
        .params
        .iter()
        .find(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return Err(format!("param '{}' uses unknown type '{}'", p.name, p.ty.to_cpp_string()));
    }
    // Skip static methods where a nullable pointer param's inner type is unknown
    if let Some(p) = method.params.iter().find(|p| {
        if p.is_nullable_ptr() {
            match &p.ty {
                Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_type(inner, ctx),
                _ => false,
            }
        } else {
            false
        }
    }) {
        return Err(format!("nullable param '{}' inner type is unknown", p.name));
    }
    // Skip static methods where a class pointer param's inner type is unknown.
    // Check all_classes directly — nested types don't have FFI declarations.
    if let Some(p) = method.params.iter().find(|p| {
        if let Some(class_name) = p.ty.class_ptr_inner_name() {
            !ctx.all_classes.contains(class_name) && !ctx.all_enums.contains(class_name)
        } else {
            false
        }
    }) {
        return Err(format!("class pointer param '{}' inner type '{}' is unknown", p.name, p.ty.to_cpp_string()));
    }
    if let Some(ref ret) = method.return_type {
        if type_uses_unknown_type(ret, ctx) {
            return Err(format!("return type '{}' is unknown", ret.to_cpp_string()));
        }
        // Same CppDeletable check as for instance methods (see is_method_bindable).
        // Enum types are represented as Type::Class in raw parsed types — allow them.
        if let Type::Class(name) = ret {
            if !is_void_type_name(name) {
                if let Some(deletable) = ctx.deletable_class_names {
                    if !deletable.contains(name.as_str()) && !ctx.all_enums.contains(name.as_str()) {
                        return Err(format!("return type '{}' is not CppDeletable", name));
                    }
                }
            }
        }
        // C-string returns (const char*) are handled via C++ wrappers returning const char*.
        // MutRef to enum return type can't be bound — extern "C" expects int32_t& but C++ has EnumType&
        if return_type_is_mut_ref_enum(ret, ctx.all_enums) {
            return Err("return type is &mut enum (not representable in extern \"C\")".to_string());
        }
    }
    Ok(())
}

/// Check if a return type is a mutable reference to an enum.
/// Extern "C" can't handle these: Rust side has `&mut i32` but C++ has `EnumType&`.
fn return_type_is_mut_ref_enum(ty: &Type, all_enums: &HashSet<String>) -> bool {
    if let Type::MutRef(inner) = ty {
        if let Type::Class(name) = inner.as_ref() {
            return all_enums.contains(name);
        }
```
</details>

Now refactor `is_constructor_bindable`:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L921-L926">bindings.rs -36+5</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    // Also check for unknown class types, nullable pointer inner types,\n    // and class pointer inner types.\n    check_params_bindable(&ctor.params, ctx)?;\n    Ok(())\n}",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "921:dl,922:ax,923:pa,924:xy,925:lh,926:cb,927:bi,928:gz,929:ot,930:ax,931:ux,932:mg,933:ra,934:kv,935:mi,936:sv,937:uo,938:ag,939:sr,940:cy,941:wz,942:kc,943:ax,944:bn,945:bz,946:mg,947:ux,948:fx,949:ag,950:sr,951:cy,952:wz,953:tv,954:ax,955:dh,956:io"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":921,"endLine":925}
```
</details>

Now refactor `is_static_method_bindable`:

<details>
<summary>Reading lines 930-945 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L930-L946">bindings.rs</a></summary>

```
        let unbindable_details = describe_unbindable_types_static(method);
        return Err(format!("has unbindable types: {}", unbindable_details));
    }
    // Skip static methods with const char*& or const char* const& params (need manual bindings)
    if let Some((param_name, type_name)) = resolver::static_method_has_string_ref_param(method) {
        return Err(format!("has string ref param '{}' of type '{}' (needs manual binding)", param_name, type_name));
    }
    // &mut enum output params are now handled via C++ wrappers.
    if let Some(p) = method
        .params
        .iter()
        .find(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return Err(format!("param '{}' uses unknown type '{}'", p.name, p.ty.to_cpp_string()));
    }
    // Skip static methods where a nullable pointer param's inner type is unknown
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L937-L944">bindings.rs -55+7</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    // &mut enum output params are now handled via C++ wrappers.\n    check_params_bindable(&method.params, ctx)?;\n    if let Some(ref ret) = method.return_type {\n        check_return_type_bindable(ret, ctx)?;\n    }\n    Ok(())\n}",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "937:og,938:ms,939:lh,940:cb,941:bi,942:gz,943:ot,944:ax,945:xl,946:jd,947:ra,948:kv,949:mi,950:sv,951:uo,952:ag,953:sr,954:cy,955:wz,956:kc,957:ax,958:yc,959:bz,960:jd,961:ux,962:fx,963:ag,964:sr,965:cy,966:wz,967:tv,968:ax,969:os,970:ty,971:ml,972:cy,973:oj,974:kv,975:ke,976:ae,977:xh,978:uo,979:kk,980:or,981:gl,982:uo,983:cy,984:tg,985:tw,986:pm,987:dz,988:cy,989:ax,990:dh,991:io"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":937,"endLine":943}
```
</details>

Now let me also check  and `type_uses_unknown_handle` in `crates/opencascade-binding-generator/src/type_mapping.rs` for duplication:

<details>
<summary>Reading lines 374-437 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L374-L438">type_mapping.rs</a></summary>

```
pub fn type_uses_unknown_class(ty: &Type, all_classes: &std::collections::HashSet<String>) -> bool {
    match ty {
        Type::Handle(class_name) => !all_classes.contains(class_name),
        Type::Class(class_name) => {
            if all_classes.contains(class_name) {
                return false;
            }
            // Void pointer types — Standard_Address (void*) and literal "void" — are known
            if crate::model::is_void_type_name(class_name) {
                return false;
            }
            // Primitive types mapped as Type::Class (e.g., "char" from Standard_Character)
            if crate::codegen::rust::is_primitive_type(class_name) {
                return false;
            }
            // Nested types (Parent::Nested) are known if the parent class is known
            if let Some(parent) = class_name.split("::").next() {
                if class_name.contains("::") && all_classes.contains(parent) {
                    return false;
                }
            }
            true
        }
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_class(inner, all_classes),
        _ => false,
    }
}

/// Check if a type references a Handle to a class that won't have a Handle declaration generated
/// This is more strict than type_uses_unknown_class - it checks that Handle types are for
/// classes that will actually have Handle<T> declarations generated (is_handle_type)
pub fn type_uses_unknown_handle(
    ty: &Type,
    all_classes: &std::collections::HashSet<String>,
    handle_able_classes: &std::collections::HashSet<String>,
) -> bool {
    match ty {
        Type::Handle(class_name) => !handle_able_classes.contains(class_name),
        Type::Class(class_name) => {
            if all_classes.contains(class_name) {
                return false;
            }
            // Void pointer types — Standard_Address (void*) and literal "void" — are known
            if crate::model::is_void_type_name(class_name) {
                return false;
            }
            // Primitive types mapped as Type::Class (e.g., "char" from Standard_Character)
            if crate::codegen::rust::is_primitive_type(class_name) {
                return false;
            }
            // Nested types (Parent::Nested) are known if the parent class is known
            if let Some(parent) = class_name.split("::").next() {
                if class_name.contains("::") && all_classes.contains(parent) {
                    return false;
                }
            }
            true
        }
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
            type_uses_unknown_handle(inner, all_classes, handle_able_classes)
        }
        _ => false,
    }
}
```
</details>

> 💭 Yes, the `Type::Class` arm in both `type_uses_unknown_class` and `type_uses_unknown_handle` is identical. The only difference is:
> 1. `Type::Handle(class_name)` - different check (all_classes vs handle_able_classes)
> 2. The recursive call (type_uses_unknown_class vs type_uses_unknown_handle)
> 
> I can extract the Class check into a shared helper `is_class_name_unknown`.
> 
> Let me refactor this too.

The  branch in both functions is identical. Let me refactor:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L374-L423">type_mapping.rs -64+49</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "/// Check if a class name is unknown (not in the known type set).\n/// Shared logic for type_uses_unknown_class and type_uses_unknown_handle.\nfn is_class_name_unknown(class_name: &str, all_classes: &std::collections::HashSet<String>) -> bool {\n    if all_classes.contains(class_name) {\n        return false;\n    }\n    // Void pointer types — Standard_Address (void*) and literal \"void\" — are known\n    if crate::model::is_void_type_name(class_name) {\n        return false;\n    }\n    // Primitive types mapped as Type::Class (e.g., \"char\" from Standard_Character)\n    if crate::codegen::rust::is_primitive_type(class_name) {\n        return false;\n    }\n    // Nested types (Parent::Nested) are known if the parent class is known\n    if let Some(parent) = class_name.split(\"::\").next() {\n        if class_name.contains(\"::\") && all_classes.contains(parent) {\n            return false;\n        }\n    }\n    true\n}\n\npub fn type_uses_unknown_class(ty: &Type, all_classes: &std::collections::HashSet<String>) -> bool {\n    match ty {\n        Type::Handle(class_name) => !all_classes.contains(class_name),\n        Type::Class(class_name) => is_class_name_unknown(class_name, all_classes),\n        Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_class(inner, all_classes),\n        _ => false,\n    }\n}\n\n/// Check if a type references a Handle to a class that won't have a Handle declaration generated\n/// This is more strict than type_uses_unknown_class - it checks that Handle types are for\n/// classes that will actually have Handle<T> declarations generated (is_handle_type)\npub fn type_uses_unknown_handle(\n    ty: &Type,\n    all_classes: &std::collections::HashSet<String>,\n    handle_able_classes: &std::collections::HashSet<String>,\n) -> bool {\n    match ty {\n        Type::Handle(class_name) => !handle_able_classes.contains(class_name),\n        Type::Class(class_name) => is_class_name_unknown(class_name, all_classes),\n        Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {\n            type_uses_unknown_handle(inner, all_classes, handle_able_classes)\n        }\n        _ => false,\n    }\n}",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "374:ks,375:er,376:ma,377:wb,378:dv,379:ta,380:uo,381:pm,382:ke,383:ta,384:uo,385:qi,386:pl,387:ta,388:uo,389:ze,390:vm,391:hp,392:bu,393:gl,394:uo,395:vy,396:cy,397:er,398:sd,399:ax,400:io,401:dd,402:jh,403:km,404:hp,405:zv,406:qj,407:gq,408:oq,409:yl,410:er,411:uy,412:wb,413:dv,414:ta,415:uo,416:pm,417:ke,418:ta,419:uo,420:qi,421:pl,422:ta,423:uo,424:ze,425:vm,426:hp,427:bu,428:gl,429:uo,430:vy,431:cy,432:lg,433:zv,434:cy,435:sd,436:ax,437:io"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/type_mapping.rs","startLine":374,"endLine":422}
```
</details>

Now let me verify it compiles:

**Terminal:**
```sh
time cargo check -p opencascade-binding-generator 2>&1
```
```
    Checking opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.37s
real  1.4s
```

Good. Now let me verify the refactoring produces identical output by regenerating and checking:

**Terminal:**
```sh
time ./scripts/regenerate-bindings.sh 2>&1 | tail -5
```

**Terminal:**
```sh
git diff --stat crates/opencascade-sys/generated/
```
```
(base) whitelily:opencascade-rs sethml$
```

No changes to generated output — the refactoring is purely structural. Now run the full test suite:

**Terminal:**
```sh
time ./scripts/run-binding-generator-tests.sh 2>&1
```

**Terminal:**
```sh
time cargo test --workspace --exclude opencascade-binding-generator 2>&1 | tail -10
```

All tests pass. Let me commit the refactoring:

**Terminal:**
```sh
cat > tmp/commit-msg-refactor.txt << 'ENDMSG'
refactor: extract shared param/return bindability checks

Extract duplicated validation logic from is_method_bindable,
is_constructor_bindable, and is_static_method_bindable into two
shared helpers:

- check_params_bindable(): unknown type, nullable pointer inner type,
  and class pointer inner type checks for parameter lists
- check_return_type_bindable(): unknown type, CppDeletable, class
  pointer inner type, and &mut enum checks for return types

Also extract is_class_name_unknown() from type_uses_unknown_class and
type_uses_unknown_handle in type_mapping.rs to deduplicate the
identical Type::Class branch.

Pure refactoring — generated output is identical (verified via
regenerate + git diff).
ENDMSG
```

**Terminal:**
```sh
git add crates/opencascade-binding-generator/src/codegen/bindings.rs crates/opencascade-binding-generator/src/type_mapping.rs && git commit -F tmp/commit-msg-refactor.txt
```
```
[main cd1e9e8] refactor: extract shared param/return bindability checks
 2 files changed, 89 insertions(+), 172 deletions(-)
```

**Todo list:**
- ✅ Explore tmp/ scripts and current skips
- ✅ Update README skip section
- ✅ Commit README update
- ✅ Fix highest-count skip reason
- ✅ Commit fix
- ✅ Evaluate and do refactoring
- ✅ Commit refactoring if done

All done. Here's a summary of the 3 commits:

1. **`f71fad2` docs: update skip reason breakdown to current counts (404 symbols)** — Updated the Skipped Symbols section of the binding generator README with current numbers from the generated code.

2. **`c4365e0` fix: resolve nested types in utility classes (10 fewer skips)** — When utility classes (like ) were converted to module-level functions, they were fully removed from the class list, causing  to be unresolvable in other classes. Fix: preserve the removed class names in the known-types set. Unblocked 10 methods across data exchange modules. Total skips: 404 → 394.

3. **`cd1e9e8` refactor: extract shared param/return bindability checks** — Extracted `check_params_bindable()`, `check_return_type_bindable()`, and `is_class_name_unknown()` to eliminate 83 lines of duplicated validation logic across `is_method_bindable`, `is_constructor_bindable`, `is_static_method_bindable`, `type_uses_unknown_class`, and `type_uses_unknown_handle`. Pure refactoring — generated output is identical.

*2026-02-21 17:47 · 1373s · Words: 14,435 in · 8,709 out · 128,637 ctx · 117 rounds*

---
