User: I think this is an artifical limitation - I suspect if we just delete this code, then inherited methods that use enum types will just work. Figure out if this is the case, or if not fix whatever limitation prevents inherited methods from using enums. Then regenerate the bindings, simplify the opencascade crate to use inherited methods that use enums directly rather than upcasting first, and update PORTING.md.

GitHub Copilot: 

  

Created 5 todos

Let me understand how inherited methods handle enums vs how direct/wrapper methods handle them. I need to trace the code paths. 

Investigate enum handling in inherited methods

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#1600-1600), lines 1600 to 1760

Searched for text `inherited` (`**/crates/opencascade-binding-generator/src/codegen/emit_ffi.rs`), no results

Searched for text `ResolvedMethod` (`**/crates/opencascade-binding-generator/src/symbol_table.rs`), no results

Searched for files matching `**/crates/opencascade-binding-generator/src/**/*.rs`, 14 matches

Searched for text `inherited` (`**/crates/opencascade-binding-generator/src/**/*.rs`), 20 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#193-193), lines 193 to 300

Searched for regex `inherited|InheritedMethod` (`**/crates/opencascade-binding-generator/src/codegen/cpp.rs`), no results

Searched for regex `inherited|InheritedMethod` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), no results

Searched for regex `ResolvedMethod|included_methods|enum_cpp_name` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 9 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#140-140), lines 140 to 200

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#1170-1170), lines 1170 to 1220

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#900-900), lines 900 to 960

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#400-400), lines 400 to 430

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#1890-1890), lines 1890 to 2380

Searched for regex `emit_ffi_rs|emit_module_rs|emit_reexport` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Searched for regex `fn emit_ffi|fn emit_module|fn emit_class_ffi|ffi_fn_name.*inherited|inherited.*ffi` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 5 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#2500-2500), lines 2500 to 2600

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#2656-2656), lines 2656 to 2900

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#278-278), lines 278 to 450

Searched for text `fn unified_type_to_string` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#493-493), lines 493 to 595

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#275-275), lines 275 to 310

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#1700-1700), lines 1700 to 1760

The investigation reveals the limitation is almost artificial - the C++ side already handles enums correctly, but the Rust FFI/reexport type strings are derived from the original unresolved types, which would produce class names instead of `i32`. A small fix to 4 lines makes it work.

Let me read the exact code to make precise edits: 

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#1670-1670), lines 1670 to 1760

Starting: *Remove enum skip and fix type mappings* (2/5)

Now I'll make the fix: remove the enum skip block and fix the 4 lines that derive Rust types from the unresolved original types. 

Replacing 39 lines with 23 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs)


```

```
Now I also need to handle the case where an enum is passed as `ConstRef` — the resolver resolves `ConstRef(Class("EnumName"))` to `i32` but the `extract_enum_name_from_type` function handles that. Let me verify the resolver handles const ref enum types properly for inherited methods: 

Searched for text `extract_enum_name_from_type` (`**/*.rs`), 3 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#1170-1170), lines 1170 to 1240

Good — the resolver correctly handles `ConstRef(Class("EnumName"))` as an enum, returning `i32` for both `rust_ffi_type` and `cpp_type`. So my fix should work.

Now let me also check if `ConstRef` enums need the same fix in my code — the `enum_cpp_name` check I added should catch both cases since the resolver sets `enum_cpp_name` for both `Class(enum)` and `ConstRef(Class(enum))`.

Let me regenerate the bindings to verify: 

Starting: *Regenerate bindings* (3/5)

Ran terminal command:  time bash scripts/regenerate-bindings.sh 2>&1

Read [](file:///Users/sethml/Library/Application%20Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5f139aa6-4685-470d-b0fe-972f17253a89/toolu_vrtx_01LssCrwF7FXtzmAJZQ2S3yj__vscode-1771111480634/content.txt#1-1), lines 1 to 50

Read [](file:///Users/sethml/Library/Application%20Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5f139aa6-4685-470d-b0fe-972f17253a89/toolu_vrtx_01LssCrwF7FXtzmAJZQ2S3yj__vscode-1771111480634/content.txt#50-50), lines 50 to 171

Bindings generated successfully. Now let me build opencascade-sys to verify the generated code compiles: 

Ran terminal command:  time cargo build -p opencascade-sys 2>&1

Ran terminal command:  cargo build -p opencascade-sys 2>&1 | grep -E "^(error|warning\[)" | wc -l

Ran terminal command:  cargo build -p opencascade-sys 2>&1 | grep -c "Finished"

opencascade-sys builds successfully. Now let me verify the new inherited methods actually exist. Let me check which enum-using methods are now generated: 

Starting: *Update opencascade crate to use direct enum methods* (4/5)

Searched for regex `orientation|shape_type|oriented|compose|composed` (`**/crates/opencascade-sys/generated/topo_ds.rs`), 20 results

Searched for regex `fn orientation|fn shape_type|fn oriented|fn compose\b|fn composed` (`**/crates/opencascade-sys/generated/topo_ds.rs`), 15 results

Let me check which subtypes now have the full set of enum methods (orientation, shape_type, oriented, compose, composed): 

Ran terminal command:  grep -n "fn orientation\|fn shape_type\|fn oriented\|fn compose\b\|fn composed\|impl Face\|impl Edge\|impl Wire\|impl Solid\|impl Shell\|impl Compound\b\|impl Vertex" crates/opencascade-sys/generated/topo_ds.rs | grep -v "//\|pub struct"

Ran terminal command:  grep -n "fn orientation\b\|fn orientation_orientation\|fn oriented\|fn compose\b\|fn composed" crates/opencascade-sys/generated/topo_ds.rs

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/topo_ds.rs#1020-1020), lines 1020 to 1100

These are on `TopoDS_Shape` itself (its own methods). Now let me check if subtypes have inherited versions: 

Ran terminal command:  grep -c "inherited_orientation\|inherited_shape_type\|inherited_oriented\|inherited_compose\b\|inherited_composed" crates/opencascade-sys/generated/topo_ds.rs
Completed with input:  grep -c "inherited_orientation\|inherited_shape_type\|inherited_oriented\|inherited_compose\b\|inherited_composed" crates/opencascade-sys/generated/topo_ds.rs

Ran terminal command:  grep -c "inherited_orientation\|inherited_shape_type\|inherited_oriented\|inherited_compose\|inherited_composed" crates/opencascade-sys/generated/ffi.rs
Completed with input:  grep -c "inherited_orientation\|inherited_shape_type\|inherited_oriented\|inherited_compose\|inherited_composed" crates/opencascade-sys/generated/ffi.rs

Let me check what `included_methods` returns — maybe the enum methods on TopoDS_Shape are direct CXX methods, not wrapper methods:  

Searched for text `included_methods` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 1 result

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#411-411), lines 411 to 430

Searched for regex `struct ResolvedMethod` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 1 result

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#154-154), lines 154 to 180

Let me look at a specific subtype to see which inherited methods it now has:  

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/topo_ds.rs#655-655), lines 655 to 850

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/topo_ds.rs#1470-1470), lines 1470 to 1490

Searched for regex `TopoDS_Shape.*(orientation|Orientation|shape_type|ShapeType)` (`**/crates/opencascade-sys/generated/ffi.rs`), 8 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi.rs#97945-97945), lines 97945 to 97980

Searched for text `all_method_names` (`**/crates/opencascade-binding-generator/src/*.rs`), 4 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#310-310), lines 310 to 425

Ran terminal command:  grep -c "Face_inherited_Orientation\|Face_inherited_ShapeType\|Face_inherited_Composed\|Face_inherited_Compose\b\|Face_inherited_Oriented" crates/opencascade-sys/generated/ffi.rs
Completed with input:  grep -c "Face_inherited_Orientation\|Face_inherited_ShapeType\|Face_inherited_Composed\|Face_inherited_Compose\b\|Face_inherited_Oriented" crates/opencascade-sys/generated/ffi.rs

They're not being generated. Let me debug why. Let me check if the resolver's symbol_table has these methods on `TopoDS_Shape`: 

Ran terminal command:  grep -c "TopoDS_Shape.*Orientation\|TopoDS_Shape.*ShapeType\|TopoDS_Shape.*Composed\|TopoDS_Shape.*Compose\b\|TopoDS_Shape.*Oriented" crates/opencascade-sys/generated/ffi.rs

Ran terminal command:  grep "TopoDS_Shape.*Orientation\|TopoDS_Shape.*ShapeType\|TopoDS_Shape.*Composed\|TopoDS_Shape.*Compose\b\|TopoDS_Shape.*Oriented" crates/opencascade-sys/generated/ffi.rs

Searched for text `use crate::` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 9 results

Good, it uses `crate::resolver::SymbolTable`. Let me add some debug logging to understand why methods aren't being inherited. Actually, let me first check whether the `Orientation` methods on `TopoDS_Shape` are classified as wrapper methods (which would be in `included_methods`) vs direct methods: 

Debug why inherited enum methods missing

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#1-1), lines 1 to 100

Searched for text `included_methods` (`**/crates/opencascade-binding-generator/**`), 5 results

Searched for text `BindingStatus` (`**/crates/opencascade-binding-generator/**`), 20 results

Searched for text `compute_inherited_method_bindings` (`**/crates/opencascade-binding-generator/**`), 2 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#100-100), lines 100 to 280

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#400-400), lines 400 to 500

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#1600-1600), lines 1600 to 1750

Searched for regex `enum_return|enum_param|EnumReturn|EnumParam|enum.*wrapper|wrapper.*enum` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 2 results

Searched for regex `resolve_method|fn resolve.*method|add_method|resolve_class` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 5 results

Searched for regex `UsesEnum|uses_enum` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 11 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#760-760), lines 760 to 960

Searched for regex `UsesEnum` (`**/crates/opencascade-binding-generator/**`), 1 result

Searched for regex `enum_return_methods|enum_param_methods|all_method_names` (`**/crates/opencascade-binding-generator/src/model.rs`), 1 result

Searched for regex `enum_return_methods|enum_param_methods|methods\.push` (`**/crates/opencascade-binding-generator/src/parser.rs`), 2 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#340-340), lines 340 to 450

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#70-70), lines 70 to 120

Searched for regex `enum_return|enum_param|EnumReturn|EnumParam` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results

Searched for regex `method_uses_enum|uses_enum` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 2 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#380-380), lines 380 to 500

Searched for regex `WrapperKind|MethodBinding|enum_methods|EnumConversion` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 20 results

Read [](file:///Users/sethml/src/opencascade-binding-generator/src/codegen/bindings.rs)

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#900-900), lines 900 to 1050

Searched for regex `fn parse_method` (`**/crates/opencascade-binding-generator/src/parser.rs`), 1 result

Searched for regex `fn is_method_bindable` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/parser.rs#724-724), lines 724 to 800

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#557-557), lines 557 to 620

Searched for regex `TopoDS_Shape.*orientation|TopoDS_Shape.*Orientation|TopoDS_Shape_Orientation` (`**/crates/opencascade-sys/generated/**`), 12 results

Searched for regex `TopoDS_Face.*inherited|TopoDS_Face_inherited` (`**/crates/opencascade-sys/generated/**`), 20 results

Searched for regex `TopoDS_Shape_ShapeType|TopoDS_Shape_shape_type|shape_type` (`**/crates/opencascade-sys/generated/**`), 20 results

Searched for regex `method_has_unsupported_by_value_params|has_unsupported_by_value` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 4 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#560-560), lines 560 to 590

Searched for regex `fn method_needs_wrapper` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 1 result

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#633-633), lines 633 to 660

Searched for regex `dump.symbols|dump_symbols|--dump|verbose|debug-class` (`**/crates/opencascade-binding-generator/src/main.rs`), 17 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/main.rs#380-380), lines 380 to 500

Searched for regex `TopoDS_Face_inherited_` (`**/crates/opencascade-sys/generated/ffi.rs`), 26 results

Searched for regex `has_unbindable_types` (`**/crates/opencascade-binding-generator/src/model.rs`), 4 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#175-175), lines 175 to 210

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#210-210), lines 210 to 240

Searched for regex `fn resolve_type` (`**/crates/opencascade-binding-generator/src/resolver.rs`), 1 result

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/resolver.rs#1177-1177), lines 1177 to 1275

Searched for regex `fn TopoDS_Shape_` (`**/crates/opencascade-sys/generated/ffi.rs`), 13 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#1600-1600), lines 1600 to 1605

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#1604-1604), lines 1604 to 1700

Searched for regex `TopoDS_Shape` (`**/crates/opencascade-sys/generated/ffi.rs`), 100 results

Searched for regex `type TopoDS_Shape` (`**/crates/opencascade-sys/generated/ffi.rs`), 1 result

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi.rs#97760-97760), lines 97760 to 97970

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi.rs#97970-97970), lines 97970 to 98020

Searched for regex `fn is_unbindable` (`**/crates/opencascade-binding-generator/src/model.rs`), 1 result

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/model.rs#530-530), lines 530 to 580

Searched for regex `fn type_uses_unknown_handle` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), 1 result

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/type_mapping.rs#316-316), lines 316 to 380

Replacing 22 lines with 26 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs)

```

```



Ran terminal command:  time bash scripts/regenerate-bindings.sh 2>&1

Read [](file:///Users/sethml/Library/Application%20Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5f139aa6-4685-470d-b0fe-972f17253a89/toolu_vrtx_016pg1KHBgY5HSNGRJQUzh43__vscode-1771111480716/content.txt#1-1), lines 1 to 50

Read [](file:///Users/sethml/Library/Application%20Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5f139aa6-4685-470d-b0fe-972f17253a89/toolu_vrtx_016pg1KHBgY5HSNGRJQUzh43__vscode-1771111480716/content.txt#50-50), lines 50 to 171

Searched for regex `TopoDS_Face_inherited_Orientation|TopoDS_Face_inherited_orientation|TopoDS_Face_inherited_ShapeType|TopoDS_Face_inherited_shape_type|TopoDS_Face_inherited_Compose|TopoDS_Face_inherited_compose` (`**/crates/opencascade-sys/generated/ffi.rs`), 4 results

Searched for regex `TopoDS_Edge_inherited_Orientation|TopoDS_Edge_inherited_ShapeType|TopoDS_Edge_inherited_Composed` (`**/crates/opencascade-sys/generated/ffi.rs`), 3 results

Ran terminal command:  time cargo build -p opencascade-sys 2>&1

Ran terminal command:  tail -20 "/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/5f139aa6-4685-470d-b0fe-972f17253a89/toolu_vrtx_01KZzxHGuHXD2VDHbM5T7cfq__vscode-1771111480721/content.txt"

Searched for regex `impl TopoDS_Face` (`**/crates/opencascade-sys/generated/topo_ds.rs`), no results

Searched for regex `orientation|shape_type|compose` (`**/crates/opencascade-sys/generated/topo_ds.rs`), 30 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#1604-1604), lines 1604 to 1615

Searched for regex `all_enum_names` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 20 results

Replacing 6 lines with 6 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs)

```

```



Ran terminal command:  time cargo build -p opencascade 2>&1 | grep -E "error|Finished|warning.*opencascade-sys" | head -5

Ran terminal command:  time cargo build -p opencascade 2>&1 | tail -5

The "unknown type" filter also catches enum types since they're `Type::Class("TopAbs_Orientation")` and not in `all_class_names`. I need to exclude enums from that check too. 

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#1650-1650), lines 1650 to 1685

The fix is already in place from the subagent. Let me regenerate and verify:  

Ran terminal command:  time bash scripts/regenerate-bindings.sh 2>&1

Ran terminal command:  grep -c "Face_inherited_Orientation\|Face_inherited_ShapeType\|Face_inherited_Composed\|Face_inherited_Compose\b\|Face_inherited_Oriented" crates/opencascade-sys/generated/ffi.rs

Now they're generated. Let me verify the build:   

Ran terminal command:  time cargo build -p opencascade-sys 2>&1

Ran terminal command:  cargo build -p opencascade-sys 2>&1 | grep -E "^error" | wc -l

Ran terminal command:  cargo build -p opencascade-sys 2>&1 | grep "Finished"

Build succeeds. Let me verify which enum methods are now available on Face as an example:  

Ran terminal command:  grep "fn orientation\|fn shape_type\|fn oriented\|fn compose\b\|fn composed\|fn orientation_orientation" crates/opencascade-sys/generated/topo_ds.rs

All the enum-using methods are now generated on all subtypes. Now let me fix the unused variable warning, then update the opencascade crate:    

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator/src/codegen/bindings.rs#1600-1600), lines 1600 to 1615

Good, the warning is already fixed. Now let me find and update the opencascade crate to use the new direct inherited methods instead of going through `as_shape()`:  

Find as_shape enum method usage

Searched for text `\.as_shape\(\)\.orientation\(\)` (`**/crates/opencascade/**`), 2 results

Searched for text `\.as_shape\(\)\.shape_type\(\)` (`**/crates/opencascade/**`), no results

Searched for text `\.as_shape\(\)\.oriented\(` (`**/crates/opencascade/**`), no results

Searched for text `\.as_shape\(\)\.compose\(` (`**/crates/opencascade/**`), no results

Searched for text `\.as_shape\(\)\.composed\(` (`**/crates/opencascade/**`), no results

Searched for text `\.as_shape\(\)\.orientation_orientation\(` (`**/crates/opencascade/**`), no results

Searched for text `as_shape_mut().orientation_orientation(` (`**/crates/opencascade/**`), no results

Searched for text `as_shape_mut().compose(` (`**/crates/opencascade/**`), no results

Searched for regex `\.as_shape(_mut)?\(\)\.\w+` (`**/crates/opencascade/**`), 7 results

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade/src/primitives/face.rs#370-370), lines 370 to 378

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade/src/primitives/shape.rs#265-265), lines 265 to 272

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade/src/primitives/face.rs#365-365), lines 365 to 380

Replacing 3 lines with 3 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade/src/primitives/face.rs)


```

```
Now let me build the `opencascade` crate to make sure it still works:  

Ran terminal command:  time cargo build -p opencascade 2>&1

Ran terminal command:  cargo build -p opencascade 2>&1 | grep -E "^error" | wc -l && cargo build -p opencascade 2>&1 | grep "Finished"

Starting: *Update PORTING.md* (5/5)

Now let me update PORTING.md to remove the limitation about inherited methods and enum types:   

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/PORTING.md#330-330), lines 330 to 370

Read [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/PORTING.md#365-365), lines 365 to 413

Replacing 23 lines with 12 lines in [](file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/PORTING.md)


```

```


Completed: *Update PORTING.md* (5/5)

Here's a summary of changes:

**Root cause:** Two issues prevented inherited methods from using enum types:
1. The explicit skip block in `compute_inherited_method_bindings` that filtered out methods with enum params/returns
2. The "unknown type" filter treated enum types (e.g., `TopAbs_Orientation`) as unknown classes since they're `Type::Class("TopAbs_Orientation")` but not in `all_class_names`

**Fixes in bindings.rs:**
- Removed the 15-line enum skip block
- Added `enum_cpp_name.is_none()` guards to the unknown-type filter so enums aren't treated as unknown classes
- Fixed Rust FFI/reexport type strings to use `"i32"` for enum types (instead of deriving from `original` which would produce class names)

**Result:** Methods like `orientation()`, `shape_type()`, `oriented()`, `compose()`, `composed()`, and `orientation_orientation()` are now available directly on all TopoDS subtypes (Face, Edge, Wire, Shell, Solid, Compound, Vertex, CompSolid) — no more `as_shape()` needed.

**Updated files:**
- bindings.rs — generator fixes
- Regenerated all files in generated
- face.rs — simplified `self.inner.as_shape().orientation()` → `self.inner.orientation()`
- PORTING.md — removed the limitation and updated the inherited methods documentation

Made changes.

User: commit

GitHub Copilot: Ran terminal command:  git status

Ran terminal command:  git add crates/opencascade-binding-generator/src/codegen/bindings.rs crates/opencascade-sys/PORTING.md crates/opencascade/src/primitives/face.rs crates/opencascade-sys/generated/

Ran terminal command:  cd /Users/sethml/src/opencascade-rs && git commit -m "Enable inherited methods with enum types on TopoDS subtypes

- Remove artificial limitation that prevented inherited methods from using enum types
- Fix Rust FFI type mapping for enum parameters/returns in inherited methods  
- Regenerate bindings to include orientation(), shape_type(), oriented(), compose(), composed() methods directly on TopoDS subtypes
- Update opencascade crate to use direct inherited enum methods instead of as_shape()
- Update PORTING.md to reflect that enum methods are now available as inherited methods"

Ran terminal command:  git log --oneline -1

The changes have been successfully committed! 

**Commit Summary:**
- **Hash:** `d8d713f`
- **Title:** "Enable inherited methods with enum types on TopoDS subtypes"
- **Files changed:** 28 files (2440 insertions, 48 deletions)
- **Description:** Removed the artificial limitation preventing inherited methods from using enum types, fixed type mappings, regenerated bindings, updated the opencascade crate, and updated documentation.

All the work to enable direct access to enum-using methods like `orientation()`, `shape_type()`, `oriented()`, `compose()`, and `composed()` on TopoDS subtypes (Face, Edge, Wire, etc.) has been completed and committed. The opencascade crate now builds successfully with the simplified API.