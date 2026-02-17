User: Fix this problem. Be careful that the fix will work on all platforms, including Linux/x64, Linux/arm64, and Windows. Then commit.

GitHub Copilot: 

  

Created 5 todos

Let me investigate how the generator handles types, particularly `Standard_Size` and the `long`/`long long` distinction. 

Investigate Standard_Size type mapping

Searched for regex `long|i64|u64|c_long|size_t|Standard_Size` (`**/crates/opencascade-binding-generator/src/**`), 100 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src)

Searched for text `Standard_Size` (`**/crates/opencascade-sys/**`), 1 result

Reading lines 340-420 of [model.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L340-L421)
Completed with input: {
  "filePath": "crates/opencascade-binding-generator/src/model.rs",
  "startLine": 340,
  "endLine": 420
}

Reading lines 1030-1170 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L1030-L1171)
Completed with input: {
  "filePath": "crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 1030,
  "endLine": 1170
}

Reading lines 1-100 of [type_mapping.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L1-L101)
Completed with input: {
  "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
  "startLine": 1,
  "endLine": 100
}

Reading lines 1340-1380 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L1340-L1381)
Completed with input: {
  "filePath": "crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 1340,
  "endLine": 1380
}

Reading lines 220-240 of [bindings.toml](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml#L220-L241)
Completed with input: {
  "filePath": "crates/opencascade-sys/bindings.toml",
  "startLine": 220,
  "endLine": 240
}

Reading lines 1280-1310 of [resolver.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#L1280-L1311)
Completed with input: {
  "endLine": 1310,
  "filePath": "crates/opencascade-binding-generator/src/resolver.rs",
  "startLine": 1280
}

Reading lines 530-600 of [model.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L530-L601)
Completed with input: {
  "endLine": 600,
  "filePath": "crates/opencascade-binding-generator/src/model.rs",
  "startLine": 530
}

Reading lines 75-105 of [cpp.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/cpp.rs#L75-L106)
Completed with input: {
  "endLine": 105,
  "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
  "startLine": 75
}

Ran terminal command:  find /opt/homebrew /usr/local -path "*/opencascade/Standard_TypeDef.hxx" 2>/dev/null | head -5

Ran terminal command:  find /opt/homebrew /usr/local -name "Standard_TypeDef.hxx" 2>/dev/null

Ran terminal command:  grep -n "Standard_Size\|typedef.*size_t\|typedef.*unsigned long" /opt/homebrew/Cellar/opencascade/7.9.3/include/opencascade/Standard_TypeDef.hxx

Reading lines 95-130 of [rust.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/rust.rs#L95-L131)
Completed with input: {
  "endLine": 130,
  "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
  "startLine": 95
}

Searched for regex `Usize|usize|size_t` (`**/crates/opencascade-binding-generator/src/**`), 48 results

Searched for regex `usize|size_t` (`**/crates/opencascade-sys/generated/**`), 30 results

Ran terminal command:  grep -n "Standard_Size\|size_t\|unsigned long" /opt/homebrew/Cellar/opencascade/7.9.3/include/opencascade/Standard_ReadBuffer.hxx

Reading lines 340-530 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L340-L531)
Completed with input: {
  "endLine": 530,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 340
}

Searched for regex `\bunsigned long\b|(?<!\w)long(?! long)\b` (`**/crates/opencascade-sys/generated/wrappers.cpp`), 4 results

Reading lines 410-460 of [type_mapping.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L410-L461)
Completed with input: {
  "endLine": 460,
  "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
  "startLine": 410
}

Searched for regex `Image_Texture.*long|file_offset|file_length` (`**/crates/opencascade-sys/generated/**`), 10 results

Searched for regex `Image_Texture` (`**/crates/opencascade-sys/generated/ffi.rs`), 10 results

Ran terminal command:  grep -n "long\|int64" /opt/homebrew/Cellar/opencascade/7.9.3/include/opencascade/Image_Texture.hxx

Ran terminal command:  grep -n "int64_t\|Standard_Integer\|Standard_Real\|Standard_Boolean" /opt/homebrew/Cellar/opencascade/7.9.3/include/opencascade/Standard_TypeDef.hxx

Searched for regex `c_long|c_ulong|CLong|CULong|std::ffi` (`**/crates/opencascade-binding-generator/**`), 11 results

Searched for regex `c_long|c_ulong|std::ffi` (`**/crates/opencascade-sys/**`), 10 results

Reading lines 1170-1250 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L1170-L1251)
Completed with input: {
  "endLine": 1250,
  "filePath": "crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 1170
}

Starting: *Fix generator for long/i64 mismatch* (2/5)

Let me read the key files I need to modify:   

Reading lines 340-560 of [model.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L340-L561)
Completed with input: {
  "filePath": "crates/opencascade-binding-generator/src/model.rs",
  "startLine": 340,
  "endLine": 560
}

Reading lines 1030-1180 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L1030-L1181)
Completed with input: {
  "filePath": "crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 1030,
  "endLine": 1180
}

Reading lines 1-100 of [type_mapping.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L1-L101)
Completed with input: {
  "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
  "startLine": 1,
  "endLine": 100
}

Read [](file:///Users/sethml/Library/Application%20Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/878a25dc-b188-43ce-83ef-10b20907144c/toolu_vrtx_01Tv4fYXJKKAgsSsjn6hHpdt__vscode-1771312778736/content.txt)

Read [](file:///Users/sethml/Library/Application%20Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/878a25dc-b188-43ce-83ef-10b20907144c/toolu_vrtx_01M6TnxAekGDetsb7WQTE1Wd__vscode-1771312778737/content.txt)

Now let me also check the resolver.rs for C++ type emission, and the parser.rs `map_standard_type` function:  

Reading lines 1330-1400 of [parser.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L1330-L1401)
Completed with input: {
  "endLine": 1400,
  "filePath": "crates/opencascade-binding-generator/src/parser.rs",
  "startLine": 1330
}

Searched for regex `Type::I64|Type::U64` (`**/crates/opencascade-binding-generator/src/**/*.rs`), 20 results

Now let me check more references and understand the resolver.rs C++ emission and also look at the rest of type_mapping.rs:   

Reading lines 1280-1310 of [resolver.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#L1280-L1311)
Completed with input: {
  "endLine": 1310,
  "filePath": "crates/opencascade-binding-generator/src/resolver.rs",
  "startLine": 1280
}

Reading lines 560-600 of [model.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L560-L601)
Completed with input: {
  "endLine": 600,
  "filePath": "crates/opencascade-binding-generator/src/model.rs",
  "startLine": 560
}

Searched for regex `long` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), 3 results

Let me also check the type_mapping.rs around line 430 for context, and also check for any test files that reference I64/U64:  

Reading lines 420-445 of [type_mapping.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L420-L446)
Completed with input: {
  "endLine": 445,
  "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
  "startLine": 420
}

Searched for regex `Type::I64|Type::U64` (`**/crates/opencascade-binding-generator/src/codegen/**/*.rs`), 8 results

Reading lines 340-560 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L340-L561)
Completed with input: {
  "endLine": 560,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 340
}

Reading lines 1310-1340 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1310-L1341)
Completed with input: {
  "endLine": 1340,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 1310
}

Read [](file:///Users/sethml/Library/Application%20Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/878a25dc-b188-43ce-83ef-10b20907144c/toolu_vrtx_01Suipm626p7vphERNdo7YjV__vscode-1771312778748/content.txt#1-1), lines 1 to 195

Now let me check for tests that reference these types: 

Searched for regex `Type::I64|Type::U64|\"i64\"|\"u64\"` (`**/crates/opencascade-binding-generator/**/*.rs`), 42 results

Let me also check the `type_to_ffi_return` function in bindings.rs around line 540-560:  

Reading lines 540-570 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L540-L571)
Completed with input: {
  "endLine": 570,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 540
}

Reading lines 100-115 of [rust.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/rust.rs#L100-L116)
Completed with input: {
  "endLine": 115,
  "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
  "startLine": 100
}

Now I have a complete picture. Let me implement the fix. I need to:

1. Add `Long` and `ULong` variants to `Type` enum
2. Map C++ `long` → `Type::Long`, `unsigned long` → `Type::ULong`, keep `long long` → `Type::I64`, `unsigned long long` → `Type::U64`
3. In Rust codegen: `Type::Long` → `std::ffi::c_long`, `Type::ULong` → `std::ffi::c_ulong`
4. In C++ codegen: `Type::Long` → `"long"`, `Type::I64` → `"long long"` (fixing current bug)
5. Also update `is_primitive_type` in rust.rs to include `c_long`/`c_ulong`

Let me also check the `to_rust_ffi_type_string` method fully: 

Reading lines 590-640 of [model.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L590-L641)
Completed with input: {
  "endLine": 640,
  "filePath": "crates/opencascade-binding-generator/src/model.rs",
  "startLine": 590
}

Now I have all the context. Let me implement all the changes using hashline_edit:Now let me apply all the edits. I'll start with model.rs (the Type enum and all match expressions), then parser.rs, then the codegen files: 

Edited: [model.rs -4+8](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L358-L366), [-2+4](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L396-L400), [-2+4](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L418-L422), [-2+4](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L553-L557), [-2+4](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#L608-L612)
Completed with input: {
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "358:qx,359:cu,360:hr,361:kl",
      "content": "    /// long long / int64_t\n    I64,\n    /// unsigned long long / uint64_t\n    U64,\n    /// long (platform-dependent: 32-bit on Windows LLP64, 64-bit on LP64)\n    Long,\n    /// unsigned long (platform-dependent size)\n    ULong,"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "392:hx,393:eg",
      "content": "            Type::I64 => \"longlong\".to_string(),\n            Type::U64 => \"ulonglong\".to_string(),\n            Type::Long => \"long\".to_string(),\n            Type::ULong => \"ulong\".to_string(),"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "412:it,413:ga",
      "content": "                | Type::I64\n                | Type::U64\n                | Type::Long\n                | Type::ULong"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "545:eo,546:ko",
      "content": "            Type::I64 => \"i64\".to_string(),\n            Type::U64 => \"u64\".to_string(),\n            Type::Long => \"std::ffi::c_long\".to_string(),\n            Type::ULong => \"std::ffi::c_ulong\".to_string(),"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "598:eo,599:ko",
      "content": "            Type::I64 => \"i64\".to_string(),\n            Type::U64 => \"u64\".to_string(),\n            Type::Long => \"std::ffi::c_long\".to_string(),\n            Type::ULong => \"std::ffi::c_ulong\".to_string(),"
    }
  ]
}

Now update parser.rs - both `parse_type()` and `map_standard_type()`:  

Edited: [parser.rs -4+4](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L1158-L1162), [-4+4](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#L1362-L1366)
Completed with input: {
  "edits": [
    {
      "content": "            \"long\" => return Type::Long,\n            \"unsigned long\" => return Type::ULong,\n            \"long long\" => return Type::I64,\n            \"unsigned long long\" => return Type::U64,",
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "1158:jb,1159:gb,1160:hg,1161:ee"
    },
    {
      "content": "        \"long\" => Some(Type::Long),\n        \"unsigned long\" => Some(Type::ULong),\n        \"long long\" => Some(Type::I64),\n        \"unsigned long long\" => Some(Type::U64),",
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "1362:mq,1363:jk,1364:ul,1365:hc"
    }
  ]
}

Now update resolver.rs: 

Edited: [resolver.rs -2+4](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#L1293-L1297)
Completed with input: {
  "edits": [
    {
      "content": "        Type::I64 => \"long long\".to_string(),\n        Type::U64 => \"unsigned long long\".to_string(),\n        Type::Long => \"long\".to_string(),\n        Type::ULong => \"unsigned long\".to_string(),",
      "filePath": "crates/opencascade-binding-generator/src/resolver.rs",
      "lineHashes": "1293:fa,1294:yb"
    }
  ]
}

Now update type_mapping.rs:  

Edited: [type_mapping.rs -12+24](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L50-L74), [-2+2](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#L445-L447)
Completed with input: {
  "edits": [
    {
      "content": "        Type::I64 => RustTypeMapping {\n            rust_type: \"i64\".to_string(),\n            needs_unique_ptr: false,\n            needs_pin: false,\n            source_module: None,\n        },\n        Type::U64 => RustTypeMapping {\n            rust_type: \"u64\".to_string(),\n            needs_unique_ptr: false,\n            needs_pin: false,\n            source_module: None,\n        },\n        Type::Long => RustTypeMapping {\n            rust_type: \"std::ffi::c_long\".to_string(),\n            needs_unique_ptr: false,\n            needs_pin: false,\n            source_module: None,\n        },\n        Type::ULong => RustTypeMapping {\n            rust_type: \"std::ffi::c_ulong\".to_string(),\n            needs_unique_ptr: false,\n            needs_pin: false,\n            source_module: None,\n        },",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "50:uk,51:jn,52:sp,53:nu,54:ty,55:or,56:sa,57:hm,58:sp,59:nu,60:ty,61:or"
    },
    {
      "content": "        \"long\" => return map_type_to_rust(&Type::Long),\n        \"unsigned long\" => return map_type_to_rust(&Type::ULong),",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "433:uf,434:ha"
    }
  ]
}

Now update bindings.rs - there are multiple functions that match on Type:   

Edited: [bindings.rs -2+4](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L349-L353), [-2+4](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L513-L517), [-2+4](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L559-L563), [-1+1](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1327-L1328)
Completed with input: {
  "edits": [
    {
      "content": "        Type::I64 => \"i64\".to_string(),\n        Type::U64 => \"u64\".to_string(),\n        Type::Long => \"std::ffi::c_long\".to_string(),\n        Type::ULong => \"std::ffi::c_ulong\".to_string(),",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "349:wh,350:ex"
    },
    {
      "content": "        Type::I64 => \"long long\".to_string(),\n        Type::U64 => \"unsigned long long\".to_string(),\n        Type::Long => \"long\".to_string(),\n        Type::ULong => \"unsigned long\".to_string(),",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "511:fa,512:kz"
    },
    {
      "content": "        Type::I64 => \"i64\".to_string(),\n        Type::U64 => \"u64\".to_string(),\n        Type::Long => \"std::ffi::c_long\".to_string(),\n        Type::ULong => \"std::ffi::c_ulong\".to_string(),",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "555:wh,556:ex"
    },
    {
      "content": "        Type::I32 | Type::U32 | Type::I64 | Type::U64 | Type::Long | Type::ULong | Type::Usize => {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1321:rf"
    }
  ]
}

Now update `is_primitive_type` in rust.rs to include `c_long`/`c_ulong`:    

Edited: [rust.rs -1+2](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/rust.rs#L107-L109)
Completed with input: {
  "edits": [
    {
      "content": "        \"bool\" | \"i32\" | \"u32\" | \"i64\" | \"u64\" | \"f32\" | \"f64\" | \"char\" | \"c_char\" |\n        \"c_long\" | \"c_ulong\" |",
      "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "lineHashes": "107:uf"
    }
  ]
}

Now let me check if there are any remaining references I missed, plus check for compile errors: 

Checked [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs), [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs), [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs), [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs), [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs), [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/rust.rs), no problems found

Good, no IDE errors. Let me also make sure I haven't missed any match expressions on `Type`. The compiler will catch non-exhaustive matches, but let me verify:    

Searched for regex `match.*ty\b|match.*self` (`**/crates/opencascade-binding-generator/src/**/*.rs`), 50 results

Let me check the cpp.rs file and also the key `match ty` blocks in resolver.rs that might need updating:  

Reading lines 60-100 of [cpp.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/cpp.rs#L60-L101)
Completed with input: {
  "endLine": 100,
  "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
  "startLine": 60
}

Reading lines 75-100 of [rust.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/rust.rs#L75-L101)
Completed with input: {
  "endLine": 100,
  "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
  "startLine": 75
}

These match expressions use wildcard `_ => {}` so they'll handle the new variants fine. Let me now try to build the binding generator to see if there are any exhaustive match errors I missed: 

Ran terminal command:  time cargo build -p opencascade-binding-generator 2>&1

Build succeeds. Now let me update the todo and remove the `Standard_ReadBuffer.hxx` exclusion from bindings.toml:    

Starting: *Remove Standard_ReadBuffer exclusion* (3/5)

Reading lines 224-235 of [bindings.toml](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml#L224-L236)
Completed with input: {
  "endLine": 235,
  "filePath": "crates/opencascade-sys/bindings.toml",
  "startLine": 224
}

Edited: [bindings.toml -4](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml#L227)
Completed with input: {
  "edits": [
    {
      "content": "",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "227:fw,228:lr,229:tw,230:dd"
    }
  ]
}

Starting: *Regenerate bindings and test* (4/5)

Now let me run the binding generator tests first, then regenerate bindings:     

Ran terminal command:  time ./scripts/run-binding-generator-tests.sh 2>&1

All 16 tests pass. Now regenerate bindings:      

Ran terminal command:  time ./scripts/regenerate-bindings.sh 2>&1

Ran terminal command:  tail -30 /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/878a25dc-b188-43ce-83ef-10b20907144c/toolu_vrtx_01E7zyxh4cEvhq15qjBSBMZo__vscode-1771312778771/content.txt

Bindings regenerated successfully. Now let me build the whole project to make sure everything compiles:        

Ran terminal command:  time cargo build -p opencascade-sys 2>&1

Build succeeds. Let me also verify that `Standard_ReadBuffer` was actually generated (i.e., it's no longer excluded):        

Searched for text `Standard_ReadBuffer` (`**/crates/opencascade-sys/generated/**/*.rs`), 20 results

`Standard_ReadBuffer` is now successfully generated. Let me verify it uses the correct types - particularly `c_long` for the `long long` parameter (which should actually be `i64` for `int64_t`/`long long`):        

Searched for text `Standard_ReadBuffer` (`**/crates/opencascade-sys/generated/ffi.rs`), 17 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi.rs#15471-15471), lines 15471 to 15490

The `theDataLen` parameter is `i64` (which is correct for `int64_t`/`long long`) and `theChunkLen` is `usize` (which is correct for `size_t`). 

Let me also verify that `c_long` is being used somewhere in the generated code for actual `long` types:         

Searched for text `c_long` (`**/crates/opencascade-sys/generated/**/*.rs`), no results

No `c_long` in generated code - OCCT doesn't use bare `long` in its APIs (uses `Standard_Integer`=int, `int64_t`=long long, `Standard_Size`=size_t). The fix is still correct in distinguishing the types. Let me verify the C++ wrappers also use `long long` now:          

Searched for text `Standard_ReadBuffer` (`**/crates/opencascade-sys/generated/**/*.cpp`), 11 results

The C++ wrapper correctly uses `long long` (matching `int64_t`) for the `theDataLen` parameter, and `size_t` for `theChunkLen`. This matches what the original OCCT header declares and will work correctly on all platforms.

Now let me also do a quick sanity check on the full build including the higher-level crate:           

Ran terminal command:  time cargo build 2>&1
Completed with input:  time cargo build 2>&1

These errors are pre-existing in the `opencascade` crate (unrelated to my change - the STEP/IGES readers/writers are excluded in bindings.toml). Let me verify `opencascade-sys` builds cleanly, which is what my change affects:    

Ran terminal command:  git diff --stat HEAD 2>&1

Let me verify the pre-existing errors don't relate to my changes by checking if the `opencascade` crate built before:      

Ran terminal command:  git stash && cargo build -p opencascade 2>&1 | grep "error\[" | wc -l && git stash pop 2>&1

Same 12 errors pre-existing. My changes don't introduce any new issues. Now let me commit:      

Starting: *Commit the fix* (5/5)

Ran terminal command:  cd /Users/sethml/src/opencascade-rs && git add -A && git commit -m "Distinguish C long from long long in type mapping

Add Type::Long and Type::ULong variants to properly distinguish C 'long'
(platform-dependent size) from 'long long' (always 64-bit). Previously
both mapped to Type::I64/U64, causing ABI mismatches on arm64 where
'long' and 'long long' are distinct 64-bit types.

Type mappings:
- C 'long' -> Type::Long -> Rust std::ffi::c_long (platform-correct)
- C 'unsigned long' -> Type::ULong -> Rust std::ffi::c_ulong
- C 'long long'/int64_t -> Type::I64 -> Rust i64 / C++ 'long long'
- C 'unsigned long long'/uint64_t -> Type::U64 -> Rust u64

This also fixes Type::I64 C++ emission which inconsistently used 'long'
in some places and 'long long' in others - now always 'long long'.

Unblocks Standard_ReadBuffer.hxx which was excluded due to this mismatch." 2>&1

Completed: *Commit the fix* (5/5)

Done. The fix adds `Type::Long` and `Type::ULong` variants to properly distinguish C `long` (platform-dependent) from `long long` (always 64-bit):

| C type | Type variant | Rust type | C++ emission |
|---|---|---|---|
| `long` | `Type::Long` | `std::ffi::c_long` | `long` |
| `unsigned long` | `Type::ULong` | `std::ffi::c_ulong` | `unsigned long` |
| `long long` / `int64_t` | `Type::I64` | `i64` | `long long` |
| `unsigned long long` / `uint64_t` | `Type::U64` | `u64` | `unsigned long long` |

This works correctly on all platforms:
- **LP64** (Linux/macOS x64 & arm64): `long` = 64-bit, `long long` = 64-bit (distinct types)
- **LLP64** (Windows): `long` = 32-bit, `long long` = 64-bit
- **ILP32** (32-bit): `long` = 32-bit, `long long` = 64-bit

`Standard_ReadBuffer.hxx` is now unblocked and generates correct bindings.