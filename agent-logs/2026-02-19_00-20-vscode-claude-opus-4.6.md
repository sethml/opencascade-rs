User: Figure out what's going on here. The generated code claims TDF_LabelMap is an unknown type, yet it's re-exported below. Fix the issue and commit.

GitHub Copilot: 

  

Created 4 todos

Let me investigate how the resolver handles collection types like `TDF_LabelMap` when checking method bindability.  

Searched for text `TDF_LabelMap` (`**/crates/opencascade-binding-generator/src/**`), 3 results

Searched for regex `unknown type|is_known|is_type_known|type_known` (`**/crates/opencascade-binding-generator/src/**`), 4 results

Reading lines 1190-1270 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1190-L1271)
Completed with input: {
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 1190,
  "endLine": 1270
}

Searched for regex `fn type_uses_unknown_type` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Reading lines 781-870 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L781-L871)
Completed with input: {
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 781,
  "endLine": 870
}

Now let me look at what `type_uses_unknown_class` does: 

Searched for regex `fn type_uses_unknown_class|fn type_uses_unknown_handle` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), 2 results

Reading lines 310-380 of [type_mapping.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L310-L381)
Completed with input: {
  "endLine": 380,
  "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
  "startLine": 310
}

Now I need to understand where `all_classes` is populated. Let me check how collection types (NCollection typedefs like `TDF_LabelMap`) are tracked:  

Searched for regex `all_classes.*insert|all_classes.*extend|all_classes.*collect` (`**/crates/opencascade-binding-generator/src/**`), 14 results

Searched for regex `collection_types|ncollection.*typedef|typedef.*ncollection` (`**/crates/opencascade-binding-generator/src/**`), 20 results

Let me look at the key section where `all_class_names` is built and whether collection types are included: 

Reading lines 3080-3150 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L3080-L3151)
Completed with input: {
  "endLine": 3150,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 3080
}

Now let me check where `collection_names` and `extra_typedef_names` are built in main.rs: 

Searched for regex `collection_type_names|extra_typedef_names` (`**/crates/opencascade-binding-generator/src/main.rs`), 9 results

Reading lines 268-300 of [main.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/main.rs#L268-L301)
Completed with input: {
  "endLine": 300,
  "filePath": "crates/opencascade-binding-generator/src/main.rs",
  "startLine": 268
}

Reading lines 578-600 of [main.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/main.rs#L578-L601)
Completed with input: {
  "endLine": 600,
  "filePath": "crates/opencascade-binding-generator/src/main.rs",
  "startLine": 578
}

Now let me check what `extra_typedef_names` includes (from `parser::get_collected_typedef_names`): 

Searched for regex `get_collected_typedef_names|NCOLLECTION_TYPEDEF_MAP` (`**/crates/opencascade-binding-generator/src/parser.rs`), 1 result

Reading lines 10-100 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L10-L101)
Completed with input: {
  "endLine": 100,
  "filePath": "crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 10
}

Now let me check which collections are in `all_known_collections()` and whether `TDF_LabelMap` is among them:  

Searched for text `TDF_LabelMap` (`**/crates/opencascade-binding-generator/src/codegen/collections.rs`), no results

Searched for regex `fn all_known_collections` (`**/crates/opencascade-binding-generator/src/codegen/collections.rs`), 1 result

So `TDF_LabelMap` is NOT in `all_known_collections()` (which is the manually curated list of collections with iterator support). But it should be in `extra_typedef_names` from the parser. Let me verify by looking at how these names are used in the two call sites.

The key finding is: In `compute_all_class_bindings`, line 3121-3122 does:
```
all_class_names.extend(collection_names.iter().cloned());
all_class_names.extend(extra_typedef_names.iter().cloned());
```

So `TDF_LabelMap` should be in `all_class_names`. But the "single header" code path (line 274) might not have the same setup. Let me check the first code path (single header parsing):  

Reading lines 260-360 of [main.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/main.rs#L260-L361)
Completed with input: {
  "endLine": 360,
  "filePath": "crates/opencascade-binding-generator/src/main.rs",
  "startLine": 260
}

Now let me check `build_symbol_table` in the resolver to see how it handles `collection_type_names`:   

Reading lines 760-830 of [resolver.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#L760-L831)
Completed with input: {
  "endLine": 830,
  "filePath": "crates/opencascade-binding-generator/src/resolver.rs",
  "startLine": 760
}

Let me check the method bindability check for return types:  

Searched for regex `return type.*is unknown|return.*unknown type` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 6 results

Reading lines 1100-1190 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1100-L1191)
Completed with input: {
  "endLine": 1190,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 1100
}

Let me check if the resolver itself makes skip decisions for methods:    

Searched for regex `is_method_bindable|type_uses_unknown|unknown type` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 1 result

Reading lines 1280-1310 of [resolver.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#L1280-L1311)
Completed with input: {
  "endLine": 1310,
  "filePath": "crates/opencascade-binding-generator/src/resolver.rs",
  "startLine": 1280
}

Reading lines 1260-1280 of [resolver.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#L1260-L1281)
Completed with input: {
  "endLine": 1280,
  "filePath": "crates/opencascade-binding-generator/src/resolver.rs",
  "startLine": 1260
}

OK, so this is the resolver checking free functions. Let me now focus on where the skip decisions are actually made at bindings time. Let me look at the skip for `TDF_DataSet::Labels` more carefully. Let me check where the "return type 'TDF_LabelMap&' is unknown" skip is produced - whether it's from `compute_class_bindings` in bindings.rs or from sketch decisions in the resolver:   

Reading lines 3300-3400 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L3300-L3401)
Completed with input: {
  "endLine": 3400,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 3300
}

Searched for regex `is_method_bindable\(` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 2 results

Reading lines 1820-1870 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1820-L1871)
Completed with input: {
  "endLine": 1870,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 1820
}

So `is_method_bindable` is called with `ffi_ctx`. Let me trace where `ffi_ctx` is set up in `compute_class_bindings`:   

Searched for regex `fn compute_class_bindings` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Reading lines 1711-1730 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1711-L1731)
Completed with input: {
  "endLine": 1730,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 1711
}

So `ffi_ctx` is passed from `compute_all_class_bindings` (line 3155), where it's built with `all_class_names` that includes both `collection_names` and `extra_typedef_names`. But wait, the issue might be that `TDF_LabelMap` is NOT being collected by the parser's `collect_ncollection_typedefs`.

Let me check what's happening — let me run the binding generator with some debug output to see if `TDF_LabelMap` is in the `extra_typedef_names`:   Let me add a quick debug print to see if `TDF_LabelMap` is in `extra_typedef_names`:    

Ran terminal command:  grep -n "TDF_LabelMap\|TDF_AttributeMap" crates/opencascade-binding-generator/src/codegen/collections.rs
Completed with input:  grep -n "TDF_LabelMap\|TDF_AttributeMap" crates/opencascade-binding-generator/src/codegen/collections.rs

Not in the curated known collections. Let me check the generated ffi.rs to see how `TDF_LabelMap` appears there:    

Searched for text `TDF_LabelMap` (`**/crates/opencascade-sys/generated/ffi.rs`), 1 result

Reading lines 18685-18700 of [ffi.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi.rs#L18685-L18701)
Completed with input: {
  "endLine": 18700,
  "filePath": "crates/opencascade-sys/generated/ffi.rs",
  "startLine": 18685
}

So `TDF_LabelMap` exists as an opaque struct in ffi.rs and is re-exported in tdf.rs. The issue is in how it's treated during method bindability checking.

Let me check how these "Referenced type from C++" structs are generated - they must be generated for types that appear in method signatures but aren't parsed classes:    

Searched for regex `Referenced type from C\+\+` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), 1 result

Reading lines 490-540 of [rust.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/rust.rs#L490-L541)
Completed with input: {
  "endLine": 540,
  "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
  "startLine": 490
}

So opaque "Referenced type from C++" structs are generated for types that appear in method signatures. This means `TDF_LabelMap` IS declared in the FFI. But the method bindability checks still consider it "unknown". 

The key issue: The method filtering at `type_uses_unknown_type` checks `all_class_names`, which includes parsed classes + collection types + extra typedef names. But does it actually include `TDF_LabelMap`?

Let me verify by checking what the TYPEDEF_MAP actually collects. The parser collects typedefs by walking the AST. `TDF_LabelMap` is a typedef of `NCollection_Map<TDF_Label, ...>`. Let me verify whether `TDF_LabelMap` appears in a header that gets parsed:     

Ran terminal command:  grep -r "TDF_LabelMap" crates/occt-sys/OCCT/src/TDF/*.hxx | head -10

Ran terminal command:  cat crates/occt-sys/OCCT/src/TDF/TDF_LabelMap.hxx

Good, so `TDF_LabelMap` is a typedef of `NCollection_Map<TDF_Label>`. The parser's `collect_ncollection_typedefs` should pick this up. But `TDF_LabelMap.hxx` needs to be included transitively during parsing.

Let me check what the bindings.toml says about which headers are parsed, specifically whether TDF module headers are included:     

Searched for text `TDF` (`**/crates/opencascade-binding-generator/bindings.toml`), no results

Searched for files matching `**/crates/opencascade-binding-generator/bindings.toml`, no matches

Searched for files matching `**/bindings.toml`, 1 match

Searched for text `TDF` (`**/crates/opencascade-sys/bindings.toml`), 7 results

Good, so `TDF` module is included. The `TDF_LabelMap.hxx` is included transitively by `TDF_DataSet.hxx` etc. So when libclang parses these headers, it should see the typedef.

The real question is: does `collect_ncollection_typedefs` actually pick up `TDF_LabelMap`? Let me add a temporary debug print and run the generator to check. Actually, let me first just run the generator and search the output for TDF_LabelMap:       

Ran terminal command:  time cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml 2>&1 | grep -i "TDF_LabelMap\|TDF_AttributeMap\|Collected.*NCollection"
Completed with input:  time cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml 2>&1 | grep -i "TDF_LabelMap\|TDF_AttributeMap\|Collected.*NCollection"

Ran terminal command:  cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml 2>&1 | head -20

Ran terminal command:  cat scripts/regenerate-bindings.sh | grep -i "DYLD\|clang\|LIBCLANG\|export"

Ran terminal command:  source <(grep 'DYLD_LIBRARY_PATH\|XCODE_TOOLCHAIN' scripts/regenerate-bindings.sh | head -5) 2>/dev/null; cat scripts/regenerate-bindings.sh

Rather than running the full generator, let me trace the logic more carefully. The `extra_typedef_names` comes from `parser::get_collected_typedef_names()`, which returns all NCollection typedefs collected during parsing. But `collect_ncollection_typedefs` only runs during header parsing (line 212 in parser.rs). The question is: when does `get_collected_typedef_names()` get called relative to parsing?

Let me check the order in the `generate_output` function:      

Reading lines 560-600 of [main.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/main.rs#L560-L601)
Completed with input: {
  "endLine": 600,
  "filePath": "crates/opencascade-binding-generator/src/main.rs",
  "startLine": 560
}

Reading lines 690-720 of [main.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/main.rs#L690-L721)
Completed with input: {
  "endLine": 720,
  "filePath": "crates/opencascade-binding-generator/src/main.rs",
  "startLine": 690
}

Let me look at how headers are parsed - whether they're batched into separate translation units:       

Searched for regex `pub fn parse_headers|fn parse_batch|translation_unit|collect_ncollection_typedefs` (`**/crates/opencascade-binding-generator/src/parser.rs`), 4 results

Reading lines 107-230 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L107-L231)
Completed with input: {
  "endLine": 230,
  "filePath": "crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 107
}

Let me add a quick debug check to confirm whether `TDF_LabelMap` is in the extra typedef names. Let me write a small debug script instead of modifying the source:     Let me add a temporary debug print to verify `TDF_LabelMap` presence: 

Edited: [main.rs +1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/main.rs#L587-L588)
Completed with input: {
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "586:mc",
      "insertAfter": true,
      "content": "    if extra_typedef_names.contains(\"TDF_LabelMap\") { eprintln!(\"DEBUG: TDF_LabelMap IS in extra_typedef_names\"); } else { eprintln!(\"DEBUG: TDF_LabelMap NOT in extra_typedef_names\"); }"
    }
  ]
}

Ran terminal command:  XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o tmp/debug-output 2>&1 | grep -E "DEBUG|TDF_LabelMap|extra_typedef"
Completed with input:  XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o tmp/debug-output 2>&1 | grep -E "DEBUG|TDF_LabelMap|extra_typedef"

Ran terminal command:  mkdir -p tmp/debug-output && XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o tmp/debug-output 2>&1 | tee tmp/debug-gen.log | grep -c "DEBUG"
Completed with input:  mkdir -p tmp/debug-output && XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o tmp/debug-output 2>&1 | tee tmp/debug-gen.log | grep -c "DEBUG"

The debug line never printed! That means `generate_output` was never called. Let me check what happened: 

Ran terminal command:  wc -l tmp/debug-gen.log && grep -E "error|Error|Generating|Building|Compiling" tmp/debug-gen.log

Ran terminal command:  cat tmp/debug-gen.log

The generator crashed before reaching my debug print. Let me move it to right after typedef collection in the parser: 

Reading lines 586-590 of [main.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/main.rs#L586-L591)
Completed with input: {
  "endLine": 590,
  "filePath": "crates/opencascade-binding-generator/src/main.rs",
  "startLine": 586
}

**Failing: [main.rs -1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/main.rs#L586-L587)**
Completed with input: {
  "edits": [
    {
      "content": "    let extra_typedef_names = parser::get_collected_typedef_names();",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "586:mc\n587:ce"
    }
  ]
}

Reading lines 586-590 of [main.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/main.rs#L586-L591)
Completed with input: {
  "endLine": 590,
  "filePath": "crates/opencascade-binding-generator/src/main.rs",
  "startLine": 586
}

Edited: [main.rs -1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/main.rs#L587-L588)
Completed with input: {
  "edits": [
    {
      "content": "    let extra_typedef_names = parser::get_collected_typedef_names();",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "587:ce"
    }
  ]
}

Now let me add the debug print in the parser right after collection:  

Reading lines 74-82 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L74-L83)
Completed with input: {
  "endLine": 82,
  "filePath": "crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 74
}

Edited: [parser.rs -1+2](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L76-L78)
Completed with input: {
  "edits": [
    {
      "content": "    eprintln!(\"  Collected {} NCollection/template typedefs\", map.len());\n    if map.values().any(|v| v == \"TDF_LabelMap\") { eprintln!(\"  DEBUG: TDF_LabelMap IS in typedef map\"); } else { eprintln!(\"  DEBUG: TDF_LabelMap NOT in typedef map\"); }",
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "76:gs"
    }
  ]
}

Ran terminal command:  XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o tmp/debug-output 2>&1 | grep -E "DEBUG|Collected"

`TDF_LabelMap` is NOT being collected by `collect_ncollection_typedefs`. Let me investigate why. The typedef is:
```cpp
typedef NCollection_Map<TDF_Label> TDF_LabelMap;
```

The collection function checks if `display.contains('<')` and if the name starts with uppercase and contains underscore. Let me check what clang reports for this typedef:     

Reading lines 47-80 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L47-L81)
Completed with input: {
  "endLine": 80,
  "filePath": "crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 47
}

Let me add more debug to check what clang reports for TDF_LabelMap specifically:   

Edited: [parser.rs -1+10](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L54-L64)
Completed with input: {
  "edits": [
    {
      "content": "            if let Some(name) = entity.get_name() {\n                if name == \"TDF_LabelMap\" || name == \"TDF_AttributeMap\" {\n                    eprintln!(\"  DEBUG typedef '{}': kind={:?}\", name, entity.get_kind());\n                    if let Some(underlying) = entity.get_typedef_underlying_type() {\n                        eprintln!(\"    underlying display: '{}'\", underlying.get_display_name());\n                        eprintln!(\"    contains '<': {}\", underlying.get_display_name().contains('<'));\n                    } else {\n                        eprintln!(\"    NO underlying type!\");\n                    }\n                }",
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "54:tn"
    }
  ]
}

Ran terminal command:  XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o tmp/debug-output 2>&1 | grep -E "DEBUG|TDF_LabelMap|TDF_AttributeMap"

Reading lines 50-90 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L50-L91)
Completed with input: {
  "endLine": 90,
  "filePath": "crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 50
}

The debug print for "underlying display" isn't appearing. That means `entity.get_typedef_underlying_type()` is returning `None` for `TDF_LabelMap`. This is the bug! Clang's Rust wrapper isn't returning the underlying type for this typedef.

But wait — this could be because the typedef occurs in a header file that isn't explicitly included, so clang might have a different representation. Let me add more detailed debugging:     

**Failing: [parser.rs -1+21](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L55-L76)**
Completed with input: {
  "edits": [
    {
      "content": "                if name == \"TDF_LabelMap\" || name == \"TDF_AttributeMap\" {\n                    eprintln!(\"  DEBUG typedef '{}': kind={:?}\", name, entity.get_kind());\n                    if let Some(underlying) = entity.get_typedef_underlying_type() {\n                        eprintln!(\"    underlying display: '{}'\", underlying.get_display_name());\n                        eprintln!(\"    contains '<': {}\", underlying.get_display_name().contains('<'));\n                    } else {\n                        eprintln!(\"    NO underlying type from get_typedef_underlying_type!\");\n                    }\n                    // Also try getting the type via get_type()\n                    if let Some(ty) = entity.get_type() {\n                        eprintln!(\"    entity.get_type() display: '{}'\", ty.get_display_name());\n                        if let Some(canon) = ty.get_canonical_type() {\n                            eprintln!(\"    canonical type display: '{}'\", canon.get_display_name());\n                        }\n                    }\n                    // Check location\n                    if let Some(loc) = entity.get_location() {\n                        let file_loc = loc.get_file_location();\n                        eprintln!(\"    location: {:?}:{}\", file_loc.file.map(|f| f.get_path()), file_loc.line);\n                    }\n                }",
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "55:lo\n56:ik\n57:oa\n58:mp\n59:ez\n60:ip\n61:pa\n62:or\n63:gl"
    }
  ]
}

Replacing 9 lines with 19 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs)


```

```


Ran terminal command:  XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o tmp/debug-output 2>&1 | grep -E "DEBUG|TDF_LabelMap|TDF_AttributeMap"
Completed with input:  XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o tmp/debug-output 2>&1 | grep -E "DEBUG|TDF_LabelMap|TDF_AttributeMap"

Ran terminal command:  XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o tmp/debug-output 2>&1 | grep "Collected"
Completed with input:  XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o tmp/debug-output 2>&1 | grep "Collected"

Ran terminal command:  XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o tmp/debug-output 2>&1 | tail -20

Compile error. `get_canonical_type()` doesn't return an `Option`. Let me fix:   

Replacing 6 lines with 5 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs)


```

```


Ran terminal command:  XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o tmp/debug-output 2>&1 | grep -E "DEBUG|TDF_Label"

Now I understand the bug! `get_typedef_underlying_type()` returns `None` for some NCollection typedefs like `TDF_LabelMap`, so they never get added to the typedef map. But the types are still used to generate opaque structs and re-exports. The fix is to use `entity.get_type().get_canonical_type()` as a fallback.

Let me first remove the debug code and implement the actual fix: 

Starting: *Fix the binding generator* (2/4)

Reading lines 47-95 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L47-L96)
Completed with input: {
  "endLine": 95,
  "filePath": "crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 47
}

Now let me replace the whole function with the fix (remove debug code, add fallback):  

Replacing 49 lines with 44 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs)


```

```
Now let me verify it compiles and test if it picks up TDF_LabelMap:    

Ran terminal command:  cargo build -p opencascade-binding-generator 2>&1

Good, it compiles. Now let me test if TDF_LabelMap gets collected:     

Ran terminal command:  XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o tmp/debug-output 2>&1 | grep "Collected"

Still 1347. The fallback isn't working. Let me add targeted debug to see what `entity.get_type()` and canonical type look like:     

Reading lines 47-95 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L47-L96)
Completed with input: {
  "endLine": 95,
  "filePath": "crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 47
}

Replacing 14 lines with 21 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs)


```

```


Ran terminal command:  XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o tmp/debug-output 2>&1 | grep -E "DEBUG|Collected"

Now I see the real problem! `get_typedef_underlying_type()` returns `"int"` for `TDF_LabelMap` — clang is misresolving the template. The `<` check correctly filters it, but the fallback canonical type is also `"int"`.

Let me look at how referenced types are collected — the opaque struct IS generated, so something collects it:       

Searched for regex `fn collect_referenced_types` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), 1 result

Reading lines 27-110 of [rust.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/rust.rs#L27-L111)
Completed with input: {
  "endLine": 110,
  "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
  "startLine": 27
}

Searched for text `math_Vector` (`**/crates/opencascade-sys/generated/ffi.rs`), 1 result

The root cause: clang misresolves `NCollection_Map<TDF_Label>` as `"int"` for the typedef underlying type. The `<` check correctly rejects it. Let me try a different approach — walking the entity's children to find template references: 

Replacing 51 lines with 30 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs)


```

```
Good, I've reverted the debug changes. Now I need a different approach. Since clang misresolves the typedef underlying type, I should scan the actual header files for NCollection typedef patterns as a fallback. Let me implement this:       

Reading lines 88-100 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L88-L101)
Completed with input: {
  "endLine": 100,
  "filePath": "crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 88
}

Now let me look at what include directories are available when the parser runs, so I can scan header files:    

Reading lines 206-225 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L206-L226)
Completed with input: {
  "endLine": 225,
  "filePath": "crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 206
}

I have the include dirs available. Let me add a text-based fallback that scans headers for typedef patterns. But wait — I think there's a cleaner approach. Instead of scanning header files, I'll approach this at the binding computation level. 

The real insight: if a type has an opaque struct in ffi.rs AND it's re-exported in a module file, it IS a known type. The issue is just that the "known type set" used for method bindability doesn't include all types that get opaque structs.

Let me instead compute the set of all referenced types BEFORE computing class bindings, and add them to `all_class_names`:         

Reading lines 3075-3170 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L3075-L3171)
Completed with input: {
  "endLine": 3170,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 3075
}

The right fix is to scan header files for NCollection typedef patterns as a fallback when clang misresolves them. Let me implement a text-based fallback scanner:       

Reading lines 10-16 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L10-L17)
Completed with input: {
  "endLine": 16,
  "filePath": "crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 10
}

Reading lines 74-100 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L74-L101)
Completed with input: {
  "endLine": 100,
  "filePath": "crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 74
}

Now I'll add a function to scan header files for typedef patterns and call it from `parse_headers`:   

Replacing 7 lines with 63 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs)


```

```
Now I need to call `supplement_typedefs_from_headers` from `parse_headers`. Let me find where to add the call:    

Reading lines 255-270 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L255-L271)
Completed with input: {
  "endLine": 270,
  "filePath": "crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 255
}

Replacing 4 lines with 10 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs)


```

```
Now I need to add the `regex` dependency to the binding generator's Cargo.toml:     

Searched for text `regex` (`**/crates/opencascade-binding-generator/Cargo.toml`), 1 result

Good, `regex` is already a dependency. Let me build:   

Ran terminal command:  cargo build -p opencascade-binding-generator 2>&1

Good. Now let me test:     

Ran terminal command:  XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o tmp/debug-output 2>&1 | grep -E "Collected|Supplemented|NCollection"

356 additional typedefs found! Let me verify `TDF_LabelMap` is among them now. Let me check the generated output for TDF:      

Ran terminal command:  grep "TDF_LabelMap" tmp/debug-output/tdf.rs | head -5

Ran terminal command:  ls tmp/debug-output/*.rs 2>&1 | head -5

Ran terminal command:  ls tmp/debug-output/ 2>&1

The generator crashed before generating output files (the same clang crash from before). But we can verify the fix works by checking if the crash happened AFTER the typedef collection. Let me check the full output:     

Ran terminal command:  XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o tmp/debug-output 2>&1 | tail -20

The crash is a known clang crate issue (pre-existing). The regeneration script works because it builds in release mode (`--release`). Let me use the full regeneration script:      

Starting: *Regenerate and verify build* (3/4)

Ran terminal command:  time scripts/regenerate-bindings.sh 2>&1

Ran terminal command:  grep -E "Collected|Supplemented" crates/opencascade-sys/generated/*.rs 2>/dev/null; grep "Collected\|Supplemented" <(scripts/regenerate-bindings.sh 2>&1) 2>/dev/null || echo "checking last output"; grep "TDF_LabelMap.*is unknown\|TDF_LabelMap.*unknown type" crates/opencascade-sys/generated/tdf.rs | head -5

Ran terminal command:  grep "TDF_LabelMap" crates/opencascade-sys/generated/tdf.rs

Ran terminal command:  grep "TDF_LabelMap" /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/tdf.rs

The fix works. Previously these methods were skipped:
- `TDF_ClosureTool::Closure` (param `aLabMap` uses unknown type `TDF_LabelMap&`)
- `TDF_DataSet::Labels` (return type `TDF_LabelMap&` is unknown)
- `TDF_RelocationTable::TargetLabelMap` (param `aLabelMap` uses unknown type `TDF_LabelMap&`)

Now they're all bound! Let me check for `TDF_AttributeMap` too:   

Ran terminal command:  grep "TDF_AttributeMap" /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/tdf.rs

Both `TDF_LabelMap` and `TDF_AttributeMap` methods are now bound. Let me verify the full build compiles:    

Ran terminal command:  time cargo build 2>&1
Completed with input:  time cargo build 2>&1

There's a method renaming issue — with more methods now bindable, some overloaded methods get suffixed. Let me check:      

Searched for regex `fn normal` (`**/crates/opencascade-sys/generated/poly.rs`), 4 results

The method was renamed from `normal` to `normal_int` because now there's also `normal_int_vec3f` (which was previously unblocked by adding `gp_Vec3f` as a known type, or by the additional NCollection typedefs). Let me check what the names were before:     

Ran terminal command:  git diff crates/opencascade-sys/generated/poly.rs | grep "normal" | head -10

The method was renamed due to a new overload now being bindable. Let me fix the call site and check for other breaking changes:      

Reading lines 90-105 of [mesh.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade/src/mesh.rs#L90-L106)
Completed with input: {
  "filePath": "crates/opencascade/src/mesh.rs",
  "startLine": 90,
  "endLine": 105
}

Edited: [mesh.rs -1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade/src/mesh.rs#L98-L99)
Completed with input: {
  "edits": [
    {
      "content": "                let normal = triangulation.normal_int(i);",
      "filePath": "crates/opencascade/src/mesh.rs",
      "lineHashes": "98:zu"
    }
  ]
}

Now let me check if there are other breaking changes:       

Ran terminal command:  time cargo build 2>&1

Build succeeds. Now let me run the binding generator tests:       

Ran terminal command:  time scripts/run-binding-generator-tests.sh 2>&1

All 16 tests pass. Now let me run the rest of the workspace tests:         

Ran terminal command:  time cargo test --workspace --exclude opencascade-binding-generator 2>&1

Ran terminal command:  cargo test --workspace --exclude opencascade-binding-generator 2>&1 | tail -20

All tests pass. Let me now check how many symbols were unblocked with this fix:       

Ran terminal command:  git diff --stat crates/opencascade-sys/generated/ | tail -5

Let me count the newly bound symbols (previously skipped):       

Ran terminal command:  diff <(git show HEAD:crates/opencascade-sys/generated/ffi.rs | grep "pub fn " | wc -l) <(grep "pub fn " crates/opencascade-sys/generated/ffi.rs | wc -l); echo "Before: $(git show HEAD:crates/opencascade-sys/generated/ffi.rs | grep "pub fn " | wc -l) ffi functions"; echo "After: $(grep "pub fn " crates/opencascade-sys/generated/ffi.rs | wc -l) ffi functions"

82 new FFI functions unblocked. Let me also check how many SKIPPED comments were removed:        

Ran terminal command:  echo "Before: $(git show HEAD:crates/opencascade-sys/generated/tdf.rs | grep -c "SKIPPED:") skipped in tdf.rs"; echo "After: $(grep -c "SKIPPED:" crates/opencascade-sys/generated/tdf.rs) skipped in tdf.rs"

15 methods unblocked in `tdf.rs` alone (from 33 to 18 skipped). Now let me update the README's skip count table:        

Ran terminal command:  echo "Total skipped before: $(git show HEAD:crates/opencascade-sys/generated/ | xargs -I{} echo {} | head -1)"; for f in crates/opencascade-sys/generated/*.rs; do grep -c "SKIPPED:" "$f" 2>/dev/null; done | awk '{s+=$1} END {print "Total skipped after:", s}'

Ran terminal command:  git stash list; git diff --name-only HEAD | head -20

Let me also clean up the unused variable warning in main.rs:      

Reading lines 583-590 of [main.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/main.rs#L583-L591)
Completed with input: {
  "endLine": 590,
  "filePath": "crates/opencascade-binding-generator/src/main.rs",
  "startLine": 583
}

There's a duplicate line. Let me fix that:     

Replacing 5 lines with 4 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/main.rs)


```

```


Ran terminal command:  cargo build -p opencascade-binding-generator 2>&1

Clean build, no warnings. Now let me also update the README.md since the skip counts changed:        

Find README sections to update

Searched for regex `TDF_LabelMap|TDF_AttributeMap` (`**/crates/opencascade-binding-generator/README.md`), 6 results

Reading lines 355-385 of [README.md](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md#L355-L386)
Completed with input: {
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md",
  "startLine": 355,
  "endLine": 385
}

Reading lines 355-378 of [README.md](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md#L355-L379)
Completed with input: {
  "filePath": "crates/opencascade-binding-generator/README.md",
  "startLine": 355,
  "endLine": 378
}

Now I need to update the README — the `TDF_LabelMap` entry should be removed from "Most Common Unknown Types" and the "Document Framework" paragraph should be updated to reflect that it's now fixed. Let me also get accurate new counts:         

Ran terminal command:  grep -r "SKIPPED:" crates/opencascade-sys/generated/tdf.rs crates/opencascade-sys/generated/t_doc_std.rs crates/opencascade-sys/generated/xcaf_doc.rs | wc -l

Ran terminal command:  echo "TDF_LabelMap skips remaining:"; grep -r "TDF_LabelMap" crates/opencascade-sys/generated/*.rs | grep "SKIPPED\|unknown" | wc -l; echo "TDF_AttributeMap skips remaining:"; grep -r "TDF_AttributeMap" crates/opencascade-sys/generated/*.rs | grep "SKIPPED\|unknown" | wc -l

Zero remaining `TDF_LabelMap` and `TDF_AttributeMap` skips. Now let me update the README:       

Edited: [README.md -1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md#L363-L364), [-1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/README.md#L376-L377)
Completed with input: {
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "363:td",
      "content": "| 27 | `TDF_LabelMap` | ~~Fixed~~ — now detected via header text scan fallback |"
    },
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "376:wd",
      "content": "**Document Framework (57 symbols)** — `TDocStd_*`, `TDF_*`, and `XCAFDoc_*` classes previously had 215 skipped symbols due to `TDF_LabelMap` and `TDF_AttributeMap` being unknown types. These are now resolved via the header text scan fallback for NCollection typedefs that clang misresolves. Remaining skips are mostly stream types (`Standard_OStream`) and other unrelated issues."
    }
  ]
}

Now let me also update the top-level skip reason table since the counts will have changed:        

Ran terminal command:  grep -r "SKIPPED:" crates/opencascade-sys/generated/*.rs | grep -oP "Reason: [^\"]*" | sed 's/Reason: //' | sort | uniq -c | sort -rn | head -15

Ran terminal command:  grep -r "//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | sort | uniq -c | sort -rn | head -15

The README update is reasonable. The overall skip count and table are informational and don't need exact updating right now. Let me now commit the changes.     

Starting: *Commit the fix* (4/4)

Ran terminal command:  cd /Users/sethml/src/opencascade-rs && cat > tmp/commit-msg.txt << 'EOF'
Fix NCollection typedef detection for clang-misresolved types

clang sometimes misresolves NCollection template typedefs (e.g.,
`typedef NCollection_Map<TDF_Label> TDF_LabelMap`) reporting their
underlying type as "int" instead of the actual template instantiation.
This caused ~356 NCollection typedefs (including TDF_LabelMap and
TDF_AttributeMap) to be missing from the known-type set, resulting
in methods using these types being incorrectly skipped as "unknown".

Fix: add a header text scan fallback in the parser that searches
for `typedef NCollection_*<...> TypeName;` patterns in the raw
header files. This supplements the clang AST-based collection with
types that clang misresolves.

Results:
- 82 new FFI functions bound (63,951 → 64,033)
- TDF_LabelMap and TDF_AttributeMap fully resolved (0 remaining skips)
- 15 previously-skipped TDF methods now bindable
- Methods unblocked across 26 generated files

Also fixes Poly_Triangulation::normal → normal_int rename in
opencascade crate (caused by new overload becoming bindable).
EOF


Created [](file:///Users/sethml/src/opencascade-rs/tmp/commit-msg.txt)

Ran terminal command:  rm /Users/sethml/src/opencascade-rs/tmp/commit-msg.txt

Created [](file:///Users/sethml/src/opencascade-rs/tmp/commit-msg.txt)

Ran terminal command:  git add -A && git commit -F tmp/commit-msg.txt 2>&1

Completed: *Commit the fix* (4/4)

Done. Here's what I found and fixed:

**Root cause:** clang misresolves certain NCollection template typedefs — for `typedef NCollection_Map<TDF_Label> TDF_LabelMap`, clang reports the underlying type as `"int"` instead of `"NCollection_Map<TDF_Label, ...>"`. The binding generator's `collect_ncollection_typedefs` function uses a `display.contains('<')` check to identify template typedefs, which correctly rejects `"int"`, but that means `TDF_LabelMap` never enters the known-type set. Meanwhile, the type IS recognized as a "referenced type" and gets an opaque struct + re-export — hence the contradiction.

**Fix:** Added `supplement_typedefs_from_headers()` in parser.rs — a text-based fallback that scans raw header files for `typedef NCollection_*<...> TypeName;` patterns. This catches the 356 typedefs that clang misresolves.

**Impact:**
- 82 new FFI functions bound
- `TDF_LabelMap` and `TDF_AttributeMap` skips fully resolved (27 → 0)
- Methods unblocked across 26 generated files
- Fixed `Poly_Triangulation::normal` → `normal_int` rename in the opencascade crate

Made changes.