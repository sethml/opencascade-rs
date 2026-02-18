User: Fix this bug.

GitHub Copilot: Let me investigate the bug by looking at the relevant files. 

Ran terminal command:  find /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src -name "ShapeProcess_ShapeContext.hxx" | head -5

Read [](file:///Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/ShapeProcess/ShapeProcess_ShapeContext.hxx#1-1), lines 1 to 100

Read [](file:///Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/ShapeProcess/ShapeProcess_ShapeContext.hxx#100-100), lines 100 to 153

Searched for text `val_enum_` (`**/crates/opencascade-binding-generator/**`), no results

Searching for text `val_enum_`

User: @agent Try Again

GitHub Copilot: Searched for text `val_enum_` (`**/crates/opencascade-binding-generator/src/**`), no results

Searched for text `val_enum_` (`**/crates/opencascade-binding-generator/**`), no results

The `val_enum_` identifier isn't in the generator source. Let me try to reproduce the error first to understand what's happening. 

Ran terminal command:  ls crates/opencascade-binding-generator/src/

Searched for text `ContinuityVal` (`**/crates/opencascade-binding-generator/src/**`), no results

Searched for text `GeomAbs_Shape` (`**/crates/opencascade-binding-generator/src/**`), no results

Searched for text `enum_val` (`**/crates/opencascade-binding-generator/**`), no results

Searched for regex `val_|_val` (`**/crates/opencascade-binding-generator/**`), 20 results

Let me look at the codegen directory in detail, focusing on what generates C++ code with "val_enum_": 

Ran terminal command:  ls /Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/

Searched for regex `val_enum_|Continuity|GeomAbs_Shape|enum.*param|by.value.*enum` (`**/crates/opencascade-binding-generator/src/codegen/**`), 20 results

Searched for regex `val_enum|MutRefEnum|enum_param|local.*enum|writeback` (`**/crates/opencascade-binding-generator/src/codegen/cpp.rs`), no results

Searched for regex `val_enum_|MutRefEnum|writeback|local.*var` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 17 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#3540-3540), lines 3540 to 3700

Now I can see the general pattern. Let me look at where `val_enum_` would appear in the generated code - it's likely a naming pattern. Let me look at the preamble generation: 

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#3750-3750), lines 3750 to 3800

Now I need to understand what `cpp_arg_expr` is set to for MutRefEnum params. Let me look at where it gets set: 

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#1165-1165), lines 1165 to 1250

Found it. On line 1192, when a `&mut` enum parameter is detected, `cpp_arg_expr` is set to `format!("{}_enum_", name)`. So if the parameter name is `val`, the local variable would be `val_enum_`.

Now let's look at the `ShapeProcess_ShapeContext` header more carefully. The `GetContinuity` method has `GeomAbs_Shape& val` as a parameter - that's the issue. `GeomAbs_Shape` is an enum and `val` is a `&mut` output param, so `cpp_arg_expr` becomes `val_enum_`. Let me check what the generated code looks like to understand the actual compile error: 

Ran terminal command:  ls /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ | grep -i shapeprocess | head -20

Ran terminal command:  ls /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ | head -20

Let me reproduce the bug by running the binding generator on just ShapeProcess_ShapeContext.hxx:  

Ran terminal command:  cargo build -p opencascade-binding-generator 2>&1 | tail -5

Good - already compiled. Let me run the binding generator on just that header: 

Ran terminal command:  cat scripts/regenerate-bindings.sh

Ran terminal command:  cat crates/opencascade-binding-generator/README.md | head -80

Now I understand the structure well. The issue is that when `GetContinuity(const Standard_CString param, GeomAbs_Shape& val)` is processed, the parameter `val` becomes a `&mut` enum output param, so `cpp_arg_expr` is `val_enum_`. 

Then in the generated C++ wrapper, it creates:
```cpp
auto val_enum_ = static_cast<GeomAbs_Shape>(val);
```
and then passes `val_enum_` to the method, which in unmodified C++ would work but might conflict with something.

Wait—the error message says `val_enum_ undeclared identifier`. Let me think about this more carefully. Let me look at what exactly the C++ wrapper would look like for `GetContinuity`:

The method signature is:
```cpp
Standard_Boolean GetContinuity(const Standard_CString param, GeomAbs_Shape& val) const;
```

The `param` is a `const Standard_CString` (which is `const char*`) - that's a string parameter.
The `val` is a `GeomAbs_Shape&` - that's a `&mut` enum output param.

So the generated wrapper should look like:
```cpp
extern "C" bool ShapeProcess_ShapeContext_GetContinuity(const ShapeProcess_ShapeContext* self_, const char* param, int32_t& val) {
    auto val_enum_ = static_cast<GeomAbs_Shape>(val);
    auto result_ = self_->GetContinuity(param, val_enum_);
    val = static_cast<int32_t>(val_enum_);
    return result_;
}
```

That looks right. But wait - could there be an issue with `cpp_arg_expr` for the `param` parameter? Let me look at the code that builds args_str to understand what `cpp_arg_expr` is for a string param:  

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#1250-1250), lines 1250 to 1350

This code path looks fine. Let me now try to actually generate the binding for ShapeProcess_ShapeContext.hxx temporarily to see the error. Let me look at what test outputs exist for the binding generator:  

Ran terminal command:  ls crates/opencascade-binding-generator/tests/ 2>/dev/null || echo "No tests dir"

Ran terminal command:  cat scripts/run-binding-generator-tests.sh

Ran terminal command:  ls crates/opencascade-binding-generator/src/

Let me try generating the binding for that header to reproduce the error. First let me make a temporary toml:   

Ran terminal command:  ls target/OCCT/include/ShapeProcess_ShapeContext.hxx 2>/dev/null && echo "exists"

Created [](file:///Users/sethml/src/opencascade-rs/tmp/test_shapeprocess.toml)

Ran terminal command:  mkdir -p tmp/test_gen && XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run --release -p opencascade-binding-generator -- --config tmp/test_shapeprocess.toml -I target/OCCT/include -o tmp/test_gen 2>&1

Ran terminal command:  grep -n "val_enum_\|GetContinuity\|ContinuityVal" /Users/sethml/src/opencascade-rs/tmp/test_gen/wrappers.cpp

Ran terminal command:  grep -n "val_enum_" /Users/sethml/src/opencascade-rs/tmp/test_gen/wrappers.cpp -B10 -A5

Found the bug. The `get_continuity` wrapper at line 127-129 uses `val_enum_` but never declared it. The preamble (which should create `auto val_enum_ = static_cast<GeomAbs_Shape>(val)`) is missing, and the postamble (which writes back) is also missing.

This is happening in the `MutRefEnumParam` wrapper for `get_continuity`. Let me understand why the preamble is missing. Looking at line 3593 in bindings.rs - let me check if the method `get_continuity` is classified as `MutRefEnumParam`: 

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#780-780), lines 780 to 870

I see the problem now. Look at line 816-823: `classify_wrapper_kind` checks for `CStringParam` BEFORE checking for `MutRefEnumParam`. The method `GetContinuity` takes both a `const Standard_CString param` (which is a C string) AND a `GeomAbs_Shape& val` (which is a `&mut` enum). 

Since `has_cstring_param` is true, it classifies as `WrapperKind::CStringParam`, not `WrapperKind::MutRefEnumParam`. Then the `CStringParam` code path doesn't handle the `val_enum_` local variable at all!

Let me look at the `CStringParam` wrapper generation: 

Searched for text `CStringParam wrapper methods` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#3201-3201), lines 3201 to 3340

Now I can see the bug clearly. The `CStringParam` code path (section 3) doesn't emit the preamble/postamble for `&mut enum` output params. It just uses `cpp_arg_expr` directly in `args_str`, but `cpp_arg_expr` for a `&mut enum` param is `val_enum_` (the local variable name), which is never declared in this code path.

The fix should be: when we have a method with BOTH a CString param AND a `&mut enum` param (or `&mut enum` takes priority), we either:

1. Prioritize `MutRefEnumParam` over `CStringParam` when both conditions are true, and handle the cstring within `MutRefEnumParam`; OR
2. Add the preamble/postamble to the `CStringParam` wrapper generation.

Option 2 is cleaner. Let me add preamble/postamble handling to the `CStringParam` section:  

Reading lines 3201-3310 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L3201-L3311)
Completed with input: {
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 3201,
  "endLine": 3310
}

Now I understand the structure. I need to add preamble/postamble handling for `&mut enum` params in the `CStringParam` section. The fix needs to:

1. After opening the `extern "C" {...` body, emit preamble (create local enum vars)
2. After the call, emit postamble (write back enum values to int32_t& output params)
3. Adjust the return type to `int32_t` for any enum return

Actually, looking more closely - the `classify_wrapper_kind` function should prefer `MutRefEnumParam` over `CStringParam` when a method has both. The `CStringParam` handling for C strings is straightforward (pass them through directly), while the `MutRefEnumParam` case needs special preamble/postamble. The simplest fix is to change the priority in `classify_wrapper_kind` so `MutRefEnumParam` takes precedence over `CStringParam`, and then ensure the `MutRefEnumParam` code handles C string params correctly (it should, because `cpp_arg_expr` for a `const char*` param is just the param name).

Let me verify how `CStringParam` affects the `cpp_arg_expr`, and whether the `MutRefEnumParam` CString params would be handled correctly: 

Searched for text `CString conversion` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Searched for text `is_c_string` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 2 results

Searched for regex `CStringParam.*build_param|build_param.*CString` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `WrapperKind::CStringParam.*=>|CStringParam =>|match.*CStringParam` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Let me understand whether CString params have a different `cpp_arg_expr` than their regular name. Looking at `build_param_binding` - `const Standard_CString` would be a `ConstRef(Class("char"))` in the type system, which would map through `type_to_cpp_extern_c_param` as `const char*`. The `cpp_arg_expr` would be set via `param_to_cpp_extern_c_arg`. Let me read that function:  

Searched for text `fn param_to_cpp_extern_c_arg` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#873-873), lines 873 to 910

For a `ConstRef(Class("char"))` type, `param_to_cpp_extern_c_arg` would return `*param_name` (dereference it). But wait, for C strings, it might be handled differently. Let me look at `build_param_binding` for the C string case more carefully. 

Actually, `const Standard_CString` may just be `const char*` (a `ConstPtr(Class("char"))`), not a `ConstRef`. Let me check:  

Searched for regex `c_string|CString|Standard_CString` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 20 results

Searched for text `is_c_string` (`**/crates/opencascade-binding-generator/src/model.rs`), 1 result

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#488-488), lines 488 to 510

So `is_c_string()` returns true for `ConstPtr(Class("char"))` (i.e., `const char*`). For such a param, `param_to_cpp_extern_c_arg` returns just `param_name` (it falls through to `_ => param_name.to_string()`). Good - so C string params pass through correctly in the `MutRefEnumParam` code path.

The root cause is clear: `classify_wrapper_kind` checks `CStringParam` before `MutRefEnumParam`. The correct fix is to check `MutRefEnumParam` first (it should take priority), OR add preamble/postamble handling to the `CStringParam` section.

The cleanest fix is to check `MutRefEnumParam` before `CStringParam` in `classify_wrapper_kind`, since the `MutRefEnumParam` code already handles enum return types and other wrapper concerns, and C string params work fine in that path (they're just passed through).

Let me look at the full relevant section around line 816: 

Reading lines 797-833 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L797-L834)
Completed with input: {
  "endLine": 833,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 797
}

Now I'll fix the priority: move `MutRefEnumParam` check before `CStringParam` (and also before `CStringReturn`), since `MutRefEnumParam` already handles enums in all param/return positions correctly: 

Edited: [bindings.rs -10+10](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L814-L824)
Completed with input: {
  "edits": [
    {
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "814:ww,815:ar,816:eh,817:pp,818:lh,819:go,820:ww,821:ae,822:rz,823:lc",
      "content": "    if returns_by_value {\n        WrapperKind::ByValueReturn\n    } else if has_mut_ref_enum_params(&method.params, all_enums) {\n        WrapperKind::MutRefEnumParam\n    } else if has_cstring_param {\n        WrapperKind::CStringParam\n    } else if returns_cstring {\n        WrapperKind::CStringReturn\n    } else if resolver::has_const_mut_return_mismatch(method) {\n        WrapperKind::ConstMutReturnFix"
    }
  ]
}

But wait - the `MutRefEnumParam` code path also needs to handle `const char*` params in its FFI declaration. Let me verify by checking how `build_param_binding` would handle a `const char*` (C string) param in the `MutRefEnumParam` context.

Looking at `build_param_binding`:
- For `ConstPtr(Class("char"))` (a C string), it falls through to the `effective_ty` path since it's not a `MutRef`, not an enum, and not nullable (probably). 
- `type_to_cpp_extern_c_param` for `ConstPtr(inner)` where inner is `Class("char")` → the match arm for `ConstPtr(inner) if matches!(inner, Type::Class(name) if name == "char")` returns `"const char*"` ... wait, let me re-read that function:

```rust
fn type_to_cpp_extern_c_param(ty: &Type) -> String {
    match ty {
        Type::ConstRef(inner) => format!("const {}*", type_to_cpp(inner)),
        Type::MutRef(inner) => format!("{}*", type_to_cpp(inner)),
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "const char*".to_string()
        }
        _ => type_to_cpp(ty),
    }
}
```

Yes, `ConstPtr(Class("char"))` returns `"const char*"` correctly.
And `param_to_cpp_extern_c_arg("param", ConstPtr(Class("char")))` falls through to `_ => param_name.to_string()` = `"param"`.

So in the `MutRefEnumParam` code path, `param` (a C string) would have `cpp_type = "const char*"` and `cpp_arg_expr = "param"` - which is correct.

And for `val` (a `MutRef(Class("GeomAbs_Shape"))`), `cpp_type = "int32_t&"` and `cpp_arg_expr = "val_enum_"` and `mut_ref_enum_cpp_name = Some("GeomAbs_Shape")`.

The generated code in `MutRefEnumParam` section would be:
```cpp
extern "C" Standard_Boolean ShapeProcess_ShapeContext_get_continuity(const ShapeProcess_ShapeContext* self_, const char* param, int32_t& val) {
    auto val_enum_ = static_cast<GeomAbs_Shape>(val);
    auto result_ = self_->GetContinuity(param, val_enum_);
    val = static_cast<int32_t>(val_enum_);
    return result_;
}
```

That's correct.

Now let me also check the static method equivalent. Let me look at `classify_static_wrapper_kind` (if it exists):   

Searched for text `fn classify_static` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `classify.*static|StaticMethod.*wrapper|cstring.*static` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Searched for regex `fn classify_wrapper_kind|static.*wrapper.*kind|wrapper.*kind.*static` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Good - there's only one `classify_wrapper_kind` for instance methods. Let me also check for static method classification in resolver.rs:   

Searched for regex `cstring.*static|static.*cstring|mut_ref_enum.*static` (`**/crates/opencascade-binding-generator/src/resolver.rs`), no results

Now let me also check if the same `CStringParam`-vs-`MutRefEnumParam` priority bug exists in the static method wrapper generation in bindings.rs:  

Searched for regex `static.*method.*wrapper|wrapper.*static|StaticWrapper|WrapperKind.*static` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#3733-3733), lines 3733 to 3840

Good - the static method wrapper *already* handles `has_mut_ref_enum` correctly. The static wrapper uses `has_mut_ref_enum` as an explicit check before other types. Only the instance method's `classify_wrapper_kind` had the priority bug.

Now let me build the generator and regenerate the test output to verify the fix:  

Ran terminal command:  cargo build --release -p opencascade-binding-generator 2>&1

Ran terminal command:  rm -rf tmp/test_gen && mkdir -p tmp/test_gen && XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run --release -p opencascade-binding-generator -- --config tmp/test_shapeprocess.toml -I target/OCCT/include -o tmp/test_gen 2>&1

Ran terminal command:  grep -n "val_enum_\|GetContinuity\|ContinuityVal" /Users/sethml/src/opencascade-rs/tmp/test_gen/wrappers.cpp -A5 -B3

The fix is correct. Now `get_continuity` generates proper preamble (line 137) and postamble (line 139) for the `&mut enum` parameter.

Now let's remove the exclusion from `bindings.toml` and run the full test suite to make sure everything passes:   

Reading lines 244-260 of [bindings.toml](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml#L244-L261)
Completed with input: {
  "endLine": 260,
  "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml",
  "startLine": 244
}

Edited: [bindings.toml -4+2](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml#L249-L251)
Completed with input: {
  "edits": [
    {
      "content": "\n    # Protected operator delete (TDF_LabelNode).",
      "filePath": "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml",
      "lineHashes": "249:hq,250:ay,251:dd,252:lw"
    }
  ]
}

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml#244-244), lines 244 to 265

Good. Now run the full binding regeneration and build: 

Created 4 todos

Ran terminal command:  time scripts/regenerate-bindings.sh 2>&1

Ran terminal command:  tail -20 "/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5ba2a68f-95ae-4c06-8770-168913b7677c/toolu_bdrk_01NscyaAvAPGUgRQ95ie5ex9__vscode-1771312782054/content.txt"

Bindings regenerated successfully. Now let's check that `ShapeProcess_ShapeContext` appears in the generated output:   

Ran terminal command:  grep -n "get_continuity\|val_enum_" /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/*.cpp 2>/dev/null | head -20

The fix is in the generated code. Now build:    

Starting: *Cargo build and tests* (4/4)

Ran terminal command:  time cargo build 2>&1

Build passes. Now run the tests:  

Ran terminal command:  time scripts/run-binding-generator-tests.sh 2>&1

Ran terminal command:  time cargo test --workspace --exclude opencascade-binding-generator 2>&1

Ran terminal command:  tail -20 "/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5ba2a68f-95ae-4c06-8770-168913b7677c/toolu_bdrk_015y89M53Znm7NsX6sswCKV2__vscode-1771312782060/content.txt"

All tests pass. Let me clean up the temp files:   

Ran terminal command:  rm -rf /Users/sethml/src/opencascade-rs/tmp/test_gen /Users/sethml/src/opencascade-rs/tmp/test_shapeprocess.toml

Completed: *Cargo build and tests* (4/4)

**Root cause:** In `classify_wrapper_kind` in bindings.rs, `CStringParam` was checked *before* `MutRefEnumParam`. `ShapeProcess_ShapeContext::GetContinuity` takes both a `const Standard_CString` param *and* a `GeomAbs_Shape&` (`&mut` enum output) param, so it was incorrectly classified as `CStringParam`. The `CStringParam` code path doesn't emit the preamble/postamble local variable pattern needed for `&mut enum` params, so the generated C++ used `val_enum_` without declaring it.

**Fix:** Moved the `MutRefEnumParam` check above `CStringParam` (and `CStringReturn`) in `classify_wrapper_kind`. The `MutRefEnumParam` code path handles C string params correctly since they pass through as `const char*` without special transformation. The exclusion of ShapeProcess_ShapeContext.hxx from bindings.toml was also removed.

Made changes.

User: Why is val of type i32 rather than the appropriate enum type here?

GitHub Copilot: Looking at `build_param_binding` in the binding generator — when a `MutRef(Class(enum_name))` is detected, the `rust_reexport_type` is hardcoded to `"&mut i32"`: 

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#1180-1180), lines 1180 to 1205

The root cause: `rust_reexport_type` is hardcoded to `"&mut i32"` with the comment explaining the C++ wrapper handles the `int32_t ↔ enum` conversion. The re-export layer exposes the raw FFI type directly without lifting it back to the Rust enum type.

Fixing it properly requires the Rust re-export wrapper to generate a local variable pattern — analogous to what the C++ wrapper does:
```rust
// Instead of passing val directly, we'd need:
let mut val_i32 = *val as i32;
ffi::ShapeProcess_ShapeContext_get_continuity(self, c_param.as_ptr(), &mut val_i32);
*val = geom_abs::Shape::try_from(val_i32).unwrap();
```

Let me check whether the re-export codegen supports this kind of pre/post-call pattern:  

Searched for regex `preamble|postamble|local.*var|mut_ref_enum|enum_rust_type` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), 2 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/rust.rs#600-600), lines 600 to 700

The Rust re-export codegen's arg-building loop has no case for `mut_ref_enum_cpp_name`. Since `enum_rust_type` is `None` and `rust_reexport_type` is `"&mut i32"`, the param just passes through as-is. The proper fix needs:

1. A new field (e.g. `mut_ref_enum_rust_type: Option<String>`) on `ParamBinding` to hold the Rust enum type
2. `build_param_binding` to set `rust_reexport_type = "&mut <EnumType>"` and populate the field 
3. The Rust re-export method emitter to generate a local `i32` variable, pass it to FFI, then write back via `try_from`

Let me look at the method body emitter to understand the full change needed:   

Searched for text `fn emit_method_wrapper` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), no results

Searched for regex `fn emit_|wrapper_method|reexport_method|WrapperMethod` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), 3 results

User: Is that consistent with other enum types? Is this a problem that's unique to enums passed by mut ref? Anyway, see if you can make it consistent with other enums, exposing the rust enum type in the public API.

GitHub Copilot: 

  

Created 6 todos

Yes — by-value and const-ref enum params get typed Rust enums, but `&mut` enum output params are stuck at `&mut i32`. Let me trace the full re-export codegen to understand what needs to change. 

Searched for regex `fn generate_method_body|fn emit_method|wrapper_methods|method_body|reexport.*method|method.*reexport` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), no results

Searched for regex `fn generate_module|impl.*block|wrapper_kind|method.*impl` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), 10 results

Reading lines 768-1000 of [rust.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/rust.rs#L768-L1001)
Completed with input: {
  "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
  "startLine": 768,
  "endLine": 1000
}

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/rust.rs#768-768), lines 768 to 1000

That's the collection codegen. I need the method wrapper generation from `emit_reexport_class`. Let me find it: 

Searched for text `fn emit_reexport_class` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#4183-4183), lines 4183 to 4350

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#4350-4350), lines 4350 to 4550

Now let me look at `convert_arg`, `cstr_prelude_params`, and `build_reexport_body` to understand the full conversion chain:  

Searched for regex `fn convert_arg\b|fn cstr_prelude|fn build_reexport_body` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#4110-4110), lines 4110 to 4183

Starting: *Add mut_ref_enum_rust_type to ParamBinding* (2/6)

Now I understand the full picture. Here's my plan:

1. **`ParamBinding`**: Add `mut_ref_enum_rust_type: Option<String>` field
2. **`build_param_binding`**: For `&mut enum` params, set `rust_reexport_type = "&mut <EnumType>"` and populate `mut_ref_enum_rust_type`
3. **`convert_arg`** / **`convert_arg_resolved`**: For `mut_ref_enum_rust_type`, generate a local temp variable name instead of passing the param directly
4. **`cstr_prelude_params`** / **`cstr_prelude_resolved`**: Add preamble for mut_ref_enum params (create local `i32`, convert from enum)
5. **Method body codegen**: Add postamble that writes back the enum value after the FFI call
6. **`ResolvedParamBinding`**: Needs the same field

Let me look at both `ParamBinding` and `ResolvedParamBinding`:   

Searched for text `struct ParamBinding` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#260-260), lines 260 to 340

Now I need to understand the approach. Currently:
- `cstr_prelude_params` runs BEFORE the `unsafe` block and produces `let c_xxx = CString::new(xxx).unwrap();` lines
- `convert_arg` produces the arg expressions used in the FFI call
- `build_reexport_body` wraps the FFI call in `unsafe { ... }`

For `&mut enum` params, the generated Rust re-export needs to be structured like:

```rust
pub fn get_continuity(&self, param: &str, val: &mut geom_abs::Shape) -> bool {
    let c_param = std::ffi::CString::new(param).unwrap();
    let mut val_i32_: i32 = (*val).into();
    let result_ = unsafe { crate::ffi::ShapeProcess_ShapeContext_get_continuity(self as *const Self, c_param.as_ptr(), &mut val_i32_) };
    *val = crate::geom_abs::Shape::try_from(val_i32_).unwrap();
    result_
}
```

This requires a more complex body generation than simply wrapping the call. The key changes:
1. Prelude: create `let mut val_i32_: i32 = (*val).into();`
2. Arg conversion: pass `&mut val_i32_` instead of `val`
3. Store result in `let result_ = unsafe { ... };`
4. Postamble: `*val = EnumType::try_from(val_i32_).unwrap();`
5. Return `result_`

The simplest approach is to detect `mut_ref_enum` params at the method body construction site and generate the full body specially. Let me now also check where `ResolvedParamBinding` gets constructed, since inherited methods need the same fix:    

Searched for text `ResolvedParamBinding {"` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `ResolvedParamBinding \{` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 3 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#2595-2595), lines 2595 to 2670

Now let me check what `p.ty` looks like for `&mut enum` params in the resolver:   

Searched for regex `mut_ref.*enum|enum.*mut_ref|MutRef.*enum_cpp` (`**/crates/opencascade-binding-generator/src/resolver.rs`), no results

Searched for regex `ResolvedParam|enum_cpp_name` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 17 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#1300-1300), lines 1300 to 1360

Now let me look at `extract_enum_name_from_type`:    

Searched for text `fn extract_enum_name_from_type` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 1 result

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#1365-1365), lines 1365 to 1395

Good. The resolver's `extract_enum_name_from_type` deliberately skips `MutRef` (line 1369 comment says "NOT MutRef (output params need special handling)"). So `MutRef(Class(enum))` falls through to `resolve_type`'s default path — it won't set `enum_cpp_name`. 

This means in the inherited method codegen (line 2643 area), a `&mut enum` param doesn't have `enum_cpp_name` set, so it just gets treated as a regular ref type. Let me check what that looks like: 

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#2620-2620), lines 2620 to 2665

So for inherited methods, a `&mut enum` param (where `enum_cpp_name` is `None` because `extract_enum_name_from_type` skips `MutRef`) just falls through and gets type `&mut EnumType` at the FFI level — but the C++ wrapper also handles this with the `MurRefEnumParam` pattern. Let me check — does the inherited method builder know about `MutRefEnumParam`?

Actually, for inherited methods, the `resolve_inhabited_method` code path in bindings.rs _also_ calls into the same C++ codegen that handles `MutRefEnumParam`. So the C++ wrappers are already correct. The issue is only in the Rust re-export layer. Let me understand the situation for inherited methods vs own methods:

For **own methods** (wrapper_methods): `build_param_binding` handles the `&mut enum` → `ParamBinding` with `mut_ref_enum_cpp_name`, `rust_reexport_type = "&mut i32"`, etc.

For **inherited methods**: The code above at line 2643 constructs `ResolvedParamBinding`. The `&mut enum` case doesn't have `enum_cpp_name` set (because the resolver skips MutRef), so `effective_ty` is `MutRef(Class("GeomAbs_Shape"))` and `rust_reexport_type` would be... let me check what `type_to_rust_string` returns for `MutRef(Class("GeomAbs_Shape"))`:   

Searched for text `fn type_to_rust_string` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#920-920), lines 920 to 990

So for inherited methods with `MutRef(Class("GeomAbs_Shape"))`, `type_to_rust_string` returns `&mut crate::geom_abs::Shape` — the correct Rust enum type! But the FFI function expects `&mut i32`. So for inherited methods, the re-export type is already right but the arg conversion is wrong (it passes the enum ref directly to an `&mut i32` parameter).

Wait, I need to check — what does the inherited method's C++ wrapper look like for a `&mut enum` param? Let me check if the `MutRefEnumParam` handling applies to inherited wrappers too.

Let me look at how inherited methods are built in the C++ generation:     

Searched for regex `inherited.*wrapper|InheritedMethod|inherited_methods` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 16 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#4011-4011), lines 4011 to 4090

I see — the inherited method C++ codegen (section 10) does NOT handle `MutRefEnumParam` at all! It doesn't generate preamble/postamble. So inherited methods with `&mut enum` params also have the same C++ bug where the local variable is used but never declared.

However, in practice it might not be hit because inherited methods with `&mut enum` params may not exist yet or may be filtered. Still, I should fix both the Rust re-export AND add the C++ preamble/postamble for inherited methods.

Let me now plan the full implementation. This is a multi-site change:

**Changes needed:**

1. **`ParamBinding` struct**: Add `mut_ref_enum_rust_type: Option<String>` 
2. **`ResolvedParamBinding` struct**: Add `mut_ref_enum_rust_type: Option<String>`
3. **`build_param_binding`**: For `&mut enum`, look up and set `rust_reexport_type` to `&mut EnumType` and `mut_ref_enum_rust_type`
4. **Inherited method's `ResolvedParamBinding` construction**: Detect `MutRef(Class(enum_name))` and set up properly
5. **`convert_arg` / `convert_arg_resolved`**: For `mut_ref_enum`, pass `&mut <name>_i32_` instead
6. **Prelude generation**: Add `let mut <name>_i32_: i32 = (*<name>).into();` for mut_ref_enum params
7. **Method body**: For methods with mut_ref_enum params, generate multi-line body with postamble
8. **Inherited method C++ codegen (section 10)**: Add preamble/postamble for `&mut enum` params
9. **Free function codegen in rust.rs**: Also handle mut_ref_enum there
10. **PORTING.md**: Update the limitation

Let me start implementing: 

Added 5 todos

Let me read all the sections I need to modify:   

Reading lines 268-300 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L268-L301)
Completed with input: {
  "endLine": 300,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 268
}

Reading lines 306-325 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L306-L326)
Completed with input: {
  "endLine": 325,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 306
}

Now I also need to check all places that construct `ParamBinding` and `ResolvedParamBinding` to make sure I add the new field everywhere. Let me search for all construction sites:  

Searched for regex `ParamBinding \{` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 9 results

Good — 5 `ParamBinding` constructions and 2 `ResolvedParamBinding` constructions. Let me read all of them to add the new field. Let me also check what the `FunctionBinding` params use:    

Searched for regex `struct FunctionBinding` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#341-341), lines 341 to 380

`FunctionBinding` uses `Vec<ParamBinding>` (same struct). Good.

Now let me read the full `build_param_binding` function and all 4 construction sites of `ParamBinding`:    

Reading lines 1177-1300 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1177-L1301)
Completed with input: {
  "endLine": 1300,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 1177
}

Now I have a clear picture. Let me implement all the changes. I'll work through the files methodically.

**Step 1: Add `mut_ref_enum_rust_type` to both structs and update `build_param_binding`** 

Edited: [bindings.rs -2+4](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L283-L287), [-4+6](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L319-L325)
Completed with input: {
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "283:ta,284:ar",
      "content": "    /// If this is a &mut enum output param, the C++ enum name for local var + writeback pattern\n    pub mut_ref_enum_cpp_name: Option<String>,\n    /// If this is a &mut enum output param, the qualified Rust enum type (e.g. \"crate::geom_abs::Shape\")\n    pub mut_ref_enum_rust_type: Option<String>,"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "317:dx,318:rk,319:ev,320:ru",
      "content": "    /// If this is a value enum param, the qualified Rust enum type\n    pub enum_rust_type: Option<String>,\n    /// If this is a &mut enum output param, the qualified Rust enum type\n    pub mut_ref_enum_rust_type: Option<String>,\n    /// If this is a nullable pointer param (T* = NULL or const T* = NULL)\n    pub is_nullable_ptr: bool,"
    }
  ]
}

Now update `build_param_binding` for the `&mut enum` case to look up the Rust enum type:     

Reading lines 1181-1205 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1181-L1206)
Completed with input: {
  "endLine": 1205,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 1181
}

Edited: [bindings.rs -20+27](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1185-L1212)
Completed with input: {
  "edits": [
    {
      "content": "    // Check for &mut enum output params — these need special local var + writeback handling\n    if let Type::MutRef(inner) = ty {\n        if let Type::Class(enum_name) = inner.as_ref() {\n            if ffi_ctx.all_enums.contains(enum_name) {\n                let enum_rust_type = ffi_ctx.enum_rust_types\n                    .and_then(|map| map.get(enum_name))\n                    .cloned();\n                let reexport_type = enum_rust_type.as_ref()\n                    .map(|t| format!(\"&mut {}\", t))\n                    .unwrap_or_else(|| \"&mut i32\".to_string());\n                return ParamBinding {\n                    cpp_name,\n                    rust_name,\n                    rust_ffi_type: \"&mut i32\".to_string(),\n                    rust_reexport_type: reexport_type,\n                    cpp_type: \"int32_t&\".to_string(),\n                    // The arg expression uses the local variable name (preamble creates it)\n                    cpp_arg_expr: format!(\"{}_enum_\", name),\n                    // No value enum conversion at Rust level\n                    enum_rust_type: None,\n                    mut_ref_enum_cpp_name: Some(enum_name.clone()),\n                    mut_ref_enum_rust_type: enum_rust_type,\n                    is_nullable_ptr: false,\n                };\n            }\n        }\n    }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1185:al,1186:yn,1187:ma,1188:rb,1189:ai,1190:kr,1191:nc,1192:yb,1193:oo,1194:wy,1195:yr,1196:uw,1197:cn,1198:af,1199:zz,1200:fm,1201:fj,1202:uo,1203:cy,1204:ax"
    }
  ]
}

Now I need to add `mut_ref_enum_rust_type: None` to all other `ParamBinding` and `ResolvedParamBinding` construction sites. Let me find them:     

Reading lines 1215-1230 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1215-L1231)
Completed with input: {
  "endLine": 1230,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 1215
}

Reading lines 1260-1275 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1260-L1276)
Completed with input: {
  "endLine": 1275,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 1260
}

Reading lines 1295-1310 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1295-L1311)
Completed with input: {
  "endLine": 1310,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 1295
}

Now let me read the two `ResolvedParamBinding` construction sites:      

Reading lines 2609-2630 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L2609-L2631)
Completed with input: {
  "endLine": 2630,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 2609
}

Reading lines 2630-2680 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L2630-L2681)
Completed with input: {
  "endLine": 2680,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 2630
}

Now I need to handle the inherited method case. The `&mut enum` param needs detection here too. The `p.ty.original` would be `MutRef(Class("GeomAbs_Shape"))` and `p.ty.enum_cpp_name` would be `None` (because the resolver skips MutRef). I need to add a check.

Let me now apply all the changes in a batch. First the 4 `ParamBinding` sites and 2 `ResolvedParamBinding` sites that need `mut_ref_enum_rust_type: None`, then the inherited method ResolvedParamBinding that needs the new `&mut enum` detection:      

Edited: [bindings.rs -3+4](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1227-L1231), [-3+4](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1271-L1275), [-3+4](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L1305-L1309), [-2+3](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L2630-L2633)
Completed with input: {
  "edits": [
    {
      "content": "            enum_rust_type,\n            mut_ref_enum_cpp_name: None,\n            mut_ref_enum_rust_type: None,\n            is_nullable_ptr: false,",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1227:mw,1228:lp,1229:xt"
    },
    {
      "content": "            enum_rust_type: None,\n            mut_ref_enum_cpp_name: None,\n            mut_ref_enum_rust_type: None,\n            is_nullable_ptr: true,",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1270:mr,1271:lp,1272:cm"
    },
    {
      "content": "        enum_rust_type: None,\n        mut_ref_enum_cpp_name: None,\n        mut_ref_enum_rust_type: None,\n        is_nullable_ptr: false,",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1303:qp,1304:bs,1305:tw"
    },
    {
      "content": "                                enum_rust_type: None,\n                                mut_ref_enum_rust_type: None,\n                                is_nullable_ptr: true,",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2627:cn,2628:ig"
    }
  ]
}

Now the main ResolvedParamBinding construction at line ~2654. I need to add `&mut enum` detection here. Let me re-read it:       

Reading lines 2634-2680 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L2634-L2681)
Completed with input: {
  "endLine": 2680,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 2634
}

I need to add a check for `MutRef(Class(enum_name))` BEFORE the `effective_ty` match, similar to what `build_param_binding` does. Let me insert it:       

Edited: [bindings.rs -2+25](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L2635-L2660)
Completed with input: {
  "edits": [
    {
      "content": "\n                        // Check for &mut enum output params — same as build_param_binding\n                        if let Type::MutRef(inner) = &p.ty.original {\n                            if let Type::Class(enum_name) = inner.as_ref() {\n                                if all_enum_names.contains(enum_name) {\n                                    let enum_rust_type = symbol_table.enum_rust_types.get(enum_name).cloned();\n                                    let reexport_type = enum_rust_type.as_ref()\n                                        .map(|t| format!(\"&mut {}\", t))\n                                        .unwrap_or_else(|| \"&mut i32\".to_string());\n                                    return ResolvedParamBinding {\n                                        name: p.name.clone(),\n                                        rust_name: p.rust_name.clone(),\n                                        rust_ffi_type: \"&mut i32\".to_string(),\n                                        rust_reexport_type: reexport_type,\n                                        cpp_type: \"int32_t&\".to_string(),\n                                        cpp_arg_expr: format!(\"{}_enum_\", p.name),\n                                        enum_rust_type: None,\n                                        mut_ref_enum_rust_type: enum_rust_type,\n                                        is_nullable_ptr: false,\n                                    };\n                                }\n                            }\n                        }\n\n                        // Convert by-value class/handle params to const ref (same as build_param_binding)",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2635:dd,2636:gh"
    }
  ]
}

Now add `mut_ref_enum_rust_type: None` to the remaining `ResolvedParamBinding` construction:        

Reading lines 2680-2700 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L2680-L2701)
Completed with input: {
  "endLine": 2700,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 2680
}

Edited: [bindings.rs -2+3](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L2692-L2695)
Completed with input: {
  "edits": [
    {
      "content": "                            enum_rust_type: p.ty.enum_cpp_name.as_ref().and_then(|n| symbol_table.enum_rust_types.get(n)).cloned(),\n                            mut_ref_enum_rust_type: None,\n                            is_nullable_ptr: false,",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2692:zu,2693:bp"
    }
  ]
}

Starting: *Update convert_arg functions* (5/11)

Now I need to update the Rust re-export codegen. The key insight: for methods with `&mut enum` params, we can't use the simple `build_reexport_body` pattern. We need a multi-statement body with:
1. Prelude: `let mut <name>_i32_: i32 = (*<name>).into();`
2. Call: `let result_ = unsafe { ffi::foo(self_, ..., &mut <name>_i32_) };`
3. Postamble: `*<name> = EnumType::try_from(<name>_i32_).unwrap();`
4. Return: `result_`

Let me update `convert_arg` and `convert_arg_resolved` first, then the prelude functions, and then the body generation sites.

For `convert_arg` and `convert_arg_resolved`, when we see a `mut_ref_enum_rust_type`, the arg should be `&mut <name>_i32_`:       

Reading lines 4114-4155 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L4114-L4156)
Completed with input: {
  "endLine": 4155,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 4114
}

Reading lines 4151-4185 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L4151-L4186)
Completed with input: {
  "endLine": 4185,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 4151
}

Edited: [bindings.rs -31+35](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L4151-L4186)
Completed with input: {
  "edits": [
    {
      "content": "fn convert_arg(p: &ParamBinding) -> String {\n    if p.is_nullable_ptr {\n        if p.rust_ffi_type.starts_with(\"*const\") {\n            format!(\"{}.map_or(std::ptr::null(), |r| r as *const _)\", p.rust_name)\n        } else {\n            format!(\"{}.map_or(std::ptr::null_mut(), |r| r as *mut _)\", p.rust_name)\n        }\n    } else if p.mut_ref_enum_rust_type.is_some() {\n        format!(\"&mut {}_i32_\", p.rust_name)\n    } else if p.rust_reexport_type == \"&str\" {\n        format!(\"c_{}.as_ptr()\", p.rust_name)\n    } else if p.enum_rust_type.is_some() {\n        format!(\"{}.into()\", p.rust_name)\n    } else {\n        p.rust_name.clone()\n    }\n}\n\nfn convert_arg_resolved(name: &str, p: &ResolvedParamBinding) -> String {\n    if p.is_nullable_ptr {\n        if p.rust_ffi_type.starts_with(\"*const\") {\n            format!(\"{}.map_or(std::ptr::null(), |r| r as *const _)\", name)\n        } else {\n            format!(\"{}.map_or(std::ptr::null_mut(), |r| r as *mut _)\", name)\n        }\n    } else if p.mut_ref_enum_rust_type.is_some() {\n        format!(\"&mut {}_i32_\", name)\n    } else if p.rust_reexport_type == \"&str\" {\n        format!(\"c_{}.as_ptr()\", name)\n    } else if p.enum_rust_type.is_some() {\n        format!(\"{}.into()\", name)\n    } else {\n        name.to_string()\n    }\n}",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4151:ly,4152:wr,4153:mg,4154:mz,4155:ag,4156:ge,4157:cy,4158:lz,4159:mv,4160:gg,4161:br,4162:ui,4163:ho,4164:ax,4165:io,4166:dd,4167:si,4168:wr,4169:mg,4170:he,4171:ag,4172:ze,4173:cy,4174:lz,4175:rf,4176:gg,4177:sl,4178:ui,4179:mn,4180:ax,4181:io"
    }
  ]
}

Starting: *Update prelude + body generation* (6/10)

Now let me update both `cstr_prelude_params` and `cstr_prelude_resolved` to also generate the `let mut <name>_i32_` bindings. And I need a helper to generate the postamble. Then I need to update all the method body generation sites to use the multi-statement pattern when `mut_ref_enum` params exist.

Let me first update the prelude functions and add a postamble function:        

Reading lines 4186-4210 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L4186-L4211)
Completed with input: {
  "endLine": 4210,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 4186
}

I'll replace the prelude functions with versions that also handle `mut_ref_enum`:         

Edited: [bindings.rs -15+27](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L4187-L4214)
Completed with input: {
  "edits": [
    {
      "content": "/// Generate let-bindings for CString (&str) params and &mut enum params.\n/// These must appear before the unsafe block so the temporaries live long enough.\nfn cstr_prelude_params(params: &[ParamBinding]) -> String {\n    let mut result = String::new();\n    for p in params {\n        if p.rust_reexport_type == \"&str\" {\n            result.push_str(&format!(\"        let c_{} = std::ffi::CString::new({}).unwrap();\\n\", p.rust_name, p.rust_name));\n        }\n        if p.mut_ref_enum_rust_type.is_some() {\n            result.push_str(&format!(\"        let mut {}_i32_: i32 = (*{}).into();\\n\", p.rust_name, p.rust_name));\n        }\n    }\n    result\n}\n\nfn cstr_prelude_resolved(params: &[ResolvedParamBinding], names: &[String]) -> String {\n    let mut result = String::new();\n    for (p, name) in params.iter().zip(names.iter()) {\n        if p.rust_reexport_type == \"&str\" {\n            result.push_str(&format!(\"        let c_{} = std::ffi::CString::new({}).unwrap();\\n\", name, name));\n        }\n        if p.mut_ref_enum_rust_type.is_some() {\n            result.push_str(&format!(\"        let mut {}_i32_: i32 = (*{}).into();\\n\", name, name));\n        }\n    }\n    result\n}",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4187:af,4188:yr,4189:cy,4190:st,4191:qh,4192:py,4193:yi,4194:io,4195:dd,4196:rv,4197:vn,4198:sb,4199:mg,4200:yi,4201:io"
    }
  ]
}

Now I need to add a postamble helper and update the body builder to handle `&mut enum` with a multi-statement body. Let me add a postamble generator and update `build_reexport_body` to support it. Actually, the cleanest approach is to add a new function that checks if `&mut enum` params need the multi-statement pattern, and produce the whole body:         

Reading lines 4214-4240 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L4214-L4241)
Completed with input: {
  "endLine": 4240,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 4214
}

Summarized conversation history

Starting: *Add mut_ref_enum postamble helper* (1/9)

Let me first read the relevant method body generation sites to understand the patterns: 

Reading lines 4300-4420 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L4300-L4421)
Completed with input: {
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 4300,
  "endLine": 4420
}

Reading lines 4420-4570 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L4420-L4571)
Completed with input: {
  "endLine": 4570,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 4420
}

Reading lines 4570-4630 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L4570-L4631)
Completed with input: {
  "endLine": 4630,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 4570
}

Now let me also check the free function codegen in rust.rs: 

Reading lines 590-680 of [rust.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/rust.rs#L590-L681)
Completed with input: {
  "endLine": 680,
  "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
  "startLine": 590
}

Reading lines 680-710 of [rust.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/rust.rs#L680-L711)
Completed with input: {
  "endLine": 710,
  "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
  "startLine": 680
}

Let me check the current state of `convert_arg` and the prelude functions: 

Reading lines 4140-4215 of [bindings.rs](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L4140-L4216)
Completed with input: {
  "endLine": 4215,
  "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
  "startLine": 4140
}

Good, the prelude and convert_arg are already updated. Now I need to:
1. Add postamble helper functions
2. Modify the method body generation at each call site

Let me add the postamble helpers right after the prelude functions, then update `build_reexport_body` to accept a postamble: 

**Failing: [bindings.rs +42](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#L4214-L4256)**
Completed with input: {
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4213:io|",
      "insertAfter": true,
      "content": "\n/// Generate the postamble for &mut enum params: write back from i32 to typed enum.\nfn mut_ref_enum_postamble_params(params: &[ParamBinding], indent: &str) -> String {\n    let mut result = String::new();\n    for p in params {\n        if let Some(ref enum_type) = p.mut_ref_enum_rust_type {\n            result.push_str(&format!(\"{}*{} = {}::try_from({}_i32_).unwrap();\\n\", indent, p.rust_name, enum_type, p.rust_name));\n        }\n    }\n    result\n}\n\nfn mut_ref_enum_postamble_resolved(params: &[ResolvedParamBinding], names: &[String], indent: &str) -> String {\n    let mut result = String::new();\n    for (p, name) in params.iter().zip(names.iter()) {\n        if let Some(ref enum_type) = p.mut_ref_enum_rust_type {\n            result.push_str(&format!(\"{}*{} = {}::try_from({}_i32_).unwrap();\\n\", indent, name, enum_type, name));\n        }\n    }\n    result\n}\n\n/// Wrap a reexport body expression with &mut enum writeback postamble.\n/// When postamble is non-empty, splits into multi-statement body:\n///   let result_ = <body>;\n///   <postamble>\n///   result_\n/// For void returns (has_return=false):\n///   <body>;\n///   <postamble trimmed>\nfn wrap_body_with_postamble(body: &str, postamble: &str, has_return: bool) -> String {\n    if postamble.is_empty() {\n        return body.to_string();\n    }\n    if has_return {\n        format!(\"let result_ = {};\\n{}        result_\", body, postamble)\n    } else {\n        // Void return: body as statement, then postamble (trim trailing newline for last line)\n        let trimmed_postamble = postamble.trim_end_matches('\\n');\n        format!(\"{};\\n{}\", body, trimmed_postamble)\n    }\n}"
    }
  ]
}

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#4210-4210), lines 4210 to 4220

Replacing 5 lines with 46 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs)


```

```


Starting: *Update wrapper method body gen* (2/9)

Now let me update the wrapper method body generation site. I need to read the current state to get exact line numbers: 

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#4410-4410), lines 4410 to 4440

Now I need to add the postamble wrapping at each of the 4 sites. Let me update all four at once: 

Replacing 20 lines with 23 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs)


```

```
Now the direct methods site:  

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#4466-4466), lines 4466 to 4490

Replacing 19 lines with 22 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs)


```

```
Now the static methods site:  

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#4525-4525), lines 4525 to 4545

Replacing 5 lines with 8 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs)


```

```
Now the inherited methods site:   

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#4618-4618), lines 4618 to 4640

Replacing 5 lines with 8 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs)


```

```


Starting: *Fix inherited C++ codegen preamble* (6/9)

Now let me check the inherited C++ codegen (section 10) to see if it handles `&mut enum` preamble/postamble:  

Searched for text `Section 10` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for text `inherited wrapper` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Searched for regex `inherited.*C\+\+|inherited.*cpp|emit_inherited|generate.*inherited` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 2 results

Let me search for where inherited method C++ wrappers are generated:  

Searched for regex `inherited.*ffi|inherited.*wrapper_fn|inherited_methods.*cpp|generate_inherited_cpp` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 2 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#370-370), lines 370 to 400

Let me find where the C++ wrapper code for inherited methods is emitted:   

Searched for regex `emit_inherited|emit_cpp.*inherited|generate_cpp.*inherited` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `InheritedMethod|inherited_method.*cpp` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 5 results

Searched for regex `cpp_inherited|inherited.*cpp_output|inherited.*writeln|inherited.*push_str.*cpp` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Let me search for where the C++ wrapper functions are generated - they should use `cpp_output` or similar: 

Searched for regex `cpp_wrapper|cpp_code|emit.*cpp` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#3132-3132), lines 3132 to 3200

Let me search for the MutRefEnumParam wrapper kind section:   

Searched for text `MutRefEnumParam` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 5 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#3587-3587), lines 3587 to 3700

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#3700-3700), lines 3700 to 3720

Good — the section 4e C++ wrapper for `MutRefEnumParam` is already properly implemented with preamble/postamble. Now let me check the inherited methods C++ section to see if it has the same handling:   

Searched for regex `Inherited method C\+\+|inherited_methods.*cpp|section.*inherited` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `Inherited.*wrapper|10\.|emit.*inherited` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#4050-4050), lines 4050 to 4140

The inherited C++ codegen (section 10) doesn't handle `&mut enum` params. It needs preamble/postamble like section 4e. Let me check the `ResolvedParam` struct:    

Searched for regex `struct InheritedMethodBinding` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#245-245), lines 245 to 285

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#285-285), lines 285 to 330

Now I see that `ResolvedParamBinding` has `mut_ref_enum_rust_type` but NOT `mut_ref_enum_cpp_name`. The C++ codegen for inherited methods (section 10) uses `ResolvedParam` from the resolver (not `ResolvedParamBinding`). Let me check what `ResolvedParam` looks like and how the C++ side uses inherited params:    

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#4050-4050), lines 4050 to 4075

Let me check the ResolvedParamBinding construction for inherited methods to see how `&mut enum` params are set up:  

Searched for text `_enum_` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 20 results

Now let me see where the inherited ResolvedParamBinding construction handles `&mut enum`:    

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#2660-2660), lines 2660 to 2750

I need to add `mut_ref_enum_cpp_name` to `ResolvedParamBinding` and handle `&mut enum` detection in the inherited method construction. Let me check the resolver's type structure first: 

Searched for regex `pub enum_cpp_name|enum_cpp_name.*is.*set|extract_enum_name_from_type` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 4 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#1365-1365), lines 1365 to 1395

OK, so the resolver's `extract_enum_name_from_type` deliberately doesn't unwrap `MutRef`. That means `p.ty.enum_cpp_name` won't be set for `&mut enum` params. I need to detect this case in the inherited method construction.

Now let me add the `mut_ref_enum_cpp_name` field to `ResolvedParamBinding` and update the inherited method construction:   

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#310-310), lines 310 to 328

Replacing 16 lines with 18 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs)


```

```
Now I need to add `mut_ref_enum_cpp_name: None` to all the existing `ResolvedParamBinding` construction sites. Let me find them: 

Searched for text `ResolvedParamBinding {` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#2618-2618), lines 2618 to 2700

I need to add `mut_ref_enum_cpp_name` to all three `ResolvedParamBinding` construction sites: 

Replacing 37 lines with 40 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs)


```

```
Now I need to update section 10 (inherited C++ wrappers) to add the preamble/postamble for `&mut enum` params, just like section 4e does:  

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#4050-4050), lines 4050 to 4145

Now I need to add preamble/postamble to section 10. The `ResolvedParamBinding` now has `mut_ref_enum_cpp_name`. I need to:
1. Before the call: create local enum vars from int32_t
2. After the call: write back to int32_t

Also need to handle the return type for `&mut enum` — the section currently uses a simple `return` pattern but needs to be multi-statement when `&mut enum` params are present.   

Replacing 81 lines with 152 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs)


```

```


Starting: *Update free function codegen* (7/9)

Now let me update the free function codegen in rust.rs. It needs to handle `&mut enum` params in its args building, prelude, and body:    

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/rust.rs#626-626), lines 626 to 690

I need to update the free function codegen to handle `&mut enum` params — add them to args, prelude, and postamble:   

Replacing 61 lines with 96 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/rust.rs)


```

```


Starting: *Update PORTING.md* (8/9)

Now let me update PORTING.md to remove the limitation about `&mut i32`:   

Searched for regex `mut.*i32|mut.*enum.*out` (`**/crates/opencascade-sys/PORTING.md`), 2 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/PORTING.md#418-418), lines 418 to 430

Replacing 7 lines with 8 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/PORTING.md)


```

```


Starting: *Regenerate, build, and test* (9/9)

Now let me first check if the binding generator compiles before regenerating:  

Ran terminal command:  time cargo build -p opencascade-binding-generator 2>&1
Completed with input:  time cargo build -p opencascade-binding-generator 2>&1

Easy fix: the parameter was named `_all_enum_names` because it was unused. Now we use it. Let me rename it:  

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#2438-2438), lines 2438 to 2448

Replacing 1 lines with 1 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs)

Replacing 7 lines with 7 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs)


```

```


Ran terminal command:  time cargo build -p opencascade-binding-generator 2>&1
Completed with input:  time cargo build -p opencascade-binding-generator 2>&1

There's another reference to `_all_enum_names` that needs updating:   

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#2546-2546), lines 2546 to 2552

Replacing 1 lines with 1 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs)


```

```


Ran terminal command:  time cargo build -p opencascade-binding-generator 2>&1

Binding generator compiles. Now let me regenerate bindings:    

Ran terminal command:  time scripts/regenerate-bindings.sh 2>&1

Ran terminal command:  time scripts/regenerate-bindings.sh 2>&1 | tail -5

Regeneration succeeded. Now let me verify the generated code looks right for the `ShapeProcess_ShapeContext` case:     

Searched for text `get_continuity` (`**/crates/opencascade-sys/generated/shape_process.rs`), 2 results

Now `val` is `&mut crate::geom_abs::Shape` instead of `&mut i32`. Let me see the full method body:       

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/shape_process.rs#529-529), lines 529 to 550

The generated code looks exactly right:
1. CString prelude for the `param` string
2. `let mut val_i32_: i32 = (*val).into();` to convert enum to i32
3. FFI call with `&mut val_i32_`
4. `*val = crate::geom_abs::Shape::try_from(val_i32_).unwrap();` to write back
5. `result_` return

Now let me build the whole project:       

Ran terminal command:  time cargo build 2>&1

Clean build! Now run the binding generator tests:        

Ran terminal command:  time scripts/run-binding-generator-tests.sh 2>&1

All 16 tests pass. Now run workspace tests:          

Ran terminal command:  time cargo test --workspace --exclude opencascade-binding-generator 2>&1

Ran terminal command:  cargo test --workspace --exclude opencascade-binding-generator 2>&1 | grep -E "^test result:|FAILED|error"

All tests pass. No failures. 

Completed: *Regenerate, build, and test* (9/9)

All done. Here's a summary of the changes:

**Problem:** `&mut enum` output parameters (e.g., `GeomAbs_Shape& val`) were exposed as `&mut i32` in the Rust re-export API instead of using typed Rust enums like `&mut crate::geom_abs::Shape`.

**Changes across 3 files:**

1. **bindings.rs:**
   - Added `mut_ref_enum_rust_type: Option<String>` to `ParamBinding` and `mut_ref_enum_cpp_name`/`mut_ref_enum_rust_type` to `ResolvedParamBinding`
   - Updated `build_param_binding()` and inherited `ResolvedParamBinding` construction to look up typed enum and set `rust_reexport_type` to `"&mut <EnumType>"`
   - Updated `convert_arg()`/`convert_arg_resolved()` to pass `&mut <name>_i32_` for `&mut enum` params
   - Updated `cstr_prelude_params()`/`cstr_prelude_resolved()` to generate `let mut <name>_i32_: i32 = (*<name>).into();`
   - Added `mut_ref_enum_postamble_params()`/`mut_ref_enum_postamble_resolved()` and `wrap_body_with_postamble()` helpers
   - Applied postamble wrapping at all 4 method body generation sites (wrapper, direct, static, inherited)
   - Updated section 10 (inherited C++ wrappers) with preamble/postamble for `&mut enum` params

2. **rust.rs:** Updated `emit_free_function_wrapper()` with `&mut enum` handling in args, prelude, and postamble

3. **PORTING.md:** Updated the limitation note to reflect that `&mut enum` out-params now use typed enum references

Made changes.

User: commit

GitHub Copilot: Created [](file:///Users/sethml/src/opencascade-rs/tmp/commit_msg.txt)

Ran terminal command:  git add -A && git commit -F tmp/commit_msg.txt

Ran terminal command:  git log --oneline -1

Commit created successfully! 

**170da28** — `Expose typed Rust enums for &mut enum output parameters`

The commit includes all the changes across the binding generator to expose typed Rust enums for `&mut enum` output parameters, making them consistent with other enum parameter types. All tests pass and bindings regenerated successfully.

Made changes.