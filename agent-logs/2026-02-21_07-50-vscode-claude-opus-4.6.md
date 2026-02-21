User: Update this section. There are leftover tools in tmp/ which might help with counting skipped symbols. Get rid of descriptions of already-fixed problems - this should just be a snapshot of the current state.

GitHub Copilot: Created 3 todos

Read [](file:///Users/sethml/src/opencascade-rs/tmp)

Read [](file:///Users/sethml/src/opencascade-rs/tmp/count_skips.py)

Read [](file:///Users/sethml/src/opencascade-rs/tmp/bucket_skips2.py#1-1), lines 1 to 80

Read [](file:///Users/sethml/src/opencascade-rs/tmp/analyze_skips.py#1-1), lines 1 to 80

Read [](file:///Users/sethml/src/opencascade-rs/tmp/analyze_skips.py#80-80), lines 80 to 160

Read [](file:///Users/sethml/src/opencascade-rs/tmp/bucket_skips2.py#80-80), lines 80 to 99

Good. Let me run the most comprehensive analysis script to get current counts. 

Starting: *Get current skip counts* (2/3)

Ran terminal command:  python3 tmp/analyze_skips.py 2>&1

Read [](file:///Users/sethml/Library/Application%20Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01NY1tXvEMHKfeHx8YVCjrGA__vscode-1771608612660/content.txt#1-1), lines 1 to 120

Read [](file:///Users/sethml/Library/Application%20Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01NY1tXvEMHKfeHx8YVCjrGA__vscode-1771608612660/content.txt#120-120), lines 120 to 189

Now let me get a cleaner categorization that properly buckets the "Other" template types: 

Created [](file:///Users/sethml/src/opencascade-rs/tmp/readme_skips.py)

Ran terminal command:  python3 tmp/readme_skips.py 2>&1

The void* category at 206 seems really high compared to the old count of 200 (old) vs 206 now. Let me also check what the big unknown types are more carefully: 

Ran terminal command:  grep -r "Reason:" crates/opencascade-sys/generated/*.rs | grep -i "BinObjMgt_SRelocationTable" | head -5

I see - my regex matches `unknown type '...'` - let me check my script more carefully. The issue is the regex `type '([^']+)' is unknown` but the reason says `uses unknown type 'BinObjMgt_SRelocationTable&'`. Let me fix the extraction: 

Created [](file:///Users/sethml/src/opencascade-rs/tmp/readme_skips.py)

Ran terminal command:  rm /Users/sethml/src/opencascade-rs/tmp/readme_skips.py

Created [](file:///Users/sethml/src/opencascade-rs/tmp/readme_skips.py)

Ran terminal command:  python3 tmp/readme_skips.py 2>&1

Starting: *Update README skipped symbols section* (3/3)

Now I have all the data. Let me also check the exact line range of the section to update in the README: 

Reading lines 365-460 of [README.md](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md#L365-L461)
Completed with input: {
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md",
  "startLine": 365,
  "endLine": 460
}

Read [](file:///Users/sethml/Library/Application%20Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01QS2gGA2BWWyY4wjHzXGN4z__vscode-1771608612671/content.txt)

Now I also need to check the same section in the root README.md since the user's selection was from that file:Now let me also check if the root README.md has the same section: 

Searched for text `## Skipped Symbols` (`**/README.md`), 1 result

The section is only in the binding generator README. The user's active file is README.md. Let me also check what the section looks like in the root README (lines 367-455) that was the selection attachment:Good, now let me replace the entire Skipped Symbols section (lines 367-454) with updated content: 

Edited: [README.md -86+76](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md#L369-L445)
Completed with input: {
  "edits": [
    {
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md",
      "lineHashes": "369:kv,370:dd,371:zg,372:nf,373:gd,374:th,375:dd,376:xr,377:fj,378:xw,379:mh,380:il,381:ea,382:pi,383:dd,384:km,385:dd,386:fa,387:oi,388:fr,389:aj,390:hj,391:pv,392:wg,393:ty,394:yd,395:dn,396:dh,397:bv,398:pe,399:nd,400:ar,401:vx,402:ek,403:dd,404:ph,405:dd,406:fg,407:dd,408:js,409:bt,410:km,411:bn,412:ak,413:iv,414:hp,415:vv,416:ab,417:dd,418:ev,419:oy,420:dn,421:mf,422:ih,423:vl,424:uk,425:fv,426:xl,427:tk,428:vd,429:fj,430:gm,431:dd,432:hi,433:dd,434:rg,435:dd,436:ew,437:dd,438:zl,439:dd,440:ta,441:dd,442:sg,443:dd,444:wj,445:dd,446:sm,447:dd,448:xt,449:dd,450:ko,451:cr,452:zo,453:dd,454:br",
      "content": "The binding generator skips ~1,175 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Every skipped symbol is documented in the generated per-module `.rs` files as a `// SKIPPED:` comment block including:\n\n- **Source location** (header file, line number, C++ symbol name)\n- **Documentation comment** from the C++ header (first 3 lines)\n- **Skip reason** explaining why the symbol was excluded\n- **Commented-out Rust stub** showing the best-guess declaration\n\nExample from `gp.rs`:\n```rust\n// SKIPPED: **Source:** `gp_XYZ.hxx`:109 - `gp_XYZ::GetData`\n//   method: Returns a const ptr to coordinates location.\n//   Reason: has unbindable types: return: raw pointer (const double*)\n//   // pub fn get_data(&self) -> /* const double* */;\n```\n\n### Skip Reason Breakdown\n\n| Count | % | Category | Description |\n|------:|----:|----------|-------------|\n| 425 | 36.2% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`BinObjMgt_SRelocationTable`, `Graphic3d_ZLayerId`, `XmlObjMgt_Element`, `IMeshData::IEdgeHandle`, etc.) |\n| 215 | 18.3% | **Ambiguous lifetimes** | `&mut` return with reference params — Rust lifetime inference is ambiguous |\n| 206 | 17.5% | **Void pointer** | `Standard_Address` (typedef for `void*`) — cannot be safely expressed in Rust FFI |\n| 95 | 8.1% | **Unresolved template type** | Template instantiations that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `NCollection_Vec3<...>`, etc.) |\n| 83 | 7.1% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(ShapePersistent_Geom::...)`, `Handle(BVH_Builder<...>)`, etc.) |\n| 72 | 6.1% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |\n| 19 | 1.6% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |\n| 19 | 1.6% | **Stream (shared_ptr)** | `std::shared_ptr<std::istream/ostream>` — smart-pointer-wrapped streams not yet bindable |\n| 17 | 1.4% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |\n| 12 | 1.0% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across FFI |\n| 5 | 0.4% | **Not CppDeletable** | Return type class has no destructor in the binding set |\n| 4 | 0.3% | **&mut enum return** | Mutable reference to enum (cxx limitation) |\n| 2 | 0.2% | **Excluded by bindings.toml** | Explicitly excluded in config (e.g., ambiguous overload workarounds) |\n| 1 | 0.1% | **Ambiguous overload** | C++ overload that would produce identical wrapper signatures |\n\n### Most Common Unknown Types\n\nThe \"unknown type\" and \"unknown Handle type\" categories (43% of all skips) are dominated by a few types:\n\n| Count | Type | How to Unblock |\n|------:|------|----------------|\n| 58 | `BinObjMgt_SRelocationTable` | Add `BinObjMgt_SRelocationTable` — used in binary persistence (BinMDataStd, BinMXCAFDoc) |\n| 40 | `Graphic3d_ZLayerId` | Typedef for `Standard_Integer` — needs typedef resolution |\n| 37 | `void*` (return type) | Would need raw pointer return support for non-method contexts |\n| 23 | `Standard_Utf32Char` | Map as `u32` in type_mapping.rs |\n| 19 | `XmlObjMgt_Element` | Add `XmlObjMgt_Element` — used in XML persistence (XmlMDataStd) |\n| 13 | `std::istream&` | Map bare `std::istream&` params (not `Standard_IStream`) — mostly in RWGltf/RWObj readers |\n| 12 | `IMeshData::IEdgeHandle` | Nested handle typedef in meshing internals — low priority |\n| 12 | `Handle(ShapePersistent_Geom::...)` | Nested template Handle types in shape persistence — low priority |\n| 11 | `Graphic3d_ArrayFlags` | Typedef for `Standard_Integer` — needs typedef resolution |\n| 11 | `ShapeProcess::OperationsFlags` | Nested type in ShapeProcess — used in STEP/IGES processing flags |\n| 10 | `IMeshData::IFaceHandle` | Nested handle typedef in meshing internals — low priority |\n| 10 | `NCollection_String` | Add `NCollection_String` — used in Graphic3d and Font modules |\n\n### Important Skipped Symbols\n\nMost skipped symbols are in internal, low-use, or specialized modules. However, some affect functionality that users commonly need:\n\n**Data Exchange (33 symbols)** — `STEPControl_*` (5), `IGESControl_*` (3), `XSControl_*` (11), `RWGltf_*` (7), `RWObj_*` (4), `RWStl` (2), `RWPly` (1). Dominated by unknown types (17) and string ref params (8, mostly `const char*&` in `XSControl_Vars`), plus rvalue references (3) and abstract classes (2). The core `Read()`/`Write()` operations are fully bound.\n\n**Document Framework (16 symbols)** — `TDocStd_*` (8), `TDF_*` (8). Mostly ambiguous lifetimes (11) from `&mut` return methods, plus a few unknown types and one void pointer.\n\n**Shape Meshing (90 symbols across 3 modules)** — `BRepMesh_*` (75), `IMeshData_*` (14), `IMeshTools_*` (1). Many BRepMesh methods reference internal mesh data types (`IMeshData::IEdgeHandle`, `IMeshData::IFaceHandle`, `IMeshData::MapOfInteger`) that are nested typedefs not yet resolvable. Also includes C-style array params and `std::pair` return types. The core `BRepMesh_IncrementalMesh` meshing API is fully bound.\n\n**Shape Analysis/Fix (0 symbols)** — All symbols are fully bound.\n\n**Geometry (0 symbols in gp/Geom/Geom2d)** — All core geometry operations are available.\n\n**Poly (13 symbols)** — Ambiguous lifetimes (4), C-style arrays (3), void pointers (2), and misc others. `Poly_CoherentTriangulation` internal access and `Poly_MakeLoops` helper interfaces. All core triangulation access is available.\n\n### How Skipped Symbols Are Tracked\n\nAll filtering decisions happen in two places:\n- `codegen/bindings.rs`: `is_method_bindable()`, `is_constructor_bindable()`, `is_static_method_bindable()` return `Result<(), String>` with a human-readable reason on `Err`\n- `compute_class_bindings()` and `compute_all_function_bindings()` collect `SkippedSymbol` structs for every rejected symbol\n\nThe `emit_reexport_class()` and `emit_skipped_functions()` functions write the skip comments to the generated module files."
    }
  ]
}

Now let me also update the same section in the root README.md (the selection came from there):Now let me check if the root README has a "Skipped Symbols" section too: 

Searched for text `Skipped Symbols` (`**/README.md`), 4 results

The section only exists in the binding generator README. Now let me verify the edit looks correct: 

Reading lines 367-450 of [README.md](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md#L367-L451)
Completed with input: {
  "endLine": 450,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md",
  "startLine": 367
}

The edit looks clean. Now let me also check if the same section appears duplicated in the root README (the user's selection attachment was from README.md):Now let me check the root README.md to see if the same section is duplicated there: 

Reading lines 360-470 of [README.md](file:///Users/sethml/src/opencascade-rs/README.md#L360-L471)
Completed with input: {
  "endLine": 470,
  "filePath": "/Users/sethml/src/opencascade-rs/README.md",
  "startLine": 360
}

The root README only has 168 lines — the selection was definitely from the binding generator README (which is also the user's active file). The update is complete. 

Completed: *Update README skipped symbols section* (3/3)

Updated the Skipped Symbols section in README.md. Key changes:

- **Total count**: ~1,036 → ~1,175 (binding set has grown significantly)
- **Skip Reason Breakdown**: Updated all counts/percentages from current generated files. Notable shifts: "Raw pointer" category dropped to 0 (class pointer returns are now bound as `Option<&T>`/`Option<&mut T>`), "Unknown Handle type" grew from 7 to 83
- **Most Common Unknown Types**: Replaced with current top blockers (`BinObjMgt_SRelocationTable` at 58, `Graphic3d_ZLayerId` at 40, etc.). Removed the old `Handle(TDocStd_Document)` at 99 — no longer appears
- **Removed "Previously resolved" list**: Deleted the 11-item strikethrough history of already-fixed types
- **Important Skipped Symbols**: Updated all domain counts to current state (Data Exchange 57→33, Document Framework 74→16, Shape Analysis/Fix now at 0, Geometry now at 0). Removed historical narrative about how counts were reduced

Made changes.

User: Commit. Then: I thought we resolved this issue by mapping void pointers to unsafe functions returning ffi::c_void pointers? Look for git commits in the past 3 days that relate to functions taking or returning pointers, and figure out why they're not handling these exceptions, and fix it, then commit.

GitHub Copilot: Created 6 todos

Ran terminal command:  cd /Users/sethml/src/opencascade-rs && git add -A && cat > tmp/commit-msg.txt << 'EOF'
Update Skipped Symbols section in binding generator README

Refresh all skip counts from current generated files (1,175 total).
Remove historical "previously resolved" list and reduction narratives.
Update domain-level analysis with current counts.
EOF
git commit -F tmp/commit-msg.txt

Ran terminal command:  git log --oneline -1

Starting: *Find recent pointer-related commits* (2/6)

Ran terminal command:  git log --oneline --since="3 days ago" | grep -i "pointer\|ptr\|void\|raw\|unsafe\|address\|c_void\|ffi"

Ran terminal command:  git log --oneline --since="3 days ago" | head -30

Ran terminal command:  git show aa43d44 --stat && echo "---" && git show 9ac8bcd --stat

Let me look at the actual skip reasons to understand what void pointer skips look like: 

Ran terminal command:  grep "Reason:.*void pointer\|Reason:.*Standard_Address" crates/opencascade-sys/generated/*.rs | head -20

Ran terminal command:  grep -c "Reason:.*Standard_Address" crates/opencascade-sys/generated/*.rs

Ran terminal command:  grep -c "Reason:.*Standard_Address" crates/opencascade-sys/generated/*.rs | grep -v ':0$' | sort -t: -k2 -rn

So `Standard_Address` is being treated as an "unknown type" rather than being recognized. It's a typedef for `void*`. Let me look at how types are resolved and where `Standard_Address` is handled: 

Ran terminal command:  grep -rn "Standard_Address\|void\b.*pointer\|c_void" crates/opencascade-binding-generator/src/ | grep -v "test\|\.rs:.*//\|target" | head -40

Starting: *Investigate void pointer skips* (3/6)

So `Standard_Address` IS recognized in some places (type_mapping.rs, bindings.rs), but it's being reported as "unknown type" in the skip reasons. Let me trace how the skip decision is made. Let me look at the bindability check functions: 

Reading lines 590-750 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L590-L751)
Completed with input: {
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 590,
  "endLine": 750
}

Read [](file:///Users/sethml/Library/Application%20Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01P4SbtKoHYT8ZHX1QjneNp4__vscode-1771608612691/content.txt)

Now I need to understand the flow. The `is_unbindable()` method is key. Let me look at that:  

Searched for text `fn is_unbindable` (`**/crates/opencascade-binding-generator/src/**`), 1 result

Reading lines 700-750 of [model.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L700-L751)
Completed with input: {
  "endLine": 750,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs",
  "startLine": 700
}

So `is_unbindable()` at line 710 does NOT consider void pointers or raw pointers as unbindable. There's `needs_unsafe_fn()` at line 717, which detects them. But the skip reasons show `Standard_Address` as "unknown type", not "void pointer". This means the skipping is happening somewhere else — in the resolver's `is_known_type` check maybe.

Let me check how `is_method_bindable` works:   

Searched for text `fn is_method_bindable` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Reading lines 800-930 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L800-L931)
Completed with input: {
  "endLine": 930,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 800
}

Now let me look at the `type_uses_unknown_type` function — that's what's rejecting `Standard_Address`: 

Searched for text `fn type_uses_unknown_type` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Reading lines 455-530 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L455-L531)
Completed with input: {
  "endLine": 530,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 455
}

Now let me look at `type_uses_unknown_class` in type_mapping.rs: 

Searched for text `fn type_uses_unknown_class` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), 1 result

Reading lines 365-430 of [type_mapping.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L365-L431)
Completed with input: {
  "endLine": 430,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs",
  "startLine": 365
}

There it is! In `type_uses_unknown_class` (line 368-382) and `type_uses_unknown_handle` (line 399-413), when the type is `Type::Class("Standard_Address")`, it checks `all_classes.contains("Standard_Address")` — and `Standard_Address` is NOT in `all_classes` (it's not a parsed class, it's a typedef for `void*`). Then it checks `is_primitive_type("Standard_Address")` — let me check if that returns true:  

Searched for text `fn is_primitive_type` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), 1 result

Reading lines 112-140 of [rust.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/rust.rs#L112-L141)
Completed with input: {
  "endLine": 140,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/rust.rs",
  "startLine": 112
}

`Standard_Address` is NOT in the `is_primitive_type()` list. So the flow is:

1. Parser sees `Standard_Address` → creates `Type::Class("Standard_Address")`
2. `type_uses_unknown_class()` checks: is it in `all_classes`? No. Is it a primitive? No. Is it nested? No. → Returns `true` (unknown)
3. Method gets skipped with "uses unknown type 'Standard_Address'"

The fix: `Standard_Address` needs to be treated as known in `type_uses_unknown_class` and `type_uses_unknown_handle`. Since it's already handled in `type_to_rust_string()` (maps to `*mut std::ffi::c_void`), we just need to add it to the "known" check.

Let me also look at what `is_void_ptr()` does to understand the complete picture:  

Searched for text `fn is_void_ptr` (`**/crates/opencascade-binding-generator/src/model.rs`), 1 result

Reading lines 595-620 of [model.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L595-L621)
Completed with input: {
  "endLine": 620,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs",
  "startLine": 595
}

Now let me check the doc comment on `is_void_ptr()` — it says "These can't be bound through the FFI", but looking at the `is_unbindable()` logic (line 709-711), void pointers are NOT unbindable. And `needs_unsafe_fn()` (line 717-719) says they need unsafe. So there's existing infrastructure to handle void pointers — they're supposed to be bound as `unsafe fn` with `*mut std::ffi::c_void`.

The problem is that `type_uses_unknown_type()` flags `Standard_Address` as unknown before we even get to the point of emitting it. The fix is straightforward: add `Standard_Address` to the known-type checks in `type_uses_unknown_class` and `type_uses_unknown_handle`.

But wait — let me also check whether the `describe_unbindable_reason` function has a "void pointer" path that generates the old skip reason format. Looking at line 731, it does: `if ty.is_void_ptr() { return format!("void pointer ({})...") }`. But that path is only reached if `is_unbindable()` returns true (line 801-804 in is_method_bindable), and `is_unbindable()` doesn't include void pointers. So the "void pointer" reason is from the OLD code before these changes were made.

Wait, let me re-read the skip reason from the actual generated files:

```
Reason: return type 'Standard_Address' is unknown
Reason: param 'PtrSvSurfaces' uses unknown type 'Standard_Address'
```

These come from lines 852-853 and 822-823 in `is_method_bindable`. They hit the `type_uses_unknown_type` check, not the `describe_unbindable_reason` path.

So the fix: `Standard_Address` must be recognized as "known" in `type_uses_unknown_class()` and `type_uses_unknown_handle()`. Then the methods won't be skipped, and since `needs_unsafe_fn()` returns true for `is_void_ptr()`, they'll be emitted as `unsafe fn`.

Let me verify `needs_unsafe_fn` is actually used in the code generation:  

Searched for text `needs_unsafe_fn` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results

Good — `needs_unsafe_fn()` is used to mark functions as `unsafe`. Let me check how it's used in context:   

Reading lines 2955-2985 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L2955-L2986)
Completed with input: {
  "endLine": 2985,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 2955
}

Good. Now let me also understand the full codegen for `Standard_Address` — when it's a plain `Type::Class("Standard_Address")`, it gets mapped to `*mut std::ffi::c_void` in `type_to_rust_string()`. When it's e.g. `Type::ConstRef(Box::new(Type::Class("Standard_Address")))`, it becomes `&*mut std::ffi::c_void` which would be `&*mut c_void` — that doesn't make sense. Let me think about what forms `Standard_Address` appears in:  

Ran terminal command:  grep "Reason:.*Standard_Address" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | sort | uniq -c | sort -rn

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

This all seems to work. The issue is just that `type_uses_unknown_class()` / `type_uses_unknown_handle()` reject `Standard_Address` before we ever get a chance to process it.

The fix is simple: add `Standard_Address` to the known-type checks alongside the "char" primitive check. Let me also check what the `void*` (non-Standard_Address) skips look like:   

Ran terminal command:  grep "Reason:.*unknown.*void" crates/opencascade-sys/generated/*.rs | head -10

So there are also `void*` types coming through as `Type::Class("void*")` or similar. Let me check how parser produces these:   

Ran terminal command:  grep -c "unknown type 'void" crates/opencascade-sys/generated/*.rs | grep -v ':0$'

Ran terminal command:  grep "unknown type 'void" crates/opencascade-sys/generated/*.rs | sed 's/.*unknown type //' | sort | uniq -c | sort -rn

So there are also 19 skips for `void*` (not `Standard_Address`, but literal `void*`). Let me check how the parser creates these:  

Searched for regex `void\*|void_ptr|void pointer` (`**/crates/opencascade-binding-generator/src/parser.rs`), 2 results

Reading lines 1610-1640 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L1610-L1641)
Completed with input: {
  "endLine": 1640,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 1610
}

I see the comments say "can't be bound through the FFI" and "so methods using it can be filtered out." That's the OLD intent — before the recent work that added `needs_unsafe_fn()` to handle void pointers as `unsafe fn`. Let me look at what happens when we have a literal `void*` (not from Standard_Address):  

Searched for regex `void` (`**/crates/opencascade-binding-generator/src/parser.rs`), 5 results

Let me look at how `void*` types from C++ get parsed. The raw `void*` would come through as a ConstPtr/MutPtr pointing type. Let me search for how pointer types are resolved in the parser:   

Searched for regex `Pointer|pointer` (`**/crates/opencascade-binding-generator/src/parser.rs`), 6 results

Reading lines 1470-1500 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L1470-L1501)
Completed with input: {
  "endLine": 1500,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 1470
}

So for a `void*` param, clang would give `TypeKind::Pointer` with pointee `TypeKind::Void`. Let me check what `parse_type` returns for `TypeKind::Void`:   

Searched for regex `TypeKind::Void` (`**/crates/opencascade-binding-generator/src/parser.rs`), 1 result

That's only checking return types. Let me look at `parse_type` more generally to see how it handles void:    

Searched for text `fn parse_type` (`**/crates/opencascade-binding-generator/src/parser.rs`), 1 result

Reading lines 1265-1360 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L1265-L1361)
Completed with input: {
  "endLine": 1360,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 1265
}

Reading lines 1360-1470 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L1360-L1471)
Completed with input: {
  "endLine": 1470,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 1360
}

So for a raw `void*` param: clang gives `TypeKind::Pointer` with pointee `TypeKind::Void`. `parse_type` recursively calls itself for the inner pointee. But there's no check for `TypeKind::Void` in `parse_type` — what falls through?    

Reading lines 1500-1570 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L1500-L1571)
Completed with input: {
  "endLine": 1570,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 1500
}

I see. So when `parse_type` is called for a `void` pointee type (from a `void*`):
- `kind` is `TypeKind::Void`
- `spelling` is `"void"`
- `map_standard_type("void")` returns `None` (it doesn't map "void")
- It falls through everything
- Eventually reaches `Type::Class("void".to_string())` at line 1564

So `void*` becomes `MutPtr(Class("void"))`. And `type_uses_unknown_class` then sees `Class("void")` — which isn't in `all_classes`, isn't a primitive type, and isn't nested → returns `true` (unknown).

Now I understand the full picture. The fix needs to handle both:
1. `Standard_Address` — `Type::Class("Standard_Address")` 
2. `void*` — `Type::MutPtr(Type::Class("void"))` or `Type::ConstPtr(Type::Class("void"))`

Both should be recognized as known types and map through as void pointers.

Let me now look at how `is_void_ptr()` handles these:

Looking at line 598-606: `is_void_ptr()` only checks `Type::Class(name) => name == "Standard_Address"`. It does NOT check for `Type::Class("void")`. So `MutPtr(Class("void"))` won't be caught by `is_void_ptr()` either.

Let me also update `is_void_ptr()` to catch both forms. Now let me implement the fix: 

Starting: *Fix void pointer handling* (4/6)

The plan:
1. In model.rs: Update `is_void_ptr()` to also recognize `Type::Class("void")`
2. In type_mapping.rs: In both `type_uses_unknown_class()` and `type_uses_unknown_handle()`, add `"Standard_Address" | "void"` alongside "char" as known Class names
3. In `type_to_rust_string()` and `return_type_to_rust_string()`: Make sure `void` maps to appropriate ptr type
4. Update the parser comment that says "can't be bound"

Let me also check `describe_unbindable_reason` to make sure the old void pointer path still works even though `is_unbindable()` won't fire for these:

Looking at the code flow again:
- `is_unbindable()` (line 709) does NOT include void ptrs — correct, they ARE bindable
- `is_method_bindable()` (line 800): first checks `has_unbindable_types()` (which calls `is_unbindable()` on all params/return), then checks `type_uses_unknown_type()`. The unknown type check is what's rejecting Standard_Address.

So the fix is in `type_uses_unknown_class()` / `type_uses_unknown_handle()` — recognize `Standard_Address` and `void` as known types. Let me also double-check what `type_to_rust_string` does for `Type::Class("void")`:

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

Reading lines 85-105 of [cpp.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/cpp.rs#L85-L106)
Completed with input: {
  "endLine": 105,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/cpp.rs",
  "startLine": 85
}

Good — C++ side already handles `"void"` and `"Standard_Address"`. Now let me also check the cpp.rs wrapper generation for how `Standard_Address` params/returns get their C++ type:     

Searched for text `Standard_Address` (`**/crates/opencascade-binding-generator/src/codegen/cpp.rs`), 2 results

OK, cpp.rs just skips the header inclusion — good. The actual C++ type string comes from `to_cpp_string()` on the Type. For `Type::Class("Standard_Address")`, that returns `"Standard_Address"`, which will work in C++ since it's a typedef for `void*`.

For `Type::Class("void")`, `to_cpp_string()` would return `"void"`. So `MutPtr(Class("void"))` → `"void*"` in C++, which is fine.

Now let me also check the `return_type_to_rust_string` and `type_to_rust_string` in the cpp.rs for the FFI layer:      

Searched for text `fn ffi_fn_type_string` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Let me check what function generates the FFI type string for params/returns:      

Searched for regex `fn.*ffi_type|fn.*rust_ffi|to_rust_ffi` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Let me look at how the FFI declarations are generated. There should be code that converts types to FFI strings. Let me search for where `ffi.rs` content is generated:      

Searched for regex `fn type_to_ffi|ffi_param_type|ffi_return_type|fn.*_ffi_type` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `ffi_safe|ffi.*type|extern.*C.*fn` (`**/crates/opencascade-binding-generator/src/codegen/**`), 20 results

Let me look at `build_param_binding` to see how FFI types are generated:       

Reading lines 1028-1130 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1028-L1131)
Completed with input: {
  "endLine": 1130,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 1028
}

Let me look at where the standard param binding is built (no special cases):        

Reading lines 1130-1230 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1130-L1231)
Completed with input: {
  "endLine": 1230,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 1130
}

Looking at line 1177-1184: when we have `Type::Class(name)` and `name != "char"` and it's not an enum, it gets wrapped in `ConstRef`. So for `Type::Class("Standard_Address")`, it would become `ConstRef(Class("Standard_Address"))` which means the C++ gets `const Standard_Address*` — that's `const void**`. That's not right for a void pointer parameter.

Wait, but Standard_Address is already `void*`. So as a param it's pass-by-value `void*`. The Rust side would be `*mut std::ffi::c_void`. But the effective_ty conversion wraps it in `ConstRef`, making it `const void**` on the C++ side and `&*mut std::ffi::c_void` on the Rust side. That would be wrong.

Actually wait — I need to also check `class_ptr_inner_name()` — it might handle this:  

Searched for text `fn class_ptr_inner_name` (`**/crates/opencascade-binding-generator/src/model.rs`), 1 result

Reading lines 639-670 of [model.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L639-L671)
Completed with input: {
  "endLine": 670,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs",
  "startLine": 639
}

At line 643: `class_ptr_inner_name` matches `Type::ConstPtr(Class(name))` where `name != "char"`. So `ConstPtr(Class("Standard_Address"))` and `MutPtr(Class("void"))` would match, returning `Some("Standard_Address")` or `Some("void")`.

But since `Standard_Address` comes as a bare `Type::Class("Standard_Address")` (not wrapped in a pointer), `class_ptr_inner_name()` would return `None` for it. And `void*` comes as `MutPtr(Class("void"))`, so `class_ptr_inner_name()` returns `Some("void")`. That means `void*` params would go through the class pointer binding path (line 1132), which produces `*mut crate::ffi::void` in the FFI and `&mut crate::ffi::void` in the reexport — both wrong since there's no `void` FFI type.

OK, I need to think about this more carefully. The `Standard_Address` and `void*` cases need special handling in several places. Let me think about the right approach:

**For `Type::Class("Standard_Address")` (a bare void pointer, since `Standard_Address` IS `void*`):**
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
1. **model.rs**: Update `is_void_ptr()` to also match `Type::Class("void")`
2. **type_mapping.rs**: In `type_uses_unknown_class()` and `type_uses_unknown_handle()`, treat `"Standard_Address"` and `"void"` as known (like "char")
3. **bindings.rs `type_to_rust_string()`**: Add `"void"` case mapping to `std::ffi::c_void`
4. **bindings.rs `build_param_binding()`**: Exclude `"Standard_Address"` and `"void"` from the by-value class→ConstRef conversion (line 1178)
5. **bindings.rs `class_ptr_inner_name()`**: Also exclude `"Standard_Address"` and `"void"` so `MutPtr(Class("void"))` isn't treated as a class pointer
6. **bindings.rs `return_type_to_rust_string()`**: Handle `void` case
7. **parser.rs**: Update comment

Let me also check `class_ptr_inner_name` exclusion for Standard_Address:

Looking at line 667 of bindings.rs: `Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != "char" && name != "Standard_Address")` — Standard_Address IS already excluded from the MutPtr return type path but NOT from class_ptr_inner_name.

Let me also check what `has_by_value_class_or_handle_params` does:

At line 479: `Type::Class(name) => !all_enums.contains(name) && name != "char"` — doesn't exclude `Standard_Address` or `void`.

Let me also look at `map_type_in_context` in type_mapping.rs to see how it handles these:         

Reading lines 180-475 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L180-L476)
Completed with input: {
  "endLine": 475,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 180
}

Let me look at `map_type_in_context`:          

Reading lines 422-480 of [type_mapping.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L422-L481)
Completed with input: {
  "endLine": 480,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs",
  "startLine": 422
}

I see: `map_type_in_context` for `Type::Class("Standard_Address")` falls through to the general `Type::Class(class_name)` case (line 434), which doesn't handle it specially — it would try to look up module and use it as an FFI class name. That's wrong.

And for `Type::Class("void")`, same issue — it would produce `void` as an FFI type name, which doesn't exist.

OK, I have a complete picture now. Let me implement all the changes: Now let me implement all changes. I need to be systematic about this. Let me define a helper concept: `is_void_type_name(name)` which returns true for `"Standard_Address"` or `"void"`. I need to apply this across multiple files.

Let me start with the edits:

**1. model.rs** - Update `is_void_ptr()` to also match `"void"` 

Edited: [model.rs -3+2](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L596-L598), [-1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L599-L600)
Completed with input: {
  "edits": [
    {
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "596:vk,597:cx,598:bo",
      "content": "    /// Check if this is a void pointer type (Standard_Address = void*, or literal void*)\n    /// Methods with these types are bound as `unsafe fn` with `*mut c_void` types."
    },
    {
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "600:dh",
      "content": "            Type::Class(name) => name == \"Standard_Address\" || name == \"void\","
    }
  ]
}

**2. model.rs** - Update `class_ptr_inner_name()` to exclude void types          

Reading lines 639-650 of [model.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L639-L651)
Completed with input: {
  "endLine": 650,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs",
  "startLine": 639
}

Edited: [model.rs -1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L642-L643)
Completed with input: {
  "edits": [
    {
      "content": "                    Type::Class(name) if name != \"char\" && name != \"Standard_Address\" && name != \"void\" => Some(name.as_str()),",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "642:tv"
    }
  ]
}

**3. type_mapping.rs** - Update `type_uses_unknown_class()` and `type_uses_unknown_handle()`  

Reading lines 365-420 of [type_mapping.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L365-L421)
Completed with input: {
  "endLine": 420,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs",
  "startLine": 365
}

Edited: [type_mapping.rs -1+5](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L372-L377), [-1+5](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L407-L412)
Completed with input: {
  "edits": [
    {
      "content": "            // Void pointer types — Standard_Address (void*) and literal \"void\" — are known\n            if class_name == \"Standard_Address\" || class_name == \"void\" {\n                return false;\n            }\n            // Primitive types mapped as Type::Class (e.g., \"char\" from Standard_Character)",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "372:qi"
    },
    {
      "content": "            // Void pointer types — Standard_Address (void*) and literal \"void\" — are known\n            if class_name == \"Standard_Address\" || class_name == \"void\" {\n                return false;\n            }\n            // Primitive types mapped as Type::Class (e.g., \"char\" from Standard_Character)",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "403:qi"
    }
  ]
}

**4. type_mapping.rs** - Update `map_type_in_context()` to handle Standard_Address and void   

Reading lines 426-445 of [type_mapping.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L426-L446)
Completed with input: {
  "endLine": 445,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs",
  "startLine": 426
}

Edited: [type_mapping.rs -9+18](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L433-L451)
Completed with input: {
  "edits": [
    {
      "content": "        Type::Class(class_name) if class_name == \"char\" => {\n            // C++ char resolved from canonical types (e.g., Standard_Character)\n            RustTypeMapping {\n                rust_type: \"std::ffi::c_char\".to_string(),\n                needs_unique_ptr: false,\n                needs_pin: false,\n                source_module: None,\n            }\n        }\n        Type::Class(class_name) if class_name == \"Standard_Address\" || class_name == \"void\" => {\n            // void pointer types — map to *mut c_void\n            RustTypeMapping {\n                rust_type: \"*mut std::ffi::c_void\".to_string(),\n                needs_unique_ptr: false,\n                needs_pin: false,\n                source_module: None,\n            }\n        }",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "433:hh,434:da,435:mu,436:ik,437:wh,438:rl,439:nk,440:uo,441:cy"
    }
  ]
}

**5. bindings.rs** - Update `type_to_rust_string()` to handle "void"    

Reading lines 606-618 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L606-L619)
Completed with input: {
  "endLine": 618,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 606
}

Edited: [bindings.rs -1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L609-L610)
Completed with input: {
  "edits": [
    {
      "content": "            } else if name == \"Standard_Address\" || name == \"void\" {",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "609:sq"
    }
  ]
}

**6. bindings.rs** - Update `return_type_to_rust_string()` to exclude void types from OwnedPtr and class pointer paths      

Reading lines 634-680 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L634-L681)
Completed with input: {
  "endLine": 680,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 634
}

I need to add `"void"` to the exclusions: 

Edited: [bindings.rs -1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L637-L638), [-1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L655-L656), [-1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L667-L668)
Completed with input: {
  "edits": [
    {
      "content": "        Type::Class(name) if name != \"char\" && name != \"Standard_Address\" && name != \"void\" => {",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "637:kr"
    },
    {
      "content": "        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != \"Standard_Address\" && name != \"void\") => {",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "655:us"
    },
    {
      "content": "        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != \"char\" && name != \"Standard_Address\" && name != \"void\") => {",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "667:qk"
    }
  ]
}

Wait, the ConstPtr guard on line 655 was `Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(_))` — it matched ANY class, but the char case was handled earlier. Now I need to also exclude void types. But the "char" case is already excluded by a prior match arm (line 651). So I need to exclude Standard_Address and void from the ConstPtr class arm too. Let me verify my edit was right:   

Reading lines 634-680 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L634-L681)
Completed with input: {
  "endLine": 680,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 634
}

Good. Now **7. bindings.rs** - Update `build_param_binding()` to exclude void types from the by-value class→ConstRef wrapping     

Reading lines 1173-1190 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1173-L1191)
Completed with input: {
  "endLine": 1190,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 1173
}

Edited: [bindings.rs -1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1178-L1179)
Completed with input: {
  "edits": [
    {
      "content": "        Type::Class(name) if name != \"char\" && name != \"Standard_Address\" && name != \"void\" && !ffi_ctx.all_enums.contains(name) => {",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1178:md"
    }
  ]
}

**8. bindings.rs** - Update `has_by_value_class_or_handle_params()` to exclude void types        

Reading lines 476-484 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L476-L485)
Completed with input: {
  "endLine": 484,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 476
}

Edited: [bindings.rs -1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L479-L480)
Completed with input: {
  "edits": [
    {
      "content": "        Type::Class(name) => !all_enums.contains(name) && name != \"char\" && name != \"Standard_Address\" && name != \"void\",",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "479:zv"
    }
  ]
}

**9.** Now let me check for other places that have `name != "char"` patterns to exclude Standard_Address that I might have missed:       

Ran terminal command:  grep -n 'name != "char"' crates/opencascade-binding-generator/src/codegen/bindings.rs

Let me check lines 786, 1854, and 2895:        

Reading lines 780-792 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L780-L793)
Completed with input: {
  "endLine": 792,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 780
}

Line 786 is for stub declarations (in SKIPPED comments) — doesn't affect compilation. Fine to leave. 

Reading lines 1845-1860 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1845-L1861)
Completed with input: {
  "endLine": 1860,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 1845
}

Line 1854 already has `name != "Standard_Address"` — let me also add `"void"`:   

Edited: [bindings.rs -1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1854-L1855)
Completed with input: {
  "edits": [
    {
      "content": "                .map(|ty| ty.is_reference() || matches!(ty, Type::ConstPtr(inner) | Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name != \"char\" && name != \"Standard_Address\" && name != \"void\")))",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1854:cj"
    }
  ]
}

Now line 2895:         

Reading lines 2885-2910 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L2885-L2911)
Completed with input: {
  "endLine": 2910,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 2885
}

Edited: [bindings.rs -1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L2895-L2896)
Completed with input: {
  "edits": [
    {
      "content": "                            Type::Class(name) if name != \"char\" && name != \"Standard_Address\" && name != \"void\" && p.ty.enum_cpp_name.is_none() => {",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2895:nu"
    }
  ]
}

Now let me also search for other patterns I might have missed across the whole codebase — checking for `name == "Standard_Address"` exclusions that need `"void"` added too:           

Ran terminal command:  grep -n '"Standard_Address"' crates/opencascade-binding-generator/src/codegen/bindings.rs

Line 3208 — let me check that:            

Reading lines 3200-3215 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L3200-L3216)
Completed with input: {
  "endLine": 3215,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 3200
}

That's fine — it's for C++ header collection, already handles "void" and "Standard_Address".

**10. Update the parser comment:**            

Edited: [parser.rs -2+2](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L1623-L1625)
Completed with input: {
  "edits": [
    {
      "content": "        // Standard_Address is void* — bound as *mut c_void in unsafe functions.\n        // Represented as Type::Class(\"Standard_Address\") so is_void_ptr() can detect it.",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "1623:ui,1624:lx"
    }
  ]
}

Now let me also handle the `void` case in the parser. When `parse_type` encounters a `void` pointee, it falls through to `Type::Class("void")`. That's fine — our changes handle this. But let me also make sure the `void` return type case (`TypeKind::Void`) is handled properly. Looking at line 1257:             

Reading lines 1253-1265 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L1253-L1266)
Completed with input: {
  "endLine": 1265,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 1253
}

That's fine — `void` as a return type (plain, not pointer) returns `None`. The `Type::Void` variant is only for the `()` return type. When a `void*` return is parsed, it goes through `Pointer(Void)` → `parse_type` on the pointee will produce `Type::Class("void")` since `TypeKind::Void` isn't handled in `parse_type`. But wait, I should double check...

Actually, the pointee of `void*` has `TypeKind::Void`. And in `parse_type`, `TypeKind::Void` with spelling `"void"` passes through:
- `map_standard_type("void")` → None (not mapped)
- Not size_t
- Not a typedef
- Canonical: "void"
- `canonical_clean` = "void"
- Not in the primitive matches  
- Falls through to `Type::Class("void")` at line 1564

Yes, this is confirmed. `void*` produces `MutPtr(Class("void"))`. Our changes handle this.

Now let me also check the `describe_unbindable_reason` function — the void pointer check there: 

Reading lines 728-737 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L728-L738)
Completed with input: {
  "endLine": 737,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 728
}

This is only reached when `is_unbindable()` returns true (for arrays, rvalue refs, unresolved templates). Since void pointers and raw pointers are NOT unbindable (only "needs_unsafe_fn"), this path will never fire for them. But if it did fire, line 731 would catch it. This is fine.

Now let me also look at the `describe_unbindable_types_method` and similar functions that call `describe_unbindable_reason`:

Looking at line 689: `if p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none()` — this describes params that ARE unbindable. Void pointers aren't unbindable, so they won't reach here. Good.

Now let me also handle the `is_method_bindable` return type check:               

Reading lines 851-877 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L851-L878)
Completed with input: {
  "endLine": 877,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 851
}

Line 856: `class_ptr_inner_name()` now excludes "Standard_Address" and "void", so they won't trigger this check. Good.

Line 866-872: The CppDeletable check for `Type::Class(name)` — does `Standard_Address` get caught here? If a method returns `Standard_Address` (void*), the return type is `Type::Class("Standard_Address")`, and line 866 matches. Then it checks `deletable_class_names`. Standard_Address is NOT a parsed class and wouldn't be in deletable. But it's also not an enum. So it would return an error "not CppDeletable".

I need to add an exclusion for void types here:                 

Edited: [bindings.rs -7+9](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L866-L875)
Completed with input: {
  "edits": [
    {
      "content": "        if let Type::Class(name) = ret {\n            if name != \"Standard_Address\" && name != \"void\" {\n                if let Some(deletable) = ctx.deletable_class_names {\n                    if !deletable.contains(name.as_str()) && !ctx.all_enums.contains(name.as_str()) {\n                        return Err(format!(\"return type '{}' is not CppDeletable\", name));\n                    }\n                }\n            }\n        }",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "866:ke,867:vy,868:se,869:yb,870:gl,871:uo,872:cy"
    }
  ]
}

Summarized conversation historyNow let me check `is_constructor_bindable` and `is_static_method_bindable` for similar patterns: 

Reading lines 876-960 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L876-L961)
Completed with input: {
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 876,
  "endLine": 960
}

Reading lines 960-1040 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L960-L1041)
Completed with input: {
  "endLine": 1040,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 960
}

I see the same CppDeletable pattern at line 988-994 in `is_static_method_bindable`. Let me also check `is_constructor_bindable` — constructors don't return custom types (they return `Self`), so it probably doesn't have such a check. Confirmed — no such check in constructor. Let me fix the static method one: 

Edited: [bindings.rs -7+9](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L988-L997)
Completed with input: {
  "edits": [
    {
      "content": "        if let Type::Class(name) = ret {\n            if name != \"Standard_Address\" && name != \"void\" {\n                if let Some(deletable) = ctx.deletable_class_names {\n                    if !deletable.contains(name.as_str()) && !ctx.all_enums.contains(name.as_str()) {\n                        return Err(format!(\"return type '{}' is not CppDeletable\", name));\n                    }\n                }\n            }\n        }",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "988:ke,989:vy,990:se,991:yb,992:gl,993:uo,994:cy"
    }
  ]
}

Now let me also check that `has_unbindable_types()` doesn't flag void pointers. The `is_void_ptr()` is separate from `is_unbindable()`: 

Starting: *Verify type_uses_unknown_type logic* (2/6)

Let me verify that `type_uses_unknown_type` properly handles all void pointer variants. The fix was in `type_uses_unknown_class` and `type_uses_unknown_handle`. Let me check `type_uses_unknown_type`: 

Searched for text `fn type_uses_unknown_type` (`**/type_mapping.rs`), no results

Searched for text `fn type_uses_unknown_type` (`**/crates/**`), 1 result

Reading lines 455-478 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L455-L479)
Completed with input: {
  "endLine": 478,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 455
}

Good — `type_uses_unknown_type` delegates to `type_uses_unknown_class` and `type_uses_unknown_handle`, which we already fixed to recognize Standard_Address and void as known types. And it checks enums first.

Now let me also verify there's no issue with `type_to_cpp_string` for void types being passed through correctly in C++ wrapper generation. Let me check how the C++ wrappers handle void pointer params: 

Searched for regex `Standard_Address|void\*|c_void` (`**/crates/opencascade-binding-generator/src/codegen/**`), 14 results

Looks comprehensive. Let me also check line 3212 to understand that context:  

Reading lines 3205-3220 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L3205-L3221)
Completed with input: {
  "endLine": 3220,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 3205
}

Good — void, size_t, and Standard_Address are already excluded from header collection. This was already there.

Now let me compile to verify everything works: 

Starting: *Compile and test* (3/6)

Ran terminal command:  time cargo check -p opencascade-binding-generator 2>&1
Completed with input:  time cargo check -p opencascade-binding-generator 2>&1

There's a parsing error in model.rs. Let me check the area around where I made changes:   

Reading lines 590-620 of [model.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L590-L621)
Completed with input: {
  "endLine": 620,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs",
  "startLine": 590
}

The function definition line for `is_void_ptr` is missing! Line 598 goes directly to `match self {` without the `pub fn is_void_ptr(&self) -> bool {`. Let me check what happened: 

Edited: [model.rs -3+4](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L596-L600)
Completed with input: {
  "edits": [
    {
      "content": "    /// Check if this is a void pointer type (Standard_Address = void*, or literal void*)\n    /// Methods with these types are bound as `unsafe fn` with `*mut c_void` types.\n    pub fn is_void_ptr(&self) -> bool {\n        match self {",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "596:vv,597:lr,598:dk"
    }
  ]
}

Ran terminal command:  time cargo check -p opencascade-binding-generator 2>&1

Compiles cleanly. Now let me regenerate the bindings to see the skip count improvement:  

Starting: *Regenerate bindings* (4/6)

Ran terminal command:  time ./scripts/regenerate-bindings.sh 2>&1

Ran terminal command:  grep -E "Total (bound|skipped)" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01UUK5wDjMEXbGv1diNkdoSz__vscode-1771608612778/content.txt
Completed with input:  grep -E "Total (bound|skipped)" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01UUK5wDjMEXbGv1diNkdoSz__vscode-1771608612778/content.txt

Ran terminal command:  grep -i "total\|bound\|skip" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01UUK5wDjMEXbGv1diNkdoSz__vscode-1771608612778/content.txt | grep -i "total"

Let me look at the tail of the regeneration output: 

Read [](file:///Users/sethml/Library/Application%20Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01UUK5wDjMEXbGv1diNkdoSz__vscode-1771608612778/content.txt#374-374), lines 374 to 374

Ran terminal command:  wc -l /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01UUK5wDjMEXbGv1diNkdoSz__vscode-1771608612778/content.txt

Read [](file:///Users/sethml/Library/Application%20Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01UUK5wDjMEXbGv1diNkdoSz__vscode-1771608612778/content.txt#300-300), lines 300 to 373

The bindings were generated. Let me now check the skip count by running the analysis script:  

Ran terminal command:  cargo run -p opencascade-binding-generator -- --config crates/opencascade-binding-generator/config.yaml --skip-report 2>&1 | grep -i "standard_address\|void\|total.*skip\|total.*bound\|skipped.*method\|bound.*method" | head -20

Hmm, `--skip-report` may not be a valid flag. Let me check:   

Ran terminal command:  cargo run -p opencascade-binding-generator -- --help 2>&1
Completed with input:  cargo run -p opencascade-binding-generator -- --help 2>&1

Right — needs the clang library path. The regenerate script sets that up. Let me use the count script from tmp/ instead:  

Ran terminal command:  python3 tmp/count_skips.py 2>&1

Ran terminal command:  python3 tmp/count_skips.py 2>&1 | grep -i "total\|standard_address\|void"

Excellent! Down from **1,175** to **936** skipped (239 fewer skips, including reducing void pointer skips from 225 to just 4). Let me check what those remaining 4 are:   

Ran terminal command:  python3 tmp/count_skips.py 2>&1 | grep -i "void" -A2

Let me find these remaining 4 void pointer skips specifically:   

Ran terminal command:  grep -r "void" crates/opencascade-sys/generated/skipped_*.txt 2>/dev/null | grep -iv "//\|#\|Standard_Address" | head -20

Ran terminal command:  grep -rn "void" crates/opencascade-sys/generated/skip_report.txt 2>/dev/null | head -20 ; ls crates/opencascade-sys/generated/skip* 2>/dev/null
Completed with input:  grep -rn "void" crates/opencascade-sys/generated/skip_report.txt 2>/dev/null | head -20 ; ls crates/opencascade-sys/generated/skip* 2>/dev/null

Ran terminal command:  ls crates/opencascade-sys/generated/*.txt 2>/dev/null
Completed with input:  ls crates/opencascade-sys/generated/*.txt 2>/dev/null

The count_skips.py script must parse the generated Rust files. Let me look at what it considers "void pointer" skips:   

Reading lines 1-30 of [count_skips.py](file:///Users/sethml/src/opencascade-rs/tmp/count_skips.py#L1-L31)
Completed with input: {
  "endLine": 30,
  "filePath": "/Users/sethml/src/opencascade-rs/tmp/count_skips.py",
  "startLine": 1
}

So it searches for `"void pointer" in reason or "Standard_Address" in reason`. Let me directly search for those entries: 

Ran terminal command:  grep -rn "Reason:.*void\|Reason:.*Standard_Address" crates/opencascade-sys/generated/*.rs 2>/dev/null

4 remaining "not CppDeletable" skips for Standard_Address! My fix should have handled this. Let me check the current state of the code:  

Reading lines 860-880 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L860-L881)
Completed with input: {
  "endLine": 880,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 860
}

The fix looks correct. But wait — the generated code still shows the old error. The bindings were regenerated with regenerate-bindings.sh which builds the generator first. Let me check — did the regeneration use the RIGHT binary by doing a fresh build? Let me re-check — the script regenerates AND builds the generator. Let me see the entries more closely:   

Reading lines 59620-59660 of [standard.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/standard.rs#L59620-L59661)
Completed with input: {
  "endLine": 59660,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/standard.rs",
  "startLine": 59620
}

These are static methods of the `Standard` class. My CppDeletable fix was applied to BOTH `is_method_bindable` (line 867) and `is_static_method_bindable` (line 989). But the regeneration must have used the old binary. Let me check the static method fix:  

Reading lines 985-1000 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L985-L1001)
Completed with input: {
  "endLine": 1000,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 985
}

The fix is in place. The regeneration script must be using a cached binary. Let me re-run it with a fresh build:   

Ran terminal command:  cargo build -p opencascade-binding-generator 2>&1 && time ./scripts/regenerate-bindings.sh 2>&1 | grep -E "Compiling|Skipped free|Standard_Address|void" | head -20

No more Standard_Address skips in the output! Let me verify:   

Ran terminal command:  grep -rn "Reason:.*void\|Reason:.*Standard_Address" crates/opencascade-sys/generated/*.rs 2>/dev/null

Still there! The regeneration script might be producing a different binary or using release mode. Let me check what it does:   

Reading lines 1-40 of [regenerate-bindings.sh](file:///Users/sethml/src/opencascade-rs/scripts/regenerate-bindings.sh#L1-L41)
Completed with input: {
  "endLine": 40,
  "filePath": "/Users/sethml/src/opencascade-rs/scripts/regenerate-bindings.sh",
  "startLine": 1
}

Reading lines 40-70 of [regenerate-bindings.sh](file:///Users/sethml/src/opencascade-rs/scripts/regenerate-bindings.sh#L40-L71)
Completed with input: {
  "endLine": 70,
  "filePath": "/Users/sethml/src/opencascade-rs/scripts/regenerate-bindings.sh",
  "startLine": 40
}

Line 41: `cargo build --release`! My earlier `cargo check` was dev mode. But the script builds release. And since I built with `cargo build -p opencascade-binding-generator` (debug mode), the release binary is still old! The `time ./scripts/regenerate-bindings.sh` on line 41 should have rebuilt it with `--release`. Wait, let me check — `cargo build --release` was already run... but the output said `Finished` in 0.20s which means it was already up to date.

Actually wait — the `cargo build -p opencascade-binding-generator` at the start used dev mode, so it compiled for dev. But the script uses `--release`. Let me check — `cargo build --release` should detect the source changed and recompile. Let me verify the release binary was actually recompiled:    

Ran terminal command:  ls -la target/release/occt-bindgen | cat

Ran terminal command:  date

The timestamp shows 08:02 — from the first regeneration. The grep | head may have prevented seeing the compilation step. Let me force rebuild in release:   

Ran terminal command:  cargo build --release -p opencascade-binding-generator 2>&1

Cargo says "Finished" without recompiling! The source changes might not be saved to disk. Let me check:    

Ran terminal command:  git diff --stat

The changes ARE on disk (bindings.rs, model.rs, type_mapping.rs, parser.rs all show changes). But cargo isn't detecting the source file changes. Let me `touch` the files:    

Ran terminal command:  touch crates/opencascade-binding-generator/src/codegen/bindings.rs crates/opencascade-binding-generator/src/model.rs crates/opencascade-binding-generator/src/type_mapping.rs crates/opencascade-binding-generator/src/parser.rs && cargo build --release -p opencascade-binding-generator 2>&1

Now it recompiled. The earlier build must have cached the old binary because cargo's mtime was ahead. Now regenerate:    

Ran terminal command:  time ./scripts/regenerate-bindings.sh 2>&1

Ran terminal command:  grep -rn "Reason:.*void\|Reason:.*Standard_Address" crates/opencascade-sys/generated/*.rs 2>/dev/null

Still there! These are "free functions" — not instance or static methods of a class. There must be a separate bindability check for free functions. Let me find it:   

Searched for regex `free.*function.*bindable|is_free_function|fn.*free.*bind` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `free_function|Skipped free|FreeFn|free_fn` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 3 results

Searched for regex `fn is_.*_bindable` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results

There's `is_static_method_bindable` and `is_params_bindable`. Free functions in OCCT classes (like `Standard::Allocate`) should be treated as static methods. Let me see where free functions get filtered. Let me search for where free functions' skips are determined:    

Searched for regex `skipped_free|free_function.*skip|skip.*free_function` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `free_func|FreeFun` (`**/crates/opencascade-binding-generator/src/codegen/**`), 2 results

Searched for regex `FunctionBinding|function_binding|bound_free|bound_function` (`**/crates/opencascade-binding-generator/src/codegen/**`), 20 results

The key function is `compute_all_function_bindings` at line 3255. Let me check how it filters functions:    

Reading lines 3255-3350 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L3255-L3351)
Completed with input: {
  "endLine": 3350,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 3255
}

Reading lines 3350-3460 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L3350-L3461)
Completed with input: {
  "endLine": 3460,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 3350
}

Found it! Lines 3396-3413 — a **third** CppDeletable check, this one for free functions, and it does NOT have my void type exclusion! Let me fix it:     

Edited: [bindings.rs -18+20](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L3396-L3416)
Completed with input: {
  "edits": [
    {
      "content": "            // CppDeletable check for return types (same as class methods)\n            if let Type::Class(name) = &ret.original {\n                if name != \"Standard_Address\" && name != \"void\" {\n                    if let Some(ref deletable) = ffi_ctx.deletable_class_names {\n                        if !deletable.contains(name.as_str()) && !ffi_ctx.all_enums.contains(name.as_str()) {\n                            skipped.push(SkippedSymbol {\n                                kind: \"function\",\n                                module: func.rust_module.clone(),\n                                cpp_name: format!(\"{}::{}\", func.namespace, func.short_name),\n                                source_header: func.source_header.clone(),\n                                source_line: func.source_line,\n                                doc_comment: func.doc_comment.clone(),\n                                skip_reason: format!(\"return type '{}' is not CppDeletable\", name),\n                                stub_rust_decl: generate_function_stub(func),\n                            });\n                            continue;\n                        }\n                    }\n                }\n            }",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "3396:pq,3397:dx,3398:ys,3399:qc,3400:ia,3401:ne,3402:gr,3403:us,3404:xo,3405:hy,3406:ts,3407:xg,3408:jf,3409:oz,3410:jr,3411:or,3412:gl,3413:uo"
    }
  ]
}

Now touch and rebuild, then regenerate:     

Ran terminal command:  touch crates/opencascade-binding-generator/src/codegen/bindings.rs && cargo build --release -p opencascade-binding-generator 2>&1

Ran terminal command:  time ./scripts/regenerate-bindings.sh 2>&1

Ran terminal command:  grep -rn "Reason:.*void\|Reason:.*Standard_Address" crates/opencascade-sys/generated/*.rs 2>/dev/null
Completed with input:  grep -rn "Reason:.*void\|Reason:.*Standard_Address" crates/opencascade-sys/generated/*.rs 2>/dev/null

Zero void/Standard_Address skips remaining. Let me check the final skip count:     

Ran terminal command:  python3 tmp/count_skips.py 2>&1 | grep -i "total\|void"

Down from **1,175** to **932** — 243 fewer skips, and void pointer category is completely gone! Now let me do the full build and test:      

Starting: *Full build and test* (5/6)

Ran terminal command:  time cargo build 2>&1
Completed with input:  time cargo build 2>&1

Ran terminal command:  grep -c "^error" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01P31zPM9zrBnKbzY32jE9vz__vscode-1771608612822/content.txt ; grep "Finished\|error\[" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01P31zPM9zrBnKbzY32jE9vz__vscode-1771608612822/content.txt

Build errors — type mismatches. Let me look at them more closely:     

Read [](file:///Users/sethml/Library/Application%20Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01P31zPM9zrBnKbzY32jE9vz__vscode-1771608612822/content.txt#1-1), lines 1 to 100

The errors show `*mut void` in the FFI declarations — meaning `Type::Class("void")` is not being mapped to `*mut std::ffi::c_void` in the extern "C" FFI layer. The FFI generation must use a different type mapping path. Let me check:  

Searched for regex `fn.*emit.*ffi|fn.*generate.*ffi|fn.*ffi_type|extern.*C.*block` (`**/crates/opencascade-binding-generator/src/codegen/**`), 12 results

Let me check `emit_ffi_class` — that's where FFI declarations for class methods and functions are generated:      

Reading lines 5363-5440 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L5363-L5441)
Completed with input: {
  "endLine": 5440,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 5363
}

Let me see the `format_params` and `format_return_type` functions — these generate the types used in the FFI block:       

Searched for text `fn format_params` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Reading lines 5544-5570 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L5544-L5571)
Completed with input: {
  "endLine": 5570,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 5544
}

So it uses `p.rust_ffi_type` — the FFI type computed during `build_param_binding`. Let me check what `build_param_binding` produces for `Type::MutPtr(Type::Class("void"))`: 

Reading lines 1030-1210 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1030-L1211)
Completed with input: {
  "endLine": 1210,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 1030
}

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#1030-1030), lines 1030 to 1210

The issue is that `Type::MutPtr(Class("void"))` passes through `build_param_binding` to `map_type_in_context`. My fix in `map_type_in_context` only handles the bare `Type::Class("void")`, not `MutPtr(Class("void"))`. Let me check the `map_type_in_context` function:      

Searched for text `fn map_type_in_context` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), 1 result

Reading lines 431-530 of [type_mapping.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L431-L531)
Completed with input: {
  "endLine": 530,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs",
  "startLine": 431
}

So `MutPtr(Class("void"))` falls through to `_ => map_type_to_rust(ty)` at line 518. Let me check that function:       

Searched for text `fn map_type_to_rust` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), 1 result

Reading lines 38-130 of [type_mapping.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L38-L131)
Completed with input: {
  "endLine": 130,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs",
  "startLine": 38
}

Reading lines 130-220 of [type_mapping.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L130-L221)
Completed with input: {
  "endLine": 220,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs",
  "startLine": 130
}

Found it! Line 196: `map_type_to_rust` only handles `Name == "Standard_Address"` but NOT `"void"`. So when `MutPtr(Class("void"))` goes through `map_type_to_rust`:
1. `MutPtr` arm (line 177): recurses with `map_type_to_rust(inner)` where inner = `Class("void")`
2. `Class("void")` doesn't match "Standard_Address" (line 196), doesn't match "char" (line 205)
3. Falls to generic `Class(class_name)` → produces `rust_type: "void"` → so result is `*mut void`

I need to add "void" to line 196:         

Edited: [type_mapping.rs -1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L196-L197)
Completed with input: {
  "edits": [
    {
      "content": "        Type::Class(class_name) if class_name == \"Standard_Address\" || class_name == \"void\" => {",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "196:by"
    }
  ]
}

Now rebuild and regenerate:          

Ran terminal command:  touch crates/opencascade-binding-generator/src/type_mapping.rs && cargo build --release -p opencascade-binding-generator 2>&1

Ran terminal command:  time ./scripts/regenerate-bindings.sh 2>&1

Ran terminal command:  time cargo build 2>&1
Completed with input:  time cargo build 2>&1

Good progress — from 38 errors to just 1! The issue is a nullable pointer (`theContext: void**` which is `*mut *mut c_void`) being wrapped as `Option<&mut *mut c_void>` when it should stay as `*mut *mut c_void`. This is a `void**` param marked nullable — the re-export wrapper is incorrectly wrapping it in Option.

Let me look at the generated code:           

Reading lines 60-95 of [standard.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/standard.rs#L60-L96)
Completed with input: {
  "endLine": 95,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/standard.rs",
  "startLine": 60
}

The issue: `Class("void")` maps to `*mut std::ffi::c_void` (includes the pointer), but `void` is just the pointee — the pointer comes from the wrapping `MutPtr`/`ConstPtr`. This causes `MutPtr(Class("void"))` to produce `*mut *mut c_void` (double pointer).

Fix: `Class("void")` should map to just `std::ffi::c_void`. `Class("Standard_Address")` remains `*mut std::ffi::c_void` (it's a typedef for the whole pointer). 

Reading lines 195-206 of [type_mapping.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L195-L207)
Completed with input: {
  "endLine": 206,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs",
  "startLine": 195
}

Edited: [type_mapping.rs -9+18](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L196-L214)
Completed with input: {
  "edits": [
    {
      "content": "        Type::Class(class_name) if class_name == \"Standard_Address\" => {\n            // Standard_Address is a typedef for void* — map to raw c_void pointer\n            RustTypeMapping {\n                rust_type: \"*mut std::ffi::c_void\".to_string(),\n                needs_unique_ptr: false,\n                needs_pin: false,\n                source_module: None,\n            }\n        }\n        Type::Class(class_name) if class_name == \"void\" => {\n            // C++ void type (pointee of void*) — just c_void, pointer wrapping handled by outer type\n            RustTypeMapping {\n                rust_type: \"std::ffi::c_void\".to_string(),\n                needs_unique_ptr: false,\n                needs_pin: false,\n                source_module: None,\n            }\n        }",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "196:jk,197:sv,198:mu,199:mi,200:wh,201:rl,202:nk,203:uo,204:cy"
    }
  ]
}

Now fix the same in `map_type_in_context`:            

Reading lines 440-460 of [type_mapping.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L440-L461)
Completed with input: {
  "endLine": 460,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs",
  "startLine": 440
}

Edited: [type_mapping.rs -9+18](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L451-L469)
Completed with input: {
  "edits": [
    {
      "content": "        Type::Class(class_name) if class_name == \"Standard_Address\" => {\n            // Standard_Address is a typedef for void* — map to raw c_void pointer\n            RustTypeMapping {\n                rust_type: \"*mut std::ffi::c_void\".to_string(),\n                needs_unique_ptr: false,\n                needs_pin: false,\n                source_module: None,\n            }\n        }\n        Type::Class(class_name) if class_name == \"void\" => {\n            // C++ void type (pointee of void*) — just c_void, pointer wrapping handled by outer type\n            RustTypeMapping {\n                rust_type: \"std::ffi::c_void\".to_string(),\n                needs_unique_ptr: false,\n                needs_pin: false,\n                source_module: None,\n            }\n        }",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "451:jk,452:ug,453:mu,454:mi,455:wh,456:rl,457:nk,458:uo,459:cy"
    }
  ]
}

Now fix `type_to_rust_string` in bindings.rs similarly:             

Reading lines 600-620 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L600-L621)
Completed with input: {
  "endLine": 620,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 600
}

Edited: [bindings.rs -2+4](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L609-L613)
Completed with input: {
  "edits": [
    {
      "content": "            } else if name == \"Standard_Address\" {\n                \"*mut std::ffi::c_void\".to_string()\n            } else if name == \"void\" {\n                \"std::ffi::c_void\".to_string()",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "609:nt,610:vu"
    }
  ]
}

Now rebuild and regenerate:               

Ran terminal command:  touch crates/opencascade-binding-generator/src/type_mapping.rs crates/opencascade-binding-generator/src/codegen/bindings.rs && cargo build --release -p opencascade-binding-generator 2>&1

Ran terminal command:  time ./scripts/regenerate-bindings.sh 2>&1

Ran terminal command:  time cargo build 2>&1
Completed with input:  time cargo build 2>&1

The remaining error: nullable `void*` params get wrapped as `Option<&mut c_void>` which is nonsensical. Nullable void pointers should stay as raw `*mut c_void`. Need to skip the nullable wrapping for void pointers:  

Reading lines 1090-1100 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1090-L1101)
Completed with input: {
  "endLine": 1100,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 1090
}

Edited: [bindings.rs -1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1093-L1094)
Completed with input: {
  "edits": [
    {
      "content": "    if is_nullable && !ty.is_void_ptr() {",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1093:ps"
    }
  ]
}

Now rebuild and regenerate:   

Ran terminal command:  touch crates/opencascade-binding-generator/src/codegen/bindings.rs && cargo build --release -p opencascade-binding-generator 2>&1 && time ./scripts/regenerate-bindings.sh 2>&1

Ran terminal command:  time cargo build 2>&1

Build succeeds! Now let me run the binding generator tests and workspace tests:             

Updated todo list

Ran terminal command:  time ./scripts/run-binding-generator-tests.sh 2>&1

All 16 tests pass. Now run workspace tests:               

Ran terminal command:  time cargo test --workspace --exclude opencascade-binding-generator 2>&1

Ran terminal command:  grep "test result" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01LsYoLyk43JGqPpjBzWzMta__vscode-1771608612856/content.txt

All tests pass — 0 failures across the board! Let me update the README with the new skip count:   

Starting: *Update README counts if needed* (6/6)

Ran terminal command:  python3 tmp/count_skips.py 2>&1

Read [](file:///Users/sethml/Library/Application%20Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/68f03462-9998-4ac3-aa2d-7308008921b9/toolu_01H4a7kqukr5GMcHfjDm7k5x__vscode-1771608612859/content.txt#1-1), lines 1 to 70

932 total — zero void pointer skips. Now update the README. Let me find the Skipped Symbols section:    

Searched for text `Skipped Symbols` (`**/crates/opencascade-binding-generator/README.md`), 4 results

Reading lines 367-445 of [README.md](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md#L367-L446)
Completed with input: {
  "endLine": 445,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md",
  "startLine": 367
}

Now I need to update the counts. Let me also regenerate the categorized counts to get accurate "Most Common Unknown Types": 

Ran terminal command:  python3 tmp/readme_skips.py 2>&1

Now update the README. The key changes: total ~1,175 → ~932, void pointer row removed, counts adjusted, "void*" removed from unknown types table, Document Framework 16→15, Poly 13→11:   

Edited: [README.md -1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md#L369-L370), [-16+15](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md#L386-L401), [-1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md#L404-L405), [-14+13](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md#L406-L419), [-1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md#L426-L427), [-1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md#L434-L435)
Completed with input: {
  "edits": [
    {
      "content": "The binding generator skips ~932 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Every skipped symbol is documented in the generated per-module `.rs` files as a `// SKIPPED:` comment block including:",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md",
      "lineHashes": "369:eq"
    },
    {
      "content": "| Count | % | Category | Description |\n|------:|----:|----------|-------------|\n| 388 | 41.6% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`BinObjMgt_SRelocationTable`, `Graphic3d_ZLayerId`, `XmlObjMgt_Element`, `IMeshData::IEdgeHandle`, etc.) |\n| 215 | 23.1% | **Ambiguous lifetimes** | `&mut` return with reference params — Rust lifetime inference is ambiguous |\n| 95 | 10.2% | **Unresolved template type** | Template instantiations that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `NCollection_Vec3<...>`, etc.) |\n| 83 | 8.9% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(ShapePersistent_Geom::...)`, `Handle(BVH_Builder<...>)`, etc.) |\n| 72 | 7.7% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |\n| 19 | 2.0% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |\n| 19 | 2.0% | **Stream (shared_ptr)** | `std::shared_ptr<std::istream/ostream>` — smart-pointer-wrapped streams not yet bindable |\n| 17 | 1.8% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |\n| 12 | 1.3% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across FFI |\n| 5 | 0.5% | **Not CppDeletable** | Return type class has no destructor in the binding set |\n| 4 | 0.4% | **&mut enum return** | Mutable reference to enum (cxx limitation) |\n| 2 | 0.2% | **Excluded by bindings.toml** | Explicitly excluded in config (e.g., ambiguous overload workarounds) |\n| 1 | 0.1% | **Ambiguous overload** | C++ overload that would produce identical wrapper signatures |",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md",
      "lineHashes": "386:fa,387:oi,388:kq,389:zq,390:or,391:oa,392:bl,393:gh,394:fm,395:tt,396:ki,397:xo,398:kr,399:bo,400:vx,401:ek"
    },
    {
      "content": "The \"unknown type\" and \"unknown Handle type\" categories (50.5% of all skips) are dominated by a few types:",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md",
      "lineHashes": "405:qo"
    },
    {
      "content": "| Count | Type | How to Unblock |\n|------:|------|----------------|\n| 58 | `BinObjMgt_SRelocationTable` | Add `BinObjMgt_SRelocationTable` — used in binary persistence (BinMDataStd, BinMXCAFDoc) |\n| 40 | `Graphic3d_ZLayerId` | Typedef for `Standard_Integer` — needs typedef resolution |\n| 23 | `Standard_Utf32Char` | Map as `u32` in type_mapping.rs |\n| 19 | `XmlObjMgt_Element` | Add `XmlObjMgt_Element` — used in XML persistence (XmlMDataStd) |\n| 13 | `std::istream&` | Map bare `std::istream&` params (not `Standard_IStream`) — mostly in RWGltf/RWObj readers |\n| 12 | `IMeshData::IEdgeHandle` | Nested handle typedef in meshing internals — low priority |\n| 12 | `Handle(ShapePersistent_Geom::...)` | Nested template Handle types in shape persistence — low priority |\n| 11 | `Graphic3d_ArrayFlags` | Typedef for `Standard_Integer` — needs typedef resolution |\n| 11 | `ShapeProcess::OperationsFlags` | Nested type in ShapeProcess — used in STEP/IGES processing flags |\n| 10 | `IMeshData::IFaceHandle` | Nested handle typedef in meshing internals — low priority |\n| 10 | `NCollection_String` | Add `NCollection_String` — used in Graphic3d and Font modules |",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md",
      "lineHashes": "407:js,408:bt,409:rm,410:xi,411:ne,412:rd,413:gh,414:ss,415:iv,416:df,417:zp,418:vv,419:ab,420:lf"
    },
    {
      "content": "**Document Framework (15 symbols)** — `TDocStd_*` (7), `TDF_*` (8). Mostly ambiguous lifetimes (11) from `&mut` return methods, plus unknown types (4).",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md",
      "lineHashes": "428:ln"
    },
    {
      "content": "**Poly (11 symbols)** — Ambiguous lifetimes (4), C-style arrays (3), excluded by config (1), and misc others. `Poly_CoherentTriangulation` internal access and `Poly_MakeLoops` helper interfaces. All core triangulation access is available.",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md",
      "lineHashes": "436:eq"
    }
  ]
}

Completed: *Update README counts if needed* (6/6)

Let me verify the final state with a summary of all changes:      

Ran terminal command:  git diff --stat

Everything is done. Here's a summary of what was fixed:

**Root cause:** `Standard_Address` (typedef for `void*`) and literal `void*` (parsed as `MutPtr(Class("void"))`) were treated as unknown classes throughout the codegen pipeline, causing ~243 methods to be skipped.

**Files changed (5 source files):**

1. **model.rs** — `is_void_ptr()` now also matches `"void"` (not just `"Standard_Address"`); `class_ptr_inner_name()` excludes `"Standard_Address"` and `"void"` from class pointer handling

2. **type_mapping.rs** — Both `type_uses_unknown_class()` and `type_uses_unknown_handle()` now recognize `Standard_Address`/`void` as known types; `map_type_in_context()` and `map_type_to_rust()` map `Standard_Address` → `*mut c_void` and `void` → `c_void` (just the pointee — pointer wrapping is handled by the outer `MutPtr`/`ConstPtr`)

3. **codegen/bindings.rs** — 10+ exclusion points updated: `type_to_rust_string`, `return_type_to_rust_string`, `has_by_value_class_or_handle_params`, `build_param_binding` (by-value conversion, nullable pointer bypass for void), CppDeletable checks in all 3 bindability functions + free function pipeline, static method lifetime check, inherited method conversion

4. **parser.rs** — Updated comment to reflect that void pointer types are now bound as `*mut c_void` in unsafe functions

5. **README.md** — Updated skip counts from ~1,175 to ~932 (243 fewer skips), removed void pointer category

**Result:** 1,175 → 932 total skips (20% reduction). All void pointer skips eliminated. Build passes, all 16 generator tests + 35 workspace tests pass.