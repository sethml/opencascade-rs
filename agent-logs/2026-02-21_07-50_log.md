# Update documentation for current state and tools

- **Date:** 2026-02-21 07:41 – 08:35
- **Model:** Claude Opus 4.6
- **Turns:** 3
- **Tool calls:** 259
- **Thinking blocks:** 18
- **Input words:** 21,676
- **Output words:** 15,736
- **Prompt tokens (last round):** 269,301
- **API rounds:** 259
- **Total elapsed:** 2729s

## Table of Contents

1. [Update this section. There are leftover tools in tmp/ which might help with counting skipped symb...](#user-1) (Claude Opus 4.6)
2. [Commit. Then: I thought we resolved this issue by mapping void pointers to unsafe functions retur...](#user-2) (Claude Opus 4.6)
3. [Commit. Then think about whether you discovered opportunities to refactor the code the avoid repe...](#user-3) (Claude Opus 4.6)

---

<a id="user-1"></a>

## User (1)

> Update this section. There are leftover tools in tmp/ which might help with counting skipped symbols. Get rid of descriptions of already-fixed problems - this should just be a snapshot of the current state.

*2026-02-21 07:42*

### Assistant — Claude Opus 4.6

> 💭 The user wants me to update the "Skipped Symbols" section of the README.md (lines 367-455) to reflect the current state. Let me first look at what tools are in tmp/ and what the current skip counts are, then update the documentation.

**Todo list:**
- 🔄 Find tmp/ analysis tools
- ⬜ Get current skip counts
- ⬜ Update README skipped symbols section

Read [tmp](../tmp)

Read [count_skips.py](../tmp/count_skips.py)

Read [bucket_skips2.py](../tmp/bucket_skips2.py#1-1), lines 1 to 80

Read [analyze_skips.py](../tmp/analyze_skips.py#1-1), lines 1 to 80

Read [analyze_skips.py](../tmp/analyze_skips.py#80-80), lines 80 to 160

Read [bucket_skips2.py](../tmp/bucket_skips2.py#80-80), lines 80 to 99

Good. Let me run the most comprehensive analysis script to get current counts.

**Todo list:**
- ✅ Find tmp/ analysis tools
- 🔄 Get current skip counts
- ⬜ Update README skipped symbols section

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && python3 tmp/analyze_skips.py 2>&1
```
<details>
<summary>Output (187 lines)</summary>

```
Total skipped symbols: 1175

=== Skip Reason Breakdown ===
| Count |     % | Category | Description |
|-------|-------|----------|-------------|
|   631 | 53.7% | Unknown/unresolved type |
|   205 | 17.4% | Ambiguous lifetimes |
|    79 |  6.7% | Unknown Handle type |
|    72 |  6.1% | Abstract class |
|    19 |  1.6% | C-style array |
|    17 |  1.4% | String ref param |
|    15 |  1.3% | Stream type |
|    12 |  1.0% | Rvalue reference |
|    10 |  0.9% | Other: returns &mut with reference params — ambiguous lifetime |
|     6 |  0.5% | Other: has unbindable types: return: unresolved template type (const NCollection_Sequen |
|     6 |  0.5% | Other: has unbindable types: param 'thePlaneEquations': unresolved template type (NColl |
|     5 |  0.4% | Not CppDeletable |
|     4 |  0.3% | Other: has unbindable types: param 'theOther': unresolved template type (const NCollect |
|     4 |  0.3% | Other: has unbindable types: return: unresolved template type (const std::pair<Standard |
|     4 |  0.3% | &mut enum return |
|     4 |  0.3% | Other: has unbindable types: return: unresolved template type (const NCollection_Vector |
|     4 |  0.3% | Other: has unbindable types: return: unresolved template type (const NCollection_Array1 |
|     4 |  0.3% | Other: has unbindable types: return: unresolved template type (NCollection_Array1<openc |
|     4 |  0.3% | Other: has unbindable types: param 'theSizeXYZ': unresolved template type (const NColle |
|     4 |  0.3% | Other: has unbindable types: return: unresolved template type (std::shared_ptr<std::str |
|     3 |  0.3% | Other: param 'theBuilder' uses unknown Handle type |
|     2 |  0.2% | Other: has unbindable types: return: unresolved template type (const NCollection_Vec3<b |
|     2 |  0.2% | Other: has unbindable types: return: unresolved template type (NCollection_Vec3<bool>&) |
|     2 |  0.2% | Other: has unbindable types: param 'theAddStatus1': unresolved template type (const NCo |
|     2 |  0.2% | Other: excluded by bindings.toml |
|     2 |  0.2% | Other: has unbindable types: param 'theValue': unresolved template type (const NCollect |
|     2 |  0.2% | Other: has unbindable types: param 'theItems': unresolved template type (const NCollect |
|     2 |  0.2% | Other: has unbindable types: return: unresolved template type (NCollection_Handle<StepV |
|     2 |  0.2% | Other: has unbindable types: return: unresolved template type (const NCollection_DataMa |
|     1 |  0.1% | Other: has unbindable types: param 'theItem': unresolved template type (const NCollecti |
|     1 |  0.1% | Other: has unbindable types: param 'theSequence': unresolved template type (NCollection |
|     1 |  0.1% | Other: has unbindable types: return: unresolved template type (NCollection_Sequence<ope |
|     1 |  0.1% | Other: has unbindable types: param 'thePnts': unresolved template type (const NCollecti |
|     1 |  0.1% | Other: has unbindable types: param 'theKnots': unresolved t
... (truncated, 12337 chars total)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01NY1tXvEMHKfeHx8YVCjrGA__vscode-1771608612660/content.txt#1-1), lines 1 to 120

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01NY1tXvEMHKfeHx8YVCjrGA__vscode-1771608612660/content.txt#120-120), lines 120 to 189

> 💭 Now I need to understand the current state better. Let me also run the bucket_skips2.py to get cleaner categories, and let me count some things more precisely. The analyze_skips.py output has the "Other" categories that I need to collapse into the right buckets. Let me create a better script that properly categorizes everything.
> 
> Looking at the data:
> - Total: 1175 skipped symbols
> - The "Other" categories mostly fall into "Unresolved template type" - let me recount properly.
> 
> Let me write a comprehensive script to get accurate numbers for the README update.

Now let me get a cleaner categorization that properly buckets the "Other" template types:

Using "Create File"

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && python3 tmp/readme_skips.py 2>&1
```
<details>
<summary>Output (73 lines)</summary>

```
Total skipped symbols: 1175

=== Skip Reason Breakdown ===
| Count |     % | Category | Description |
|------:|------:|----------|-------------|
|   425 | 36.2% | **Unknown/unresolved type** | |
|   215 | 18.3% | **Ambiguous lifetimes** | |
|   206 | 17.5% | **Void pointer** | |
|    95 |  8.1% | **Unresolved template type** | |
|    83 |  7.1% | **Unknown Handle type** | |
|    72 |  6.1% | **Abstract class** | |
|    19 |  1.6% | **C-style array** | |
|    19 |  1.6% | **Stream (shared_ptr)** | |
|    17 |  1.4% | **String ref param** | |
|    12 |  1.0% | **Rvalue reference** | |
|     5 |  0.4% | **Not CppDeletable** | |
|     4 |  0.3% | **&mut enum return** | |
|     2 |  0.2% | **Excluded by bindings.toml** | |
|     1 |  0.1% | **Ambiguous overload** | |

=== Most Common Unknown Types (unknown + unknown Handle combined) ===
|    15 | `void` |
|    12 | `Handle(ShapePersistent_Geom::geometryBase<Geom_Surface>)` |
|    10 | `Handle(ShapePersistent_Geom::geometryBase<Geom_Curve>)` |
|    10 | `Handle(ShapePersistent_Geom::geometryBase<Geom2d_Curve>)` |
|     9 | `Handle(BVH_Builder<double, 3>)` |
|     8 | `Graphic3d_ZLayerId` |
|     6 | `Handle(NCollection_Shared<NCollection_List<gp_Pnt2d>>)` |
|     6 | `IMeshData::MapOfInteger` |
|     6 | `XmlObjMgt_Element` |
|     4 | `Handle(NCollection_Shared<TColStd_PackedMapOfInteger>)` |
|     4 | `IMeshData::IMapOfReal` |
|     3 | `Aspect_FBConfig` |
|     3 | `IMeshData::IFaceHandle` |
|     3 | `TopOpeBRep_PLineInter` |
|     3 | `V3d_ListOfLightIterator` |

=== Domain Analysis ===

**Data Exchange** (33 symbols) — xs_control (11), rw_gltf (7), step_control (5), rw_obj (4), iges_control (3), rw_stl (2), rw_ply (1)
   17 unknown type
    8 string ref param
    3 rvalue reference
    2 unresolved template
    2 abstract class
    1 other

**Document Framework** (16 symbols) — t_doc_std (8), tdf (8)
   11 ambiguous lifetimes
    4 unknown type
    1 void pointer

**Shape Meshing** (90 symbols) — b_rep_mesh (75), i_mesh_data (14), i_mesh_tools (1)
   70 unknown type
   10 unresolved template
    6 other
    3 abstract class
    1 ambiguous lifetimes

**Shape Analysis/Fix** (0 symbols)

**Geometry** (0 symbols)

**Poly** (13 symbols) — poly (13)
    4 ambiguous lifetimes
    4 other
    2 void pointer
    1 unknown type
    1 abstract class
    1 unresolved template

**Boolean Ops** (2 symbols) — b_rep_algo_api (2)
    2 unknown type
```
</details>

> 💭 Wait - the numbers don't match the existing README. The existing README says ~1,036 but now we have 1,175. Also the categories have shifted. Let me also check - the "Unknown/unresolved type" category at 425 seems different from the old 356. And the void pointer category at 206 has a notable overlap with "Standard_Address" - but that script should handle it. Let me also check the most common unknown type counts in the existing table more carefully.
> 
> Looking at the old table:
> - 99 Handle(TDocStd_Document) -- but now there are 0? That's suspicious -- maybe TDocStd_Document was added?
> 
> 
> - Now I'm seeing the breakdown of those missing types: void pointers at 24, XCAFPrs_Style at 19, and various IMeshData handles in the 10-12 range. The void pointer count of 206 in the new analysis maps to Standard_Address, and Handle(TDocStd_Document) dropping to zero suggests those types got properly bound already. I'm realizing the categorization logic is inconsistent between the two scripts. The original analyze_skips.py counts `Standard_Address` as "Unknown/unresolved type" because it matches the pattern for unknown types, but my new script filters it into "Void pointer" based on the reason text containing "void pointer". I need to look more carefully at how the original script actually categorizes these entries to match its logic properly. and handles the ambiguous lifetime variants more accurately. The key categories I need to focus on for the "Most Common Unknown Types" table are the Unknown/unresolved type and Unknown Handle type entries. Let me trace through why certain items like BinObjMgt_SRelocationTable aren't showing up in my unknown types list when they should be.
> 
> I think the issue is that my regex pattern for extracting unknown types is too narrow—it's only matching the specific format `type '...' is unknown`, but the actual reason text for these cases might be formatted differently. Let me search through the generated files to see what the actual text looks like for BinObjMgt_SRelocationTable.

The void* category at 206 seems really high compared to the old count of 200 (old) vs 206 now. Let me also check what the big unknown types are more carefully:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "Reason:" crates/opencascade-sys/generated/*.rs | grep -i "BinObjMgt_SRelocationTable" | head -5
```
<details>
<summary>Output (5 lines)</summary>

```
crates/opencascade-sys/generated/bin_m_data_std.rs://   Reason: param 'RelocTable' uses unknown type 'BinObjMgt_SRelocationTable&'
crates/opencascade-sys/generated/bin_m_data_std.rs://   Reason: param 'RelocTable' uses unknown type 'BinObjMgt_SRelocationTable&'
crates/opencascade-sys/generated/bin_m_data_std.rs://   Reason: param 'RelocTable' uses unknown type 'BinObjMgt_SRelocationTable&'
crates/opencascade-sys/generated/bin_m_data_std.rs://   Reason: param 'RelocTable' uses unknown type 'BinObjMgt_SRelocationTable&'
crates/opencascade-sys/generated/bin_m_data_std.rs://   Reason: param 'RelocTable' uses unknown type 'BinObjMgt_SRelocationTable&'
```
</details>

I see - my regex matches `unknown type '...'` - let me check my script more carefully. The issue is the regex `type '([^']+)' is unknown` but the reason says `uses unknown type 'BinObjMgt_SRelocationTable&'`. Let me fix the extraction:

Using "Create File"

**Terminal:**
```sh
rm /Users/sethml/src/opencascade-rs/tmp/readme_skips.py
```
```
(base) whitelily:opencascade-rs sethml$
```

Created [readme_skips.py](../tmp/readme_skips.py)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && python3 tmp/readme_skips.py 2>&1
```
<details>
<summary>Output (78 lines)</summary>

```
Total skipped symbols: 1175

=== Skip Reason Breakdown ===
| Count |     % | Category |
|------:|------:|----------|
|   425 | 36.2% | **Unknown/unresolved type** |
|   215 | 18.3% | **Ambiguous lifetimes** |
|   206 | 17.5% | **Void pointer** |
|    95 |  8.1% | **Unresolved template type** |
|    83 |  7.1% | **Unknown Handle type** |
|    72 |  6.1% | **Abstract class** |
|    19 |  1.6% | **C-style array** |
|    19 |  1.6% | **Stream (shared_ptr)** |
|    17 |  1.4% | **String ref param** |
|    12 |  1.0% | **Rvalue reference** |
|     5 |  0.4% | **Not CppDeletable** |
|     4 |  0.3% | **&mut enum return** |
|     2 |  0.2% | **Excluded by bindings.toml** |
|     1 |  0.1% | **Ambiguous overload** |

Unknown/unresolved + Unknown Handle combined: 508 (43.2%)

=== Most Common Unknown Types ===
| Count | Type |
|------:|------|
|    58 | `BinObjMgt_SRelocationTable` |
|    40 | `Graphic3d_ZLayerId` |
|    37 | `void` |
|    23 | `Standard_Utf32Char` |
|    19 | `XmlObjMgt_Element` |
|    13 | `std::istream` |
|    12 | `IMeshData::IEdgeHandle` |
|    12 | `Handle(ShapePersistent_Geom::geometryBase<Geom_Surface>)` |
|    11 | `Graphic3d_ArrayFlags` |
|    11 | `ShapeProcess::OperationsFlags` |
|    10 | `IMeshData::IFaceHandle` |
|    10 | `NCollection_String` |
|    10 | `Handle(ShapePersistent_Geom::geometryBase<Geom_Curve>)` |
|    10 | `Handle(ShapePersistent_Geom::geometryBase<Geom2d_Curve>)` |
|     9 | `Handle(BVH_Builder<double, 3>)` |

=== Domain Analysis ===

**Data Exchange** (33 symbols) -- `iges_control` (3), `rw_gltf` (7), `rw_obj` (4), `rw_ply` (1), `rw_stl` (2), `step_control` (5), `xs_control` (11)
   17 unknown type
    8 string ref param
    3 rvalue reference
    2 unresolved template
    2 abstract class
    1 stream (shared_ptr)

**Document Framework** (16 symbols) -- `t_doc_std` (8), `tdf` (8)
   11 ambiguous lifetimes
    4 unknown type
    1 void pointer

**Shape Meshing** (90 symbols) -- `b_rep_mesh` (75), `i_mesh_data` (14), `i_mesh_tools` (1)
   70 unknown type
   10 unresolved template
    6 C-style array
    3 abstract class
    1 ambiguous lifetimes

**Shape Analysis/Fix** (0 symbols)

**Geometry** (0 symbols)

**Poly** (13 symbols) -- `poly` (13)
    4 ambiguous lifetimes
    3 C-style array
    2 void pointer
    1 excluded by config
    1 unknown type
    1 abstract class
    1 unresolved template

**Boolean Ops** (2 symbols) -- `b_rep_algo_api` (2)
    2 unknown type
```
</details>

**Todo list:**
- ✅ Find tmp/ analysis tools
- ✅ Get current skip counts
- 🔄 Update README skipped symbols section

Now I have all the data. Let me also check the exact line range of the section to update in the README:

<details>
<summary>Reading lines 365-460 of <a href="../crates/opencascade-binding-generator/README.md#L365-L461">README.md</a></summary>

````
---

## Skipped Symbols

The binding generator skips ~1,036 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Every skipped symbol is documented in the generated per-module `.rs` files as a `// SKIPPED:` comment block including:

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
| 356 | 34.4% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`Handle(TDocStd_Document)`, `std::istream&`, `XCAFPrs_Style`, `IMeshData::IEdgeHandle`, etc.) |
| 200 | 19.3% | **Void pointer** | `Standard_Address` (typedef for `void*`) — cannot be safely expressed in Rust FFI |
| 176 | 17.0% | **Ambiguous lifetimes** | `&mut` return with reference params — Rust lifetime inference is ambiguous |
| 134 | 12.9% | **Raw pointer** | `T*`/`const T*` returns (static/free fn) or primitive pointer params (`int*`, `double*`) — class pointer returns on instance methods are now bound as `Option<&T>`/`Option<&mut T>` |
| 48 | 4.6% | **Unresolved template type** | Template instantiations that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `LDOMBasicString`, etc.) |
| 43 | 4.2% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |
| 17 | 1.6% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |
| 16 | 1.5% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |
| 15 | 1.4% | **Stream (shared_ptr)** | `std::shared_ptr<std::istream/ostream>` — smart-pointer-wrapped streams not yet bindable |
| 12 | 1.2% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across FFI |
| 7 | 0.7% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(IGESData_IGESModel)`, `Handle(CDM_MetaData)`, etc.) |
| 5 | 0.5% | **Not CppDeletable** | Return type class has no destructor in the binding set |
| 4 | 0.4% | **&mut enum return** | Mutable reference to enum (cxx limitation) |
| 2 | 0.2% | **Excluded by bindings.toml** | Explicitly excluded in config (e.g., ambiguous overload workarounds) |
| 1 | 0.1% | **Ambiguous overload** | C++ overload that would produce identical wrapper signatures |

### Most Common Unknown Types

The "unknown type" and "unknown Handle type" categories (35% of all skips) are dominated by a few types:

| Count | Type | How to Unblock |
|------:|------|----------------|
| 99 | `Handle(TDocStd_Document)` | Add `TDocStd_Document` — needed for document framework access |
| 24 | `void*` (return type) | Would need raw pointer return support for non-method contexts |
| 19 | `XCAFPrs_Style` | Add `XCAFPrs_Style` — used in XCAF presentation styles |
| 12 | `IMeshData::IEdgeHandle` | Nested handle typedef in meshing internals — low priority |
| 12 | `std::istream&` | Map bare `std::istream&` params (not `Standard_IStream`) — mostly in RWGltf/RWObj readers |
| 11 | `ShapeProcess::OperationsFlags` | Nested type in ShapeProcess — used in STEP/IGES processing flags |
| 10 | `IMeshData::IFaceHandle` | Nested handle typedef in meshing internals — low priority |

Previously common unknown types that have been resolved:
- ~~`Standard_OStream`/`Standard_IStream`~~ (was 546 skips) — resolved by adding `manual_types` config for stream types. `Standard_OStream&`/`Standard_IStream&` params are now fully bound; `std::cout`/`std::cerr`/`std::clog`/`std::
... (truncated, 8670 chars)
````
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01QS2gGA2BWWyY4wjHzXGN4z__vscode-1771608612671/content.txt)

Now I also need to check the same section in the root `README.md` since the user's selection was from that file:Now let me also check if the root `README.md` has the same section:

Searched for text `## Skipped Symbols` (`**/README.md`), 1 result
- [README.md:367](../crates/opencascade-binding-generator/README.md#L367)

> 💭 The section only exists in the binding generator README. The user's selection from the root README.md lines 367-455 must actually be from the binding generator README since that's the active file shown in the editor context. Let me also check the root README to be sure.

The section is only in the binding generator README. The user's active file is `crates/opencascade-binding-generator/README.md`. Let me also check what the section looks like in the root README (lines 367-455) that was the selection attachment:Good, now let me replace the entire Skipped Symbols section (lines 367-454) with updated content:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/README.md#L369-L445">README.md -86+76</a></summary>

**Input:**
````
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "369:kv,370:dd,371:zg,372:nf,373:gd,374:th,375:dd,376:xr,377:fj,378:xw,379:mh,380:il,381:ea,382:pi,383:dd,384:km,385:dd,386:fa,387:oi,388:fr,389:aj,390:hj,391:pv,392:wg,393:ty,394:yd,395:dn,396:dh,397:bv,398:pe,399:nd,400:ar,401:vx,402:ek,403:dd,404:ph,405:dd,406:fg,407:dd,408:js,409:bt,410:km,411:bn,412:ak,413:iv,414:hp,415:vv,416:ab,417:dd,418:ev,419:oy,420:dn,421:mf,422:ih,423:vl,424:uk,425:fv,426:xl,427:tk,428:vd,429:fj,430:gm,431:dd,432:hi,433:dd,434:rg,435:dd,436:ew,437:dd,438:zl,439:dd,440:ta,441:dd,442:sg,443:dd,444:wj,445:dd,446:sm,447:dd,448:xt,449:dd,450:ko,451:cr,452:zo,453:dd,454:br",
      "content": "The binding generator skips ~1,175 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Every skipped symbol is documented in the generated per-module `.rs` files as a `// SKIPPED:` comment block including:\n\n- **Source location** (header file, line number, C++ symbol name)\n- **Documentation comment** from the C++ header (first 3 lines)\n- **Skip reason** explaining why the symbol was excluded\n- **Commented-out Rust stub** showing the best-guess declaration\n\nExample from `gp.rs`:\n```rust\n// SKIPPED: **Source:** `gp_XYZ.hxx`:109 - `gp_XYZ::GetData`\n//   method: Returns a const ptr to coordinates location.\n//   Reason: has unbindable types: return: raw pointer (const double*)\n//   // pub fn get_data(&self) -> /* const double* */;\n```\n\n### Skip Reason Breakdown\n\n| Count | % | Category | Description |\n|------:|----:|----------|-------------|\n| 425 | 36.2% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`BinObjMgt_SRelocationTable`, `Graphic3d_ZLayerId`, `XmlObjMgt_Element`, `IMeshData::IEdgeHandle`, etc.) |\n| 215 | 18.3% | **Ambiguous lifetimes** | `&mut` return with reference params — Rust lifetime inference is ambiguous |\n| 206 | 17.5% | **Void pointer** | `Standard_Address` (typedef for `void*`) — cannot be safely expressed in Rust FFI |\n| 95 | 8.1% | **Unresolved template type** | Template instantiations that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `NCollection_Vec3<...>`, etc.) |\n| 83 | 7.1% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(ShapePersistent_Geom::...)`, `Handle(BVH_Builder<...>)`, etc.) |\n| 72 | 6.1% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |\n| 19 | 1.6% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |\n| 19 | 1.6% | **Stream (shared_ptr)** | `std::shared_ptr<std::istream/ostream>` — smart-pointer-wrapped streams not yet bindable |\n| 17 | 1.4% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |\n| 12 | 1.0% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across F
... (truncated, 6874 chars)
````
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/README.md","startLine":369,"endLine":444}
```
</details>

Now let me also update the same section in the root `README.md` (the selection came from there):Now let me check if the root README has a "Skipped Symbols" section too:

Searched for text `Skipped Symbols` (`**/README.md`), 4 results
- [README.md:367](../crates/opencascade-binding-generator/README.md#L367)
- [README.md:422](../crates/opencascade-binding-generator/README.md#L422)
- [README.md:424](../crates/opencascade-binding-generator/README.md#L424)
- [README.md:438](../crates/opencascade-binding-generator/README.md#L438)

The section only exists in the binding generator README. Now let me verify the edit looks correct:

<details>
<summary>Reading lines 367-450 of <a href="../crates/opencascade-binding-generator/README.md#L367-L451">README.md</a></summary>

````
## Skipped Symbols

The binding generator skips ~1,175 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Every skipped symbol is documented in the generated per-module `.rs` files as a `// SKIPPED:` comment block including:

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
| 425 | 36.2% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`BinObjMgt_SRelocationTable`, `Graphic3d_ZLayerId`, `XmlObjMgt_Element`, `IMeshData::IEdgeHandle`, etc.) |
| 215 | 18.3% | **Ambiguous lifetimes** | `&mut` return with reference params — Rust lifetime inference is ambiguous |
| 206 | 17.5% | **Void pointer** | `Standard_Address` (typedef for `void*`) — cannot be safely expressed in Rust FFI |
| 95 | 8.1% | **Unresolved template type** | Template instantiations that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `NCollection_Vec3<...>`, etc.) |
| 83 | 7.1% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(ShapePersistent_Geom::...)`, `Handle(BVH_Builder<...>)`, etc.) |
| 72 | 6.1% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |
| 19 | 1.6% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |
| 19 | 1.6% | **Stream (shared_ptr)** | `std::shared_ptr<std::istream/ostream>` — smart-pointer-wrapped streams not yet bindable |
| 17 | 1.4% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |
| 12 | 1.0% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across FFI |
| 5 | 0.4% | **Not CppDeletable** | Return type class has no destructor in the binding set |
| 4 | 0.3% | **&mut enum return** | Mutable reference to enum (cxx limitation) |
| 2 | 0.2% | **Excluded by bindings.toml** | Explicitly excluded in config (e.g., ambiguous overload workarounds) |
| 1 | 0.1% | **Ambiguous overload** | C++ overload that would produce identical wrapper signatures |

### Most Common Unknown Types

The "unknown type" and "unknown Handle type" categories (43% of all skips) are dominated by a few types:

| Count | Type | How to Unblock |
|------:|------|----------------|
| 58 | `BinObjMgt_SRelocationTable` | Add `BinObjMgt_SRelocationTable` — used in binary persistence (BinMDataStd, BinMXCAFDoc) |
| 40 | `Graphic3d_ZLayerId` | Typedef for `Standard_Integer` — needs typedef resolution |
| 37 | `void*` (return type) | Would need raw pointer return support for non-method contexts |
| 23 | `Standard_Utf32Char` | Map as `u32` in type_mapping.rs |
| 19 | `XmlObjMgt_Element` | Add `XmlObjMgt_Element` — used in XML persistence (XmlMDataStd) |
| 13 | `std::istream&` | Map bare `std::istream&` params (not `Standard_IStream`) — mostly in RWGltf/RWObj readers |
| 12 | `IMeshData::IEdgeHandle` | Nested handle typedef in meshing internals — low priority |
| 12 | `Handle(ShapePersistent_Geom::...)` | Nested template Handle types in shape persistence — low priority |
| 11 | `Graphic3d_ArrayFlags` | Typedef for `Standard_Integer` — needs typedef resolution |
| 11 | `ShapeProcess::OperationsFlags` | Nested type in ShapeProcess — used in STEP/IGES processing flags |
| 10 | `IMeshData::IFaceHandle` | Nested handle typedef in meshing internals — low priority |
| 10 | `NCollection_String` | Add `NCollection_String` — used in Graphic3d and Fo
... (truncated, 6243 chars)
````
</details>

The edit looks clean. Now let me also check if the same section appears duplicated in the root README (the user's selection attachment was from README.md):Now let me check the root `README.md` to see if the same section is duplicated there:

<details>
<summary>Reading lines 360-470 of <a href="../README.md#L360-L471">README.md</a></summary>

```
Error: startLine 360 exceeds file length (168 lines).
```
</details>

The root README only has 168 lines — the selection was definitely from the binding generator README (which is also the user's active file). The update is complete.

**Todo list:**
- ✅ Find tmp/ analysis tools
- ✅ Get current skip counts
- ✅ Update README skipped symbols section

Updated the Skipped Symbols section in `crates/opencascade-binding-generator/README.md`. Key changes:

- **Total count**: ~1,036 → ~1,175 (binding set has grown significantly)
- **Skip Reason Breakdown**: Updated all counts/percentages from current generated files. Notable shifts: "Raw pointer" category dropped to 0 (class pointer returns are now bound as `Option<&T>`/`Option<&mut T>`), "Unknown Handle type" grew from 7 to 83
- **Most Common Unknown Types**: Replaced with current top blockers (`BinObjMgt_SRelocationTable` at 58, `Graphic3d_ZLayerId` at 40, etc.). Removed the old `Handle(TDocStd_Document)` at 99 — no longer appears
- **Removed "Previously resolved" list**: Deleted the 11-item strikethrough history of already-fixed types
- **Important Skipped Symbols**: Updated all domain counts to current state (Data Exchange 57→33, Document Framework 74→16, Shape . Key changes:

- **Total count**: ~1,036 → ~1,175 (binding set has grown significantly)
- **Skip Reason Breakdown**: Updated all counts/percentages from current generated files. Notable shifts: "Raw pointer" category dropped to 0 (class pointer returns are now bound as `Option<&T>`/`Option<&mut T>`), "Unknown Handle type" grew from 7 to 83
- **Most Common Unknown Types**: Replaced with current top blockers (`BinObjMgt_SRelocationTable` at 58, `Graphic3d_ZLayerId` at 40, etc.). Removed the old `Handle(TDocStd_Document)` at 99 — no longer appears
- **Removed "Previously resolved" list**: Deleted the 11-item strikethrough history of already-fixed types
- **Important Skipped Symbols**: Updated all domain counts to current state (Data Exchange 57→33, Document Framework 74→16, Shape Analysis/Fix now at 0, Geometry now at 0). Removed historical narrative about how counts were reduced

*2026-02-21 07:46 · 254s · Words: 2,149 in · 4,224 out · 59,400 ctx · 25 rounds*

---

<a id="user-2"></a>

## User (2)

> Commit. Then: I thought we resolved this issue by mapping void pointers to unsafe functions returning ffi::c_void pointers? Look for git commits in the past 3 days that relate to functions taking or returning pointers, and figure out why they&#x27;re not handling these exceptions, and fix it, then commit.

*2026-02-21 07:51*

### Assistant — Claude Opus 4.6

> 💭 The user wants me to:
> 1. First commit the current changes
> 2. Then investigate why void pointer (`

> 💭 The user wants me to:
> 1. First commit the current changes
> 2. Then investigate why void pointer (`Standard_Address`) skips are still happening, since there was apparently a recent commit that should have handled void pointers by mapping them to `ffi::c_void` pointers with unsafe functions
> 3. Fix the issue and commit
> 
> Let me start by committing, then investigating.

**Todo list:**
- 🔄 Commit current README changes
- ⬜ Find recent pointer-related commits
- ⬜ Investigate void pointer skips
- ⬜ Fix void pointer handling
- ⬜ Rebuild and test
- ⬜ Commit fix

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git add -A && cat > tmp/commit-msg.txt << 'EOF'
Update Skipped Symbols section in binding generator README

Refresh all skip counts from current generated files (1,175 total).
Remove historical "previously resolved" list and reduction narratives.
Update domain-level analysis with current counts.
EOF
git commit -F tmp/commit-msg.txt
```
```
(base) whitelily:opencascade-rs sethml$ git commit -F tmp/commit-msg.txt
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git log --oneline -1
```
```
9b17728 (HEAD -> main) Update Skipped Symbols section in binding generator README
```

**Todo list:**
- ✅ Commit current README changes
- 🔄 Find recent pointer-related commits
- ⬜ Investigate void pointer skips
- ⬜ Fix void pointer handling
- ⬜ Rebuild and test
- ⬜ Commit fix

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git log --oneline --since="3 days ago" | grep -i "pointer\|ptr\|void\|raw\|unsafe\|address\|c_void\|ffi"
```
<details>
<summary>Output (5 lines)</summary>

```
e0fd0bc Bind Message_LazyProgressScope and Poly_CoherentTriPtr without new/delete
8ba1671 Build binding generator in release mode to avoid clang crate UB crash
aa43d44 Bind class pointer returns as Option<&T>
9ac8bcd Bind non-nullable class pointer params as references
9bde978 Remove FFI type name escaping, use fully-qualified Result in TryFrom
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git log --oneline --since="3 days ago" | head -30
```
<details>
<summary>Output (30 lines)</summary>

```
9b17728 Update Skipped Symbols section in binding generator README
3a00dd6 Fix method-vs-constructor name collision disambiguation
367fb53 Skip std:: template typedefs in binding generator
e0fd0bc Bind Message_LazyProgressScope and Poly_CoherentTriPtr without new/delete
552b5c3 Fix build errors and warnings after un-excluding visualization modules
c932973 Un-exclude visualization/rendering modules from bindings
5842f32 Fix build warnings in opencascade-sys
bbe1bb0 Support nested C++ classes in binding generator
f2fb80e Include persistence/storage modules and fix nested class handle names
6058160 Detect pure virtual destructors to correctly mark abstract classes
8ba1671 Build binding generator in release mode to avoid clang crate UB crash
6614771 binding-generator: detect handle-able classes via RTTI fallback
8b0a109 Add u8/i8 type mapping for unsigned char/signed char in binding generator
227df89 Fix typedef map: 1:many, remove text scanner, use clang canonical types
d5296ae Fix Handle case, CHAR16 type mapping, and restore template typedef normalization
9f68892 Add Type::I16, fix char16_t vs uint16_t, clean up normalize_template_spelling
32f5c25 Support iostream/ostream types in generated bindings
5a2a383 Fix false template type heuristic and typedef map pollution
6e9ef9f Rename "unresolved template/nested type" skip reason to "unresolved template type"
067587d Fix typedef collection filter to include lowercase-prefix OCCT types (math_*)
aa43d44 Bind class pointer returns as Option<&T>
9ac8bcd Bind non-nullable class pointer params as references
1eec724 Map Standard_Character, Standard_ExtCharacter/ExtString, and make extra typedef types CppDeletable
b0d2282 Skip non-standard header names during OCCT module discovery
bf53604 Remove split('_').skip(1) heuristic, fix stack reference warning
95c4ef8 Switch binding config from module whitelist to blacklist
52dcd55 Update skipped symbols analysis in binding generator README
4632bfc Remove misresolution heuristic (dead code after OSD_WNT.hxx fix)
d2d1a1f Fail loudly when clang hits fatal parse errors during batch parsing
a8cca99 Fix root cause of template type misresolution: OSD_WNT.hxx fatal parse error
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git show aa43d44 --stat && echo "---" && git show 9ac8bcd --stat
```
<details>
<summary>Output (261 lines)</summary>

```
commit aa43d4459fee1303e2ab07b1ac59eb29ad83f989
Author: Seth LaForge <sethml@ofb.net>
Date:   Thu Feb 19 15:32:23 2026 -0800

    Bind class pointer returns as Option<&T>
    
    Instance methods returning const T* or T* (where T is a known class type)
    are now bound as Option<&T> or Option<&mut T> in Rust, with a null check.
    This unlocks 2,364 methods (mostly inherited This() methods) that were
    previously skipped as "raw pointer" returns.
    
    Key design decisions:
    - Const methods (&self) returning T* use Option<&T> (not &mut T) to avoid
      unsound &self -> &mut T references
    - Static methods and free functions are NOT bound this way since they have
      no &self lifetime to borrow from
    - The C++ wrapper passes the raw pointer through unchanged; the Rust
      reexport does the null check
    
    Raw pointer skips reduced from 178 to 135 (43 fewer). Total skips
    reduced from 2,259 to 2,232.

 .../2026-02-19_14-15-vscode-claude-opus-4.6.md     | 1515 +++-
 crates/opencascade-binding-generator/README.md     |   37 +-
 .../src/codegen/bindings.rs                        |  137 +-
 crates/opencascade-binding-generator/src/model.rs  |    2 +-
 crates/opencascade-sys/generated/adaptor2d.rs      |   37 +
 crates/opencascade-sys/generated/adaptor3d.rs      |   74 +
 crates/opencascade-sys/generated/adv_app2_var.rs   |   36 +
 .../generated/api_header_section.rs                |   14 +
 crates/opencascade-sys/generated/app_def.rs        |   40 +
 crates/opencascade-sys/generated/app_par_curves.rs |   58 +
 crates/opencascade-sys/generated/approx.rs         |   52 +
 crates/opencascade-sys/generated/b_rep.rs          |  245 +
 crates/opencascade-sys/generated/b_rep_adaptor.rs  |   65 +
 crates/opencascade-sys/generated/b_rep_algo.rs     |   12 +
 crates/opencascade-sys/generated/b_rep_approx.rs   |   13 +
 crates/opencascade-sys/generated/b_rep_blend.rs    |   63 +
 .../opencascade-sys/generated/b_rep_builder_api.rs |   27 +
 crates/opencascade-sys/generated/b_rep_check.rs    |   84 +
 crates/opencascade-sys/generated/b_rep_extrema.rs  |   14 +
 crates/opencascade-sys/generated/b_rep_fill.rs     |  124 +
 crates/opencascade-sys/generated/b_rep_mesh.rs     |  361 +
 crates/opencascade-sys/generated/b_rep_offset.rs   |   39 +-
 crates/opencascade-sys/generated/b_rep_tools.rs    |   93 +
 .../opencascade-sys/generated/b_rep_top_adaptor.rs |   26 +
 crates/opencascade-sys/generated/b_spl_c_lib.rs    |   12 +
 crates/opencascade-sys/generated/b_spl_s_lib.rs    |   12 +
 crates/opencascade-sys/generated/bi_tgte.rs        |   25 +
 crates/opencascade-sys/generated/bisector.rs       |   48 +
 crates/
... (truncated, 17779 chars total)
```
</details>

Let me look at the actual skip reasons to understand what void pointer skips look like:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep "Reason:.*void pointer\|Reason:.*Standard_Address" crates/opencascade-sys/generated/*.rs | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
crates/opencascade-sys/generated/adaptor3d.rs://   Reason: return type 'Standard_Address' is unknown
crates/opencascade-sys/generated/b_rep_approx.rs://   Reason: param 'PtrSvSurfaces' uses unknown type 'Standard_Address'
crates/opencascade-sys/generated/b_rep_prim_api.rs://   Reason: return type 'Standard_Address' is unknown
crates/opencascade-sys/generated/b_rep_prim_api.rs://   Reason: return type 'Standard_Address' is unknown
crates/opencascade-sys/generated/b_rep_prim_api.rs://   Reason: return type 'Standard_Address' is unknown
crates/opencascade-sys/generated/b_rep_prim_api.rs://   Reason: return type 'Standard_Address' is unknown
crates/opencascade-sys/generated/b_rep_prim_api.rs://   Reason: return type 'Standard_Address' is unknown
crates/opencascade-sys/generated/b_rep_prim_api.rs://   Reason: return type 'Standard_Address' is unknown
crates/opencascade-sys/generated/b_rep_top_adaptor.rs://   Reason: return type 'Standard_Address' is unknown
crates/opencascade-sys/generated/cdm.rs://   Reason: param 'aModifContext' uses unknown type 'Standard_Address'
crates/opencascade-sys/generated/cdm.rs://   Reason: param 'aModifContext' uses unknown type 'Standard_Address'
crates/opencascade-sys/generated/extrema.rs://   Reason: return type 'Standard_Address' is unknown
crates/opencascade-sys/generated/extrema.rs://   Reason: param 'C' uses unknown type 'Standard_Address'
crates/opencascade-sys/generated/extrema.rs://   Reason: return type 'Standard_Address' is unknown
crates/opencascade-sys/generated/extrema.rs://   Reason: param 'C' uses unknown type 'Standard_Address'
crates/opencascade-sys/generated/geom_int.rs://   Reason: param 'PtrSvSurfaces' uses unknown type 'Standard_Address'
crates/opencascade-sys/generated/graphic3d.rs://   Reason: param 'theOwner' uses unknown type 'Standard_Address'
crates/opencascade-sys/generated/graphic3d.rs://   Reason: return type 'Standard_Address' is unknown
crates/opencascade-sys/generated/hlrb_rep.rs://   Reason: param 'C' uses unknown type 'const Standard_Address&'
crates/opencascade-sys/generated/hlrb_rep.rs://   Reason: param 'C' uses unknown type 'const Standard_Address&'
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -c "Reason:.*Standard_Address" crates/opencascade-sys/generated/*.rs
```
<details>
<summary>Output (337 lines)</summary>

```
crates/opencascade-sys/generated/adaptor2d.rs:0
crates/opencascade-sys/generated/adaptor3d.rs:1
crates/opencascade-sys/generated/adv_app2_var.rs:0
crates/opencascade-sys/generated/adv_approx.rs:0
crates/opencascade-sys/generated/ais.rs:0
crates/opencascade-sys/generated/api_header_section.rs:0
crates/opencascade-sys/generated/app_blend.rs:0
crates/opencascade-sys/generated/app_cont.rs:0
crates/opencascade-sys/generated/app_def.rs:0
crates/opencascade-sys/generated/app_par_curves.rs:0
crates/opencascade-sys/generated/app_std_l.rs:0
crates/opencascade-sys/generated/app_std.rs:0
crates/opencascade-sys/generated/approx_int.rs:0
crates/opencascade-sys/generated/approx.rs:0
crates/opencascade-sys/generated/aspect.rs:0
crates/opencascade-sys/generated/b_rep_adaptor.rs:0
crates/opencascade-sys/generated/b_rep_algo_api.rs:0
crates/opencascade-sys/generated/b_rep_algo.rs:0
crates/opencascade-sys/generated/b_rep_approx.rs:1
crates/opencascade-sys/generated/b_rep_blend.rs:0
crates/opencascade-sys/generated/b_rep_bnd_lib.rs:0
crates/opencascade-sys/generated/b_rep_builder_api.rs:0
crates/opencascade-sys/generated/b_rep_check.rs:0
crates/opencascade-sys/generated/b_rep_class.rs:0
crates/opencascade-sys/generated/b_rep_class3d.rs:0
crates/opencascade-sys/generated/b_rep_extrema.rs:0
crates/opencascade-sys/generated/b_rep_feat.rs:0
crates/opencascade-sys/generated/b_rep_fill.rs:0
crates/opencascade-sys/generated/b_rep_fillet_api.rs:0
crates/opencascade-sys/generated/b_rep_g_prop.rs:0
crates/opencascade-sys/generated/b_rep_int_curve_surface.rs:0
crates/opencascade-sys/generated/b_rep_l_prop.rs:0
crates/opencascade-sys/generated/b_rep_lib.rs:0
crates/opencascade-sys/generated/b_rep_mat2d.rs:0
crates/opencascade-sys/generated/b_rep_mesh.rs:0
crates/opencascade-sys/generated/b_rep_offset_api.rs:0
crates/opencascade-sys/generated/b_rep_offset.rs:0
crates/opencascade-sys/generated/b_rep_preview_api.rs:0
crates/opencascade-sys/generated/b_rep_prim_api.rs:6
crates/opencascade-sys/generated/b_rep_prim.rs:0
crates/opencascade-sys/generated/b_rep_proj.rs:0
crates/opencascade-sys/generated/b_rep_sweep.rs:0
crates/opencascade-sys/generated/b_rep_to_iges.rs:0
crates/opencascade-sys/generated/b_rep_to_igesb_rep.rs:0
crates/opencascade-sys/generated/b_rep_tools.rs:0
crates/opencascade-sys/generated/b_rep_top_adaptor.rs:1
crates/opencascade-sys/generated/b_rep.rs:0
crates/opencascade-sys/generated/b_spl_c_lib.rs:0
crates/opencascade-sys/generated/b_spl_s_lib.rs:0
crates/opencascade-sys/generated/bi_tgte.rs:0
crates/opencascade-sys/generated/bin_drivers.rs:0
crates/opencascade-sys/generated/bin_l_drivers.rs:0
crates/opencascade-sys/generated/bin_m_data_std.rs:0
crates/opencascade-sys/generated/bin_m_data_xtd.rs:0
crates/opencascade-sys/generated/bin_m_doc_std.rs:0
crates/opencascade-sys/generated/bin_m_function.rs:0
crates/opencascade-sys/generated/bin_m_naming.rs:0
crates/opencascade-sys/generated/bin_mdf.rs:0
crates/open
... (truncated, 16723 chars total)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -c "Reason:.*Standard_Address" crates/opencascade-sys/generated/*.rs | grep -v ':0$' | sort -t: -k2 -rn
```
<details>
<summary>Output (17 lines)</summary>

```
crates/opencascade-sys/generated/hlrb_rep.rs:151
crates/opencascade-sys/generated/top_ope_b_rep_build.rs:15
crates/opencascade-sys/generated/standard.rs:6
crates/opencascade-sys/generated/b_rep_prim_api.rs:6
crates/opencascade-sys/generated/osd.rs:5
crates/opencascade-sys/generated/mesh_vs.rs:5
crates/opencascade-sys/generated/extrema.rs:4
crates/opencascade-sys/generated/poly.rs:2
crates/opencascade-sys/generated/math.rs:2
crates/opencascade-sys/generated/graphic3d.rs:2
crates/opencascade-sys/generated/cdm.rs:2
crates/opencascade-sys/generated/top_ope_b_rep_ds.rs:1
crates/opencascade-sys/generated/t_doc_std.rs:1
crates/opencascade-sys/generated/geom_int.rs:1
crates/opencascade-sys/generated/b_rep_top_adaptor.rs:1
crates/opencascade-sys/generated/b_rep_approx.rs:1
crates/opencascade-sys/generated/adaptor3d.rs:1
```
</details>

So `Standard_Address` is being treated as an "unknown type" rather than being recognized. It's a typedef for `void*`. Let me look at how types are resolved and where `Standard_Address` is handled:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -rn "Standard_Address\|void\b.*pointer\|c_void" crates/opencascade-binding-generator/src/ | grep -v "test\|\.rs:.*//\|target" | head -40
```
<details>
<summary>Output (12 lines)</summary>

```
crates/opencascade-binding-generator/src/type_mapping.rs:196:        Type::Class(class_name) if class_name == "Standard_Address" => {
crates/opencascade-binding-generator/src/type_mapping.rs:199:                rust_type: "*mut std::ffi::c_void".to_string(),
crates/opencascade-binding-generator/src/codegen/cpp.rs:96:                    "void" | "size_t" | "Standard_Address"
crates/opencascade-binding-generator/src/codegen/bindings.rs:609:            } else if name == "Standard_Address" {
crates/opencascade-binding-generator/src/codegen/bindings.rs:610:                "*mut std::ffi::c_void".to_string()
crates/opencascade-binding-generator/src/codegen/bindings.rs:637:        Type::Class(name) if name != "char" && name != "Standard_Address" => {
crates/opencascade-binding-generator/src/codegen/bindings.rs:667:        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "char" && name != "Standard_Address") => {
crates/opencascade-binding-generator/src/codegen/bindings.rs:731:    if ty.is_void_ptr() { return format!("void pointer ({})", ty.to_cpp_string()); }
crates/opencascade-binding-generator/src/codegen/bindings.rs:1854:                .map(|ty| ty.is_reference() || matches!(ty, Type::ConstPtr(inner) | Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "char" && name != "Standard_Address")))
crates/opencascade-binding-generator/src/codegen/bindings.rs:3208:                "void" | "size_t" | "Standard_Address"
crates/opencascade-binding-generator/src/parser.rs:1625:        "Standard_Address" => Some(Type::Class("Standard_Address".to_string())),
crates/opencascade-binding-generator/src/model.rs:600:            Type::Class(name) => name == "Standard_Address",
```
</details>

**Todo list:**
- ✅ Commit current README changes
- ✅ Find recent pointer-related commits
- 🔄 Investigate void pointer skips
- ⬜ Fix void pointer handling
- ⬜ Rebuild and test
- ⬜ Commit fix

So `Standard_Address` IS recognized in some places (type_mapping.rs, bindings.rs), but it's being reported as "unknown type" in the skip reasons. Let me trace how the skip decision is made. Let me look at the bindability check functions:

<details>
<summary>Reading lines 590-750 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L590-L751">bindings.rs</a></summary>

```
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
        Type::CHAR16 => "u16".to_string(),
        Type::U8 => "u8".to_string(),
        Type::I8 => "i8".to_string(),
        Type::Class(name) => {
            if name == "char" {
                "std::ffi::c_char".to_string()
            } else if name == "Standard_Address" {
                "*mut std::ffi::c_void".to_string()
            } else if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            }
        }
        Type::Handle(name) => format!("crate::ffi::{}", type_mapping::handle_type_name(name)),
        Type::ConstRef(inner) => format!("&{}", type_to_rust_string(inner, reexport_ctx)),
        Type::MutRef(inner) => {
            format!("&mut {}", type_to_rust_string(inner, reexport_ctx))
        }
        Type::RValueRef(_) => "()".to_string(),
        Type::ConstPtr(inner) => {
            if matches!(inner.as_ref(), Type::Class(name) if name == "char") {
                "&str".to_string()
            } else {
                format!("*const {}", type_to_rust_string(inner, reexport_ctx))
            }
        }
        Type::MutPtr(inner) => format!("*mut {}", type_to_rust_string(inner, reexport_ctx)),
    }
}

/// Convert a return Type to Rust type string for re-export files
fn return_type_to_rust_string(ty: &Type, reexport_ctx: Option<&ReexportTypeContext>) -> String {
    match ty {
        Type::Class(name) if name != "char" && name != "Standard_Address" => {
            let inner = if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            };
            format!("crate::OwnedPtr<{}>", inner)
        }
        Type::Handle(name) => {
            format!(
                "crate::OwnedPtr<crate::ffi::{}>",
                type_mapping::handle_type_name(name)
            )
        }
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "std::string::String".to_string()
        }
        // Class pointer returns -> Option<&T> / Option<&mut T>
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(_)) => {
            if let Type::Class(name) = inner.as_ref() {
                let resolved = if let Some(ctx) = reexport_ctx {
                    ctx.resolve_class(name)
                } else {
                    format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
                };
                format!("Option<&{}>", resolved)
            } else {
                unreachable!()
            }
        }
        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "char" && name != "Standard_Address") => {
            if let Type::Class(name) = inner.as_ref() {
                let resolved = if let Some(ctx) = reexport_ctx {
                    ctx.resolve_class(name)
                } else {
                    format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
                };
                format!("Option<&mut {}>", resolved)
            } else {
                unreachable!()
            }
        }
        _ => type_to_rust_string(ty, reexport_ctx),
    }
}

// ── Filtering predicates ────────────────────────────────────────────────────

/// Describe which types in a method's params/return are unbindable.
... (truncated, 7113 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01P4SbtKoHYT8ZHX1QjneNp4__vscode-1771608612691/content.txt)

Now I need to understand the flow. The `is_unbindable()` method is key. Let me look at that:

Searched for text `fn is_unbindable` (`**/crates/opencascade-binding-generator/src/**`), 1 result
- [model.rs:709](../crates/opencascade-binding-generator/src/model.rs#L709)

<details>
<summary>Reading lines 700-750 of <a href="../crates/opencascade-binding-generator/src/model.rs#L700-L751">model.rs</a></summary>

```
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
    }

    /// Check if this type involves raw pointers that require the containing
    /// function to be marked `unsafe`. True for void pointers (Standard_Address)
    /// and raw T*/const T* pointers (excluding const char* which is handled
    /// as C strings).
    pub fn needs_unsafe_fn(&self) -> bool {
        self.is_void_ptr() || self.is_raw_ptr()
    }

    /// Convert this type to a C++ parameter type for extern "C" wrapper functions.
    /// References become pointers (const T& → const T*, T& → T*).
    pub fn to_cpp_extern_c_param(&self) -> String {
        match self {
            Type::ConstRef(inner) => format!("const {}*", inner.to_cpp_string()),
            Type::MutRef(inner) => format!("{}*", inner.to_cpp_string()),
            Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
                "const char*".to_string()
            }
            _ => self.to_cpp_string(),
        }
    }

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
```
</details>

So  at line 710 does NOT consider void pointers or raw pointers as unbindable. There's `needs_unsafe_fn()` at line 717, which detects them. But the skip reasons show `Standard_Address` as "unknown type", not "void pointer". This means the skipping is happening somewhere else — in the resolver's `is_known_type` check maybe.

Let me check how `is_method_bindable` works:

Searched for text `fn is_method_bindable` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:800](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L800)

<details>
<summary>Reading lines 800-930 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L800-L931">bindings.rs</a></summary>

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
    if resolver::method_needs_explicit_lifetimes(method) {
        return Err("returns &mut with reference params (ambiguous lifetimes)".to_string());
    }
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
            if let Some(deletable) = ctx.deletable_class_names {
                if !deletable.contains(name.as_str()) && !ctx.all_enums.contains(name.as_str()) {
                    return Err(format!("return type '{}' is not CppDeletable", name));
                }
            }
        }
        // MutRef to enum return type can't be bound — extern "C" expects int32_t& but C++ has EnumType&
        if ret
... (truncated, 6190 chars)
```
</details>

Now let me look at the `type_uses_unknown_type` function — that's what's rejecting `Standard_Address`:

Searched for text `fn type_uses_unknown_type` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:455](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L455)

<details>
<summary>Reading lines 455-530 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L455-L531">bindings.rs</a></summary>

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
        Type::Class(name) => !all_enums.contains(name) && name != "char",
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
        WrapperKind::ByValueReturn
    } else if has_cstring_param {
        WrapperKind::CStringParam
    } else if returns_cstring {
        WrapperKind::CStringReturn
    } else if resolver::has_const_mut_return_mismatch(method) {
        WrapperKind::ConstMutReturnFix
    } else if resolver::method_uses_enum(method, all_enums) {
```
</details>

Now let me look at `type_uses_unknown_class` in type_mapping.rs:

Searched for text `fn type_uses_unknown_class` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), 1 result
- [type_mapping.rs:365](../crates/opencascade-binding-generator/src/type_mapping.rs#L365)

<details>
<summary>Reading lines 365-430 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L365-L431">type_mapping.rs</a></summary>

```
pub fn type_uses_unknown_class(ty: &Type, all_classes: &std::collections::HashSet<String>) -> bool {
    match ty {
        Type::Handle(class_name) => !all_classes.contains(class_name),
        Type::Class(class_name) => {
            if all_classes.contains(class_name) {
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
    match ty {
        Type::Class(class_name) if class_name == "char" => {
            // C++ char resolved from canonical types (e.g., Standard_Character)
            RustTypeMapping {
                rust_type: "std::ffi::c_char".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
```
</details>

There it is! In  (line 368-382) and `type_uses_unknown_handle` (line 399-413), when the type is `Type::Class("Standard_Address")`, it checks `all_classes.contains("Standard_Address")` — and `Standard_Address` is NOT in `all_classes` (it's not a parsed class, it's a typedef for `void*`). Then it checks `is_primitive_type("Standard_Address")` — let me check if that returns true:

Searched for text `fn is_primitive_type` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), 1 result
- [rust.rs:112](../crates/opencascade-binding-generator/src/codegen/rust.rs#L112)

<details>
<summary>Reading lines 112-140 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L112-L141">rust.rs</a></summary>

```
pub fn is_primitive_type(name: &str) -> bool {
    matches!(
        name,
        // Rust primitive names
        "bool" | "i8" | "u8" | "i32" | "u32" | "u16" | "i64" | "u64" | "f32" | "f64" | "char" | "c_char" |
        "c_long" | "c_ulong" |
        // C++ primitive names (may appear from canonical type resolution)
        "double" | "float" | "int" | "unsigned int" | "long" | "unsigned long" |
        "long long" | "unsigned long long" | "short" | "unsigned short" |
        "signed char" | "unsigned char"
    )
}

// =============================================================================
// FFI MODULE GENERATION
// =============================================================================
//
// These functions generate the FFI module containing ALL types,
// plus per-module re-export files. This avoids cross-module type filtering
// issues and simplifies the architecture.

/// Generate the ffi.rs file containing ALL types from all modules
///
/// This generates extern "C" declarations with all types using full C++ names
/// (e.g., gp_Pnt, TopoDS_Shape) to avoid collisions and make the mapping obvious.
///
/// Returns the generated Rust code as a String.
pub fn generate_ffi(
    all_classes: &[&ParsedClass],
```
</details>

`Standard_Address` is NOT in the  list. So the flow is:

1. Parser sees `Standard_Address` → creates `Type::Class("Standard_Address")`
2.  checks: is it in `all_classes`? No. Is it a primitive? No. Is it nested? No. → Returns `true` (unknown)
3. Method gets skipped with "uses unknown type 'Standard_Address'"

The fix: `Standard_Address` needs to be treated as known in  and `type_uses_unknown_handle`. Since it's already handled in `type_to_rust_string()` (maps to `*mut std::ffi::c_void`), we just need to add it to the "known" check.

Let me also look at what `is_void_ptr()` does to understand the complete picture:

Searched for text `fn is_void_ptr` (`**/crates/opencascade-binding-generator/src/model.rs`), 1 result
- [model.rs:598](../crates/opencascade-binding-generator/src/model.rs#L598)

<details>
<summary>Reading lines 595-620 of <a href="../crates/opencascade-binding-generator/src/model.rs#L595-L621">model.rs</a></summary>

```

    /// Check if this is a Standard_Address (void*) type
    /// These can't be bound through the FFI
    pub fn is_void_ptr(&self) -> bool {
        match self {
            Type::Class(name) => name == "Standard_Address",
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_void_ptr()
            }
            _ => false,
        }
    }

    /// Check if this type is a C-style array (e.g., gp_Pnt[8])
    pub fn is_array(&self) -> bool {
        match self {
            Type::Class(name) => name.contains('[') && name.contains(']'),
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_array()
            }
            _ => false,
        }
    }

    /// Check if this type is a raw pointer (requires unsafe in FFI)
    /// Note: const char* is NOT considered a raw pointer here because we handle it specially
```
</details>

Now let me check the doc comment on  — it says "These can't be bound through the FFI", but looking at the  logic (line 709-711), void pointers are NOT unbindable. And `needs_unsafe_fn()` (line 717-719) says they need unsafe. So there's existing infrastructure to handle void pointers — they're supposed to be bound as `unsafe fn` with `*mut std::ffi::c_void`.

The problem is that  flags `Standard_Address` as unknown before we even get to the point of emitting it. The fix is straightforward: add `Standard_Address` to the known-type checks in  and `type_uses_unknown_handle`.

But wait — let me also check whether the `describe_unbindable_reason` function has a "void pointer" path that generates the old skip reason format. Looking at line 731, it does: `if ty.is_void_ptr() { return format!("void pointer ({})...") }`. But that path is only reached if  returns true (line 801-804 in is_method_bindable), and  doesn't include void pointers. So the "void pointer" reason is from the OLD code before these changes were made.

Wait, let me re-read the skip reason from the actual generated files:

```
Reason: return type 'Standard_Address' is unknown
Reason: param 'PtrSvSurfaces' uses unknown type 'Standard_Address'
```

These come from lines 852-853 and 822-823 in . They hit the  check, not the `describe_unbindable_reason` path.

So the fix: `Standard_Address` must be recognized as "known" in  and `type_uses_unknown_handle()`. Then the methods won't be skipped, and since `needs_unsafe_fn()` returns true for , they'll be emitted as `unsafe fn`.

Let me verify `needs_unsafe_fn` is actually used in the code generation:

Searched for text `needs_unsafe_fn` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results
- [bindings.rs:2967](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2967)
- [bindings.rs:2971](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2971)
- [bindings.rs:3540](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3540)
- [bindings.rs:3541](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3541)

Good — `needs_unsafe_fn()` is used to mark functions as `unsafe`. Let me check how it's used in context:

<details>
<summary>Reading lines 2955-2985 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2955-L2986">bindings.rs</a></summary>

```
                // If the method is const (&self) and returns a class pointer,
                // downgrade Option<&mut T> to Option<&T> to avoid unsound &self -> &mut T.
                if resolved_method.is_const {
                    if let Some(ref mut rt) = return_type {
                        if rt.is_class_ptr_return && rt.rust_reexport_type.starts_with("Option<&mut ") {
                            rt.rust_reexport_type = rt.rust_reexport_type.replace("Option<&mut ", "Option<&");
                        }
                    }
                }

                // Check if inherited method has unsafe types (raw pointers / void pointers)
                let is_unsafe = resolved_method.params.iter().any(|p| {
                    p.ty.original.needs_unsafe_fn()
                        && !p.is_nullable_ptr()
                        && p.ty.original.class_ptr_inner_name().is_none()
                }) || resolved_method.return_type.as_ref().map_or(false, |rt| {
                    rt.original.needs_unsafe_fn() && rt.original.class_ptr_inner_name().is_none()
                });

                result.push(InheritedMethodBinding {
                    ffi_fn_name,
                    impl_method_name,
                    is_const: resolved_method.is_const,
                    params,
                    return_type,
                    cpp_method_name: resolved_method.cpp_name.clone(),
                    source_class: ancestor_name.clone(),
                    source_header: ancestor_class.source_header.clone(),
                    source_line: resolved_method.source_line,
                    is_unsafe,
                });
```
</details>

Good. Now let me also understand the full codegen for `Standard_Address` — when it's a plain `Type::Class("Standard_Address")`, it gets mapped to `*mut std::ffi::c_void` in `type_to_rust_string()`. When it's e.g. `Type::ConstRef(Box::new(Type::Class("Standard_Address")))`, it becomes `&*mut std::ffi::c_void` which would be `&*mut c_void` — that doesn't make sense. Let me think about what forms `Standard_Address` appears in:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep "Reason:.*Standard_Address" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | sort | uniq -c | sort -rn
```
<details>
<summary>Output (36 lines)</summary>

```
  41 param 'S' uses unknown type 'Standard_Address'
  31 param 'C' uses unknown type 'Standard_Address'
  22 param 'PCurve' uses unknown type 'const Standard_Address&'
  18 return type 'Standard_Address' is unknown
  13 param 'C1' uses unknown type 'const Standard_Address&'
  11 param 'C' uses unknown type 'const Standard_Address&'
   8 param 'Surface' uses unknown type 'const Standard_Address&'
   7 param 's' uses unknown type 'Standard_Address'
   6 param 'str' uses unknown type 'Standard_Address'
   6 param 'A' uses unknown type 'Standard_Address'
   5 param 'S' uses unknown type 'const Standard_Address&'
   4 param 'thePtr' uses unknown type 'Standard_Address'
   3 param 'aModifContext' uses unknown type 'Standard_Address'
   3 param 'A1' uses unknown type 'Standard_Address'
   2 param 'theResult' uses unknown type 'Standard_Address&'
   2 param 'Tab' uses unknown type 'Standard_Address'
   2 param 'PtrSvSurfaces' uses unknown type 'Standard_Address'
   2 param 'Curve1' uses unknown type 'const Standard_Address&'
   2 param 'Curve' uses unknown type 'const Standard_Address&'
   2 param 'Addr' uses unknown type 'Standard_Address'
   1 return type 'const Standard_Address&' is unknown
   1 param 'TheParCurve' uses unknown type 'const Standard_Address&'
   1 param 'TheParCurev' uses unknown type 'const Standard_Address&'
   1 param 'theOwner' uses unknown type 'Standard_Address'
   1 param 'theBuffer' uses unknown type 'Standard_Address'
   1 param 'theAtt' uses unknown type 'Standard_Address'
   1 param 'PC' uses unknown type 'const Standard_Address&'
   1 param 'ParCurve' uses unknown type 'const Standard_Address&'
   1 param 'MeshEntity' uses unknown type 'Standard_Address'
   1 param 'F' uses unknown type 'Standard_Address'
   1 param 'E' uses unknown type 'Standard_Address'
   1 param 'data' uses unknown type 'Standard_Address'
   1 param 'curve1' uses unknown type 'const Standard_Address&'
   1 param 'Buffer' uses unknown type 'Standard_Address'
   1 param 'A' uses unknown type 'Standard_Address&'
   1 param 'a' uses unknown type 'Standard_Address'
```
</details>

So we have these forms:
- `Standard_Address` (plain `Type::Class("Standard_Address")`) — param or return
- `const Standard_Address&` — `Type::ConstRef(Box::new(Type::Class("Standard_Address")))` 
- `Standard_Address&` — `Type::MutRef(Box::new(Type::Class("Standard_Address")))`

For `void*`, a `const void*&` is a reference to a void pointer. In Rust FFI, this would be `&*mut c_void` for const ref, `&mut *mut c_void` for mut ref. Or more practically, `*const *mut c_void` and `*mut *mut c_void` in the extern "C" layer.

Actually, wait. Let me look at what C++ types these actually are. `Standard_Address` is `void*`. So:
- `Standard_Address` param = `void*` param → `*mut c_void` in Rust
- `const Standard_Address&` param = `const void*&` = reference to const void pointer → `*const *mut c_void` or `&*mut c_void`

Hmm, this is getting complicated. The `type_to_rust_string` function handles `Type::Class("Standard_Address")` → `*mut std::ffi::c_void`, and `Type::ConstRef` wraps it with `&`, giving `&*mut std::ffi::c_void`. That might or might not be correct but it would at least compile.

Actually, let me re-check — in the extern "C" layer, references are converted to pointers. So `const Standard_Address&` becomes `const Standard_Address*` in C, which is `const void**` — a pointer to a void pointer. In Rust that would be `*const *mut c_void`.

Let me look at how `to_cpp_extern_c_param` handles this:

The `to_cpp_extern_c_param` function (line 723-731) converts `ConstRef(inner)` to `const inner*`. So `ConstRef(Class("Standard_Address"))` becomes `const Standard_Address*`. In C++ this expands to `const void**` since `Standard_Address` is `void*`.

In the reexport layer, `type_to_rust_string` would produce `&*mut std::ffi::c_void` for `ConstRef(Class("Standard_Address"))`, which is an immutable reference to a mutable void pointer.

This all seems to work. The issue is just that  / `type_uses_unknown_handle()` reject `Standard_Address` before we ever get a chance to process it.

The fix is simple: add `Standard_Address` to the known-type checks alongside the "char"  / `type_uses_unknown_handle()` reject `Standard_Address` before we ever get a chance to process it.

The fix is simple: add `Standard_Address` to the known-type checks alongside the "char" primitive check. Let me also check what the `void*` (non-Standard_Address) skips look like:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep "Reason:.*unknown.*void" crates/opencascade-sys/generated/*.rs | head -10
```
<details>
<summary>Output (10 lines)</summary>

```
crates/opencascade-sys/generated/adv_app2_var.rs://   Reason: param 't' uses unknown type 'void*'
crates/opencascade-sys/generated/adv_app2_var.rs://   Reason: param 't' uses unknown type 'void*'
crates/opencascade-sys/generated/adv_app2_var.rs://   Reason: param 'tin' uses unknown type 'void*'
crates/opencascade-sys/generated/adv_app2_var.rs://   Reason: param 'adt' uses unknown type 'void*'
crates/opencascade-sys/generated/adv_app2_var.rs://   Reason: param 'adt' uses unknown type 'void*'
crates/opencascade-sys/generated/ais.rs://   Reason: param 'theCallbackUserPtr' uses unknown type 'void*'
crates/opencascade-sys/generated/aspect.rs://   Reason: param 'theTexture' uses unknown type 'void*'
crates/opencascade-sys/generated/aspect.rs://   Reason: param 'theTexture' uses unknown type 'void*'
crates/opencascade-sys/generated/graphic3d.rs://   Reason: param 'theCallbackUserPtr' uses unknown type 'void*'
crates/opencascade-sys/generated/n_collection.rs://   Reason: param 'theAddress' uses unknown type 'void*'
```
</details>

So there are also `void*` types coming through as `Type::Class("void*")` or similar. Let me check how parser produces these:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -c "unknown type 'void" crates/opencascade-sys/generated/*.rs | grep -v ':0$'
```
<details>
<summary>Output (6 lines)</summary>

```
crates/opencascade-sys/generated/adv_app2_var.rs:5
crates/opencascade-sys/generated/ais.rs:1
crates/opencascade-sys/generated/aspect.rs:2
crates/opencascade-sys/generated/graphic3d.rs:1
crates/opencascade-sys/generated/n_collection.rs:7
crates/opencascade-sys/generated/osd.rs:3
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep "unknown type 'void" crates/opencascade-sys/generated/*.rs | sed 's/.*unknown type //' | sort | uniq -c | sort -rn
```
```
  19 'void*'
```

So there are also 19 skips for `void*` (not `Standard_Address`, but literal `void*`). Let me check how the parser creates these:

Searched for regex `void\*|void_ptr|void pointer` (`**/crates/opencascade-binding-generator/src/parser.rs`), 2 results
- [parser.rs:1623](../crates/opencascade-binding-generator/src/parser.rs#L1623)
- [parser.rs:1624](../crates/opencascade-binding-generator/src/parser.rs#L1624)

<details>
<summary>Reading lines 1610-1640 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1610-L1641">parser.rs</a></summary>

```
        "int" => Some(Type::I32),
        "unsigned int" => Some(Type::U32),
        "long" => Some(Type::Long),
        "unsigned long" => Some(Type::ULong),
        "long long" => Some(Type::I64),
        "unsigned long long" => Some(Type::U64),
        "short" => Some(Type::I16),
        "int16_t" => Some(Type::I16),
        "unsigned short" | "uint16_t" => Some(Type::U16),
        "char16_t" => Some(Type::CHAR16),
        "unsigned char" | "uint8_t" | "Standard_Byte" | "Standard_Utf8UChar" => Some(Type::U8),
        "signed char" | "int8_t" => Some(Type::I8),
        "bool" => Some(Type::Bool),
        // Standard_Address is void* - can't be bound through the FFI, but we need to recognize it
        // so methods using it can be filtered out. Using a special class name that is_void_ptr() checks for.
        "Standard_Address" => Some(Type::Class("Standard_Address".to_string())),
        // Stream types - these can't be bound through the FFI
        "Standard_OStream" => Some(Type::Class("Standard_OStream".to_string())),
        "Standard_IStream" => Some(Type::Class("Standard_IStream".to_string())),
        "Standard_SStream" => Some(Type::Class("Standard_SStream".to_string())),
        _ => None,
    }
}

/// Add system C++ standard library include paths to clang arguments
/// 
/// libclang doesn't automatically include these paths, so we need to detect
/// and add them manually. This is platform-specific.
fn add_system_include_paths(args: &mut Vec<String>) {
    #[cfg(target_os = "macos")]
    {
```
</details>

I see the comments say "can't be bound through the FFI" and "so methods using it can be filtered out." That's the OLD intent — before the recent work that added `needs_unsafe_fn()` to handle void pointers as `unsafe fn`. Let me look at what happens when we have a literal `void*` (not from Standard_Address):

<details>
<summary>Searched for regex <code>void</code> (<code>**/crates/opencascade-binding-generator/src/parser.rs</code>), 5 results</summary>

- [parser.rs:933](../crates/opencascade-binding-generator/src/parser.rs#L933)
- [parser.rs:1256](../crates/opencascade-binding-generator/src/parser.rs#L1256)
- [parser.rs:1257](../crates/opencascade-binding-generator/src/parser.rs#L1257)
- [parser.rs:1623](../crates/opencascade-binding-generator/src/parser.rs#L1623)
- [parser.rs:1624](../crates/opencascade-binding-generator/src/parser.rs#L1624)
</details>

Let me look at how `void*` types from C++ get parsed. The raw `void*` would come through as a ConstPtr/MutPtr pointing type. Let me search for how pointer types are resolved in the parser:

<details>
<summary>Searched for regex <code>Pointer|pointer</code> (<code>**/crates/opencascade-binding-generator/src/parser.rs</code>), 6 results</summary>

- [parser.rs:1294](../crates/opencascade-binding-generator/src/parser.rs#L1294)
- [parser.rs:1400](../crates/opencascade-binding-generator/src/parser.rs#L1400)
- [parser.rs:1437](../crates/opencascade-binding-generator/src/parser.rs#L1437)
- [parser.rs:1438](../crates/opencascade-binding-generator/src/parser.rs#L1438)
- [parser.rs:1470](../crates/opencascade-binding-generator/src/parser.rs#L1470)
- [parser.rs:1471](../crates/opencascade-binding-generator/src/parser.rs#L1471)
</details>

<details>
<summary>Reading lines 1470-1500 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1470-L1501">parser.rs</a></summary>

```
    // Handle pointer types
    if kind == TypeKind::Pointer {
        if let Some(pointee) = clang_type.get_pointee_type() {
            let is_const = pointee.is_const_qualified();
            let inner = parse_type(&pointee);
            return if is_const {
                Type::ConstPtr(Box::new(inner))
            } else {
                Type::MutPtr(Box::new(inner))
            };
        }
    }

    // Handle Handle<T> types (opencascade::handle<T>)
    // Strip const prefix before checking
    let clean_spelling = spelling.trim_start_matches("const ").trim();
    if clean_spelling.starts_with("opencascade::handle<") || clean_spelling.starts_with("Handle(") {
        // Prefer the canonical type spelling for the inner type name, because
        // clang's display name may use unqualified names for nested classes
        // (e.g., "Curve" instead of "ShapePersistent_BRep::Curve") when the
        // Handle appears in a method within the parent class scope.
        let canonical = clang_type.get_canonical_type();
        let canonical_spelling = canonical.get_display_name();
        let clean_canonical = canonical_spelling.trim_start_matches("const ").trim();
        let inner = if clean_canonical.starts_with("opencascade::handle<") {
            extract_template_arg(clean_canonical)
        } else {
            extract_template_arg(clean_spelling)
        };
        return Type::Handle(inner);
    }
```
</details>

So for a `void*` param, clang would give  with pointee . Let me check what `parse_type` returns for :

Searched for regex `TypeKind::Void` (`**/crates/opencascade-binding-generator/src/parser.rs`), 1 result
- [parser.rs:1257](../crates/opencascade-binding-generator/src/parser.rs#L1257)

That's only checking return types. Let me look at `parse_type` more generally to see how it handles void:

Searched for text `fn parse_type` (`**/crates/opencascade-binding-generator/src/parser.rs`), 1 result
- [parser.rs:1265](../crates/opencascade-binding-generator/src/parser.rs#L1265)

<details>
<summary>Reading lines 1265-1360 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1265-L1361">parser.rs</a></summary>

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
    let clean_for_lookup = trimmed_spelling
        .trim_start_matches("const ")
        .trim_start_matches("struct ")
        .trim_start_matches("class ")
        .trim_start_matches("typename ")
        .trim();
    if clean_for_lookup.contains('<') && !clean_for_lookup.starts_with("opencascade::handle<") && !clean_for_lookup.starts_with("Handle(") {
        if let Some(typedef_name) = lookup_typedef(clean_for_lookup) {
            return Type::Class(typedef_name);
        }
    }


    // Get canonical type for resolving typedefs
    let canonical = clang_type.get_canonical_type();
    let canonical_spelling = canonical.get_display_name();
    
    // Strip const/volatile from canonical spelling for primitive matching
    let canonical_clean = canonical_spelling
        .trim()
        .trim_start_matches("const ")
        .trim_start_matches("volatile ")
        .trim();
    // Defense-in-depth: detect when clang's canonical type is a primitive (int, double, etc.)
    // but the display name clearly identifies a class/typedef. This can happen if a template
    // type fails to instantiate. Legitimate typedefs to primitives (e.g.,
    // `typedef unsigned int Poly_MeshPurpose`) use a typedef chain to a builtin type.
    let spelling_looks_like_class = {
        let s = trimmed_spelling
            .trim_start_matches("const ")
            .trim_start_matches("struct ")
            .trim_start_matches("class ")
            .trim_start_matches("typename ")
            .trim();
        let looks_like_class = s.starts_with(|c: char| c.is_ascii_uppercase())
            && map_standard_type(s).is_none()
            && s != "Standard_Boolean"
            && !s.contains('<')
            && !s.contains("::");

        if !looks_like_class {
            false
        } else {
            // Check if this is a typedef whose underlying type is a primitive.
            // If so, it's a genuine typedef-to-primitive (like Poly_MeshPurpose = unsigned int).
            // Note: clang wraps typedefs in Elaborated sugar, so check both Typedef and Elaborated kinds.
            let is_primitive_typedef = matches!(kind, TypeKind::Typedef | TypeKind::Elaborated)
                && clang_type.get_declaration()
                    .filter(|d| d.get_kind() == clang::EntityKind::TypedefDecl)
                    .and_then(|d| d.get_typedef_underlying_type())
                    .map(|u| matches!(u.get_kind(),
                
... (truncated, 4655 chars)
```
</details>

<details>
<summary>Reading lines 1360-1470 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1360-L1471">parser.rs</a></summary>

```
        }
    };

    // Handle primitives via canonical type.
    // Skip this if:
    // 1. The spelling clearly identifies a class type (spelling_looks_like_class), OR
    // 2. The spelling contains '<' or '::' — template or namespace-scoped types
    //    whose canonical resolves to int/double/etc. should not be treated as primitives.
    let spelling_is_template_or_namespaced = {
        let s = trimmed_spelling
            .trim_start_matches("const ")
            .trim_start_matches("volatile ")
            .trim();
        s.contains('<') || s.contains("::")
    };
    if !spelling_looks_like_class && !spelling_is_template_or_namespaced {

        match canonical_clean {
            "bool" => return Type::Bool,
            "int" => return Type::I32,
            "unsigned int" => return Type::U32,
            "unsigned short" | "uint16_t" => return Type::U16,
            "char16_t" => return Type::CHAR16,
            "unsigned char" | "uint8_t" => return Type::U8,
            "signed char" | "int8_t" => return Type::I8,
            "short" | "int16_t" => return Type::I16,
            "long" => return Type::Long,
            "unsigned long" => return Type::ULong,
            "long long" => return Type::I64,
            "unsigned long long" => return Type::U64,
            "float" => return Type::F32,
            "double" => return Type::F64,
            _ => {}
        }
    }

    // Guard: when the OUTER type's display name identifies an OCCT class but the
    // canonical type is "int", construct the class type directly instead of recursing
    // into the pointee (whose display name might already be "int", losing the
    // typedef info).
    if kind == TypeKind::LValueReference || kind == TypeKind::RValueReference || kind == TypeKind::Pointer

    {
        let canonical_base = canonical_clean
            .trim_end_matches(" &")
            .trim_end_matches(" &&")
            .trim_end_matches(" *")
            .trim();
        if canonical_base == "int" {
            // Strip qualifiers and ref/ptr decorators from the outer display name
            let base = trimmed_spelling
                .trim_start_matches("const ")
                .trim_start_matches("volatile ")
                .trim_start_matches("struct ")
                .trim_start_matches("class ")
                .trim_start_matches("typename ")
                .trim_end_matches('&')
                .trim_end_matches('*')
                .trim();
            let base_looks_like_class = base.starts_with(|c: char| c.is_ascii_uppercase())
                && map_standard_type(base).is_none()
                && base != "Standard_Boolean"
                && !base.contains(' ');
            // Also handle template/namespaced types (e.g. "NCollection_Map<...>" or
            // "IMeshData::IMapOfReal") — these are clearly not primitives.
            let base_looks_like_type = base_looks_like_class
                || base.contains('<')
                || base.contains("::");
            if base_looks_like_type {

                let inner = Type::Class(base.to_string());
                if let Some(pointee) = clang_type.get_pointee_type() {
                    let is_const = pointee.is_const_qualified();
                    return match kind {
                        TypeKind::LValueReference if is_const => Type::ConstRef(Box::new(inner)),
                        TypeKind::LValueReference => Type::MutRef(Box::new(inner)),
                        TypeKind::RValueReference => Type::RValueRef(Box::new(inner)),
                        TypeKind::Pointer if is_const => Type::ConstPtr(Box::new(inner)),
                        TypeKind::Pointer => Type::MutPtr(Box::new(inner)),
                        _ => inner,
                    };
                }
                return inner;
            }
        }
    }


    // Handle reference types

    if kind == TypeKind::LValueReference {
        if let Some(pointee) = clang_type.get_pointee_type() {
            let i
... (truncated, 4599 chars)
```
</details>

So for a raw `void*` param: clang gives .  recursively calls itself for the inner pointee. But there's no check for  in  — what falls through?

<details>
<summary>Reading lines 1500-1570 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1500-L1571">parser.rs</a></summary>

```
    }

    // For nested types (e.g., TColgp_Array1OfPnt::value_type) or template types,
    // use the canonical type to get the resolved underlying type.
    // clang resolves these for us (e.g., value_type -> gp_Pnt)
    let clean_name = spelling
        .trim_start_matches("const ")
        .trim_start_matches("class ")
        .trim_start_matches("struct ")
        .trim_start_matches("typename ")
        .trim_start_matches("enum ")
        .trim_end_matches(" &")
        .trim_end_matches(" *")
        .trim();
    
    // If the spelling contains :: or < (nested/template type), try typedef map first,
    // then try to use canonical
    if clean_name.contains("::") || clean_name.contains('<') {
        // For template types, check if this is a known typedef
        if clean_name.contains('<') {
            if let Some(typedef_name) = lookup_typedef(clean_name) {
                return Type::Class(typedef_name);
            }
        }
        let canonical_clean = canonical_spelling

            .trim_start_matches("const ")
            .trim_start_matches("class ")
            .trim_start_matches("struct ")
            .trim_start_matches("enum ")
            .trim_end_matches(" &")
            .trim_end_matches(" *")
            .trim();
        
        // Only use canonical if it's simpler (no :: or <) AND still looks like a class name.
        // If canonical is a primitive like "int", that would produce Type::Class("int")
        // which is nonsensical. By keeping the template/namespaced spelling,
        // type_uses_unknown_type() will properly filter methods with unresolvable types.
        let canonical_looks_like_class = canonical_clean
            .starts_with(|c: char| c.is_ascii_uppercase());
        if !canonical_clean.contains("::") && !canonical_clean.contains('<') && !canonical_clean.is_empty() && canonical_looks_like_class {
            return Type::Class(canonical_clean.to_string());
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

    Type::Class(clean_name.to_string())
}

/// Extract template argument from Handle<T> or similar
fn extract_template_arg(type_name: &str) -> String {
    if type_name.starts_with("Handle(") {
        // Handle(Foo) format
```
</details>

I see. So when  is called for a `void` pointee type (from a `void*`):
-  is 
- `spelling` is `"void"`
- `map_standard_type("void")` returns `None` (it doesn't map "void")
- It falls through everything
- Eventually reaches  at line 1564

So `void*` becomes `MutPtr(Class("void"))`. And  then sees `Class("void")` — which isn't in `all_classes`, isn't a primitive type, and isn't nested → returns `true` (unknown).

Now I understand the full picture. The fix needs to handle both:
1.  then sees `Class("void")` — which isn't in `all_classes`, isn't a primitive type, and isn't nested → returns `true` (unknown).

Now I understand the full picture. The fix needs to handle both:
1. `Standard_Address` —  
2. `void*` —  or 

Both should be recognized as known types and map through as void pointers.

Let me now look at how  handles these:

Looking at line 598-606:  only checks . It does NOT check for . So `MutPtr(Class("void"))` won't be caught by  either.

Let me also update  to catch both forms. Now let me implement the fix:

**Todo list:**
- ✅ Commit current README changes
- ✅ Find recent pointer-related commits
- ✅ Investigate void pointer skips
- 🔄 Fix void pointer handling
- ⬜ Rebuild and test
- ⬜ Commit fix

The plan:
1. In `crates/opencascade-binding-generator/src/model.rs`: Update  to also recognize 
2. In `crates/opencascade-binding-generator/src/type_mapping.rs`: In both  and `type_uses_unknown_handle()`, add `"Standard_Address" | "void"` alongside "char" as known Class names
3. In `type_to_rust_string()` and `return_type_to_rust_string()`: Make sure `void` maps to appropriate ptr type
4. Update the parser comment that says "can't be bound"

Let me also check `describe_unbindable_reason` to make sure the old void pointer path still works even though  won't fire for these:

Looking at the code flow again:
-  (line 709) does NOT include void ptrs — correct, they ARE bindable
-  (line 800): first checks `has_unbindable_types()` (which calls  on all params/return), then checks . The unknown type check is what's rejecting Standard_Address.

So the fix is in  / `type_uses_unknown_handle()` — recognize `Standard_Address` and `void` as known types. Let me also double-check what `type_to_rust_string` does for :

Looking at line 606-616 in bindings.rs:
```rust
Type::Class(name) => {
    if name == "char" {
        "std::ffi::c_char".to_string()
    } else if name == "Standard_Address" {
        "*mut std::ffi::c_void".to_string()
    } else if let Some(ctx) = reexport_ctx {
        ...
```

It handles "Standard_Address" but NOT "void". For `MutPtr(Class("void"))`, it would produce `*mut crate::ffi::void` which doesn't exist. I need to also handle `Class("void")` → `std::ffi::c_void` so that `MutPtr(Class("void"))` → `*mut std::ffi::c_void`.

Let me also check what happens in the C++ wrapper layer — `to_cpp_extern_c_param` for `Standard_Address`:

<details>
<summary>Reading lines 85-105 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L85-L106">cpp.rs</a></summary>

```
        // But allow class raw pointers — they're bindable as &T / &mut T
        if ty.is_unbindable() && ty.class_ptr_inner_name().is_none() {
            return;
        }

        match ty {
            Type::Class(name) => {
                // Skip primitive types that don't have headers
                // Also skip Standard_Address which is defined in Standard_TypeDef.hxx, not its own file
                if matches!(name.as_str(), 
                    "bool" | "char" | "int" | "unsigned" | "float" | "double" | 
                    "void" | "size_t" | "Standard_Address"
                ) {
                    return;
                }
                // For nested types (Parent::Nested), include the parent class header
                if let Some(parent) = name.split("::").next() {
                    if name.contains("::") {
                        // Nested type — include the parent's header
                        if parent.contains('_') || parent.starts_with("Standard") {
                            let header = format!("{}.hxx", parent);
```
</details>

Good — C++ side already handles `"void"` and `"Standard_Address"`. Now let me also check the cpp.rs wrapper generation for how `Standard_Address` params/returns get their C++ type:

Searched for text `Standard_Address` (`**/crates/opencascade-binding-generator/src/codegen/cpp.rs`), 2 results
- [cpp.rs:93](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L93)
- [cpp.rs:96](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L96)

OK, `crates/opencascade-binding-generator/src/codegen/cpp.rs` just skips the header inclusion — good. The actual C++ type string comes from `to_cpp_string()` on the Type. For , that returns `"Standard_Address"`, which will work in C++ since it's a typedef for `void*`.

For , `to_cpp_string()` would return `"void"`. So `MutPtr(Class("void"))` → `"void*"` in C++, which is fine.

Now let me also check the `return_type_to_rust_string` and `type_to_rust_string` in the `crates/opencascade-binding-generator/src/codegen/cpp.rs` for the FFI layer:

Searched for text `fn ffi_fn_type_string` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Let me check what function generates the FFI type string for params/returns:

Searched for regex `fn.*ffi_type|fn.*rust_ffi|to_rust_ffi` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Let me look at how the FFI declarations are generated. There should be code that converts types to FFI strings. Let me search for where `ffi.rs` content is generated:

Searched for regex `fn type_to_ffi|ffi_param_type|ffi_return_type|fn.*_ffi_type` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

<details>
<summary>Searched for regex <code>ffi_safe|ffi.*type|extern.*C.*fn</code> (<code>**/crates/opencascade-binding-generator/src/codegen/**</code>), 20 results</summary>

- [bindings.rs:309](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L309)
- [bindings.rs:310](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L310)
- [bindings.rs:332](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L332)
- [bindings.rs:353](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L353)
- [bindings.rs:354](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L354)
- [bindings.rs:373](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L373)
- [bindings.rs:568](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L568)
- [bindings.rs:614](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L614)
- [bindings.rs:617](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L617)
- [bindings.rs:641](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L641)
- [bindings.rs:660](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L660)
- [bindings.rs:672](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L672)
- [bindings.rs:841](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L841)
- [bindings.rs:1028](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1028)
- [bindings.rs:1036](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1036)
</details>

Let me look at `build_param_binding` to see how FFI types are generated:

<details>
<summary>Reading lines 1028-1130 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1028-L1131">bindings.rs</a></summary>

```
fn build_param_binding(name: &str, ty: &Type, is_nullable: bool, ffi_ctx: &TypeContext, reexport_ctx: Option<&ReexportTypeContext>) -> ParamBinding {
    let cpp_name = name.to_string();
    let rust_name = safe_param_name(name);

    // Check for &mut enum output params — these need special local var + writeback handling
    if let Type::MutRef(inner) = ty {
        if let Type::Class(enum_name) = inner.as_ref() {
            if ffi_ctx.all_enums.contains(enum_name) {
                let enum_rust_type = ffi_ctx.enum_rust_types
                    .and_then(|map| map.get(enum_name))
                    .cloned();
                let reexport_type = enum_rust_type.as_ref()
                    .map(|t| format!("&mut {}", t))
                    .unwrap_or_else(|| "&mut i32".to_string());
                return ParamBinding {
                    cpp_name,
                    rust_name,
                    rust_ffi_type: "&mut i32".to_string(),
                    rust_reexport_type: reexport_type,
                    cpp_type: "int32_t&".to_string(),
                    // The arg expression uses the local variable name (preamble creates it)
                    cpp_arg_expr: format!("{}_enum_", name),
                    // No value enum conversion at Rust level
                    enum_rust_type: None,
                    mut_ref_enum_cpp_name: Some(enum_name.clone()),
                    mut_ref_enum_rust_type: enum_rust_type,
                    is_nullable_ptr: false,
                    is_class_ptr: false,
                };
            }
        }
    }

    // Check if this parameter is an enum type (by value or const ref)
    if let Some(enum_cpp_name) = extract_enum_name(ty, ffi_ctx.all_enums) {
        // Look up the Rust enum type for value enums
        let enum_rust_type = ffi_ctx.enum_rust_types
            .and_then(|map| map.get(&enum_cpp_name))
            .cloned();
        let reexport_type = enum_rust_type.clone().unwrap_or_else(|| "i32".to_string());
        return ParamBinding {
            cpp_name,
            rust_name,
            rust_ffi_type: "i32".to_string(),
            rust_reexport_type: reexport_type,
            cpp_type: "int32_t".to_string(),
            cpp_arg_expr: format!("static_cast<{}>({})", enum_cpp_name, name),
            enum_rust_type,
            mut_ref_enum_cpp_name: None,
            mut_ref_enum_rust_type: None,
            is_nullable_ptr: false,
            is_class_ptr: false,
        };
    }

    // Nullable pointer params: const T* = NULL -> Option<&T>, T* = NULL -> Option<&mut T>
    // In ffi.rs: *const T / *mut T (raw pointers, nullable)
    // In re-export: Option<&T> / Option<&mut T>
    // In C++: const T* / T* (passed through directly)
    if is_nullable {
        let (rust_ffi_type, rust_reexport_type, cpp_type, cpp_arg_expr) = match ty {
            Type::ConstPtr(inner) => {
                let inner_rust = type_to_rust_string(inner, reexport_ctx);
                let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
                let cpp_inner = inner.to_cpp_string();
                (
                    format!("*const {}", inner_ffi),
                    format!("Option<&{}>", inner_rust),
                    format!("const {}*", cpp_inner),
                    name.to_string(),
                )
            }
            Type::MutPtr(inner) => {
                let inner_rust = type_to_rust_string(inner, reexport_ctx);
                let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
                let cpp_inner = inner.to_cpp_string();
                (
                    format!("*mut {}", inner_ffi),
                    format!("Option<&mut {}>", inner_rust),
                    format!("{}*", cpp_inner),
                    name.to_string(),
                )
            }
            _ => unreachable!("is_nullable_ptr() returned true for non-pointer type"),
        };
        return ParamBinding {
            cpp_name,
   
... (truncated, 4484 chars)
```
</details>

Let me look at where the standard param binding is built (no special cases):

<details>
<summary>Reading lines 1130-1230 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1130-L1231">bindings.rs</a></summary>

```
    // In re-export: &T / &mut T
    // In C++: const T* / T* (passed through directly)
    if let Some(_class_name) = ty.class_ptr_inner_name() {
        let (rust_ffi_type, rust_reexport_type, cpp_type, cpp_arg_expr) = match ty {
            Type::ConstPtr(inner) => {
                let inner_rust = type_to_rust_string(inner, reexport_ctx);
                let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
                let cpp_inner = inner.to_cpp_string();
                (
                    format!("*const {}", inner_ffi),
                    format!("&{}", inner_rust),
                    format!("const {}*", cpp_inner),
                    name.to_string(),
                )
            }
            Type::MutPtr(inner) => {
                let inner_rust = type_to_rust_string(inner, reexport_ctx);
                let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
                let cpp_inner = inner.to_cpp_string();
                (
                    format!("*mut {}", inner_ffi),
                    format!("&mut {}", inner_rust),
                    format!("{}*", cpp_inner),
                    name.to_string(),
                )
            }
            _ => unreachable!("class_ptr_inner_name() returned Some for non-pointer type"),
        };
        return ParamBinding {
            cpp_name,
            rust_name,
            rust_ffi_type,
            rust_reexport_type,
            cpp_type,
            cpp_arg_expr,
            enum_rust_type: None,
            mut_ref_enum_cpp_name: None,
            mut_ref_enum_rust_type: None,
            is_nullable_ptr: false,
            is_class_ptr: true,
        };
    }

    // By-value class/handle params — opaque types
    // must be passed by reference. We convert them to const T& at the FFI
    // boundary; the C++ wrapper passes the reference to the original method
    // which accepts by value (C++ handles the implicit copy).
    let effective_ty = match ty {
        Type::Class(name) if name != "char" && !ffi_ctx.all_enums.contains(name) => {
            Type::ConstRef(Box::new(ty.clone()))
        }
        Type::Handle(_) => {
            Type::ConstRef(Box::new(ty.clone()))
        }
        _ => ty.clone(),
    };

    let mapped = map_type_in_context(&effective_ty, ffi_ctx);
    let rust_ffi_type = mapped.rust_type;
    let rust_reexport_type = type_to_rust_string(&effective_ty, reexport_ctx);
    let cpp_type = effective_ty.to_cpp_extern_c_param();
    let cpp_arg_expr = param_to_cpp_extern_c_arg(name, &effective_ty);

    ParamBinding {
        cpp_name,
        rust_name,
        rust_ffi_type,
        rust_reexport_type,
        cpp_type,
        cpp_arg_expr,
        enum_rust_type: None,
        mut_ref_enum_cpp_name: None,
        mut_ref_enum_rust_type: None,
        is_nullable_ptr: false,
        is_class_ptr: false,
    }
}

fn build_return_type_binding(ty: &Type, ffi_ctx: &TypeContext, reexport_ctx: Option<&ReexportTypeContext>) -> ReturnTypeBinding {
    // Check if this return type is an enum
    if let Some(enum_cpp_name) = extract_enum_name(ty, ffi_ctx.all_enums) {
        let enum_rust_type = ffi_ctx.enum_rust_types
            .and_then(|map| map.get(&enum_cpp_name))
            .cloned();
        let reexport_type = enum_rust_type.clone().unwrap_or_else(|| "i32".to_string());
        return ReturnTypeBinding {
            rust_ffi_type: "i32".to_string(),
            rust_reexport_type: reexport_type,
            cpp_type: "int32_t".to_string(),
            needs_unique_ptr: false,
            enum_cpp_name: Some(enum_cpp_name),
            enum_rust_type,
            is_class_ptr_return: false,
        };
    }

    // Check if this return type is a class pointer (const T* or T* where T is a known class)
    // These are bound as Option<&T> / Option<&mut T> since they may return null.
    if let Some(class_name) = ty.class_ptr_inner_name() {
        let mapped = map_return_type_in_context(ty, ffi_ctx)
... (truncated, 4047 chars)
```
</details>

Looking at line 1177-1184: when we have  and `name != "char"` and it's not an enum, it gets wrapped in `ConstRef`. So for , it would become `ConstRef(Class("Standard_Address"))` which means the C++ gets `const Standard_Address*` — that's `const void**`. That's not right for a void pointer parameter.

Wait, but Standard_Address is already `void*`. So as a param it's pass-by-value `void*`. The Rust side would be `*mut std::ffi::c_void`. But the effective_ty conversion wraps it in `ConstRef`, making it `const void**` on the C++ side and `&*mut std::ffi::c_void` on the Rust side. That would be wrong.

Actually wait — I need to also check , it would become `ConstRef(Class("Standard_Address"))` which means the C++ gets `const Standard_Address*` — that's `const void**`. That's not right for a void pointer parameter.

Wait, but Standard_Address is already `void*`. So as a param it's pass-by-value `void*`. The Rust side would be `*mut std::ffi::c_void`. But the effective_ty conversion wraps it in `ConstRef`, making it `const void**` on the C++ side and `&*mut std::ffi::c_void` on the Rust side. That would be wrong.

Actually wait — I need to also check `class_ptr_inner_name()` — it might handle this:

Searched for text `fn class_ptr_inner_name` (`**/crates/opencascade-binding-generator/src/model.rs`), 1 result
- [model.rs:639](../crates/opencascade-binding-generator/src/model.rs#L639)

<details>
<summary>Reading lines 639-670 of <a href="../crates/opencascade-binding-generator/src/model.rs#L639-L671">model.rs</a></summary>

```
    pub fn class_ptr_inner_name(&self) -> Option<&str> {
        match self {
            Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                match inner.as_ref() {
                    Type::Class(name) if name != "char" => Some(name.as_str()),
                    _ => None,
                }
            }
            _ => None,
        }
    }


    /// Similar to `Param::is_nullable_ptr()` but operates on a bare `Type` without
    /// requiring a `has_default` check. Excludes `const char*` (handled as strings).
    pub fn is_nullable_ptr(&self) -> bool {
        match self {
            Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => false,
            Type::ConstPtr(_) | Type::MutPtr(_) => true,
            _ => false,
        }
    }

    /// Check if this type is a nested/qualified type (e.g., SomeClass::value_type) or template type
    /// that couldn't be resolved to a simple type name.
    pub fn is_nested_type(&self) -> bool {
        match self {
            Type::Class(name) => {
                // Explicit nested type indicators
                if name.contains("::") || name.contains('<') || name.contains('>') {
                    return true;
                }
```
</details>

At line 643:  matches  where `name != "char"`. So  and `MutPtr(Class("void"))` would match, returning `Some("Standard_Address")` or `Some("void")`.

But since `Standard_Address` comes as a bare  (not wrapped in a pointer),  would return `None` for it. And `void*` comes as `MutPtr(Class("void"))`, so  returns `Some("void")`. That means `void*` params would go through the class pointer binding path (line 1132), which produces `*mut crate::ffi::void` in the FFI and `&mut crate::ffi::void` in the reexport — both wrong since there's no `void` FFI type.

OK, I need to think about this more carefully. The `Standard_Address` and `void*` cases need special handling in several places. Let me think about the right approach:

**For  (a bare void pointer, since `Standard_Address` IS `void*`):**
- FFI type: `*mut std::ffi::c_void`
- Reexport type: `*mut std::ffi::c_void`
- C++ type: `Standard_Address` (which is `void*`)
- This should NOT be wrapped in ConstRef like other by-value class params

**For `MutPtr(Class("void"))` (explicit `void*`):**
- FFI type: `*mut std::ffi::c_void`
- Reexport type: `*mut std::ffi::c_void`
- C++ type: `void*`

**For `ConstRef(Class("Standard_Address"))` (a `const void*&`):**
- This is a reference to a void pointer. Probably rare and unusual but exists.
- FFI type: `*const *mut std::ffi::c_void` (pointer to void pointer)
- Actually, in FFI, references become pointers, so `const void*&` → `const void**` → `*const *mut c_void`

This is getting complex. Let me take a simpler approach: recognize `Standard_Address` and `void` as special "known" types that don't need by-value-class treatment, and make sure the type mapping functions handle them correctly.

Here's my plan:
1. **`crates/opencascade-binding-generator/src/model.rs`**: Update  to also match 
2. **`crates/opencascade-binding-generator/src/type_mapping.rs`**: In  and `type_uses_unknown_handle()`, treat `"Standard_Address"` and `"void"` as known (like "char")
3. **`crates/opencascade-binding-generator/src/codegen/bindings.rs` `type_to_rust_string()`**: Add `"void"` case mapping to `std::ffi::c_void`
4. **`crates/opencascade-binding-generator/src/codegen/bindings.rs` `build_param_binding()`**: Exclude `"Standard_Address"` and `"void"` from the by-value class→ConstRef conversion (line 1178)
5. **`crates/opencascade-binding-generator/src/codegen/bindings.rs`**: Also exclude `"Standard_Address"` and `"void"` so `MutPtr(Class("void"))` isn't treated as a class pointer
6. **`crates/opencascade-binding-generator/src/codegen/bindings.rs` `return_type_to_rust_string()`**: Handle `void` case
7. **`crates/opencascade-binding-generator/src/parser.rs`**: Update comment

Let me also check  exclusion for Standard_Address:

Looking at line 667 of bindings.rs:  — Standard_Address IS already excluded from the MutPtr return type path but NOT from class_ptr_inner_name.

Let me also check what `has_by_value_class_or_handle_params` does:

At line 479:  — doesn't exclude `Standard_Address` or `void`.

Let me also look at `map_type_in_context` in `crates/opencascade-binding-generator/src/type_mapping.rs` to see how it handles these:

<details>
<summary>Reading lines 180-475 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L180-L476">bindings.rs</a></summary>

```
pub struct WrapperMethodBinding {
    /// FFI function name (full, e.g. "gp_Pnt_mirrored_pnt")
    pub ffi_fn_name: String,
    /// Method name in re-export impl block (may differ from ffi base if name conflict)
    pub impl_method_name: String,
    /// Whether this is a const method
    pub is_const: bool,
    /// Parameters (excluding self)
    pub params: Vec<ParamBinding>,
    /// Return type
    pub return_type: Option<ReturnTypeBinding>,
    /// What kind of wrapper is needed
    pub wrapper_kind: WrapperKind,
    /// Original C++ method name
    pub cpp_method_name: String,
    /// Doc comment
    pub doc_comment: Option<String>,
    /// Source line in C++ header
    pub source_line: Option<u32>,
    /// Whether this method should be marked `unsafe fn` (has raw pointer params/returns)
    pub is_unsafe: bool,
}

/// A static method binding.
#[derive(Debug, Clone)]
pub struct StaticMethodBinding {
    /// FFI function name (full, e.g. "gp_Pnt_origin_static")
    pub ffi_fn_name: String,
    /// Method name in re-export impl block (may differ for instance/static conflicts)
    pub impl_method_name: String,
    /// Parameters
    pub params: Vec<ParamBinding>,
    /// Return type
    pub return_type: Option<ReturnTypeBinding>,
    /// Original C++ method name
    pub cpp_method_name: String,
    /// Whether reference returns need 'static lifetime
    pub needs_static_lifetime: bool,
    /// Doc comment
    pub doc_comment: Option<String>,
    /// Source line in C++ header
    pub source_line: Option<u32>,
    /// Whether this method should be marked `unsafe fn` (has raw pointer params/returns)
    pub is_unsafe: bool,
}

/// An upcast binding (Derived → Base).
#[derive(Debug, Clone)]
pub struct UpcastBinding {
    /// Base class FFI-safe name ("::" replaced with "_"), e.g. "Geom_Curve"
    pub base_class: String,
    /// Base class C++ qualified name (uses "::"), e.g. "Geom_Curve" or "Outer::Inner"
    pub base_class_cpp: String,
    /// Base class short name, e.g. "Curve"
    pub base_short_name: String,
    /// Base class module, e.g. "Geom"
    pub base_module: String,
    /// FFI function name for const upcast, e.g. "Geom_BSplineCurve_as_Geom_Curve"
    pub ffi_fn_name: String,
    /// FFI function name for mutable upcast
    pub ffi_fn_name_mut: String,
    /// Impl method name in re-export, e.g. "as_geom_curve" or "as_curve"
    pub impl_method_name: String,
}

/// A Handle upcast binding (Handle<Derived> → Handle<Base>).
#[derive(Debug, Clone)]
pub struct HandleUpcastBinding {
    /// Base handle type name, e.g. "HandleGeomCurve"
    pub base_handle_name: String,
    /// Base class C++ name, e.g. "Geom_Curve"
    pub base_class: String,
    /// Base class module, e.g. "Geom"
    pub base_module: String,
    /// FFI function name
    pub ffi_fn_name: String,
    /// Derived handle type name, e.g. "HandleGeomBSplineCurve"
    pub derived_handle_name: String,
}

/// A Handle downcast binding (Handle<Base> → Option<Handle<Derived>> via DownCast).
#[derive(Debug, Clone)]
pub struct HandleDowncastBinding {
    /// Derived handle type name, e.g. "HandleGeomPlane"
    pub derived_handle_name: String,
    /// Derived class C++ name, e.g. "Geom_Plane"
    pub derived_class: String,
    /// Derived class module, e.g. "Geom"
    pub derived_module: String,
    /// Base handle type name, e.g. "HandleGeomSurface"
    pub base_handle_name: String,
    /// FFI function name, e.g. "HandleGeomSurface_downcast_to_HandleGeomPlane"
    pub ffi_fn_name: String,
}


/// An inherited method from an ancestor class.
#[derive(Debug, Clone)]
pub struct InheritedMethodBinding {
    /// FFI function name, e.g. "Geom_BSplineCurve_inherited_Continuity"
    pub ffi_fn_name: String,
    /// Impl method name in re-export
    pub impl_method_name: String,
    /// Whether this is a const method
    pub is_const: bool,
    /// Parameters (resolved types from ancestor)
    pub params: Vec<ResolvedParamBinding>,
    /// Return type (resolved from ancestor
... (truncated, 11991 chars)
```
</details>

Let me look at `map_type_in_context`:

<details>
<summary>Reading lines 422-480 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L422-L481">type_mapping.rs</a></summary>

```
/// Map a type to Rust, using short names for same-module types
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
```
</details>

I see: `map_type_in_context` for  falls through to the general  case (line 434), which doesn't handle it specially — it would try to look up module and use it as an FFI class name. That's wrong.

And for , same issue — it would produce `void` as an FFI type name, which doesn't exist.

OK, I have a complete picture now. Let me implement all the changes: Now let me implement all changes. I need to be systematic about this. Let me define a helper concept: `is_void_type_name(name)` which returns true for `"Standard_Address"` or `"void"`. I need to apply this across multiple files.

Let me start with the edits:

**1. `crates/opencascade-binding-generator/src/model.rs`** - Update  to also match `"void"`

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L596-L598">model.rs -3+2</a>, <a href="../crates/opencascade-binding-generator/src/model.rs#L599-L600">-1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "596:vk,597:cx,598:bo",
      "content": "    /// Check if this is a void pointer type (Standard_Address = void*, or literal void*)\n    /// Methods with these types are bound as `unsafe fn` with `*mut c_void` types."
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "600:dh",
      "content": "            Type::Class(name) => name == \"Standard_Address\" || name == \"void\","
    }
  ]
}
```
**Output:**
```
{
  "applied": 2,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/model.rs",
      "startLine": 596,
      "endLine": 599
    }
  ]
}
```
</details>

**2. `crates/opencascade-binding-generator/src/model.rs`** - Update `class_ptr_inner_name()` to exclude void types

<details>
<summary>Reading lines 639-650 of <a href="../crates/opencascade-binding-generator/src/model.rs#L639-L651">model.rs</a></summary>

```
        match self {
            Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                match inner.as_ref() {
                    Type::Class(name) if name != "char" => Some(name.as_str()),
                    _ => None,
                }
            }
            _ => None,
        }
    }


```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L642-L643">model.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "                    Type::Class(name) if name != \"char\" && name != \"Standard_Address\" && name != \"void\" => Some(name.as_str()),",
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "642:tv"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/model.rs","startLine":642,"endLine":642}
```
</details>

**3. `crates/opencascade-binding-generator/src/type_mapping.rs` and `type_uses_unknown_handle()`

<details>
<summary>Reading lines 365-420 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L365-L421">type_mapping.rs</a></summary>

```
pub fn type_uses_unknown_class(ty: &Type, all_classes: &std::collections::HashSet<String>) -> bool {
    match ty {
        Type::Handle(class_name) => !all_classes.contains(class_name),
        Type::Class(class_name) => {
            if all_classes.contains(class_name) {
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

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L372-L377">type_mapping.rs -1+5</a>, <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L407-L412">-1+5</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            // Void pointer types — Standard_Address (void*) and literal \"void\" — are known\n            if class_name == \"Standard_Address\" || class_name == \"void\" {\n                return false;\n            }\n            // Primitive types mapped as Type::Class (e.g., \"char\" from Standard_Character)",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "372:qi"
    },
    {
      "content": "            // Void pointer types — Standard_Address (void*) and literal \"void\" — are known\n            if class_name == \"Standard_Address\" || class_name == \"void\" {\n                return false;\n            }\n            // Primitive types mapped as Type::Class (e.g., \"char\" from Standard_Character)",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "403:qi"
    }
  ]
}
```
**Output:**
```
{
  "applied": 2,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "startLine": 372,
      "endLine": 411
    }
  ]
}
```
</details>

**4. `crates/opencascade-binding-generator/src/type_mapping.rs`** - Update `map_type_in_context()` to handle Standard_Address and void

<details>
<summary>Reading lines 426-445 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L426-L446">type_mapping.rs</a></summary>

```
        _ => false,
    }
}

/// Map a type to Rust, using short names for same-module types
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
        Type::Class(class_name) => {
            // Enums are passed as i32 at the FFI boundary (integer pass-through)
            if ctx.all_enums.contains(class_name) {
                return RustTypeMapping {
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L433-L451">type_mapping.rs -9+18</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::Class(class_name) if class_name == \"char\" => {\n            // C++ char resolved from canonical types (e.g., Standard_Character)\n            RustTypeMapping {\n                rust_type: \"std::ffi::c_char\".to_string(),\n                needs_unique_ptr: false,\n                needs_pin: false,\n                source_module: None,\n            }\n        }\n        Type::Class(class_name) if class_name == \"Standard_Address\" || class_name == \"void\" => {\n            // void pointer types — map to *mut c_void\n            RustTypeMapping {\n                rust_type: \"*mut std::ffi::c_void\".to_string(),\n                needs_unique_ptr: false,\n                needs_pin: false,\n                source_module: None,\n            }\n        }",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "433:hh,434:da,435:mu,436:ik,437:wh,438:rl,439:nk,440:uo,441:cy"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/type_mapping.rs","startLine":433,"endLine":450}
```
</details>

**5. `crates/opencascade-binding-generator/src/codegen/bindings.rs`** - Update `type_to_rust_string()` to handle "void"

<details>
<summary>Reading lines 606-618 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L606-L619">bindings.rs</a></summary>

```
        Type::Class(name) => {
            if name == "char" {
                "std::ffi::c_char".to_string()
            } else if name == "Standard_Address" {
                "*mut std::ffi::c_void".to_string()
            } else if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            }
        }
        Type::Handle(name) => format!("crate::ffi::{}", type_mapping::handle_type_name(name)),
        Type::ConstRef(inner) => format!("&{}", type_to_rust_string(inner, reexport_ctx)),
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L609-L610">bindings.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            } else if name == \"Standard_Address\" || name == \"void\" {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "609:sq"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":609,"endLine":609}
```
</details>

**6. `crates/opencascade-binding-generator/src/codegen/bindings.rs`** - Update `return_type_to_rust_string()` to exclude void types from OwnedPtr and class pointer paths

<details>
<summary>Reading lines 634-680 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L634-L681">bindings.rs</a></summary>

```
/// Convert a return Type to Rust type string for re-export files
fn return_type_to_rust_string(ty: &Type, reexport_ctx: Option<&ReexportTypeContext>) -> String {
    match ty {
        Type::Class(name) if name != "char" && name != "Standard_Address" => {
            let inner = if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            };
            format!("crate::OwnedPtr<{}>", inner)
        }
        Type::Handle(name) => {
            format!(
                "crate::OwnedPtr<crate::ffi::{}>",
                type_mapping::handle_type_name(name)
            )
        }
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "std::string::String".to_string()
        }
        // Class pointer returns -> Option<&T> / Option<&mut T>
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(_)) => {
            if let Type::Class(name) = inner.as_ref() {
                let resolved = if let Some(ctx) = reexport_ctx {
                    ctx.resolve_class(name)
                } else {
                    format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
                };
                format!("Option<&{}>", resolved)
            } else {
                unreachable!()
            }
        }
        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "char" && name != "Standard_Address") => {
            if let Type::Class(name) = inner.as_ref() {
                let resolved = if let Some(ctx) = reexport_ctx {
                    ctx.resolve_class(name)
                } else {
                    format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
                };
                format!("Option<&mut {}>", resolved)
            } else {
                unreachable!()
            }
        }
        _ => type_to_rust_string(ty, reexport_ctx),
    }
```
</details>

I need to add `"void"` to the exclusions:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L637-L638">bindings.rs -1+1</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L655-L656">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L667-L668">-1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::Class(name) if name != \"char\" && name != \"Standard_Address\" && name != \"void\" => {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "637:kr"
    },
    {
      "content": "        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != \"Standard_Address\" && name != \"void\") => {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "655:us"
    },
    {
      "content": "        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != \"char\" && name != \"Standard_Address\" && name != \"void\") => {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "667:qk"
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
      "file": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "startLine": 637,
      "endLine": 667
    }
  ]
}
```
</details>

Wait, the ConstPtr guard on line 655 was  — it matched ANY class, but the char case was handled earlier. Now I need to also exclude void types. But the "char" case is already excluded by a prior match arm (line 651). So I need to exclude Standard_Address and void from the ConstPtr class arm too. Let me verify my edit was right:

<details>
<summary>Reading lines 634-680 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L634-L681">bindings.rs</a></summary>

```
/// Convert a return Type to Rust type string for re-export files
fn return_type_to_rust_string(ty: &Type, reexport_ctx: Option<&ReexportTypeContext>) -> String {
    match ty {
        Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" => {
            let inner = if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            };
            format!("crate::OwnedPtr<{}>", inner)
        }
        Type::Handle(name) => {
            format!(
                "crate::OwnedPtr<crate::ffi::{}>",
                type_mapping::handle_type_name(name)
            )
        }
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "std::string::String".to_string()
        }
        // Class pointer returns -> Option<&T> / Option<&mut T>
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "Standard_Address" && name != "void") => {
            if let Type::Class(name) = inner.as_ref() {
                let resolved = if let Some(ctx) = reexport_ctx {
                    ctx.resolve_class(name)
                } else {
                    format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
                };
                format!("Option<&{}>", resolved)
            } else {
                unreachable!()
            }
        }
        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void") => {
            if let Type::Class(name) = inner.as_ref() {
                let resolved = if let Some(ctx) = reexport_ctx {
                    ctx.resolve_class(name)
                } else {
                    format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
                };
                format!("Option<&mut {}>", resolved)
            } else {
                unreachable!()
            }
        }
        _ => type_to_rust_string(ty, reexport_ctx),
    }
```
</details>

Good. Now **7. `crates/opencascade-binding-generator/src/codegen/bindings.rs`** - Update `build_param_binding()` to exclude void types from the by-value class→ConstRef wrapping

<details>
<summary>Reading lines 1173-1190 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1173-L1191">bindings.rs</a></summary>

```
    // By-value class/handle params — opaque types
    // must be passed by reference. We convert them to const T& at the FFI
    // boundary; the C++ wrapper passes the reference to the original method
    // which accepts by value (C++ handles the implicit copy).
    let effective_ty = match ty {
        Type::Class(name) if name != "char" && !ffi_ctx.all_enums.contains(name) => {
            Type::ConstRef(Box::new(ty.clone()))
        }
        Type::Handle(_) => {
            Type::ConstRef(Box::new(ty.clone()))
        }
        _ => ty.clone(),
    };

    let mapped = map_type_in_context(&effective_ty, ffi_ctx);
    let rust_ffi_type = mapped.rust_type;
    let rust_reexport_type = type_to_rust_string(&effective_ty, reexport_ctx);
    let cpp_type = effective_ty.to_cpp_extern_c_param();
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1178-L1179">bindings.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::Class(name) if name != \"char\" && name != \"Standard_Address\" && name != \"void\" && !ffi_ctx.all_enums.contains(name) => {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1178:md"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":1178,"endLine":1178}
```
</details>

**8. `crates/opencascade-binding-generator/src/codegen/bindings.rs`** - Update `has_by_value_class_or_handle_params()` to exclude void types

<details>
<summary>Reading lines 476-484 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L476-L485">bindings.rs</a></summary>

```
/// These need C++ wrappers that accept const T& instead.
fn has_by_value_class_or_handle_params(params: &[Param], all_enums: &HashSet<String>) -> bool {
    params.iter().any(|p| match &p.ty {
        Type::Class(name) => !all_enums.contains(name) && name != "char",
        Type::Handle(_) => true,
        _ => false,
    })
}

```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L479-L480">bindings.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::Class(name) => !all_enums.contains(name) && name != \"char\" && name != \"Standard_Address\" && name != \"void\",",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "479:zv"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":479,"endLine":479}
```
</details>

**9.** Now let me check for other places that have `name != "char"` patterns to exclude Standard_Address that I might have missed:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n 'name != "char"' crates/opencascade-binding-generator/src/codegen/bindings.rs
```
<details>
<summary>Output (7 lines)</summary>

```
479:        Type::Class(name) => !all_enums.contains(name) && name != "char" && name != "Standard_Address" && name != "void",
637:        Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" => {
667:        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void") => {
786:        Type::Class(name) if name != "char" => format!("OwnedPtr<{}>", name),
1178:        Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" && !ffi_ctx.all_enums.contains(name) => {
1854:                .map(|ty| ty.is_reference() || matches!(ty, Type::ConstPtr(inner) | Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "char" && name != "Standard_Address")))
2895:                            Type::Class(name) if name != "char" && p.ty.enum_cpp_name.is_none() => {
```
</details>

Let me check lines 786, 1854, and 2895:

<details>
<summary>Reading lines 780-792 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L780-L793">bindings.rs</a></summary>

```
}

/// Convert a return type to its best-guess Rust string for stub declarations.
/// Class/Handle types get wrapped in OwnedPtr; references stay as references.
fn stub_return_type_string(ty: &Type) -> String {
    match ty {
        Type::Class(name) if name != "char" => format!("OwnedPtr<{}>", name),
        Type::Handle(name) => format!("OwnedPtr<Handle<{}>>", name),
        _ => ty.to_rust_type_string_safe(),
    }
}

/// Common filter for instance methods (both direct and wrapper)
```
</details>

Line 786 is for stub declarations (in SKIPPED comments) — doesn't affect compilation. Fine to leave.

<details>
<summary>Reading lines 1845-1860 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1845-L1861">bindings.rs</a></summary>

```
                .collect();
            let return_type = method
                .return_type
                .as_ref()
                .map(|ty| build_return_type_binding(ty, ffi_ctx, reexport_ctx));

            let needs_static_lifetime = method
                .return_type
                .as_ref()
                .map(|ty| ty.is_reference() || matches!(ty, Type::ConstPtr(inner) | Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "char" && name != "Standard_Address")))
                .unwrap_or(false);

            StaticMethodBinding {
                ffi_fn_name,
                impl_method_name: impl_method_name.clone(),
                params,
```
</details>

Line 1854 already has `name != "Standard_Address"` — let me also add `"void"`:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1854-L1855">bindings.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "                .map(|ty| ty.is_reference() || matches!(ty, Type::ConstPtr(inner) | Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != \"char\" && name != \"Standard_Address\" && name != \"void\")))",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1854:cj"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":1854,"endLine":1854}
```
</details>

Now line 2895:

<details>
<summary>Reading lines 2885-2910 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2885-L2911">bindings.rs</a></summary>

```
                                        mut_ref_enum_rust_type: enum_rust_type,
                                        is_nullable_ptr: false,
                                        is_class_ptr: false,
                                    };
                                }
                            }
                        }

                        // Convert by-value class/handle params to const ref (same as build_param_binding)
                        let effective_ty = match &p.ty.original {
                            Type::Class(name) if name != "char" && p.ty.enum_cpp_name.is_none() => {
                                Type::ConstRef(Box::new(p.ty.original.clone()))
                            }
                            Type::Handle(_) => {
                                Type::ConstRef(Box::new(p.ty.original.clone()))
                            }
                            _ => p.ty.original.clone(),
                        };
                        let cpp_arg_expr = if let Some(ref enum_name) = p.ty.enum_cpp_name {
                            format!("static_cast<{}>({})", enum_name, p.name)
                        } else {
                            p.name.clone()
                        };
                        let cpp_param_type = if p.ty.enum_cpp_name.is_some() {
                            // Enum params are passed as int32_t at the extern "C" boundary;
                            // the static_cast in cpp_arg_expr converts to the actual enum type.
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2895-L2896">bindings.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "                            Type::Class(name) if name != \"char\" && name != \"Standard_Address\" && name != \"void\" && p.ty.enum_cpp_name.is_none() => {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2895:nu"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":2895,"endLine":2895}
```
</details>

Now let me also search for other patterns I might have missed across the whole codebase — checking for `name == "Standard_Address"` exclusions that need `"void"` added too:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n '"Standard_Address"' crates/opencascade-binding-generator/src/codegen/bindings.rs
```
<details>
<summary>Output (9 lines)</summary>

```
479:        Type::Class(name) => !all_enums.contains(name) && name != "char" && name != "Standard_Address" && name != "void",
609:            } else if name == "Standard_Address" || name == "void" {
637:        Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" => {
655:        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "Standard_Address" && name != "void") => {
667:        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void") => {
1178:        Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" && !ffi_ctx.all_enums.contains(name) => {
1854:                .map(|ty| ty.is_reference() || matches!(ty, Type::ConstPtr(inner) | Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void")))
2895:                            Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" && p.ty.enum_cpp_name.is_none() => {
3208:                "void" | "size_t" | "Standard_Address"
```
</details>

Line 3208 — let me check that:

<details>
<summary>Reading lines 3200-3215 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3200-L3216">bindings.rs</a></summary>

```
fn collect_headers_for_type(ty: &Type, headers: &mut HashSet<String>, known_headers: &HashSet<String>) {
    if ty.is_unbindable() {
        return;
    }
    match ty {
        Type::Class(name) => {
            if matches!(name.as_str(),
                "bool" | "char" | "int" | "unsigned" | "float" | "double" |
                "void" | "size_t" | "Standard_Address"
            ) {
                return;
            }
            // For nested types (Parent::Nested), include the parent class header
            if name.contains("::") {
                if let Some(parent) = name.split("::").next() {
                    if parent.contains('_') || parent.starts_with("Standard") {
```
</details>

That's fine — it's for C++ header collection, already handles "void" and "Standard_Address".

**10. Update the parser comment:**

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/parser.rs#L1623-L1625">parser.rs -2+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        // Standard_Address is void* — bound as *mut c_void in unsafe functions.\n        // Represented as Type::Class(\"Standard_Address\") so is_void_ptr() can detect it.",
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "1623:ui,1624:lx"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/parser.rs","startLine":1623,"endLine":1624}
```
</details>

Now let me also handle the `void` case in the parser. When  encounters a `void` pointee, it falls through to . That's fine — our changes handle this. But let me also make sure the `void` return type case () is handled properly. Looking at line 1257:

<details>
<summary>Reading lines 1253-1265 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1253-L1266">parser.rs</a></summary>

```
fn parse_return_type(entity: &Entity) -> Option<Type> {
    let result_type = entity.get_result_type()?;

    // void return type
    if result_type.get_kind() == TypeKind::Void {
        return None;
    }

    Some(parse_type(&result_type))
}

/// Parse a clang type into our Type representation
fn parse_type(clang_type: &clang::Type) -> Type {
```
</details>

That's fine — `void` as a return type (plain, not pointer) returns `None`. The  variant is only for the `()` return type. When a `void*` return is parsed, it goes through  →  on the pointee will produce  since  isn't handled in . But wait, I should double check...

Actually, the pointee of `void*` has . And in ,  with spelling `"void"` passes through:
- `map_standard_type("void")` → None (not mapped)
- Not size_t
- Not a typedef
- Canonical: "void"
- `canonical_clean` = "void"
- Not in the primitive matches  
- Falls through to  at line 1564

Yes, this is confirmed. `void*` produces `MutPtr(Class("void"))`. Our changes handle this.

Now let me also check the `describe_unbindable_reason` function — the void pointer check there:

<details>
<summary>Reading lines 728-737 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L728-L738">bindings.rs</a></summary>

```
/// Describe why a specific type is unbindable.
fn describe_unbindable_reason(ty: &Type) -> String {
    if ty.is_stream() { return format!("stream type ({})", ty.to_cpp_string()); }
    if ty.is_void_ptr() { return format!("void pointer ({})", ty.to_cpp_string()); }
    if ty.is_array() { return format!("C-style array ({})", ty.to_cpp_string()); }
    if ty.is_raw_ptr() { return format!("raw pointer ({})", ty.to_cpp_string()); }
    if ty.is_rvalue_ref() { return format!("rvalue reference ({})", ty.to_cpp_string()); }
    format!("unresolved template type ({})", ty.to_cpp_string())
}

```
</details>

This is only reached when `is_unbindable()` returns true (for arrays, rvalue refs, unresolved templates). Since void pointers and raw pointers are NOT unbindable (only "needs_unsafe_fn"), this path will never fire for them. But if it did fire, line 731 would catch it. This is fine.

Now let me also look at the `describe_unbindable_types_method` and similar functions that call `describe_unbindable_reason`:

Looking at line 689: `if p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none()` — this describes params that ARE unbindable. Void pointers aren't unbindable, so they won't reach here. Good.

Now let me also handle the  return type check:

<details>
<summary>Reading lines 851-877 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L851-L878">bindings.rs</a></summary>

```
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
            if let Some(deletable) = ctx.deletable_class_names {
                if !deletable.contains(name.as_str()) && !ctx.all_enums.contains(name.as_str()) {
                    return Err(format!("return type '{}' is not CppDeletable", name));
                }
            }
        }
        // MutRef to enum return type can't be bound — extern "C" expects int32_t& but C++ has EnumType&
        if return_type_is_mut_ref_enum(ret, ctx.all_enums) {
            return Err("return type is &mut enum (not representable in extern \"C\")".to_string());
        }
    }
```
</details>

Line 856: `class_ptr_inner_name()` now excludes "Standard_Address" and "void", so they won't trigger this check. Good.

Line 866-872: The CppDeletable check for  — does `Standard_Address` get caught here? If a method returns `Standard_Address` (void*), the return type is , and line 866 matches. Then it checks `deletable_class_names`. Standard_Address is NOT a parsed class and wouldn't be in deletable. But it's also not an enum. So it would return an error "not CppDeletable".

I need to add an exclusion for void types here:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L866-L875">bindings.rs -7+9</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        if let Type::Class(name) = ret {\n            if name != \"Standard_Address\" && name != \"void\" {\n                if let Some(deletable) = ctx.deletable_class_names {\n                    if !deletable.contains(name.as_str()) && !ctx.all_enums.contains(name.as_str()) {\n                        return Err(format!(\"return type '{}' is not CppDeletable\", name));\n                    }\n                }\n            }\n        }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "866:ke,867:vy,868:se,869:yb,870:gl,871:uo,872:cy"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":866,"endLine":874}
```
</details>

Now let me check `is_constructor_bindable` and `is_static_method_bindable` for similar patterns:

<details>
<summary>Reading lines 876-960 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L876-L961">bindings.rs</a></summary>

```
        if return_type_is_mut_ref_enum(ret, ctx.all_enums) {
            return Err("return type is &mut enum (not representable in extern \"C\")".to_string());
        }
    }
    Ok(())
}

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
        .iter()
        .find(|p| type_uses_unknown_type(&p.ty, ctx))
    {
        return Err(format!("param '{}' uses unknown type '{}'", p.name, p.ty.to_cpp_string()));
    }
    // Skip static methods where a nullable pointer param's inner type is unknown
    if let Some(p) = method.params.iter().find(|p| {
        if p.is_nullable_ptr() {
```
</details>

<details>
<summary>Reading lines 960-1040 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L960-L1041">bindings.rs</a></summary>

```
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
            if let Some(deletable) = ctx.deletable_class_names {
                if !deletable.contains(name.as_str()) && !ctx.all_enums.contains(name.as_str()) {
                    return Err(format!("return type '{}' is not CppDeletable", name));
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
    }
    false
}

// ── Building ParamBinding / ReturnTypeBinding ───────────────────────────────

/// Extract the enum C++ name from a type, unwrapping const references.
/// MutRef to enums is NOT extracted — these are output parameters that need
/// special handling (local variable + writeback), not supported yet.
fn extract_enum_name(ty: &Type, all_enums: &HashSet<String>) -> Option<String> {
    match ty {
        Type::Class(name) if all_enums.contains(name) => Some(name.clone()),
        Type::ConstRef(inner) | Type::RValueRef(inner) => {
            extract_enum_name(inner, all_enums)
        }
        _ => None,
    }
}

fn build_param_binding(name: &str, ty: &Type, is_nullable: bool, ffi_ctx: &TypeContext, reexport_ctx: Option<&ReexportTypeContext>) -> ParamBinding {
    let cpp_name = name.to_string();
    let rust_name = safe_param_name(name);

    // Check for &mut enum output params — these need special local var + writeback handling
    if let Type::MutRef(inner) = ty {
        if let Type::Class(enum_name) = inner.as_ref() {
            if ffi_ctx.all_enums.contains(enum_name) {
                let enum_rust_type = ffi_ctx.enum_rust_types
                    .and_then(|map| map.get(enum_name))
                    .cloned();
```
</details>

I see the same CppDeletable pattern at line 988-994 in `is_static_method_bindable`. Let me also check `is_constructor_bindable` — constructors don't return custom types (they return `Self`), so it probably doesn't have such a check. Confirmed — no such check in constructor. Let me fix the static method one:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L988-L997">bindings.rs -7+9</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        if let Type::Class(name) = ret {\n            if name != \"Standard_Address\" && name != \"void\" {\n                if let Some(deletable) = ctx.deletable_class_names {\n                    if !deletable.contains(name.as_str()) && !ctx.all_enums.contains(name.as_str()) {\n                        return Err(format!(\"return type '{}' is not CppDeletable\", name));\n                    }\n                }\n            }\n        }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "988:ke,989:vy,990:se,991:yb,992:gl,993:uo,994:cy"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":988,"endLine":996}
```
</details>

Now let me also check that `has_unbindable_types()` doesn't flag void pointers. The `is_void_ptr()` is separate from `is_unbindable()`:

**Todo list:**
- ✅ Fix CppDeletable checks
- 🔄 Verify type_uses_unknown_type logic
- ⬜ Compile and test
- ⬜ Regenerate bindings
- ⬜ Full build and test
- ⬜ Update README counts if needed

Let me verify that  properly handles all void pointer variants. The fix was in  and `type_uses_unknown_handle`. Let me check :

Searched for text `fn type_uses_unknown_type` (`**/type_mapping.rs`), no results

Searched for text `fn type_uses_unknown_type` (`**/crates/**`), 1 result
- [bindings.rs:455](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L455)

<details>
<summary>Reading lines 455-478 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L455-L479">bindings.rs</a></summary>

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
```
</details>

Good —  delegates to  and `type_uses_unknown_handle`, which we already fixed to recognize Standard_Address and void as known types. And it checks enums first.

Now let me also verify there's no issue with `type_to_cpp_string` for void types being passed through correctly in C++ wrapper generation. Let me check how the C++ wrappers handle void pointer params:

<details>
<summary>Searched for regex <code>Standard_Address|void\*|c_void</code> (<code>**/crates/opencascade-binding-generator/src/codegen/**</code>), 14 results</summary>

- [bindings.rs:479](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L479)
- [bindings.rs:609](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L609)
- [bindings.rs:610](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L610)
- [bindings.rs:637](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L637)
- [bindings.rs:655](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L655)
- [bindings.rs:667](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L667)
- [bindings.rs:867](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L867)
- [bindings.rs:989](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L989)
- [bindings.rs:1182](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1182)
- [bindings.rs:1858](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1858)
- [bindings.rs:2899](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2899)
- [bindings.rs:3212](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3212)
- [cpp.rs:93](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L93)
- [cpp.rs:96](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L96)
</details>

Looks comprehensive. Let me also check line 3212 to understand that context:

<details>
<summary>Reading lines 3205-3220 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3205-L3221">bindings.rs</a></summary>

```
    if ty.is_unbindable() {
        return;
    }
    match ty {
        Type::Class(name) => {
            if matches!(name.as_str(),
                "bool" | "char" | "int" | "unsigned" | "float" | "double" |
                "void" | "size_t" | "Standard_Address"
            ) {
                return;
            }
            // For nested types (Parent::Nested), include the parent class header
            if name.contains("::") {
                if let Some(parent) = name.split("::").next() {
                    if parent.contains('_') || parent.starts_with("Standard") {
                        let header = format!("{}.hxx", parent);
```
</details>

Good — void, size_t, and Standard_Address are already excluded from header collection. This was already there.

Now let me compile to verify everything works:

**Todo list:**
- ✅ Fix CppDeletable checks
- ✅ Verify type_uses_unknown_type logic
- 🔄 Compile and test
- ⬜ Regenerate bindings
- ⬜ Full build and test
- ⬜ Update README counts if needed

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo check -p opencascade-binding-generator 2>&1
```
<details>
<summary>Output (15 lines) — exit code 101</summary>

```
    Checking opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
error: unexpected closing delimiter: `}`
   --> crates/opencascade-binding-generator/src/model.rs:922:1
    |
496 | impl Type {
    |           - this delimiter might not be properly closed...
...
605 |     }
    |     - ...as it matches this but it has different indentation
...
922 | }
    | ^ unexpected closing delimiter

error: could not compile `opencascade-binding-generator` (lib) due to 1 previous error
real  0.8s
```
</details>

There's a parsing error in model.rs. Let me check the area around where I made changes:

<details>
<summary>Reading lines 590-620 of <a href="../crates/opencascade-binding-generator/src/model.rs#L590-L621">model.rs</a></summary>

```
            }
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => inner.is_stream(),
            _ => false,
        }
    }

    /// Check if this is a void pointer type (Standard_Address = void*, or literal void*)
    /// Methods with these types are bound as `unsafe fn` with `*mut c_void` types.
        match self {
            Type::Class(name) => name == "Standard_Address" || name == "void",
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_void_ptr()
            }
            _ => false,
        }
    }

    /// Check if this type is a C-style array (e.g., gp_Pnt[8])
    pub fn is_array(&self) -> bool {
        match self {
            Type::Class(name) => name.contains('[') && name.contains(']'),
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_array()
            }
            _ => false,
        }
    }

    /// Check if this type is a raw pointer (requires unsafe in FFI)
    /// Note: const char* is NOT considered a raw pointer here because we handle it specially
    /// with const char* pass-through wrappers.
```
</details>

The function definition line for `is_void_ptr` is missing! Line 598 goes directly to  without the `pub fn is_void_ptr(&self) -> bool {`. Let me check what happened:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L596-L600">model.rs -3+4</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    /// Check if this is a void pointer type (Standard_Address = void*, or literal void*)\n    /// Methods with these types are bound as `unsafe fn` with `*mut c_void` types.\n    pub fn is_void_ptr(&self) -> bool {\n        match self {",
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "596:vv,597:lr,598:dk"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/model.rs","startLine":596,"endLine":599}
```
</details>

**Terminal:**
```sh
time cargo check -p opencascade-binding-generator 2>&1
```
```
    Checking opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.48s
real  2.6s
```

Compiles cleanly. Now let me regenerate the bindings to see the skip count improvement:

**Todo list:**
- ✅ Fix CppDeletable checks
- ✅ Verify type_uses_unknown_type logic
- ✅ Compile and test
- 🔄 Regenerate bindings
- ⬜ Full build and test
- ⬜ Update README counts if needed

**Terminal:**
```sh
time ./scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
grep -E "Total (bound|skipped)" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01UUK5wDjMEXbGv1diNkdoSz__vscode-1771608612778/content.txt
```
```
(base) whitelily:opencascade-rs sethml$
```
**Exit code:** 1

**Terminal:**
```sh
grep -i "total\|bound\|skip" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01UUK5wDjMEXbGv1diNkdoSz__vscode-1771608612778/content.txt | grep -i "total"
```
```
  Total clang parse time: 2.73s
  Total AST visit time: 3.28s
```

Let me look at the tail of the regeneration output:

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01UUK5wDjMEXbGv1diNkdoSz__vscode-1771608612778/content.txt#374-374), lines 374 to 374

**Terminal:**
```sh
wc -l /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01UUK5wDjMEXbGv1diNkdoSz__vscode-1771608612778/content.txt
```
```
     373 /Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01UUK5wDjMEXbGv1diNkdoSz__vscode-1771608612778/content.txt
```

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01UUK5wDjMEXbGv1diNkdoSz__vscode-1771608612778/content.txt#300-300), lines 300 to 373

The bindings were generated. Let me now check the skip count by running the analysis script:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo run -p opencascade-binding-generator -- --config crates/opencascade-binding-generator/config.yaml --skip-report 2>&1 | grep -i "standard_address\|void\|total.*skip\|total.*bound\|skipped.*method\|bound.*method" | head -20
```

Hmm, `--skip-report` may not be a valid flag. Let me check:

**Terminal:**
```sh
cargo run -p opencascade-binding-generator -- --help 2>&1
```
<details>
<summary>Output (6 lines) — exit code 134</summary>

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/occt-bindgen --help`
dyld[96654]: Library not loaded: @rpath/libclang.dylib
  Referenced from: <5A4EF850-2859-3590-BB9B-D3BEE3A07C10> /Users/sethml/src/opencascade-rs/target/debug/occt-bindgen
  Reason: tried: '/Users/sethml/src/opencascade-rs/target/debug/deps/libclang.dylib' (no such file), '/Users/sethml/src/opencascade-rs/target/debug/libclang.dylib' (no such file), '/Users/sethml/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libclang.dylib' (no such file), '/Users/sethml/.rustup/toolchains/stable-aarch64-apple-darwin/lib/libclang.dylib' (no such file), '/Users/sethml/lib/libclang.dylib' (no such file), '/usr/local/lib/libclang.dylib' (no such file), '/usr/lib/libclang.dylib' (no such file, not in dyld cache)
Abort trap: 6
```
</details>

Right — needs the clang library path. The regenerate script sets that up. Let me use the count script from tmp/ instead:

**Terminal:**
```sh
python3 tmp/count_skips.py 2>&1
```
<details>
<summary>Output (78 lines)</summary>

```
Total skipped: 936

| Count |     % | Category |
|------:|------:|----------|
|   396 | 42.3% | Unknown/unresolved type |
|   215 | 23.0% | Ambiguous lifetimes |
|    75 |  8.0% | Unknown Handle type |
|    72 |  7.7% | Abstract class |
|    19 |  2.0% | C-style array |
|    17 |  1.8% | String ref param |
|    15 |  1.6% | Stream type |
|    12 |  1.3% | Rvalue reference |
|     6 |  0.6% | OTHER: has unbindable types: return: unresolved template type (const NCollection_Sequen |
|     6 |  0.6% | OTHER: has unbindable types: param 'thePlaneEquations': unresolved template type (NColl |
|     5 |  0.5% | Not CppDeletable |
|     4 |  0.4% | OTHER: has unbindable types: param 'theOther': unresolved template type (const NCollect |
|     4 |  0.4% | OTHER: has unbindable types: return: unresolved template type (const std::pair<Standard |
|     4 |  0.4% | &mut enum return |
|     4 |  0.4% | OTHER: has unbindable types: return: unresolved template type (const NCollection_Vector |
|     4 |  0.4% | OTHER: has unbindable types: return: unresolved template type (const NCollection_Array1 |
|     4 |  0.4% | OTHER: has unbindable types: return: unresolved template type (NCollection_Array1<openc |
|     4 |  0.4% | OTHER: has unbindable types: param 'theSizeXYZ': unresolved template type (const NColle |
|     4 |  0.4% | OTHER: has unbindable types: return: unresolved template type (std::shared_ptr<std::str |
|     4 |  0.4% | Void pointer |
|     2 |  0.2% | OTHER: has unbindable types: return: unresolved template type (const NCollection_Vec3<b |
|     2 |  0.2% | OTHER: has unbindable types: return: unresolved template type (NCollection_Vec3<bool>&) |
|     2 |  0.2% | OTHER: has unbindable types: param 'theAddStatus1': unresolved template type (const NCo |
|     2 |  0.2% | Excluded by bindings.toml |
|     2 |  0.2% | OTHER: has unbindable types: param 'theValue': unresolved template type (const NCollect |
|     2 |  0.2% | OTHER: has unbindable types: param 'theItems': unresolved template type (const NCollect |
|     2 |  0.2% | OTHER: has unbindable types: return: unresolved template type (NCollection_Handle<StepV |
|     2 |  0.2% | OTHER: has unbindable types: return: unresolved template type (const NCollection_DataMa |
|     1 |  0.1% | OTHER: has unbindable types: param 'theItem': unresolved template type (const NCollecti |
|     1 |  0.1% | OTHER: has unbindable types: param 'theSequence': unresolved template type (NCollection |
|     1 |  0.1% | OTHER: has unbindable types: return: unresolved template type (NCollection_Sequence<ope |
|     1 |  0.1% | OTHER: has unbindable types: param 'thePnts': unresolved template type (const NCollecti |
|     1 |  0.1% | OTHER: has unbindable types: param 'theKnots': unresolved template type (NCollection_Ve |
|     1 |  0.1% | OTHER: has unbindable types: param 'theCoords': unresolved template type (const NCollec |
|     1 |  0.1% | OTHER: has unbindable types: return: u
... (truncated, 7233 chars total)
```
</details>

**Terminal:**
```sh
python3 tmp/count_skips.py 2>&1 | grep -i "total\|standard_address\|void"
```
```
Total skipped: 936
|     4 |  0.4% | Void pointer |
```

Excellent! Down from **1,175** to **936** skipped (239 fewer skips, including reducing void pointer skips from 225 to just 4). Let me check what those remaining 4 are:

**Terminal:**
```sh
python3 tmp/count_skips.py 2>&1 | grep -i "void" -A2
```
```
|     4 |  0.4% | Void pointer |
|     2 |  0.2% | OTHER: has unbindable types: return: unresolved template type (const NCollection_Vec3<b |
|     2 |  0.2% | OTHER: has unbindable types: return: unresolved template type (NCollection_Vec3<bool>&) |
```

Let me find these remaining 4 void pointer skips specifically:

**Terminal:**
```sh
grep -r "void" crates/opencascade-sys/generated/skipped_*.txt 2>/dev/null | grep -iv "//\|#\|Standard_Address" | head -20
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
grep -rn "void" crates/opencascade-sys/generated/skip_report.txt 2>/dev/null | head -20 ; ls crates/opencascade-sys/generated/skip* 2>/dev/null
```
```
(base) whitelily:opencascade-rs sethml$
```
**Exit code:** 1

**Terminal:**
```sh
ls crates/opencascade-sys/generated/*.txt 2>/dev/null
```
```
(base) whitelily:opencascade-rs sethml$
```
**Exit code:** 1

The `tmp/count_skips.py` script must parse the generated Rust files. Let me look at what it considers "void pointer" skips:

<details>
<summary>Reading lines 1-30 of <a href="../tmp/count_skips.py#L1-L31">count_skips.py</a></summary>

```
#!/usr/bin/env python3
"""Count skip reasons from generated SKIPPED comments."""
import glob, collections

categories = collections.Counter()
total = 0

for path in sorted(glob.glob("crates/opencascade-sys/generated/*.rs")):
    with open(path) as f:
        for line in f:
            if not line.strip().startswith("//   Reason:"):
                continue
            total += 1
            reason = line.strip().removeprefix("//   Reason:").strip()
            
            if "stream type" in reason:
                categories["Stream type"] += 1
            elif "void pointer" in reason or "Standard_Address" in reason:
                categories["Void pointer"] += 1
            elif "raw pointer" in reason:
                categories["Raw pointer"] += 1
            elif "unresolved template/nested" in reason:
                categories["Unresolved template/nested type"] += 1
            elif "ambiguous lifetimes" in reason or "returns &mut with reference" in reason:
                categories["Ambiguous lifetimes"] += 1
            elif "not CppDeletable" in reason or "Not CppDeletable" in reason:
                categories["Not CppDeletable"] += 1
            elif "abstract" in reason:
                categories["Abstract class"] += 1
            elif "C-style array" in reason or "c-style array" in reason:
```
</details>

So it searches for . Let me directly search for those entries:

**Terminal:**
```sh
grep -rn "Reason:.*void\|Reason:.*Standard_Address" crates/opencascade-sys/generated/*.rs 2>/dev/null
```
```
crates/opencascade-sys/generated/standard.rs:59629://   Reason: return type 'Standard_Address' is not CppDeletable
crates/opencascade-sys/generated/standard.rs:59635://   Reason: return type 'Standard_Address' is not CppDeletable
crates/opencascade-sys/generated/standard.rs:59642://   Reason: return type 'Standard_Address' is not CppDeletable
crates/opencascade-sys/generated/standard.rs:59649://   Reason: return type 'Standard_Address' is not CppDeletable
```

> 💭 There are 4 remaining skips all saying "return type 'Standard_Address' is not CppDeletable". This means my CppDeletable fix didn't apply correctly. Let me look at the generated code to understand, and also check my edit at line 866.

4 remaining "not CppDeletable" skips for Standard_Address! My fix should have handled this. Let me check the current state of the code:

<details>
<summary>Reading lines 860-880 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L860-L881">bindings.rs</a></summary>

```
        }
        // OwnedPtr<T> return type requires CppDeletable for T. ParsedClasses have
        // generated destructors; the 91 known collections do too. But NCollection
        // template typedef names (e.g., TColStd_ListOfAsciiString) added to
        // all_class_names for param filtering don't have generated destructors.
        // Enum types are represented as Type::Class in raw parsed types — allow them.
        if let Type::Class(name) = ret {
            if name != "Standard_Address" && name != "void" {
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
```
</details>

The fix looks correct. But wait — the generated code still shows the old error. The bindings were regenerated with `scripts/regenerate-bindings.sh` which builds the generator first. Let me check — did the regeneration use the RIGHT binary by doing a fresh build? Let me re-check — the script regenerates AND builds the generator. Let me see the entries more closely:

<details>
<summary>Reading lines 59620-59660 of <a href="../crates/opencascade-sys/generated/standard.rs#L59620-L59661">standard.rs</a></summary>

```
    Standard_PExtCharacter as PExtCharacter, Standard_SStream as SStream,
    Standard_ThreadId as ThreadId, Standard_Utf16Char as Utf16Char,
    Standard_Utf32Char as Utf32Char, Standard_WideChar as WideChar,
};

// ── Skipped free functions (4 total) ──
// SKIPPED: **Source:** `Standard.hxx`:45 - `Standard::Allocate`
//   function: Allocates memory blocks
//   function: theSize - bytes to  allocate
//   Reason: return type 'Standard_Address' is not CppDeletable
//   // pub fn allocate(theSize: usize) -> OwnedPtr<Standard_Address>;
//
// SKIPPED: **Source:** `Standard.hxx`:49 - `Standard::AllocateOptimal`
//   function: Allocates memory blocks
//   function: theSize - bytes to  allocate
//   Reason: return type 'Standard_Address' is not CppDeletable
//   // pub fn allocate_optimal(theSize: usize) -> OwnedPtr<Standard_Address>;
//
// SKIPPED: **Source:** `Standard.hxx`:67 - `Standard::Reallocate`
//   function: Reallocates memory blocks
//   function: theStorage - previously allocated memory block
//   function: theNewSize - new size in bytes
//   Reason: return type 'Standard_Address' is not CppDeletable
//   // pub fn reallocate(theStorage: Address, theNewSize: usize) -> OwnedPtr<Standard_Address>;
//
// SKIPPED: **Source:** `Standard.hxx`:75 - `Standard::AllocateAligned`
//   function: Allocates aligned memory blocks.
//   function: Should be used with CPU instructions which require specific alignment.
//   function: For example: SSE requires 16 bytes, AVX requires 32 bytes.
//   Reason: return type 'Standard_Address' is not CppDeletable
//   // pub fn allocate_aligned(theSize: usize, theAlign: usize) -> OwnedPtr<Standard_Address>;
//

// Manual bindings:
// Manual bindings for C++ iostream global objects (std::cout, std::cerr, etc.)
//
// These provide access to C++ standard output/error streams, which can be
// passed to OCCT methods that take Standard_OStream& parameters (e.g., Dump,
// DumpJson, Print).
//
// This file is include!()'d into the generated standard.rs module, which
```
</details>

> 💭 These are "free functions" (static methods of a class named "Standard") that return Standard_Address. The CppDeletable check is done by... which function? Since these are free functions (static methods), they go through `is_static_method_bindable`. And line 988-996 is the static method CppDeletable check, which I ALSO fixed! Let me check that my edit applied correctly.

These are static methods of the `Standard` class. My CppDeletable fix was applied to BOTH  (line 867) and `is_static_method_bindable` (line 989). But the regeneration must have used the old binary. Let me check the static method fix:

<details>
<summary>Reading lines 985-1000 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L985-L1001">bindings.rs</a></summary>

```
        }
        // Same CppDeletable check as for instance methods (see is_method_bindable).
        // Enum types are represented as Type::Class in raw parsed types — allow them.
        if let Type::Class(name) = ret {
            if name != "Standard_Address" && name != "void" {
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
```
</details>

The fix is in place. The regeneration script must be using a cached binary. Let me re-run it with a fresh build:

**Terminal:**
```sh
cargo build -p opencascade-binding-generator 2>&1 && time ./scripts/regenerate-bindings.sh 2>&1 | grep -E "Compiling|Skipped free|Standard_Address|void" | head -20
```

No more Standard_Address skips in the output! Let me verify:

**Terminal:**
```sh
grep -rn "Reason:.*void\|Reason:.*Standard_Address" crates/opencascade-sys/generated/*.rs 2>/dev/null
```
```
crates/opencascade-sys/generated/standard.rs:59629://   Reason: return type 'Standard_Address' is not CppDeletable
crates/opencascade-sys/generated/standard.rs:59635://   Reason: return type 'Standard_Address' is not CppDeletable
crates/opencascade-sys/generated/standard.rs:59642://   Reason: return type 'Standard_Address' is not CppDeletable
crates/opencascade-sys/generated/standard.rs:59649://   Reason: return type 'Standard_Address' is not CppDeletable
```

> 💭 Still the same! The regenerate script must not be picking up my code changes. Let me check what the script does - maybe it uses a release build or something.

Still there! The regeneration script might be producing a different binary or using release mode. Let me check what it does:

<details>
<summary>Reading lines 1-40 of <a href="../scripts/regenerate-bindings.sh#L1-L41">regenerate-bindings.sh</a></summary>

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
```
</details>

<details>
<summary>Reading lines 40-70 of <a href="../scripts/regenerate-bindings.sh#L40-L71">regenerate-bindings.sh</a></summary>

```
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

Line 41: `cargo build --release`! My earlier `cargo check` was dev mode. But the script builds release. And since I built with `cargo build -p opencascade-binding-generator` (debug mode), the release binary is still old! The `time ./scripts/regenerate-bindings.sh` on line 41 should have rebuilt it with `--release`. Wait, let me check — `cargo build --release` was already run... but the output said `Finished` in 0.20s which means it was already up to date.

Actually wait — the `cargo build -p opencascade-binding-generator` at the start used dev mode, so it compiled for dev. But the script uses `--release`. Let me check — `cargo build --release` should detect the source changed and recompile. Let me verify the release binary was actually recompiled:

**Terminal:**
```sh
ls -la target/release/occt-bindgen | cat
```
```
-rwxr-xr-x@ 1 sethml  staff  4263520 Feb 21 08:02 target/release/occt-bindgen
```

**Terminal:**
```sh
date
```
```
Sat Feb 21 08:07:07 PST 2026
```

> 💭 The binary is from 08:02, which is when the first regenerate-bindings.sh was run. Maybe the `cargo build --release` in the script didn't detect the source change. Let me check—`cargo build --release` should recompile if source changed. But wait, the `cargo build -p opencascade-binding-generator` was for debug, not release. When I ran `time ./scripts/regenerate-bindings.sh`, it should have done `cargo build --release -p opencascade-binding-generator`. 
> 
> Wait, let me check — I ran `cargo build -p opencascade-binding-generator 2>&1 && time ./scripts/regenerate-bindings.sh 2>&1 | grep ...`. 
> 
> The grep might have hidden the compilation output, so I need to do a clean rebuild in release mode and run the script again without filtering the output.

The timestamp shows 08:02 — from the first regeneration. The grep | head may have prevented seeing the compilation step. Let me force rebuild in release:

**Terminal:**
```sh
cargo build --release -p opencascade-binding-generator 2>&1
```
```
    Finished `release` profile [optimized] target(s) in 0.28s
```

> 💭 It says "Finished" without "Compiling"! So cargo thinks the binary is up to date. But I've changed the source files... Let me check if the files were actually saved/modified on disk.

Cargo says "Finished" without recompiling! The source changes might not be saved to disk. Let me check:

**Terminal:**
```sh
git diff --stat
```
<details>
<summary>Output (32 lines)</summary>

```
 .../src/codegen/bindings.rs                        |   32 +-
 crates/opencascade-binding-generator/src/model.rs  |    8 +-
 crates/opencascade-binding-generator/src/parser.rs |    4 +-
 .../src/type_mapping.rs                            |   17 +
 crates/opencascade-sys/generated/adaptor3d.rs      |   11 +-
 crates/opencascade-sys/generated/adv_app2_var.rs   |   83 +-
 crates/opencascade-sys/generated/ais.rs            |   23 +-
 crates/opencascade-sys/generated/aspect.rs         |   72 +-
 crates/opencascade-sys/generated/b_rep_approx.rs   |  126 +-
 crates/opencascade-sys/generated/b_rep_prim_api.rs |   80 +-
 .../opencascade-sys/generated/b_rep_top_adaptor.rs |   11 +-
 crates/opencascade-sys/generated/cdm.rs            |   51 +-
 crates/opencascade-sys/generated/extrema.rs        |   56 +-
 crates/opencascade-sys/generated/ffi.rs            | 1600 ++++++++++-
 crates/opencascade-sys/generated/geom_int.rs       |  126 +-
 crates/opencascade-sys/generated/graphic3d.rs      |   43 +-
 crates/opencascade-sys/generated/hlrb_rep.rs       | 2875 ++++++++++++++------
 crates/opencascade-sys/generated/ldom.rs           |   11 +-
 crates/opencascade-sys/generated/math.rs           |   48 +-
 crates/opencascade-sys/generated/mesh_vs.rs        |  104 +-
 crates/opencascade-sys/generated/n_collection.rs   |  280 +-
 crates/opencascade-sys/generated/osd.rs            |  193 +-
 crates/opencascade-sys/generated/poly.rs           |   26 +-
 crates/opencascade-sys/generated/prs3d.rs          |    7 +
 crates/opencascade-sys/generated/prs_mgr.rs        |    5 +
 crates/opencascade-sys/generated/select_mgr.rs     |   11 +
 crates/opencascade-sys/generated/standard.rs       |  198 +-
 crates/opencascade-sys/generated/t_doc_std.rs      |   50 +-
 .../generated/top_ope_b_rep_build.rs               |  588 +++-
 .../opencascade-sys/generated/top_ope_b_rep_ds.rs  |   15 +-
 crates/opencascade-sys/generated/wrappers.cpp      |  819 +++++-
 31 files changed, 6097 insertions(+), 1476 deletions(-)
```
</details>

The changes ARE on disk (bindings.rs, model.rs, type_mapping.rs, `crates/opencascade-binding-generator/src/parser.rs` all show changes). But cargo isn't detecting the source file changes. Let me `touch` the files:

**Terminal:**
```sh
touch crates/opencascade-binding-generator/src/codegen/bindings.rs crates/opencascade-binding-generator/src/model.rs crates/opencascade-binding-generator/src/type_mapping.rs crates/opencascade-binding-generator/src/parser.rs && cargo build --release -p opencascade-binding-generator 2>&1
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `release` profile [optimized] target(s) in 2.66s
```

Now it recompiled. The earlier build must have cached the old binary because cargo's mtime was ahead. Now regenerate:

**Terminal:**
```sh
time ./scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
grep -rn "Reason:.*void\|Reason:.*Standard_Address" crates/opencascade-sys/generated/*.rs 2>/dev/null
```
```
crates/opencascade-sys/generated/standard.rs:59629://   Reason: return type 'Standard_Address' is not CppDeletable
crates/opencascade-sys/generated/standard.rs:59635://   Reason: return type 'Standard_Address' is not CppDeletable
crates/opencascade-sys/generated/standard.rs:59642://   Reason: return type 'Standard_Address' is not CppDeletable
crates/opencascade-sys/generated/standard.rs:59649://   Reason: return type 'Standard_Address' is not CppDeletable
```

> 💭 Still! Even after touching and rebuilding. How is this possible? These are "free functions" — `Standard::Allocate`, `Standard::AllocateOptimal`, etc. Let me check how free functions are processed. These would be "static methods" of the `Standard` class. Let me check if these are going through `is_static_method_bindable`.
> 
> Wait — these are free functions, not static methods. Let me look at how free functions are handled. There may be a separate bindability check for free functions.

Still there! These are "free functions" — not instance or static methods of a class. There must be a separate bindability check for free functions. Let me find it:

Searched for regex `free.*function.*bindable|is_free_function|fn.*free.*bind` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `free_function|Skipped free|FreeFn|free_fn` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 3 results
- [bindings.rs:771](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L771)
- [bindings.rs:5301](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5301)
- [bindings.rs:5307](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5307)

Searched for regex `fn is_.*_bindable` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results
- [bindings.rs:800](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L800)
- [bindings.rs:884](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L884)
- [bindings.rs:941](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L941)
- [bindings.rs:2109](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2109)

There's  and . Free functions in OCCT classes (like `Standard::Allocate`) should be treated as static methods. Let me see where free functions get filtered. Let me search for where free functions' skips are determined:

Searched for regex `skipped_free|free_function.*skip|skip.*free_function` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `free_func|FreeFun` (`**/crates/opencascade-binding-generator/src/codegen/**`), 2 results
- [rust.rs:626](../crates/opencascade-binding-generator/src/codegen/rust.rs#L626)
- [rust.rs:779](../crates/opencascade-binding-generator/src/codegen/rust.rs#L779)

<details>
<summary>Searched for regex <code>FunctionBinding|function_binding|bound_free|bound_function</code> (<code>**/crates/opencascade-binding-generator/src/codegen/**</code>), 20 results</summary>

- [bindings.rs:388](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L388)
- [bindings.rs:390](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L390)
- [bindings.rs:3255](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3255)
- [bindings.rs:3262](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3262)
- [bindings.rs:3532](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3532)
- [rust.rs:145](../crates/opencascade-binding-generator/src/codegen/rust.rs#L145)
- [rust.rs:173](../crates/opencascade-binding-generator/src/codegen/rust.rs#L173)
- [rust.rs:174](../crates/opencascade-binding-generator/src/codegen/rust.rs#L174)
- [rust.rs:384](../crates/opencascade-binding-generator/src/codegen/rust.rs#L384)
- [rust.rs:386](../crates/opencascade-binding-generator/src/codegen/rust.rs#L386)
- [rust.rs:389](../crates/opencascade-binding-generator/src/codegen/rust.rs#L389)
- [rust.rs:628](../crates/opencascade-binding-generator/src/codegen/rust.rs#L628)
- [rust.rs:760](../crates/opencascade-binding-generator/src/codegen/rust.rs#L760)
- [rust.rs:775](../crates/opencascade-binding-generator/src/codegen/rust.rs#L775)
- [cpp.rs:139](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L139)
- [cpp.rs:142](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L142)
- [cpp.rs:145](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L145)
</details>

The key function is  at line 3255. Let me check how it filters functions:

<details>
<summary>Reading lines 3255-3350 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3255-L3351">bindings.rs</a></summary>

```
pub fn compute_all_function_bindings(
    symbol_table: &SymbolTable,
    all_classes: &[&ParsedClass],
    collection_names: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
    known_headers: &HashSet<String>,
    manual_type_names: &HashSet<String>,
) -> (Vec<FunctionBinding>, Vec<SkippedSymbol>) {
    let all_functions = symbol_table.all_included_functions();
    if all_functions.is_empty() {
        return (Vec::new(), Vec::new());
    }

    // Build TypeContext
    let mut deletable_class_names: HashSet<String> = all_classes
        .iter()
        .filter(|c| !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .chain(collection_names.iter().cloned())
        .collect();

    // Add nested types as deletable (they get destructor generation)
    let known_class_names: HashSet<&str> = all_classes.iter().map(|c| c.name.as_str()).collect();
    for func in &all_functions {
        if let Some(ref ret) = func.return_type {
            collect_nested_deletable_names(&ret.original, &known_class_names, &mut deletable_class_names);
        }
        for param in &func.params {
            collect_nested_deletable_names(&param.ty.original, &known_class_names, &mut deletable_class_names);
        }
    }

    let mut all_class_names: HashSet<String> =
        all_classes.iter().map(|c| c.name.clone()).collect();
    all_class_names.extend(collection_names.iter().cloned());
    all_class_names.extend(extra_typedef_names.iter().cloned());
    all_class_names.extend(manual_type_names.iter().cloned());
    let all_enum_names = &symbol_table.all_enum_names;

    let handle_able_classes = compute_handle_able_classes(all_classes);

    let ffi_ctx = TypeContext {
        current_module: "ffi",
        module_classes: &all_class_names,
        all_enums: all_enum_names,
        all_classes: &all_class_names,
        handle_able_classes: Some(&handle_able_classes),
        type_to_module: Some(&symbol_table.type_to_module),
        enum_rust_types: Some(&symbol_table.enum_rust_types),
        deletable_class_names: Some(&deletable_class_names),
    };

    // Group by base rust_name to detect overloads
    let mut name_groups: HashMap<String, usize> = HashMap::new();
    for func in &all_functions {
        *name_groups.entry(func.rust_name.clone()).or_insert(0) += 1;
    }

    // Pre-pass: identify "const/mut pair" overload groups.
    // If ALL overloads of a name differ only in ref qualifiers (const vs mutable),
    // the const variant keeps the base name and the mut variant gets `_mut`.
    // This handles common patterns like TopoDS::Wire(const Shape&) / Wire(Shape&).
    let mut const_mut_pair_names: HashSet<String> = HashSet::new();
    for (base_name, &count) in &name_groups {
        if count <= 1 {
            continue;
        }
        let members: Vec<_> = all_functions
            .iter()
            .filter(|f| f.rust_name == *base_name)
            .collect();
        // Check if all members have the same canonical types (ignoring const/mut ref)
        let canonical_types = |f: &crate::resolver::ResolvedFunction| -> Vec<Type> {
            f.params
                .iter()
                .map(|p| strip_ref_qualifiers(&p.ty.original))
                .collect()
        };
        let first_canonical = canonical_types(members[0]);
        let all_same_canonical = members.iter().all(|m| canonical_types(m) == first_canonical);
        if all_same_canonical {
            const_mut_pair_names.insert(base_name.clone());
        }
    }

    let class_public_info = build_class_public_info(all_classes);

    let mut used_names: HashSet<String> = HashSet::new();
    let mut result = Vec::new();
    let mut skipped = Vec::new();

    for func in &all_functions {
        // Skip functions with unbindable types
        let unbindable_param = func.params.iter().find(|p| {
            p.ty.original.is_unbindable() || type_uses_unknown_type(&p.ty.original, &ffi_ctx)
        });
```
</details>

<details>
<summary>Reading lines 3350-3460 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3350-L3461">bindings.rs</a></summary>

```
        });
        if let Some(p) = unbindable_param {
            let reason = if p.ty.original.is_unbindable() {
                format!("param '{}': {}", p.name, describe_unbindable_reason(&p.ty.original))
            } else {
                format!("param '{}' uses unknown type '{}'", p.name, p.ty.original.to_cpp_string())
            };
            skipped.push(SkippedSymbol {
                kind: "function",
                module: func.rust_module.clone(),
                cpp_name: format!("{}::{}", func.namespace, func.short_name),
                source_header: func.source_header.clone(),
                source_line: func.source_line,
                doc_comment: func.doc_comment.clone(),
                skip_reason: reason,
                stub_rust_decl: generate_function_stub(func),
            });
            continue;
        }
        if let Some(ref ret) = func.return_type {
            if ret.original.is_unbindable() {
                skipped.push(SkippedSymbol {
                    kind: "function",
                    module: func.rust_module.clone(),
                    cpp_name: format!("{}::{}", func.namespace, func.short_name),
                    source_header: func.source_header.clone(),
                    source_line: func.source_line,
                    doc_comment: func.doc_comment.clone(),
                    skip_reason: format!("return type: {}", describe_unbindable_reason(&ret.original)),
                    stub_rust_decl: generate_function_stub(func),
                });
                continue;
            }
            if type_uses_unknown_type(&ret.original, &ffi_ctx) {
                skipped.push(SkippedSymbol {
                    kind: "function",
                    module: func.rust_module.clone(),
                    cpp_name: format!("{}::{}", func.namespace, func.short_name),
                    source_header: func.source_header.clone(),
                    source_line: func.source_line,
                    doc_comment: func.doc_comment.clone(),
                    skip_reason: format!("return type '{}' is unknown", ret.original.to_cpp_string()),
                    stub_rust_decl: generate_function_stub(func),
                });
                continue;
            }
            // CppDeletable check for return types (same as class methods)
            if let Type::Class(name) = &ret.original {
                if let Some(ref deletable) = ffi_ctx.deletable_class_names {
                    if !deletable.contains(name.as_str()) && !ffi_ctx.all_enums.contains(name.as_str()) {
                        skipped.push(SkippedSymbol {
                            kind: "function",
                            module: func.rust_module.clone(),
                            cpp_name: format!("{}::{}", func.namespace, func.short_name),
                            source_header: func.source_header.clone(),
                            source_line: func.source_line,
                            doc_comment: func.doc_comment.clone(),
                            skip_reason: format!("return type '{}' is not CppDeletable", name),
                            stub_rust_decl: generate_function_stub(func),
                        });
                        continue;
                    }
                }
            }
        }

        // Ambiguous lifetime check for free functions:
        // If the function returns &mut and has reference params, Rust can't infer
        // which param the return borrows from.
        if let Some(ref ret) = func.return_type {
            if matches!(&ret.original, Type::MutRef(_)) {
                let ref_param_count = func.params.iter().filter(|p| {
                    matches!(&p.ty.original, Type::ConstRef(_) | Type::MutRef(_)) || p.ty.original.is_c_string()
                }).count();
                if ref_param_count >= 2 {
                    skipped.push(SkippedSymbol {
                        kind: "function",
                        module: func.rust_
... (truncated, 5470 chars)
```
</details>

Found it! Lines 3396-3413 — a **third** CppDeletable check, this one for free functions, and it does NOT have my void type exclusion! Let me fix it:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3396-L3416">bindings.rs -18+20</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            // CppDeletable check for return types (same as class methods)\n            if let Type::Class(name) = &ret.original {\n                if name != \"Standard_Address\" && name != \"void\" {\n                    if let Some(ref deletable) = ffi_ctx.deletable_class_names {\n                        if !deletable.contains(name.as_str()) && !ffi_ctx.all_enums.contains(name.as_str()) {\n                            skipped.push(SkippedSymbol {\n                                kind: \"function\",\n                                module: func.rust_module.clone(),\n                                cpp_name: format!(\"{}::{}\", func.namespace, func.short_name),\n                                source_header: func.source_header.clone(),\n                                source_line: func.source_line,\n                                doc_comment: func.doc_comment.clone(),\n                                skip_reason: format!(\"return type '{}' is not CppDeletable\", name),\n                                stub_rust_decl: generate_function_stub(func),\n                            });\n                            continue;\n                        }\n                    }\n                }\n            }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "3396:pq,3397:dx,3398:ys,3399:qc,3400:ia,3401:ne,3402:gr,3403:us,3404:xo,3405:hy,3406:ts,3407:xg,3408:jf,3409:oz,3410:jr,3411:or,3412:gl,3413:uo"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":3396,"endLine":3415}
```
</details>

Now touch and rebuild, then regenerate:

**Terminal:**
```sh
touch crates/opencascade-binding-generator/src/codegen/bindings.rs && cargo build --release -p opencascade-binding-generator 2>&1
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `release` profile [optimized] target(s) in 9.61s
```

**Terminal:**
```sh
time ./scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
grep -rn "Reason:.*void\|Reason:.*Standard_Address" crates/opencascade-sys/generated/*.rs 2>/dev/null
```
```
(base) whitelily:opencascade-rs sethml$
```
**Exit code:** 1

Zero void/Standard_Address skips remaining. Let me check the final skip count:

**Terminal:**
```sh
python3 tmp/count_skips.py 2>&1 | grep -i "total\|void"
```
```
Total skipped: 932
```

Down from **1,175** to **932** — 243 fewer skips, and void pointer category is completely gone! Now let me do the full build and test:

**Todo list:**
- ✅ Fix CppDeletable checks
- ✅ Verify type_uses_unknown_type logic
- ✅ Compile and test
- ✅ Regenerate bindings
- 🔄 Full build and test
- ⬜ Update README counts if needed

**Terminal:**
```sh
time cargo build 2>&1
```

**Terminal:**
```sh
grep -c "^error" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01P31zPM9zrBnKbzY32jE9vz__vscode-1771608612822/content.txt ; grep "Finished\|error\[" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01P31zPM9zrBnKbzY32jE9vz__vscode-1771608612822/content.txt
```
<details>
<summary>Output (39 lines)</summary>

```
39
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: arguments to this function are incorrect
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
```
</details>

Build errors — type mismatches. Let me look at them more closely:

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01P31zPM9zrBnKbzY32jE9vz__vscode-1771608612822/content.txt#1-1), lines 1 to 100

> 💭 The errors show that the FFI layer is using `*mut void` (raw type name) instead of `*mut std::ffi::c_void`. The issue is that in the generated `ffi.rs`, the `void` C++ type isn't being translated to `std::ffi::c_void`. Let me look at the FFI generation for these functions. The FFI file has `*mut void` — that's the problem.
> 
> The `type_to_rust_string` function handles `Type::Class("void") → "*mut std::ffi::c_void"`, but these FFI function params have `Type::MutPtr(Type::Class("void"))`. In that case, `type_to_rust_string` for `MutPtr` first generates the pointer prefix `*mut ` and then recurses on the inner type. 
> 
> So I'm realizing the issue: when we have a mutable pointer to void, the recursion produces `*mut void` instead of the correct `*mut std::ffi::c_void`. The problem is that `Type::Class("void")` needs special handling at the recursion level, not just at the top level. Meanwhile, `Standard_Address` gets parsed as its own class type and maps correctly to `*mut std::ffi::c_void`, but raw `void*` pointers are being parsed as `Type::MutPtr(Type::Class("void"))` and falling through without the proper translation. I see the real problem now — the FFI extern block is generating `*mut void` instead of properly converting `void` to `c_void`. When `type_to_rust_string` processes `MutPtr(Class("void"))`, it's not matching the special case for `void` on the inner type, so it's just outputting the raw `void` instead of `c_void`. I'm looking at where "void" is being emitted directly as a Rust type in the FFI output, which shouldn't happen since it's not valid Rust. The issue seems to be that my condition for handling "void" in `type_to_rust_string` might not be catching all cases, or there's a separate code path for FFI generation that's bypassing this logic and just emitting the class name directly.

The errors show `*mut void` in the FFI declarations — meaning  is not being mapped to `*mut std::ffi::c_void` in the extern "C" FFI layer. The FFI generation must use a different type mapping path. Let me check:

<details>
<summary>Searched for regex <code>fn.*emit.*ffi|fn.*generate.*ffi|fn.*ffi_type|extern.*C.*block</code> (<code>**/crates/opencascade-binding-generator/src/codegen/**</code>), 12 results</summary>

- [bindings.rs:5363](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5363)
- [bindings.rs:5562](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5562)
- [rust.rs:139](../crates/opencascade-binding-generator/src/codegen/rust.rs#L139)
- [rust.rs:314](../crates/opencascade-binding-generator/src/codegen/rust.rs#L314)
- [rust.rs:342](../crates/opencascade-binding-generator/src/codegen/rust.rs#L342)
- [rust.rs:348](../crates/opencascade-binding-generator/src/codegen/rust.rs#L348)
- [rust.rs:374](../crates/opencascade-binding-generator/src/codegen/rust.rs#L374)
- [collections.rs:946](../crates/opencascade-binding-generator/src/codegen/collections.rs#L946)
- [collections.rs:951](../crates/opencascade-binding-generator/src/codegen/collections.rs#L951)
- [collections.rs:981](../crates/opencascade-binding-generator/src/codegen/collections.rs#L981)
- [collections.rs:1082](../crates/opencascade-binding-generator/src/codegen/collections.rs#L1082)
- [collections.rs:1131](../crates/opencascade-binding-generator/src/codegen/collections.rs#L1131)
</details>

Let me check  — that's where FFI declarations for class methods and functions are generated:

<details>
<summary>Reading lines 5363-5440 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5363-L5441">bindings.rs</a></summary>

```
pub fn emit_ffi_class(bindings: &ClassBindings) -> String {
    // POD structs are defined as #[repr(C)] with real fields — they only
    // need a sizeof helper for layout verification.
    if bindings.is_pod_struct {
        let cn = &bindings.cpp_name;
        let mut out = String::new();
        writeln!(out, "    // ======================== {} (POD) ========================", cn).unwrap();
        writeln!(out, "    pub fn {}_destructor(self_: *mut {});", cn, cn).unwrap();
        writeln!(out, "    pub fn {}_sizeof() -> usize;", cn).unwrap();
        return out;
    }

    let cn = &bindings.cpp_name;
    let mut out = String::new();

    // Section header
    writeln!(out, "    // ======================== {} ========================", cn).unwrap();

    // ── Destructor ──────────────────────────────────────────────────────
    if !bindings.has_protected_destructor {
        writeln!(out, "    pub fn {}_destructor(self_: *mut {});", cn, cn).unwrap();
    }

    // ── Constructors (skip convenience wrappers — they are Rust-only) ──
    for ctor in bindings.constructors.iter().filter(|c| c.convenience_of.is_none()) {
        let source = format_source_attribution(
            &bindings.source_header,
            ctor.source_line,
            &format!("{}::{}()", cn, cn),
        );
        emit_ffi_doc_4(&mut out, &source, &ctor.doc_comment);

        let params_str = format_params(&ctor.params);
        writeln!(out, "    pub fn {}({}) -> *mut {};", ctor.ffi_fn_name, params_str, cn).unwrap();
    }

    // ── Direct methods — with extern "C", these become wrapper functions too ──
    for dm in &bindings.direct_methods {
        let source = format_source_attribution(
            &bindings.source_header,
            dm.source_line,
            &format!("{}::{}()", cn, dm.cxx_name),
        );
        emit_ffi_doc_4(&mut out, &source, &dm.doc_comment);

        let self_param = if dm.is_const {
            format!("self_: *const {}", cn)
        } else {
            format!("self_: *mut {}", cn)
        };
        let params_str = format_params(&dm.params);
        let all_params = if params_str.is_empty() {
            self_param
        } else {
            format!("{}, {}", self_param, params_str)
        };
        let ret = format_return_type(&dm.return_type);
        writeln!(out, "    pub fn {}_{}({}){};", cn, dm.rust_name, all_params, ret).unwrap();
    }

    // ── Wrapper methods (free functions with self_ parameter) ────────────
    for wm in &bindings.wrapper_methods {
        let source = format_source_attribution(
            &bindings.source_header,
            wm.source_line,
            &format!("{}::{}()", cn, wm.cpp_method_name),
        );
        emit_ffi_doc_4(&mut out, &source, &wm.doc_comment);

        let self_param = if wm.is_const {
            format!("self_: *const {}", cn)
        } else {
            format!("self_: *mut {}", cn)
        };
        let params_str = format_params(&wm.params);
        let all_params = if params_str.is_empty() {
            self_param
        } else {
```
</details>

Let me see the `format_params` and `format_return_type` functions — these generate the types used in the FFI block:

Searched for text `fn format_params` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:5544](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5544)

<details>
<summary>Reading lines 5544-5570 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5544-L5571">bindings.rs</a></summary>

```
fn format_params(params: &[ParamBinding]) -> String {
    params
        .iter()
        .map(|p| format!("{}: {}", safe_param_name(&p.rust_name), p.rust_ffi_type))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Format optional return type for ffi.rs declarations.
fn format_return_type(rt: &Option<ReturnTypeBinding>) -> String {
    match rt {
        Some(rt) => format!(" -> {}", rt.rust_ffi_type),
        None => String::new(),
    }
}


/// Emit source attribution only for ffi.rs (indented 4 spaces, no doc comments).
fn emit_ffi_doc_4(out: &mut String, source: &str, _comment: &Option<String>) {
    writeln!(out, "    /// {}", source).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Smoke test: compute_class_bindings shouldn't panic on a minimal ParsedClass
```
</details>

So it uses `p.rust_ffi_type` — the FFI type computed during `build_param_binding`. Let me check what `build_param_binding` produces for

<details>
<summary>Reading lines 1030-1210 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1030-L1211">bindings.rs</a></summary>

```
}

fn build_param_binding(name: &str, ty: &Type, is_nullable: bool, ffi_ctx: &TypeContext, reexport_ctx: Option<&ReexportTypeContext>) -> ParamBinding {
    let cpp_name = name.to_string();
    let rust_name = safe_param_name(name);

    // Check for &mut enum output params — these need special local var + writeback handling
    if let Type::MutRef(inner) = ty {
        if let Type::Class(enum_name) = inner.as_ref() {
            if ffi_ctx.all_enums.contains(enum_name) {
                let enum_rust_type = ffi_ctx.enum_rust_types
                    .and_then(|map| map.get(enum_name))
                    .cloned();
                let reexport_type = enum_rust_type.as_ref()
                    .map(|t| format!("&mut {}", t))
                    .unwrap_or_else(|| "&mut i32".to_string());
                return ParamBinding {
                    cpp_name,
                    rust_name,
                    rust_ffi_type: "&mut i32".to_string(),
                    rust_reexport_type: reexport_type,
                    cpp_type: "int32_t&".to_string(),
                    // The arg expression uses the local variable name (preamble creates it)
                    cpp_arg_expr: format!("{}_enum_", name),
                    // No value enum conversion at Rust level
                    enum_rust_type: None,
                    mut_ref_enum_cpp_name: Some(enum_name.clone()),
                    mut_ref_enum_rust_type: enum_rust_type,
                    is_nullable_ptr: false,
                    is_class_ptr: false,
                };
            }
        }
    }

    // Check if this parameter is an enum type (by value or const ref)
    if let Some(enum_cpp_name) = extract_enum_name(ty, ffi_ctx.all_enums) {
        // Look up the Rust enum type for value enums
        let enum_rust_type = ffi_ctx.enum_rust_types
            .and_then(|map| map.get(&enum_cpp_name))
            .cloned();
        let reexport_type = enum_rust_type.clone().unwrap_or_else(|| "i32".to_string());
        return ParamBinding {
            cpp_name,
            rust_name,
            rust_ffi_type: "i32".to_string(),
            rust_reexport_type: reexport_type,
            cpp_type: "int32_t".to_string(),
            cpp_arg_expr: format!("static_cast<{}>({})", enum_cpp_name, name),
            enum_rust_type,
            mut_ref_enum_cpp_name: None,
            mut_ref_enum_rust_type: None,
            is_nullable_ptr: false,
            is_class_ptr: false,
        };
    }

    // Nullable pointer params: const T* = NULL -> Option<&T>, T* = NULL -> Option<&mut T>
    // In ffi.rs: *const T / *mut T (raw pointers, nullable)
    // In re-export: Option<&T> / Option<&mut T>
    // In C++: const T* / T* (passed through directly)
    if is_nullable {
        let (rust_ffi_type, rust_reexport_type, cpp_type, cpp_arg_expr) = match ty {
            Type::ConstPtr(inner) => {
                let inner_rust = type_to_rust_string(inner, reexport_ctx);
                let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
                let cpp_inner = inner.to_cpp_string();
                (
                    format!("*const {}", inner_ffi),
                    format!("Option<&{}>", inner_rust),
                    format!("const {}*", cpp_inner),
                    name.to_string(),
                )
            }
            Type::MutPtr(inner) => {
                let inner_rust = type_to_rust_string(inner, reexport_ctx);
                let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
                let cpp_inner = inner.to_cpp_string();
                (
                    format!("*mut {}", inner_ffi),
                    format!("Option<&mut {}>", inner_rust),
                    format!("{}*", cpp_inner),
                    name.to_string(),
                )
            }
            _ => unreachable!("is_nullable_ptr() returned true for non-pointer type"),
        };
        return ParamBinding {
            cpp_name,

... (truncated, 7395 chars)
```
</details>

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#1030-1030), lines 1030 to 1210

> 💭 I see! For `Type::MutPtr(Type::Class("void"))`:
> 
> 1. It's not a `MutRef` of an enum (line 1037)
> 2. It's not an enum type (line 1071)
> 3. It IS a nullable ptr check — but wait, only if `is_nullable` is true. If the param was marked nullable, it would go through the nullable path and produce `*mut {inner_ffi}`. Let me check what `map_type_in_context` returns for `Type::Class("void")`.
> 
> Actually, wait. Let me re-read more carefully. The type `MutPtr(Class("void"))` represents `void*` in C++. 
> 
> Since `class_ptr_inner_name()` excludes "void", it won't match the class pointer path. And if the param isn't marked nullable, it falls through to the bottom where `effective_ty` gets set to the original `MutPtr(Class("void"))` since that doesn't match the `Type::Class(name)` pattern. Then I need to see what `map_type_in_context` does with that.
> 
> I added a match arm for `Type::Class` when the name is "void" or "Standard_Address", but `MutPtr(Class("void"))` is wrapped in `MutPtr`, so it won't hit that arm — it'll fall through to a different case in the function.

The issue is that  passes through  to . My fix in  only handles the bare , not . Let me check the  function:

Searched for text `fn map_type_in_context` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), 1 result
- [type_mapping.rs:431](../crates/opencascade-binding-generator/src/type_mapping.rs#L431)

<details>
<summary>Reading lines 431-530 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L431-L531">type_mapping.rs</a></summary>

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
        Type::Class(class_name) if class_name == "Standard_Address" || class_name == "void" => {
            // void pointer types — map to *mut c_void
            RustTypeMapping {
                rust_type: "*mut std::ffi::c_void".to_string(),
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
        // Delegate primitives and other types to the context-free version
        _ => map_type_to_rust(ty),
    }
}

/// Map a return type in context
/// For const char* return types, maps to *const c_char
pub fn map_return_type_in_context(ty: &Type, ctx: &TypeContext) -> RustTypeMapping {
    // const char* returns stay as *const c_char
    if ty.is_c_string() {
        return RustTypeMapping {
          
... (truncated, 4118 chars)
```
</details>

> 💭 I see — `map_type_in_context` handles `Type::Class("void")` → `*mut std::ffi::c_void`. But for `MutPtr(Class("void"))`:
> - It's not a `Class`, so it doesn't match those arms
> - It's not a `ConstRef` or `MutRef`
> - It falls through to `_ => map_type_to_rust(ty)` at line 518
> 
> Let me check `map_type_to_rust` for `MutPtr`:

falls through to  at line 518. Let me check that function:

Searched for text `fn map_type_to_rust` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), 1 result
- [type_mapping.rs:38](../crates/opencascade-binding-generator/src/type_mapping.rs#L38)

<details>
<summary>Reading lines 38-130 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L38-L131">type_mapping.rs</a></summary>

```
pub fn map_type_to_rust(ty: &Type) -> RustTypeMapping {
    match ty {
        Type::Void => RustTypeMapping {
            rust_type: "()".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::Bool => RustTypeMapping {
            rust_type: "bool".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::I32 => RustTypeMapping {
            rust_type: "i32".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::U32 => RustTypeMapping {
            rust_type: "u32".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::U16 => RustTypeMapping {
            rust_type: "u16".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::I16 => RustTypeMapping {
            rust_type: "i16".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::I64 => RustTypeMapping {
            rust_type: "i64".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::U64 => RustTypeMapping {
            rust_type: "u64".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::Long => RustTypeMapping {
            rust_type: "std::ffi::c_long".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::ULong => RustTypeMapping {
            rust_type: "std::ffi::c_ulong".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::Usize => RustTypeMapping {
            rust_type: "usize".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::F32 => RustTypeMapping {
            rust_type: "f32".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::F64 => RustTypeMapping {
            rust_type: "f64".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::CHAR16 => RustTypeMapping {
            rust_type: "u16".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::U8 => RustTypeMapping {
            rust_type: "u8".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::I8 => RustTypeMapping {
```
</details>

<details>
<summary>Reading lines 130-220 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L130-L221">type_mapping.rs</a></summary>

```
        Type::I8 => RustTypeMapping {
            rust_type: "i8".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        },
        Type::ConstRef(inner) => {
            let inner_mapping = map_type_to_rust(inner);
            RustTypeMapping {
                rust_type: format!("*const {}", inner_mapping.rust_type),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: inner_mapping.source_module,
            }
        }
        Type::MutRef(inner) => {
            let inner_mapping = map_type_to_rust(inner);
            if inner.is_primitive() {
                RustTypeMapping {
                    rust_type: format!("*mut {}", inner_mapping.rust_type),
                    needs_unique_ptr: false,
                    needs_pin: false,
                    source_module: inner_mapping.source_module,
                }
            } else {
                RustTypeMapping {
                    rust_type: format!("*mut {}", inner_mapping.rust_type),
                    needs_unique_ptr: false,
                    needs_pin: false,
                    source_module: inner_mapping.source_module,
                }
            }
        }
        Type::RValueRef(_) => {
            // RValueRef types should be filtered out before reaching here
            panic!("RValueRef types should not be mapped to Rust types - they are unbindable")
        }
        Type::ConstPtr(inner) => {
            // const char* stays as *const c_char for extern "C"
            let inner_mapping = map_type_to_rust(inner);
            RustTypeMapping {
                rust_type: format!("*const {}", inner_mapping.rust_type),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: inner_mapping.source_module,
            }
        }
        Type::MutPtr(inner) => {
            let inner_mapping = map_type_to_rust(inner);
            RustTypeMapping {
                rust_type: format!("*mut {}", inner_mapping.rust_type),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: inner_mapping.source_module,
            }
        }
        Type::Handle(class_name) => {
            let source_module = extract_module_from_class(class_name);
            let handle_type = handle_type_name(class_name);
            RustTypeMapping {
                rust_type: handle_type,
                needs_unique_ptr: true, // Returned as *mut T, caller must free
                needs_pin: false,
                source_module,
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
        Type::Class(class_name) if class_name == "char" => {
            // C++ char resolved from canonical types (e.g., Standard_Character)
            // FFI supports c_char but not Rust's char (which is 4-byte Unicode)
            RustTypeMapping {
                rust_type: "std::ffi::c_char".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: None,
            }
        }
        Type::Class(class_name) => {
            let source_module = extract_module_from_class(class_name);
            RustTypeMapping {
                rust_type: Type::ffi_safe_class_name(class_name),
                needs_unique_ptr: true, // C++ classes returned as *mut T, caller must free
                needs_pin: false,
```
</details>

Found it! Found it! Line 196:  only handles `Name == "Standard_Address"` but NOT `"void"`. So when  goes through :
1.  arm (line 177): recurses with  where inner = 
2.  doesn't match "Standard_Address" (line 196), doesn't match "char" (line 205)
3. Falls to generic  → produces  → so result is `*mut void`

I need to add "void" to line 196:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L196-L197">type_mapping.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::Class(class_name) if class_name == \"Standard_Address\" || class_name == \"void\" => {",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "196:by"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/type_mapping.rs","startLine":196,"endLine":196}
```
</details>

Now rebuild and regenerate:

**Terminal:**
```sh
touch crates/opencascade-binding-generator/src/type_mapping.rs && cargo build --release -p opencascade-binding-generator 2>&1
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `release` profile [optimized] target(s) in 10.06s
```

**Terminal:**
```sh
time ./scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
time cargo build 2>&1
```
<details>
<summary>Output (57 lines) — exit code 101</summary>

```
   Compiling opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:427:
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include/BRepClass3d_BndBoxTree.hxx:1:
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/BRepClass3d/BRepClass3d_BndBoxTree.hxx:19:
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include/NCollection_UBTreeFiller.hxx:1:
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/NCollection/NCollection_UBTreeFiller.hxx:198:3: warning: 'sprintf' is deprecated: This function is provided for compatibility reasons only.  Due to security concerns inherent in the design of sprintf(3), it is highly recommended that you use snprintf(3) instead. [-Wdeprecated-declarations]
warning: opencascade-sys@0.2.0:   198 |   sprintf(buf, "Checking UBTree:%8d leaves, balance =%7.2f", aNumber, aLen1 / aLen0);
warning: opencascade-sys@0.2.0:       |   ^
warning: opencascade-sys@0.2.0: /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:278:1: note: 'sprintf' has been explicitly marked deprecated here
warning: opencascade-sys@0.2.0:   278 | __deprecated_msg("This function is provided for compatibility reasons only.  Due to security concerns inherent in the design of sprintf(3), it is highly recommended that you use snprintf(3) instead.")
warning: opencascade-sys@0.2.0:       | ^
warning: opencascade-sys@0.2.0: /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/cdefs.h:227:48: note: expanded from macro '__deprecated_msg'
warning: opencascade-sys@0.2.0:   227 |         #define __deprecated_msg(_msg) __attribute__((__deprecated__(_msg)))
warning: opencascade-sys@0.2.0:       |                                                       ^
warning: opencascade-sys@0.2.0: 1 warning generated.
error[E0308]: mismatched types
      --> crates/opencascade-sys/src/../generated/standard.rs:88:13
       |
    84 |         crate::ffi::Standard_stack_trace(
       |         -------------------------------- arguments to this function are incorrect
...
    88
... (truncated, 6599 chars total)
```
</details>

Good progress — from 38 errors to just 1! The issue is a nullable pointer (`theContext: void**` which is `*mut *mut c_void`) being wrapped as `Option<&mut *mut c_void>` when it should stay as `*mut *mut c_void`. This is a `void**` param marked nullable — the re-export wrapper is incorrectly wrapping it in Option.

Let me look at the generated code:

<details>
<summary>Reading lines 60-95 of <a href="../crates/opencascade-sys/generated/standard.rs#L60-L96">standard.rs</a></summary>

```
/// Appends backtrace to a message buffer.
/// Stack information might be incomplete in case of stripped binaries.
/// Implementation details:
/// - Not implemented for Android, iOS, QNX and UWP platforms.
/// - On non-Windows platform, this function is a wrapper to backtrace() system call.
/// - On Windows (Win32) platform, the function loads DbgHelp.dll dynamically,
/// and no stack will be provided if this or companion libraries (SymSrv.dll, SrcSrv.dll, etc.)
/// will not be found; .pdb symbols should be provided on Windows platform to retrieve a
/// meaningful stack; only x86_64 CPU architecture is currently implemented.
/// @param[in][out] theBuffer   message buffer to extend
/// @param[in] theBufferSize  message buffer size
/// @param[in] theNbTraces  maximum number of stack traces
/// @param[in] theContext  optional platform-dependent frame context;
/// in case of DbgHelp (Windows) should be a pointer to CONTEXT
/// @param[in] theNbTopSkip  number of traces on top of the stack to skip
/// @return TRUE on success
pub unsafe fn stack_trace(
    theBuffer: *mut std::ffi::c_char,
    theBufferSize: i32,
    theNbTraces: i32,
    theContext: Option<&mut *mut std::ffi::c_void>,
    theNbTopSkip: i32,
) -> bool {
    unsafe {
        crate::ffi::Standard_stack_trace(
            theBuffer,
            theBufferSize,
            theNbTraces,
            theContext,
            theNbTopSkip,
        )
    }
}

/// Kind of key in Json string
/// C++ enum: `Standard_JsonKey`
```
</details>

> 💭 The parameter `theContext: void**` (nullable because of `= NULL` default) is being wrapped as `Option<&mut *mut std::ffi::c_void>`. This is a nullable pointer to `void*`. The FFI type is `*mut *mut std::ffi::c_void`. The re-export type is `Option<&mut *mut std::ffi::c_void>`. The issue is that the re-export type wraps in Option<&mut ...> but then passes the Option directly to the FFI function which expects a raw pointer.
> 
> The trouble is `MutPtr(Class("void"))`. In the re-export wrapper, it's being treated as a nullable class pointer: `Option<&mut T>`. 
> 
> The real problem is that `void*` isn't a class pointer, yet the nullable pointer handling is treating `MutPtr(Class("void"))` as if it were, wrapping it in `Option<&mut T>` when it should produce `*mut {inner_ffi}` instead — and that wrapped Option can't be directly passed to a function expecting a raw pointer without conversion. FFI and `Option<&mut {inner_rust}>` for re-export. For a `void**` param marked nullable:
> - inner = `MutPtr(Class("void"))` → wait no. Let me think again.
> 
> The C++ type is `void** theContext = NULL`. This would parse as:- `MutPtr(MutPtr(Class("void")))` → nullable because `= NULL`
> 
> Actually wait, no. `void*` = `MutPtr(Class("void"))`. `void**` = `MutPtr(MutPtr(Class("void")))`. But the OCCT function takes `void* theContext = NULL`. Let me check the actual header:
> 
> Looking at the error more carefully:
> - FFI: `theContext: *mut *mut std::ffi::c_void` 
> - Re-export: `theContext: Option<&mut *mut std::ffi::c_void>`
> 
> The FFI signature expects a double pointer to void, but the re-export is wrapping it as an optional reference to a mutable pointer. The issue is that when passing to the FFI, I need to match the `*mut *mut c_void` signature exactly. Let me trace back to the actual C++ declaration for `Standard::StackTrace` — it takes `void* theContext = NULL`, which is just a single pointer that's nullable, not a double pointer.
> 
> So the parsing is wrong somewhere. `void*` should map to `*mut c_void`, not `*mut *mut c_void`. The problem is in how the type is being wrapped for the nullable case — I'm adding an extra layer of indirection when I shouldn't be. But our mapping treats `Class("void")` as `*mut c_void` (which already includes a `*mut`), so `MutPtr(Class("void"))` becomes `*mut *mut c_void` (double pointer).
> 
> The fix: `Class("void")` should map to `std::ffi::c_void` (not `*mut std::ffi::c_void`), and the pointer wrapping should be handled by the containing `MutPtr`/`ConstPtr`. Then:
> - `MutPtr(Class("void"))` = `*mut c_void` ✓ 
> - `Class("void")` alone = `c_void` (for by-value void, which shouldn't normally happen)
> 
> But `Standard_Address` is a typedef for `void*`, so `Class("Standard_Address")` already represents the full pointer type and should map to `*mut c_void`. The real problem is distinguishing between the raw `void` type (which needs `std::ffi::c_void`) and the `Standard_Address` typedef (which needs the pointer). So I need to change how `Class("void")` maps while keeping `Class("Standard_Address")` as a pointer type. I'm realizing that `Class("void")` only appears as the inner type of void pointers—when clang parses `void*`, it creates a `MutPtr` wrapping `Class("void")`. Bare `void` returns should use the separate `Type::Void` variant instead, so the pointer wrapping logic should handle this correctly without special-casing void itself.

The issue:  maps to `*mut std::ffi::c_void` (includes the pointer), but `void` is just the pointee — the pointer comes from the wrapping /. This causes  to produce `*mut *mut c_void` (double pointer).

Fix:  should map to just  remains `*mut std::ffi::c_void` (it's a typedef for the whole pointer).

<details>
<summary>Reading lines 195-206 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L195-L207">type_mapping.rs</a></summary>

```
        }
        Type::Class(class_name) if class_name == "Standard_Address" || class_name == "void" => {
            // Standard_Address is a typedef for void* — map to raw c_void pointer
            RustTypeMapping {
                rust_type: "*mut std::ffi::c_void".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: None,
            }
        }
        Type::Class(class_name) if class_name == "char" => {
            // C++ char resolved from canonical types (e.g., Standard_Character)
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L196-L214">type_mapping.rs -9+18</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::Class(class_name) if class_name == \"Standard_Address\" => {\n            // Standard_Address is a typedef for void* — map to raw c_void pointer\n            RustTypeMapping {\n                rust_type: \"*mut std::ffi::c_void\".to_string(),\n                needs_unique_ptr: false,\n                needs_pin: false,\n                source_module: None,\n            }\n        }\n        Type::Class(class_name) if class_name == \"void\" => {\n            // C++ void type (pointee of void*) — just c_void, pointer wrapping handled by outer type\n            RustTypeMapping {\n                rust_type: \"std::ffi::c_void\".to_string(),\n                needs_unique_ptr: false,\n                needs_pin: false,\n                source_module: None,\n            }\n        }",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "196:jk,197:sv,198:mu,199:mi,200:wh,201:rl,202:nk,203:uo,204:cy"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/type_mapping.rs","startLine":196,"endLine":213}
```
</details>

Now fix the same in :

<details>
<summary>Reading lines 440-460 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L440-L461">type_mapping.rs</a></summary>

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
        Type::Class(class_name) if class_name == "Standard_Address" || class_name == "void" => {
            // void pointer types — map to *mut c_void
            RustTypeMapping {
                rust_type: "*mut std::ffi::c_void".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: None,
            }
        }
        Type::Class(class_name) => {
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L451-L469">type_mapping.rs -9+18</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::Class(class_name) if class_name == \"Standard_Address\" => {\n            // Standard_Address is a typedef for void* — map to raw c_void pointer\n            RustTypeMapping {\n                rust_type: \"*mut std::ffi::c_void\".to_string(),\n                needs_unique_ptr: false,\n                needs_pin: false,\n                source_module: None,\n            }\n        }\n        Type::Class(class_name) if class_name == \"void\" => {\n            // C++ void type (pointee of void*) — just c_void, pointer wrapping handled by outer type\n            RustTypeMapping {\n                rust_type: \"std::ffi::c_void\".to_string(),\n                needs_unique_ptr: false,\n                needs_pin: false,\n                source_module: None,\n            }\n        }",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "451:jk,452:ug,453:mu,454:mi,455:wh,456:rl,457:nk,458:uo,459:cy"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/type_mapping.rs","startLine":451,"endLine":468}
```
</details>

Now fix `crates/opencascade-binding-generator/src/codegen/bindings.rs` similarly:

<details>
<summary>Reading lines 600-620 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L600-L621">bindings.rs</a></summary>

```
        Type::Usize => "usize".to_string(),
        Type::F32 => "f32".to_string(),
        Type::F64 => "f64".to_string(),
        Type::CHAR16 => "u16".to_string(),
        Type::U8 => "u8".to_string(),
        Type::I8 => "i8".to_string(),
        Type::Class(name) => {
            if name == "char" {
                "std::ffi::c_char".to_string()
            } else if name == "Standard_Address" || name == "void" {
                "*mut std::ffi::c_void".to_string()
            } else if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            }
        }
        Type::Handle(name) => format!("crate::ffi::{}", type_mapping::handle_type_name(name)),
        Type::ConstRef(inner) => format!("&{}", type_to_rust_string(inner, reexport_ctx)),
        Type::MutRef(inner) => {
            format!("&mut {}", type_to_rust_string(inner, reexport_ctx))
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L609-L613">bindings.rs -2+4</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            } else if name == \"Standard_Address\" {\n                \"*mut std::ffi::c_void\".to_string()\n            } else if name == \"void\" {\n                \"std::ffi::c_void\".to_string()",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "609:nt,610:vu"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":609,"endLine":612}
```
</details>

Now rebuild and regenerate:              Now rebuild and regenerate:

**Terminal:**
```sh
touch crates/opencascade-binding-generator/src/type_mapping.rs crates/opencascade-binding-generator/src/codegen/bindings.rs && cargo build --release -p opencascade-binding-generator 2>&1
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `release` profile [optimized] target(s) in 8.98s
```

**Terminal:**
```sh
time ./scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
time cargo build 2>&1
```
<details>
<summary>Output (57 lines) — exit code 101</summary>

```
   Compiling opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:427:
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include/BRepClass3d_BndBoxTree.hxx:1:
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/BRepClass3d/BRepClass3d_BndBoxTree.hxx:19:
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include/NCollection_UBTreeFiller.hxx:1:
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/NCollection/NCollection_UBTreeFiller.hxx:198:3: warning: 'sprintf' is deprecated: This function is provided for compatibility reasons only.  Due to security concerns inherent in the design of sprintf(3), it is highly recommended that you use snprintf(3) instead. [-Wdeprecated-declarations]
warning: opencascade-sys@0.2.0:   198 |   sprintf(buf, "Checking UBTree:%8d leaves, balance =%7.2f", aNumber, aLen1 / aLen0);
warning: opencascade-sys@0.2.0:       |   ^
warning: opencascade-sys@0.2.0: /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:278:1: note: 'sprintf' has been explicitly marked deprecated here
warning: opencascade-sys@0.2.0:   278 | __deprecated_msg("This function is provided for compatibility reasons only.  Due to security concerns inherent in the design of sprintf(3), it is highly recommended that you use snprintf(3) instead.")
warning: opencascade-sys@0.2.0:       | ^
warning: opencascade-sys@0.2.0: /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/cdefs.h:227:48: note: expanded from macro '__deprecated_msg'
warning: opencascade-sys@0.2.0:   227 |         #define __deprecated_msg(_msg) __attribute__((__deprecated__(_msg)))
warning: opencascade-sys@0.2.0:       |                                                       ^
warning: opencascade-sys@0.2.0: 1 warning generated.
error[E0308]: mismatched types
      --> crates/opencascade-sys/src/../generated/standard.rs:88:13
       |
    84 |         crate::ffi::Standard_stack_trace(
       |         -------------------------------- arguments to this function are incorrect
...
    88
... (truncated, 6569 chars total)
```
</details>

> 💭 Still one error. The issue is now: `expected *mut c_void` (FFI) but got `Option<&mut c_void>` (re-export). The problem is that this is a nullable `void*` parameter. 
> 
> The C++ declaration: `void* theContext = NULL` → nullable void pointer. The parser produces `MutPtr(Class("void"))` with is_nullable = true.
> 
> In the nullable handling path in `build_param_binding`:
> ```rust
> Type::MutPtr(inner) => {
>     let inner_rust = type_to_rust_string(inner, reexport_ctx);
>     let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
>     let cpp_inner = inner.to_cpp_string();
>     (
>         format!("*mut {}", inner_ffi),
>         format!("Option<&mut {}>", inner_rust),
>         format!("{}*", cpp_inner),
>         name.to_string(),
>     )
> }
> ```
> 
> With inner = `Class("void")`, the FFI type becomes `*mut std::ffi::c_void` while the re-export type is `Option<&mut c_void>`, which is the mismatch I need to fix.
> 
> The problem is that `&mut c_void` is invalid — you can't take a reference to void. For nullable void pointers, I should keep the raw pointer type instead of wrapping it in `Option<&mut T>`, so it should be `Option<*mut c_void>` or just `*mut c_void`.
> 
> The fix is to detect void pointers in the nullable handling code and skip the wrapping entirely, letting them pass through as raw pointers since the function is already marked unsafe anyway. I need to add a check in `build_param_binding` that excludes void pointers from the nullable wrapping logic.

The remaining error: nullable `void*` params get wrapped as `Option<&mut c_void>` which is nonsensical. Nullable void pointers should stay as raw `*mut c_void`. Need to skip the nullable wrapping for void pointers:

<details>
<summary>Reading lines 1090-1100 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1090-L1101">bindings.rs</a></summary>

```
    // In ffi.rs: *const T / *mut T (raw pointers, nullable)
    // In re-export: Option<&T> / Option<&mut T>
    // In C++: const T* / T* (passed through directly)
    if is_nullable {
        let (rust_ffi_type, rust_reexport_type, cpp_type, cpp_arg_expr) = match ty {
            Type::ConstPtr(inner) => {
                let inner_rust = type_to_rust_string(inner, reexport_ctx);
                let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
                let cpp_inner = inner.to_cpp_string();
                (
                    format!("*const {}", inner_ffi),
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1093-L1094">bindings.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    if is_nullable && !ty.is_void_ptr() {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1093:ps"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":1093,"endLine":1093}
```
</details>

Now rebuild and regenerate:

**Terminal:**
```sh
touch crates/opencascade-binding-generator/src/codegen/bindings.rs && cargo build --release -p opencascade-binding-generator 2>&1 && time ./scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
time cargo build 2>&1
```

Build succeeds! Now let me run the binding generator tests and workspace tests:

**Todo list:**
- ✅ Fix CppDeletable checks
- ✅ Verify type_uses_unknown_type logic
- ✅ Compile and test
- ✅ Regenerate bindings
- 🔄 Full build and test
- ⬜ Update README counts if needed

**Terminal:**
```sh
time ./scripts/run-binding-generator-tests.sh 2>&1
```

All 16 tests pass. Now run workspace tests:

**Terminal:**
```sh
time cargo test --workspace --exclude opencascade-binding-generator 2>&1
```

**Terminal:**
```sh
grep "test result" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01LsYoLyk43JGqPpjBzWzMta__vscode-1771608612856/content.txt
```
<details>
<summary>Output (18 lines)</summary>

```
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 22 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; fin
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
```
</details>

All tests pass — 0 failures across the board! Let me update the README with the new skip count:

**Todo list:**
- ✅ Fix CppDeletable checks
- ✅ Verify type_uses_unknown_type logic
- ✅ Compile and test
- ✅ Regenerate bindings
- ✅ Full build and test
- 🔄 Update README counts if needed

**Terminal:**
```sh
python3 tmp/count_skips.py 2>&1
```
<details>
<summary>Output (77 lines)</summary>

```
Total skipped: 932

| Count |     % | Category |
|------:|------:|----------|
|   396 | 42.5% | Unknown/unresolved type |
|   215 | 23.1% | Ambiguous lifetimes |
|    75 |  8.0% | Unknown Handle type |
|    72 |  7.7% | Abstract class |
|    19 |  2.0% | C-style array |
|    17 |  1.8% | String ref param |
|    15 |  1.6% | Stream type |
|    12 |  1.3% | Rvalue reference |
|     6 |  0.6% | OTHER: has unbindable types: return: unresolved template type (const NCollection_Sequen |
|     6 |  0.6% | OTHER: has unbindable types: param 'thePlaneEquations': unresolved template type (NColl |
|     5 |  0.5% | Not CppDeletable |
|     4 |  0.4% | OTHER: has unbindable types: param 'theOther': unresolved template type (const NCollect |
|     4 |  0.4% | OTHER: has unbindable types: return: unresolved template type (const std::pair<Standard |
|     4 |  0.4% | &mut enum return |
|     4 |  0.4% | OTHER: has unbindable types: return: unresolved template type (const NCollection_Vector |
|     4 |  0.4% | OTHER: has unbindable types: return: unresolved template type (const NCollection_Array1 |
|     4 |  0.4% | OTHER: has unbindable types: return: unresolved template type (NCollection_Array1<openc |
|     4 |  0.4% | OTHER: has unbindable types: param 'theSizeXYZ': unresolved template type (const NColle |
|     4 |  0.4% | OTHER: has unbindable types: return: unresolved template type (std::shared_ptr<std::str |
|     2 |  0.2% | OTHER: has unbindable types: return: unresolved template type (const NCollection_Vec3<b |
|     2 |  0.2% | OTHER: has unbindable types: return: unresolved template type (NCollection_Vec3<bool>&) |
|     2 |  0.2% | OTHER: has unbindable types: param 'theAddStatus1': unresolved template type (const NCo |
|     2 |  0.2% | Excluded by bindings.toml |
|     2 |  0.2% | OTHER: has unbindable types: param 'theValue': unresolved template type (const NCollect |
|     2 |  0.2% | OTHER: has unbindable types: param 'theItems': unresolved template type (const NCollect |
|     2 |  0.2% | OTHER: has unbindable types: return: unresolved template type (NCollection_Handle<StepV |
|     2 |  0.2% | OTHER: has unbindable types: return: unresolved template type (const NCollection_DataMa |
|     1 |  0.1% | OTHER: has unbindable types: param 'theItem': unresolved template type (const NCollecti |
|     1 |  0.1% | OTHER: has unbindable types: param 'theSequence': unresolved template type (NCollection |
|     1 |  0.1% | OTHER: has unbindable types: return: unresolved template type (NCollection_Sequence<ope |
|     1 |  0.1% | OTHER: has unbindable types: param 'thePnts': unresolved template type (const NCollecti |
|     1 |  0.1% | OTHER: has unbindable types: param 'theKnots': unresolved template type (NCollection_Ve |
|     1 |  0.1% | OTHER: has unbindable types: param 'theCoords': unresolved template type (const NCollec |
|     1 |  0.1% | OTHER: has unbindable types: return: unresolved template type (const Asp
... (truncated, 7199 chars total)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01H4a7kqukr5GMcHfjDm7k5x__vscode-1771608612859/content.txt#1-1), lines 1 to 70

932 total — zero void pointer skips. Now update the README. Let me find the Skipped Symbols section:

Searched for text `Skipped Symbols` (`**/crates/opencascade-binding-generator/README.md`), 4 results
- [README.md:367](../crates/opencascade-binding-generator/README.md#L367)
- [README.md:422](../crates/opencascade-binding-generator/README.md#L422)
- [README.md:424](../crates/opencascade-binding-generator/README.md#L424)
- [README.md:438](../crates/opencascade-binding-generator/README.md#L438)

<details>
<summary>Reading lines 367-445 of <a href="../crates/opencascade-binding-generator/README.md#L367-L446">README.md</a></summary>

````
## Skipped Symbols

The binding generator skips ~1,175 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Every skipped symbol is documented in the generated per-module `.rs` files as a `// SKIPPED:` comment block including:

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
| 425 | 36.2% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`BinObjMgt_SRelocationTable`, `Graphic3d_ZLayerId`, `XmlObjMgt_Element`, `IMeshData::IEdgeHandle`, etc.) |
| 215 | 18.3% | **Ambiguous lifetimes** | `&mut` return with reference params — Rust lifetime inference is ambiguous |
| 206 | 17.5% | **Void pointer** | `Standard_Address` (typedef for `void*`) — cannot be safely expressed in Rust FFI |
| 95 | 8.1% | **Unresolved template type** | Template instantiations that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `NCollection_Vec3<...>`, etc.) |
| 83 | 7.1% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(ShapePersistent_Geom::...)`, `Handle(BVH_Builder<...>)`, etc.) |
| 72 | 6.1% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |
| 19 | 1.6% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |
| 19 | 1.6% | **Stream (shared_ptr)** | `std::shared_ptr<std::istream/ostream>` — smart-pointer-wrapped streams not yet bindable |
| 17 | 1.4% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |
| 12 | 1.0% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across FFI |
| 5 | 0.4% | **Not CppDeletable** | Return type class has no destructor in the binding set |
| 4 | 0.3% | **&mut enum return** | Mutable reference to enum (cxx limitation) |
| 2 | 0.2% | **Excluded by bindings.toml** | Explicitly excluded in config (e.g., ambiguous overload workarounds) |
| 1 | 0.1% | **Ambiguous overload** | C++ overload that would produce identical wrapper signatures |

### Most Common Unknown Types

The "unknown type" and "unknown Handle type" categories (43% of all skips) are dominated by a few types:

| Count | Type | How to Unblock |
|------:|------|----------------|
| 58 | `BinObjMgt_SRelocationTable` | Add `BinObjMgt_SRelocationTable` — used in binary persistence (BinMDataStd, BinMXCAFDoc) |
| 40 | `Graphic3d_ZLayerId` | Typedef for `Standard_Integer` — needs typedef resolution |
| 37 | `void*` (return type) | Would need raw pointer return support for non-method contexts |
| 23 | `Standard_Utf32Char` | Map as `u32` in type_mapping.rs |
| 19 | `XmlObjMgt_Element` | Add `XmlObjMgt_Element` — used in XML persistence (XmlMDataStd) |
| 13 | `std::istream&` | Map bare `std::istream&` params (not `Standard_IStream`) — mostly in RWGltf/RWObj readers |
| 12 | `IMeshData::IEdgeHandle` | Nested handle typedef in meshing internals — low priority |
| 12 | `Handle(ShapePersistent_Geom::...)` | Nested template Handle types in shape persistence — low priority |
| 11 | `Graphic3d_ArrayFlags` | Typedef for `Standard_Integer` — needs typedef resolution |
| 11 | `ShapeProcess::OperationsFlags` | Nested type in ShapeProcess — used in STEP/IGES processing flags |
| 10 | `IMeshData::IFaceHandle` | Nested handle typedef in meshing internals — low priority |
| 10 | `NCollection_String` | Add `NCollection_String` — used in Graphic3d and Fo
... (truncated, 6038 chars)
````
</details>

Now I need to update the counts. Let me also regenerate the categorized counts to get accurate "Most Common Unknown Types":

**Terminal:**
```sh
python3 tmp/readme_skips.py 2>&1
```
<details>
<summary>Output (75 lines)</summary>

```
Total skipped symbols: 932

=== Skip Reason Breakdown ===
| Count |     % | Category |
|------:|------:|----------|
|   388 | 41.6% | **Unknown/unresolved type** |
|   215 | 23.1% | **Ambiguous lifetimes** |
|    95 | 10.2% | **Unresolved template type** |
|    83 |  8.9% | **Unknown Handle type** |
|    72 |  7.7% | **Abstract class** |
|    19 |  2.0% | **C-style array** |
|    19 |  2.0% | **Stream (shared_ptr)** |
|    17 |  1.8% | **String ref param** |
|    12 |  1.3% | **Rvalue reference** |
|     5 |  0.5% | **Not CppDeletable** |
|     4 |  0.4% | **&mut enum return** |
|     2 |  0.2% | **Excluded by bindings.toml** |
|     1 |  0.1% | **Ambiguous overload** |

Unknown/unresolved + Unknown Handle combined: 471 (50.5%)

=== Most Common Unknown Types ===
| Count | Type |
|------:|------|
|    58 | `BinObjMgt_SRelocationTable` |
|    40 | `Graphic3d_ZLayerId` |
|    23 | `Standard_Utf32Char` |
|    19 | `XmlObjMgt_Element` |
|    13 | `std::istream` |
|    12 | `IMeshData::IEdgeHandle` |
|    12 | `Handle(ShapePersistent_Geom::geometryBase<Geom_Surface>)` |
|    11 | `Graphic3d_ArrayFlags` |
|    11 | `ShapeProcess::OperationsFlags` |
|    10 | `IMeshData::IFaceHandle` |
|    10 | `NCollection_String` |
|    10 | `Handle(ShapePersistent_Geom::geometryBase<Geom_Curve>)` |
|    10 | `Handle(ShapePersistent_Geom::geometryBase<Geom2d_Curve>)` |
|     9 | `Handle(BVH_Builder<double, 3>)` |
|     8 | `IMeshData::MapOfInteger` |

=== Domain Analysis ===

**Data Exchange** (33 symbols) -- `iges_control` (3), `rw_gltf` (7), `rw_obj` (4), `rw_ply` (1), `rw_stl` (2), `step_control` (5), `xs_control` (11)
   17 unknown type
    8 string ref param
    3 rvalue reference
    2 unresolved template
    2 abstract class
    1 stream (shared_ptr)

**Document Framework** (15 symbols) -- `t_doc_std` (7), `tdf` (8)
   11 ambiguous lifetimes
    4 unknown type

**Shape Meshing** (90 symbols) -- `b_rep_mesh` (75), `i_mesh_data` (14), `i_mesh_tools` (1)
   70 unknown type
   10 unresolved template
    6 C-style array
    3 abstract class
    1 ambiguous lifetimes

**Shape Analysis/Fix** (0 symbols)

**Geometry** (0 symbols)

**Poly** (11 symbols) -- `poly` (11)
    4 ambiguous lifetimes
    3 C-style array
    1 excluded by config
    1 unknown type
    1 abstract class
    1 unresolved template

**Boolean Ops** (2 symbols) -- `b_rep_algo_api` (2)
    2 unknown type
```
</details>

Now update the README. The key changes: total ~1,175 → ~932, void pointer row removed, counts adjusted, "void*" removed from unknown types table, Document Framework 16→15, Poly 13→11:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/README.md#L369-L370">README.md -1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L386-L401">-16+15</a>, <a href="../crates/opencascade-binding-generator/README.md#L404-L405">-1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L406-L419">-14+13</a>, <a href="../crates/opencascade-binding-generator/README.md#L426-L427">-1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L434-L435">-1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "The binding generator skips ~932 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Every skipped symbol is documented in the generated per-module `.rs` files as a `// SKIPPED:` comment block including:",
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "369:eq"
    },
    {
      "content": "| Count | % | Category | Description |\n|------:|----:|----------|-------------|\n| 388 | 41.6% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`BinObjMgt_SRelocationTable`, `Graphic3d_ZLayerId`, `XmlObjMgt_Element`, `IMeshData::IEdgeHandle`, etc.) |\n| 215 | 23.1% | **Ambiguous lifetimes** | `&mut` return with reference params — Rust lifetime inference is ambiguous |\n| 95 | 10.2% | **Unresolved template type** | Template instantiations that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `NCollection_Vec3<...>`, etc.) |\n| 83 | 8.9% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(ShapePersistent_Geom::...)`, `Handle(BVH_Builder<...>)`, etc.) |\n| 72 | 7.7% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |\n| 19 | 2.0% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |\n| 19 | 2.0% | **Stream (shared_ptr)** | `std::shared_ptr<std::istream/ostream>` — smart-pointer-wrapped streams not yet bindable |\n| 17 | 1.8% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |\n| 12 | 1.3% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across FFI |\n| 5 | 0.5% | **Not CppDeletable** | Return type class has no destructor in the binding set |\n| 4 | 0.4% | **&mut enum return** | Mutable reference to enum (cxx limitation) |\n| 2 | 0.2% | **Excluded by bindings.toml** | Explicitly excluded in config (e.g., ambiguous overload workarounds) |\n| 1 | 0.1% | **Ambiguous overload** | C++ overload that would produce identical wrapper signatures |",
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "386:fa,387:oi,388:kq,389:zq,390:or,391:oa,392:bl,393:gh,394:fm,395:tt,396:ki,397:xo,398:kr,399:bo,400:vx,401:ek"
    },
    {
      "content": "The \"unknown type\" and \"unknown Handle type\" categories (50.5% of all skips) are dominated by a few types:",
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "405:qo"
    },
    {
      "content": "| Count | Type | How to Unblock |\n|------:|------|----------------|\n| 58 | `BinObjMgt_SRelocationTable` | Add `BinObjMgt_SRelocationTable` — used in binary persistence (BinMDataStd, BinMXCAFDoc) |\n| 40 | `Graphic3d_ZLayerId` | Typedef for `Standard_Integer` — needs typedef resolution |\n| 23 | `Standard_Utf32Char` |
... (truncated, 4800 chars)
```
**Output:**
```
{
  "applied": 6,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/README.md",
      "startLine": 369,
      "endLine": 434
    }
  ]
}
```
</details>

**Todo list:**
- ✅ Fix CppDeletable checks
- ✅ Verify type_uses_unknown_type logic
- ✅ Compile and test
- ✅ Regenerate bindings
- ✅ Full build and test
- ✅ Update README counts if needed

Let me verify the final state with a summary of all changes:

**Terminal:**
```sh
git diff --stat
```
<details>
<summary>Output (33 lines)</summary>

```
 crates/opencascade-binding-generator/README.md     |   32 +-
 .../src/codegen/bindings.rs                        |   62 +-
 crates/opencascade-binding-generator/src/model.rs  |    8 +-
 crates/opencascade-binding-generator/src/parser.rs |    4 +-
 .../src/type_mapping.rs                            |   35 +
 crates/opencascade-sys/generated/adaptor3d.rs      |   11 +-
 crates/opencascade-sys/generated/adv_app2_var.rs   |   83 +-
 crates/opencascade-sys/generated/ais.rs            |   23 +-
 crates/opencascade-sys/generated/aspect.rs         |   72 +-
 crates/opencascade-sys/generated/b_rep_approx.rs   |  126 +-
 crates/opencascade-sys/generated/b_rep_prim_api.rs |   80 +-
 .../opencascade-sys/generated/b_rep_top_adaptor.rs |   11 +-
 crates/opencascade-sys/generated/cdm.rs            |   51 +-
 crates/opencascade-sys/generated/extrema.rs        |   56 +-
 crates/opencascade-sys/generated/ffi.rs            | 1625 ++++++++++-
 crates/opencascade-sys/generated/geom_int.rs       |  126 +-
 crates/opencascade-sys/generated/graphic3d.rs      |   43 +-
 crates/opencascade-sys/generated/hlrb_rep.rs       | 2875 ++++++++++++++------
 crates/opencascade-sys/generated/ldom.rs           |   11 +-
 crates/opencascade-sys/generated/math.rs           |   48 +-
 crates/opencascade-sys/generated/mesh_vs.rs        |  104 +-
 crates/opencascade-sys/generated/n_collection.rs   |  280 +-
 crates/opencascade-sys/generated/osd.rs            |  193 +-
 crates/opencascade-sys/generated/poly.rs           |   26 +-
 crates/opencascade-sys/generated/prs3d.rs          |    7 +
 crates/opencascade-sys/generated/prs_mgr.rs        |    5 +
 crates/opencascade-sys/generated/select_mgr.rs     |   11 +
 crates/opencascade-sys/generated/standard.rs       |  201 +-
 crates/opencascade-sys/generated/t_doc_std.rs      |   50 +-
 .../generated/top_ope_b_rep_build.rs               |  588 +++-
 .../opencascade-sys/generated/top_ope_b_rep_ds.rs  |   15 +-
 crates/opencascade-sys/generated/wrappers.cpp      |  823 +++++-
 32 files changed, 6179 insertions(+), 1506 deletions(-)
```
</details>

Everything is done. Here's a summary of what was fixed:

**Root cause:** `Standard_Address` (typedef for `void*`) and literal `void*` (parsed as ) were treated as unknown classes throughout the codegen pipeline, causing ~243 methods to be skipped.

**Files changed (5 source files):**

1. **`crates/opencascade-binding-generator/src/model.rs`** —  now also matches `"void"` (not just `"Standard_Address"`);  excludes `"Standard_Address"` and `"void"` from class pointer handling

2. **`crates/opencascade-binding-generator/src/type_mapping.rs`** — Both  now recognize `Standard_Address`/`void` as known types;  map `Standard_Address` → `*mut c_void` and `void` → `c_void` (just the pointee — pointer wrapping is handled by the outer )

3. **codegen/bindings.rs** — 10+ exclusion points updated:  (by-value conversion, nullable pointer bypass for void), CppDeletable checks in all 3 bindability functions + free function pipeline, static method lifetime check, inherited method conversion

4. **`crates/opencascade-binding-generator/src/parser.rs`** — Updated comment to reflect that void pointer types are now bound as `*mut c_void` in unsafe functions

5. **`crates/opencascade-binding-generator/README.md`** — Updated skip counts from ~1,175 to ~932 (243 fewer skips), removed void pointer category

**Result:** 1,175 → 932 total skips (20% reduction). All void pointer skips eliminated. Build passes, all 16 generator tests + 35 workspace tests pass.

*2026-02-21 08:27 · 2153s · Words: 17,755 in · 9,636 out · 96,632 ctx · 189 rounds*

---

<a id="user-3"></a>

## User (3)

> Commit. Then think about whether you discovered opportunities to refactor the code the avoid repetition in this session. If refactoring seems worthwhile, do it, and commit.

*2026-02-21 08:30*

### Assistant — Claude Opus 4.6

> 💭 The user wants me to:
> 1. Commit the current changes
> 2. Think about refactoring opportunities to reduce repetition
> 3. If worthwhile, do the refactoring and commit
> 
> Let me first check the current state of changes and commit.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff --stat
```
<details>
<summary>Output (34 lines)</summary>

```
 .../2026-02-21_07-50-vscode-claude-opus-4.6.md     | 1360 ++++++++-
 crates/opencascade-binding-generator/README.md     |   32 +-
 .../src/codegen/bindings.rs                        |   62 +-
 crates/opencascade-binding-generator/src/model.rs  |    8 +-
 crates/opencascade-binding-generator/src/parser.rs |    4 +-
 .../src/type_mapping.rs                            |   35 +
 crates/opencascade-sys/generated/adaptor3d.rs      |   11 +-
 crates/opencascade-sys/generated/adv_app2_var.rs   |   83 +-
 crates/opencascade-sys/generated/ais.rs            |   23 +-
 crates/opencascade-sys/generated/aspect.rs         |   72 +-
 crates/opencascade-sys/generated/b_rep_approx.rs   |  126 +-
 crates/opencascade-sys/generated/b_rep_prim_api.rs |   80 +-
 .../opencascade-sys/generated/b_rep_top_adaptor.rs |   11 +-
 crates/opencascade-sys/generated/cdm.rs            |   51 +-
 crates/opencascade-sys/generated/extrema.rs        |   56 +-
 crates/opencascade-sys/generated/ffi.rs            | 1625 ++++++++++-
 crates/opencascade-sys/generated/geom_int.rs       |  126 +-
 crates/opencascade-sys/generated/graphic3d.rs      |   43 +-
 crates/opencascade-sys/generated/hlrb_rep.rs       | 2875 ++++++++++++++------
 crates/opencascade-sys/generated/ldom.rs           |   11 +-
 crates/opencascade-sys/generated/math.rs           |   48 +-
 crates/opencascade-sys/generated/mesh_vs.rs        |  104 +-
 crates/opencascade-sys/generated/n_collection.rs   |  280 +-
 crates/opencascade-sys/generated/osd.rs            |  193 +-
 crates/opencascade-sys/generated/poly.rs           |   26 +-
 crates/opencascade-sys/generated/prs3d.rs          |    7 +
 crates/opencascade-sys/generated/prs_mgr.rs        |    5 +
 crates/opencascade-sys/generated/select_mgr.rs     |   11 +
 crates/opencascade-sys/generated/standard.rs       |  201 +-
 crates/opencascade-sys/generated/t_doc_std.rs      |   50 +-
 .../generated/top_ope_b_rep_build.rs               |  588 +++-
 .../opencascade-sys/generated/top_ope_b_rep_ds.rs  |   15 +-
 crates/opencascade-sys/generated/wrappers.cpp      |  823 +++++-
 33 files changed, 7538 insertions(+), 1507 deletions(-)
```
</details>

Created [commit-msg.txt](../tmp/commit-msg.txt)

**Terminal:**
```sh
cat > tmp/commit-msg.txt << 'ENDCOMMIT'
Bind void pointer types (Standard_Address and void*) through FFI

Previously, ~243 methods using Standard_Address (typedef for void*) or
literal void* parameters/returns were skipped as "unknown type" because
the codegen pipeline treated them as opaque C++ classes.

Changes across the binding generator:

- model.rs: is_void_ptr() now matches "void" alongside "Standard_Address";
  class_ptr_inner_name() excludes void types from class pointer handling
- type_mapping.rs: type_uses_unknown_class/handle recognize void types as
  known; map_type_to_rust maps Standard_Address -> *mut c_void and
  void -> c_void (pointee only, pointer wrapping by outer MutPtr/ConstPtr);
  map_type_in_context has matching separate arms
- codegen/bindings.rs: void type exclusions in type_to_rust_string,
  return_type_to_rust_string, has_by_value_class_or_handle_params,
  build_param_binding (by-value conversion + nullable bypass),
  CppDeletable checks in is_method_bindable, is_static_method_bindable,
  and compute_all_function_bindings, static method lifetime check,
  inherited method conversion
- parser.rs: updated comment to reflect void types are now bound

Void pointer functions are marked unsafe via the existing needs_unsafe_fn()
mechanism. Nullable void* params stay as raw *mut c_void rather than
being wrapped in Option<&mut c_void>.

Result: 1,175 -> 932 total skips (20% reduction, 243 fewer).
README updated with new counts.
ENDCOMMIT
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
git add -A && git commit -F tmp/commit-msg.txt
```
```
[main 276243a] Bind void pointer types (Standard_Address and void*) through FFI
 33 files changed, 7538 insertions(+), 1507 deletions(-)
```

Committed. Now let me analyze the repetition patterns I encountered during this Committed. Now let me analyze the repetition patterns I encountered during this session.

**Todo list:**
- ✅ Commit void pointer changes
- 🔄 Analyze repetition patterns
- ⬜ Implement refactoring if worthwhile
- ⬜ Commit refactoring

Let me look at the specific repetition patterns. The core issue is  repeated in ~15 places across `crates/opencascade-binding-generator/src/codegen/bindings.rs` and type_mapping.rs:

**Terminal:**
```sh
grep -n 'Standard_Address.*void\|void.*Standard_Address' crates/opencascade-binding-generator/src/codegen/bindings.rs crates/opencascade-binding-generator/src/type_mapping.rs crates/opencascade-binding-generator/src/model.rs | grep -v '//' | cat
```
<details>
<summary>Output (15 lines)</summary>

```
crates/opencascade-binding-generator/src/codegen/bindings.rs:479:        Type::Class(name) => !all_enums.contains(name) && name != "char" && name != "Standard_Address" && name != "void",
crates/opencascade-binding-generator/src/codegen/bindings.rs:639:        Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" => {
crates/opencascade-binding-generator/src/codegen/bindings.rs:657:        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "Standard_Address" && name != "void") => {
crates/opencascade-binding-generator/src/codegen/bindings.rs:669:        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void") => {
crates/opencascade-binding-generator/src/codegen/bindings.rs:869:            if name != "Standard_Address" && name != "void" {
crates/opencascade-binding-generator/src/codegen/bindings.rs:991:            if name != "Standard_Address" && name != "void" {
crates/opencascade-binding-generator/src/codegen/bindings.rs:1184:        Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" && !ffi_ctx.all_enums.contains(name) => {
crates/opencascade-binding-generator/src/codegen/bindings.rs:1860:                .map(|ty| ty.is_reference() || matches!(ty, Type::ConstPtr(inner) | Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void")))
crates/opencascade-binding-generator/src/codegen/bindings.rs:2901:                            Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" && p.ty.enum_cpp_name.is_none() => {
crates/opencascade-binding-generator/src/codegen/bindings.rs:3214:                "void" | "size_t" | "Standard_Address"
crates/opencascade-binding-generator/src/codegen/bindings.rs:3400:                if name != "Standard_Address" && name != "void" {
crates/opencascade-binding-generator/src/type_mapping.rs:382:            if class_name == "Standard_Address" || class_name == "void" {
crates/opencascade-binding-generator/src/type_mapping.rs:417:            if class_name == "Standard_Address" || class_name == "void" {
crates/opencascade-binding-generator/src/model.rs:600:            Type::Class(name) => name == "Standard_Address" || name == "void",
crates/opencascade-binding-generator/src/model.rs:643:                    Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" => Some(name.as_str()),
```
</details>

15 occurrences. There are also two different patterns:

1. **"Is this a void-pointer-related type name?"** —  (positive check, ~5 occurrences)
2. **"Is this an opaque class type (not char, not void)?"** —  (negative check, ~10 occurrences)

The `is_void_ptr()` method on  already exists but works on the full type (recursing through pointers). What's missing is a simple **name-level** predicate. Let me also check for the "char" exclusion pattern:

**Terminal:**
```sh
grep -n 'name != "char"' crates/opencascade-binding-generator/src/codegen/bindings.rs crates/opencascade-binding-generator/src/model.rs | cat
```
<details>
<summary>Output (8 lines)</summary>

```
crates/opencascade-binding-generator/src/codegen/bindings.rs:479:        Type::Class(name) => !all_enums.contains(name) && name != "char" && name != "Standard_Address" && name != "void",
crates/opencascade-binding-generator/src/codegen/bindings.rs:639:        Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" => {
crates/opencascade-binding-generator/src/codegen/bindings.rs:669:        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void") => {
crates/opencascade-binding-generator/src/codegen/bindings.rs:788:        Type::Class(name) if name != "char" => format!("OwnedPtr<{}>", name),
crates/opencascade-binding-generator/src/codegen/bindings.rs:1184:        Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" && !ffi_ctx.all_enums.contains(name) => {
crates/opencascade-binding-generator/src/codegen/bindings.rs:1860:                .map(|ty| ty.is_reference() || matches!(ty, Type::ConstPtr(inner) | Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void")))
crates/opencascade-binding-generator/src/codegen/bindings.rs:2901:                            Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" && p.ty.enum_cpp_name.is_none() => {
crates/opencascade-binding-generator/src/model.rs:643:                    Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" => Some(name.as_str()),
```
</details>

**Todo list:**
- ✅ Commit void pointer changes
- ✅ Analyze repetition patterns
- 🔄 Implement refactoring if worthwhile
- ⬜ Commit refactoring

The pattern is clear. There are two distinct concepts being checked by name:

1. **** —  — the type name represents a void pointer
2. ** — the type name is a real opaque C++ class (not a primitive mapped to a special Rust type)

I'll add these as methods on  and also as standalone `fn` helpers (since many call sites have just a `&str` name, not a ). Let me look at the model.rs Type impl to add them:

<details>
<summary>Reading lines 594-610 of <a href="../crates/opencascade-binding-generator/src/model.rs#L594-L611">model.rs</a></summary>

```
    }

    /// Check if this is a void pointer type (Standard_Address = void*, or literal void*)
    /// Methods with these types are bound as `unsafe fn` with `*mut c_void` types.
    pub fn is_void_ptr(&self) -> bool {
        match self {
            Type::Class(name) => name == "Standard_Address" || name == "void",
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_void_ptr()
            }
            _ => false,
        }
    }

    /// Check if this type is a C-style array (e.g., gp_Pnt[8])
    pub fn is_array(&self) -> bool {
        match self {
```
</details>

Now let me also check where `class_ptr_inner_name` lives:

<details>
<summary>Reading lines 630-660 of <a href="../crates/opencascade-binding-generator/src/model.rs#L630-L661">model.rs</a></summary>

```
        }
    }

    /// Check if this type is a raw pointer to a class type (e.g., `const SomeClass*` or `SomeClass*`).
    /// Returns the inner class name if so. Excludes:
    /// - `const char*` (handled as strings)
    /// - Pointer-to-pointer (`T**`)
    /// - Reference-to-pointer (`T*&`)
    /// - Primitive type pointers (`int*`, `double*`, etc.)
    pub fn class_ptr_inner_name(&self) -> Option<&str> {
        match self {
            Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                match inner.as_ref() {
                    Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" => Some(name.as_str()),
                    _ => None,
                }
            }
            _ => None,
        }
    }


    /// Similar to `Param::is_nullable_ptr()` but operates on a bare `Type` without
    /// requiring a `has_default` check. Excludes `const char*` (handled as strings).
    pub fn is_nullable_ptr(&self) -> bool {
        match self {
            Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => false,
            Type::ConstPtr(_) | Type::MutPtr(_) => true,
            _ => false,
        }
    }
```
</details>

Good. The plan:

1. Add `pub fn is_void_type_name(name: &str) -> bool` to model.rs (standalone function)
2. Add `pub fn is_opaque_class_name(name: &str) -> bool` to model.rs (standalone function) — returns true for names that are real C++ classes (not char, not void types)
3. Replace all scattered inline checks with these helpers

Let me also check the `crates/opencascade-binding-generator/src/type_mapping.rs` usages more closely:Now let me implement the refactoring. First, add the two helper functions in model.rs, then replace all call sites:

<details>
<summary>Reading lines 494-500 of <a href="../crates/opencascade-binding-generator/src/model.rs#L494-L501">model.rs</a></summary>

```
}

impl Type {
    /// Get a short name for this type (for generating overload suffixes)
    pub fn short_name(&self) -> String {
        match self {
            Type::Void => "void".to_string(),
```
</details>

I'll add the helpers just before the  block:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L494-L509">model.rs -2+15</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "}\n\n/// Check if a class name represents a void pointer type.\n/// Standard_Address is a typedef for void*, and \"void\" is the parsed\n/// form of literal void* parameters.\npub fn is_void_type_name(name: &str) -> bool {\n    name == \"Standard_Address\" || name == \"void\"\n}\n\n/// Check if a class name is a real opaque C++ class (not a primitive\n/// mapped to a special Rust type like char or void pointer types).\npub fn is_opaque_class_name(name: &str) -> bool {\n    name != \"char\" && !is_void_type_name(name)\n}\n",
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "494:io,495:dd"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/model.rs","startLine":494,"endLine":508}
```
</details>

Now update `is_void_ptr` to use the new helper:

<details>
<summary>Reading lines 608-622 of <a href="../crates/opencascade-binding-generator/src/model.rs#L608-L623">model.rs</a></summary>

```

    /// Check if this is a void pointer type (Standard_Address = void*, or literal void*)
    /// Methods with these types are bound as `unsafe fn` with `*mut c_void` types.
    pub fn is_void_ptr(&self) -> bool {
        match self {
            Type::Class(name) => name == "Standard_Address" || name == "void",
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_void_ptr()
            }
            _ => false,
        }
    }

    /// Check if this type is a C-style array (e.g., gp_Pnt[8])
    pub fn is_array(&self) -> bool {
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L613-L614">model.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            Type::Class(name) => is_void_type_name(name),",
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "613:cn"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/model.rs","startLine":613,"endLine":613}
```
</details>

Now update `class_ptr_inner_name`:

<details>
<summary>Reading lines 650-662 of <a href="../crates/opencascade-binding-generator/src/model.rs#L650-L663">model.rs</a></summary>

```
    /// - Reference-to-pointer (`T*&`)
    /// - Primitive type pointers (`int*`, `double*`, etc.)
    pub fn class_ptr_inner_name(&self) -> Option<&str> {
        match self {
            Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                match inner.as_ref() {
                    Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" => Some(name.as_str()),
                    _ => None,
                }
            }
            _ => None,
        }
    }
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L656-L657">model.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "                    Type::Class(name) if is_opaque_class_name(name) => Some(name.as_str()),",
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "656:lv"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/model.rs","startLine":656,"endLine":656}
```
</details>

Good. Now do the `crates/opencascade-binding-generator/src/type_mapping.rs` replacements. Let me read the relevant sections: Now let me update type_mapping.rs.  replacements. Let me read the relevant sections: Now let me update type_mapping.rs. The two functions  and  need to use `is_void_type_name`:

<details>
<summary>Reading lines 375-430 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L375-L431">type_mapping.rs</a></summary>

```
    match ty {
        Type::Handle(class_name) => !all_classes.contains(class_name),
        Type::Class(class_name) => {
            if all_classes.contains(class_name) {
                return false;
            }
            // Void pointer types — Standard_Address (void*) and literal "void" — are known
            if class_name == "Standard_Address" || class_name == "void" {
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
            if class_name == "Standard_Address" || class_name == "void" {
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
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L381-L385">type_mapping.rs -4+4</a>, <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L416-L420">-4+4</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            // Void pointer types — Standard_Address (void*) and literal \"void\" — are known\n            if crate::model::is_void_type_name(class_name) {\n                return false;\n            }",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "381:pm,382:pz,383:ta,384:uo"
    },
    {
      "content": "            // Void pointer types — Standard_Address (void*) and literal \"void\" — are known\n            if crate::model::is_void_type_name(class_name) {\n                return false;\n            }",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "416:pm,417:pz,418:ta,419:uo"
    }
  ]
}
```
**Output:**
```
{
  "applied": 2,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "startLine": 381,
      "endLine": 419
    }
  ]
}
```
</details>

Now update the  functions — these have  in separate match arms, which is fine (they have different behavior), but the comments could reference the helper. Actually these are already cleanly separated — `Standard_Address` maps to `*mut c_void` while `void` maps to just `c_void`. That separation is intentional, not repetition.

Now the big one — bindings.rs. Let me find all the spots:  Let me read the relevant `crates/opencascade-binding-generator/src/codegen/bindings.rs` sections to do all the replacements. I need to update many spots. Let me read them all:

<details>
<summary>Reading lines 476-485 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L476-L486">bindings.rs</a></summary>

```
/// These need C++ wrappers that accept const T& instead.
fn has_by_value_class_or_handle_params(params: &[Param], all_enums: &HashSet<String>) -> bool {
    params.iter().any(|p| match &p.ty {
        Type::Class(name) => !all_enums.contains(name) && name != "char" && name != "Standard_Address" && name != "void",
        Type::Handle(_) => true,
        _ => false,
    })
}

/// Check if params contain any &mut enum output parameters.
```
</details>

<details>
<summary>Reading lines 606-614 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L606-L615">bindings.rs</a></summary>

```
        Type::Class(name) => {
            if name == "char" {
                "std::ffi::c_char".to_string()
            } else if name == "Standard_Address" {
                "*mut std::ffi::c_void".to_string()
            } else if name == "void" {
                "std::ffi::c_void".to_string()
            } else if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
```
</details>

This one has intentionally different behavior per name — not a good candidate for the helper.

<details>
<summary>Reading lines 636-676 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L636-L677">bindings.rs</a></summary>

```
/// Convert a return Type to Rust type string for re-export files
fn return_type_to_rust_string(ty: &Type, reexport_ctx: Option<&ReexportTypeContext>) -> String {
    match ty {
        Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" => {
            let inner = if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            };
            format!("crate::OwnedPtr<{}>", inner)
        }
        Type::Handle(name) => {
            format!(
                "crate::OwnedPtr<crate::ffi::{}>",
                type_mapping::handle_type_name(name)
            )
        }
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "std::string::String".to_string()
        }
        // Class pointer returns -> Option<&T> / Option<&mut T>
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "Standard_Address" && name != "void") => {
            if let Type::Class(name) = inner.as_ref() {
                let resolved = if let Some(ctx) = reexport_ctx {
                    ctx.resolve_class(name)
                } else {
                    format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
                };
                format!("Option<&{}>", resolved)
            } else {
                unreachable!()
            }
        }
        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void") => {
            if let Type::Class(name) = inner.as_ref() {
                let resolved = if let Some(ctx) = reexport_ctx {
                    ctx.resolve_class(name)
                } else {
                    format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
                };
                format!("Option<&mut {}>", resolved)
```
</details>

Now let me also check the three CppDeletable spots and other usages. Let me do all the replacements at once. First I need to use `use crate::model::{is_void_type_name, is_opaque_class_name}` in bindings.rs:

<details>
<summary>Reading lines 1-15 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1-L16">bindings.rs</a></summary>

```
//! Shared intermediate representation for binding decisions.
//!
//! `ClassBindings` computes all filtering, naming, overload suffixes,
//! and conflict resolution for a class **once**. The emit functions for
//! ffi.rs, wrappers.hxx, and per-module re-exports consume this struct
//! without re-deriving any decisions.

use crate::model::{Constructor, Method, Param, ParsedClass, ParsedField, StaticMethod, Type};
use crate::module_graph;
use crate::resolver::{self, SymbolTable};
use crate::type_mapping::{self, map_return_type, map_return_type_in_context, map_type_in_context, map_type_to_rust, TypeContext};
use heck::ToSnakeCase;
use std::fmt::Write as _;
use std::collections::{HashMap, HashSet};

```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L8-L9">bindings.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "use crate::model::{Constructor, Method, Param, ParsedClass, ParsedField, StaticMethod, Type, is_void_type_name, is_opaque_class_name};",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "8:uf"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":8,"endLine":8}
```
</details>

Now replace all the occurrences. Let me gather the line hashes for each spot. I'll batch the hashline_read calls:

<details>
<summary>Reading lines 479-482 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L479-L483">bindings.rs</a></summary>

```
        Type::Class(name) => !all_enums.contains(name) && name != "char" && name != "Standard_Address" && name != "void",
        Type::Handle(_) => true,
        _ => false,
    })
```
</details>

<details>
<summary>Reading lines 866-875 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L866-L876">bindings.rs</a></summary>

```
        // all_class_names for param filtering don't have generated destructors.
        // Enum types are represented as Type::Class in raw parsed types — allow them.
        if let Type::Class(name) = ret {
            if name != "Standard_Address" && name != "void" {
                if let Some(deletable) = ctx.deletable_class_names {
                    if !deletable.contains(name.as_str()) && !ctx.all_enums.contains(name.as_str()) {
                        return Err(format!("return type '{}' is not CppDeletable", name));
                    }
                }
            }
```
</details>

<details>
<summary>Reading lines 988-997 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L988-L998">bindings.rs</a></summary>

```
        // Same CppDeletable check as for instance methods (see is_method_bindable).
        // Enum types are represented as Type::Class in raw parsed types — allow them.
        if let Type::Class(name) = ret {
            if name != "Standard_Address" && name != "void" {
                if let Some(deletable) = ctx.deletable_class_names {
                    if !deletable.contains(name.as_str()) && !ctx.all_enums.contains(name.as_str()) {
                        return Err(format!("return type '{}' is not CppDeletable", name));
                    }
                }
            }
```
</details>

<details>
<summary>Reading lines 1183-1190 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1183-L1191">bindings.rs</a></summary>

```
    let effective_ty = match ty {
        Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" && !ffi_ctx.all_enums.contains(name) => {
            Type::ConstRef(Box::new(ty.clone()))
        }
        Type::Handle(_) => {
            Type::ConstRef(Box::new(ty.clone()))
        }
        _ => ty.clone(),
```
</details>

<details>
<summary>Reading lines 1855-1865 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1855-L1866">bindings.rs</a></summary>

```
                .map(|ty| build_return_type_binding(ty, ffi_ctx, reexport_ctx));

            let needs_static_lifetime = method
                .return_type
                .as_ref()
                .map(|ty| ty.is_reference() || matches!(ty, Type::ConstPtr(inner) | Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void")))
                .unwrap_or(false);

            StaticMethodBinding {
                ffi_fn_name,
                impl_method_name: impl_method_name.clone(),
```
</details>

<details>
<summary>Reading lines 2898-2910 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2898-L2911">bindings.rs</a></summary>

```

                        // Convert by-value class/handle params to const ref (same as build_param_binding)
                        let effective_ty = match &p.ty.original {
                            Type::Class(name) if name != "char" && name != "Standard_Address" && name != "void" && p.ty.enum_cpp_name.is_none() => {
                                Type::ConstRef(Box::new(p.ty.original.clone()))
                            }
                            Type::Handle(_) => {
                                Type::ConstRef(Box::new(p.ty.original.clone()))
                            }
                            _ => p.ty.original.clone(),
                        };
                        let cpp_arg_expr = if let Some(ref enum_name) = p.ty.enum_cpp_name {
                            format!("static_cast<{}>({})", enum_name, p.name)
```
</details>

<details>
<summary>Reading lines 3398-3410 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3398-L3411">bindings.rs</a></summary>

```
            // CppDeletable check for return types (same as class methods)
            if let Type::Class(name) = &ret.original {
                if name != "Standard_Address" && name != "void" {
                    if let Some(ref deletable) = ffi_ctx.deletable_class_names {
                        if !deletable.contains(name.as_str()) && !ffi_ctx.all_enums.contains(name.as_str()) {
                            skipped.push(SkippedSymbol {
                                kind: "function",
                                module: func.rust_module.clone(),
                                cpp_name: format!("{}::{}", func.namespace, func.short_name),
                                source_header: func.source_header.clone(),
                                source_line: func.source_line,
                                doc_comment: func.doc_comment.clone(),
                                skip_reason: format!("return type '{}' is not CppDeletable", name),
```
</details>

Now let me also read the header-collection spot at line 3214:

<details>
<summary>Reading lines 3210-3220 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3210-L3221">bindings.rs</a></summary>

```
    match ty {
        Type::Class(name) => {
            if matches!(name.as_str(),
                "bool" | "char" | "int" | "unsigned" | "float" | "double" |
                "void" | "size_t" | "Standard_Address"
            ) {
                return;
            }
            // For nested types (Parent::Nested), include the parent class header
            if name.contains("::") {
                if let Some(parent) = name.split("::").next() {
```
</details>

This one includes additional names like "bool", "int", etc. — it's about which types DON'T need headers. Not quite the same pattern, but could use a comment. I'll leave it.

Now let me also check the stub generation line 788:

<details>
<summary>Reading lines 785-794 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L785-L795">bindings.rs</a></summary>

```
/// Class/Handle types get wrapped in OwnedPtr; references stay as references.
fn stub_return_type_string(ty: &Type) -> String {
    match ty {
        Type::Class(name) if name != "char" => format!("OwnedPtr<{}>", name),
        Type::Handle(name) => format!("OwnedPtr<Handle<{}>>", name),
        _ => ty.to_rust_type_string_safe(),
    }
}

/// Common filter for instance methods (both direct and wrapper)
```
</details>

Line 788 uses  but not the void exclusion. Since this is for stub generation (commented-out code for skipped symbols), Standard_Address and void getting `OwnedPtr` wrapping in stubs is acceptable — they're just informational comments. Leave it.

Now do all the replacements:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L479-L480">bindings.rs -1+1</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L639-L640">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L657-L658">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L669-L670">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L869-L870">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L991-L992">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1184-L1185">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1860-L1861">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2901-L2902">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3400-L3401">-1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::Class(name) => !all_enums.contains(name) && is_opaque_class_name(name),",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "479:dd"
    },
    {
      "content": "        Type::Class(name) if is_opaque_class_name(name) => {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "639:zy"
    },
    {
      "content": "        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if !is_void_type_name(name)) => {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "657:fm"
    },
    {
      "content": "        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if is_opaque_class_name(name)) => {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "669:vt"
    },
    {
      "content": "            if !is_void_type_name(name) {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "869:fk"
    },
    {
      "content": "            if !is_void_type_name(name) {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "991:fk"
    },
    {
      "content": "        Type::Class(name) if is_opaque_class_name(name) && !ffi_ctx.all_enums.contains(name) => {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1184:wo"
    },
    {
      "content": "                .map(|ty| ty.is_reference() || matches!(ty, Type::ConstPtr(inner) | Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if is_opaque_class_name(name))))",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1860:tq"
    },
    {
      "content": "                            Type::Class(name) if is_opaque_class_name(name) && p.ty.enum_cpp_name.is_none() => {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2901:bm"
    },
    {
      "content": "                if !is_void_type_name(name) {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "3400:vu"
    }
  ]
}
```
**Output:**
```
{
  "applied": 10,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "startLine": 479,
      "endLine": 3400
    }
  ]
}
```
</details>

Now there's also a third CppDeletable check pattern I should look for — the one I missed (the one in compute_all_function_bindings at the original line 3400). I already updated it. And there could be some in the  function where it checks  /  specifically. Those use  already conceptually, but they have different actions for each. Let me use  in the  too — it has `if name == "Standard_Address"` / `else if name == "void"`, which is fine as-is since they map differently.

Let me also check the  ConstPtr arm, which used  without checking char:

<details>
<summary>Reading lines 656-660 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L656-L661">bindings.rs</a></summary>

```
        // Class pointer returns -> Option<&T> / Option<&mut T>
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if !is_void_type_name(name)) => {
            if let Type::Class(name) = inner.as_ref() {
                let resolved = if let Some(ctx) = reexport_ctx {
                    ctx.resolve_class(name)
```
</details>

The ConstPtr(Class("char")) case is already handled by the  arm at line 653, so  is correct — it just needs to not match void types. Technically `char` would still match here but it's caught by the earlier arm. Good.

Now compile to verify:

**Terminal:**
```sh
cargo check -p opencascade-binding-generator 2>&1
```
```
    Blocking waiting for file lock on build directory
    Checking opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.10s
```

Compiles cleanly. Now run the binding generator tests:

**Terminal:**
```sh
time ./scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (37 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.49s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-3404803fe4cd90de)

running 16 tests
test module_graph::tests::test_extract_rust_type_name ... ok
test header_deps::tests::test_is_likely_occt_header ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test parser::tests::test_extract_template_arg ... ok
test parser::tests::test_extract_module_from_header ... ok
test parser::tests::test_map_standard_type ... ok
test resolver::tests::test_safe_param_name ... ok
test resolver::tests::test_safe_method_name ... ok
test type_mapping::tests::test_map_class ... ok
test type_mapping::tests::test_map_const_ref ... ok
test type_mapping::tests::test_map_primitives ... ok
test type_mapping::tests::test_map_return_type ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/occt_bindgen-eda4fc796a0d21f2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests opencascade_binding_generator

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

real  7.0s
```
</details>

All 16 tests pass. Now regenerate bindings and do a full build to confirm the refactoring is behavior-preserving:

**Terminal:**
```sh
touch crates/opencascade-binding-generator/src/model.rs crates/opencascade-binding-generator/src/codegen/bindings.rs crates/opencascade-binding-generator/src/type_mapping.rs && cargo build --release -p opencascade-binding-generator 2>&1
```

**Terminal:**
```sh
time ./scripts/regenerate-bindings.sh 2>&1
```

Let me verify the generated output is identical:

**Terminal:**
```sh
git diff --stat -- crates/opencascade-sys/generated/
```
```
(base) whitelily:opencascade-rs sethml$
```

No changes to generated output — the refactoring is purely behavioral-preserving. Let me verify the source changes:

**Terminal:**
```sh
git diff --stat
```
```
 .../src/codegen/bindings.rs                        | 22 +++++++++++-----------
 crates/opencascade-binding-generator/src/model.rs  | 17 +++++++++++++++--
 .../src/type_mapping.rs                            |  4 ++--
 3 files changed, 28 insertions(+), 15 deletions(-)
```

Clean refactoring — 3 source files changed, zero generated output changes. Let me commit:

**Todo list:**
- ✅ Commit void pointer changes
- ✅ Analyze repetition patterns
- ✅ Implement refactoring if worthwhile
- 🔄 Commit refactoring

**Terminal:**
```sh
cat > tmp/commit-msg.txt << 'ENDCOMMIT'
Extract is_void_type_name/is_opaque_class_name helpers

The void pointer binding changes introduced ~15 repetitions of
`name != "Standard_Address" && name != "void"` and similar patterns
across bindings.rs, type_mapping.rs, and model.rs.

Extract two helper functions in model.rs:
- is_void_type_name(name) — true for "Standard_Address" or "void"
- is_opaque_class_name(name) — true for real C++ class names
  (not char, not void types)

Replace all scattered inline checks with these helpers. Purely
mechanical refactoring — generated output is identical.
ENDCOMMIT
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
git add -A && git commit -F tmp/commit-msg.txt
```
```
[main 77580a0] Extract is_void_type_name/is_opaque_class_name helpers
 3 files changed, 28 insertions(+), 15 deletions(-)
```

**Todo list:**
- ✅ Commit void pointer changes
- ✅ Analyze repetition patterns
- ✅ Implement refactoring if worthwhile
- ✅ Commit refactoring

Done. Two commits:

1. **`276243a`** — Bind void pointer types (Standard_Address and void*) through FFI — 243 fewer skips (1,175 → 932)
2. **`77580a0`** — Extract / helpers — replaced ~15 scattered inline  checks with two named functions in model.rs. Generated output identical.

*2026-02-21 08:35 · 323s · Words: 1,772 in · 1,876 out · 113,269 ctx · 45 rounds*

---
