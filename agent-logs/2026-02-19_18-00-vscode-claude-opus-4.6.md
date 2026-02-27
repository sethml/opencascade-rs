# Code Fix and Commit Instructions

- **Date:** 2026-02-19 16:31 – 21:30
- **Model:** Claude Opus 4.6
- **Turns:** 4
- **Tool calls:** 539
- **Thinking blocks:** 57
- **Input words:** 33,793
- **Output words:** 32,037
- **Prompt tokens (last round):** 251,375
- **API rounds:** 399
- **Total elapsed:** 10233s

## Table of Contents

1. [Fix this, and commit.](#user-1) (Claude Opus 4.6)
2. [OK, investigate the template instantiations and bare unqualified types that are causing skips, sp...](#user-2) (Claude Opus 4.6)
3. [Let&#x27;s make functions that take istream/ostream work, so clients can print debugging info. I think...](#user-3) (Claude Opus 4.6)
4. [I modified the stream accessors to not be unsafe. Update the README to reflect your and my stream...](#user-4) (Claude Opus 4.6)

---

<a id="user-1"></a>

## User (1)

> Fix this, and commit.

*2026-02-19 16:31*

### Assistant — Claude Opus 4.6

> 💭 The user is pointing at line 369-370 in README.md which shows:
> 
> ```markdown
> | 79 | 5.0% | **Unresolved template/nested type** | Template instantiations or nested types that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `math_VectorBase<>`, etc.) |
> ```
> 
> They want me to "fix this" - I need to understand what's wrong. The nested type issue was marked as "(SOLVED)" in the binding generator README's Future Work section. So the skip reason table in the main README.md still lists "Unresolved template/nested type" combining both template and nested types, but nested types have been solved.
> 
> 
> I need to check the actual skip counts in the generated files and compare them against what the binding generator README says to see if the table needs updating or if the categories need to be split.

<details>
<summary>Reading lines 355-395 of <a href="../README.md#L355-L396">README.md</a></summary>

```
Error: startLine 355 exceeds file length (168 lines).
```
</details>

<details>
<summary>Reading lines 275-310 of <a href="../crates/opencascade-binding-generator/README.md#L275-L311">README.md</a></summary>

```
3. **Generate**: All binding decisions computed into `ClassBindings` structs (`codegen/bindings.rs`), then Rust and C++ code emitted from them (`codegen/rust.rs`, `codegen/cpp.rs`)

All method filtering (enum checks, lifetime issues, by-value params, etc.) is centralized in `resolver.rs` and applied consistently to both FFI and impl generation.

### Wrapper Functions

All methods use extern "C" C++ wrapper functions, since there is no direct Rust–C++ ABI bridge. The wrappers handle:

| C++ Pattern | Wrapper Approach |
|-------------|-----------------|
| Constructor | `new T(args)` returning `T*` |
| Return by value | `new T(obj.Method(args))` returning `T*` |
| Static method | Free function calling `ClassName::Method()` |
| `Handle<T>` | `typedef opencascade::handle<T> HandleT;` |
| Overloaded method | Suffix: `_real3`, `_pnt2`, etc. |
| `const char*` param | Pass-through as `const char*` |
| `const char*` return | Pass-through as `const char*` |
| Nullable `T*` param | Pass-through as `T*` (Rust uses `Option<&mut T>`) |
| Nullable `const T*` param | Pass-through as `const T*` (Rust uses `Option<&T>`) |
| Inherited method | Free function calling `self->Method()` |
| Upcast (const) | `Derived_as_Base(self) -> const Base*` |
| Upcast (mut) | `Derived_as_Base_mut(self) -> Base*` |
| By-value Handle param | Wrapper accepts `const Handle<T>&` |
| By-value class param | Wrapper accepts `const T&` |
| Const/mut return fix | `ConstMutReturnFix`: const-cast for non-const `self` |
| `&mut` enum out-param | Local `int32_t` var + writeback |
| `const T*` return | Pass-through as pointer (Rust uses `Option<&T>`) |
| `T*` return (non-const method) | Pass-through as pointer (Rust uses `Option<&mut T>`) |
| `T*` return (const method) | Pass-through as pointer (Rust uses `Option<&T>`, downgraded for soundness) |

### Handle Support

Classes inheriting from `Standard_Transient` get:
- `ClassName::to_handle(obj)` — wrap in `Handle<T>`
- `handle.to_handle_base()` — upcast Handle to base type
- `handle.downcast_to_derived()` — type-checked downcast
```
</details>

<details>
<summary>Reading lines 1-168 of <a href="../README.md#L1-L169">README.md</a></summary>

````
# opencascade-rs

Rust bindings to [OpenCascade](https://dev.opencascade.org). The code is currently a major work in progress.

I currently work a full-time job and work on this in my spare time, so please adjust timing expectations accordingly :)

## Major Goals
* Define 3D CAD models suitable for 3D printing or machining, in ergonomic Rust code
* Code-first approach, but allow use of a GUI where it makes sense (2D sketches)
* Support fillets, chamfers, lofts, surface filling, pipes, extrusions, revolutions, etc.
* Support quick compile times for faster iterations
* Ability to import/export STEP files, STL files, SVG, DXF, KiCAD files, and hopefully more!
* Easy to install the viewer app (provide pre-built binaries for major platforms)
* Easy to create and use user-authored libraries (via crates.io) for quick and easy code-sharing
* Pretty visualizations of created parts
* Ability to specify assemblies of parts, and constraints between assembled parts

## Rationale

This project was born out of me designing [my own keyboard](https://github.com/bschwind/key-ripper) and wanting to make a 3D-printed or CNCed aluminum case for it. In typical over-engineering fashion, I didn't want to just use Fusion360 and call it a day. I wanted a fully parameterized, fully open-source, code-based approach so I can easily make changes, and store the model code in version control. I also want to be fairly confident I can build these models any time in the future given I have a C++/Rust toolchain available.

So I researched what kernels are out there, learned that OpenCascade is one of the few open-source B-Rep (boundary representation) kernels available, and started writing bindings to it with cxx.rs to see if usage of the kernel is feasible. Turns out it is!

### Why Rust?

At this point I'm most comfortable with Rust, so most tools I build will be with Rust. I also don't find any joy in creating my own language or forcing people to learn one I created. Rust is a far better language than I could ever make myself, and contains pretty much every facility I would want for defining 3D models in code. Ultimately it's a hobby project and when you run a hobby project, you get to pick whatever you want :)

There are other benefits:

* Easy to install the Rust toolchain
* Strong type system can inform you what you can do with say, a `Wire` or a `Shape`
* Great generated documentation
* Good cross-platform support
* Excellent library ecosystem on crates.io, making parts sharing a breeze
* High level Rust can be ergonomic, with iterators, closures, operator overloading, and enums
* Rust's unique (`&mut T`) and shared (`&T`) references and function type signatures inform you when an operation modifies a shape vs. creating a new one

## Dependencies

* Rust Toolchain (https://rustup.rs/)
* CMake (https://cmake.org/)
* A C++ compiler with C++11 support

## Building

* The `OCCT` codebase is included as a git submodule. Clone the repo with the `--recursive` flag, or use `git submodule update --init` to fetch the submodule.
* `cargo build --release`

### Using pre-installed OpenCASCADE

If you have the `OCCT` library already installed via a package manager, you can dynamically link to it which will significantly decrease build times. By default, the `builtin` feature is enabled which means compiling OCCT from source. You can disable it via the command line:

`cargo build --no-default-features`

or by specifying `default-features = false` in your `Cargo.toml`:

```
[dependencies]
opencascade = { version = "0.2", default-features = false }
```

NOTE: If you have installed `OCCT` manually you may need specify the path to it via the `DEP_OCCT_ROOT` environment variable. The specified root directory usually contains `include` and `lib` directories.

## Run Examples

* `cargo run --release --example bottle`

The program will output `bottle.stl` in the current working directory.

### Lower Level

There are low level examples which are more or less directly calling Ope
... (truncated, 9824 chars)
````
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_01CDKu3FFei9bwrJQ2ni2F8d__vscode-1771312785121/content.txt)

> 💭 The README.md is only 168 lines long. The user's selection at lines 369-370 must be from the binding-generator README.md, not the main README.md. Let me re-read the user's selection - it's from `README.md` but lines 369-370. Wait, let me check - the attachment says "Excerpt from README.md, lines 369 to 370" but the file is only 168 lines. Let me check if the user is looking at the binding-generator README.
> 
> Looking at the binding generator README.md - nah, the attachment is explicitly from `README.md`. But the main README.md is only 168 lines. 
> 
> Ah, the `editorContext` clarifies it - the user's current file is the binding-generator README.md, so the selection is from that file at lines 369-370, not the main project README.

<details>
<summary>Reading lines 355-385 of <a href="../crates/opencascade-binding-generator/README.md#L355-L386">README.md</a></summary>

````
// SKIPPED: **Source:** `gp_XYZ.hxx`:109 - `gp_XYZ::GetData`
//   method: Returns a const ptr to coordinates location.
//   Reason: has unbindable types: return: raw pointer (const double*)
//   // pub fn get_data(&self) -> /* const double* */;
```

### Skip Reason Breakdown

| Count | % | Category | Description |
|------:|----:|----------|-------------|
| 546 | 34.8% | **Stream type** | C++ `std::istream`/`std::ostream` (`Standard_IStream`/`Standard_OStream`) — no Rust equivalent |
| 451 | 28.7% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`Standard_SStream`, `Handle(TDocStd_Document)`, etc.) |
| 199 | 12.7% | **Void pointer** | `Standard_Address` (typedef for `void*`) — cannot be safely expressed in Rust FFI |
| 135 | 8.6% | **Raw pointer** | `T*`/`const T*` returns (static/free fn) or primitive pointer params (`int*`, `double*`) — class pointer returns on instance methods are now bound as `Option<&T>`/`Option<&mut T>` |
| 79 | 5.0% | **Unresolved template/nested type** | Template instantiations or nested types that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `math_VectorBase<>`, etc.) |
| 53 | 3.4% | **Ambiguous lifetimes** | `&mut` return with reference params — Rust lifetime inference is ambiguous |
| 43 | 2.7% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |
| 18 | 1.1% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |
| 16 | 1.0% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |
| 12 | 0.8% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across FFI |
| 7 | 0.4% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(IGESData_IGESModel)`, `Handle(CDM_MetaData)`, etc.) |
| 5 | 0.3% | **Not CppDeletable** | Return type class has no destructor in the binding set |
| 4 | 0.3% | **&mut enum return** | Mutable reference to enum (cxx limitation) |
| 2 | 0.1% | **Excluded by bindings.toml** | Explicitly excluded in config (e.g., ambiguous overload workarounds) |
| 1 | 0.1% | **Ambiguous overload** | C++ overload that would produce identical wrapper signatures |

### Most Common Unknown Types

The "unknown type" and "unknown Handle type" categories (29% of all skips) are dominated by a few types:

| Count | Type | How to Unblock |
````
</details>

> 💭 The issue is that line 369 says "**Unresolved template/nested type**" but according to the Future Work section, nested types have been SOLVED. The description says "Template instantiations or nested types that can't be resolved" but nested types are now resolved. I need to:
> 
> 1. Get the actual current skip counts by running the binding generator
> 2. Update the table to reflect reality
> 
> Let me first check what the actual skip reasons look like in the generated code to get accurate counts.

**Todo list:**
- 🔄 Get actual skip counts from generated code
- ⬜ Update skip reason table in README
- ⬜ Commit the fix

Let me get the actual current skip counts from the generated code to update the table accurately.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^// SKIPPED:" crates/opencascade-sys/generated/*.rs | grep -oP 'Reason: .*' | sed 's/Reason: //' | sort | uniq -c | sort -rn
```
<details>
<summary>Output (5 lines)</summary>

```
grep: invalid option -- P
usage: grep [-abcdDEFGHhIiJLlMmnOopqRSsUVvwXxZz] [-A num] [-B num] [-C[num]]
[-e pattern] [-f file] [--binary-files=value] [--color=when]
[--context[=num]] [--directories=action] [--label] [--line-buffered]
[--null] [pattern] [file ...]
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | sort | uniq -c | sort -rn
```
<details>
<summary>Output (431 lines)</summary>

```
 207 has unbindable types: param 'S': stream type (Standard_OStream&)
  85 param 'theMessage' uses unknown type 'Standard_SStream&'
  57 has unbindable types: param 'anOS': stream type (Standard_OStream&); return: stream type (Standard_OStream&)
  53 returns &mut with reference params (ambiguous lifetimes)
  46 param 'theDocument' uses unknown type 'const Handle(TDocStd_Document)&'
  46 has unbindable types: param 'o': stream type (Standard_OStream&)
  43 class is abstract (has unimplemented pure virtual methods)
  41 has unbindable types: param 'S': void pointer (Standard_Address)
  40 has unbindable types: param 'OS': stream type (Standard_OStream&)
  38 has unbindable types: param 'anOStream': stream type (Standard_OStream&); return: stream type (Standard_OStream&)
  31 has unbindable types: param 'C': void pointer (Standard_Address)
  25 param 'theDoc' uses unknown type 'const Handle(TDocStd_Document)&'
  22 has unbindable types: param 'PCurve': void pointer (const Standard_Address&)
  22 has unbindable types: param 'anOStream': stream type (Standard_OStream&)
  16 has unbindable types: param 'IS': stream type (Standard_IStream&)
  14 return type 'void*' is unknown
  14 param 'theStyle' uses unknown type 'const XCAFPrs_Style&'
  13 has unbindable types: return: void pointer (Standard_Address)
  11 has unbindable types: param 'theStream': stream type (Standard_OStream&)
  11 has unbindable types: param 'theOStream': stream type (Standard_OStream&)
  11 has unbindable types: param 'C': void pointer (const Standard_Address&)
  10 param 'doc' uses unknown type 'const Handle(TDocStd_Document)&'
  10 has unbindable types: param 'OS': stream type (Standard_OStream&); return: stream type (Standard_OStream&)
  10 has unbindable types: param 'C1': void pointer (const Standard_Address&); param 'C2': void pointer (const Standard_Address&)
  10 has unbindable types: param 'anOS': stream type (Standard_OStream&)
   9 has string ref param 'name' of type 'const char*&' (needs manual binding)
   8 param 'theFlags' uses unknown type 'const ShapeProcess::OperationsFlags&'
   8 has unbindable types: param 'theParameters': rvalue reference (XSAlgo_ShapeProcessor::ParameterMap&&)
   8 has unbindable types: param 'Surface': void pointer (const Standard_Address&)
   7 param 'theStream' uses unknown type 'const Standard_SStream&'
   7 has unbindable types: return: raw pointer (const uint16_t*)
   7 has unbindable types: param 'theIStream': stream type (std::istream&)
   6 return type 'Handle(IMeshData::ListOfPnt2d)' is unknown
   6 return type 'const IMeshData::MapOfInteger&' is unknown
   6 param 'theDFace' uses unknown type 'const IMeshData::IFaceHandle&'
   6 param 'theDEdge' uses unknown type 'const IMeshData::IEdgeHandle&'
   6 has unbindable types: return: unresolved template/nested type (LDOMString)
   6 has unbindable types: param 'theOS': stream type (Standard_OStream&); return: stream type (Standard_OStream&)
   6 has u
... (truncated, 49244 chars total)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_016MLWT5ceHFUe1t8iykWL1o__vscode-1771312785126/content.txt#1-1), lines 1 to 50

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | wc -l
```
```
    1567
```

Now let me categorize these into the skip reason buckets:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "stream type"
```
```
546
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "void pointer"
```
```
199
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "raw pointer"
```
```
135
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "unresolved template/nested type"
```
```
79
```

> 💭 The counts match the table exactly. So the numbers are correct. The issue the user is highlighting is that line 369 says "**Unresolved template/nested type**" with the description "Template instantiations or nested types that can't be resolved", but the Future Work section says nested types are "(SOLVED)". 
> 
> Let me check what the actual unresolved template/nested type skips look like - are any of them actually nested types, or are they all template types now?

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep "unresolved template/nested type" | sort | uniq -c | sort -rn
```
<details>
<summary>Output (54 lines)</summary>

```
   6 has unbindable types: return: unresolved template/nested type (LDOMString)
   4 has unbindable types: return: unresolved template/nested type (std::shared_ptr<std::streambuf>)
   4 has unbindable types: return: unresolved template/nested type (const std::pair<Standard_Real, Standard_Real>&)
   3 has unbindable types: param 'theData': unresolved template/nested type (const LDOMString&)
   3 has unbindable types: param 'aName': unresolved template/nested type (const LDOMString&)
   2 has unbindable types: return: unresolved template/nested type (NCollection_Array1<opencascade::handle<HLRAlgo_PolyData>>&)
   2 has unbindable types: return: unresolved template/nested type (math_VectorBase<>)
   2 has unbindable types: return: unresolved template/nested type (const NCollection_DataMap<TCollection_AsciiString, opencascade::handle<STEPCAFControl_ExternFile>>&)
   2 has unbindable types: return: unresolved template/nested type (const LDOMBasicString&)
   2 has unbindable types: param 'V': unresolved template/nested type (const math_VectorBase<>&)
   2 has unbindable types: param 'theTagName': unresolved template/nested type (const LDOMString&)
   2 has unbindable types: param 'theItems': unresolved template/nested type (const NCollection_Handle<StepVisual_Array1OfTessellatedItem>&)
   2 has unbindable types: param 'theAddStatus1': unresolved template/nested type (const NCollection_Vector<ProxPnt_Status>&)
   2 has unbindable types: param 'aValue': unresolved template/nested type (const LDOMString&)
   2 has unbindable types: param 'aValue': unresolved template/nested type (const LDOMBasicString&)
   1 has unbindable types: return: unresolved template/nested type (std::pair<Standard_Integer, Standard_Integer>)
   1 has unbindable types: return: unresolved template/nested type (NCollection_List<const char *>::Iterator)
   1 has unbindable types: return: unresolved template/nested type (NCollection_Handle<StepVisual_VectorOfHSequenceOfInteger>)
   1 has unbindable types: return: unresolved template/nested type (NCollection_Handle<StepVisual_Array1OfTessellatedItem>)
   1 has unbindable types: return: unresolved template/nested type (NCollection_Array1<opencascade::handle<HLRAlgo_PolyShellData>>&)
   1 has unbindable types: return: unresolved template/nested type (NCollection_Array1<gp_Vec3f>&)
   1 has unbindable types: return: unresolved template/nested type (const NCollection_Sequence<RWGltf_GltfPrimArrayData>&)
   1 has unbindable types: return: unresolved template/nested type (const NCollection_List<gp_Pnt>&)
   1 has unbindable types: return: unresolved template/nested type (const NCollection_IndexedMap<Message_MetricType>&)
   1 has unbindable types: return: unresolved template/nested type (const NCollection_Array1<opencascade::handle<HLRAlgo_PolyShellData>>&)
   1 has unbindable types: return: unresolved template/nested type (BVH_Box<Standard_Real, 2>)
   1 has unbindable types: param 'theXYZs': unresolved template/nest
... (truncated, 7347 chars total)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_012mMUzWdt9qZTHwv18mwQGy__vscode-1771312785133/content.txt)

> 💭 So looking at all 79 "unresolved template/nested type" skips — they are ALL template types (like `LDOMString`, `std::pair<...>`, `NCollection_DataMap<...>`, `math_VectorBase<>`, etc.) and not nested class types. The nested type issue was solved as described in the Future Work section.
> 
> So the fix should be to rename the category from "Unresolved template/nested type" to just "Unresolved template type" and update the description accordingly. The examples given in the table (`NCollection_DataMap<...>`, `std::pair<...>`, `math_VectorBase<>`) are all template types, not nested types.
> 
> 
> Wait, I'm second-guessing myself on `LDOMString` and `LDOMBasicString` — those don't look like templates at first glance, so let me verify whether they're actually template types or just unresolved types not in the binding set.
> 
> Looking at the skip messages more carefully, they're all marked as "unresolved template/nested type," but that classification might not be accurate for all of them. `LDOMString` and `LDOMBasicString` seem to be regular typedefs or unknown types rather than actual templates. The rest — `std::pair`, `NCollection_DataMap`, `std::shared_ptr`, `NCollection_Array1`, `BVH_Box`, and `NCollection_List<...>::Iterator` — are clearly either templates or nested types within templates.
> 
> Let me check the resolver logic to understand why `LDOMString` and `LDOMBasicString` are getting classified this way, or maybe I should just look at what the skip reason actually says and work backward from there.
> 
> Actually, rereading the user's selection, I notice the category description mentions "nested types that can't be resolved," but according to the Future Work section, nested types have already been solved. So the category name and description are outdated. These are really just unresolved template instantiations — types that aren't in the binding set. I should update the label to "Unresolved template type" and adjust the description accordingly.
> 
> But wait, `LDOMString` isn't even a template, so I need to check what the resolver actually considers this category to be.

Searched for text `unresolved template/nested type` (`**/crates/opencascade-binding-generator/src/**`), 1 result
- [bindings.rs:809](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L809)

<details>
<summary>Reading lines 790-825 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L790-L826">bindings.rs</a></summary>

```
        if p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none() {
            parts.push(format!("param '{}': {}", p.name, describe_unbindable_reason(&p.ty)));
        }
    }
    if let Some(ref ret) = method.return_type {
        if ret.is_unbindable() {
            parts.push(format!("return: {}", describe_unbindable_reason(ret)));
        }
    }
    if parts.is_empty() { "unknown".to_string() } else { parts.join("; ") }
}

/// Describe why a specific type is unbindable.
fn describe_unbindable_reason(ty: &Type) -> String {
    if ty.is_stream() { return format!("stream type ({})", ty.to_cpp_string()); }
    if ty.is_void_ptr() { return format!("void pointer ({})", ty.to_cpp_string()); }
    if ty.is_array() { return format!("C-style array ({})", ty.to_cpp_string()); }
    if ty.is_raw_ptr() { return format!("raw pointer ({})", ty.to_cpp_string()); }
    if ty.is_rvalue_ref() { return format!("rvalue reference ({})", ty.to_cpp_string()); }
    format!("unresolved template/nested type ({})", ty.to_cpp_string())
}

/// Generate a best-guess stub Rust declaration for a skipped method.
fn generate_method_stub(_class_name: &str, method: &Method) -> String {
    let self_param = if method.is_const { "&self" } else { "&mut self" };
    let params: Vec<String> = std::iter::once(self_param.to_string())
        .chain(method.params.iter().map(|p| format!("{}: {}", safe_param_name(&p.name), p.ty.to_rust_type_string_safe())))
        .collect();
    let ret = method.return_type.as_ref()
        .map(|ty| format!(" -> {}", stub_return_type_string(ty)))
        .unwrap_or_default();
    format!("pub fn {}({}){};", safe_method_name(&method.name), params.join(", "), ret)
}

/// Generate a best-guess stub Rust declaration for a skipped constructor.
fn generate_ctor_stub(_class_name: &str, ctor: &Constructor) -> String {
```
</details>

> 💭 So the catch-all at line 809 is "unresolved template/nested type" - it's the fallback when a type is unbindable but doesn't match any of the specific categories (stream, void pointer, array, raw pointer, rvalue ref). Now that nested types are solved, what remains in this catch-all are all template instantiations and a few other unresolved types like `LDOMString` and `LDOMBasicString`.
> 
> Let me check if `LDOMString` is nested or template. It's actually neither - it's from the LDOM XML parser in OCCT. It's just a class that isn't in the binding set. 
> 
> The `is_unbindable()` function is what's catching it, so I need to understand what criteria determine whether a type gets marked as unbindable.

Searched for text `fn is_unbindable` (`**/crates/opencascade-binding-generator/src/**`), 1 result
- [model.rs:640](../crates/opencascade-binding-generator/src/model.rs#L640)

<details>
<summary>Reading lines 640-680 of <a href="../crates/opencascade-binding-generator/src/model.rs#L640-L681">model.rs</a></summary>

```
    pub fn is_unbindable(&self) -> bool {
        self.is_stream() || self.is_void_ptr() || self.is_array() || self.is_raw_ptr() || self.is_rvalue_ref() || self.is_unresolved_template_type()
    }

    /// Get a human-readable C++-like type string for diagnostic messages.
    pub fn to_cpp_string(&self) -> String {
        match self {
            Type::Void => "void".to_string(),
            Type::Bool => "bool".to_string(),
            Type::I32 => "int".to_string(),
            Type::U32 => "unsigned int".to_string(),
            Type::U16 => "uint16_t".to_string(),
            Type::I64 => "long long".to_string(),
            Type::U64 => "unsigned long long".to_string(),
            Type::Long => "long".to_string(),
            Type::ULong => "unsigned long".to_string(),
            Type::Usize => "size_t".to_string(),
            Type::F32 => "float".to_string(),
            Type::F64 => "double".to_string(),
            Type::ConstRef(inner) => format!("const {}&", inner.to_cpp_string()),
            Type::MutRef(inner) => format!("{}&", inner.to_cpp_string()),
            Type::RValueRef(inner) => format!("{}&&", inner.to_cpp_string()),
            Type::ConstPtr(inner) => format!("const {}*", inner.to_cpp_string()),
            Type::MutPtr(inner) => format!("{}*", inner.to_cpp_string()),
            Type::Handle(name) => format!("Handle({})", name),
            Type::Class(name) => name.clone(),
        }
    }

    /// Check if this type is an unresolved template or bare nested type that can't be
    /// represented in Rust FFI. Qualified nested types (`Parent::Nested` where parent
    /// follows OCCT naming) ARE representable.
    fn is_unresolved_template_type(&self) -> bool {
        match self {
            Type::Class(name) => {
                // Template types with angle brackets are not representable
                if name.contains('<') || name.contains('>') {
                    return true;
                }
                // Qualified nested types (Parent::Nested) are representable if
                // the parent follows OCCT naming (contains '_')
```
</details>

<details>
<summary>Reading lines 680-710 of <a href="../crates/opencascade-binding-generator/src/model.rs#L680-L711">model.rs</a></summary>

```
                // the parent follows OCCT naming (contains '_')
                if name.contains("::") {
                    return false;
                }
                // Types without underscore that aren't primitives are likely
                // unqualified nested types (e.g., StreamBuffer from
                // Message_Messenger::StreamBuffer resolved by clang to bare name)
                if !name.contains('_') {
                    if matches!(name.as_str(), "bool" | "char" | "int" | "unsigned" | "float" | "double" | "void" | "size_t") {
                        return false;
                    }
                    return true;
                }
                false
            }
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_unresolved_template_type()
            }
            _ => false,
        }
    }

    /// Convert this type to a Rust type string for use in method signatures
    pub fn to_rust_type_string(&self) -> String {
        match self {
            Type::Void => "()".to_string(),
            Type::Bool => "bool".to_string(),
            Type::I32 => "i32".to_string(),
            Type::U32 => "u32".to_string(),
            Type::U16 => "u16".to_string(),
            Type::I64 => "i64".to_string(),
```
</details>

> 💭 Now I understand. The `is_unresolved_template_type()` function catches:
> 1. Template types with angle brackets (e.g., `NCollection_DataMap<...>`, `std::pair<...>`)
> 2. Types without underscore that aren't primitives (e.g., `LDOMString`, `LDOMBasicString`) - these are unqualified nested or external types that clang resolves to bare names
> 
> The function name is `is_unresolved_template_type` and the nested type part is the "types without underscore" check, which catches things like `LDOMString`, `LDOMBasicString`, `VectorOfPoint`, `DataMapOfStringInteger`, `ProxPnt_Status` (wait, that has underscore...).
> 
> 
> So I'm seeing the pattern now - types like `LDOMString`, `LDOMBasicString`, `VectorOfPoint` get flagged because they lack underscores, while template types with angle brackets like `NCollection_DataMap<>`, `std::pair<>`, `math_VectorBase<>`, and `std::shared_ptr<>` are caught by the bracket check.
> 
> But `ProxPnt_Status` itself has an underscore and no brackets, so it shouldn't be caught directly - it's actually the outer `NCollection_Vector<ProxPnt_Status>` wrapper with the angle brackets that triggers the error. The issue is the template container, not the inner type.
> 
> So there are really two categories of unresolved types here: template instantiations like `NCollection_Vector<...>` and `std::shared_ptr<...>`, and bare unqualified names without underscores that are probably C++ typedefs that lost their module qualification like `LDOMString` and `DataMapOfStringInteger`. The nested type issue mentioned in the future work section has already been addressed for `Parent::Nested` style types, but these bare unqualified names are still problematic. I should update the README to be more precise about what "unresolved template/nested type" actually means, since the nested types in the OCCT sense are now being handled.
> 
> Looking at the `is_unresolved_template_type()` function, it catches both cases: the `contains('<')` check handles templates, and the `!contains('_')` check catches those bare names without module prefixes. These bare names are genuinely unresolved types—either they're from external libraries using non-standard naming conventions, or their module context has been lost somehow. I need to update both the code comment and the README to reflect this more accurately. template args got resolved by clang into bare names, so the category should be "**Unresolved template type**" instead of "**Unresolved template/nested type**" since nested types are now handled. Looking at the user's request to fix line 369, they're pointing at the README text itself. The simplest approach is just updating the documentation to reflect that nested types are resolved, rather than regenerating bindings which would be time-consuming. template/nested type" = type is `is_unbindable()` due to template args, bare names, etc.
> 
> These are actually different code paths. The "unknown type" is checked in the resolver when the type isn't in the symbol table, while "unresolved template type" is checked via `is_unbindable()` in the type system.
> 
> I think the best fix is renaming to "**Unresolved template type**" with a description covering template instantiations and unqualified types that can't be resolved, then updating both the code and READMEs. Let me start with the code changes, regenerate, then update the documentation and commit.
> 
> Actually, let me think through this more carefully. The `is_unresolved_template_type` function checks for types with angle brackets (templates) and bare names without underscores that aren't primitives. That second case covers things like `LDOMString` from OCCT's LDOM module that don't follow the standard naming convention, plus typedefs like `VectorOfPoint` that lost their context. The old name was more fitting when nested types weren't properly handled, but now that they are, "unresolved template type" is more accurate since nested types like `Parent::Nested` are handled separately.
> 
> Looking at `LDOMString` specifically, it's a top-level class from the LDOM module that just doesn't follow the `Module_Name` convention, so the underscore heuristic is catching it incorrectly. For the README fix, I need to update the function name to reflect what it actually does, update the documentation table, regenerate the bindings, and commit the changes.

**Todo list:**
- ✅ Get actual skip counts from generated code
- 🔄 Update code and READMEs
- ⬜ Regenerate bindings
- ⬜ Commit the fix

The category says "Unresolved template/nested type" but nested types are now solved (documented as "(SOLVED)" in the Future Work section). The remaining 79 skips are all template instantiations (`NCollection_DataMap<...>`, `std::pair<...>`, etc.) and bare unqualified types (`LDOMString`). Let me fix the code and both READMEs.

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L809-L810">bindings.rs -1+1</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L669-L673">model.rs -4+4</a>  
Edited: <a href="../crates/opencascade-binding-generator/README.md#L369-L370">README.md -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "809:dh",
      "content": "    format!(\"unresolved template type ({})\", ty.to_cpp_string())"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "669:uy,670:ld,671:oy,672:os",
      "content": "    /// Check if this type is an unresolved template or bare unqualified type that can't be\n    /// represented in Rust FFI. Qualified nested types (`Parent::Nested` where parent\n    /// follows OCCT naming) ARE representable.\n    fn is_unresolved_template_type(&self) -> bool {"
    },
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "369:uh",
      "content": "| 79 | 5.0% | **Unresolved template type** | Template instantiations or unqualified types that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `LDOMString`, etc.) |"
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
      "startLine": 809,
      "endLine": 809
    },
    {
      "file": "crates/opencascade-binding-generator/src/model.rs",
      "startLine": 669,
      "endLine": 672
    },
    {
      "file": "crates/opencascade-binding-generator/README.md",
      "startLine": 369,
      "endLine": 369
    }
  ]
}
```
</details>

Now regenerate bindings and build:

**Todo list:**
- ✅ Get actual skip counts from generated code
- ✅ Update code and READMEs
- 🔄 Regenerate bindings and build
- ⬜ Commit the fix

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && tail -5 /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_01M6JNuMZuDi2pLs73q2N76U__vscode-1771312785143/content.txt
```
<details>
<summary>Output (5 lines)</summary>

```
Code generation complete!
  263 modules generated

Bindings generated in /Users/sethml/src/opencascade-rs/crates/opencascade-sys/ge
nerated                                                                         real  50.7s
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (37 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.79s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-3404803fe4cd90de)

running 16 tests
test header_deps::tests::test_is_likely_occt_header ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test module_graph::tests::test_extract_rust_type_name ... ok
test parser::tests::test_extract_module_from_header ... ok
test parser::tests::test_map_standard_type ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
test parser::tests::test_extract_template_arg ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok
test resolver::tests::test_safe_method_name ... ok
test resolver::tests::test_safe_param_name ... ok
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

real  4.8s
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo test --workspace --exclude opencascade-binding-generator 2>&1
```

**Terminal:**
```sh
grep -c "test result: ok" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_01AcsbM7kjVaJtC1BEL9gi8R__vscode-1771312785147/content.txt && grep "FAILED" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_01AcsbM7kjVaJtC1BEL9gi8R__vscode-1771312785147/content.txt || echo "No failures"
```
```
17
No failures
```

All 17 test suites pass, no failures. Now let me commit.

**Todo list:**
- ✅ Get actual skip counts from generated code
- ✅ Update code and READMEs
- ✅ Regenerate bindings and build
- 🔄 Commit the fix

Created [commit-msg.txt](../tmp/commit-msg.txt)

**Terminal:**
```sh
cat > /Users/sethml/src/opencascade-rs/tmp/commit-msg.txt << 'ENDOFMSG'
Rename "unresolved template/nested type" skip reason to "unresolved template type"

Nested C++ types (Parent::Nested) were solved earlier but the skip reason
category still mentioned them. The remaining 79 skips in this category are
all template instantiations (NCollection_DataMap<...>, std::pair<...>, etc.)
and bare unqualified type names (LDOMString, LDOMBasicString). Update the
reason string in describe_unbindable_reason(), the doc comment on
is_unresolved_template_type(), and the README skip table to reflect this.
ENDOFMSG
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git add -A && git commit -F tmp/commit-msg.txt
```
```
[main 6e9ef9f] Rename "unresolved template/nested type" skip reason to "unresolved template type"
 24 files changed, 82 insertions(+), 82 deletions(-)
```

**Todo list:**
- ✅ Get actual skip counts from generated code
- ✅ Update code and READMEs
- ✅ Regenerate bindings and build
- ✅ Commit the fix

Done. Renamed "unresolved template/nested type" → "unresolved template type" in three places:
- `describe_unbindable_reason()` in `crates/opencascade-binding-generator/src/codegen/bindings.rs#L809`
- Doc comment on `is_unresolved_template_type()` in `crates/opencascade-binding-generator/src/model.rs#L669`
- Skip reason table in the `crates/opencascade-binding-generator/README.md#L369`

All 79 skips in this category are template instantiations (`NCollection_DataMap<...>`, `std::pair<...>`) and bare unqualified types (`LDOMString`), not nested types — those were already solved. Regenerated bindings updated 24 files (82 reason strings). All tests pass.

*2026-02-19 16:38 · 391s · Words: 2,962 in · 2,361 out · 46,976 ctx · 35 rounds*

---

<a id="user-2"></a>

## User (2)

> OK, investigate the template instantiations and bare unqualified types that are causing skips, speculate about whether they&#x27;re blocking useful APIs, and if so unblock them and commit.

*2026-02-19 16:48*

### Assistant — Claude Opus 4.6

> 💭 Let me investigate the 79 "unresolved template type" skips to understand what they are, which APIs they're blocking, and whether we can unblock them.

**Todo list:**
- 🔄 Investigate template type skips
- ⬜ Categorize and assess impact
- ⬜ Implement fixes if warranted
- ⬜ Build, test, and commit

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^// SKIPPED:" crates/opencascade-sys/generated/*.rs | grep "unresolved template type" | sed 's|.*generated/||' | sort
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -rn "unresolved template type" crates/opencascade-sys/generated/*.rs | head -5
```
<details>
<summary>Output (5 lines)</summary>

```
crates/opencascade-sys/generated/approx_int.rs:70://   Reason: has unbindable types: param 'theKnots': unresolved template type (NCollection_Vector<Standard_Integer>&)
crates/opencascade-sys/generated/approx_int.rs:75://   Reason: has unbindable types: param 'theCoords': unresolved template type (const NCollection_LocalArray<Standard_Real>&)
crates/opencascade-sys/generated/b_rep_extrema.rs:1257://   Reason: has unbindable types: param 'theAddStatus1': unresolved template type (const NCollection_Vector<ProxPnt_Status>&)
crates/opencascade-sys/generated/b_rep_extrema.rs:1267://   Reason: has unbindable types: param 'theAddStatus1': unresolved template type (const NCollection_Vector<ProxPnt_Status>&)
crates/opencascade-sys/generated/b_rep_mesh.rs:831://   Reason: has unbindable types: param 'theWire': unresolved template type (const NCollection_Sequence<const gp_Pnt2d *>&); param 'theTolUV': unresolved template type (const std::pair<Standard_Real, Standard_Real>&); param 'theRangeU': unresolved template type (const std::pair<Standard_Real, Standard_Real>&); param 'theRangeV': unresolved template type (const std::pair<Standard_Real, Standard_Real>&)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -rn "unresolved template type" crates/opencascade-sys/generated/*.rs | sed 's/.*generated\///' | sed 's/\.rs:.*//' | sort | uniq -c | sort -rn
```
<details>
<summary>Output (21 lines)</summary>

```
  28 ldom
  10 b_rep_mesh
   6 math
   5 step_visual
   4 osd
   4 hlr_algo
   2 xcaf_doc
   2 stepcaf_control
   2 int_patch
   2 if_select
   2 bnd
   2 b_rep_extrema
   2 approx_int
   1 vrml_data
   1 standard
   1 rw_gltf
   1 poly
   1 moni_tool
   1 message
   1 ldom_basic_string
   1 int_ana
```
</details>

LDOM module has 28 skips — that's the biggest. Let me get the full details for all of these:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -B2 "unresolved template type" crates/opencascade-sys/generated/*.rs | grep "SKIPPED:" | sed 's/.*SKIPPED: \*\*Source:\*\* //' | sort
```
<details>
<summary>Output (62 lines)</summary>

```
`ApproxInt_KnotTools.hxx`:75 - `ApproxInt_KnotTools::BuildCurvature`
`BRepExtrema_ProximityDistTool.hxx`:114 - `BRepExtrema_ProximityDistTool::LoadAdditionalPointsFirstSet`
`BRepExtrema_ProximityDistTool.hxx`:95 - `BRepExtrema_ProximityDistTool::BRepExtrema_ProximityDistTool`
`BRepMesh_DefaultRangeSplitter.hxx`:88 - `BRepMesh_DefaultRangeSplitter::GetRangeU`
`BRepMesh_DefaultRangeSplitter.hxx`:91 - `BRepMesh_DefaultRangeSplitter::GetRangeV`
`BRepMesh_DefaultRangeSplitter.hxx`:94 - `BRepMesh_DefaultRangeSplitter::GetDelta`
`BRepMesh_DefaultRangeSplitter.hxx`:96 - `BRepMesh_DefaultRangeSplitter::GetToleranceUV`
`BRepMesh_GeomTool.hxx`:206 - `BRepMesh_GeomTool::CellsCount`
`BRepMesh_Triangulator.hxx`:38 - `BRepMesh_Triangulator::ToPolyTriangulation`
`BRepMesh_Triangulator.hxx`:44 - `BRepMesh_Triangulator::BRepMesh_Triangulator`
`BRepMesh_Triangulator.hxx`:49 - `BRepMesh_Triangulator::Perform`
`HLRAlgo_PolyAlgo.hxx`:38 - `HLRAlgo_PolyAlgo::PolyShell`
`HLRAlgo_PolyAlgo.hxx`:40 - `HLRAlgo_PolyAlgo::ChangePolyShell`
`HLRAlgo_PolyShellData.hxx`:46 - `HLRAlgo_PolyShellData::PolyData`
`HLRAlgo_PolyShellData.hxx`:48 - `HLRAlgo_PolyShellData::HidingPolyData`
`IFSelect_SignatureList.hxx`:78 - `IFSelect_SignatureList::Init`
`IntAna_Quadric.hxx`:95 - `IntAna_Quadric::SpecialPoints`
`IntPatch_Intersection.hxx`:194 - `IntPatch_Intersection::PrepareSurfaces`
`LDOM_Attr.hxx`:56 - `LDOM_Attr::getName`
`LDOM_Attr.hxx`:58 - `LDOM_Attr::getValue`
`LDOM_Attr.hxx`:60 - `LDOM_Attr::setValue`
`LDOM_BasicAttribute.hxx`:46 - `LDOM_BasicAttribute::GetValue`
`LDOM_BasicAttribute.hxx`:48 - `LDOM_BasicAttribute::SetValue`
`LDOM_BasicElement.hxx`:66 - `LDOM_BasicElement::GetAttribute`
`LDOM_BasicText.hxx`:43 - `LDOM_BasicText::GetData`
`LDOM_BasicText.hxx`:45 - `LDOM_BasicText::SetData`
`LDOM_CharacterData.hxx`:53 - `LDOM_CharacterData::getData`
`LDOM_CharacterData.hxx`:57 - `LDOM_CharacterData::setData`
`LDOM_Document.hxx`:43 - `LDOM_Document::createDocument`
`LDOM_Document.hxx`:46 - `LDOM_Document::createElement`
`LDOM_Document.hxx`:52 - `LDOM_Document::createCDATASection`
`LDOM_Document.hxx`:54 - `LDOM_Document::createComment`
`LDOM_Document.hxx`:56 - `LDOM_Document::createTextNode`
`LDOM_Document.hxx`:62 - `LDOM_Document::getElementsByTagName`
`LDOM_Element.hxx`:57 - `LDOM_Element::getTagName`
`LDOM_Element.hxx`:59 - `LDOM_Element::getAttribute`
`LDOM_Element.hxx`:61 - `LDOM_Element::getAttributeNode`
`LDOM_Element.hxx`:63 - `LDOM_Element::getElementsByTagName`
`LDOM_Element.hxx`:65 - `LDOM_Element::setAttribute`
`LDOM_Element.hxx`:69 - `LDOM_Element::removeAttribute`
`LDOM_Element.hxx`:72 - `LDOM_Element::GetChildByTagName`
`LDOM_LDOMImplementation.hxx`:31 - `LDOM_LDOMImplementation::createDocument`
`LDOM_MemManager.hxx`:52 - `LDOM_MemManager::HashedAllocate`
`LDOM_Node.hxx`:84 - `LDOM_Node::getNodeName`
`LDOM_Node.hxx`:86 - `LDOM_Node::getNodeValue`
`LDOM_XmlReader.hxx`:69 - `LDOM_XmlReader::getInteger`
`LDOMBasicString.hxx`:63
... (truncated, 4145 chars total)
```
</details>

Now let me understand the specific types causing the blocks:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep "unresolved template type" crates/opencascade-sys/generated/*.rs | sed 's/.*unresolved template type (//' | sed 's/).*//' | sort | uniq -c | sort -rn
```
<details>
<summary>Output (37 lines)</summary>

```
  14 const LDOMString&
   7 LDOMString
   6 const LDOMBasicString&
   5 const std::pair<Standard_Real, Standard_Real>&
   4 std::shared_ptr<std::streambuf>
   3 math_VectorBase<>
   3 const math_VectorBase<>&
   2 NCollection_IndexedDataMap<TDF_Label, TopoDS_Shape>&
   2 NCollection_Array1<opencascade::handle<HLRAlgo_PolyData>>&
   2 LDOMBasicString&
   2 const NCollection_Vector<ProxPnt_Status>&
   2 const NCollection_List<gp_Pnt>&
   2 const NCollection_Handle<StepVisual_Array1OfTessellatedItem>&
   2 const NCollection_DataMap<TCollection_AsciiString, opencascade::handle<STEPCAFControl_ExternFile>>&
   1 std::pair<Standard_Real, Standard_Real>
   1 std::pair<Standard_Integer, Standard_Integer>
   1 NCollection_Vector<Standard_Integer>&
   1 NCollection_Vector<opencascade::handle<Adaptor3d_Surface>>&
   1 NCollection_List<Poly_Triangle>&
   1 NCollection_List<const char *>::Iterator
   1 NCollection_List<Bnd_Range>&
   1 NCollection_IndexedDataMap<TCollection_AsciiString, Standard_DumpValue>&
   1 NCollection_Handle<StepVisual_VectorOfHSequenceOfInteger>
   1 NCollection_Handle<StepVisual_Array1OfTessellatedItem>
   1 NCollection_Array1<opencascade::handle<HLRAlgo_PolyShellData>>&
   1 NCollection_Array1<gp_Vec3f>&
   1 DataMapOfStringInteger&
   1 const NCollection_Vector<Standard_Integer>&
   1 const NCollection_Sequence<RWGltf_GltfPrimArrayData>&
   1 const NCollection_LocalArray<Standard_Real>&
   1 const NCollection_List<TColStd_SequenceOfInteger>&
   1 const NCollection_List<Poly_Triangle>&
   1 const NCollection_IndexedMap<Message_MetricType>&
   1 const NCollection_IndexedDataMap<TCollection_AsciiString, opencascade::handle<Standard_Transient>>&
   1 const NCollection_Handle<StepVisual_VectorOfHSequenceOfInteger>&
   1 const NCollection_Array1<opencascade::handle<HLRAlgo_PolyShellData>>&
   1 BVH_Box<Standard_Real, 2>
```
</details>

**Todo list:**
- ✅ Investigate template type skips
- 🔄 Categorize and assess impact
- ⬜ Implement fixes if warranted
- ⬜ Build, test, and commit

> 💭 Let me analyze what I've found. The 79 skips break into these categories:
> 
> 1. **LDOMString / LDOMBasicString** (28 skips in LDOM module + 1 in ldom_basic_string) — These are bare unqualified type names. `LDOMString` and `LDOMBasicString` are likely real OCCT types that should be in the binding set. Let me check.
> 
> 2. **std::pair** (7 skips) — Template instantiation; used for ranges in BRepMesh and IntPatch.
> 
> 3. **std::shared_ptr<std::streambuf>** (4 skips) — OSD filesystem; essentially stream-like.
> 
> 
> 4. **math_VectorBase<>** (6 skips) — Template base class; `math_Vector` is already bound but not this base. The NCollection template instantiations account for roughly 25 more skips—these are inline template classes that aren't typedef'd in the headers. Then there are a few scattered internal types like DataMapOfStringInteger, BVH_Box, and ProxPnt_Status. The main puzzle is LDOMString with 28 skips—it's part of the XML DOM module for XCAF, so I need to verify whether it's an actual exportable type.

Let me dig into the two biggest blockers — `LDOMString` and `LDOMBasicString`:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && find crates/occt-sys/OCCT/src -name "LDOMString*" -o -name "LDOM_String*" | sort
```
```
crates/occt-sys/OCCT/src/LDOM/LDOMString.cxx
crates/occt-sys/OCCT/src/LDOM/LDOMString.hxx
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cat crates/occt-sys/OCCT/src/LDOM/LDOMString.hxx
```
<details>
<summary>Output (109 lines)</summary>

```
// Created on: 2001-06-25
// Created by: Alexander GRIGORIEV
// Copyright (c) 2001-2014 OPEN CASCADE SAS
//
// This file is part of Open CASCADE Technology software library.
//
// This library is free software; you can redistribute it and/or modify it under
// the terms of the GNU Lesser General Public License version 2.1 as published
// by the Free Software Foundation, with special exception defined in the file
// OCCT_LGPL_EXCEPTION.txt. Consult the file LICENSE_LGPL_21.txt included in OCCT
// distribution for complete text of the license and disclaimer of any warranty.
//
// Alternatively, this file may be used under the terms of Open CASCADE
// commercial license or contractual agreement.

#ifndef LDOMString_HeaderFile
#define LDOMString_HeaderFile

#include <LDOMBasicString.hxx>

class LDOM_MemManager;

//  Class LDOMString
//  Represents various object types which can be mapped to XML strings
//  LDOMString is not an independent type: you must be sure that the owner
//  LDOM_Document is never lost during the lifetime of its LDOMStrings - for
//  that it is necessary to keep at least one LDOM_Document or LDOM_Node alive
//  before all LDOMString's (LDOM_AsciiDoc type) are destroyed.

class LDOMString : public LDOMBasicString
{
public:
  // ---------- PUBLIC METHODS ----------

  LDOMString()
      : myPtrDoc(NULL)
  {
  }

  //    Empty constructor

  LDOMString(const LDOMString& anOther)
      : LDOMBasicString(anOther),
        myPtrDoc(anOther.myPtrDoc)
  {
  }

  //    Copy constructor

  LDOMString(const Standard_Integer aValue)
      : LDOMBasicString(aValue),
        myPtrDoc(NULL)
  {
  }

  //    Integer => LDOMString

  //  Standard_EXPORT LDOMString (const Standard_Real aValue);

  LDOMString(const char* aValue)
      : LDOMBasicString(aValue),
        myPtrDoc(NULL)
  {
  }

  //    Create LDOM_AsciiFree

  const LDOM_MemManager& getOwnerDocument() const { return *myPtrDoc; }

  LDOMString& operator=(const LDOM_NullPtr* aNull)
  {
    LDOMBasicString::operator=(aNull);
    return *this;
  }

  LDOMString& operator=(const LDOMString& anOther)
  {
    myPtrDoc = anOther.myPtrDoc;
    LDOMBasicString::operator=(anOther);
    return *this;
  }

private:
  friend class LDOM_Document;
  friend class LDOM_Node;
  friend class LDOM_Element;
  friend class LDOM_BasicElement;
  friend class LDOM_BasicAttribute;
  friend class LDOM_BasicText;

  static LDOMString CreateDirectString(const char* aValue, const LDOM_MemManager& aDoc);

  LDOMString(const LDOMBasicString& anOther, const LDOM_MemManager& aDoc)
      : LDOMBasicString(anOther),
        myPtrDoc(&aDoc)
  {
  }

  //    Plain copy from LDOMBasicString

  LDOMString(const LDOMBasicString& anOther, const Handle(LDOM_MemManager)& aDoc);
  //    Copy from another string with allocation in the document space

private:
  // ---------- PRIVATE FIELDS -------------
  const LDOM_MemManage
... (truncated, 3026 chars total)
```
</details>

> 💭 `LDOMString` is a class that inherits from `LDOMBasicString`. It's defined in a header called `LDOMString.hxx`. The key thing is that this type doesn't follow OCCT naming conventions — it doesn't have an underscore (`_`) separating module and class name. It's just `LDOMString`, not `LDOM_String`.
> 
> The current `is_unresolved_template_type()` function in `model.rs` has this logic:
> 
> ```rust
> // Types without underscore that aren't primitives are likely
> // unqualified nested types
> if !name.contains('_') {
>     if matches!(name.as_str(), "bool" | "char" | "int" | ...) {
>         return false;
>     }
>     return true;
> }
> ```
> 
> So `LDOMString` and `LDOMBasicString` are being flagged as "unresolved" because they don't contain an underscore and aren't primitives. But they're actually legitimate OCCT classes — they just don't follow the `Module_Class` naming convention. I need to look at other types without underscores like `DataMapOfStringInteger` to understand the full scope of this issue.
> 
> The real solution is to check if a type actually exists in the symbol table rather than relying on naming conventions. The problem is that `is_unresolved_template_type()` lives in the parsed IR and doesn't have access to the symbol table, while the actual filtering in `is_method_bindable()` does. Let me trace how these methods interact to see where the fix should go. I'm checking whether `LDOMString` and `LDOMBasicString` are actually in the binding set, since the LDOM module already has 28 skips for other types like `LDOM_Element` and `LDOM_Node`. The real problem is that `LDOMString` doesn't follow the standard OCCT naming convention with an underscore, so the generator incorrectly flags it as an unresolved type even though it's a legitimate class with a header file.
> 
> The challenge is distinguishing between real top-level OCCT classes that happen to lack underscores versus bare nested types that clang resolved to unqualified names. I could whitelist known non-underscore classes, but a cleaner approach would be to make the template and nested type checking context-aware by passing the symbol table into the validation logic in `bindings.rs` rather than keeping it as a pure type predicate.
> 
> Looking at the actual types being flagged as unresolved template types — `LDOMString`, `LDOMBasicString`, and `ProxPnt_Status` — I should refactor the check to keep the template syntax detection but remove the assumption that missing underscores means unresolved, then move the symbol lookup into the resolver layer where I have access to the symbol table. , this HAS an underscore. Let me re-check.
> - `DataMapOfStringInteger` (1 skip)
> 
> Let me look at `ProxPnt_Status`:
> ```
> const NCollection_Vector<ProxPnt_Status>&
> ```
> This is caught because it's inside a template instantiation (contains `<`).
> 
> And `DataMapOfStringInteger` — no underscore. Probably a typedef inside a class.
> 
> So the big wins are:
> 1. `LDOMString` and `LDOMBasicString` — 29 skips, blocking most LDOM XML DOM functionality
> 2. Template instantiations like `std::pair<...>`, `NCollection_Vector<...>` — 50+ skips, mostly internal/specialized
> 
> For #1, I should check if these types are in the symbol table before marking them as unresolved, which would unblock 29 methods in the LDOM module. For #2, the template instantiations aren't really representable in the current FFI without generating specific instantiations or typedefs, which is a bigger lift. Let me verify whether the LDOM headers are being parsed and if those string types are actually in the symbol table.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -i "ldom" crates/opencascade-sys/bindings.toml
```
```
(base) whitelily:opencascade-rs sethml$
```
**Exit code:** 1

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -c "LDOMString\|LDOMBasicString" crates/opencascade-sys/generated/ffi.rs && grep "pub type LDOM\|LDOMBasicString\|LDOMString" crates/opencascade-sys/generated/ffi.rs | head -10
```
<details>
<summary>Output (11 lines)</summary>

```
38
pub struct LDOMBasicString {
pub struct LDOMString {
pub struct LDOMBasicString_StringType {
    // ======================== LDOMBasicString ========================
    pub fn LDOMBasicString_destructor(self_: *mut LDOMBasicString);
    /// **Source:** `LDOMBasicString.hxx`:89 - `LDOMBasicString::LDOMBasicString()`
    pub fn LDOMBasicString_ctor() -> *mut LDOMBasicString;
    /// **Source:** `LDOMBasicString.hxx`:101 - `LDOMBasicString::LDOMBasicString()`
    pub fn LDOMBasicString_ctor_int(aValue: i32) -> *mut LDOMBasicString;
    /// **Source:** `LDOMBasicString.hxx`:107 - `LDOMBasicString::LDOMBasicString()`
```
</details>

So `LDOMString` and `LDOMBasicString` ARE already parsed and present in the FFI as opaque structs. The problem is that `is_unresolved_template_type()` incorrectly flags them because they lack underscores.

Let me look at how the type resolution works:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n "is_unresolved_template_type\|is_unbindable" crates/opencascade-binding-generator/src/codegen/bindings.rs | head -20
```
<details>
<summary>Output (10 lines)</summary>

```
763:        if p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none() {
768:        if ret.is_unbindable() && ret.class_ptr_inner_name().is_none() {
779:        if p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none() {
790:        if p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none() {
795:        if ret.is_unbindable() {
2162:    if params.iter().any(|p| p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none()) {
3125:    if ty.is_unbindable() {
3267:            p.ty.original.is_unbindable() || type_uses_unknown_type(&p.ty.original, &ffi_ctx)
3270:            let reason = if p.ty.original.is_unbindable() {
3288:            if ret.original.is_unbindable() {
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n "fn type_uses_unknown_type\|fn is_type_known\|fn.*unknown.*type" crates/opencascade-binding-generator/src/codegen/bindings.rs
```
```
498:fn type_uses_unknown_type(ty: &Type, ctx: &TypeContext) -> bool {
```

<details>
<summary>Reading lines 490-540 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L490-L541">bindings.rs</a></summary>

```
            param_uses_unknown_handle(inner, handle_able_classes)
        }
        _ => false,
    }
}

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
```
</details>

Searched for text `fn type_uses_unknown_class` (`**/crates/opencascade-binding-generator/src/**`), 1 result
- [type_mapping.rs:318](../crates/opencascade-binding-generator/src/type_mapping.rs#L318)

<details>
<summary>Reading lines 315-365 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L315-L366">type_mapping.rs</a></summary>

```

/// Check if a type references an unknown class/handle
/// Returns true if the type uses a Handle or Class that is not in all_classes
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
```
</details>

> 💭 Now I understand the full picture. There are TWO separate checks:
> 
> 1. `is_unresolved_template_type()` in `model.rs` — a structural check that says "this type LOOKS like it can't be represented" (templates with `<>`, or bare names without `_`)
> 2. `type_uses_unknown_class()` in `type_mapping.rs` — a semantic check that says "this type isn't in our binding set"
> 
> These are used in different places:
> - `is_unbindable()` (which includes `is_unresolved_template_type()`) is checked in `is_method_bindable()` etc.
> 
> 
> - `type_uses_unknown_class()` is checked separately in function binding computation
> 
> The issue with `LDOMString` is that the "no underscore means unresolved" heuristic in `is_unresolved_template_type()` catches it first, and since `is_unbindable()` is evaluated before the semantic check, it gets rejected even though `type_uses_unknown_class()` would correctly recognize it as a known class.
> 
> The cleanest fix is to remove the "no underscore" heuristic entirely and let `type_uses_unknown_class()` handle detection of truly unknown types — it should already catch bare names that aren't in `all_classes`. But there's a catch: if a bare name is already in `all_classes` because it was parsed as a top-level type, then `type_uses_unknown_class()` won't flag it as problematic.
> 
> Let me trace through the actual cases to verify this works. For something like `StreamBuffer` from a nested class, if the parser resolves it to the flattened name `Message_Messenger_StreamBuffer`, then the bare name `StreamBuffer` won't be in `all_classes` and will correctly be flagged as unknown. The real question is whether removing the underscore check causes any bare names to incorrectly slip through as "known" when they shouldn't be. unresolved_template_type` is checking and what paths it's used in.
> 
> `is_unbindable()` includes `is_unresolved_template_type()`. This is called in `is_method_bindable()` in bindings.rs. If a type is NOT `is_unbindable()`, it still gets checked via `type_uses_unknown_class()` or `type_uses_unknown_handle()` in other code paths.
> 
> So the question is: if I remove the "no underscore" heuristic from `is_unresolved_template_type()`, will `LDOMString` then pass the `is_unbindable()` check, and then get properly handled (since it IS in `all_classes`)?
> 
> Yes! That should work. Let me check what other non-underscore names exist in the codebase that might be affected.
> 
> The no-underscore heuristic was originally added because clang sometimes resolves nested type names to their bare form — for instance, if `LDOM_Element` has a method taking `StreamBuffer` that's actually `Message_Messenger::StreamBuffer`, clang might just parse it as `StreamBuffer`. Since that bare name wouldn't be in `all_classes` (it'd be stored as `Message_Messenger_StreamBuffer`), it would get flagged as unknown and skipped anyway. So removing the heuristic would just change the skip reason for those cases, not the actual behavior, while correctly unblocking `LDOMString` and similar types.
> 
> Now I need to check what `DataMapOfStringInteger` is — that's another non-underscore type being caught by this heuristic.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && find crates/occt-sys/OCCT/src -name "DataMapOfStringInteger*" 2>/dev/null; grep -r "DataMapOfStringInteger" crates/occt-sys/OCCT/src/ 2>/dev/null | head -5
```
<details>
<summary>Output (5 lines)</summary>

```
crates/occt-sys/OCCT/src/CDM/CDM_NamesDirectory.hxx:#include <TColStd_DataMapOfStringInteger.hxx>
crates/occt-sys/OCCT/src/CDM/CDM_NamesDirectory.hxx:typedef TColStd_DataMapOfStringInteger CDM_NamesDirectory;
crates/occt-sys/OCCT/src/Express/Express_Entity.hxx:  typedef NCollection_DataMap<TCollection_AsciiString, Standard_Integer> DataMapOfStringInteger;
crates/occt-sys/OCCT/src/Express/Express_Entity.cxx:  DataMapOfStringInteger aDict;
crates/occt-sys/OCCT/src/StdLPersistent/StdLPersistent_NamedData.cxx:#include <TColStd_DataMapOfStringInteger.hxx>
```
</details>

`DataMapOfStringInteger` is a **nested typedef** inside `Express_Entity` — not a standalone type. This is a genuine unresolved nested type that should stay skipped.

Now let me understand the full picture. The fix is to remove the "no underscore" heuristic from `is_unresolved_template_type()` and instead let the  check in the binding logic handle those cases. `LDOMString`/`LDOMBasicString` ARE in `all_classes`, so they'll pass. Bare nested types like `DataMapOfStringInteger` are NOT in `all_classes`, so they'll get caught by  instead.

Let me verify what non-underscore types are actually in the generated FFI:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep "^pub struct " crates/opencascade-sys/generated/ffi.rs | sed 's/pub struct //' | sed 's/ {//' | grep -v '_' | sort
```
<details>
<summary>Output (1029 lines)</summary>

```
HandleStepDimTolCommonDatum
HandleStepDimTolConcentricityTolerance
HandleStepDimTolCylindricityTolerance
HandleStepDimTolDatum
HandleStepDimTolDatumFeature
HandleStepDimTolDatumReference
HandleStepDimTolDatumReferenceCompartment
HandleStepDimTolDatumReferenceElement
HandleStepDimTolDatumReferenceModifierWithValue
HandleStepDimTolDatumSystem
HandleStepDimTolDatumTarget
HandleStepDimTolFlatnessTolerance
HandleStepDimTolGeneralDatumReference
HandleStepDimTolGeometricTolerance
HandleStepDimTolGeometricToleranceRelationship
HandleStepDimTolGeometricToleranceWithDatumReference
HandleStepDimTolGeometricToleranceWithDefinedAreaUnit
HandleStepDimTolGeometricToleranceWithDefinedUnit
HandleStepDimTolGeometricToleranceWithMaximumTolerance
HandleStepDimTolGeometricToleranceWithModifiers
HandleStepDimTolGeoTolAndGeoTolWthDatRef
HandleStepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol
HandleStepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMod
HandleStepDimTolGeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol
HandleStepDimTolGeoTolAndGeoTolWthDatRefAndUneqDisGeoTol
HandleStepDimTolGeoTolAndGeoTolWthMaxTol
HandleStepDimTolGeoTolAndGeoTolWthMod
HandleStepDimTolHArray1OfDatumReference
HandleStepDimTolHArray1OfDatumReferenceCompartment
HandleStepDimTolHArray1OfDatumReferenceElement
HandleStepDimTolHArray1OfDatumReferenceModifier
HandleStepDimTolHArray1OfDatumSystemOrReference
HandleStepDimTolHArray1OfGeometricToleranceModifier
HandleStepDimTolHArray1OfToleranceZoneTarget
HandleStepDimTolLineProfileTolerance
HandleStepDimTolModifiedGeometricTolerance
HandleStepDimTolNonUniformZoneDefinition
HandleStepDimTolParallelismTolerance
HandleStepDimTolPerpendicularityTolerance
HandleStepDimTolPlacedDatumTargetFeature
HandleStepDimTolPositionTolerance
HandleStepDimTolProjectedZoneDefinition
HandleStepDimTolRoundnessTolerance
HandleStepDimTolRunoutZoneDefinition
HandleStepDimTolRunoutZoneOrientation
HandleStepDimTolSimpleDatumReferenceModifierMember
HandleStepDimTolStraightnessTolerance
HandleStepDimTolSurfaceProfileTolerance
HandleStepDimTolSymmetryTolerance
HandleStepDimTolToleranceZone
HandleStepDimTolToleranceZoneDefinition
HandleStepDimTolToleranceZoneForm
HandleStepDimTolTotalRunoutTolerance
HandleStepDimTolUnequallyDisposedGeometricTolerance
HandleSTEPEditEditContext
HandleSTEPEditEditSDR
HandleStepElementAnalysisItemWithinRepresentation
HandleStepElementCurve3dElementDescriptor
HandleStepElementCurveElementEndReleasePacket
HandleStepElementCurveElementFreedomMember
HandleStepElementCurveElementPurposeMember
HandleStepElementCurveElementSectionDefinition
HandleStepElementCurveElementSectionDerivedDefinitions
HandleStepElementElementAspectMember
HandleStepElementElementDescriptor
HandleStepElementElementMaterial
HandleStepElementHArray1OfCurveElementEndReleasePacket
HandleStepElementHArray1OfCurveElementSectionDefinition
HandleStepElementHArray1OfHSequenceOfCurveElementPurposeMember
HandleStepElementHArray1OfH
... (truncated, 36684 chars total)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_01GwRi2ENYBAfrQK8BWNFP41__vscode-1771312785171/content.txt#1-1), lines 1 to 20

Those are all Handle types (no underscore because the `Handle` prefix + `StepDimTol...` format). Let me look for actual class types:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep "^pub struct " crates/opencascade-sys/generated/ffi.rs | sed 's/pub struct //' | sed 's/ {//' | grep -v '_' | grep -v "^Handle" | sort
```
<details>
<summary>Output (44 lines)</summary>

```
alist
Array1OfCirc2dIterator
Array1OfDir2dIterator
Array1OfDirIterator
Array1OfLin2dIterator
Array1OfPnt2dIterator
Array1OfPntIterator
Array1OfVec2dIterator
Array1OfVecIterator
Array1OfXYIterator
Array1OfXYZIterator
Array2OfCirc2dIterator
Array2OfDir2dIterator
Array2OfDirIterator
Array2OfLin2dIterator
Array2OfPnt2dIterator
Array2OfPntIterator
Array2OfVec2dIterator
Array2OfVecIterator
Array2OfXYIterator
Array2OfXYZIterator
cilist
cllist
complex
DataMapOfShapeShapeIterator
doublecomplex
FilletPoint
icilist
IndexedDataMapOfShapeListOfShapeIterator
IndexedMapOfShapeIterator
inlist
Interval
LDOMBasicString
LDOMParser
LDOMString
ListOfShapeIterator
MapOfShapeIterator
MyDirectPolynomialRoots
Namelist
olist
PeriodicInterval
PeriodicityInfo
SequenceOfShapeIterator
Vardesc
```
</details>

> 💭 So

> 💭 So there are several non-underscore class types in the FFI:
> - `LDOMBasicString`, `LDOMString`, `LDOMParser` — real OCCT types without underscore naming
> - Iterator types (`Array1OfCirc2dIterator`, etc.) — these are generated by the collection codegen
> - Fortran types (`alist`, `cilist`, `cllist`, `complex`, `doublecomplex`, `icilist`, `inlist`, `Namelist`, `olist`, `Vardesc`) — from Fortran common blocks
> - `FilletPoint`, `Interval`, `PeriodicInterval`, `PeriodicityInfo`, `MyDirectPolynomialRoots` — likely nested types that were detected and parsed
> 
> 
> The main issue is that `LDOMString` and `LDOMBasicString` are being flagged as unresolved because of the underscore heuristic. My approach is to remove that heuristic from `is_unresolved_template_type()` since `type_uses_unknown_class()` already handles unknown class detection properly, which should unblock these types.
> 
> Now I need to trace through the actual binding logic in `is_method_bindable()` to see how `is_unbindable()` determines whether a method gets skipped based on its parameters and return types.

<details>
<summary>Reading lines 750-800 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L750-L801">bindings.rs</a></summary>

```
                unreachable!()
            }
        }
        _ => type_to_rust_string(ty, reexport_ctx),
    }
}

// ── Filtering predicates ────────────────────────────────────────────────────

/// Describe which types in a method's params/return are unbindable.
fn describe_unbindable_types_method(method: &Method) -> String {
    let mut parts = Vec::new();
    for p in &method.params {
        if p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none() {
            parts.push(format!("param '{}': {}", p.name, describe_unbindable_reason(&p.ty)));
        }
    }
    if let Some(ref ret) = method.return_type {
        if ret.is_unbindable() && ret.class_ptr_inner_name().is_none() {
            parts.push(format!("return: {}", describe_unbindable_reason(ret)));
        }
    }
    if parts.is_empty() { "unknown".to_string() } else { parts.join("; ") }
}

/// Describe which types in a constructor's params are unbindable.
fn describe_unbindable_types_ctor(ctor: &Constructor) -> String {
    let mut parts = Vec::new();
    for p in &ctor.params {
        if p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none() {
            parts.push(format!("param '{}': {}", p.name, describe_unbindable_reason(&p.ty)));
        }
    }
    if parts.is_empty() { "unknown".to_string() } else { parts.join("; ") }
}

/// Describe which types in a static method's params/return are unbindable.
fn describe_unbindable_types_static(method: &StaticMethod) -> String {
    let mut parts = Vec::new();
    for p in &method.params {
        if p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none() {
            parts.push(format!("param '{}': {}", p.name, describe_unbindable_reason(&p.ty)));
        }
    }
    if let Some(ref ret) = method.return_type {
        if ret.is_unbindable() {
            parts.push(format!("return: {}", describe_unbindable_reason(ret)));
        }
    }
    if parts.is_empty() { "unknown".to_string() } else { parts.join("; ") }
}
```
</details>

Now let me find `is_method_bindable` where the actual decision is made:

Searched for text `fn is_method_bindable` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:874](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L874)

<details>
<summary>Reading lines 874-950 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L874-L951">bindings.rs</a></summary>

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
... (truncated, 4156 chars)
```
</details>

Now I can see clearly: line 875 checks `method.has_unbindable_types()` FIRST — before the `type_uses_unknown_type` check on line 895. The `has_unbindable_types()` call uses  which includes `is_unresolved_template_type()`. So `LDOMString` gets caught at line 875 and never reaches the proper unknown-type check.

The fix: remove the "no underscore" heuristic from `is_unresolved_template_type()`. Types like `LDOMString` will pass , then correctly pass `type_uses_unknown_type()` (since they're in `all_classes`). Types like `DataMapOfStringInteger` will pass  but get caught by `type_uses_unknown_type()`.

Let me also check `has_unbindable_types()`:

Searched for text `fn has_unbindable_types` (`**/crates/opencascade-binding-generator/src/model.rs`), 4 results
- [model.rs:49](../crates/opencascade-binding-generator/src/model.rs#L49)
- [model.rs:216](../crates/opencascade-binding-generator/src/model.rs#L216)
- [model.rs:246](../crates/opencascade-binding-generator/src/model.rs#L246)
- [model.rs:314](../crates/opencascade-binding-generator/src/model.rs#L314)

**Todo list:**
- ✅ Investigate template type skips
- ✅ Categorize and assess impact
- 🔄 Implement fixes
- ⬜ Build, test, and commit

Now let me implement the fix. The "no underscore" heuristic in `is_unresolved_template_type()` needs to be removed — it incorrectly catches real OCCT types like `LDOMString` and `LDOMBasicString`. The  check in `crates/opencascade-binding-generator/src/codegen/bindings.rs` already handles unknown bare names properly.

<details>
<summary>Reading lines 669-700 of <a href="../crates/opencascade-binding-generator/src/model.rs#L669-L701">model.rs</a></summary>

```
    /// Check if this type is an unresolved template or bare unqualified type that can't be
    /// represented in Rust FFI. Qualified nested types (`Parent::Nested` where parent
    /// follows OCCT naming) ARE representable.
    fn is_unresolved_template_type(&self) -> bool {
        match self {
            Type::Class(name) => {
                // Template types with angle brackets are not representable
                if name.contains('<') || name.contains('>') {
                    return true;
                }
                // Qualified nested types (Parent::Nested) are representable if
                // the parent follows OCCT naming (contains '_')
                if name.contains("::") {
                    return false;
                }
                // Types without underscore that aren't primitives are likely
                // unqualified nested types (e.g., StreamBuffer from
                // Message_Messenger::StreamBuffer resolved by clang to bare name)
                if !name.contains('_') {
                    if matches!(name.as_str(), "bool" | "char" | "int" | "unsigned" | "float" | "double" | "void" | "size_t") {
                        return false;
                    }
                    return true;
                }
                false
            }
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_unresolved_template_type()
            }
            _ => false,
        }
    }
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L669-L686">model.rs -32+17</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "669:ku,670:ld,671:oy,672:os,673:dk,674:rl,675:lk,676:lx,677:qh,678:gl,679:cq,680:ql,681:uo,682:bu,683:gl,684:uw,685:wh,686:dh,687:wc,688:ve,689:bg,690:or,691:qh,692:gl,693:uu,694:uo,695:mb,696:qg,697:uo,698:ed,699:cy,700:ax",
      "content": "    /// Check if this type is an unresolved template instantiation that can't be\n    /// represented in Rust FFI. Only catches template types with `<>`.\n    /// Non-underscore class names (e.g., `LDOMString`) are NOT caught here —\n    /// they are handled by `type_uses_unknown_class()` in the binding layer\n    /// which checks against the symbol table.\n    fn is_unresolved_template_type(&self) -> bool {\n        match self {\n            Type::Class(name) => {\n                // Template types with angle brackets are not representable\n                name.contains('<') || name.contains('>')\n            }\n            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {\n                inner.is_unresolved_template_type()\n            }\n            _ => false,\n        }\n    }"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/model.rs","startLine":669,"endLine":685}
```
</details>

Now let me build the binding generator first to make sure it compiles:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.94s
real  4.1s
```

Now regenerate bindings:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep "263 modules\|Code generation complete" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_013ug6XAGfDgtBX3FNo5dJSr__vscode-1771312785182/content.txt
```
```
Code generation complete!
  263 modules generated
```

Now let me check what happened to the skip counts:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | wc -l
```
```
    1538
```

Went from 1567 to 1538 — 29 methods unblocked! Let me verify:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "unresolved template type"
```
```
49
```

Down from 79 to 49 — 30 fewer unresolved template type skips (29 `LDOMString`/`LDOMBasicString` + 1 `DataMapOfStringInteger` which now gets caught by  with a different message). Let me verify the `DataMapOfStringInteger` is still properly skipped:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -A2 "DataMapOfStringInteger" crates/opencascade-sys/generated/*.rs
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep "DataMapOfStringInteger" crates/opencascade-sys/generated/*.rs | grep "Reason:"
```
```
(base) whitelily:opencascade-rs sethml$
```
**Exit code:** 1

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -B1 -A2 "DataMapOfStringInteger" crates/opencascade-sys/generated/moni_tool.rs
```
```
//   Reason: has string ref param 'satisname' of type 'const char*&' (needs manual binding)
//   // pub fn internals(&self, interp: &mut ValueInterpret, satisf: &mut ValueSatisfies, satisname: &mut *const char, enums: &mut DataMapOfStringInteger);
//
// SKIPPED: **Source:** `MoniTool_TypedValue.hxx`:91 - `MoniTool_TypedValue::Print`
```

`DataMapOfStringInteger` appears in a method that's already skipped for `string ref param` — it was never the primary reason. Let me check the total skip category breakdown now:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "stream type"
grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "void pointer"
grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "raw pointer"
grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "uses unknown type"
grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "ambiguous lifetimes"
grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "abstract"
grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "C-style array"
grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "string ref param"
grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "rvalue reference"
grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "Unknown Handle type\|unknown Handle"
grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "not CppDeletable\|Not CppDeletable"
grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "mut enum"
grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "Excluded by\|bindings.toml"
grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "ambiguous overload\|Ambiguous overload"
```
```
 's/.*Reason: //' | grep -grep -r "^//   Reason:" crates/opencascade-sys/ge546
```

Let me run each count individually:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' > tmp/skip-reasons.txt && wc -l tmp/skip-reasons.txt && echo "---" && grep -c "stream type" tmp/skip-reasons.txt && grep -c "void pointer" tmp/skip-reasons.txt && grep -c "raw pointer" tmp/skip-reasons.txt && grep -c "unresolved template type" tmp/skip-reasons.txt && grep -c "ambiguous lifetimes" tmp/skip-reasons.txt && grep -c "abstract" tmp/skip-reasons.txt && grep -c "C-style array" tmp/skip-reasons.txt && grep -c "string ref param" tmp/skip-reasons.txt && grep -c "rvalue reference" tmp/skip-reasons.txt && grep -c "CppDeletable" tmp/skip-reasons.txt && grep -c "mut enum" tmp/skip-reasons.txt
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs
grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' > tmp/skip-reasons.txt
wc -l tmp/skip-reasons.txt
```
```
(base) whitelily:opencascade-rs sethml$ grep -r "^//   Reason:" crates/opencasca
```

**Terminal:**
```sh
wc -l tmp/skip-reasons.txt
```
```
    1538 tmp/skip-reasons.txt
```

**Terminal:**
```sh
grep -c "stream type" tmp/skip-reasons.txt
```
```
546
```

**Terminal:**
```sh
echo "Stream: $(grep -c 'stream type' tmp/skip-reasons.txt)"
echo "Unknown: $(grep -c 'uses unknown type\|is unknown' tmp/skip-reasons.txt)"
echo "Void ptr: $(grep -c 'void pointer' tmp/skip-reasons.txt)"
echo "Raw ptr: $(grep -c 'raw pointer' tmp/skip-reasons.txt)"
echo "Template: $(grep -c 'unresolved template type' tmp/skip-reasons.txt)"
echo "Lifetimes: $(grep -c 'ambiguous lifetimes' tmp/skip-reasons.txt)"
echo "Abstract: $(grep -c 'abstract' tmp/skip-reasons.txt)"
echo "C-array: $(grep -c 'C-style array' tmp/skip-reasons.txt)"
echo "StrRef: $(grep -c 'string ref param' tmp/skip-reasons.txt)"
echo "RvalRef: $(grep -c 'rvalue reference' tmp/skip-reasons.txt)"
echo "Deletable: $(grep -c 'CppDeletable' tmp/skip-reasons.txt)"
echo "MutEnum: $(grep -c 'mut enum' tmp/skip-reasons.txt)"
echo "Excluded: $(grep -c 'bindings.toml' tmp/skip-reasons.txt)"
echo "AmbigOver: $(grep -c 'ambiguous overload\|Ambiguous overload' tmp/skip-reasons.txt)"
```
```
Stream: 546
```

**Terminal:**
```sh
grep -c 'uses unknown type' tmp/skip-reasons.txt
```
```
336
```

**Terminal:**
```sh
grep -c 'is unknown' tmp/skip-reasons.txt
```
```
115
```

**Terminal:**
```sh
grep -c 'void pointer' tmp/skip-reasons.txt
grep -c 'raw pointer' tmp/skip-reasons.txt
grep -c 'unresolved template type' tmp/skip-reasons.txt
grep -c 'ambiguous lifetimes' tmp/skip-reasons.txt
grep -c 'abstract' tmp/skip-reasons.txt
grep -c 'C-style array' tmp/skip-reasons.txt
grep -c 'string ref param' tmp/skip-reasons.txt
grep -c 'rvalue reference' tmp/skip-reasons.txt
grep -c 'CppDeletable' tmp/skip-reasons.txt
grep -c 'mut enum' tmp/skip-reasons.txt
```
```
199
```

**Terminal:**
```sh
for pat in "raw pointer" "unresolved template type" "ambiguous lifetimes" "abstract" "C-style array" "string ref param" "rvalue reference" "CppDeletable" "mut enum"; do echo "$pat: $(grep -c "$pat" tmp/skip-reasons.txt)"; done
```
<details>
<summary>Output (9 lines)</summary>

```
raw pointer: 135
unresolved template type: 49
ambiguous lifetimes: 53
abstract: 43
C-style array: 18
string ref param: 17
rvalue reference: 12
CppDeletable: 5
mut enum: 4
```
</details>

> 💭 Let me compute the new unknown type count. Before: 451 "unknown/unresolved type". Now I have 336 "uses unknown type" + some "is unknown" return types. But the "is unknown" count (115) includes both "return type X is unknown" and "is unknown" in Handle types. Let me be more precise.
> 
> Let me calculate the new unknown category. The "unknown/unresolved type" category previously included methods where params or returns use types not in the binding set. The 336 "uses unknown type" count includes the former `LDOMString` methods that are now being caught as "uses unknown type" for a different reason (maybe `DataMapOfStringInteger`?). No wait, LDOMString IS in the binding set, so those 29 methods should now be unblocked.
>  
> Now I'm looking at the breakdown across all the other categories to see what shifted. Most counts stayed the same, but the unresolved template category dropped from 79 to 49, meaning 30 methods moved elsewhere. The string ref count went up by one, which is odd.
> 
> Let me trace where those 30 methods went. I found that 29 of them were LDOMString/LDOMBasicString types that are now completely unblocked, and one DataMapOfStringInteger that's now categorized as using an unknown type instead. That should add 1 to the unknown count, bringing it from 451 to 452.
> 
> When I sum everything up—546 + 452 + 199 + 135 + 49 + 53 + 43 + 18 + 17 + 12 + 5 + 4 + 2 + 1—I get 1536, but the total should be 1538. I'm missing 2 methods somewhere, so I need to look more carefully at the distinction between "is unknown" and "uses unknown type" to find where they went.

**Terminal:**
```sh
grep -v 'stream type\|void pointer\|raw pointer\|unresolved template type\|ambiguous lifetimes\|abstract\|C-style array\|string ref param\|rvalue reference\|CppDeletable\|mut enum\|bindings.toml\|ambiguous overload' tmp/skip-reasons.txt | sort | uniq -c | sort -rn | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
  85 param 'theMessage' uses unknown type 'Standard_SStream&'
  46 param 'theDocument' uses unknown type 'const Handle(TDocStd_Document)&'
  25 param 'theDoc' uses unknown type 'const Handle(TDocStd_Document)&'
  14 return type 'void*' is unknown
  14 param 'theStyle' uses unknown type 'const XCAFPrs_Style&'
  10 param 'doc' uses unknown type 'const Handle(TDocStd_Document)&'
   8 param 'theFlags' uses unknown type 'const ShapeProcess::OperationsFlags&'
   7 param 'theStream' uses unknown type 'const Standard_SStream&'
   6 return type 'Handle(IMeshData::ListOfPnt2d)' is unknown
   6 return type 'const IMeshData::MapOfInteger&' is unknown
   6 param 'theDFace' uses unknown type 'const IMeshData::IFaceHandle&'
   6 param 'theDEdge' uses unknown type 'const IMeshData::IEdgeHandle&'
   5 return type 'const XCAFPrs_Style&' is unknown
   5 return type 'const BRepExtrema_MapOfIntegerPackedMapOfInteger&' is unknown
   5 param 'theWriter' uses unknown type 'RWGltf_GltfOStreamWriter*'
   4 return type 'Handle(TDocStd_Document)' is unknown
   4 return type 'Handle(IMeshData::MapOfInteger)' is unknown
   4 param 'tostr' uses unknown type 'Standard_PCharacter&'
   4 param 'theEdge' uses unknown type 'const IMeshData::IEdgeHandle&'
   4 param 'theDoc' uses unknown Handle type
```
</details>

**Terminal:**
```sh
grep -v 'stream type\|void pointer\|raw pointer\|unresolved template type\|ambiguous lifetimes\|abstract\|C-style array\|string ref param\|rvalue reference\|CppDeletable\|mut enum\|bindings.toml\|ambiguous overload' tmp/skip-reasons.txt | wc -l
```
```
     458
```

> 💭 OK so the "unknown" category went from 451+7=458 to 458. That's about right — the `LDOMString` ones moved from "unresolved template type" to being unblocked (29), and `DataMapOfStringInteger` moved from "unresolved template type" to "unknown type" (+1). Plus the string ref param went from 16 to 17 (one method now makes it further in the check pipeline and gets caught by string ref instead of unresolved template). So:- 30 moved out of "unresolved template type"
> - 29 entirely unblocked (LDOMString/LDOMBasicString methods now bound)- 1 reclassified from "unresolved template type" to something else (probably an LDOMString method that also had another issue... 
> 
> The total dropped from 1567 to 1538, confirming that 29 were completely unblocked and 1 shifted to a different category. Now I'm checking the "unknown Handle type" count to see if there are any patterns I might have missed with the Handle keyword matching. Let me verify the full breakdown across all categories to make sure the numbers add up correctly. I see there's overlap in how I'm categorizing these errors—some methods hit multiple patterns at once. Rather than getting bogged down in perfect accounting, I should focus on what matters: 29 LDOM methods got unblocked. Let me build and test this now.

Good — 29 methods unblocked. Let me now build to make sure everything compiles:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```
<details>
<summary>Output (177 lines) — exit code 101</summary>

```
   Compiling opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:388008:51: error: unknown type name 'DataMapOfStringInteger'
warning: opencascade-sys@0.2.0:  388008 | extern "C" void DataMapOfStringInteger_destructor(DataMapOfStringInteger* self_) { delete self_; }
warning: opencascade-sys@0.2.0:         |                                                   ^
warning: opencascade-sys@0.2.0: 1 error generated.
warning: opencascade-sys@0.2.0: ToolExecError: command did not execute successfully (status code exit status: 1): env -u IPHONEOS_DEPLOYMENT_TARGET LC_ALL="C" "sccache" "c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm64-apple-macosx" "-mmacosx-version-min=26.2" "-I" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include" "-I" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated" "-Wall" "-Wextra" "-std=c++14" "-Wno-unused-function" "-Wno-deprecated-declarations" "-Wno-return-type-c-linkage" "-Wno-delete-non-abstract-non-virtual-dtor" "-Wno-delete-abstract-non-virtual-dtor" "-D_USE_MATH_DEFINES=TRUE" "-o" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/74271c7d28a70e01-wrappers.o" "-c" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp"
error: failed to run custom build command for `opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)`

Caused by:
  process didn't exit successfully: `/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-489f9534c9bc0b7a/build-script-build` (exit status: 1)
  --- stdout
  cargo:rerun-if-env-changed=DEP_OCCT_ROOT
  cargo:rustc-link-search=native=/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/lib
  cargo:rustc-link-lib=static=TKMath
  cargo:rustc-link-lib=static=TKernel
  cargo:rustc-link-lib=static=TKDE
  cargo:rustc-link-lib=static=TKFeat
  cargo:rustc-link-lib=static=TKGeomBase
  cargo:rustc-link-lib=static=TKG2d
  cargo:rustc-link-lib=static=TKG3d
  cargo:rustc-link-lib=static=TKTopAlgo
  cargo:rustc-link-lib=static=TKGeomAlgo
  cargo:rustc-link-lib=static=TKBRep
  cargo:rustc-link-lib=static=TKPrim
  cargo:rustc-link-lib=static=TKDESTEP
  cargo:rustc-link-lib=static=TKDEIGES
  cargo:rustc-link-lib=static=TKDESTL
  cargo:rustc-link-lib=static=TKMesh
  cargo:rustc-link-lib=static=TKShHealing
  cargo:rustc-link-lib=static=TKFillet
  cargo:rustc-link-lib=static=TKBool
  cargo:rustc-link-lib=static=TKBO
  cargo:rustc-link-lib=static=TKOffset
  cargo:rustc-link-lib=
... (truncated, 10950 chars total)
```
</details>

**Terminal:**
```sh
grep "error\|warning\|Finished" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_01KR6HXLhmCvPS9duraqJjdS__vscode-1771312785201/content.txt
```
<details>
<summary>Output (12 lines)</summary>

```
-rs/crates/opencascade-binding-generator)                                       warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencasc
ade-sys/generated/wrappers.cpp:388008:51: error: unknown type name 'DataMapOfStringInteger'                                                                     warning: opencascade-sys@0.2.0:  388008 | extern "C" void DataMapOfStringInteger
_destructor(DataMapOfStringInteger* self_) { delete self_; }                    warning: opencascade-sys@0.2.0:         |                                       
            ^                                                                   warning: opencascade-sys@0.2.0: 1 error generated.
warning: opencascade-sys@0.2.0: ToolExecError: command did not execute successfu
lly (status code exit status: 1): env -u IPHONEOS_DEPLOYMENT_TARGET LC_ALL="C" "sccache" "c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm64-apple-macosx" "-mmacosx-version-min=26.2" "-I" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include" "-I" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated" "-Wall" "-Wextra" "-std=c++14" "-Wno-unused-function" "-Wno-deprecated-declarations" "-Wno-return-type-c-linkage" "-Wno-delete-non-abstract-non-virtual-dtor" "-Wno-delete-abstract-non-virtual-dtor" "-D_USE_MATH_DEFINES=TRUE" "-o" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/74271c7d28a70e01-wrappers.o" "-c" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp"                                       error: failed to run custom build command for `opencascade-sys v0.2.0 (/Users/se
  cargo:warning=/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generate
d/wrappers.cpp:388008:51: error: unknown type name 'DataMapOfStringInteger'       cargo:warning= 388008 | extern "C" void DataMapOfStringInteger_destructor(Data
MapOfStringInteger* self_) { delete self_; }                                      cargo:warning=        |                                                   ^
  cargo:warning=1 error generated.
  cargo:warning=ToolExecError: command did not execute successfully (status code
  error occurred in cc-rs: command did not execute successfully (status code exi
```
</details>

C++ compilation error: `DataMapOfStringInteger` is now getting a destructor generated for it because the no-underscore guard was removed. This bare nested typedef shouldn't have a destructor generated. Let me investigate how it ends up as a parsed class:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep "DataMapOfStringInteger" crates/opencascade-sys/generated/ffi.rs
```
<details>
<summary>Output (79 lines)</summary>

```
/// Handle to TDataStd_HDataMapOfStringInteger
pub struct HandleTDataStdHDataMapOfStringInteger {
pub struct TDataStd_HDataMapOfStringInteger {
pub struct DataMapOfStringInteger {
pub struct TColStd_DataMapOfStringInteger {
    /// Downcast Handle<Standard_Transient> to Handle<TDataStd_HDataMapOfStringInteger> (returns null on failure)
    pub fn HandleStandardTransient_downcast_to_HandleTDataStdHDataMapOfStringInteger(
    ) -> *mut HandleTDataStdHDataMapOfStringInteger;
    // ======================== TDataStd_HDataMapOfStringInteger ========================
    pub fn TDataStd_HDataMapOfStringInteger_destructor(
        self_: *mut TDataStd_HDataMapOfStringInteger,
    /// **Source:** `TDataStd_HDataMapOfStringInteger.hxx`:31 - `TDataStd_HDataMapOfStringInteger::TDataStd_HDataMapOfStringInteger()`
    pub fn TDataStd_HDataMapOfStringInteger_ctor_int(
    ) -> *mut TDataStd_HDataMapOfStringInteger;
    /// **Source:** `TDataStd_HDataMapOfStringInteger.hxx`:33 - `TDataStd_HDataMapOfStringInteger::TDataStd_HDataMapOfStringInteger()`
    pub fn TDataStd_HDataMapOfStringInteger_ctor_datamapofstringinteger(
        theOther: *const TColStd_DataMapOfStringInteger,
    ) -> *mut TDataStd_HDataMapOfStringInteger;
    /// **Source:** `TDataStd_HDataMapOfStringInteger.hxx`:29 - `TDataStd_HDataMapOfStringInteger::DynamicType()`
    pub fn TDataStd_HDataMapOfStringInteger_dynamic_type(
        self_: *const TDataStd_HDataMapOfStringInteger,
    /// **Source:** `TDataStd_HDataMapOfStringInteger.hxx`:35 - `TDataStd_HDataMapOfStringInteger::Map()`
    pub fn TDataStd_HDataMapOfStringInteger_map(
        self_: *const TDataStd_HDataMapOfStringInteger,
    ) -> *const TColStd_DataMapOfStringInteger;
    /// **Source:** `TDataStd_HDataMapOfStringInteger.hxx`:37 - `TDataStd_HDataMapOfStringInteger::ChangeMap()`
    pub fn TDataStd_HDataMapOfStringInteger_change_map(
        self_: *mut TDataStd_HDataMapOfStringInteger,
    ) -> *mut TColStd_DataMapOfStringInteger;
    /// **Source:** `TDataStd_HDataMapOfStringInteger.hxx`:29 - `TDataStd_HDataMapOfStringInteger::get_type_name()`
    pub fn TDataStd_HDataMapOfStringInteger_get_type_name() -> *const std::ffi::c_char;
    /// **Source:** `TDataStd_HDataMapOfStringInteger.hxx`:29 - `TDataStd_HDataMapOfStringInteger::get_type_descriptor()`
    pub fn TDataStd_HDataMapOfStringInteger_get_type_descriptor() -> *const HandleStandardType;
    /// Upcast TDataStd_HDataMapOfStringInteger to Standard_Transient
    pub fn TDataStd_HDataMapOfStringInteger_as_Standard_Transient(
        self_: *const TDataStd_HDataMapOfStringInteger,
    /// Upcast TDataStd_HDataMapOfStringInteger to Standard_Transient (mutable)
    pub fn TDataStd_HDataMapOfStringInteger_as_Standard_Transient_mut(
        self_: *mut TDataStd_HDataMapOfStringInteger,
    /// Wrap TDataStd_HDataMapOfStringInteger in a Handle
    pub fn TDataStd_HDataMapOfStringInteger_to_handle(
        obj: *mut TDataStd_HDataMapOfStringI
... (truncated, 5353 chars total)
```
</details>

> 💭 The problem is that `DataMapOfStringInteger` is being parsed as a standalone class and getting its own struct declaration, destructor, AND CppDeletable impl in ffi.rs. But it's a nested typedef in `Express_Entity`. When the "no underscore → unresolved" heuristic was in place, the FFI declaration existed but the C++ destructor wrapper couldn't actually work because C++ doesn't know `DataMapOfStringInteger` as a standalone type — it's `Express_Entity::DataMapOfStringInteger`.
> 
> So the issue is: the parser already creates the struct declaration, but the C++ wrapper generates a `DataMapOfStringInteger_destructor` function that refers to a nonexistent C++ type.
>  
> Now I need to trace through where destructors are actually being generated, since removing the no-underscore check only affects method parameters and return types. Let me check the git diff to see what actually changed in the generated files.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff --stat crates/opencascade-sys/generated/
```
<details>
<summary>Output (7 lines)</summary>

```
 crates/opencascade-sys/generated/b_rep_mesh.rs     |   4 +-
 crates/opencascade-sys/generated/ffi.rs            | 191 ++++++-
 crates/opencascade-sys/generated/ldom.rs           | 568 ++++++++++++++++-----
 .../opencascade-sys/generated/ldom_basic_string.rs |  11 +-
 crates/opencascade-sys/generated/moni_tool.rs      |   4 +-
 crates/opencascade-sys/generated/wrappers.cpp      | 145 +++++-
 6 files changed, 777 insertions(+), 146 deletions(-)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff crates/opencascade-sys/generated/wrappers.cpp | grep "DataMapOfStringInteger"
```
```
+extern "C" void DataMapOfStringInteger_destructor(DataMapOfStringInteger* self_) { delete self_; }
```

> 💭 So `DataMapOfStringInteger_destructor` is newly added in this diff. Previously, it was blocked from being generated because `is_unresolved_template_type()` returned true for `DataMapOfStringInteger`. But now that we removed that check, the destructor gets generated but C++ doesn't know what `DataMapOfStringInteger` is.
> 
> Let me check where destructors are filtered. The destructor generation must check `is_unresolved_template_type()` somehow.Actually, looking at the code more carefully, I think what's happening is:
> 1. The parser sees `DataMapOfStringInteger` as a typedef/class and creates a `ParsedClass` for it
> 
> 
> 2. The code generator then creates a destructor for it, but the issue is that `DataMapOfStringInteger` is actually a nested typedef inside `Express_Entity` in C++, not a standalone type. Meanwhile, the FFI bindings define it as a struct, creating a mismatch where the generated destructor references a type that doesn't exist in the C++ namespace.
> 
> Let me check if this destructor was already present in the previous commit's wrappers to understand when this broke.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git stash && grep "DataMapOfStringInteger_destructor" crates/opencascade-sys/generated/wrappers.cpp && git stash pop
```
<details>
<summary>Output (18 lines)</summary>

```
Saved working directory and index state WIP on main: 6e9ef9f Rename "unresolved template/nested type" skip reason to "unresolved template type"
extern "C" void HandleTDataStdHDataMapOfStringInteger_destructor(HandleTDataStdHDataMapOfStringInteger* self_) { delete self_; }
extern "C" void TDataStd_HDataMapOfStringInteger_destructor(TDataStd_HDataMapOfStringInteger* self_) { delete self_; }
extern "C" void TColStd_DataMapOfStringInteger_destructor(TColStd_DataMapOfStringInteger* self_) { delete self_; }
On branch main
Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
modified:   crates/opencascade-binding-generator/src/model.rs
modified:   crates/opencascade-sys/generated/b_rep_mesh.rs
modified:   crates/opencascade-sys/generated/ffi.rs
modified:   crates/opencascade-sys/generated/ldom.rs
modified:   crates/opencascade-sys/generated/ldom_basic_string.rs
modified:   crates/opencascade-sys/generated/moni_tool.rs
modified:   crates/opencascade-sys/generated/wrappers.cpp

no changes added to commit (use "git add" and/or "git commit -a")
Dropped refs/stash@{0} (7fb2ac243530b6180ed9a8d0a6c081d64e48534e)
```
</details>

> 💭 So in the PREVIOUS version, there was no `DataMapOfStringInteger_destructor` — only `TColStd_DataMapOfStringInteger_destructor`, `TDataStd_HDataMapOfStringInteger_destructor`, and `HandleTDataStdHDataMapOfStringInteger_destructor`.
> 
> The `DataMapOfStringInteger` struct was already in ffi.rs AND was marked as CppDeletable (`DataMapOfStringInteger_destructor`), but the DESTRUCTOR FUNCTION wasn't in wrappers.cpp. So somehow the destructor was being suppressed.
> 
> Let me look at how destructors are generated. 
> 
> I'm realizing there's a mismatch here — the Rust side declared both the struct and the extern function for its destructor, and even marked it as CppDeletable, but the actual C++ wrapper function was never emitted. I need to check whether that extern declaration was actually present in the previous version's ffi.rs.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git stash && grep "DataMapOfStringInteger" crates/opencascade-sys/generated/ffi.rs | head -10 && git stash pop
```
<details>
<summary>Output (25 lines)</summary>

```
Saved working directory and index state WIP on main: 6e9ef9f Rename "unresolved template/nested type" skip reason to "unresolved template type"
/// Handle to TDataStd_HDataMapOfStringInteger
pub struct HandleTDataStdHDataMapOfStringInteger {
pub struct TDataStd_HDataMapOfStringInteger {
pub struct TColStd_DataMapOfStringInteger {
    /// Downcast Handle<Standard_Transient> to Handle<TDataStd_HDataMapOfStringInteger> (returns null on failure)
    pub fn HandleStandardTransient_downcast_to_HandleTDataStdHDataMapOfStringInteger(
    ) -> *mut HandleTDataStdHDataMapOfStringInteger;
    // ======================== TDataStd_HDataMapOfStringInteger ========================
    pub fn TDataStd_HDataMapOfStringInteger_destructor(
        self_: *mut TDataStd_HDataMapOfStringInteger,
On branch main
Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
modified:   crates/opencascade-binding-generator/src/model.rs
modified:   crates/opencascade-sys/generated/b_rep_mesh.rs
modified:   crates/opencascade-sys/generated/ffi.rs
modified:   crates/opencascade-sys/generated/ldom.rs
modified:   crates/opencascade-sys/generated/ldom_basic_string.rs
modified:   crates/opencascade-sys/generated/moni_tool.rs
modified:   crates/opencascade-sys/generated/wrappers.cpp

no changes added to commit (use "git add" and/or "git commit -a")
Dropped refs/stash@{0} (12848edbe03064999739ad198950d3cc1e4a7f6f)
```
</details>

So previously `DataMapOfStringInteger` didn't even have a struct declaration in ffi.rs. It was filtered out entirely. Now with the heuristic removed, the parser is emitting it as a class. Let me see where this filtering happens — it must be in the resolver or codegen layer:

Searched for regex `is_unresolved_template_type|contains\('_'\).*class.*name` (`**/crates/opencascade-binding-generator/src/**`), 3 results
- [model.rs:641](../crates/opencascade-binding-generator/src/model.rs#L641)
- [model.rs:674](../crates/opencascade-binding-generator/src/model.rs#L674)
- [model.rs:681](../crates/opencascade-binding-generator/src/model.rs#L681)

Searched for regex `filter.*class_name.*underscore|skip.*class.*no.*underscore|exclude.*class.*name.*_` (`**/crates/opencascade-binding-generator/src/**`), 2 results
- [bindings.rs:1597](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1597)
- [bindings.rs:2665](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2665)

<details>
<summary>Searched for regex <code>contains\(&#x27;_&#x27;\).*class|class.*contains\(&#x27;_&#x27;\)|filter.*class|skip.*class.*struct</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 9 results</summary>

- [main.rs:395](../crates/opencascade-binding-generator/src/main.rs#L395)
- [cpp.rs:50](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L50)
- [parser.rs:433](../crates/opencascade-binding-generator/src/parser.rs#L433)
- [parser.rs:739](../crates/opencascade-binding-generator/src/parser.rs#L739)
- [parser.rs:1599](../crates/opencascade-binding-generator/src/parser.rs#L1599)
- [resolver.rs:366](../crates/opencascade-binding-generator/src/resolver.rs#L366)
- [bindings.rs:1657](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1657)
- [bindings.rs:1963](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1963)
- [bindings.rs:2267](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2267)
</details>

<details>
<summary>Reading lines 385-410 of <a href="../crates/opencascade-binding-generator/src/main.rs#L385-L411">main.rs</a></summary>

```
fn convert_utility_classes_to_functions(
    parsed: &mut [model::ParsedHeader],
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
```
</details>

That's a utility class conversion function, not what filters the class from ffi.rs. Let me look at where class struct declarations are emitted:

<details>
<summary>Searched for regex <code>emit.*struct|struct.*declaration|opaque.*type|class.*ffi</code> (<code>**/crates/opencascade-binding-generator/src/codegen/rust.rs</code>), 13 results</summary>

- [rust.rs:145](../crates/opencascade-binding-generator/src/codegen/rust.rs#L145)
- [rust.rs:167](../crates/opencascade-binding-generator/src/codegen/rust.rs#L167)
- [rust.rs:169](../crates/opencascade-binding-generator/src/codegen/rust.rs#L169)
- [rust.rs:178](../crates/opencascade-binding-generator/src/codegen/rust.rs#L178)
- [rust.rs:287](../crates/opencascade-binding-generator/src/codegen/rust.rs#L287)
- [rust.rs:292](../crates/opencascade-binding-generator/src/codegen/rust.rs#L292)
- [rust.rs:296](../crates/opencascade-binding-generator/src/codegen/rust.rs#L296)
- [rust.rs:327](../crates/opencascade-binding-generator/src/codegen/rust.rs#L327)
- [rust.rs:359](../crates/opencascade-binding-generator/src/codegen/rust.rs#L359)
- [rust.rs:428](../crates/opencascade-binding-generator/src/codegen/rust.rs#L428)
- [rust.rs:432](../crates/opencascade-binding-generator/src/codegen/rust.rs#L432)
- [rust.rs:459](../crates/opencascade-binding-generator/src/codegen/rust.rs#L459)
- [rust.rs:487](../crates/opencascade-binding-generator/src/codegen/rust.rs#L487)
</details>

<details>
<summary>Reading lines 459-530 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L459-L531">rust.rs</a></summary>

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
        // Skip types with pointer/ref qualifiers leaked into the name
        // (e.g., "IMeshData_Edge *const" from typedef resolution)
        if safe_name.contains('*') || safe_name.contains('&') {
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

```
</details>

> 💭 I see! The `generate_opaque_declarations` function emits destructors for `nested_types` (line 523: `if (is_nested || is_extra_typedef) && !has_protected_dtor`). But `DataMapOfStringInteger` is NOT a nested type (no `::` in its name), and it's not an extra typedef. So the destructor would NOT be generated by this code path.
> 
> Let me trace where `DataMapOfStringInteger_destructor` comes from. Looking at the ffi.rs output, there's:
> ```rust
> pub struct DataMapOfStringInteger { ... }
> pub fn DataMapOfStringInteger_destructor(self_: *mut DataMapOfStringInteger);
> 
> 
> ```
> 
> These are all in the ffi.rs. And the C++ wrapper has:
> ```cpp
> extern "C" void DataMapOfStringInteger_destructor(DataMapOfStringInteger* self_) { delete self_; }
> ```
> 
> This comes from somewhere other than the nested types path. Let me look at the class bindings — where destructors are generated for regular classes. I'm seeing that the Rust trait implementation and the C++ extern function are both being generated, which means there's a separate code path handling regular class destructors that I need to find.

<details>
<summary>Searched for regex <code>CppDeletable|destructor_fn_name|emit.*destructor</code> (<code>**/crates/opencascade-binding-generator/src/codegen/rust.rs</code>), 12 results</summary>

- [rust.rs:194](../crates/opencascade-binding-generator/src/codegen/rust.rs#L194)
- [rust.rs:200](../crates/opencascade-binding-generator/src/codegen/rust.rs#L200)
- [rust.rs:202](../crates/opencascade-binding-generator/src/codegen/rust.rs#L202)
- [rust.rs:333](../crates/opencascade-binding-generator/src/codegen/rust.rs#L333)
- [rust.rs:365](../crates/opencascade-binding-generator/src/codegen/rust.rs#L365)
- [rust.rs:432](../crates/opencascade-binding-generator/src/codegen/rust.rs#L432)
- [rust.rs:448](../crates/opencascade-binding-generator/src/codegen/rust.rs#L448)
- [rust.rs:488](../crates/opencascade-binding-generator/src/codegen/rust.rs#L488)
- [rust.rs:1172](../crates/opencascade-binding-generator/src/codegen/rust.rs#L1172)
- [rust.rs:1174](../crates/opencascade-binding-generator/src/codegen/rust.rs#L1174)
- [rust.rs:1180](../crates/opencascade-binding-generator/src/codegen/rust.rs#L1180)
- [rust.rs:1182](../crates/opencascade-binding-generator/src/codegen/rust.rs#L1182)
</details>

<details>
<summary>Searched for regex <code>destructor|CppDeletable|emit_class</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 20 results</summary>

- [bindings.rs:38](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L38)
- [bindings.rs:935](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L935)
- [bindings.rs:936](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L936)
- [bindings.rs:938](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L938)
- [bindings.rs:943](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L943)
- [bindings.rs:1058](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1058)
- [bindings.rs:1063](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1063)
- [bindings.rs:1598](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1598)
- [bindings.rs:1634](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1634)
- [bindings.rs:1648](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1648)
- [bindings.rs:1657](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1657)
- [bindings.rs:1666](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1666)
- [bindings.rs:1956](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1956)
- [bindings.rs:1960](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1960)
- [bindings.rs:1963](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1963)
- [bindings.rs:1965](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1965)
- [bindings.rs:1968](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1968)
- [bindings.rs:2032](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2032)
- [bindings.rs:2615](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2615)
</details>

<details>
<summary>Reading lines 2600-2630 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2600-L2631">bindings.rs</a></summary>

```
        })
        .collect()
}

// ── Inherited method bindings ───────────────────────────────────────────────────
fn compute_inherited_method_bindings(
    class: &ParsedClass,
    symbol_table: &SymbolTable,
    handle_able_classes: &HashSet<String>,
    all_class_names: &HashSet<String>,
    all_enum_names: &HashSet<String>,
    deletable_class_names: Option<&HashSet<String>>,
    reexport_ctx: Option<&ReexportTypeContext>,
    exclude_methods: &HashSet<(String, String)>,
) -> Vec<InheritedMethodBinding> {
    if class.has_protected_destructor {
        return Vec::new();
    }

    let existing_method_names: HashSet<String> =
        class.methods.iter().map(|m| m.name.clone()).collect();
    let mut seen_methods: HashSet<String> = HashSet::new();
    // Methods that an intermediate ancestor has re-declared as protected/private.
    // These must not be generated as inherited bindings even if a more-distant
    // ancestor exposes them publicly (e.g. BOPAlgo_PaveFiller narrowing Clear()).
    let mut protected_in_ancestors: HashSet<String> = HashSet::new();
    let mut result = Vec::new();

    let ancestors = symbol_table.get_all_ancestors_by_name(&class.name);

    for ancestor_name in &ancestors {
```
</details>

Let me look at where `emit_ffi_class` generates the destructor in the FFI output:

Searched for regex `emit_ffi_class` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:5250](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5250)

<details>
<summary>Reading lines 5250-5310 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5250-L5311">bindings.rs</a></summary>

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
```
</details>

I see —  generates a destructor for every class that doesn't have a protected destructor (line 5269-5271). The `DataMapOfStringInteger` is a parsed class, so it gets a destructor generated. But in C++, `DataMapOfStringInteger` is `Express_Entity::DataMapOfStringInteger` — a nested typedef, not a standalone type.

The issue is that `DataMapOfStringInteger` was being parsed as a class (by libclang resolving the typedef), but previously the "no underscore" heuristic prevented its parameters from being bound so it wasn't being treated as a "real" class needing bindings. But actually, looking at the git diff, the struct IS newly appearing in ffi.rs.

Let me check where parsed classes are collected and if there's already a filter for unresolved types:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n "DataMapOfStringInteger" crates/opencascade-sys/generated/ffi.rs
```
<details>
<summary>Output (79 lines)</summary>

```
10461:/// Handle to TDataStd_HDataMapOfStringInteger
10463:pub struct HandleTDataStdHDataMapOfStringInteger {
26836:pub struct TDataStd_HDataMapOfStringInteger {
29979:pub struct DataMapOfStringInteger {
32144:pub struct TColStd_DataMapOfStringInteger {
333167:    /// Downcast Handle<Standard_Transient> to Handle<TDataStd_HDataMapOfStringInteger> (returns null on failure)
333168:    pub fn HandleStandardTransient_downcast_to_HandleTDataStdHDataMapOfStringInteger(
333170:    ) -> *mut HandleTDataStdHDataMapOfStringInteger;
486562:    // ======================== TDataStd_HDataMapOfStringInteger ========================
486563:    pub fn TDataStd_HDataMapOfStringInteger_destructor(
486564:        self_: *mut TDataStd_HDataMapOfStringInteger,
486566:    /// **Source:** `TDataStd_HDataMapOfStringInteger.hxx`:31 - `TDataStd_HDataMapOfStringInteger::TDataStd_HDataMapOfStringInteger()`
486567:    pub fn TDataStd_HDataMapOfStringInteger_ctor_int(
486569:    ) -> *mut TDataStd_HDataMapOfStringInteger;
486570:    /// **Source:** `TDataStd_HDataMapOfStringInteger.hxx`:33 - `TDataStd_HDataMapOfStringInteger::TDataStd_HDataMapOfStringInteger()`
486571:    pub fn TDataStd_HDataMapOfStringInteger_ctor_datamapofstringinteger(
486572:        theOther: *const TColStd_DataMapOfStringInteger,
486573:    ) -> *mut TDataStd_HDataMapOfStringInteger;
486574:    /// **Source:** `TDataStd_HDataMapOfStringInteger.hxx`:29 - `TDataStd_HDataMapOfStringInteger::DynamicType()`
486575:    pub fn TDataStd_HDataMapOfStringInteger_dynamic_type(
486576:        self_: *const TDataStd_HDataMapOfStringInteger,
486578:    /// **Source:** `TDataStd_HDataMapOfStringInteger.hxx`:35 - `TDataStd_HDataMapOfStringInteger::Map()`
486579:    pub fn TDataStd_HDataMapOfStringInteger_map(
486580:        self_: *const TDataStd_HDataMapOfStringInteger,
486581:    ) -> *const TColStd_DataMapOfStringInteger;
486582:    /// **Source:** `TDataStd_HDataMapOfStringInteger.hxx`:37 - `TDataStd_HDataMapOfStringInteger::ChangeMap()`
486583:    pub fn TDataStd_HDataMapOfStringInteger_change_map(
486584:        self_: *mut TDataStd_HDataMapOfStringInteger,
486585:    ) -> *mut TColStd_DataMapOfStringInteger;
486586:    /// **Source:** `TDataStd_HDataMapOfStringInteger.hxx`:29 - `TDataStd_HDataMapOfStringInteger::get_type_name()`
486587:    pub fn TDataStd_HDataMapOfStringInteger_get_type_name() -> *const std::ffi::c_char;
486588:    /// **Source:** `TDataStd_HDataMapOfStringInteger.hxx`:29 - `TDataStd_HDataMapOfStringInteger::get_type_descriptor()`
486589:    pub fn TDataStd_HDataMapOfStringInteger_get_type_descriptor() -> *const HandleStandardType;
486590:    /// Upcast TDataStd_HDataMapOfStringInteger to Standard_Transient
486591:    pub fn TDataStd_HDataMapOfStringInteger_as_Standard_Transient(
486592:        self_: *const TDataStd_HDataMapOfStringInteger,
486594:    /// Upcast TDataStd_HDataMapOfStringInteger to Standard_Transient (mutable)
486595:    pub fn TDataStd_HData
... (truncated, 5901 chars total)
```
</details>

> 💭 OK, so `DataMapOfStringInteger` appears at line 29979 as a `pub struct` and at line 555668 as `DataMapOfStringInteger_destructor`. This is an opaque type declaration (generated by `generate_opaque_declarations`). The `DataMapOfStringInteger` struct IS being emitted because it's referenced as a type somewhere (e.g., in the constructor parameter of `TDataStd_HDataMapOfStringInteger_ctor_datamapofstringinteger`), but wait, that uses `TColStd_DataMapOfStringInteger`, not `DataMapOfStringInteger`.
>  
> So `DataMapOfStringInteger` must be getting pulled into `collected_types.classes` because it's referenced in some method signature, which is why `generate_opaque_declarations` creates a struct declaration for it even though it wasn't parsed as a defined class. I see that `DataMapOfStringInteger_destructor` is declared in the extern block, and then there's a `CppDeletable` implementation right after it. So the destructor declaration and the trait impl are being generated together for this opaque type — that's where it's coming from, not from the nested types path.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n "DataMapOfStringInteger" crates/opencascade-sys/generated/wrappers.cpp
```
<details>
<summary>Output (38 lines)</summary>

```
4076:#include <TColStd_DataMapOfStringInteger.hxx>
4250:#include <TDataStd_HDataMapOfStringInteger.hxx>
7092:typedef opencascade::handle<TDataStd_HDataMapOfStringInteger> HandleTDataStdHDataMapOfStringInteger;
9437:extern "C" void HandleTDataStdHDataMapOfStringInteger_destructor(HandleTDataStdHDataMapOfStringInteger* self_) { delete self_; }
230342:extern "C" HandleTDataStdHDataMapOfStringInteger* HandleStandardTransient_downcast_to_HandleTDataStdHDataMapOfStringInteger(const HandleStandardTransient* self_) {
230343:    opencascade::handle<TDataStd_HDataMapOfStringInteger> result = opencascade::handle<TDataStd_HDataMapOfStringInteger>::DownCast(*self_);
230345:    return new HandleTDataStdHDataMapOfStringInteger(result);
335983:// TDataStd_HDataMapOfStringInteger wrappers
335986:extern "C" TDataStd_HDataMapOfStringInteger* TDataStd_HDataMapOfStringInteger_ctor_int(Standard_Integer NbBuckets) {
335987:    return new TDataStd_HDataMapOfStringInteger(NbBuckets);
335989:extern "C" TDataStd_HDataMapOfStringInteger* TDataStd_HDataMapOfStringInteger_ctor_datamapofstringinteger(const TColStd_DataMapOfStringInteger* theOther) {
335990:    return new TDataStd_HDataMapOfStringInteger(*theOther);
335992:extern "C" const opencascade::handle<Standard_Type>& TDataStd_HDataMapOfStringInteger_dynamic_type(const TDataStd_HDataMapOfStringInteger* self_) {
335995:extern "C" const TColStd_DataMapOfStringInteger& TDataStd_HDataMapOfStringInteger_map(const TDataStd_HDataMapOfStringInteger* self_) {
335998:extern "C" TColStd_DataMapOfStringInteger& TDataStd_HDataMapOfStringInteger_change_map(TDataStd_HDataMapOfStringInteger* self_) {
336001:extern "C" const char* TDataStd_HDataMapOfStringInteger_get_type_name() {
336002:    return TDataStd_HDataMapOfStringInteger::get_type_name();
336004:extern "C" const opencascade::handle<Standard_Type>& TDataStd_HDataMapOfStringInteger_get_type_descriptor() {
336005:    return TDataStd_HDataMapOfStringInteger::get_type_descriptor();
336007:extern "C" const Standard_Transient* TDataStd_HDataMapOfStringInteger_as_Standard_Transient(const TDataStd_HDataMapOfStringInteger* self_) { return static_cast<const Standard_Transient*>(self_); }
336008:extern "C" Standard_Transient* TDataStd_HDataMapOfStringInteger_as_Standard_Transient_mut(TDataStd_HDataMapOfStringInteger* self_) { return static_cast<Standard_Transient*>(self_); }
336009:extern "C" HandleTDataStdHDataMapOfStringInteger* TDataStd_HDataMapOfStringInteger_to_handle(TDataStd_HDataMapOfStringInteger* obj) {
336010:    return new HandleTDataStdHDataMapOfStringInteger(obj);
336012:extern "C" const TDataStd_HDataMapOfStringInteger* HandleTDataStdHDataMapOfStringInteger_get(const HandleTDataStdHDataMapOfStringInteger* handle) { return (*handle).get(); }
336013:extern "C" TDataStd_HDataMapOfStringInteger* HandleTDataStdHDataMapOfStringInteger_get_mut(HandleTDataStdHDataMapOfStringInteger* handle) { return (*handle).get(); }
336014:extern "C" HandleStandardTransien
... (truncated, 4761 chars total)
```
</details>

Line 388008 is the problematic one. Let me trace how `DataMapOfStringInteger` ends up in the C++ wrapper. Since it's not a nested type and not an extra typedef, it must be getting emitted as a class binding. Let me check if it's a parsed class:

Trace DataMapOfStringInteger destructor generation

Searched for text `DataMapOfStringInteger` (`**/crates/opencascade-binding-generator/src/**`), no results

<details>
<summary>Searched for text <code>destructor</code> (<code>**/crates/opencascade-binding-generator/src/codegen/rust.rs</code>), 20 results</summary>

- [rust.rs:38](../crates/opencascade-binding-generator/src/codegen/rust.rs#L38)
- [rust.rs:142](../crates/opencascade-binding-generator/src/codegen/rust.rs#L142)
- [rust.rs:143](../crates/opencascade-binding-generator/src/codegen/rust.rs#L143)
- [rust.rs:173](../crates/opencascade-binding-generator/src/codegen/rust.rs#L173)
- [rust.rs:178](../crates/opencascade-binding-generator/src/codegen/rust.rs#L178)
- [rust.rs:179](../crates/opencascade-binding-generator/src/codegen/rust.rs#L179)
- [rust.rs:185](../crates/opencascade-binding-generator/src/codegen/rust.rs#L185)
- [rust.rs:189](../crates/opencascade-binding-generator/src/codegen/rust.rs#L189)
- [rust.rs:204](../crates/opencascade-binding-generator/src/codegen/rust.rs#L204)
- [rust.rs:327](../crates/opencascade-binding-generator/src/codegen/rust.rs#L327)
- [rust.rs:328](../crates/opencascade-binding-generator/src/codegen/rust.rs#L328)
- [rust.rs:359](../crates/opencascade-binding-generator/src/codegen/rust.rs#L359)
- [rust.rs:360](../crates/opencascade-binding-generator/src/codegen/rust.rs#L360)
- [rust.rs:432](../crates/opencascade-binding-generator/src/codegen/rust.rs#L432)
- [rust.rs:442](../crates/opencascade-binding-generator/src/codegen/rust.rs#L442)
- [rust.rs:450](../crates/opencascade-binding-generator/src/codegen/rust.rs#L450)
- [rust.rs:460](../crates/opencascade-binding-generator/src/codegen/rust.rs#L460)
- [rust.rs:471](../crates/opencascade-binding-generator/src/codegen/rust.rs#L471)
</details>

<details>
<summary>Searched for text <code>destructor</code> (<code>**/crates/opencascade-binding-generator/src/codegen/cpp.rs</code>), 7 results</summary>

- [cpp.rs:21](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L21)
- [cpp.rs:282](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L282)
- [cpp.rs:283](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L283)
- [cpp.rs:287](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L287)
- [cpp.rs:303](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L303)
- [cpp.rs:306](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L306)
- [cpp.rs:310](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L310)
</details>

<details>
<summary>Searched for text <code>DataMapOfStringInteger_destructor</code> (<code>**/crates/opencascade-sys/generated/**</code>), 12 results</summary>

- [t_data_std.rs:8505](../crates/opencascade-sys/generated/t_data_std.rs#L8505)
- [t_data_std.rs:8668](../crates/opencascade-sys/generated/t_data_std.rs#L8668)
- [ffi.rs:486563](../crates/opencascade-sys/generated/ffi.rs#L486563)
- [ffi.rs:486603](../crates/opencascade-sys/generated/ffi.rs#L486603)
- [ffi.rs:555668](../crates/opencascade-sys/generated/ffi.rs#L555668)
- [ffi.rs:556535](../crates/opencascade-sys/generated/ffi.rs#L556535)
- [ffi.rs:557184](../crates/opencascade-sys/generated/ffi.rs#L557184)
- [ffi.rs:559233](../crates/opencascade-sys/generated/ffi.rs#L559233)
- [wrappers.cpp:9437](../crates/opencascade-sys/generated/wrappers.cpp#L9437)
- [wrappers.cpp:336038](../crates/opencascade-sys/generated/wrappers.cpp#L336038)
- [wrappers.cpp:388008](../crates/opencascade-sys/generated/wrappers.cpp#L388008)
- [wrappers.cpp:388417](../crates/opencascade-sys/generated/wrappers.cpp#L388417)
</details>

<details>
<summary>Reading lines 130-220 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L130-L221">rust.rs</a></summary>

```
///
/// Returns the generated Rust code as a String.
pub fn generate_ffi(
    all_classes: &[&ParsedClass],
    all_headers: &[String],
    collections: &[super::collections::CollectionInfo],
    symbol_table: &crate::resolver::SymbolTable,
    all_bindings: &[super::bindings::ClassBindings],
    function_bindings: &[super::bindings::FunctionBinding],
    handle_able_classes: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
) -> (String, Vec<NestedTypeInfo>) {
    // Get all classes with protected destructors
    let protected_destructor_class_names = symbol_table.protected_destructor_class_names();

    // All enum names (needed for opaque type filtering)
    let all_enum_names = &symbol_table.all_enum_names;

    // Collect collection type names to exclude from class generation
    // Collections are generated separately with specialized wrappers
    let collection_type_names: HashSet<String> = collections.iter()
        .map(|c| c.typedef_name.clone())
        .collect();

    // Emit ffi declarations from pre-computed ClassBindings
    let class_items: String = all_bindings
        .iter()
        .filter(|b| !collection_type_names.contains(&b.cpp_name))
        .map(|b| super::bindings::emit_ffi_class(b))
        .collect();

    // Generate namespace-level free functions from pre-computed FunctionBindings
    let function_items = generate_functions_from_bindings(function_bindings);

    // Generate Handle type declarations
    let handle_decls = generate_handle_declarations(all_classes, handle_able_classes, &symbol_table.handle_able_classes);

    // Collect opaque type declarations (types referenced but not defined)
    let collected_types = collect_referenced_types(all_classes, handle_able_classes);
    let (opaque_type_decls, nested_types) = generate_opaque_declarations(
        &collected_types,
        all_classes,
        all_enum_names,
        &protected_destructor_class_names,
        &collection_type_names,
        extra_typedef_names,
    );

    // Generate destructor declarations for nested types and extra typedef types
    let nested_destructor_decls = if nested_types.is_empty() {
        String::new()
    } else {
        let mut s = String::new();
        writeln!(s).unwrap();
        writeln!(s, "    // ========================================").unwrap();
        writeln!(s, "    // Nested type & typedef type destructors").unwrap();
        writeln!(s, "    // ========================================").unwrap();
        writeln!(s).unwrap();
        for nt in &nested_types {
            writeln!(s, "    pub fn {}_destructor(self_: *mut {});", nt.ffi_name, nt.ffi_name).unwrap();
        }
        s
    };

    // Generate CppDeletable impls for nested types and extra typedef types
    let nested_deletable_impls = if nested_types.is_empty() {
        String::new()
    } else {
        let mut s = String::new();
        writeln!(s).unwrap();
        writeln!(s, "// CppDeletable impls for nested and typedef types").unwrap();
        for nt in &nested_types {
            writeln!(s, "unsafe impl crate::CppDeletable for {} {{", nt.ffi_name).unwrap();
            writeln!(s, "    unsafe fn cpp_delete(ptr: *mut Self) {{").unwrap();
            writeln!(s, "        {}_destructor(ptr);", nt.ffi_name).unwrap();
            writeln!(s, "    }}").unwrap();
            writeln!(s, "}}").unwrap();
        }
        s
    };

    // Build the output
    let mut out = String::new();

    // File header
    let header_count = all_headers.len();
    writeln!(out, "//! extern \"C\" FFI for OpenCASCADE").unwrap();
    writeln!(out, "//!").unwrap();
    writeln!(out, "//! This file was automatically generated by opencascade-binding-generator").unwrap();
    writeln!(out, "//! from {} OCCT headers.", header_count).unwrap();
    writeln!(out, "//!").unwrap();
```
</details>

<details>
<summary>Reading lines 310-380 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L310-L381">rust.rs</a></summary>

```
            writeln!(out, "    // ========================").unwrap();
            writeln!(out).unwrap();
            out.push_str(&class_items);
        }

        // Free functions section
        if !function_items.is_empty() {
            writeln!(out).unwrap();
            writeln!(out, "    // ========================").unwrap();
            writeln!(out, "    // Free functions").unwrap();
            writeln!(out, "    // ========================").unwrap();
            writeln!(out).unwrap();
            out.push_str(&function_items);
        }

        out.push_str(&coll_ffi_decls);

        // Nested type destructor declarations
        out.push_str(&nested_destructor_decls);

        // Close extern "C" block
        writeln!(out, "}}").unwrap();

        // CppDeletable impls for nested types (must be after extern block)
        out.push_str(&nested_deletable_impls);
    } else {
        // Open extern "C" block
        writeln!(out, "extern \"C\" {{").unwrap();

        // All types and methods section
        if !class_items.is_empty() {
            writeln!(out).unwrap();
            writeln!(out, "    // ========================").unwrap();
            writeln!(out, "    // All types and methods").unwrap();
            writeln!(out, "    // ========================").unwrap();
            writeln!(out).unwrap();
            out.push_str(&class_items);
        }

        // Free functions section
        if !function_items.is_empty() {
            writeln!(out).unwrap();
            writeln!(out, "    // ========================").unwrap();
            writeln!(out, "    // Free functions").unwrap();
            writeln!(out, "    // ========================").unwrap();
            writeln!(out).unwrap();
            out.push_str(&function_items);
        }

        // Nested type destructor declarations
        out.push_str(&nested_destructor_decls);

        // Close extern "C" block
        writeln!(out, "}}").unwrap();

        // CppDeletable impls for nested types (must be after extern block)
        out.push_str(&nested_deletable_impls);
    }

    (out, nested_types)
}

/// Generate free function declarations from pre-computed FunctionBindings.
fn generate_functions_from_bindings(
    function_bindings: &[super::bindings::FunctionBinding],
) -> String {
    let mut out = String::new();
    for func in function_bindings {
        let params_str: String = func.params.iter()
            .map(|p| format!("{}: {}", p.rust_name, p.rust_ffi_type))
            .collect::<Vec<_>>()
```
</details>

<details>
<summary>Reading lines 420-510 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L420-L511">rust.rs</a></summary>

```
        handles.insert(name.clone());
    }

    let mut out = String::new();
    for class_name in &handles {
        let handle_type_name = format!("Handle{}", class_name.replace('_', ""));
        writeln!(out, "/// Handle to {}", class_name).unwrap();
        writeln!(out, "#[repr(C)]").unwrap();
        writeln!(out, "pub struct {} {{ _opaque: [u8; 0] }}", handle_type_name).unwrap();
    }

    // For extra handle types (not from parsed classes), generate standalone
    // CppDeletable impls and destructor FFI declarations.
    // Parsed classes get these in their module files instead.
    let extra_handles: Vec<_> = handles.iter()
        .filter(|name| !defined_handles.contains(*name))
        .collect();
    if !extra_handles.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "extern \"C\" {{").unwrap();
        for class_name in &extra_handles {
            let handle_type_name = format!("Handle{}", class_name.replace('_', ""));
            writeln!(out, "    pub fn {}_destructor(ptr: *mut {});", handle_type_name, handle_type_name).unwrap();
        }
        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();
        for class_name in &extra_handles {
            let handle_type_name = format!("Handle{}", class_name.replace('_', ""));
            writeln!(out, "unsafe impl crate::CppDeletable for {} {{", handle_type_name).unwrap();
            writeln!(out, "    unsafe fn cpp_delete(ptr: *mut Self) {{").unwrap();
            writeln!(out, "        {}_destructor(ptr);", handle_type_name).unwrap();
            writeln!(out, "    }}").unwrap();
            writeln!(out, "}}").unwrap();
        }
    }

    out
}

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
        // Skip types with pointer/ref qualifiers leaked into the name
        // (e.g., "IMeshData_Edge *const" from typedef resolution)
        if safe_name.contains('*') || safe_name.contains('&') {
            continue;
        }
        // Avoid duplicate opaque declarations (flattened nested name might collide
```
</details>

<details>
<summary>Reading lines 275-320 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L275-L321">cpp.rs</a></summary>

```
                "typedef opencascade::handle<{}> {};",
                inner_class, handle_name
            )
            .unwrap();
        }
        writeln!(output).unwrap();

        // Handle type destructors
        writeln!(output, "// Handle type destructors").unwrap();
        for (_inner_class, handle_name) in &handle_types {
            writeln!(
                output,
                "extern \"C\" void {}_destructor({}* self_) {{ delete self_; }}",
                handle_name, handle_name
            )
            .unwrap();
        }
        writeln!(output).unwrap();
    }

    // Generate wrapper functions for ALL classes from pre-computed ClassBindings
    for bindings in all_bindings {
        output.push_str(&super::bindings::emit_cpp_class(bindings));
    }

    // Generate wrappers for ALL namespace-level free functions
    generate_function_wrappers(&mut output, function_bindings, known_headers);

    // Generate destructors for nested types and extra typedef types (e.g., gp_Vec3f)
    if !nested_types.is_empty() {
        writeln!(output).unwrap();
        writeln!(output, "// Nested type and typedef type destructors").unwrap();
        for nt in nested_types {
            writeln!(
                output,
                "extern \"C\" void {ffi}_destructor({cpp}* self_) {{ delete self_; }}",
                ffi = nt.ffi_name,
                cpp = nt.cpp_name
            )
            .unwrap();
        }
    }

    // Generate collection wrappers
    if !collections.is_empty() {
        output.push_str(&super::collections::generate_cpp_collections(collections));
```
</details>

<details>
<summary>Reading lines 555650-555700 of <a href="../crates/opencascade-sys/generated/ffi.rs#L555650-L555701">ffi.rs</a></summary>

```
    pub fn BVH_Vec3d_destructor(self_: *mut BVH_Vec3d);
    pub fn Bnd_Array1OfBox_destructor(self_: *mut Bnd_Array1OfBox);
    pub fn Bnd_Array1OfBox2d_destructor(self_: *mut Bnd_Array1OfBox2d);
    pub fn Bnd_Array1OfSphere_destructor(self_: *mut Bnd_Array1OfSphere);
    pub fn ChFiDS_ListOfHElSpine_destructor(self_: *mut ChFiDS_ListOfHElSpine);
    pub fn ChFiDS_ListOfStripe_destructor(self_: *mut ChFiDS_ListOfStripe);
    pub fn ChFiDS_SecArray1_destructor(self_: *mut ChFiDS_SecArray1);
    pub fn ChFiDS_SequenceOfSurfData_destructor(self_: *mut ChFiDS_SequenceOfSurfData);
    pub fn ChFiDS_SequenceOfSurfData_value_type_destructor(
        self_: *mut ChFiDS_SequenceOfSurfData_value_type,
    );
    pub fn Contap_TheSequenceOfPoint_destructor(self_: *mut Contap_TheSequenceOfPoint);
    pub fn DESTEP_Parameters_ReadMode_ProductContext_destructor(
        self_: *mut DESTEP_Parameters_ReadMode_ProductContext,
    );
    pub fn DE_ConfigurationFormatMap_destructor(self_: *mut DE_ConfigurationFormatMap);
    pub fn DE_ResourceMap_destructor(self_: *mut DE_ResourceMap);
    pub fn DE_ShapeFixParameters_FixMode_destructor(self_: *mut DE_ShapeFixParameters_FixMode);
    pub fn DataMapOfStringInteger_destructor(self_: *mut DataMapOfStringInteger);
    pub fn ExprIntrp_SequenceOfNamedExpression_destructor(
        self_: *mut ExprIntrp_SequenceOfNamedExpression,
    );
    pub fn ExprIntrp_SequenceOfNamedFunction_destructor(
        self_: *mut ExprIntrp_SequenceOfNamedFunction,
    );
    pub fn Expr_Array1OfGeneralExpression_destructor(self_: *mut Expr_Array1OfGeneralExpression);
    pub fn Expr_Array1OfNamedUnknown_destructor(self_: *mut Expr_Array1OfNamedUnknown);
    pub fn Expr_SequenceOfGeneralExpression_destructor(
        self_: *mut Expr_SequenceOfGeneralExpression,
    );
    pub fn Extrema_Array1OfPOnCurv_destructor(self_: *mut Extrema_Array1OfPOnCurv);
    pub fn Extrema_Array1OfPOnCurv2d_destructor(self_: *mut Extrema_Array1OfPOnCurv2d);
    pub fn Extrema_Array1OfPOnSurf_destructor(self_: *mut Extrema_Array1OfPOnSurf);
    pub fn Extrema_Array2OfPOnCurv_destructor(self_: *mut Extrema_Array2OfPOnCurv);
    pub fn Extrema_Array2OfPOnCurv2d_destructor(self_: *mut Extrema_Array2OfPOnCurv2d);
    pub fn Extrema_Array2OfPOnSurf_destructor(self_: *mut Extrema_Array2OfPOnSurf);
    pub fn Extrema_SequenceOfPOnCurv_destructor(self_: *mut Extrema_SequenceOfPOnCurv);
    pub fn Extrema_SequenceOfPOnSurf_destructor(self_: *mut Extrema_SequenceOfPOnSurf);
    pub fn FEmTool_AssemblyTable_destructor(self_: *mut FEmTool_AssemblyTable);
    pub fn FEmTool_AssemblyTable_value_type_destructor(
        self_: *mut FEmTool_AssemblyTable_value_type,
    );
    pub fn GeomFill_Array1OfLocationLaw_destructor(self_: *mut GeomFill_Array1OfLocationLaw);
    pub fn GeomFill_Array1OfLocationLaw_value_type_destructor(
        self_: *mut GeomFill_Array1OfLocationLaw_value_type,
    );
    pub fn GeomFill_Array1OfSectionLaw_destructor(self_: *mut GeomFill_Array1OfSectionLaw);
    pub fn GeomFill_Array1OfSectionLaw_value_type_destructor(
        self_: *mut GeomFill_Array1OfSectionLaw_value_type,
    );
    pub fn GeomFill_SequenceOfAx2_destructor(self_: *mut GeomFill_SequenceOfAx2);
```
</details>

<details>
<summary>Reading lines 510-600 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L510-L601">rust.rs</a></summary>

```
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
        for line in comment.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                writeln!(output, "///").unwrap();
            } else {
                writeln!(output, "/// {}", trimmed).unwrap();
            }
        }
    }
    writeln!(output, "/// C++ enum: `{}`", resolved.cpp_name).unwrap();

    // Collect unique variants (skip duplicated values — C++ allows alias enum values, Rust doesn't)
    let mut seen_values = std::collections::HashSet::new();
    let mut unique_variants = Vec::new();
    let mut next_value: i64 = 0;
    for variant in &resolved.variants {
        let value = variant.value.unwrap_or(next_value);
        if seen_values.insert(value) {
            unique_variants.push((variant, value));
        }
        next_value = value + 1;
    }

    writeln!(output, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]").unwrap();
    writeln!(output, "#[repr(i32)]").unwrap();
    writeln!(output, "pub enum {} {{", resolved.rust_name).unwrap();

    for (variant, value) in &unique_variants {
        if let Some(ref comment) = variant.doc_comment {
            for line in comment.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    writeln!(output, "    ///").unwrap();
                } else {
                    writeln!(output, "    /// {}", trimmed).unwrap();
                }
            }
        }
        writeln!(output, "    {} = {},", variant.rust_name, value).unwrap();
    }
    writeln!(output, "}}").unwrap();
    writeln!(output).unwrap();

    // Generate From<EnumName> for i32
    let name = &resolved.rust_name;
    writeln!(output, "impl From<{}> for i32 {{", name).unwrap();
    writeln!(output, "    fn from(value: {}) -> Self {{", name).unwrap();
    writeln!(output, "        value as i32").unwrap();
    writeln!(output, "    }}").unwrap();
    writeln!(output, "}}").unwrap();
    writeln!(output).unwrap();

    // Generate TryFrom<i32> for EnumName
    // Use explicit error type to avoid ambiguity if enum has an "Error" variant
    writeln!(output, "impl TryFrom<i32> for {} {{", name).unwrap();
    writeln!(output, "    type Error = i32;").unwrap();
    writeln!(output).unwrap();
    writeln!(output, "    fn try_from(value: i32) -> ::core::result::Result<Self, i32> {{").unwrap();
    writeln!(output, "        match value {{").unwrap();
    for (variant, value) in &unique_variants {
        writeln!(output, "            {} => Ok({}::{}),", value, name, variant.rust_name).unwrap();
```
</details>

<details>
<summary>Searched for text <code>DataMapOfStringInteger</code> (<code>**/crates/opencascade-sys/generated/ffi.rs</code>), 30 results</summary>

- [ffi.rs:10461](../crates/opencascade-sys/generated/ffi.rs#L10461)
- [ffi.rs:10463](../crates/opencascade-sys/generated/ffi.rs#L10463)
- [ffi.rs:26836](../crates/opencascade-sys/generated/ffi.rs#L26836)
- [ffi.rs:29979](../crates/opencascade-sys/generated/ffi.rs#L29979)
- [ffi.rs:32144](../crates/opencascade-sys/generated/ffi.rs#L32144)
- [ffi.rs:333167](../crates/opencascade-sys/generated/ffi.rs#L333167)
- [ffi.rs:333168](../crates/opencascade-sys/generated/ffi.rs#L333168)
- [ffi.rs:333170](../crates/opencascade-sys/generated/ffi.rs#L333170)
- [ffi.rs:486562](../crates/opencascade-sys/generated/ffi.rs#L486562)
- [ffi.rs:486563](../crates/opencascade-sys/generated/ffi.rs#L486563)
- [ffi.rs:486564](../crates/opencascade-sys/generated/ffi.rs#L486564)
- [ffi.rs:486566](../crates/opencascade-sys/generated/ffi.rs#L486566)
- [ffi.rs:486567](../crates/opencascade-sys/generated/ffi.rs#L486567)
- [ffi.rs:486569](../crates/opencascade-sys/generated/ffi.rs#L486569)
- [ffi.rs:486570](../crates/opencascade-sys/generated/ffi.rs#L486570)
- [ffi.rs:486571](../crates/opencascade-sys/generated/ffi.rs#L486571)
- [ffi.rs:486572](../crates/opencascade-sys/generated/ffi.rs#L486572)
- [ffi.rs:486573](../crates/opencascade-sys/generated/ffi.rs#L486573)
- [ffi.rs:486574](../crates/opencascade-sys/generated/ffi.rs#L486574)
- [ffi.rs:486575](../crates/opencascade-sys/generated/ffi.rs#L486575)
- [ffi.rs:486576](../crates/opencascade-sys/generated/ffi.rs#L486576)
- [ffi.rs:486578](../crates/opencascade-sys/generated/ffi.rs#L486578)
- [ffi.rs:486579](../crates/opencascade-sys/generated/ffi.rs#L486579)
</details>

<details>
<summary>Reading lines 29970-29985 of <a href="../crates/opencascade-sys/generated/ffi.rs#L29970-L29986">ffi.rs</a></summary>

```
    _opaque: [u8; 0],
}
/// Referenced type from C++
#[repr(C)]
pub struct DE_ShapeFixParameters_FixMode {
    _opaque: [u8; 0],
}
/// Referenced type from C++
#[repr(C)]
pub struct DataMapOfStringInteger {
    _opaque: [u8; 0],
}
/// Referenced type from C++
#[repr(C)]
pub struct ExprIntrp_SequenceOfNamedExpression {
    _opaque: [u8; 0],
```
</details>

<details>
<summary>Searched for regex <code>extra_typedef</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 16 results</summary>

- [main.rs:602](../crates/opencascade-binding-generator/src/main.rs#L602)
- [main.rs:604](../crates/opencascade-binding-generator/src/main.rs#L604)
- [main.rs:608](../crates/opencascade-binding-generator/src/main.rs#L608)
- [main.rs:624](../crates/opencascade-binding-generator/src/main.rs#L624)
- [rust.rs:140](../crates/opencascade-binding-generator/src/codegen/rust.rs#L140)
- [rust.rs:175](../crates/opencascade-binding-generator/src/codegen/rust.rs#L175)
- [rust.rs:473](../crates/opencascade-binding-generator/src/codegen/rust.rs#L473)
- [rust.rs:522](../crates/opencascade-binding-generator/src/codegen/rust.rs#L522)
- [rust.rs:523](../crates/opencascade-binding-generator/src/codegen/rust.rs#L523)
- [bindings.rs:3024](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3024)
- [bindings.rs:3029](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3029)
- [bindings.rs:3036](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3036)
- [bindings.rs:3066](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3066)
- [bindings.rs:3179](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3179)
- [bindings.rs:3209](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3209)
</details>

<details>
<summary>Reading lines 557160-557200 of <a href="../crates/opencascade-sys/generated/ffi.rs#L557160-L557201">ffi.rs</a></summary>

```
    }
}
unsafe impl crate::CppDeletable for DESTEP_Parameters_ReadMode_ProductContext {
    unsafe fn cpp_delete(ptr: *mut Self) {
        DESTEP_Parameters_ReadMode_ProductContext_destructor(ptr);
    }
}
unsafe impl crate::CppDeletable for DE_ConfigurationFormatMap {
    unsafe fn cpp_delete(ptr: *mut Self) {
        DE_ConfigurationFormatMap_destructor(ptr);
    }
}
unsafe impl crate::CppDeletable for DE_ResourceMap {
    unsafe fn cpp_delete(ptr: *mut Self) {
        DE_ResourceMap_destructor(ptr);
    }
}
unsafe impl crate::CppDeletable for DE_ShapeFixParameters_FixMode {
    unsafe fn cpp_delete(ptr: *mut Self) {
        DE_ShapeFixParameters_FixMode_destructor(ptr);
    }
}
unsafe impl crate::CppDeletable for DataMapOfStringInteger {
    unsafe fn cpp_delete(ptr: *mut Self) {
        DataMapOfStringInteger_destructor(ptr);
    }
}
unsafe impl crate::CppDeletable for ExprIntrp_SequenceOfNamedExpression {
    unsafe fn cpp_delete(ptr: *mut Self) {
        ExprIntrp_SequenceOfNamedExpression_destructor(ptr);
    }
}
unsafe impl crate::CppDeletable for ExprIntrp_SequenceOfNamedFunction {
    unsafe fn cpp_delete(ptr: *mut Self) {
        ExprIntrp_SequenceOfNamedFunction_destructor(ptr);
    }
}
unsafe impl crate::CppDeletable for Expr_Array1OfGeneralExpression {
    unsafe fn cpp_delete(ptr: *mut Self) {
        Expr_Array1OfGeneralExpression_destructor(ptr);
    }
```
</details>

Searched for text `get_collected_typedef_names` (`**/crates/opencascade-binding-generator/src/**`), 2 results
- [parser.rs:149](../crates/opencascade-binding-generator/src/parser.rs#L149)
- [main.rs:602](../crates/opencascade-binding-generator/src/main.rs#L602)

Searched for regex `DataMapOfStringInteger` (`**/crates/opencascade-binding-generator/src/**`), no results

<details>
<summary>Reading lines 140-200 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L140-L201">parser.rs</a></summary>

```
/// Look up a type's display name in the typedef map.
/// Returns the typedef name if found.
fn lookup_typedef(display_name: &str) -> Option<String> {
    let key = normalize_template_spelling(display_name);
    TYPEDEF_MAP.with(|m| m.borrow().get(&key).cloned())
}
/// Get all typedef names collected during the last `parse_headers` call.
/// Returns the set of OCCT typedef names that resolve to template specializations.
/// Used by the resolver to register these as known class types.
pub fn get_collected_typedef_names() -> HashSet<String> {
    TYPEDEF_MAP.with(|m| m.borrow().values().cloned().collect())
}



/// Parse a collection of OCCT header files
/// 
/// Uses batch parsing: creates a synthetic source file that includes all headers,
/// parses once, then extracts entities from each target header. This is much faster
/// than parsing each header separately since OCCT headers have deep include chains.
pub fn parse_headers(
    headers: &[impl AsRef<Path>],
    include_dirs: &[impl AsRef<Path>],
    verbose: bool,
) -> Result<Vec<ParsedHeader>> {
    let clang =
        Clang::new().map_err(|e| anyhow::anyhow!("Failed to initialize libclang: {}", e))?;
    let index = Index::new(&clang, false, true);

    // Build canonical path set for target headers
    let header_paths: Vec<std::path::PathBuf> = headers
        .iter()
        .map(|h| {
            let path = h.as_ref();
            // Try to resolve relative paths using include directories
            if path.is_relative() {
                for inc_dir in include_dirs {
                    let full_path = inc_dir.as_ref().join(path);
                    if let Ok(canonical) = full_path.canonicalize() {
                        return canonical;
                    }
                }
            }
            // Fall back to canonicalizing the path as-is
            path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
        })
        .collect();
    let header_set: std::collections::HashSet<&std::path::Path> = 
        header_paths.iter().map(|p| p.as_path()).collect();

    // Create synthetic source that includes all headers
    let mut synthetic_source = String::new();
    for header in headers {
        synthetic_source.push_str(&format!("#include \"{}\"\n", header.as_ref().display()));
    }

    // Build clang arguments
    let mut args: Vec<String> = vec![
        "-x".to_string(),
        "c++".to_string(),
        "-std=c++17".to_string(),
```
</details>

<details>
<summary>Searched for text <code>DataMapOfStringInteger</code> (<code>**/crates/occt-sys/OCCT/src/**</code>), 20 results</summary>

- [TColStd_DataMapOfStringInteger.hxx:15](../crates/occt-sys/OCCT/src/TColStd/TColStd_DataMapOfStringInteger.hxx#L15)
- [TColStd_DataMapOfStringInteger.hxx:16](../crates/occt-sys/OCCT/src/TColStd/TColStd_DataMapOfStringInteger.hxx#L16)
- [TColStd_DataMapOfStringInteger.hxx:22](../crates/occt-sys/OCCT/src/TColStd/TColStd_DataMapOfStringInteger.hxx#L22)
- [TColStd_DataMapOfStringInteger.hxx:24](../crates/occt-sys/OCCT/src/TColStd/TColStd_DataMapOfStringInteger.hxx#L24)
- [TColStd_DataMapIteratorOfDataMapOfStringInteger.hxx:14](../crates/occt-sys/OCCT/src/TColStd/TColStd_DataMapIteratorOfDataMapOfStringInteger.hxx#L14)
- [TColStd_DataMapIteratorOfDataMapOfStringInteger.hxx:15](../crates/occt-sys/OCCT/src/TColStd/TColStd_DataMapIteratorOfDataMapOfStringInteger.hxx#L15)
- [TColStd_DataMapIteratorOfDataMapOfStringInteger.hxx:17](../crates/occt-sys/OCCT/src/TColStd/TColStd_DataMapIteratorOfDataMapOfStringInteger.hxx#L17)
- [FILES:20](../crates/occt-sys/OCCT/src/TColStd/FILES#L20)
- [FILES:27](../crates/occt-sys/OCCT/src/TColStd/FILES#L27)
- [DDataStd_BasicCommands.cxx:57](../crates/occt-sys/OCCT/src/DDataStd/DDataStd_BasicCommands.cxx#L57)
- [DDataStd_BasicCommands.cxx:3691](../crates/occt-sys/OCCT/src/DDataStd/DDataStd_BasicCommands.cxx#L3691)
- [DDataStd_BasicCommands.cxx:3692](../crates/occt-sys/OCCT/src/DDataStd/DDataStd_BasicCommands.cxx#L3692)
- [TDataStd_NamedData.cxx:21](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_NamedData.cxx#L21)
- [TDataStd_NamedData.cxx:30](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_NamedData.cxx#L30)
- [TDataStd_NamedData.cxx:109](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_NamedData.cxx#L109)
- [TDataStd_NamedData.cxx:110](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_NamedData.cxx#L110)
- [TDataStd_NamedData.cxx:122](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_NamedData.cxx#L122)
- [TDataStd_NamedData.cxx:123](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_NamedData.cxx#L123)
- [TDataStd_NamedData.cxx:138](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_NamedData.cxx#L138)
- [TDataStd_NamedData.cxx:139](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_NamedData.cxx#L139)
</details>

Read [TColStd_DataMapOfStringInteger.hxx](../crates/occt-sys/OCCT/src/TColStd/TColStd_DataMapOfStringInteger.hxx)

<details>
<summary>Searched for text <code>DataMapOfStringInteger</code> (<code>**/crates/occt-sys/OCCT/src/TDataStd/TDataStd_HDataMapOfStringInteger.hxx</code>), 14 results</summary>

- [TDataStd_HDataMapOfStringInteger.hxx:16](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_HDataMapOfStringInteger.hxx#L16)
- [TDataStd_HDataMapOfStringInteger.hxx:17](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_HDataMapOfStringInteger.hxx#L17)
- [TDataStd_HDataMapOfStringInteger.hxx:21](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_HDataMapOfStringInteger.hxx#L21)
- [TDataStd_HDataMapOfStringInteger.hxx:25](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_HDataMapOfStringInteger.hxx#L25)
- [TDataStd_HDataMapOfStringInteger.hxx:27](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_HDataMapOfStringInteger.hxx#L27)
- [TDataStd_HDataMapOfStringInteger.hxx:29](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_HDataMapOfStringInteger.hxx#L29)
- [TDataStd_HDataMapOfStringInteger.hxx:31](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_HDataMapOfStringInteger.hxx#L31)
- [TDataStd_HDataMapOfStringInteger.hxx:33](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_HDataMapOfStringInteger.hxx#L33)
- [TDataStd_HDataMapOfStringInteger.hxx:35](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_HDataMapOfStringInteger.hxx#L35)
- [TDataStd_HDataMapOfStringInteger.hxx:37](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_HDataMapOfStringInteger.hxx#L37)
- [TDataStd_HDataMapOfStringInteger.hxx:40](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_HDataMapOfStringInteger.hxx#L40)
- [TDataStd_HDataMapOfStringInteger.hxx:43](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_HDataMapOfStringInteger.hxx#L43)
- [TDataStd_HDataMapOfStringInteger.hxx:45](../crates/occt-sys/OCCT/src/TDataStd/TDataStd_HDataMapOfStringInteger.hxx#L45)
</details>

Searched for text `collect_referenced_types` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), 2 results
- [rust.rs:27](../crates/opencascade-binding-generator/src/codegen/rust.rs#L27)
- [rust.rs:168](../crates/opencascade-binding-generator/src/codegen/rust.rs#L168)

<details>
<summary>Reading lines 1-130 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L1-L131">rust.rs</a></summary>

```
//! Rust extern "C" FFI code generation
//!
//! Generates the extern "C" FFI module with all OCCT types,
//! plus per-module re-export files with short names and impl blocks.

use crate::model::{ParsedClass, Type};
use std::collections::{BTreeSet, HashSet};
use std::fmt::Write as _;

/// Generate source attribution for a declaration (header, line number, and C++ identifier)
fn format_source_attribution(header: &str, line: Option<u32>, cpp_name: &str) -> String {
    match line {
        Some(l) => format!("**Source:** `{}`:{} - `{}`", header, l, cpp_name),
        None => format!("**Source:** `{}` - `{}`", header, cpp_name),
    }
}

/// Types collected from class interfaces
pub struct CollectedTypes {
    /// Class types (e.g., "gp_Pnt", "Geom_TrimmedCurve") - sorted for deterministic output
    pub classes: BTreeSet<String>,
    /// Handle types with their inner class (e.g., "Geom_TrimmedCurve" for Handle<Geom_TrimmedCurve>) - sorted for deterministic output
    pub handles: BTreeSet<String>,
}

/// Collect all referenced OCCT types from class methods and constructors
pub fn collect_referenced_types(
    classes: &[&ParsedClass],
    handle_able_classes: &HashSet<String>,
) -> CollectedTypes {
    let mut result = CollectedTypes {
        classes: BTreeSet::new(),
        handles: BTreeSet::new(),
    };

    for class in classes {
        // Add Handle type for classes that are transient (can be wrapped in Handle)
        // Handle types with protected destructors are included because Handle<T>
        // manages lifetime via reference counting, not direct delete.
        if handle_able_classes.contains(&class.name) {
            result.handles.insert(class.name.clone());
        }

        // From constructors
        for ctor in &class.constructors {
            for param in &ctor.params {
                collect_types_from_type(&param.ty, &mut result);
            }
        }

        // From methods
        for method in &class.methods {
            for param in &method.params {
                collect_types_from_type(&param.ty, &mut result);
            }
            if let Some(ref ret) = method.return_type {
                collect_types_from_type(ret, &mut result);
            }
        }

        // From static methods
        for method in &class.static_methods {
            for param in &method.params {
                collect_types_from_type(&param.ty, &mut result);
            }
            if let Some(ref ret) = method.return_type {
                collect_types_from_type(ret, &mut result);
            }
        }
    }

    result
}

/// Recursively collect OCCT class and Handle types from a type
fn collect_types_from_type(ty: &Type, collected: &mut CollectedTypes) {
    // Skip unbindable types (arrays, streams, void ptrs, etc.)
    if ty.is_unbindable() {
        return;
    }

    match ty {
        Type::Class(name) => {
            // Skip primitive types that may come from canonical type resolution
            if !is_primitive_type(name) {
                collected.classes.insert(name.clone());
            }
        }
        Type::Handle(name) => {
            // Record the Handle type AND the inner class
            collected.handles.insert(name.clone());
            collected.classes.insert(name.clone());
        }
        Type::ConstRef(inner)
        | Type::MutRef(inner)
        | Type::ConstPtr(inner)
        | Type::MutPtr(inner) => {
            collect_types_from_type(inner, collected);
        }
        _ => {}
    }
}

/// Check if a type name is a primitive (not an OCCT class)
pub fn is_primitive_type(name: &str) -> bool {
    matches!(
        name,
        // Rust primitive names
        "bool" | "i32" | "u32" | "u16" | "i64" | "u64" | "f32" | "f64" | "char" | "c_char" |
        "c_long" | "c_ulong" |
        // C++ primitive names (may appear from canonical type resolution)
        "double" | "float" | "int" | "unsigned int" | "long" | "unsigned long" |
        "long long" | "unsigned lo
... (truncated, 4700 chars)
```
</details>

<details>
<summary>Searched for regex <code>[^_]DataMapOfStringInteger[^_]</code> (<code>**/crates/opencascade-sys/generated/ffi.rs</code>), 20 results</summary>

- [ffi.rs:10463](../crates/opencascade-sys/generated/ffi.rs#L10463)
- [ffi.rs:26836](../crates/opencascade-sys/generated/ffi.rs#L26836)
- [ffi.rs:29979](../crates/opencascade-sys/generated/ffi.rs#L29979)
- [ffi.rs:333167](../crates/opencascade-sys/generated/ffi.rs#L333167)
- [ffi.rs:333168](../crates/opencascade-sys/generated/ffi.rs#L333168)
- [ffi.rs:333170](../crates/opencascade-sys/generated/ffi.rs#L333170)
- [ffi.rs:486562](../crates/opencascade-sys/generated/ffi.rs#L486562)
- [ffi.rs:486564](../crates/opencascade-sys/generated/ffi.rs#L486564)
- [ffi.rs:486566](../crates/opencascade-sys/generated/ffi.rs#L486566)
- [ffi.rs:486569](../crates/opencascade-sys/generated/ffi.rs#L486569)
- [ffi.rs:486570](../crates/opencascade-sys/generated/ffi.rs#L486570)
- [ffi.rs:486573](../crates/opencascade-sys/generated/ffi.rs#L486573)
- [ffi.rs:486574](../crates/opencascade-sys/generated/ffi.rs#L486574)
- [ffi.rs:486576](../crates/opencascade-sys/generated/ffi.rs#L486576)
- [ffi.rs:486578](../crates/opencascade-sys/generated/ffi.rs#L486578)
</details>

Searched for regex `: \*(?:mut|const) DataMapOfStringInteger` (`**/crates/opencascade-sys/generated/ffi.rs`), 1 result
- [ffi.rs:555668](../crates/opencascade-sys/generated/ffi.rs#L555668)

<details>
<summary>Searched for regex <code>TYPEDEF_MAP|typedef_map|normalize_template</code> (<code>**/crates/opencascade-binding-generator/src/parser.rs</code>), 10 results</summary>

- [parser.rs:23](../crates/opencascade-binding-generator/src/parser.rs#L23)
- [parser.rs:30](../crates/opencascade-binding-generator/src/parser.rs#L30)
- [parser.rs:48](../crates/opencascade-binding-generator/src/parser.rs#L48)
- [parser.rs:69](../crates/opencascade-binding-generator/src/parser.rs#L69)
- [parser.rs:79](../crates/opencascade-binding-generator/src/parser.rs#L79)
- [parser.rs:101](../crates/opencascade-binding-generator/src/parser.rs#L101)
- [parser.rs:126](../crates/opencascade-binding-generator/src/parser.rs#L126)
- [parser.rs:143](../crates/opencascade-binding-generator/src/parser.rs#L143)
- [parser.rs:144](../crates/opencascade-binding-generator/src/parser.rs#L144)
- [parser.rs:150](../crates/opencascade-binding-generator/src/parser.rs#L150)
</details>

<details>
<summary>Reading lines 18-140 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L18-L141">parser.rs</a></summary>

```
    /// Map from NCollection template display names to their typedef names.
    /// Populated by `collect_ncollection_typedefs()` before type parsing begins.
    /// Key: normalized template spelling (whitespace-stripped), e.g.
    ///   "NCollection_Map<TDF_Label,NCollection_DefaultHasher<TDF_Label>>"
    /// Value: typedef name, e.g. "TDF_LabelMap"
    static TYPEDEF_MAP: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

/// Normalize a C++ type spelling for typedef map lookup.
/// Removes whitespace AND normalizes Standard_* type aliases to their C++ equivalents
/// (e.g. Standard_Integer → int) so that typedef keys match display names even when
/// clang uses different spellings.
fn normalize_template_spelling(s: &str) -> String {
    let no_ws: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    // Normalize OCCT type aliases to C++ primitives for consistent matching.
    // Order matters: longer names first to avoid partial matches.
    no_ws
        .replace("Standard_Integer", "int")
        .replace("Standard_Real", "double")
        .replace("Standard_Boolean", "bool")
        .replace("Standard_ShortReal", "float")
        .replace("Standard_Character", "char")
        .replace("Standard_ExtCharacter", "uint16_t")
        .replace("Standard_Byte", "unsignedchar")
        .replace("Standard_Utf8Char", "char")
}


/// Walk the AST to collect all typedef/using declarations that resolve to
/// template specializations (NCollection, math_VectorBase, etc.).
/// Populates the thread-local TYPEDEF_MAP.
fn collect_ncollection_typedefs(root: &Entity) {
    let mut map = HashMap::new();

    root.visit_children(|entity, _| {
        if entity.get_kind() == EntityKind::TypedefDecl
            || entity.get_kind() == EntityKind::TypeAliasDecl
        {
            if let Some(name) = entity.get_name() {
                // Only record if the typedef name looks like an OCCT type
                // (contains underscore — e.g., math_Vector, TopTools_ListOfShape)
                if !name.contains('_') {
                    return EntityVisitResult::Recurse;
                }

                if let Some(underlying) = entity.get_typedef_underlying_type() {
                    let display = underlying.get_display_name();
                    // Check if this typedef resolves to an NCollection template,
                    // math_VectorBase, math_Matrix, or another template type that
                    // clang might misresolve.
                    if display.contains('<') {
                        let key = normalize_template_spelling(&display);
                        map.insert(key, name.clone());
                    }
                }
            }
        }
        EntityVisitResult::Recurse
    });

    eprintln!("  Collected {} NCollection/template typedefs", map.len());
    TYPEDEF_MAP.with(|m| {
        *m.borrow_mut() = map;
    });
}

/// Supplement the typedef map by text-scanning header files for NCollection typedef
/// patterns. This catches typedefs that clang misresolves (e.g., returning "int"
/// instead of the actual NCollection template type).
///
/// Scans for patterns like:
///   typedef NCollection_Map<TDF_Label> TDF_LabelMap;
///   typedef NCollection_List<TopoDS_Shape> TopTools_ListOfShape;
fn supplement_typedefs_from_headers(include_dirs: &[impl AsRef<Path>]) {
    use std::io::BufRead;

    // Regex: typedef <template_type> <typedef_name>;
    // where <template_type> contains '<' (i.e., is a template instantiation)
    let re = regex::Regex::new(
        r"^\s*typedef\s+((?:NCollection_|TCollection_H)\w+<[^;]+>)\s+(\w+)\s*;"
    ).unwrap();

    let mut count = 0;
    TYPEDEF_MAP.with(|m| {
        let mut map = m.borrow_mut();
        let existing_values: HashSet<String> = map.values().cloned().collect();

        for dir in include_dirs {
            let dir_path = dir.as_ref();
            let entries = match std::fs::read_dir(dir_path) {
                Ok(e) => e,
        
... (truncated, 5271 chars)
```
</details>

Searching for regex `typedef.*DataMapOfStringInteger[^_]` (`**/crates/occt-sys/OCCT/src/**`)

<details>
<summary>Searched for regex <code>DataMapOfStringInteger[^_]</code> (<code>**/crates/opencascade-sys/generated/wrappers.cpp</code>), 10 results</summary>

- [wrappers.cpp:4076](../crates/opencascade-sys/generated/wrappers.cpp#L4076)
- [wrappers.cpp:4250](../crates/opencascade-sys/generated/wrappers.cpp#L4250)
- [wrappers.cpp:7092](../crates/opencascade-sys/generated/wrappers.cpp#L7092)
- [wrappers.cpp:9437](../crates/opencascade-sys/generated/wrappers.cpp#L9437)
- [wrappers.cpp:230342](../crates/opencascade-sys/generated/wrappers.cpp#L230342)
- [wrappers.cpp:230343](../crates/opencascade-sys/generated/wrappers.cpp#L230343)
- [wrappers.cpp:230345](../crates/opencascade-sys/generated/wrappers.cpp#L230345)
</details>

Read [Express_Entity.hxx](../crates/occt-sys/OCCT/src/Express/Express_Entity.hxx#55-55), lines 55 to 75

Searched for text `Express_Entity` (`**/crates/opencascade-sys/generated/ffi.rs`), no results

<details>
<summary>Searched for regex <code>lookup_typedef|resolve.*type.*name</code> (<code>**/crates/opencascade-binding-generator/src/parser.rs</code>), 5 results</summary>

- [parser.rs:142](../crates/opencascade-binding-generator/src/parser.rs#L142)
- [parser.rs:283](../crates/opencascade-binding-generator/src/parser.rs#L283)
- [parser.rs:871](../crates/opencascade-binding-generator/src/parser.rs#L871)
- [parser.rs:1288](../crates/opencascade-binding-generator/src/parser.rs#L1288)
- [parser.rs:1498](../crates/opencascade-binding-generator/src/parser.rs#L1498)
</details>

<details>
<summary>Reading lines 91-100 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L91-L101">parser.rs</a></summary>

```
fn supplement_typedefs_from_headers(include_dirs: &[impl AsRef<Path>]) {
    use std::io::BufRead;

    // Regex: typedef <template_type> <typedef_name>;
    // where <template_type> contains '<' (i.e., is a template instantiation)
    let re = regex::Regex::new(
        r"^\s*typedef\s+((?:NCollection_|TCollection_H)\w+<[^;]+>)\s+(\w+)\s*;"
    ).unwrap();

    let mut count = 0;
```
</details>

<details>
<summary>Reading lines 467-530 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L467-L531">rust.rs</a></summary>

```
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
        // Skip types with pointer/ref qualifiers leaked into the name
        // (e.g., "IMeshData_Edge *const" from typedef resolution)
        if safe_name.contains('*') || safe_name.contains('&') {
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

```
</details>

**Terminal:**
```sh
grep -rn 'typedef NCollection_DataMap<TCollection_AsciiString.*DataMapOfStringInteger' crates/occt-sys/OCCT/src/Express/Express_Entity.hxx
```
```
crates/occt-sys/OCCT/src/Express/Express_Entity.hxx:66:  typedef NCollection_DataMap<TCollection_AsciiString, Standard_Integer> DataMapOfStringInteger;
```

**Terminal:**
```sh
python3 -c "
import re
pattern = r'^\s*typedef\s+((?:NCollection_|TCollection_H)\w+<[^;]+>)\s+(\w+)\s*;'
line = '  typedef NCollection_DataMap<TCollection_AsciiString, Standard_Integer> DataMapOfStringInteger;'
m = re.match(pattern, line)
print('Match:', m is not None)
if m:
    print('Group 1:', m.group(1))
    print('Group 2:', m.group(2))
"
```
```
Match: True
Group 1: NCollection_DataMap<TCollection_AsciiString, Standard_Integer>
Group 2: DataMapOfStringInteger
```

**Terminal:**
```sh
python3 -c "print('DataMapOfStringInteger contains _:', '_' in 'DataMapOfStringInteger')"
```
```
DataMapOfStringInteger contains _: False
```

<details>
<summary>Searched for regex <code>NCollection_DataMap&lt;TCollection_AsciiString.*Standard_Integer</code> (<code>**/crates/occt-sys/OCCT/src/**/*.hxx</code>), 11 results</summary>

- [OpenGl_Context.hxx:1122](../crates/occt-sys/OCCT/src/OpenGl/OpenGl_Context.hxx#L1122)
- [TColStd_DataMapOfAsciiStringInteger.hxx:22](../crates/occt-sys/OCCT/src/TColStd/TColStd_DataMapOfAsciiStringInteger.hxx#L22)
- [TColStd_DataMapOfAsciiStringInteger.hxx:24](../crates/occt-sys/OCCT/src/TColStd/TColStd_DataMapOfAsciiStringInteger.hxx#L24)
- [StepData_ESDescr.hxx:109](../crates/occt-sys/OCCT/src/StepData/StepData_ESDescr.hxx#L109)
- [IFSelect_Editor.hxx:193](../crates/occt-sys/OCCT/src/IFSelect/IFSelect_Editor.hxx#L193)
- [IFSelect_SessionFile.hxx:270](../crates/occt-sys/OCCT/src/IFSelect/IFSelect_SessionFile.hxx#L270)
- [Express_Entity.hxx:66](../crates/occt-sys/OCCT/src/Express/Express_Entity.hxx#L66)
- [MoniTool_TypedValue.hxx:74](../crates/occt-sys/OCCT/src/MoniTool/MoniTool_TypedValue.hxx#L74)
- [MoniTool_TypedValue.hxx:354](../crates/occt-sys/OCCT/src/MoniTool/MoniTool_TypedValue.hxx#L354)
- [Interface_TypedValue.hxx:84](../crates/occt-sys/OCCT/src/Interface/Interface_TypedValue.hxx#L84)
- [Interface_Static.hxx:286](../crates/occt-sys/OCCT/src/Interface/Interface_Static.hxx#L286)
</details>

<details>
<summary>Searched for text <code>MoniTool_TypedValue</code> (<code>**/crates/opencascade-sys/generated/ffi.rs</code>), 5 results</summary>

- [ffi.rs:4486](../crates/opencascade-sys/generated/ffi.rs#L4486)
- [ffi.rs:20728](../crates/opencascade-sys/generated/ffi.rs#L20728)
- [ffi.rs:279873](../crates/opencascade-sys/generated/ffi.rs#L279873)
- [ffi.rs:279874](../crates/opencascade-sys/generated/ffi.rs#L279874)
- [ffi.rs:279876](../crates/opencascade-sys/generated/ffi.rs#L279876)
</details>

Read [TColStd_DataMapOfAsciiStringInteger.hxx](../crates/occt-sys/OCCT/src/TColStd/TColStd_DataMapOfAsciiStringInteger.hxx)

**Terminal:**
```sh
python3 -c "
def normalize(s):
    no_ws = ''.join(c for c in s if not c.isspace())
    return (no_ws
        .replace('Standard_Integer', 'int')
        .replace('Standard_Real', 'double')
        .replace('Standard_Boolean', 'bool')
        .replace('Standard_ShortReal', 'float')
        .replace('Standard_Character', 'char')
        .replace('Standard_ExtCharacter', 'uint16_t')
        .replace('Standard_Byte', 'unsignedchar')
        .replace('Standard_Utf8Char', 'char'))

# From TColStd_DataMapOfAsciiStringInteger.hxx (clang AST likely gives this display name)
key1 = normalize('NCollection_DataMap<TCollection_AsciiString, Standard_Integer>')

# From Express_Entity.hxx private typedef (supplement regex captures)
key2 = normalize('NCollection_DataMap<TCollection_AsciiString, Standard_Integer>')

print('Key 1 (TColStd):', key1)
print('Key 2 (Express):', key2)
print('Same key:', key1 == key2)
"
```
```
Key 1 (TColStd): NCollection_DataMap<TCollection_AsciiString,int>
Key 2 (Express): NCollection_DataMap<TCollection_AsciiString,int>
Same key: True
```

> 💭 Excellent analysis! The root cause is that `supplement_typedefs_from_headers` text-scans headers and picks up a private nested typedef `DataMapOfStringInteger` (inside `Express_Entity`), and since it doesn't contain `_`, the clang-based `collect_ncollection_typedefs` would have filtered it out but the text-scan doesn't.
> 
> So the fix is:
> 1. Add the same underscore filter to `supplement_typedefs_from_headers` that `collect_ncollection_typedefs` has at line 59.
> 2. Or, more precisely, add a filter to `supplement_typedefs_from_headers` that skips typedef names without underscores.
> 
> 
> But wait, `LDOMString` and `LDOMBasicString` don't have underscores either, though they're actual classes rather than NCollection typedefs. The text-scan only targets NCollection_* and TCollection_H* typedefs anyway, so adding the underscore filter would eliminate the `DataMapOfStringInteger` problem without affecting those legitimate types. I need to apply the underscore filter to `supplement_typedefs_from_headers` to resolve both issues.
> 
> With that fix in place, `MoniTool_TypedValue::Internals` should properly resolve to `TColStd_DataMapOfAsciiStringInteger` instead of the incorrect `DataMapOfStringInteger`, which might allow the method to become bound.

Root cause found. The `supplement_typedefs_from_headers` text-scan picks up `Express_Entity`'s private nested typedef `DataMapOfStringInteger` because it's scope-unaware, and this **overwrites** the correct `TColStd_DataMapOfAsciiStringInteger` in the `TYPEDEF_MAP`.

Fix: add the same underscore filter that `collect_ncollection_typedefs` has.

<details>
<summary>Reading lines 91-140 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L91-L141">parser.rs</a></summary>

```
fn supplement_typedefs_from_headers(include_dirs: &[impl AsRef<Path>]) {
    use std::io::BufRead;

    // Regex: typedef <template_type> <typedef_name>;
    // where <template_type> contains '<' (i.e., is a template instantiation)
    let re = regex::Regex::new(
        r"^\s*typedef\s+((?:NCollection_|TCollection_H)\w+<[^;]+>)\s+(\w+)\s*;"
    ).unwrap();

    let mut count = 0;
    TYPEDEF_MAP.with(|m| {
        let mut map = m.borrow_mut();
        let existing_values: HashSet<String> = map.values().cloned().collect();

        for dir in include_dirs {
            let dir_path = dir.as_ref();
            let entries = match std::fs::read_dir(dir_path) {
                Ok(e) => e,
                Err(_) => continue,
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) != Some("hxx") {
                    continue;
                }
                let file = match std::fs::File::open(&path) {
                    Ok(f) => f,
                    Err(_) => continue,
                };
                for line in std::io::BufReader::new(file).lines().flatten() {
                    if let Some(caps) = re.captures(&line) {
                        let template_type = &caps[1];
                        let typedef_name = &caps[2];
                        // Only add if not already collected by clang scan
                        if !existing_values.contains(typedef_name) {
                            let key = normalize_template_spelling(template_type);
                            map.insert(key, typedef_name.to_string());
                            count += 1;
                        }
                    }
                }
            }
        }
    });
    if count > 0 {
        eprintln!("  Supplemented {} additional NCollection typedefs from header text scan", count);
    }
}

/// Look up a type's display name in the typedef map.
```
</details>

<details>
<summary>Reading lines 49-70 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L49-L71">parser.rs</a></summary>

```
fn collect_ncollection_typedefs(root: &Entity) {
    let mut map = HashMap::new();

    root.visit_children(|entity, _| {
        if entity.get_kind() == EntityKind::TypedefDecl
            || entity.get_kind() == EntityKind::TypeAliasDecl
        {
            if let Some(name) = entity.get_name() {
                // Only record if the typedef name looks like an OCCT type
                // (contains underscore — e.g., math_Vector, TopTools_ListOfShape)
                if !name.contains('_') {
                    return EntityVisitResult::Recurse;
                }

                if let Some(underlying) = entity.get_typedef_underlying_type() {
                    let display = underlying.get_display_name();
                    // Check if this typedef resolves to an NCollection template,
                    // math_VectorBase, math_Matrix, or another template type that
                    // clang might misresolve.
                    if display.contains('<') {
                        let key = normalize_template_spelling(&display);
                        map.insert(key, name.clone());
```
</details>

Now I'll fix `supplement_typedefs_from_headers` to skip non-underscore typedef names:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/parser.rs#L121-L138">parser.rs -10+17</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "                    if let Some(caps) = re.captures(&line) {\n                        let template_type = &caps[1];\n                        let typedef_name = &caps[2];\n                        // Skip typedef names without underscore — they're likely\n                        // private nested typedefs inside class bodies (e.g.,\n                        // Express_Entity::DataMapOfStringInteger). Same filter\n                        // as collect_ncollection_typedefs.\n                        if !typedef_name.contains('_') {\n                            continue;\n                        }\n                        // Only add if not already collected by clang scan\n                        if !existing_values.contains(typedef_name) {\n                            let key = normalize_template_spelling(template_type);\n                            map.insert(key, typedef_name.to_string());\n                            count += 1;\n                        }\n                    }",
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "121:sx,122:xb,123:kt,124:uj,125:dk,126:er,127:qe,128:is,129:mf,130:or"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/parser.rs","startLine":121,"endLine":137}
```
</details>

Now rebuild and regenerate:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.98s
real  4.1s
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

Build succeeds. Now run tests:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (37 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.04s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-3404803fe4cd90de)

running 16 tests
test header_deps::tests::test_is_likely_occt_header ... ok
test parser::tests::test_extract_module_from_header ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test module_graph::tests::test_extract_rust_type_name ... ok
test parser::tests::test_map_standard_type ... ok
test parser::tests::test_extract_template_arg ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test resolver::tests::test_safe_method_name ... ok
test resolver::tests::test_safe_param_name ... ok
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

real  4.1s
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo test --workspace --exclude opencascade-binding-generator 2>&1
```

**Terminal:**
```sh
grep -c "test result: ok" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_01KyWsb7TetNL3GzER9isq7A__vscode-1771312785269/content.txt && grep "FAILED" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_01KyWsb7TetNL3GzER9isq7A__vscode-1771312785269/content.txt || echo "No failures"
```
```
17
No failures
```

All 17 test suites pass. Now let me get the final skip count and verify what was unblocked:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | wc -l
```
```
    1519
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && for pat in "stream type" "void pointer" "raw pointer" "unresolved template type" "ambiguous lifetimes" "abstract" "C-style array" "string ref param" "rvalue reference" "CppDeletable" "mut enum"; do echo "$pat: $(grep -r '^//   Reason:' crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "$pat")"; done
```
<details>
<summary>Output (11 lines)</summary>

```
stream type: 546
void pointer: 199
raw pointer: 135
unresolved template type: 49
ambiguous lifetimes: 53
abstract: 43
C-style array: 18
string ref param: 17
rvalue reference: 12
CppDeletable: 5
mut enum: 4
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -v "stream type\|void pointer\|raw pointer\|unresolved template type\|ambiguous lifetimes\|abstract\|C-style array\|string ref param\|rvalue reference\|CppDeletable\|mut enum" | wc -l
```
```
     442
```

> 💭 Let me compute the numbers:
> 
> Before (previous commit): 1567 total
> After my changes: 1519 total
> Reduction: 48 methods unblocked
> 
> Category breakdown:
> - Stream: 546 (unchanged)
> - Unknown: 442 (was ~458, reduced by 16 — some methods previously hitting the template check now properly resolve)
> - Void pointer: 199 (unchanged)
> - Raw pointer: 135 (unchanged)
> - Unresolved template: 49 (was 79, reduced by 30)
> - Ambiguous lifetimes: 53 (unchanged)
> 
> 
> - Abstract: 43 (unchanged)
> - C-style array: 18 (unchanged)
> - String ref param: 17 (was 16, +1 — one method now gets further in the checks)
> - Rvalue reference: 12 (unchanged)
> - CppDeletable: 5 (unchanged)
> - Mut enum: 4 (unchanged)
> 
> When I add these up, I get 1523, but the total should be 1519 — there's a 4-method discrepancy somewhere in my accounting.
> 
> The difference comes from reasons that overlap; some methods match multiple patterns, like ones with both stream and template parameters. I should verify the "unknown Handle type" category is still there too.
> 
> Looking at the actual improvement: 1567 minus 1519 gives me 48 newly unblocked methods. The bulk of that comes from removing the no-underscore heuristic, which freed up 29 LDOMString and LDOMBasicString methods. Another 19 methods got unblocked after I fixed the DataMapOfStringInteger pollution issue — now the typedef map correctly resolves those NCollection_DataMap types instead of using the bogus category.

1567 → 1519 = 48 methods unblocked! Let me see which specific methods were unblocked:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff --stat crates/opencascade-sys/generated/
```
<details>
<summary>Output (13 lines)</summary>

```
 crates/opencascade-sys/generated/b_rep_extrema.rs  |  67 ++-
 crates/opencascade-sys/generated/b_rep_fill.rs     |  91 +++-
 crates/opencascade-sys/generated/b_rep_mesh.rs     |   4 +-
 crates/opencascade-sys/generated/ffi.rs            | 317 +++++++++++-
 crates/opencascade-sys/generated/ldom.rs           | 568 ++++++++++++++++-----
 .../opencascade-sys/generated/ldom_basic_string.rs |  11 +-
 crates/opencascade-sys/generated/loc_ope.rs        |  14 +-
 crates/opencascade-sys/generated/moni_tool.rs      |   4 +-
 crates/opencascade-sys/generated/shape_analysis.rs | 150 ++++--
 crates/opencascade-sys/generated/t_col_std.rs      |   1 +
 crates/opencascade-sys/generated/t_colgp.rs        |  53 +-
 crates/opencascade-sys/generated/wrappers.cpp      | 210 +++++++-
 12 files changed, 1218 insertions(+), 272 deletions(-)
```
</details>

> 💭 Significant changes across many modules:
> - `ldom.rs`: 568 new lines — the LDOM XML DOM module gets much more functionality
> - `shape_analysis.rs`: 150 new lines — shape analysis methods unblocked
> - `b_rep_fill.rs`: 91 new lines — BRep fill operations unblocked
> - `b_rep_extrema.rs`: 67 new lines — BRep extrema (distance) operations
> - `t_colgp.rs`: 53 new lines — geometry collections
> - `loc_ope.rs`: 14 new lines  - `t_col_std.rs`: 1 new line
> 
> 
> - The `DataMapOfStringInteger` type was incorrectly overwriting `TColStd_DataMapOfAsciiStringInteger`, which broke any method relying on `NCollection_DataMap<TCollection_AsciiString, Standard_Integer>` — fixing this unblocks a cascade of previously skipped operations. Now I need to update the READMEs with the correct numbers and commit.

Excellent — the typedef pollution fix had a much bigger impact than expected. Not just LDOM methods, but methods across `shape_analysis`, `b_rep_fill`, `b_rep_extrema`, `t_colgp` that used `NCollection_DataMap<TCollection_AsciiString, Standard_Integer>`.

Now let me update the README with new counts:

**Todo list:**
- ✅ Investigate template type skips
- ✅ Categorize and assess impact
- ✅ Implement fixes
- 🔄 Update READMEs and commit

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -v "stream type\|void pointer\|raw pointer\|unresolved template type\|ambiguous lifetimes\|abstract\|C-style array\|string ref param\|rvalue reference\|CppDeletable\|mut enum" | grep -c "unknown Handle\|unknown type.*Handle"
```
```
128
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "unknown Handle type"
```
```
7
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "bindings.toml"
```
```
2
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "ambiguous overload"
```
```
1
```

> 💭 OK let me get the precise unknown/unresolved type count. Total 1519 - (546 + 199 + 135 + 49 + 53 + 43 + 18 + 17 + 12 + 7 + 5 + 4 + 2 + 1) = 1519 - 1091 = 428. 
> 
> But some methods have multiple reasons counted. Let me just use the "exclusive" approach — count methods not matching any of the other specific categories.
> 
> Actually, wait. The problem is category overlap — a skip reason can contain BOTH "stream type" and "unresolved template type" (e.g., "has unbindable types: param 'S': stream type (...); return: unresolved template type (...)"). 
> 
> I'm realizing the grep counts overlap since a single line can match multiple categories. Rather than trying to force exclusive counts, I should just present the per-category numbers as they are and acknowledge the total, since the README table has always been approximate anyway. For the unknown/unresolved type column, I'll focus on methods skipped primarily because of that reason rather than trying to subtract out all the overlaps.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -v "stream type\|void pointer\|raw pointer\|unresolved template type\|ambiguous lifetimes\|abstract\|C-style array\|string ref param\|rvalue reference\|CppDeletable\|mut enum\|ambiguous overload\|bindings.toml\|unknown Handle type" | wc -l
```
```
     432
```

Now let me update the README. The new totals:

| Count | % | Category |
|---|---|---|
| 546 | 35.9% | Stream type |
| 432 | 28.4% | Unknown/unresolved type |
| 199 | 13.1% | Void pointer |
| 135 | 8.9% | Raw pointer |
| 53 | 3.5% | Ambiguous lifetimes |
| 49 | 3.2% | Unresolved template type |
| 43 | 2.8% | Abstract class |
| 18 | 1.2% | C-style array |
| 17 | 1.1% | String ref param |
| 12 | 0.8% | Rvalue reference |
| 7 | 0.5% | Unknown Handle type |
| 5 | 0.3% | Not CppDeletable |
| 4 | 0.3% | &mut enum return |
| 2 | 0.1% | Excluded by bindings.toml |
| 1 | 0.1% | Ambiguous overload |

<details>
<summary>Reading lines 340-400 of <a href="../crates/opencascade-binding-generator/README.md#L340-L401">README.md</a></summary>

````
See `crates/opencascade-sys/manual/` and the comments in `bindings.toml` for the two existing examples (`Transfer_Finder::GetStringAttribute` and `BRepFill_AdvancedEvolved::SetTemporaryDirectory`).

---

## Skipped Symbols

The binding generator skips ~1,567 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Every skipped symbol is documented in the generated per-module `.rs` files as a `// SKIPPED:` comment block including:

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
| 546 | 34.8% | **Stream type** | C++ `std::istream`/`std::ostream` (`Standard_IStream`/`Standard_OStream`) — no Rust equivalent |
| 451 | 28.7% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`Standard_SStream`, `Handle(TDocStd_Document)`, etc.) |
| 199 | 12.7% | **Void pointer** | `Standard_Address` (typedef for `void*`) — cannot be safely expressed in Rust FFI |
| 135 | 8.6% | **Raw pointer** | `T*`/`const T*` returns (static/free fn) or primitive pointer params (`int*`, `double*`) — class pointer returns on instance methods are now bound as `Option<&T>`/`Option<&mut T>` |
| 79 | 5.0% | **Unresolved template type** | Template instantiations or unqualified types that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `LDOMString`, etc.) |
| 53 | 3.4% | **Ambiguous lifetimes** | `&mut` return with reference params — Rust lifetime inference is ambiguous |
| 43 | 2.7% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |
| 18 | 1.1% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |
| 16 | 1.0% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |
| 12 | 0.8% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across FFI |
| 7 | 0.4% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(IGESData_IGESModel)`, `Handle(CDM_MetaData)`, etc.) |
| 5 | 0.3% | **Not CppDeletable** | Return type class has no destructor in the binding set |
| 4 | 0.3% | **&mut enum return** | Mutable reference to enum (cxx limitation) |
| 2 | 0.1% | **Excluded by bindings.toml** | Explicitly excluded in config (e.g., ambiguous overload workarounds) |
| 1 | 0.1% | **Ambiguous overload** | C++ overload that would produce identical wrapper signatures |

### Most Common Unknown Types

The "unknown type" and "unknown Handle type" categories (29% of all skips) are dominated by a few types:

| Count | Type | How to Unblock |
|------:|------|----------------|
| 97 | `Handle(TDocStd_Document)` | Add `TDocStd_Document` — needed for document framework access |
| 96 | `Standard_SStream` | Map `Standard_SStream` (`std::stringstream`) — mainly used in `Raise()` methods on exception classes |
| 19 | `XCAFPrs_Style` | Add `XCAFPrs_Style` — used in XCAF presentation styles |
| 13 | `TColgp_SequenceOfPnt` | Add `TColgp_SequenceOfPnt` collection — used in shape analysis |
| 12 | `IMeshData::IEdgeHandle` | Nested handle typedef in meshing internals — low priority |
| 11 | `ShapeProcess::OperationsFlags` | Nested type in ShapeProcess — used in STEP/IGES processing flags |
| 10 | `IMeshData::IFaceHandle` | Nested handle typedef in meshing internals — low priority |

Previously common unknown types that have been resolved:
- ~~`math_Vector`~~ (was 665 skips) — resol
... (truncated, 4377 chars)
````
</details>

<details>
<summary>Reading lines 400-412 of <a href="../crates/opencascade-binding-generator/README.md#L400-L413">README.md</a></summary>

```
- ~~`Interface_EntityIterator`~~ (was 59 skips) — now in the binding set
- ~~`Handle(Interface_Protocol)`~~ (was 23 skips) — now in the binding set
- ~~`Handle(Transfer_TransientProcess)`~~ (was 22 skips) — now in the binding set
- ~~`TDF_LabelMap`~~ (was 27 skips) — resolved via header text scan fallback

### Important Skipped Symbols

Most skipped symbols are in internal, low-use, or specialized modules. However, some affect functionality that users commonly need:

**Data Exchange (44 symbols)** — Reduced from 189 after adding `Transfer_TransientProcess`, `Interface_Protocol`, and `Interface_EntityIterator` to the binding set. Remaining skips in `STEPControl_*` (8), `IGESControl_*` (7), `XSControl_*` (25), and `RWStl` (4) are mostly stream types (`Standard_OStream`/`Standard_IStream`), string ref params (`const char*&` in `XSControl_Vars`), unknown IGES-specific Handle types (`Handle(IGESData_IGESModel)`), and rvalue references (`XSAlgo_ShapeProcessor::ParameterMap&&`). The core `Read()`/`Write()` operations are fully bound. **Remaining unblock opportunity**: add `IGESData_IGESModel` (14 skips).

**Document Framework (59 symbols)** — `TDocStd_*` (17 skipped), `TDF_*` (18 skipped), `XCAFDoc_*` (20 skipped), `XCAFPrs_*` (4 skipped). Previously had 215+ skipped symbols due to `TDF_LabelMap` and `TDF_AttributeMap` being unknown — those are now resolved via the header text scan fallback. Remaining skips are dominated by stream types (`Standard_OStream` — 26 of 35 in TDocStd/TDF), with a few raw pointers, unknown types (`TDocStd_XLinkPtr`, `TDF_LabelNodePtr`), and XCAFDoc-specific handles.

```
</details>

Let me also check the "most common unknown types" to see if those changed:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep "uses unknown type\|is unknown" | sed "s/.*unknown type '//; s/'$//" | sed "s/.*type '//; s/' is unknown//" | sort | uniq -c | sort -rn | head -10
```
<details>
<summary>Output (10 lines)</summary>

```
  87 const Handle(TDocStd_Document)&
  86 Standard_SStream&
  24 void*
  19 const XCAFPrs_Style&
  12 const IMeshData::IEdgeHandle&
  11 const ShapeProcess::OperationsFlags&
  10 const Standard_SStream&
  10 const IMeshData::IFaceHandle&
   7 Standard_PCharacter&
   7 const IMeshData::MapOfInteger&
```
</details>

> 💭 The most common unknown types have changed slightly:
> - `Handle(TDocStd_Document)`: 87 (was 97 in the table — some methods that previously had TDocStd_Document as unknown also had DataMapOfStringInteger and were double-counted; now those methods resolve)
> - `Standard_SStream`: 86 (was 96)
> - `XCAFPrs_Style`: 19 (unchanged)
> - `IMeshData::IEdgeHandle`: 12 (unchanged)
> - `ShapeProcess::OperationsFlags`: 11 (unchanged)
> - `IMeshData::IFaceHandle`: 10 (unchanged)
> 
> 
> - And notably, `TColgp_SequenceOfPnt` has dropped out of the top 10 entirely, so I should check its actual count.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^//   Reason:" crates/opencascade-sys/generated/*.rs | grep -c "TColgp_SequenceOfPnt"
```
```
0
```
**Exit code:** 1

`TColgp_SequenceOfPnt` was resolved too — probably because the typedef map fix restored correct type resolution for it. Let me verify it's now in the binding set:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep "TColgp_SequenceOfPnt" crates/opencascade-sys/generated/ffi.rs | head -3
```
```
pub struct TColgp_SequenceOfPnt {
pub struct TColgp_SequenceOfPnt2d {
pub struct TColgp_SequenceOfPnt2d_value_type {
```

Now let me update the README with the new numbers:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/README.md#L346-L347">README.md -1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L363-L380">-17+17</a>, <a href="../crates/opencascade-binding-generator/README.md#L383-L393">-11+10</a>, <a href="../crates/opencascade-binding-generator/README.md#L394-L406">-9+12</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "The binding generator skips ~1,519 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Every skipped symbol is documented in the generated per-module `.rs` files as a `// SKIPPED:` comment block including:",
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "346:zh"
    },
    {
      "content": "| Count | % | Category | Description |\n|------:|----:|----------|-------------|\n| 546 | 35.9% | **Stream type** | C++ `std::istream`/`std::ostream` (`Standard_IStream`/`Standard_OStream`) — no Rust equivalent |\n| 432 | 28.4% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`Standard_SStream`, `Handle(TDocStd_Document)`, etc.) |\n| 199 | 13.1% | **Void pointer** | `Standard_Address` (typedef for `void*`) — cannot be safely expressed in Rust FFI |\n| 135 | 8.9% | **Raw pointer** | `T*`/`const T*` returns (static/free fn) or primitive pointer params (`int*`, `double*`) — class pointer returns on instance methods are now bound as `Option<&T>`/`Option<&mut T>` |\n| 53 | 3.5% | **Ambiguous lifetimes** | `&mut` return with reference params — Rust lifetime inference is ambiguous |\n| 49 | 3.2% | **Unresolved template type** | Template instantiations that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `LDOMBasicString`, etc.) |\n| 43 | 2.8% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |\n| 18 | 1.2% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |\n| 17 | 1.1% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |\n| 12 | 0.8% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across FFI |\n| 7 | 0.5% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(IGESData_IGESModel)`, `Handle(CDM_MetaData)`, etc.) |\n| 5 | 0.3% | **Not CppDeletable** | Return type class has no destructor in the binding set |\n| 4 | 0.3% | **&mut enum return** | Mutable reference to enum (cxx limitation) |\n| 2 | 0.1% | **Excluded by bindings.toml** | Explicitly excluded in config (e.g., ambiguous overload workarounds) |\n| 1 | 0.1% | **Ambiguous overload** | C++ overload that would produce identical wrapper signatures |",
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "363:fa,364:oi,365:lz,366:ly,367:sg,368:ee,369:iv,370:if,371:yt,372:zw,373:ja,374:ki,375:ug,376:vv,377:bo,378:ma,379:ek"
    },
    {
      "content": "The \"unknown type\" and \"unknown Handle type\" categories (29% of all skips) are dominated by a few types:\n\n| Count | Type | How to Unblock |\n|------:|------|----------------|\n| 87 | `Handle(TDocStd_Document)` | Add `TDocStd_Document` — needed for document framework access |\n| 86 | `Standard_SStream` | Map `Stand
... (truncated, 4962 chars)
```
**Output:**
```
{
  "applied": 4,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/README.md",
      "startLine": 346,
      "endLine": 405
    }
  ]
}
```
</details>

Now let me also update the "Important Skipped Symbols" section — the Shape Analysis section mentioned 5 skips from TColgp_SequenceOfPnt which is now resolved:

<details>
<summary>Reading lines 415-445 of <a href="../crates/opencascade-binding-generator/README.md#L415-L446">README.md</a></summary>

````
**Shape Meshing (93 symbols across 3 modules)** — `BRepMesh_*` (78 skipped), `IMeshData_*` (14 skipped), `IMeshTools_*` (1 skipped). The `IMeshData` and `IMeshTools` modules are now in the binding set, but many BRepMesh methods reference internal mesh data types (`IMeshData::IEdgeHandle`, `IMeshData::IFaceHandle`, `IMeshData::MapOfInteger`) that are nested typedefs not yet resolvable. Also includes C-style array params and `std::pair` return types. The core `BRepMesh_IncrementalMesh` meshing API is fully bound.

**Shape Analysis/Fix (5 symbols)** — Reduced from 72 after fixing the OSD_WNT.hxx fatal parse error root cause and adding `ShapeBuild_ReShape` and `GeomAdaptor_Surface`. The 54 legitimate `Standard_Integer&` mode accessors in `ShapeFix_*` are now bound. Remaining 5 skips are all `TColgp_SequenceOfPnt` parameters in `ShapeAnalysis_Curve` methods.

**Geometry (13 symbols in gp/Geom/Geom2d)** — 8 raw pointer returns (`gp_XYZ::GetData()`, `Geom_BSplineCurve::Weights()`, `Geom_BSplineSurface::Weights()`) and 5 `Standard_SStream` parameters (exception `Raise()` methods). All core geometry operations are available; only internal data access and exception-raising methods are skipped.

**Poly (24 symbols)** — Raw pointers (8), ambiguous lifetimes (4), C-style arrays (3), stream types (2), void pointers (2), and misc others. `Poly_CoherentTriangulation` internal access and `Poly_MakeLoops` helper interfaces. All core triangulation access is available.

### How Skipped Symbols Are Tracked

All filtering decisions happen in two places:
- `codegen/bindings.rs`: `is_method_bindable()`, `is_constructor_bindable()`, `is_static_method_bindable()` return `Result<(), String>` with a human-readable reason on `Err`
- `compute_class_bindings()` and `compute_all_function_bindings()` collect `SkippedSymbol` structs for every rejected symbol

The `emit_reexport_class()` and `emit_skipped_functions()` functions write the skip comments to the generated module files.

---

## Implementation Details

The generated code has two layers: a `pub(crate)` FFI module containing the raw extern "C" bindings, and public per-module re-export files that provide the user-facing API.

### Internal: `ffi.rs` (the FFI declarations)

A single `extern "C"` block declares all types and functions using full C++ names. This is `pub(crate)` — users never interact with it directly.

Types use their full C++ identifiers as Rust names to avoid collisions:

```rust
// generated/ffi.rs (pub(crate), not user-facing)
extern "C" {
````
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "^// SKIPPED:" crates/opencascade-sys/generated/shape_analysis.rs | wc -l
```
```
       0
```

Shape Analysis has zero skips now. Let me also check the other modules mentioned:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && for m in shape_fix shape_analysis; do echo "$m: $(grep -c '^// SKIPPED:' crates/opencascade-sys/generated/$m.rs 2>/dev/null || echo 0)"; done
```
```
shape_fix: 0
0
shape_analysis: 0
0
```

Both Shape Fix and Shape Analysis have zero skips now!

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/README.md#L417-L418">README.md -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "**Shape Analysis/Fix (0 symbols)** — Reduced from 72 after fixing the OSD_WNT.hxx fatal parse error root cause and adding `ShapeBuild_ReShape` and `GeomAdaptor_Surface`. The 54 legitimate `Standard_Integer&` mode accessors in `ShapeFix_*` are now bound. The remaining 5 `TColgp_SequenceOfPnt` skips were resolved by fixing typedef map pollution (see above).",
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "417:uj"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/README.md","startLine":417,"endLine":417}
```
</details>

Let me check what the README currently says around the Important Skipped Symbols section and what BRepMesh count is now.

<details>
<summary>Reading lines 400-450 of <a href="../crates/opencascade-binding-generator/README.md#L400-L451">README.md</a></summary>

```
- ~~`Handle(Interface_Protocol)`~~ (was 23 skips) — now in the binding set
- ~~`Handle(Transfer_TransientProcess)`~~ (was 22 skips) — now in the binding set
- ~~`TDF_LabelMap`~~ (was 27 skips) — resolved via header text scan fallback
- ~~`LDOMString`~~ (was 21 skips) — unblocked by removing false "no underscore" heuristic in `is_unresolved_template_type()`
- ~~`LDOMBasicString`~~ (was 8 skips) — same fix as `LDOMString`
- ~~`TColgp_SequenceOfPnt`~~ (was 13 skips) — resolved by fixing typedef map pollution from private nested typedef `DataMapOfStringInteger`

### Important Skipped Symbols

Most skipped symbols are in internal, low-use, or specialized modules. However, some affect functionality that users commonly need:

**Data Exchange (44 symbols)** — Reduced from 189 after adding `Transfer_TransientProcess`, `Interface_Protocol`, and `Interface_EntityIterator` to the binding set. Remaining skips in `STEPControl_*` (8), `IGESControl_*` (7), `XSControl_*` (25), and `RWStl` (4) are mostly stream types (`Standard_OStream`/`Standard_IStream`), string ref params (`const char*&` in `XSControl_Vars`), unknown IGES-specific Handle types (`Handle(IGESData_IGESModel)`), and rvalue references (`XSAlgo_ShapeProcessor::ParameterMap&&`). The core `Read()`/`Write()` operations are fully bound. **Remaining unblock opportunity**: add `IGESData_IGESModel` (14 skips).

**Document Framework (59 symbols)** — `TDocStd_*` (17 skipped), `TDF_*` (18 skipped), `XCAFDoc_*` (20 skipped), `XCAFPrs_*` (4 skipped). Previously had 215+ skipped symbols due to `TDF_LabelMap` and `TDF_AttributeMap` being unknown — those are now resolved via the header text scan fallback. Remaining skips are dominated by stream types (`Standard_OStream` — 26 of 35 in TDocStd/TDF), with a few raw pointers, unknown types (`TDocStd_XLinkPtr`, `TDF_LabelNodePtr`), and XCAFDoc-specific handles.

**Shape Meshing (93 symbols across 3 modules)** — `BRepMesh_*` (78 skipped), `IMeshData_*` (14 skipped), `IMeshTools_*` (1 skipped). The `IMeshData` and `IMeshTools` modules are now in the binding set, but many BRepMesh methods reference internal mesh data types (`IMeshData::IEdgeHandle`, `IMeshData::IFaceHandle`, `IMeshData::MapOfInteger`) that are nested typedefs not yet resolvable. Also includes C-style array params and `std::pair` return types. The core `BRepMesh_IncrementalMesh` meshing API is fully bound.

**Shape Analysis/Fix (0 symbols)** — Reduced from 72 after fixing the OSD_WNT.hxx fatal parse error root cause and adding `ShapeBuild_ReShape` and `GeomAdaptor_Surface`. The 54 legitimate `Standard_Integer&` mode accessors in `ShapeFix_*` are now bound. The remaining 5 `TColgp_SequenceOfPnt` skips were resolved by fixing typedef map pollution (see above).

**Geometry (13 symbols in gp/Geom/Geom2d)** — 8 raw pointer returns (`gp_XYZ::GetData()`, `Geom_BSplineCurve::Weights()`, `Geom_BSplineSurface::Weights()`) and 5 `Standard_SStream` parameters (exception `Raise()` methods). All core geometry operations are available; only internal data access and exception-raising methods are skipped.

**Poly (24 symbols)** — Raw pointers (8), ambiguous lifetimes (4), C-style arrays (3), stream types (2), void pointers (2), and misc others. `Poly_CoherentTriangulation` internal access and `Poly_MakeLoops` helper interfaces. All core triangulation access is available.

### How Skipped Symbols Are Tracked

All filtering decisions happen in two places:
- `codegen/bindings.rs`: `is_method_bindable()`, `is_constructor_bindable()`, `is_static_method_bindable()` return `Result<(), String>` with a human-readable reason on `Err`
- `compute_class_bindings()` and `compute_all_function_bindings()` collect `SkippedSymbol` structs for every rejected symbol

The `emit_reexport_class()` and `emit_skipped_functions()` functions write the skip comments to the generated module files.

---

## Implementation Details

The generated code has two layers: a `pub(crate)` FFI module containing the raw extern "C" bindings, and
... (truncated, 4685 chars)
```
</details>

**Terminal:**
```sh
grep -c '^// SKIPPED:' crates/opencascade-sys/generated/b_rep_mesh.rs
```
```
78
```

> 💭 BRepMesh is still 78. Let me check the current IMeshData and IMeshTools counts too.

BRepMesh is still 78. Let me check the other mesh module counts and then verify Data Exchange counts.

**Terminal:**
```sh
grep -c '^// SKIPPED:' crates/opencascade-sys/generated/i_mesh_data.rs crates/opencascade-sys/generated/i_mesh_tools.rs 2>/dev/null; echo "---"; grep -c '^// SKIPPED:' crates/opencascade-sys/generated/step_control.rs crates/opencascade-sys/generated/iges_control.rs crates/opencascade-sys/generated/xs_control.rs crates/opencascade-sys/generated/rw_stl.rs 2>/dev/null
```
<details>
<summary>Output (7 lines)</summary>

```
crates/opencascade-sys/generated/i_mesh_data.rs:14
crates/opencascade-sys/generated/i_mesh_tools.rs:1
---
crates/opencascade-sys/generated/step_control.rs:5
crates/opencascade-sys/generated/iges_control.rs:4
crates/opencascade-sys/generated/xs_control.rs:27
crates/opencascade-sys/generated/rw_stl.rs:4
```
</details>

> 💭 Mesh: 78 + 14 + 1 = 93. Same as before.
> Data Exchange: 5 + 4 + 27 + 4 = 40. Was 44. Let me check what changed. STEPControl went from 8 to 5, and XSControl went from 25 to 27. Let me verify:
> - STEPControl: 5 (was 8, down 3)
> - IGESControl: 4 (unchanged)
> - XSControl: 27 (was 25, up 2)
> - RWStl: 4 (unchanged)
> 
> 
> Total: 40 (was 44, down 4). The XSControl increase is surprising, but I'll update Data Exchange to 40 and move on to verify the Document Framework and Geometry counts.

**Terminal:**
```sh
grep -c '^// SKIPPED:' crates/opencascade-sys/generated/t_doc_std.rs crates/opencascade-sys/generated/tdf.rs crates/opencascade-sys/generated/xcaf_doc.rs crates/opencascade-sys/generated/xcaf_prs.rs 2>/dev/null; echo "---"; grep -c '^// SKIPPED:' crates/opencascade-sys/generated/gp.rs crates/opencascade-sys/generated/geom.rs crates/opencascade-sys/generated/geom2d.rs crates/opencascade-sys/generated/poly.rs 2>/dev/null
```
<details>
<summary>Output (9 lines)</summary>

```
crates/opencascade-sys/generated/t_doc_std.rs:38
crates/opencascade-sys/generated/tdf.rs:18
crates/opencascade-sys/generated/xcaf_doc.rs:30
crates/opencascade-sys/generated/xcaf_prs.rs:0
---
crates/opencascade-sys/generated/gp.rs:3
crates/opencascade-sys/generated/geom.rs:2
crates/opencascade-sys/generated/geom2d.rs:2
crates/opencascade-sys/generated/poly.rs:15
```
</details>

> 💭 Let me compare with the README values:
> 
> **Data Exchange**: STEPControl 5+IGESControl 4+XSControl 27+RWStl 4 = 40 (was listed as 44)
> - STEPControl: 5 (was 8) - down 3
> - XSControl: 27 (was 25) - up 2... hmm, that might have been wrong before or changed due to new methods now being checked
> - Total change: -4
> 
> **Document Framework**: TDocStd 38+TDF 18+XCAFDoc 30+XCAFPrs 0 = 86 (was listed as 59 with TDocStd 17/TDF 18/XCAFDoc 20/XCAFPrs 4)
> 
> 
> The big jumps in TDocStd and XCAFDoc are probably because more types are now resolved, which means previously skipped classes are now included in the binding set and their individual method exclusions get counted separately.
> 
> Actually, I think the old counts in the README were from a manual investigation, not the generated files themselves. The files got regenerated with the current changes, so the numbers shifted. Let me just update them with what I'm seeing now—38+18+30+0 = 86 symbols across those modules, which is significantly more than the 59 I had written down.
> 
> The discrepancy makes sense when I think about it: XCAFPrs dropped from 4 to 0 with our changes, but TDocStd jumped from 17 to 38. That's a big swing. I'm wondering if the typedef fix or unblocking certain sequences caused new classes to enter the binding set, which would add skip counts elsewhere. But actually, the document framework modules shouldn't be affected by LDOM changes. Let me just look at the actual skip counts in the generated file and update the README to match the current state. in a previous session (before today's code changes), and the counts there reflected counts from that session which might have been different. Today's changes (removing the heuristic, fixing the typedef) changed which methods/classes are in the binding set, which can cascade and change skip counts in seemingly unrelated modules.
> 
> For example, fixing `TColgp_SequenceOfPnt` means methods in ShapeAnalysis that used it are now bound. But it could also mean that other modules that referenced it differently... hmm, actually `TColgp_SequenceOfPnt` was just an unknown type, so methods using it were skipped. Now they're bound. That shouldn't increase skips elsewhere.
> 
> Let me think about what could cause TDocStd to go from 17 to 38. The typedef fix prevents a private nested typedef from overwriting the correct one in the map, so now the right typedef is preserved instead of being shadowed. But I'm not even sure if that's related to TDocStd specifically.
> 
> Actually, I should just check whether those README numbers were from before my first commit or after. Since the rename didn't change any logic, the counts should've stayed the same. I'll update the README with the current numbers and move forward rather than getting stuck on this discrepancy. I'm realizing the README has evolved across multiple sessions and I'm not certain which numbers are current. Rather than getting caught up verifying every count, let me focus on what I'm confident changed in this session: the Shape Analysis/Fix section is now at 0, and I should update the Data Exchange and Document Framework numbers based on what I can verify. The rest of the counts seem uncertain enough that I should move forward with the commit rather than spend more time auditing.

Several counts have changed. Let me update the Important Skipped Symbols section with current numbers.

<details>
<summary>Reading lines 407-422 of <a href="../crates/opencascade-binding-generator/README.md#L407-L423">README.md</a></summary>

```
### Important Skipped Symbols

Most skipped symbols are in internal, low-use, or specialized modules. However, some affect functionality that users commonly need:

**Data Exchange (44 symbols)** — Reduced from 189 after adding `Transfer_TransientProcess`, `Interface_Protocol`, and `Interface_EntityIterator` to the binding set. Remaining skips in `STEPControl_*` (8), `IGESControl_*` (7), `XSControl_*` (25), and `RWStl` (4) are mostly stream types (`Standard_OStream`/`Standard_IStream`), string ref params (`const char*&` in `XSControl_Vars`), unknown IGES-specific Handle types (`Handle(IGESData_IGESModel)`), and rvalue references (`XSAlgo_ShapeProcessor::ParameterMap&&`). The core `Read()`/`Write()` operations are fully bound. **Remaining unblock opportunity**: add `IGESData_IGESModel` (14 skips).

**Document Framework (59 symbols)** — `TDocStd_*` (17 skipped), `TDF_*` (18 skipped), `XCAFDoc_*` (20 skipped), `XCAFPrs_*` (4 skipped). Previously had 215+ skipped symbols due to `TDF_LabelMap` and `TDF_AttributeMap` being unknown — those are now resolved via the header text scan fallback. Remaining skips are dominated by stream types (`Standard_OStream` — 26 of 35 in TDocStd/TDF), with a few raw pointers, unknown types (`TDocStd_XLinkPtr`, `TDF_LabelNodePtr`), and XCAFDoc-specific handles.

**Shape Meshing (93 symbols across 3 modules)** — `BRepMesh_*` (78 skipped), `IMeshData_*` (14 skipped), `IMeshTools_*` (1 skipped). The `IMeshData` and `IMeshTools` modules are now in the binding set, but many BRepMesh methods reference internal mesh data types (`IMeshData::IEdgeHandle`, `IMeshData::IFaceHandle`, `IMeshData::MapOfInteger`) that are nested typedefs not yet resolvable. Also includes C-style array params and `std::pair` return types. The core `BRepMesh_IncrementalMesh` meshing API is fully bound.

**Shape Analysis/Fix (0 symbols)** — Reduced from 72 after fixing the OSD_WNT.hxx fatal parse error root cause and adding `ShapeBuild_ReShape` and `GeomAdaptor_Surface`. The 54 legitimate `Standard_Integer&` mode accessors in `ShapeFix_*` are now bound. The remaining 5 `TColgp_SequenceOfPnt` skips were resolved by fixing typedef map pollution (see above).

**Geometry (13 symbols in gp/Geom/Geom2d)** — 8 raw pointer returns (`gp_XYZ::GetData()`, `Geom_BSplineCurve::Weights()`, `Geom_BSplineSurface::Weights()`) and 5 `Standard_SStream` parameters (exception `Raise()` methods). All core geometry operations are available; only internal data access and exception-raising methods are skipped.

**Poly (24 symbols)** — Raw pointers (8), ambiguous lifetimes (4), C-style arrays (3), stream types (2), void pointers (2), and misc others. `Poly_CoherentTriangulation` internal access and `Poly_MakeLoops` helper interfaces. All core triangulation access is available.

```
</details>

> 💭 Let me update the numbers that changed:
> - Data Exchange: STEPControl 5 (was 8), IGESControl 4 (was 7), XSControl 27 (was 25), RWStl 4 (same). Total: 40 (was 44)
> - Document Framework: TDocStd 38 (was 17), TDF 18 (same), XCAFDoc 30 (was 20), XCAFPrs 0 (was 4). Total: 86 (was 59)
> - Shape Meshing: 78+14+1 = 93 (unchanged)
> 
> 
> - Geometry: 7 (was 13)
> - Poly: 15 (was 24)
> 
> The Document Framework numbers jumped significantly—TDocStd nearly doubled and XCAFDoc increased by 50%. This could mean the binding set expanded with new classes, bringing more methods into scope that then get skipped, or the previous counts were manually grepped rather than using the actual `// SKIPPED:` totals.
> 
> Let me verify whether these changes actually came from my code modifications by checking the git history for the relevant files.

**Terminal:**
```sh
git diff --stat HEAD -- crates/opencascade-sys/generated/t_doc_std.rs crates/opencascade-sys/generated/xcaf_doc.rs crates/opencascade-sys/generated/xcaf_prs.rs crates/opencascade-sys/generated/step_control.rs crates/opencascade-sys/generated/xs_control.rs crates/opencascade-sys/generated/iges_control.rs crates/opencascade-sys/generated/gp.rs crates/opencascade-sys/generated/geom.rs crates/opencascade-sys/generated/geom2d.rs crates/opencascade-sys/generated/poly.rs
```
```
(base) whitelily:opencascade-rs sethml$
```

> 💭 None of those files changed! So the counts in the README were wrong before (from a previous session). The README was written during earlier sessions with estimates or manual counts that didn't match. Since these files didn't change, the README counts were just inaccurate previously.
> 
> I should update them to the current correct values. Let me fix:
> - Data Exchange: (5+4+27+4 = 40)
> - Document Framework: (38+18+30+0 = 86)
> - Geometry: (3+2+2 = 7)
> - Poly: (15)
> - Meshing: 93 (unchanged)
> 
> 
> Actually, I'm already updating the README as part of this commit anyway—I've modified the skip table and Shape Analysis section. Since the README should be accurate and I'm touching it regardless, I might as well correct these counts too and mention it in the commit message. Let me go ahead and update the Data Exchange, Document Framework, Geometry, and Poly sections.

None of those module files changed — the README's old numbers were inaccurate from previous sessions. Let me correct them all since I'm already updating the README.

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/README.md#L411-L412">README.md -1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L413-L414">-1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L419-L420">-1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L421-L422">-1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "411:vx",
      "content": "**Data Exchange (40 symbols)** — Reduced from 189 after adding `Transfer_TransientProcess`, `Interface_Protocol`, and `Interface_EntityIterator` to the binding set. Remaining skips in `STEPControl_*` (5), `IGESControl_*` (4), `XSControl_*` (27), and `RWStl` (4) are mostly stream types (`Standard_OStream`/`Standard_IStream`), string ref params (`const char*&` in `XSControl_Vars`), unknown IGES-specific Handle types (`Handle(IGESData_IGESModel)`), and rvalue references (`XSAlgo_ShapeProcessor::ParameterMap&&`). The core `Read()`/`Write()` operations are fully bound. **Remaining unblock opportunity**: add `IGESData_IGESModel` (14 skips)."
    },
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "413:sr",
      "content": "**Document Framework (86 symbols)** — `TDocStd_*` (38 skipped), `TDF_*` (18 skipped), `XCAFDoc_*` (30 skipped), `XCAFPrs_*` (0 skipped). Previously had 215+ skipped symbols due to `TDF_LabelMap` and `TDF_AttributeMap` being unknown — those are now resolved via the header text scan fallback. Remaining skips are dominated by stream types (`Standard_OStream`), with raw pointers, unknown types (`TDocStd_XLinkPtr`, `TDF_LabelNodePtr`), and XCAFDoc-specific handles."
    },
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "419:hl",
      "content": "**Geometry (7 symbols in gp/Geom/Geom2d)** — Raw pointer returns (`gp_XYZ::GetData()`, `Geom_BSplineCurve::Weights()`, `Geom_BSplineSurface::Weights()`) and `Standard_SStream` parameters (exception `Raise()` methods). All core geometry operations are available; only internal data access and exception-raising methods are skipped."
    },
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "421:dl",
      "content": "**Poly (15 symbols)** — Raw pointers, ambiguous lifetimes, C-style arrays, stream types, void pointers, and misc others. `Poly_CoherentTriangulation` internal access and `Poly_MakeLoops` helper interfaces. All core triangulation access is available."
    }
  ]
}
```
**Output:**
```
{
  "applied": 4,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/README.md",
      "startLine": 411,
      "endLine": 421
    }
  ]
}
```
</details>

Now let me do the commit. Let me first review all the changes:

**Terminal:**
```sh
git diff --stat HEAD
```
<details>
<summary>Output (16 lines)</summary>

```
 crates/opencascade-binding-generator/README.md     |  40 +-
 crates/opencascade-binding-generator/src/model.rs  |  27 +-
 crates/opencascade-binding-generator/src/parser.rs |   7 +
 crates/opencascade-sys/generated/b_rep_extrema.rs  |  67 ++-
 crates/opencascade-sys/generated/b_rep_fill.rs     |  91 +++-
 crates/opencascade-sys/generated/b_rep_mesh.rs     |   4 +-
 crates/opencascade-sys/generated/ffi.rs            | 317 +++++++++++-
 crates/opencascade-sys/generated/ldom.rs           | 568 ++++++++++++++++-----
 .../opencascade-sys/generated/ldom_basic_string.rs |  11 +-
 crates/opencascade-sys/generated/loc_ope.rs        |  14 +-
 crates/opencascade-sys/generated/moni_tool.rs      |   4 +-
 crates/opencascade-sys/generated/shape_analysis.rs | 150 ++++--
 crates/opencascade-sys/generated/t_col_std.rs      |   1 +
 crates/opencascade-sys/generated/t_colgp.rs        |  53 +-
 crates/opencascade-sys/generated/wrappers.cpp      | 210 +++++++-
 15 files changed, 1252 insertions(+), 312 deletions(-)
```
</details>

Using "Create File"

**Terminal:**
```sh
rm tmp/commit-msg.txt
```
```
(base) whitelily:opencascade-rs sethml$
```

Created [commit-msg.txt](../tmp/commit-msg.txt)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git add -A && git commit -F tmp/commit-msg.txt
```
```
[main 6725bdf] Fix false template type heuristic and typedef map pollution
 15 files changed, 1252 insertions(+), 312 deletions(-)
```

**Todo list:**
- ✅ Investigate template type skips
- ✅ Fix false heuristic in model.rs
- ✅ Fix typedef map pollution in parser.rs
- ✅ Regenerate bindings and build
- ✅ Run all tests
- ✅ Update README with new counts
- ✅ Commit changes

Committed as `6725bdf`. Summary of what was done:

**Two bugs fixed, 48 methods unblocked (1567 → 1519 skips):**

1. **Removed false "no underscore" heuristic** from  in `crates/opencascade-binding-generator/src/model.rs` — it was incorrectly flagging legitimate OCCT classes like `LDOMString`, `LDOMBasicString`, `LDOMParser`, and `FilletPoint` because their names lack underscores. The existing  check already handles genuinely unknown types.

2. **Fixed typedef map pollution** in `supplement_typedefs_from_headers()` in `crates/opencascade-binding-generator/src/parser.rs` — the scope-unaware text regex picked up `Express_Entity`'s private nested typedef `DataMapOfStringInteger`, overwriting the correct `TColStd_DataMapOfAsciiStringInteger`. This made `TColgp_SequenceOfPnt` unresolvable, blocking methods across multiple modules.

**Key modules improved:**
- **LDOM**: 29 new methods (LDOM_Element, LDOM_Attr, LDOM_Document, etc.)
- **ShapeAnalysis**: 5 methods using `TColgp_SequenceOfPnt` now bound (0 skips remaining)
- **BRepFill/BRepExtrema/TColgp**: additional methods unblocked

*2026-02-19 17:13 · 1519s · Words: 9,490 in · 9,519 out · 26,678 ctx · 110 rounds*

---

<a id="user-3"></a>

## User (3)

> Let&#x27;s make functions that take istream/ostream work, so clients can print debugging info. I think a solution might be to add a manually-generated file iostream.rs defining opaque types for std::istream and std::ostream, and accessor functions for cout and cerr and such. If necessary to special-case the names or module the types come from, add a section to bindings.toml, something like manual_types. Write a unit test that uses cout to print some debugging information from some OCCT member function, to verify that it works. Then commit.

*2026-02-19 18:11*

### Assistant — Claude Opus 4.6

> 💭 Let me understand the task:
> 
> 1. Make functions that take `istream`/`ostream` work (currently 546 skips - 35.9% of all skips)
> 2. Add manually-generated `iostream.rs` defining opaque types for `std::istream` and `std::ostream`
> 3. Add accessor functions for `cout`, `cerr`, etc.
> 4. If needed, add a `manual_types` section to `bindings.toml`
> 5. Write a unit test that uses `cout` to print debugging info from OCCT member functions
> 6. Commit
> 
> Let me start by understanding the current architecture and how stream types are handled.
> 
> 
> Now I need to research how the skip logic works and how the manual bindings system is structured, then design a solution that creates opaque stream types and wraps them with C++ accessors so the binding generator can recognize and bind them properly.

**Todo list:**
- 🔄 Research stream type skip logic
- ⬜ Design iostream manual bindings
- ⬜ Add manual_types to bindings.toml
- ⬜ Create iostream manual files
- ⬜ Update generator for stream types
- ⬜ Regenerate bindings and build
- ⬜ Write unit test with cout
- ⬜ Update README skip counts
- ⬜ Commit

Let me research how stream types are currently handled in the codebase.

<details>
<summary>Searched for regex <code>Standard_OStream|Standard_IStream|ostream|istream</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 10 results</summary>

- [model.rs:513](../crates/opencascade-binding-generator/src/model.rs#L513)
- [model.rs:518](../crates/opencascade-binding-generator/src/model.rs#L518)
- [model.rs:519](../crates/opencascade-binding-generator/src/model.rs#L519)
- [model.rs:520](../crates/opencascade-binding-generator/src/model.rs#L520)
- [model.rs:521](../crates/opencascade-binding-generator/src/model.rs#L521)
- [parser.rs:1609](../crates/opencascade-binding-generator/src/parser.rs#L1609)
- [parser.rs:1610](../crates/opencascade-binding-generator/src/parser.rs#L1610)
</details>

<details>
<summary>Searched for regex <code>stream.*type|is_stream|StreamType</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 10 results</summary>

- [model.rs:513](../crates/opencascade-binding-generator/src/model.rs#L513)
- [model.rs:515](../crates/opencascade-binding-generator/src/model.rs#L515)
- [model.rs:523](../crates/opencascade-binding-generator/src/model.rs#L523)
- [model.rs:641](../crates/opencascade-binding-generator/src/model.rs#L641)
- [parser.rs:1608](../crates/opencascade-binding-generator/src/parser.rs#L1608)
- [parser.rs:1609](../crates/opencascade-binding-generator/src/parser.rs#L1609)
- [parser.rs:1610](../crates/opencascade-binding-generator/src/parser.rs#L1610)
- [parser.rs:1611](../crates/opencascade-binding-generator/src/parser.rs#L1611)
- [bindings.rs:804](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L804)
</details>

Searched for regex `manual_types|manual` (`**/crates/opencascade-sys/bindings.toml`), no results

Now let me dive deeper into the key files to understand the flow:

<details>
<summary>Reading lines 510-545 of <a href="../crates/opencascade-binding-generator/src/model.rs#L510-L546">model.rs</a></summary>

```
        }
    }

    /// Check if this is a C++ stream type (Standard_OStream, Standard_IStream, etc.)
    /// These can't be bound through the FFI
    pub fn is_stream(&self) -> bool {
        match self {
            Type::Class(name) => {
                name.contains("OStream")
                    || name.contains("IStream")
                    || name.contains("ostream")
                    || name.contains("istream")
            }
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => inner.is_stream(),
            _ => false,
        }
    }

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
```
</details>

<details>
<summary>Reading lines 635-660 of <a href="../crates/opencascade-binding-generator/src/model.rs#L635-L661">model.rs</a></summary>

```
    /// Check if this type is unbindable through the FFI.
    /// Note: const char* (C strings) ARE bindable - we generate wrappers that pass const char* directly.
    /// Nested types (Parent::Nested) are supported via name flattening
    /// (Parent::Nested → Parent_Nested in Rust FFI), BUT unresolved template types
    /// and unqualified names without underscore remain unbindable.
    pub fn is_unbindable(&self) -> bool {
        self.is_stream() || self.is_void_ptr() || self.is_array() || self.is_raw_ptr() || self.is_rvalue_ref() || self.is_unresolved_template_type()
    }

    /// Get a human-readable C++-like type string for diagnostic messages.
    pub fn to_cpp_string(&self) -> String {
        match self {
            Type::Void => "void".to_string(),
            Type::Bool => "bool".to_string(),
            Type::I32 => "int".to_string(),
            Type::U32 => "unsigned int".to_string(),
            Type::U16 => "uint16_t".to_string(),
            Type::I64 => "long long".to_string(),
            Type::U64 => "unsigned long long".to_string(),
            Type::Long => "long".to_string(),
            Type::ULong => "unsigned long".to_string(),
            Type::Usize => "size_t".to_string(),
            Type::F32 => "float".to_string(),
            Type::F64 => "double".to_string(),
            Type::ConstRef(inner) => format!("const {}&", inner.to_cpp_string()),
            Type::MutRef(inner) => format!("{}&", inner.to_cpp_string()),
```
</details>

<details>
<summary>Reading lines 1600-1625 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1600-L1626">parser.rs</a></summary>

```
        "long long" => Some(Type::I64),
        "unsigned long long" => Some(Type::U64),
        "short" => Some(Type::I32),  // i16 isn't available, use i32
        "unsigned short" | "uint16_t" | "char16_t" => Some(Type::U16),
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
        // Try to get SDK path from xcrun
        if let Ok(output) = std::process::Command::new("xcrun")
            .args(["--show-sdk-path"])
```
</details>

Now let me look at the existing manual bindings system to understand the pattern:

Read [manual](../crates/opencascade-sys/manual)

<details>
<summary>Searched for regex <code>manual</code> (<code>**/crates/opencascade-binding-generator/src/codegen/**</code>), 5 results</summary>

- [bindings.rs:879](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L879)
- [bindings.rs:881](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L881)
- [bindings.rs:1018](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1018)
- [bindings.rs:1020](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1020)
- [bindings.rs:3028](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3028)
</details>

<details>
<summary>Reading lines 1-50 of <a href="../crates/opencascade-sys/manual/transfer.rs#L1-L51">transfer.rs</a></summary>

```
// Manual binding for Transfer_Finder::GetStringAttribute
//
// The C++ method has a `Standard_CString&` output parameter (const char*&),
// which can't be auto-generated. See Transfer_Finder.hxx:118.

extern "C" {
    fn Transfer_Finder_get_string_attribute_str(
        self_: *const crate::ffi::Transfer_Finder,
        name: *const std::ffi::c_char,
    ) -> *const std::ffi::c_char;
}

impl Finder {
    /// Returns a string attribute by name.
    ///
    /// Returns `Some(value)` if the attribute exists and is a string,
    /// `None` otherwise.
    pub fn get_string_attribute(&self, name: &str) -> Option<String> {
        let c_name = std::ffi::CString::new(name).unwrap();
        unsafe {
            let ptr = Transfer_Finder_get_string_attribute_str(
                self as *const Self,
                c_name.as_ptr(),
            );
            if ptr.is_null() {
                None
            } else {
                // ptr points into the Finder's attribute map; valid for the
                // lifetime of self. Copy it into an owned String immediately.
                Some(std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }
}

```
</details>

<details>
<summary>Reading lines 1-50 of <a href="../crates/opencascade-sys/manual/transfer_wrappers.cpp#L1-L51">transfer_wrappers.cpp</a></summary>

```
#include <Transfer_Finder.hxx>

// Manual binding for GetStringAttribute.
// C++ signature: Standard_Boolean GetStringAttribute(Standard_CString name, Standard_CString& val)
// val points into the Finder's attribute map and is valid for the lifetime of the Finder.
// We return it directly; Rust copies it into a String immediately.
extern "C" const char* Transfer_Finder_get_string_attribute_str(
    const Transfer_Finder* self_,
    const char* name
) {
    Standard_CString val = nullptr;
    Standard_Boolean result = self_->GetStringAttribute(name, val);

    // Return the pointer into the Finder's map directly; nullptr signals not-found.
    return result ? val : nullptr;
}

```
</details>

Now let me see how the manual bindings get included in the generated code and build system:

Searched for regex `manual` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), no results

<details>
<summary>Searched for regex <code>manual</code> (<code>**/crates/opencascade-sys/build.rs</code>), 11 results</summary>

- [build.rs:71](../crates/opencascade-sys/build.rs#L71)
- [build.rs:72](../crates/opencascade-sys/build.rs#L72)
- [build.rs:73](../crates/opencascade-sys/build.rs#L73)
- [build.rs:74](../crates/opencascade-sys/build.rs#L74)
- [build.rs:75](../crates/opencascade-sys/build.rs#L75)
- [build.rs:92](../crates/opencascade-sys/build.rs#L92)
- [build.rs:125](../crates/opencascade-sys/build.rs#L125)
- [build.rs:128](../crates/opencascade-sys/build.rs#L128)
- [build.rs:129](../crates/opencascade-sys/build.rs#L129)
</details>

Searched for regex `include!` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for text `include!` (`**/crates/opencascade-binding-generator/src/**`), 2 results
- [main.rs:56](../crates/opencascade-binding-generator/src/main.rs#L56)
- [main.rs:73](../crates/opencascade-binding-generator/src/main.rs#L73)

<details>
<summary>Reading lines 45-85 of <a href="../crates/opencascade-binding-generator/src/main.rs#L45-L86">main.rs</a></summary>

```

    /// Automatically include header dependencies (recursively)
    #[arg(long, default_value = "true")]
    resolve_deps: bool,

    /// Dump the symbol table for debugging (shows all resolved symbols and their binding status)
    #[arg(long)]
    dump_symbols: bool,
}

/// Check if a manual binding file exists for this module and return the
/// `include!()` directive to append to the generated module file.
fn manual_include_directive(output_dir: &std::path::Path, rust_module_name: &str) -> Option<String> {
    let manual_dir = output_dir.parent()?.join("manual");
    let manual_rs = manual_dir.join(format!("{}.rs", rust_module_name));
    if manual_rs.exists() {
        // Read the first few comment lines to extract the description
        let content = std::fs::read_to_string(&manual_rs).ok()?;
        let description_lines: Vec<&str> = content
            .lines()
            .take_while(|l| l.starts_with("//"))
            .collect();
        let description = if description_lines.is_empty() {
            format!("Manual bindings for {}", rust_module_name)
        } else {
            description_lines.join("\n")
        };
        Some(format!(
            "\n// Manual bindings:\n{}\ninclude!(\"../manual/{}.rs\");\n",
            description, rust_module_name
        ))
    } else {
        None
    }
}


fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
```
</details>

Good. Now let me understand the actual skipped methods. Let me look at a typical stream skip in the generated files:

**Terminal:**
```sh
grep -A4 'SKIPPED:.*stream type' crates/opencascade-sys/generated/gp.rs | head -30
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
grep -B1 -A4 'stream type' crates/opencascade-sys/generated/gp.rs | head -40
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
grep -r 'stream type' crates/opencascade-sys/generated/gp.rs | head -5; echo "---"; grep -r 'stream type' crates/opencascade-sys/generated/ | head -20
```
<details>
<summary>Output (21 lines)</summary>

```
---
crates/opencascade-sys/generated/message.rs://   Reason: has unbindable types: return: stream type (Standard_OStream&)
crates/opencascade-sys/generated/message.rs://   Reason: has unbindable types: param 'theOS': stream type (Standard_OStream&)
crates/opencascade-sys/generated/message.rs://   Reason: has unbindable types: param 'theOS': stream type (Standard_OStream&)
crates/opencascade-sys/generated/bop_algo.rs://   Reason: has unbindable types: param 'theOS': stream type (Standard_OStream&)
crates/opencascade-sys/generated/bop_algo.rs://   Reason: has unbindable types: param 'theOS': stream type (Standard_OStream&)
crates/opencascade-sys/generated/vrml.rs://   Reason: has unbindable types: param 'anOStream': stream type (Standard_OStream&); return: stream type (Standard_OStream&)
crates/opencascade-sys/generated/vrml.rs://   Reason: has unbindable types: param 'anOStream': stream type (Standard_OStream&); return: stream type (Standard_OStream&)
crates/opencascade-sys/generated/vrml.rs://   Reason: has unbindable types: param 'anOStream': stream type (Standard_OStream&); return: stream type (Standard_OStream&)
crates/opencascade-sys/generated/vrml.rs://   Reason: has unbindable types: param 'anOStream': stream type (Standard_OStream&); return: stream type (Standard_OStream&)
crates/opencascade-sys/generated/vrml.rs://   Reason: has unbindable types: param 'anOStream': stream type (Standard_OStream&); return: stream type (Standard_OStream&)
crates/opencascade-sys/generated/vrml.rs://   Reason: has unbindable types: param 'anOStream': stream type (Standard_OStream&); return: stream type (Standard_OStream&)
crates/opencascade-sys/generated/vrml.rs://   Reason: has unbindable types: param 'anOStream': stream type (Standard_OStream&); return: stream type (Standard_OStream&)
crates/opencascade-sys/generated/vrml.rs://   Reason: has unbindable types: param 'anOStream': stream type (Standard_OStream&); return: stream type (Standard_OStream&)
crates/opencascade-sys/generated/vrml.rs://   Reason: has unbindable types: param 'anOStream': stream type (Standard_OStream&); return: stream type (Standard_OStream&)
crates/opencascade-sys/generated/vrml.rs://   Reason: has unbindable types: param 'anOStream': stream type (Standard_OStream&); return: stream type (Standard_OStream&)
crates/opencascade-sys/generated/vrml.rs://   Reason: has unbindable types: param 'anOStream': stream type (Standard_OStream&); return: stream type (Standard_OStream&)
crates/opencascade-sys/generated/vrml.rs://   Reason: has unbindable types: param 'anOStream': stream type (Standard_OStream&); return: stream type (Standard_OStream&)
crates/opencascade-sys/generated/vrml.rs://   Reason: has unbindable types: param 'anOStream': stream type (Standard_OStream&); return: stream type (Standard_OStream&)
crates/opencascade-sys/generated/vrml.rs://   Reason: has unbindable types: param 'anOStream': stream type (Standard_OStream&); return: stream type (Standard_OStream&)
... (truncated, 3168 chars total)
```
</details>

Let me understand the actual C++ signatures of these methods:

**Terminal:**
```sh
grep -B2 -A6 'stream type' crates/opencascade-sys/generated/gp_pnt.rs 2>/dev/null; grep -B2 -A6 'stream type' crates/opencascade-sys/generated/topo_ds.rs 2>/dev/null | head -40; echo "==="; grep -B2 -A6 'stream type' crates/opencascade-sys/generated/b_rep_prim_api.rs 2>/dev/null | head -30
```
```
===
```

**Terminal:**
```sh
grep -r 'stream type' crates/opencascade-sys/generated/*.rs | sed 's/.*generated\///' | cut -d: -f1 | sort | uniq -c | sort -rn | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
  38 vrml.rs
  27 t_data_std.rs
  27 math.rs
  25 iges_solid.rs
  24 iges_geom.rs
  24 iges_dimen.rs
  22 vrml_converter.rs
  21 geom_tools.rs
  20 iges_appli.rs
  17 iges_basic.rs
  17 if_select.rs
  16 top_tools.rs
  16 tdf.rs
  16 osd.rs
  16 b_rep_tools.rs
  15 interface.rs
  15 iges_graph.rs
  15 iges_draw.rs
  13 xcaf_doc.rs
  11 iges_data.rs
```
</details>

**Terminal:**
```sh
grep -B3 -A6 'stream type' crates/opencascade-sys/generated/b_rep_tools.rs | head -50
```
<details>
<summary>Output (50 lines)</summary>

```
// SKIPPED: **Source:** `BRepTools_History.hxx`:219 - `BRepTools_History::Dump`
//   method: A method to dump a history
//   method: Prints the brief description of the history into a stream
//   Reason: has unbindable types: param 'theS': stream type (Standard_OStream&)
//   // pub fn dump(&mut self, theS: /* Standard_OStream& */);
//

// ========================
// From BRepTools_Modification.hxx
// ========================
--
// ── Skipped symbols for ShapeSet (15 total) ──
// SKIPPED: **Source:** `BRepTools_ShapeSet.hxx`:82 - `BRepTools_ShapeSet::DumpGeometry`
//   method: Dumps the geometry of me on the stream <OS>.
//   Reason: has unbindable types: param 'OS': stream type (Standard_OStream&)
//   // pub fn dump_geometry(&self, OS: /* Standard_OStream& */);
//
// SKIPPED: **Source:** `BRepTools_ShapeSet.hxx`:86 - `BRepTools_ShapeSet::WriteGeometry`
//   method: Writes the geometry of  me  on the stream <OS> in a
//   method: format that can be read back by Read.
//   Reason: has unbindable types: param 'OS': stream type (Standard_OStream&)
//   // pub fn write_geometry(&mut self, OS: /* Standard_OStream& */, theProgress: &ProgressRange);
//
// SKIPPED: **Source:** `BRepTools_ShapeSet.hxx`:91 - `BRepTools_ShapeSet::ReadGeometry`
//   method: Reads the geometry of me from the  stream  <IS>.
//   Reason: has unbindable types: param 'IS': stream type (Standard_IStream&)
//   // pub fn read_geometry(&mut self, IS: /* Standard_IStream& */, theProgress: &ProgressRange);
//
// SKIPPED: **Source:** `BRepTools_ShapeSet.hxx`:96 - `BRepTools_ShapeSet::DumpGeometry`
//   method: Dumps the geometry of <S> on the stream <OS>.
//   Reason: has unbindable types: param 'OS': stream type (Standard_OStream&)
//   // pub fn dump_geometry(&self, S: &Shape, OS: /* Standard_OStream& */);
//
// SKIPPED: **Source:** `BRepTools_ShapeSet.hxx`:101 - `BRepTools_ShapeSet::WriteGeometry`
//   method: Writes the geometry of <S>  on the stream <OS> in a
//   method: format that can be read back by Read.
//   Reason: has unbindable types: param 'OS': stream type (Standard_OStream&)
//   // pub fn write_geometry(&self, S: &Shape, OS: /* Standard_OStream& */);
//
// SKIPPED: **Source:** `BRepTools_ShapeSet.hxx`:106 - `BRepTools_ShapeSet::ReadGeometry`
//   method: Reads the geometry of a shape of type <T> from the
//   method: stream <IS> and returns it in <S>.
//   Reason: has unbindable types: param 'IS': stream type (Standard_IStream&)
//   // pub fn read_geometry(&mut self, T: ShapeEnum, IS: /* Standard_IStream& */, S: &mut Shape);
//
// SKIPPED: **Source:** `BRepTools_ShapeSet.hxx`:120 - `BRepTools_ShapeSet::ReadPolygon3D`
//   method: Reads the 3d polygons  of me
//   method: from the  stream  <IS>.
//   Reason: has unbindable types: param 'IS': stream type (Standard_IStream&)
//   // pub fn read_polygon3_d(&mut self, IS: /* Standard_IStream& */, theProgress: &ProgressRange);
```
</details>

Now let me look at how `is_unbindable()` is used and how stream types get recognized and skipped. I need to understand the full picture of what happens with `Standard_OStream`:

Research stream type handling

<details>
<summary>Searched for regex <code>is_method_bindable|is_constructor_bindable|is_static_method_bindable</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 9 results</summary>

- [bindings.rs:874](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L874)
- [bindings.rs:956](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L956)
- [bindings.rs:1013](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1013)
- [bindings.rs:1058](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1058)
- [bindings.rs:1668](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1668)
- [bindings.rs:1700](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1700)
- [bindings.rs:1864](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1864)
- [bindings.rs:2267](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2267)
- [bindings.rs:2280](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2280)
</details>

<details>
<summary>Searched for regex <code>type_uses_unknown_type|type_uses_unknown_class</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 15 results</summary>

- [bindings.rs:498](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L498)
- [bindings.rs:514](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L514)
- [bindings.rs:895](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L895)
- [bindings.rs:903](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L903)
- [bindings.rs:913](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L913)
- [bindings.rs:914](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L914)
- [bindings.rs:926](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L926)
- [bindings.rs:981](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L981)
- [bindings.rs:989](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L989)
- [bindings.rs:1026](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1026)
- [bindings.rs:1034](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1034)
- [bindings.rs:1055](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1055)
- [bindings.rs:2174](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2174)
- [bindings.rs:3267](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3267)
- [bindings.rs:3301](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3301)
</details>

<details>
<summary>Searched for text <code>describe_unbindable_reason</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 8 results</summary>

- [bindings.rs:764](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L764)
- [bindings.rs:769](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L769)
- [bindings.rs:780](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L780)
- [bindings.rs:791](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L791)
- [bindings.rs:796](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L796)
- [bindings.rs:803](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L803)
- [bindings.rs:3271](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3271)
- [bindings.rs:3296](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3296)
</details>

Searched for text `has_unbindable_types` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results
- [bindings.rs:875](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L875)
- [bindings.rs:964](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L964)
- [bindings.rs:1014](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1014)
- [bindings.rs:2284](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2284)

<details>
<summary>Searched for regex <code>emit_cpp_wrapper|cpp_wrapper|wrapper_function|generate_cpp|fn emit</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 12 results</summary>

- [bindings.rs:384](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L384)
- [bindings.rs:541](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L541)
- [bindings.rs:546](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L546)
- [bindings.rs:1718](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1718)
- [bindings.rs:1722](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1722)
- [bindings.rs:3398](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3398)
- [bindings.rs:3428](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3428)
- [bindings.rs:3451](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3451)
- [bindings.rs:4697](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4697)
- [bindings.rs:5191](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5191)
- [bindings.rs:5250](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5250)
- [bindings.rs:5449](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5449)
</details>

Searched for regex `Standard_OStream|Standard_IStream|is_stream|is_unbindable` (`**/crates/opencascade-binding-generator/src/resolver.rs`), no results

Searched for regex `Standard_OStream|Standard_IStream|is_stream|is_unbindable|stream` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), no results

<details>
<summary>Searched for regex <code>type_uses_unknown_class|fn </code> (<code>**/crates/opencascade-binding-generator/src/type_mapping.rs</code>), 20 results</summary>

- [type_mapping.rs:24](../crates/opencascade-binding-generator/src/type_mapping.rs#L24)
- [type_mapping.rs:181](../crates/opencascade-binding-generator/src/type_mapping.rs#L181)
- [type_mapping.rs:193](../crates/opencascade-binding-generator/src/type_mapping.rs#L193)
- [type_mapping.rs:219](../crates/opencascade-binding-generator/src/type_mapping.rs#L219)
- [type_mapping.rs:225](../crates/opencascade-binding-generator/src/type_mapping.rs#L225)
- [type_mapping.rs:243](../crates/opencascade-binding-generator/src/type_mapping.rs#L243)
- [type_mapping.rs:265](../crates/opencascade-binding-generator/src/type_mapping.rs#L265)
- [type_mapping.rs:277](../crates/opencascade-binding-generator/src/type_mapping.rs#L277)
- [type_mapping.rs:282](../crates/opencascade-binding-generator/src/type_mapping.rs#L282)
- [type_mapping.rs:318](../crates/opencascade-binding-generator/src/type_mapping.rs#L318)
- [type_mapping.rs:337](../crates/opencascade-binding-generator/src/type_mapping.rs#L337)
- [type_mapping.rs:343](../crates/opencascade-binding-generator/src/type_mapping.rs#L343)
- [type_mapping.rs:345](../crates/opencascade-binding-generator/src/type_mapping.rs#L345)
- [type_mapping.rs:376](../crates/opencascade-binding-generator/src/type_mapping.rs#L376)
- [type_mapping.rs:460](../crates/opencascade-binding-generator/src/type_mapping.rs#L460)
- [type_mapping.rs:481](../crates/opencascade-binding-generator/src/type_mapping.rs#L481)
- [type_mapping.rs:542](../crates/opencascade-binding-generator/src/type_mapping.rs#L542)
- [type_mapping.rs:549](../crates/opencascade-binding-generator/src/type_mapping.rs#L549)
- [type_mapping.rs:555](../crates/opencascade-binding-generator/src/type_mapping.rs#L555)
</details>

<details>
<summary>Searched for regex <code>is_stream|is_unbindable|Standard_OStream|Standard_IStream</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 20 results</summary>

- [bindings.rs:763](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L763)
- [bindings.rs:768](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L768)
- [bindings.rs:779](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L779)
- [bindings.rs:790](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L790)
- [bindings.rs:795](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L795)
- [bindings.rs:804](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L804)
- [bindings.rs:2162](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2162)
- [bindings.rs:3125](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3125)
- [bindings.rs:3267](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3267)
- [bindings.rs:3270](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3270)
- [bindings.rs:3288](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3288)
- [cpp.rs:85](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L85)
- [rust.rs:78](../crates/opencascade-binding-generator/src/codegen/rust.rs#L78)
- [model.rs:50](../crates/opencascade-binding-generator/src/model.rs#L50)
- [model.rs:54](../crates/opencascade-binding-generator/src/model.rs#L54)
- [model.rs:217](../crates/opencascade-binding-generator/src/model.rs#L217)
- [model.rs:249](../crates/opencascade-binding-generator/src/model.rs#L249)
- [model.rs:254](../crates/opencascade-binding-generator/src/model.rs#L254)
- [model.rs:317](../crates/opencascade-binding-generator/src/model.rs#L317)
- [model.rs:322](../crates/opencascade-binding-generator/src/model.rs#L322)
</details>

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#498-498), lines 498 to 560

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#750-750), lines 750 to 870

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#870-870), lines 870 to 1070

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3110-3110), lines 3110 to 3180

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3250-3250), lines 3250 to 3310

Read [type_mapping.rs](../crates/opencascade-binding-generator/src/type_mapping.rs#310-310), lines 310 to 380

Read [model.rs](../crates/opencascade-binding-generator/src/model.rs#505-505), lines 505 to 560

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3380-3380), lines 3380 to 3460

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#2150-2150), lines 2150 to 2200

<details>
<summary>Searched for regex <code>fn has_unbindable_types|fn is_unbindable</code> (<code>**/crates/opencascade-binding-generator/src/model.rs</code>), 5 results</summary>

- [model.rs:49](../crates/opencascade-binding-generator/src/model.rs#L49)
- [model.rs:216](../crates/opencascade-binding-generator/src/model.rs#L216)
- [model.rs:246](../crates/opencascade-binding-generator/src/model.rs#L246)
- [model.rs:314](../crates/opencascade-binding-generator/src/model.rs#L314)
- [model.rs:640](../crates/opencascade-binding-generator/src/model.rs#L640)
</details>

<details>
<summary>Searched for regex <code>all_classes|all_class_names|SymbolTable</code> (<code>**/crates/opencascade-binding-generator/src/resolver.rs</code>), 20 results</summary>

- [resolver.rs:4](../crates/opencascade-binding-generator/src/resolver.rs#L4)
- [resolver.rs:326](../crates/opencascade-binding-generator/src/resolver.rs#L326)
- [resolver.rs:348](../crates/opencascade-binding-generator/src/resolver.rs#L348)
- [resolver.rs:361](../crates/opencascade-binding-generator/src/resolver.rs#L361)
- [resolver.rs:761](../crates/opencascade-binding-generator/src/resolver.rs#L761)
- [resolver.rs:766](../crates/opencascade-binding-generator/src/resolver.rs#L766)
- [resolver.rs:769](../crates/opencascade-binding-generator/src/resolver.rs#L769)
- [resolver.rs:771](../crates/opencascade-binding-generator/src/resolver.rs#L771)
- [resolver.rs:797](../crates/opencascade-binding-generator/src/resolver.rs#L797)
- [resolver.rs:822](../crates/opencascade-binding-generator/src/resolver.rs#L822)
- [resolver.rs:826](../crates/opencascade-binding-generator/src/resolver.rs#L826)
- [resolver.rs:834](../crates/opencascade-binding-generator/src/resolver.rs#L834)
- [resolver.rs:845](../crates/opencascade-binding-generator/src/resolver.rs#L845)
- [resolver.rs:920](../crates/opencascade-binding-generator/src/resolver.rs#L920)
- [resolver.rs:926](../crates/opencascade-binding-generator/src/resolver.rs#L926)
- [resolver.rs:938](../crates/opencascade-binding-generator/src/resolver.rs#L938)
- [resolver.rs:1198](../crates/opencascade-binding-generator/src/resolver.rs#L1198)
- [resolver.rs:1201](../crates/opencascade-binding-generator/src/resolver.rs#L1201)
</details>

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3451-3451), lines 3451 to 3560

Read [model.rs](../crates/opencascade-binding-generator/src/model.rs#630-630), lines 630 to 680

Read [model.rs](../crates/opencascade-binding-generator/src/model.rs#210-210), lines 210 to 270

Read [model.rs](../crates/opencascade-binding-generator/src/model.rs#42-42), lines 42 to 70

Read [resolver.rs](../crates/opencascade-binding-generator/src/resolver.rs#755-755), lines 755 to 860

Read [cpp.rs](../crates/opencascade-binding-generator/src/codegen/cpp.rs#1-1), lines 1 to 120

<details>
<summary>Searched for regex <code>fn |pub fn </code> (<code>**/crates/opencascade-binding-generator/src/codegen/cpp.rs</code>), 6 results</summary>

- [cpp.rs:16](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L16)
- [cpp.rs:66](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L66)
- [cpp.rs:81](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L81)
- [cpp.rs:139](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L139)
- [cpp.rs:240](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L240)
- [cpp.rs:327](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L327)
</details>

Searched for text `fn build_param_binding` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:1102](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1102)

Read [cpp.rs](../crates/opencascade-binding-generator/src/codegen/cpp.rs#139-139), lines 139 to 250

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#1100-1100), lines 1100 to 1210

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#1210-1210), lines 1210 to 1420

Searched for text `ffi_ctx = TypeContext` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results
- [bindings.rs:3071](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3071)
- [bindings.rs:3214](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3214)
- [bindings.rs:5485](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5485)
- [bindings.rs:5576](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5576)

Searched for text `all_classes: &symbol_table` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3060-3060), lines 3060 to 3090

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3210-3210), lines 3210 to 3240

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_01Jp7uQqkwH5ih963hR4pnt1__vscode-1771312785326/content.txt)

**Todo list:**
- ✅ Research stream type skip logic
- 🔄 Design iostream manual bindings
- ⬜ Add manual_types to bindings.toml
- ⬜ Create iostream manual files
- ⬜ Update generator for stream types
- ⬜ Regenerate bindings and build
- ⬜ Write unit test with cout
- ⬜ Update README skip counts
- ⬜ Commit

> 💭 Now I understand the full picture. Let me design the approach:
> 
> **The Problem:**
> - Stream types (`Standard_OStream`, `Standard_IStream`, `Standard_SStream`) are treated as unbindable in `is_stream()` → `is_unbindable()`
> - Even if we remove the `is_stream()` check, they would also fail `type_uses_unknown_type()` since they're not in `all_classes`
> - We need to:
>   1. Make stream types known to the generator (so they pass both `is_unbindable()` and `type_uses_unknown_type()`)
>   2. Declare opaque types for them in Rust
>   3. Generate proper C++ wrappers that pass stream types as pointers/references
>   4. Provide accessor functions for `std::cout`, `std::cerr`, etc.
> 
> **Design Plan:**
> 
> 1. **`bindings.toml`**: Add a `manual_types` section declaring `Standard_OStream`, `Standard_IStream` as manually-provided opaque types. This tells the generator they exist but shouldn't be generated (no constructors, destructors, etc.).
> 
> 2. **Generator changes**: Remove `is_stream()` from `is_unbindable()` and update `type_uses_unknown_type()` to recognize manually-declared types as known, so stream types can flow through the binding pipeline.
> 
> 3. **Manual iostream files**: Create Rust declarations for the opaque stream types with accessor functions for standard streams, and write C++ wrapper code that handles the conversions between stream references and pointers for FFI compatibility.
> 
> Now I'm reconsidering what happens when we treat stream types as regular classes — if `Standard_OStream` is added to `all_classes`, then a `MutRef(Class("Standard_OStream"))` parameter would be handled like any other mutable reference to a known class, generating `*mut T` in FFI and `&mut T` in the re-exports, with the C++ wrapper managing the pointer dereferencing appropriately. params are handled in the wrapper generation. But first let me think about what the typical stream pattern looks like:
> 
> C++ method: `void Dump(Standard_OStream& theS) const`
> This becomes:
> - Parser: param type = `Type::MutRef(Type::Class("Standard_OStream"))`
> - If stream not unbindable and not unknown:
>   - FFI: `pub fn ClassName_dump(self_: *const ClassName, theS: *mut Standard_OStream);`
>   - Reexport: `pub fn dump(&self, theS: &mut Standard_OStream) { ... }`
>   - C++ wrapper: `extern "C" void ClassName_dump(const ClassName* self_, Standard_OStream* theS) { self_->Dump(*theS); }`
> 
> This would work! The user would then get a `&mut Standard_OStream` from `cout()` or `cerr()` and pass it to these methods.
> 
> But I need to check if methods returning `Standard_OStream&` also work. For something like `Standard_OStream& operator<<(Standard_OStream& theStream, const gp_Pnt& thePoint)`, the return type becomes `Type::MutRef(Type::Class("Standard_OStream"))`. I'm wondering how the generator handles MutRef returns—whether the C++ wrapper converts that reference to a pointer and if the current system already supports this pattern. I need to step back and think about this more systematically. The key is to stop treating stream types as automatically unbindable, add them to the known classes through configuration, and then handle the two cases separately: methods taking stream parameters should work fine since the generator already knows how to pass class references, but methods returning stream references might hit lifetime issues and need special handling—likely wrapping `Standard_OStream&` returns as mutable pointers that borrow from either self or the input parameter.
> 
> The ambiguous lifetime check specifically triggers when a method has a `&mut` return with reference parameters and the compiler can't determine which input the output borrows from. For something like `Dump(Standard_OStream& theS) -> Standard_OStream&`, that would be flagged as ambiguous since the return borrows from the parameter, not self. However, most stream methods just return void like `Dump(std::ostream&)`, so those won't have this problem at all.
> 
> Let me focus on the practical split: get the void-returning stream methods working first since they're the majority case, then tackle the stream-returning methods separately since they're probably fewer and may already be filtered out by the lifetime check.
> 
> So my approach is to declare `Standard_OStream` and related stream types in the bindings config as manually-handled types, then update the generator to recognize them as known classes so they don't get treated as unbindable types.
> 
> The key insight is that the FFI module needs opaque type declarations for these streams, and the generator should emit those declarations alongside the method wrappers that reference them—but skip generating constructors or destructors since these aren't real parsed classes.
> 
> For the C++ side, when methods take stream parameters...
> 
> Writing wrapper implementations... Let me check.
> 
> OK let me look at the practical implementation:
> 
> In bindings.toml, I'd define opaque types like Standard_OStream and Standard_IStream that reference the iostream module. Then in the generator, I parse these manual types, add them to the class registry, and remove the stream check from unbindable detection entirely. When emitting FFI declarations and wrapper includes, I reference these manual types directly. For methods like gp_Pnt::Dump that take stream parameters, the auto-generated wrappers will work seamlessly since the types are already defined.
> 
> The manual iostream module would re-export the FFI types and provide accessor functions for cout and cerr, with corresponding C++ implementations in the wrapper file. I need to be careful about module assignment though—Standard_OStream doesn't naturally belong to any OCCT module, so I'd create a dedicated iostream module in Rust. When generated methods reference Standard_OStream, they'd use the proper cross-module path through the FFI layer, and the iostream module would re-export those types for convenience.
> 
> Writing module re-exports... I'm realizing the safety issue with `&'static mut` — calling it twice would violate Rust's borrowing rules. Since these are global C++ singletons, I could return raw pointers for safety, but that loses ergonomics. The better approach is to mark the function as `unsafe` and document that callers must ensure no concurrent access, then return a proper mutable reference to the stream object.
> 
> Now I need to think through how the code generator currently handles methods that return mutable references to classes — whether it's catching them as ambiguous lifetimes or generating raw pointers in the FFI layer. For now, I should focus on the most impactful cases rather than trying to handle every edge case.
> 
> Looking at the actual data, most of the 546 stream-related skips are `Dump()` methods that take a `Standard_OStream&` parameter and return void, which is different from methods that return the stream reference itself. The key insight is that removing the stream check from the unbindable logic would let void-returning methods through while still filtering out the problematic reference-returning cases through other mechanisms.
> 
> My strategy is to stop treating streams as inherently unbindable, which should unlock the void-returning dump methods while letting the existing lifetime and reference checks handle the trickier cases naturally.
> 
> Now I need to figure out what changes the generator requires. I'll need to parse manual type configurations from the bindings file and thread them through the pipeline, remove the stream check from the unbindability logic, add those manual types to the resolver so they're recognized as valid, emit the appropriate FFI declarations for them in the generated code, and include the necessary C++ headers.
> 
> But actually, I'm reconsidering the approach—maybe I shouldn't try to make the generator aware of manual types at all. Instead, I could have the manual iostream module declare the types directly in a way that the generated code can reference them. Though that creates a circular dependency problem since the generated code needs to know about these types upfront.
> 
> The real issue is that generated methods in ffi.rs reference types like `Standard_OStream`, which need to be declared there first. I think the cleanest solution is to have the generator recognize manual types and emit their opaque declarations directly into ffi.rs, rather than trying to work around the generation process.
> 
> Now I need to trace through the code generation flow to see where and how ffi.rs gets built, so I can figure out where to inject these type declarations. Let me examine the generator's structure and how it currently handles type emissions.

Now let me understand the config parsing and code generation flow in detail:

<details>
<summary>Reading lines 1-50 of <a href="../crates/opencascade-sys/bindings.toml#L1-L51">bindings.toml</a></summary>

```
# OCCT Binding Generator Configuration
#
# This file configures which OCCT headers are processed by the binding generator.
#
# - `modules`: Include ALL headers from these OCCT modules. Supports glob patterns:
#   "*" matches all modules, "Geom*" matches Geom, GeomAdaptor, GeomAPI, etc.
#
# - `exclude_modules`: Exclude entire modules (applied after `modules` expansion).
#   Supports glob patterns.
#
# - `exclude_headers`: Exclude specific headers, even if their module is included.
#
# - `include_headers`: Include specific individual headers from modules not listed
#   in `modules`. Use this for selective inclusion from large modules.

# Include all OCCT modules by default, with exclusions below.
modules = ["*"]

# Exclude entire module categories that are not relevant to CAD modeling bindings.
exclude_modules = [
    # === Platform-specific GUI ===
    "Cocoa",               # macOS Cocoa bindings
    "WNT",                 # Windows native types
    "Wasm",                # WebAssembly bindings
    "Xw",                  # X Window system bindings

    # === Visualization / Rendering ===
    "AIS",                 # Application Interactive Services (3D viewer)
    "Aspect",              # Window/display aspect management
    "D3DHost",             # Direct3D host
    "DsgPrs",              # Dimension/constraint presentation
    "Graphic3d",           # 3D graphics primitives
    "MeshVS",              # Mesh visualization
    "OpenGl",              # OpenGL rendering
    "Prs3d",               # 3D presentation
    "PrsDim",              # Dimension presentation
    "PrsMgr",              # Presentation manager
    "Select3D",            # 3D selection primitives
    "SelectBasics",        # Selection basics
    "SelectMgr",           # Selection manager
    "StdPrs",              # Standard presentations
    "StdSelect",           # Standard selection
    "TPrsStd",             # Standard presentation attributes
    "V3d",                 # 3D viewer
    "XCAFPrs",             # XCAF presentation

    # === Font / Image handling ===
    "Font",                # Font management
    "Image",               # Image processing
    "Media",               # Media playback
```
</details>

<details>
<summary>Searched for regex <code>bindings\.toml|BindingsConfig|parse_config</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 6 results</summary>

- [parser.rs:250](../crates/opencascade-binding-generator/src/parser.rs#L250)
- [bindings.rs:869](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L869)
- [bindings.rs:1601](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1601)
- [bindings.rs:1610](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1610)
- [bindings.rs:1695](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1695)
- [bindings.rs:1859](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1859)
</details>

<details>
<summary>Searched for regex <code>struct.*Config|config.*toml|toml.*config|parse.*toml</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 7 results</summary>

- [main.rs:19](../crates/opencascade-binding-generator/src/main.rs#L19)
- [main.rs:143](../crates/opencascade-binding-generator/src/main.rs#L143)
- [config.rs:3](../crates/opencascade-binding-generator/src/config.rs#L3)
- [config.rs:11](../crates/opencascade-binding-generator/src/config.rs#L11)
- [config.rs:47](../crates/opencascade-binding-generator/src/config.rs#L47)
- [config.rs:63](../crates/opencascade-binding-generator/src/config.rs#L63)
- [config.rs:68](../crates/opencascade-binding-generator/src/config.rs#L68)
</details>

<details>
<summary>Reading lines 1-130 of <a href="../crates/opencascade-binding-generator/src/config.rs#L1-L131">config.rs</a></summary>

```
//! Configuration file parsing for the binding generator.
//!
//! Reads a TOML configuration file that specifies which OCCT headers to process.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Top-level configuration structure.
#[derive(Debug, Deserialize)]
pub struct BindingConfig {
    /// General settings.
    #[serde(default)]
    pub general: GeneralConfig,

    /// Include headers from these OCCT modules.
    /// Supports glob patterns: "*" matches all modules, "Geom*" matches
    /// Geom, GeomAdaptor, GeomAPI, etc.
    #[serde(default)]
    pub modules: Vec<String>,

    /// Exclude entire modules from binding generation.
    /// Applied after `modules` expansion (including glob matching).
    /// Supports glob patterns.
    #[serde(default)]
    pub exclude_modules: Vec<String>,

    /// Exclude specific headers, even if their module is included.
    #[serde(default)]
    pub exclude_headers: Vec<String>,

    /// Include specific individual headers (from modules not fully listed in `modules`).
    #[serde(default)]
    pub include_headers: Vec<String>,

    /// Exclude specific methods/constructors from binding generation.
    /// Format: "ClassName::MethodName" for instance/static methods,
    /// or "ClassName::ClassName" for constructors (C++ constructor naming convention).
    /// Methods matching these patterns will be skipped during codegen,
    /// even though the rest of the class is bound.
    #[serde(default)]
    pub exclude_methods: Vec<String>,
}

/// General configuration options.
#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    /// Whether to automatically resolve header dependencies.
    #[serde(default = "default_true")]
    pub resolve_deps: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self { resolve_deps: true }
    }
}

fn default_true() -> bool {
    true
}

/// Load and parse a TOML configuration file.
pub fn load_config(path: &Path) -> Result<BindingConfig> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("Failed to read config file: {}", path.display()))?;
    let config: BindingConfig =
        toml::from_str(&content).with_context(|| format!("Failed to parse config file: {}", path.display()))?;
    Ok(config)
}

/// Check if a module name matches a glob pattern.
/// Supports `*` (matches any sequence of characters) and `?` (matches exactly one character).
pub fn module_matches_pattern(module: &str, pattern: &str) -> bool {
    glob_match(module, pattern)
}

/// Simple glob matching: `*` matches any sequence, `?` matches one char.
fn glob_match(text: &str, pattern: &str) -> bool {
    let text = text.as_bytes();
    let pattern = pattern.as_bytes();
    let mut ti = 0;
    let mut pi = 0;
    let mut star_pi = usize::MAX;
    let mut star_ti = 0;

    while ti < text.len() {
        if pi < pattern.len() && (pattern[pi] == b'?' || pattern[pi] == text[ti]) {
            ti += 1;
            pi += 1;
        } else if pi < pattern.len() && pattern[pi] == b'*' {
            star_pi = pi;
            star_ti = ti;
            pi += 1;
        } else if star_pi != usize::MAX {
            pi = star_pi + 1;
            star_ti += 1;
            ti = star_ti;
        } else {
            return false;
        }
    }
    while pi < pattern.len() && pattern[pi] == b'*' {
        pi += 1;
    }
    pi == pattern.len()
}

/// Discover all unique module names present in the OCCT include directory.
/// A module is identified by the filename prefix before the first `_` in `.hxx` files,
/// or by a bare `{Module}.hxx` file with no underscore.
///
/// Headers with non-standard names (e.g. containing dots like `step.tab.hxx`) are
/// skipped — they are parser tables or other internal files, not real OCCT modules.
fn discover_all_modules(occt_include_dir: &Path) -> Result<Vec<String>> {
    let mut modules: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let entr
... (truncated, 4673 chars)
```
</details>

Now let me look at how the config flows through to the generator:

<details>
<summary>Reading lines 82-200 of <a href="../crates/opencascade-binding-generator/src/main.rs#L82-L201">main.rs</a></summary>

```
fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("OCCT Binding Generator");
        println!("======================");
        println!("Include directories: {:?}", args.include_dirs);
        println!("Output directory: {:?}", args.output);
        if let Some(ref module) = args.module {
            println!("Filtering to module: {}", module);
        }
    }

    // Determine explicit headers from config file or CLI arguments
    let (explicit_headers, resolve_deps, exclude_set, exclude_modules, exclude_methods) = if let Some(ref config_path) = args.config {
        let cfg = config::load_config(config_path)?;
        let resolve = cfg.general.resolve_deps;

        if args.include_dirs.is_empty() {
            anyhow::bail!("--config requires at least one -I <include_dir>");
        }
        let occt_include_dir = &args.include_dirs[0];

        let headers = config::expand_headers(&cfg, occt_include_dir)?;

        if args.verbose {
            println!("Config file: {:?}", config_path);
            println!("  Modules: {:?}", cfg.modules);
            println!("  Exclude modules: {:?}", cfg.exclude_modules);
            println!("  Include headers: {} entries", cfg.include_headers.len());
            println!("  Exclude headers: {} entries", cfg.exclude_headers.len());
            println!("  Exclude methods: {} entries", cfg.exclude_methods.len());
            println!("  Expanded to {} headers", headers.len());
        }

        println!("Loaded config: {} module patterns, {} exclude module patterns, {} individual headers, {} header exclusions -> {} headers",
            cfg.modules.len(), cfg.exclude_modules.len(), cfg.include_headers.len(), cfg.exclude_headers.len(), headers.len());

        // Parse exclude_methods into (ClassName, MethodName) pairs.
        // Uses rsplit to support nested classes: "Outer::Inner::Method" splits
        // as class="Outer::Inner", method="Method".
        let method_exclusions: HashSet<(String, String)> = cfg.exclude_methods
            .iter()
            .filter_map(|s| {
                if let Some(pos) = s.rfind("::") {
                    let class_name = s[..pos].to_string();
                    let method_name = s[pos + 2..].to_string();
                    Some((class_name, method_name))
                } else {
                    eprintln!("Warning: invalid exclude_methods entry (expected ClassName::MethodName): {}", s);
                    None
                }
            })
            .collect();

        let excludes: std::collections::HashSet<String> = cfg.exclude_headers.into_iter().collect();
        let exclude_mods: Vec<String> = cfg.exclude_modules;
        (headers, resolve, excludes, exclude_mods, method_exclusions)
    } else if !args.headers.is_empty() {
        (args.headers.clone(), args.resolve_deps, std::collections::HashSet::new(), Vec::new(), HashSet::new())
    } else {
        anyhow::bail!("Either --config <file.toml> or positional header arguments are required");
    };

    // Resolve header dependencies if requested
    let headers_to_process = if resolve_deps && !args.include_dirs.is_empty() {
        // Use first include dir as OCCT include root
        let occt_include_dir = &args.include_dirs[0];

        if args.verbose {
            println!("\nResolving header dependencies...");
            println!("  OCCT include dir: {:?}", occt_include_dir);
        }

        let resolved = header_deps::resolve_header_dependencies(
            &explicit_headers,
            occt_include_dir,
            args.verbose,
        )?;

        if args.verbose {
            println!("  Explicit headers: {}", explicit_headers.len());
            println!("  Resolved headers: {}", resolved.len());
            println!("  Added {} dependency headers", resolved.len() - explicit_headers.len());
        }

        resolved
    } else {
        explicit_headers
    };

    // Apply exclusions to resolved headers (ex
... (truncated, 5267 chars)
```
</details>

<details>
<summary>Reading lines 200-280 of <a href="../crates/opencascade-binding-generator/src/main.rs#L200-L281">main.rs</a></summary>

```
            println!("  Excluded {} headers after dependency resolution", before - filtered.len());
        }
        filtered
    };

    println!("Parsing {} headers...", headers_to_process.len());
    let mut parsed = parser::parse_headers(&headers_to_process, &args.include_dirs, args.verbose)?;

    // Detect "utility namespace classes" — classes with no underscore in the name
    // (class name == module name), only static methods, and no instance methods/constructors.
    // These are OCCT's namespace-like patterns (e.g., `gp` with `gp::OX()`, `gp::Origin()`).
    // Convert their static methods to free functions so they appear as module-level
    // functions (e.g., `gp::ox()`) instead of awkward `gp::gp::ox()`.
    convert_utility_classes_to_functions(&mut parsed, args.verbose);

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
```
</details>

<details>
<summary>Reading lines 280-360 of <a href="../crates/opencascade-binding-generator/src/main.rs#L280-L361">main.rs</a></summary>

```
    // Generate code
    println!("\nGenerating code...");

    // Collect all classes and enums by module
    let all_classes: Vec<_> = parsed.iter().flat_map(|h| &h.classes).collect();
    let all_enums: Vec<_> = parsed.iter().flat_map(|h| &h.enums).collect();
    let all_functions: Vec<_> = parsed.iter().flat_map(|h| &h.functions).collect();

    // Get collection type names (needed for symbol resolution filtering)
    let all_collections = codegen::collections::all_known_collections();
    let collection_type_names: HashSet<String> =
        all_collections.iter().map(|c| c.typedef_name.clone()).collect();

    // Compute handle-able classes via transitive closure of inheritance graph.
    // This is done before the symbol table so the resolver can use it instead of
    // relying on the old per-class parser heuristic.
    let handle_able_classes = codegen::bindings::compute_handle_able_classes(&all_classes);

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
    );

    if args.verbose {
        println!("\nSymbol table built:");
        println!("  {} classes", symbol_table.classes.len());
        println!("  {} constructors", symbol_table.constructors.len());
        println!("  {} methods", symbol_table.methods.len());
        println!("  {} static methods", symbol_table.static_methods.len());
        println!("  {} functions", symbol_table.functions.len());
        println!("  {} enums", symbol_table.enums.len());

        // Count included vs excluded
        let included_classes = symbol_table.classes.values().filter(|c| c.status.is_included()).count();
        let included_ctors = symbol_table.constructors.values().filter(|c| c.status.is_included()).count();
        let included_methods = symbol_table.methods.values().filter(|m| m.status.is_included()).count();
        let included_static = symbol_table.static_methods.values().filter(|m| m.status.is_included()).count();
        let included_funcs = symbol_table.functions.values().filter(|f| f.status.is_included()).count();

        println!("\n  Included for binding:");
        println!("    {} classes (of {})", included_classes, symbol_table.classes.len());
        println!("    {} constructors (of {})", included_ctors, symbol_table.constructors.len());
        println!("    {} methods (of {})", included_methods, symbol_table.methods.len());
        println!("    {} static methods (of {})", included_static, symbol_table.static_methods.len());
        println!("    {} functions (of {})", included_funcs, symbol_table.functions.len());
    }

    // Dump symbol table if requested
    if args.dump_symbols {
        dump_symbol_table(&symbol_table);
        return Ok(());
    }

    // Collect set of known header filenames that actually exist
    // This is used to filter out headers for types that don't have their own header files
    let known_headers: HashSet<String> = if !args.include_dirs.is_empty() {
        let occt_include_dir = &args.include_dirs[0];
        std::fs::read_dir(occt_include_dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter_map(|e| {
                        let path = e.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("hxx") {
                            path.file_name().and_then(|s| s.to_str()).map(|s| s.to_string())
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    } else {
```
</details>

<details>
<summary>Reading lines 360-440 of <a href="../crates/opencascade-binding-generator/src/main.rs#L360-L441">main.rs</a></summary>

```
    } else {
        HashSet::new()
    };

    if args.verbose {
        println!("  Found {} known OCCT headers", known_headers.len());
    }

    // Generate FFI output
    generate_output(&args, &all_classes, &all_functions, &graph, &symbol_table, &known_headers, &exclude_methods, &handle_able_classes)
}

/// Detect "utility namespace classes" and convert their static methods to free functions.
///
/// OCCT has a pattern where some packages use a class with only static methods instead of
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
                let mut return_type = sm.return_type.clone();

                // If return type is ConstRef and there are no ref params,
                // strip the ConstRef wrapper (return by-value copy). The FFI can't
                // express references from free functions with no borrowable
                // params, so we copy instead.
                let has_ref_params = sm.params.iter().any(|p| matches!(&p.ty, model::Type::ConstRef(_) | model::Type::MutRef(_)));
                if !has_ref_params {
                    if let Some(model::Type::ConstRef(inner)) = &return_type {
                        return_type = Some(*inner.clone());
                    }
                }

                functions_to_add.push(model::ParsedFunction {
                    name: format!("{}::{}", class.name, sm.name),
                    namespace: class.name.clone(),
```
</details>

Now let me look at `generate_output` and how ffi.rs is built:

Searched for regex `fn generate_output` (`**/crates/opencascade-binding-generator/src/main.rs`), 1 result
- [main.rs:572](../crates/opencascade-binding-generator/src/main.rs#L572)

<details>
<summary>Reading lines 572-700 of <a href="../crates/opencascade-binding-generator/src/main.rs#L572-L701">main.rs</a></summary>

```
fn generate_output(
    args: &Args,
    all_classes: &[&model::ParsedClass],
    all_functions: &[&model::ParsedFunction],
    graph: &module_graph::ModuleGraph,
    symbol_table: &resolver::SymbolTable,
    known_headers: &HashSet<String>,
    exclude_methods: &HashSet<(String, String)>,
    handle_able_classes: &HashSet<String>,
) -> Result<()> {
    use model::ParsedClass;

    println!("\n=== Generating FFI output ===\n");

    // Collect all headers
    let mut all_headers: HashSet<String> = HashSet::new();
    for class in all_classes {
        all_headers.insert(class.source_header.clone());
    }
    for func in all_functions {
        all_headers.insert(func.source_header.clone());
    }
    let all_headers_list: Vec<String> = all_headers.into_iter().collect();

    // Get all collections
    let all_collections = codegen::collections::all_known_collections();

    // Compute ClassBindings once for ALL classes — shared by all three generators
    let collection_type_names: std::collections::HashSet<String> =
        all_collections.iter().map(|c| c.typedef_name.clone()).collect();
    let extra_typedef_names = parser::get_collected_typedef_names();
    let all_bindings =
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods);

    // Compute FunctionBindings once for ALL free functions — shared by all three generators
    let (all_function_bindings, all_skipped_functions) = codegen::bindings::compute_all_function_bindings(
        symbol_table, all_classes, &collection_type_names, &extra_typedef_names, known_headers,
    );

    // Track generated files for formatting
    let mut generated_rs_files: Vec<PathBuf> = Vec::new();

    // 1. Generate ffi.rs
    println!("Generating ffi.rs...");
    let (ffi_code, nested_types) = codegen::rust::generate_ffi(
        all_classes,
        &all_headers_list,
        &all_collections,
        symbol_table,
        &all_bindings,
        &all_function_bindings,
        &handle_able_classes,
        &extra_typedef_names,
    );
    let ffi_path = args.output.join("ffi.rs");
    std::fs::write(&ffi_path, ffi_code)?;
    generated_rs_files.push(ffi_path.clone());
    println!("  Wrote: {} ({} classes, {} functions)",
        ffi_path.display(), all_classes.len(), all_functions.len());

    // 2. Generate wrappers.cpp
    println!("Generating wrappers.cpp...");
    let cpp_code = codegen::cpp::generate_wrappers(
        all_classes,
        &all_collections,
        known_headers,
        symbol_table,
        &all_bindings,
        &all_function_bindings,
        &nested_types,
        &handle_able_classes,
    );
    let cpp_path = args.output.join("wrappers.cpp");
    std::fs::write(&cpp_path, &cpp_code)?;
    println!("  Wrote: {}", cpp_path.display());

    // 3. Generate per-module re-export files
    println!("Generating module re-exports...");

    // Index bindings by module for quick lookup
    let mut bindings_by_module: HashMap<String, Vec<&codegen::bindings::ClassBindings>> =
        HashMap::new();
    for b in &all_bindings {
        bindings_by_module
            .entry(b.module.clone())
            .or_default()
            .push(b);
    }

    // Index function bindings by module for quick lookup
    let mut fn_bindings_by_module: HashMap<String, Vec<&codegen::bindings::FunctionBinding>> =
        HashMap::new();
    for fb in &all_function_bindings {
        fn_bindings_by_module
            .entry(fb.module.clone())
            .or_default()
            .push(fb);
    }

    // Index skipped function symbols by module
    let mut skipped_fns_by_module: HashMap<String, Vec<&codegen::bindings::SkippedSymbol>> =
        HashMap::new();
    for sf in &all_skipped_functions {
        skipped_fns_by_module
            .entry(sf.module.clone())
            .or_default()
            .push(sf);
    }

    // Compute ALL types that appear in ffi.rs so we can find unreexported ones
   
... (truncated, 4815 chars)
```
</details>

<details>
<summary>Reading lines 700-830 of <a href="../crates/opencascade-binding-generator/src/main.rs#L700-L831">main.rs</a></summary>

```
    }

    // Now compute ALL types in ffi.rs and find unreexported ones:
    // A. Handle types for all transient classes
    let mut all_ffi_types: Vec<(String, String)> = Vec::new(); // (ffi_name, module_prefix)
    for class in all_classes {
        if handle_able_classes.contains(&class.name) {
            let handle_name = format!("Handle{}", class.name.replace('_', ""));
            if !already_reexported.contains(&handle_name) {
                // Use the class's actual module (not derived from handle name)
                all_ffi_types.push((handle_name, class.module.clone()));
            }
        }
    }

    // B. Opaque referenced types (types referenced in method signatures but not defined)
    let collected_types = codegen::rust::collect_referenced_types(all_classes, &handle_able_classes);
    let defined_classes: HashSet<String> = all_classes.iter().map(|c| c.name.clone()).collect();
    let all_enum_names = &symbol_table.all_enum_names;

    for type_name in &collected_types.classes {
        if defined_classes.contains(type_name) { continue; }
        if all_enum_names.contains(type_name) { continue; }
        // Protected destructor classes are still needed as type references in module reexports
        if codegen::rust::is_primitive_type(type_name) { continue; }
        if collection_type_names.contains(type_name) { continue; }
        if already_reexported.contains(type_name) { continue; }
        // Skip namespace-scoped types and types with pointer/ref qualifiers
        // leaked into the name (e.g., "IMeshData::ListOfPnt2d", "IMeshData_Edge *const")
        if type_name.contains("::") || type_name.contains('*') || type_name.contains('&') { continue; }

        // Determine module from type_to_module map, falling back to name-based
        if let Some(module) = symbol_table.type_to_module.get(type_name) {
            all_ffi_types.push((type_name.clone(), module.clone()));
        } else if let Some(underscore_pos) = type_name.find('_') {
            let module_prefix = &type_name[..underscore_pos];
            all_ffi_types.push((type_name.clone(), module_prefix.to_string()));
        }
    }

    // C. Collection iterator types (not currently re-exported)
    for coll in &all_collections {
        match coll.kind {
            codegen::collections::CollectionKind::Array1 | codegen::collections::CollectionKind::Array2 => {
                // Array types don't have iterator types
            }
            _ => {
                let iter_name = format!("{}Iterator", coll.short_name);
                if !already_reexported.contains(&iter_name) {
                    // Determine module from collection module
                    // coll.module is already a rust module name; we need the C++ module name
                    // Extract it from the typedef_name
                    if let Some(underscore_pos) = coll.typedef_name.find('_') {
                        let module_prefix = &coll.typedef_name[..underscore_pos];
                        all_ffi_types.push((iter_name, module_prefix.to_string()));
                    }
                }
            }
        }
    }

    // Group extra types by module (C++ module name)
    let mut extra_types_by_module: HashMap<String, Vec<(String, String)>> = HashMap::new();
    for (ffi_name, module_prefix) in &all_ffi_types {
        // Compute short name based on type category
        let short_name = if ffi_name.starts_with("Handle") && !ffi_name.contains('_') {
            // Handle types like "HandleGeomEvaluatorCurve" — keep as-is (no short alias)
            ffi_name.clone()
        } else if ffi_name.ends_with("Iterator") && !ffi_name.contains('_') {
            // Collection iterator types like "ListOfShapeIterator" — keep as-is
            ffi_name.clone()
        } else {
            // Use module-relative short name derivation
            opencascade_binding_generator::type_mapping::safe_short_name(&opencascade_binding_generator::type_mapping::short_name_for_
... (truncated, 6165 chars)
```
</details>

<details>
<summary>Reading lines 830-940 of <a href="../crates/opencascade-binding-generator/src/main.rs#L830-L941">main.rs</a></summary>

```
        let empty_skipped_fns: Vec<&codegen::bindings::SkippedSymbol> = Vec::new();
        let module_skipped_fns = skipped_fns_by_module
            .get(&module.rust_name)
            .unwrap_or(&empty_skipped_fns);

        let reexport_code = codegen::rust::generate_module_reexports(
            &module.name,
            &module.rust_name,
            &module_classes,
            &module_collections,
            symbol_table,
            module_bindings,
            module_fn_bindings,
            module_skipped_fns,
            module_extra_types,
        );

        let module_path = args.output.join(format!("{}.rs", module.rust_name));
        let mut reexport_code = reexport_code;
        if let Some(include) = manual_include_directive(&args.output, &module.rust_name) {
            reexport_code.push_str(&include);
        }
        std::fs::write(&module_path, reexport_code)?;
        generated_rs_files.push(module_path.clone());
        println!("  Wrote: {} ({} types, {} extra)",
            module_path.display(), module_classes.len(), module_extra_types.len());
    }

    // Generate module files for extra types whose modules aren't in the graph
    // (e.g., handle types, opaque references from dependency headers)
    let graph_module_names: HashSet<&String> = ordered.iter().map(|m| &m.name).collect();
    let graph_rust_names: HashSet<String> = ordered.iter().map(|m| m.rust_name.clone()).collect();
    let mut extra_only_modules: Vec<(String, String)> = Vec::new(); // (cpp_name, rust_name)
    for (module_name, types) in &extra_types_by_module {
        if !graph_module_names.contains(module_name) && !types.is_empty() {
            let rust_name = module_graph::module_to_rust_name(module_name);
            // Get collections for this module (needed for typedef'd NCollection types)
            let module_collections: Vec<_> = all_collections
                .iter()
                .filter(|c| c.module == rust_name)
                .collect();
            let empty_fn_bindings: Vec<&codegen::bindings::FunctionBinding> = Vec::new();
            let module_fn_bindings = fn_bindings_by_module
                .get(&rust_name)
                .unwrap_or(&empty_fn_bindings);
            let empty_skipped_fns: Vec<&codegen::bindings::SkippedSymbol> = Vec::new();
            let module_skipped_fns = skipped_fns_by_module
                .get(&rust_name)
                .unwrap_or(&empty_skipped_fns);
            let reexport_code = codegen::rust::generate_module_reexports(
                module_name,
                &rust_name,
                &[],
                &module_collections,
                symbol_table,
                &[],
                module_fn_bindings,
                module_skipped_fns,
                types,
            );
            let module_path = args.output.join(format!("{}.rs", rust_name));
            let mut reexport_code = reexport_code;
            if let Some(include) = manual_include_directive(&args.output, &rust_name) {
                reexport_code.push_str(&include);
            }
            std::fs::write(&module_path, &reexport_code)?;
            generated_rs_files.push(module_path.clone());
            extra_only_modules.push((module_name.clone(), rust_name.clone()));
            println!("  Wrote: {} (extra types only, {} types)", module_path.display(), types.len());
        }
    }

    // Generate module files for function-only modules (utility classes converted
    // to free functions that left no classes behind in the module)
    let extra_type_modules: HashSet<String> = extra_only_modules.iter().map(|(_, r)| r.clone()).collect();
    let _empty_fn_bindings_vec: Vec<&codegen::bindings::FunctionBinding> = Vec::new();
    for (rust_module, fn_bindings) in &fn_bindings_by_module {
        if graph_rust_names.contains(rust_module) || extra_type_modules.contains(rust_module) {
            continue;
        }
        if fn_bindings.is_empty() {
            continue;
        }
   
... (truncated, 5190 chars)
```
</details>

Now let me look at how the `generate_ffi` function and `generate_wrappers` emit type declarations:

Research ffi.rs type emission

<details>
<summary>Searched for regex <code>return.*binding|build_return</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 20 results</summary>

- [bindings.rs:146](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L146)
- [bindings.rs:186](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L186)
- [bindings.rs:207](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L207)
- [bindings.rs:280](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L280)
- [bindings.rs:318](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L318)
- [bindings.rs:320](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L320)
- [bindings.rs:360](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L360)
- [bindings.rs:362](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L362)
- [bindings.rs:394](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L394)
- [bindings.rs:881](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L881)
- [bindings.rs:1020](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1020)
- [bindings.rs:1087](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1087)
- [bindings.rs:1116](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1116)
- [bindings.rs:1142](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1142)
- [bindings.rs:1187](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1187)
- [bindings.rs:1232](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1232)
- [bindings.rs:1282](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1282)
- [bindings.rs:1289](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1289)
- [bindings.rs:1318](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1318)
</details>

<details>
<summary>Searched for regex <code>MutRef.*Class|wrapper.*param|cpp.*param|emit.*param</code> (<code>**/crates/opencascade-binding-generator/src/codegen/cpp.rs</code>), 5 results</summary>

- [cpp.rs:190](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L190)
- [cpp.rs:209](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L209)
- [cpp.rs:219](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L219)
- [cpp.rs:225](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L225)
- [cpp.rs:232](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L232)
</details>

Searched for text `fn generate_ffi` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), 1 result
- [rust.rs:132](../crates/opencascade-binding-generator/src/codegen/rust.rs#L132)

Searched for text `fn collect_referenced_types` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), 1 result
- [rust.rs:27](../crates/opencascade-binding-generator/src/codegen/rust.rs#L27)

Searched for text `fn build_param_binding` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:1102](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1102)

Read [rust.rs](../crates/opencascade-binding-generator/src/codegen/rust.rs#27-27), lines 27 to 130

Read [rust.rs](../crates/opencascade-binding-generator/src/codegen/rust.rs#132-132), lines 132 to 260

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#1087-1087), lines 1087 to 1350

Read [cpp.rs](../crates/opencascade-binding-generator/src/codegen/cpp.rs#170-170), lines 170 to 280

Searched for text `fn type_to_cpp_extern_c_param` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:610](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L610)

Searched for text `fn param_to_cpp_extern_c_arg` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:623](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L623)

Searched for text `fn map_type_in_context` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `fn generate_class_wrappers|fn emit_method|method.*wrapper` (`**/crates/opencascade-binding-generator/src/codegen/cpp.rs`), no results

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#610-610), lines 610 to 680

<details>
<summary>Searched for regex <code>emit_ffi_class|class_bindings|fn generate.*class|for.*method</code> (<code>**/crates/opencascade-binding-generator/src/codegen/cpp.rs</code>), 10 results</summary>

- [cpp.rs:3](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L3)
- [cpp.rs:27](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L27)
- [cpp.rs:29](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L29)
- [cpp.rs:34](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L34)
- [cpp.rs:36](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L36)
- [cpp.rs:347](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L347)
- [cpp.rs:348](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L348)
- [cpp.rs:350](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L350)
- [cpp.rs:355](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L355)
- [cpp.rs:357](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L357)
</details>

Searched for text `fn map_type` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

<details>
<summary>Searched for regex <code>cpp_wrapper_name|params_str|cpp_type.*cpp_name</code> (<code>**/crates/opencascade-binding-generator/src/codegen/cpp.rs</code>), 7 results</summary>

- [cpp.rs:187](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L187)
- [cpp.rs:191](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L191)
- [cpp.rs:193](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L193)
- [cpp.rs:209](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L209)
- [cpp.rs:219](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L219)
- [cpp.rs:225](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L225)
- [cpp.rs:232](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L232)
</details>

Searched for text `fn emit_ffi_class` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:5250](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5250)

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#5250-5250), lines 5250 to 5450

Read [cpp.rs](../crates/opencascade-binding-generator/src/codegen/cpp.rs#53-53), lines 53 to 170

Searched for text `emit_cpp_class` (`**/crates/opencascade-binding-generator/src/codegen/cpp.rs`), 1 result
- [cpp.rs:297](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L297)

Searched for text `fn emit_cpp_class` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:3451](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3451)

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3451-3451), lines 3451 to 3650

Read [cpp.rs](../crates/opencascade-binding-generator/src/codegen/cpp.rs#280-280), lines 280 to 350

Searched for regex `fn map_type_in_context|fn map_return_type_in_context` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

<details>
<summary>Searched for text <code>map_type_in_context</code>, 20 results</summary>

- [2026-02-14_15-20-vscode-claude-opus-4.6.md:598](../agent-logs/2026-02-14_15-20-vscode-claude-opus-4.6.md#L598)
- [2026-02-14_15-20-vscode-claude-opus-4.6.md:600](../agent-logs/2026-02-14_15-20-vscode-claude-opus-4.6.md#L600)
- [2026-02-14_15-20-vscode-claude-opus-4.6.md:604](../agent-logs/2026-02-14_15-20-vscode-claude-opus-4.6.md#L604)
- [2026-02-14_15-20-vscode-claude-opus-4.6.md:882](../agent-logs/2026-02-14_15-20-vscode-claude-opus-4.6.md#L882)
- [2026-02-17_16-04-vscode-claude-opus-4.6.md:617](../agent-logs/2026-02-17_16-04-vscode-claude-opus-4.6.md#L617)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1258](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1258)
- [claude-conversation-2026-02-05-0e611961.md:806](../agent-logs/claude-conversation-2026-02-05-0e611961.md#L806)
- [claude-conversation-2026-02-05-0e611961.md:807](../agent-logs/claude-conversation-2026-02-05-0e611961.md#L807)
- [claude-conversation-2026-02-05-0e611961.md:2148](../agent-logs/claude-conversation-2026-02-05-0e611961.md#L2148)
- [claude-conversation-2026-02-05-0e611961.md:2149](../agent-logs/claude-conversation-2026-02-05-0e611961.md#L2149)
- [2026-02-02_19-59_vscode-gemini-3-pro-preview.md:1442](../agent-logs/2026-02-02_19-59_vscode-gemini-3-pro-preview.md#L1442)
- [2026-02-02_19-59_vscode-gemini-3-pro-preview.md:1569](../agent-logs/2026-02-02_19-59_vscode-gemini-3-pro-preview.md#L1569)
- [2026-02-02_19-59_vscode-gemini-3-pro-preview.md:1573](../agent-logs/2026-02-02_19-59_vscode-gemini-3-pro-preview.md#L1573)
- [2026-02-02_19-59_vscode-gemini-3-pro-preview.md:2055](../agent-logs/2026-02-02_19-59_vscode-gemini-3-pro-preview.md#L2055)
- [2026-02-19_14-15-vscode-claude-opus-4.6.md:825](../agent-logs/2026-02-19_14-15-vscode-claude-opus-4.6.md#L825)
- [2026-02-14_12-26-vscode-claude-opus-4.6.md:1593](../agent-logs/2026-02-14_12-26-vscode-claude-opus-4.6.md#L1593)
- [2026-02-14_12-26-vscode-claude-opus-4.6.md:1595](../agent-logs/2026-02-14_12-26-vscode-claude-opus-4.6.md#L1595)
- [2026-02-14_12-26-vscode-claude-opus-4.6.md:1597](../agent-logs/2026-02-14_12-26-vscode-claude-opus-4.6.md#L1597)
</details>

Searched for text `fn map_type_in_context` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), 1 result
- [type_mapping.rs:376](../crates/opencascade-binding-generator/src/type_mapping.rs#L376)

Read [type_mapping.rs](../crates/opencascade-binding-generator/src/type_mapping.rs#376-376), lines 376 to 470

Read [type_mapping.rs](../crates/opencascade-binding-generator/src/type_mapping.rs#470-470), lines 470 to 569

Read [rust.rs](../crates/opencascade-binding-generator/src/codegen/rust.rs#260-260), lines 260 to 340

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3650-3650), lines 3650 to 3720

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_013hZhcJbhXx1RctAmCWTNCD__vscode-1771312785376/content.txt)

> 💭 Excellent! Now I have a very thorough understanding of the flow. Let me summarize the design:
> 
> **The Plan:**
> 
> 1. **bindings.toml**: Add a `[manual_types]` section declaring `Standard_OStream`, `Standard_IStream`, and `Standard_SStream` as manually-provided opaque types with their C++ header names and module assignment.
> 
> 2. **config.rs**: Add `manual_types` field to `BindingConfig`.
> 
> 3. **model.rs**: Remove `is_stream()` from `is_unbindable()`. Stream types will no longer be automatically unbindable.
>    
> 4. **main.rs**: Thread the manual type names through the code generation pipeline—register them as known classes so type checking passes, emit their opaque declarations in ffi.rs, and include the necessary C++ headers in wrappers.cpp.
> 
> 5. **codegen/rust.rs** and **codegen/cpp.rs**: Generate the opaque struct definitions for Rust and the corresponding include directives for C++.
> 
> 6. **resolver.rs**: Make sure manual type names are recognized during type resolution.
> 
> 7. **Manual files**: Create the iostream module with type re-exports and accessor functions, plus the C++ wrapper implementations.
> 
> Now I'm realizing these types don't actually belong to an OCCT module—they come from Standard_OStream.hxx but there's no "iostream" module in the library. I need to decide whether to fold them into the existing standard module or create a separate namespace for them, since the standard module is already quite large and these types feel somewhat orthogonal to the rest of it.
> 
> Looking at how the codebase handles this, manual files like transfer.rs get included directly into their corresponding generated module files via include!(). But iostream doesn't have a natural home in the auto-generated modules. However, the build system does create "extra types only" modules for types that are referenced elsewhere but don't have their own classes, so I could leverage that same pattern here.
> 
> The cleanest solution is to register the stream types as known classes in the generator — they'll get opaque struct declarations in ffi.rs since they appear in method signatures, and the module assignment will follow the Standard_ prefix convention. Then I can remove the stream check from the unbindable types list, and the auto-generated wrappers will handle them correctly.
> 
> For accessing std::cout and std::cerr, I need to decide whether to create a manual/standard.rs file with accessor functions or set up a separate iostream module, since the standard module might already have manual includes.
> 
> Looking at how this would work: the opaque Standard_OStream type would be generated in ffi.rs from method signatures that reference it, then re-exported through the standard module, while manual accessor functions would be included via include!() to give users a clean API like `standard::cout()`.
> 
> But there's a catch—stream types are currently filtered out as unbindable, so their methods never get processed, meaning the types never appear in signatures to be collected in the first place. I'd need to remove the stream filter from the unbindable check to make this work.
> 
> The actual sequence is: add manual types to the config, parse them, register them in the resolver so they're recognized as valid classes, remove the stream filter so methods using them get processed, then let the auto-generation handle the opaque declarations and wrappers. I also need to make sure the C++ wrapper includes are set up correctly for methods that take stream parameters.
> 
> Now I'm checking whether the OCCT headers already pull in the Standard_OStream definition when we include the class headers like gp_Pnt.hxx, or if we need an explicit include in wrappers.cpp. I'm tracing through how `MutRef(Class)` return types get handled in the FFI binding—the C++ wrapper receives a reference from the original method and needs to convert it to a pointer, but since there's no ownership transfer, the caller shouldn't free it. I need to trace through how the C++ wrapper handles converting the reference return value to a pointer for the FFI boundary—there should be similar conversion logic to what handles parameters, likely in the code that emits the wrapper body.
> 
> Rather than keep digging through the codebase, I'll move forward with implementation and address issues as they surface. My approach is to add a manual_types section to the configuration that specifies which types need special handling, then update the config structure to track these mappings.
> 
> In the model layer, I'll remove the stream check from the unbindable logic while keeping the utility function for diagnostics. The resolver needs to incorporate these manual types into its symbol table during the build phase, and I'll thread them through the main pipeline to the code generator. Since these types will appear in method signatures but not as defined classes, they'll naturally flow through the existing referenced types pathway without requiring major codegen changes.
> 
> For the manual implementations, I need to create the Rust accessors for standard I/O streams and their C++ counterparts. I should also write a test that exercises cout with one of OCCT's dump methods to verify the integration works end-to-end.
> 
> Now I'm realizing there's a complication with free functions that take stream references as parameters—like the overloaded operators for inserting objects into streams. These need special handling since they're not methods on a class, so I need to think through how to represent them in the bindings. The operator<< methods are typically free functions and probably won't be in the binding set anyway. Looking at the actual skip patterns, I see methods are being rejected because they return `Standard_OStream&` — the C++ wrapper would need to convert that reference to a pointer, but then the Rust side gets `*mut Standard_OStream`, and I'm not sure how the reexport handles raw pointer returns that don't use `needs_unique_ptr`.
> 
> Let me trace through the return type binding logic for `MutRef(Class)` — since it's not a direct `Class` type and not a handle, `needs_unique_ptr` stays false, which means the reexport won't wrap it in `OwnedPtr`, but I need to understand what actually gets returned in that case.
> 
> Looking at the reexport emit code, when `needs_unique_ptr` is false, it just returns the raw FFI result directly — so for `*mut Standard_OStream`, that's a bare pointer being exposed in the public API, which isn't great. Stream-returning methods are rare enough that I can address those later and focus on void returns first.
> 
> I'm wondering if the "ambiguous lifetimes" check might actually catch these `MutRef(Class)` returns...
> 
> Let me trace through that check in bindings.rs. For something like `Dump(Standard_OStream& theS) -> Standard_OStream&`, the return borrows from the parameter, which would trigger the ambiguous lifetime flag and skip the method entirely. So stream-returning methods would still be filtered out by that check anyway, which is actually what we want — the void-returning ones can now work without that restriction.
> 
> I should just try implementing this and see what breaks. But I'm wondering about the C++ wrapper generation for methods returning `Standard_OStream&` — the extern C wrapper would need to convert that reference to a pointer, so I need to verify the generator handles that conversion correctly. I'm realizing this code path has never actually been tested since no methods with MutRef returns currently exist in the generated code. If they do slip through after removing the is_stream() check, they'll likely fail to compile because the C++ wrapper won't know how to convert the reference return to a pointer. The ambiguous lifetime check should catch most cases anyway—methods returning &mut T with &mut parameters—but some edge cases might get through, like const methods returning Standard_OStream& without a stream parameter, which would be unusual in OCCT.
> 
> Looking at actual OCCT methods, the ones that take Standard_OStream& parameters either return void or return the same stream reference, and those returning the stream would be caught by the ambiguous lifetime check since they have both a MutRef return and MutRef parameters. So the protection is already there.
> 
> For methods like `Standard_OStream& DumpToStream() const` with no stream parameter, those wouldn't trigger the ambiguous lifetime check, but returning a mutable reference from a const method should get downgraded anyway. Rather than overthinking the edge cases, I should just implement this and see what the compiler tells me.
> 
> Looking at the skip breakdown, I estimate around 300-400 of the 546 stream-related skips could be unblocked once I remove the `is_stream()` check — mostly the void-returning methods. The ones with both stream parameters and returns will still hit the ambiguous lifetime issue, and others with unknown types will be caught by existing filters.
> 
> Now I need to figure out how to thread the `manual_types` information through to the resolver in `build_symbol_table`. names to `all_class_names` in that function.
> 
> Let me look at how to do this minimally:
> 
> Option 1: Add a `manual_types: &[String]` parameter to `build_symbol_table`
> Option 2: Have the caller add manual types to some existing data structure
> 
> Actually, the simplest thing might be to just pass the manual type names as a separate parameter. Let me trace through the code path:
> 
> In main.rs:
> 1. Config is loaded (line 97)
> 2. Parsing happens (line 206)
> 3. Symbol table is built (line 301-309)
> 4. Output is generated (line 369)
> 
> The resolver's `build_symbol_table` creates `all_class_names`. I need to add manual types to it. The cleanest way is to add a parameter.
> 
> But actually, looking at the code more carefully, there's a simpler approach. If I add the manual type names directly to `all_class_names` in the resolver, then the TypeContext that gets created downstream will automatically include them, which means `type_uses_unknown_type` will recognize them and methods using those types won't get filtered out. This cascades through both `compute_all_class_bindings` and `compute_all_function_bindings` without needing any other changes.
> 
> So the minimal fix is just to add the manual types to `all_class_names` in the resolver.
> 
> Now about the `collect_referenced_types()` function in ffi.rs — it scans all parsed methods regardless of whether they'll be bound, so `Standard_OStream` will already be collected as a referenced type even if I don't remove the unknown type filter.
> 
> But then I need to check why it's not being emitted as an opaque declaration. Looking at the filtering logic in `generate_opaque_declarations()`, the referenced types go through several checks — they're skipped if they're already defined classes, enums, primitives, collection types, or reexported types, and there's also filtering for types containing special characters.
> 
> `Standard_OStream` would pass all those filters since it's none of those things, so it gets added to the FFI types list with "Standard" as its module prefix. Then it should get emitted as an opaque declaration, but something's preventing that from happening.
> 
> Let me trace through whether `collect_referenced_types()` is actually picking it up in the first place — it only looks at methods, constructors, and static methods from the parsed classes, and when it recurses through a `MutRef`, it extracts the inner class name. So `Standard_OStream` should be getting collected from those `Dump()` methods. But I need to verify if it's actually being emitted in the generated code. I've thought through the theory enough—let me just start implementing. I need to refactor the stream handling by removing the stream check from the unbindable logic, add manual type configuration to the bindings, parse those types through the resolver, create the manual wrappers for standard I/O streams, regenerate everything, and then test it all out. I should also figure out the module structure for iostream support.
> 
> Actually, I'm realizing the `standard` module already exists with all the Standard_ prefixed types, so I just need to append the cout/cerr/cin accessor functions to the generated standard.rs via an include statement rather than creating a completely separate module.
> 
> The simplest approach would be to just remove `is_stream()` from the unbindable check and add the stream type names directly to `all_class_names` in the resolver — the parser already recognizes these types, so I just need to make sure they're treated as regular bindable classes instead of being filtered out. is_stream()` from `is_unbindable()` 
> 2. Add "Standard_OStream", "Standard_IStream", "Standard_SStream" to `all_class_names` in the resolver
> 
> Then methods using streams will pass both checks and get generated.
> 
> The question is where to add these names. I could:
> A. Hardcode them in resolver.rs (simplest, but not great for extensibility)
> B. Add a `manual_types` section to bindings.toml (proper, extensible)
> C. Add them to the `type_mapping.rs` known types list
> 
> I think the user specifically asked for a `manual_types` section in bindings.toml. Let me do that.
> 
> But there's a subtlety: the resolver's all_class_names is used for binding decisions. Adding manual types there means methods using them won't be skipped. But I also need the types to be declared in ffi.rs and re-exported. The types will appear in the referenced types from collect_referenced_types(), so they'll get opaque declarations, and they'll get re-exported to the standard module based on the Standard_ prefix. Let me check if Standard_OStream is already in the collected referenced types or if there's an existing opaque declaration for it in ffi.rs.

Let me check whether `Standard_OStream` already appears in the generated ffi.rs (from referenced types collection):

**Terminal:**
```sh
grep -n 'Standard_OStream\|Standard_IStream\|Standard_SStream' crates/opencascade-sys/generated/ffi.rs | head -10
```
```
30994:pub struct Standard_SStream {
```

**Terminal:**
```sh
grep -c 'Standard_OStream\|Standard_IStream' crates/opencascade-sys/generated/ffi.rs; grep -n 'Standard_SStream' crates/opencascade-sys/generated/ffi.rs
```
```
0
30994:pub struct Standard_SStream {
```

> 💭 Interesting! `Standard_SStream` already has an opaque declaration in ffi.rs (it's already collected as a referenced type), but `Standard_OStream` and `Standard_IStream` do NOT. Why the difference?
> 
> Let me think... `Standard_SStream` appears in method signatures as `Type::Class("Standard_SStream")` and it's collected by `collect_referenced_types()`. But `Standard_OStream` also appears in method signatures... unless it's filtered somewhere.
> 
> Wait — let me check. Maybe `Standard_OStream` is being collected but then the `is_stream()` check causes the method to be filtered out before the type appears in any bound method's signature? No, `collect_referenced_types()` scans ALL parsed methods, not just bound ones.
>  
> Actually, I need to look more carefully at `collect_referenced_types()`. It iterates over all parsed methods—constructors, methods, and static methods—recursively collecting types from parameters and return types. So `Standard_OStream` should definitely be collected if it appears anywhere. But the grep shows it's not in ffi.rs, which means something must be filtering it out. Maybe the `collect_types_from_type()` function itself has some logic that's skipping stream types?
> 
> Wait, let me reconsider. Even if `Standard_OStream` is collected, it could be getting filtered out later by the checks in main.rs around lines 720-738. I should look at what `is_primitive_type()` actually considers primitive, or whether there's some other filtering happening based on the type name itself. Checking the filtering conditions in that loop — primitives, collections, already-reexported types, and names with special characters all get skipped, but none of these would catch `Standard_OStream`. I'm wondering if `collect_referenced_types()` simply doesn't pick it up in the first place, or if the underscore in the name is causing the module prefix extraction to fail somehow.
> 
> Let me trace through what `collect_types_from_type()` actually does — the documentation says it recurses into wrapper types like ConstRef and MutRef to find the inner Class or Handle names, so it should theoretically find `Standard_OStream` even when it's wrapped, but I need to verify that's actually happening in the code.

Interesting — only `Standard_SStream` has an opaque declaration, not `Standard_OStream`/`Standard_IStream`. Let me check why:

<details>
<summary>Reading lines 27-130 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L27-L131">rust.rs</a></summary>

```
pub fn collect_referenced_types(
    classes: &[&ParsedClass],
    handle_able_classes: &HashSet<String>,
) -> CollectedTypes {
    let mut result = CollectedTypes {
        classes: BTreeSet::new(),
        handles: BTreeSet::new(),
    };

    for class in classes {
        // Add Handle type for classes that are transient (can be wrapped in Handle)
        // Handle types with protected destructors are included because Handle<T>
        // manages lifetime via reference counting, not direct delete.
        if handle_able_classes.contains(&class.name) {
            result.handles.insert(class.name.clone());
        }

        // From constructors
        for ctor in &class.constructors {
            for param in &ctor.params {
                collect_types_from_type(&param.ty, &mut result);
            }
        }

        // From methods
        for method in &class.methods {
            for param in &method.params {
                collect_types_from_type(&param.ty, &mut result);
            }
            if let Some(ref ret) = method.return_type {
                collect_types_from_type(ret, &mut result);
            }
        }

        // From static methods
        for method in &class.static_methods {
            for param in &method.params {
                collect_types_from_type(&param.ty, &mut result);
            }
            if let Some(ref ret) = method.return_type {
                collect_types_from_type(ret, &mut result);
            }
        }
    }

    result
}

/// Recursively collect OCCT class and Handle types from a type
fn collect_types_from_type(ty: &Type, collected: &mut CollectedTypes) {
    // Skip unbindable types (arrays, streams, void ptrs, etc.)
    if ty.is_unbindable() {
        return;
    }

    match ty {
        Type::Class(name) => {
            // Skip primitive types that may come from canonical type resolution
            if !is_primitive_type(name) {
                collected.classes.insert(name.clone());
            }
        }
        Type::Handle(name) => {
            // Record the Handle type AND the inner class
            collected.handles.insert(name.clone());
            collected.classes.insert(name.clone());
        }
        Type::ConstRef(inner)
        | Type::MutRef(inner)
        | Type::ConstPtr(inner)
        | Type::MutPtr(inner) => {
            collect_types_from_type(inner, collected);
        }
        _ => {}
    }
}

/// Check if a type name is a primitive (not an OCCT class)
pub fn is_primitive_type(name: &str) -> bool {
    matches!(
        name,
        // Rust primitive names
        "bool" | "i32" | "u32" | "u16" | "i64" | "u64" | "f32" | "f64" | "char" | "c_char" |
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
```
</details>

Found it! Line 77-80: `collect_types_from_type` calls `ty.is_unbindable()` at the top and skips unbindable types (which includes streams). That's why `Standard_OStream` is not collected — it's inside a `MutRef(Class("Standard_OStream"))`, and when the function recurses into the `MutRef`, the inner  has  → `is_unbindable() -> true`, so it's skipped.

But `Standard_SStream` IS collected... Wait.  checks for names containing "OStream" or "IStream" or "ostream" or "istream". `Standard_SStream` doesn't match any of those! It's `SStream`, not `IStream`. So `Standard_SStream` passes through.

So the current  check doesn't catch `Standard_SStream` — that's actually a bug in the is_stream() function (SStream = stringstream). But it works in our favor now because `Standard_SStream` already has an opaque declaration.

Now I have a complete picture. Let me design and implement:

**Todo list:**
- ✅ Research stream type skip logic
- ✅ Design iostream manual bindings
- 🔄 Add manual_types to bindings.toml
- ⬜ Create manual iostream files
- ⬜ Update generator for stream types
- ⬜ Regenerate bindings and build
- ⬜ Write unit test with cout
- ⬜ Update README skip counts
- ⬜ Commit

Let me now look at any additional generated code I need to understand — specifically how the reexport handles referenced types:

**Terminal:**
```sh
grep -n 'Standard_SStream\|SStream' crates/opencascade-sys/generated/standard.rs | head -10
```
<details>
<summary>Output (10 lines)</summary>

```
1281://   Reason: param 'theMessage' uses unknown type 'Standard_SStream&'
1282://   // pub fn raise(theMessage: &mut SStream);
1738://   Reason: param 'theMessage' uses unknown type 'Standard_SStream&'
1739://   // pub fn raise(theMessage: &mut SStream);
2039://   Reason: param 'theMessage' uses unknown type 'Standard_SStream&'
2040://   // pub fn raise(theMessage: &mut SStream);
2347://   Reason: param 'theMessage' uses unknown type 'Standard_SStream&'
2348://   // pub fn raise(theMessage: &mut SStream);
2610://   Reason: param 'theMessage' uses unknown type 'Standard_SStream&'
2611://   // pub fn raise(theMessage: &mut SStream);
```
</details>

**Terminal:**
```sh
grep -n 'pub use.*SStream' crates/opencascade-sys/generated/standard.rs
```
```
(base) whitelily:opencascade-rs sethml$
```
**Exit code:** 1

**Terminal:**
```sh
grep -n 'SStream' crates/opencascade-sys/generated/standard.rs | grep -v '//'
```
```
50081:    Standard_PExtCharacter as PExtCharacter, Standard_SStream as SStream,
```

SStream IS re-exported. Now let me look at the full context of that line and how the re-export works:

<details>
<summary>Reading lines 50075-50090 of <a href="../crates/opencascade-sys/generated/standard.rs#L50075-L50091">standard.rs</a></summary>

```
// ========================
// Additional type re-exports
// ========================

pub use crate::ffi::{
    Standard_Byte as Byte, Standard_JmpBuf as JmpBuf, Standard_PCharacter as PCharacter,
    Standard_PExtCharacter as PExtCharacter, Standard_SStream as SStream,
    Standard_ThreadId as ThreadId, Standard_Utf16Char as Utf16Char,
};

```
</details>

Now I have the complete picture. Let me look at the resolver to see where `all_class_names` is built:

<details>
<summary>Searched for regex <code>all_class_names</code> (<code>**/crates/opencascade-binding-generator/src/resolver.rs</code>), 10 results</summary>

- [resolver.rs:348](../crates/opencascade-binding-generator/src/resolver.rs#L348)
- [resolver.rs:769](../crates/opencascade-binding-generator/src/resolver.rs#L769)
- [resolver.rs:771](../crates/opencascade-binding-generator/src/resolver.rs#L771)
- [resolver.rs:822](../crates/opencascade-binding-generator/src/resolver.rs#L822)
- [resolver.rs:845](../crates/opencascade-binding-generator/src/resolver.rs#L845)
- [resolver.rs:926](../crates/opencascade-binding-generator/src/resolver.rs#L926)
- [resolver.rs:1201](../crates/opencascade-binding-generator/src/resolver.rs#L1201)
- [resolver.rs:1244](../crates/opencascade-binding-generator/src/resolver.rs#L1244)
- [resolver.rs:1283](../crates/opencascade-binding-generator/src/resolver.rs#L1283)
</details>

<details>
<summary>Reading lines 760-830 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L760-L831">resolver.rs</a></summary>

```
    graph: &ModuleGraph,
    all_classes: &[&ParsedClass],
    all_enums: &[&ParsedEnum],
    all_functions: &[&ParsedFunction],
    collection_type_names: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
) -> SymbolTable {
    // Collect all enum and class names first
    let all_enum_names: HashSet<String> = all_enums.iter().map(|e| e.name.clone()).collect();
    let mut all_class_names: HashSet<String> = all_classes.iter().map(|c| c.name.clone()).collect();
    // Collection typedefs are known types for filtering purposes
    all_class_names.extend(collection_type_names.iter().cloned());

    // Start from the pre-computed handle-able classes (transitive closure of inheritance graph)
    let mut handle_able_classes = handle_able_classes.clone();

    // Also add any class name that appears inside Type::Handle(...) in method signatures.
    // If C++ code uses Handle(X) for a type, X must inherit from Standard_Transient,
    // so it's handle-able even if its own header is excluded.
    fn collect_handle_types(ty: &crate::model::Type, set: &mut HashSet<String>) {
        match ty {
            crate::model::Type::Handle(name) => {
                // Only add clean OCCT type names (not template forms)
                if !name.contains('<') && !name.contains("::") {
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
```
</details>

Now let me look at the `build_symbol_table` function signature to understand the parameters:

<details>
<summary>Reading lines 750-760 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L750-L761">resolver.rs</a></summary>

```

    let all_powers_of_2 = nonzero.iter().all(|&v| (v & (v - 1)) == 0);
    let max_val = nonzero.iter().copied().max().unwrap_or(0);

    all_powers_of_2 && max_val >= 4
}

/// Build the symbol table from parsed headers and module graph
pub fn build_symbol_table(
    modules: &[&Module],
    graph: &ModuleGraph,
```
</details>

Now I have a complete understanding. Let me now implement everything. Here's the plan:

1. **`crates/opencascade-binding-generator/src/config.rs`**: Add `manual_types` field to `BindingConfig`
2. **`crates/opencascade-binding-generator/src/model.rs`**: Remove  from `is_unbindable()`; also update `collect_types_from_type` in rust.rs
3. **`crates/opencascade-binding-generator/src/resolver.rs`**: Add `manual_types` parameter, add those names to 
4. **`crates/opencascade-binding-generator/src/main.rs`**: Pass manual types from config through to resolver
5. **manual/standard.rs**: Create iostream accessor functions
6. **manual/standard_wrappers.cpp**: C++ implementations
7. Regenerate, build, test

Let me also check what OCCT header defines Standard_OStream to know what to include in the C++ wrapper:

**Terminal:**
```sh
cat crates/occt-sys/OCCT/src/Standard/Standard_OStream.hxx 2>/dev/null || find crates/occt-sys/OCCT -name 'Standard_OStream*' -type f | head -5
```
<details>
<summary>Output (22 lines)</summary>

```
// Copyright (c) 1998-1999 Matra Datavision
// Copyright (c) 1999-2014 OPEN CASCADE SAS
//
// This file is part of Open CASCADE Technology software library.
//
// This library is free software; you can redistribute it and/or modify it under
// the terms of the GNU Lesser General Public License version 2.1 as published
// by the Free Software Foundation, with special exception defined in the file
// OCCT_LGPL_EXCEPTION.txt. Consult the file LICENSE_LGPL_21.txt included in OCCT
// distribution for complete text of the license and disclaimer of any warranty.
//
// Alternatively, this file may be used under the terms of Open CASCADE
// commercial license or contractual agreement.

#ifndef _Standard_OStream_HeaderFile
#define _Standard_OStream_HeaderFile

#include <Standard_Stream.hxx>

typedef std::ostream Standard_OStream;

#endif
```
</details>

**Terminal:**
```sh
cat crates/occt-sys/OCCT/src/Standard/Standard_IStream.hxx; echo "==="; cat crates/occt-sys/OCCT/src/Standard/Standard_SStream.hxx
```
<details>
<summary>Output (46 lines)</summary>

```
// Copyright (c) 1998-1999 Matra Datavision
// Copyright (c) 1999-2014 OPEN CASCADE SAS
//
// This file is part of Open CASCADE Technology software library.
//
// This library is free software; you can redistribute it and/or modify it under
// the terms of the GNU Lesser General Public License version 2.1 as published
// by the Free Software Foundation, with special exception defined in the file
// OCCT_LGPL_EXCEPTION.txt. Consult the file LICENSE_LGPL_21.txt included in OCCT
// distribution for complete text of the license and disclaimer of any warranty.
//
// Alternatively, this file may be used under the terms of Open CASCADE
// commercial license or contractual agreement.

#ifndef _Standard_IStream_HeaderFile
#define _Standard_IStream_HeaderFile

#include <Standard_Stream.hxx>

typedef std::istream Standard_IStream;

#endif
===
// Copyright (c) 1998-1999 Matra Datavision
// Copyright (c) 1999-2014 OPEN CASCADE SAS
//
// This file is part of Open CASCADE Technology software library.
//
// This library is free software; you can redistribute it and/or modify it under
// the terms of the GNU Lesser General Public License version 2.1 as published
// by the Free Software Foundation, with special exception defined in the file
// OCCT_LGPL_EXCEPTION.txt. Consult the file LICENSE_LGPL_21.txt included in OCCT
// distribution for complete text of the license and disclaimer of any warranty.
//
// Alternatively, this file may be used under the terms of Open CASCADE
// commercial license or contractual agreement.

#ifndef _Standard_SStream_HeaderFile
#define _Standard_SStream_HeaderFile

#include <sstream>

//! Defines Standard_SStream as typedef to C++ string stream.
typedef std::stringstream Standard_SStream;

#endif // _Standard_SStream_HeaderFile
```
</details>

Now let me also look at the C++ return type handling - I need to understand how MutRef returns are emitted in the C++ wrapper:

Research C++ wrapper return emit

Searched for text `emit_cpp_class` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:3451](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3451)

<details>
<summary>Searched for text <code>build_return_type_binding</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 5 results</summary>

- [bindings.rs:1282](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1282)
- [bindings.rs:1739](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1739)
- [bindings.rs:1813](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1813)
- [bindings.rs:1901](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1901)
- [bindings.rs:3412](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3412)
</details>

<details>
<summary>Searched for regex <code>cpp_return_expr|type_to_cpp_return|needs_unique_ptr</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 17 results</summary>

- [bindings.rs:328](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L328)
- [bindings.rs:367](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L367)
- [bindings.rs:1293](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1293)
- [bindings.rs:1322](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1322)
- [bindings.rs:1333](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1333)
- [bindings.rs:1339](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1339)
- [bindings.rs:2945](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2945)
- [bindings.rs:3937](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3937)
- [bindings.rs:3977](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3977)
- [bindings.rs:4109](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4109)
- [bindings.rs:4159](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4159)
- [bindings.rs:4401](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4401)
- [bindings.rs:4826](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4826)
- [bindings.rs:4888](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4888)
- [bindings.rs:4948](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4948)
- [bindings.rs:5045](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5045)
</details>

<details>
<summary>Reading lines 1282-1345 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1282-L1346">bindings.rs</a></summary>

```
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
        let mapped = map_return_type_in_context(ty, ffi_ctx);
        let rust_ffi_type = mapped.rust_type;
        let cpp_type = type_to_cpp(ty);
        // Build the reexport type as Option<&T> or Option<&mut T>
        let is_const = matches!(ty, Type::ConstPtr(_));
        let inner_rust_type = if let Some(ctx) = reexport_ctx {
            ctx.resolve_class(class_name)
        } else {
            format!("crate::ffi::{}", Type::ffi_safe_class_name(class_name))
        };
        let rust_reexport_type = if is_const {
            format!("Option<&{}>", inner_rust_type)
        } else {
            format!("Option<&mut {}>", inner_rust_type)
        };
        return ReturnTypeBinding {
            rust_ffi_type,
            rust_reexport_type,
            cpp_type,
            needs_unique_ptr: false,
            enum_cpp_name: None,
            enum_rust_type: None,
            is_class_ptr_return: true,
        };
    }

    let mapped = map_return_type_in_context(ty, ffi_ctx);
    let rust_ffi_type = mapped.rust_type;
    let rust_reexport_type = return_type_to_rust_string(ty, reexport_ctx);
    let cpp_type = type_to_cpp(ty);
    let needs_unique_ptr = ty.is_class() || ty.is_handle();

    ReturnTypeBinding {
        rust_ffi_type,
        rust_reexport_type,
        cpp_type,
        needs_unique_ptr,
        enum_cpp_name: None,
        enum_rust_type: None,
        is_class_ptr_return: false,
    }
}

```
</details>

<details>
<summary>Reading lines 320-375 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L320-L376">bindings.rs</a></summary>

```
pub struct ReturnTypeBinding {
    /// Type as it appears in ffi.rs
    pub rust_ffi_type: String,
    /// Type as it appears in re-export impl
    pub rust_reexport_type: String,
    /// C++ type for wrappers.hxx
    pub cpp_type: String,
    /// Whether the C++ return needs std::unique_ptr wrapping
    pub needs_unique_ptr: bool,
    /// If this is an enum return, the original C++ enum name (for static_cast)
    pub enum_cpp_name: Option<String>,
    /// If this is a value enum return, the qualified Rust enum type
    pub enum_rust_type: Option<String>,
    /// If this is a raw pointer return to a known class type (const T* / T*)
    pub is_class_ptr_return: bool,
}

/// A resolved parameter binding (from SymbolTable, for inherited methods).
#[derive(Debug, Clone)]
pub struct ResolvedParamBinding {
    pub name: String,
    /// Rust parameter name (keyword-escaped)
    pub rust_name: String,
    pub rust_ffi_type: String,
    /// Type as it appears in re-export impl (e.g. "&crate::ffi::gp_Pnt" or enum type)
    pub rust_reexport_type: String,
    pub cpp_type: String,
    pub cpp_arg_expr: String,
    /// If this is a value enum param, the qualified Rust enum type
    pub enum_rust_type: Option<String>,
    /// If this is a &mut enum output param, the C++ enum name (for preamble/postamble)
    pub mut_ref_enum_cpp_name: Option<String>,
    /// If this is a &mut enum output param, the qualified Rust enum type
    pub mut_ref_enum_rust_type: Option<String>,
    /// If this is a nullable pointer param (T* = NULL or const T* = NULL)
    pub is_nullable_ptr: bool,
    /// If this is a non-nullable class pointer param (const T* / T* where T is a known class)
    pub is_class_ptr: bool,
}

/// A resolved return type binding (from SymbolTable, for inherited methods).
#[derive(Debug, Clone)]
pub struct ResolvedReturnTypeBinding {
    pub rust_ffi_type: String,
    /// Type as it appears in re-export impl
    pub rust_reexport_type: String,
    pub cpp_type: String,
    pub needs_unique_ptr: bool,
    /// If this is an enum return, the original C++ enum name (for static_cast)
    pub enum_cpp_name: Option<String>,
    /// If this is a value enum return, the qualified Rust enum type
    pub enum_rust_type: Option<String>,
    /// If this is a raw pointer return to a known class type (const T* / T*)
    pub is_class_ptr_return: bool,
}

```
</details>

<details>
<summary>Reading lines 3920-4000 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3920-L4001">bindings.rs</a></summary>

```
            format!("{cn}* self_")
        };

        let other_params = wm
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
            .collect::<Vec<_>>()
            .join(", ");
        let params = if other_params.is_empty() {
            self_param
        } else {
            format!("{}, {}", self_param, other_params)
        };

        // Determine return type
        let ret_type_cpp = match &wm.return_type {
            Some(rt) if rt.needs_unique_ptr => format!("{}*", rt.cpp_type),
            Some(rt) if rt.enum_cpp_name.is_some() => "int32_t".to_string(),
            Some(rt) => rt.cpp_type.clone(),
            None => "void".to_string(),
        };

        writeln!(
            output,
            "extern \"C\" {} {fn_name}({params}) {{",
            ret_type_cpp,
            fn_name = wm.ffi_fn_name
        )
        .unwrap();

        // Emit preamble: create local enum variables from int32_t input values
        for p in &wm.params {
            if let Some(ref enum_name) = p.mut_ref_enum_cpp_name {
                writeln!(
                    output,
                    "    auto {local} = static_cast<{enum_name}>({param});",
                    local = p.cpp_arg_expr,
                    param = p.cpp_name,
                )
                .unwrap();
            }
        }

        // Emit the call
        let args_str = wm
            .params
            .iter()
            .map(|p| p.cpp_arg_expr.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let has_enum_return = wm.return_type.as_ref()
            .and_then(|rt| rt.enum_cpp_name.as_ref())
            .is_some();

        if let Some(ref rt) = wm.return_type {
            if rt.needs_unique_ptr {
                writeln!(
                    output,
                    "    auto result_ = new {cpp_type}(self_->{method}({args_str}));",
                    cpp_type = rt.cpp_type,
                    method = wm.cpp_method_name,
                )
                .unwrap();
            } else if has_enum_return {
                writeln!(
                    output,
                    "    auto result_ = static_cast<int32_t>(self_->{method}({args_str}));",
                    method = wm.cpp_method_name,
                )
                .unwrap();
            } else {
                let auto_kw = if rt.cpp_type.ends_with('&') { "auto&" } else { "auto" };
                writeln!(
                    output,
                    "    {auto_kw} result_ = self_->{method}({args_str});",
                    auto_kw = auto_kw,
                    method = wm.cpp_method_name,
                )
                .unwrap();
```
</details>

<details>
<summary>Reading lines 4090-4180 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4090-L4181">bindings.rs</a></summary>

```
        writeln!(output, "}}").unwrap();
    }

    // 5. Static method wrappers
    for sm in &bindings.static_methods {
        let params_str = sm
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
            .collect::<Vec<_>>()
            .join(", ");
        let args_str = sm
            .params
            .iter()
            .map(|p| p.cpp_arg_expr.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let (ret_type, needs_up) = match &sm.return_type {
            Some(rt) => (rt.cpp_type.clone(), rt.needs_unique_ptr),
            None => ("void".to_string(), false),
        };

        let has_enum_return = sm
            .return_type
            .as_ref()
            .and_then(|rt| rt.enum_cpp_name.as_ref())
            .is_some();

        let has_mut_ref_enum = sm.params.iter().any(|p| p.mut_ref_enum_cpp_name.is_some());

        // Check for c_string return (const char* -> const char*)
        let returns_cstring = sm.return_type.as_ref()
            .map(|rt| rt.cpp_type == "const char*")
            .unwrap_or(false);

        if has_mut_ref_enum {
            // Static methods with &mut enum output params need preamble/postamble
            let ret_type_cpp = if needs_up {
                format!("{}*", ret_type)
            } else if has_enum_return {
                "int32_t".to_string()
            } else {
                ret_type.clone()
            };

            writeln!(
                output,
                "extern \"C\" {} {fn_name}({params_str}) {{",
                ret_type_cpp,
                fn_name = sm.ffi_fn_name
            )
            .unwrap();

            // Preamble: create local enum vars
            for p in &sm.params {
                if let Some(ref enum_name) = p.mut_ref_enum_cpp_name {
                    writeln!(
                        output,
                        "    auto {local} = static_cast<{enum_name}>({param});",
                        local = p.cpp_arg_expr,
                        param = p.cpp_name,
                    )
                    .unwrap();
                }
            }

            // Call
            if let Some(ref rt) = sm.return_type {
                if rt.needs_unique_ptr {
                    writeln!(
                        output,
                        "    auto result_ = new {cpp_type}({cn}::{method}({args_str}));",
                        cpp_type = rt.cpp_type,
                        method = sm.cpp_method_name,
                    )
                    .unwrap();
                } else if has_enum_return {
                    writeln!(
                        output,
                        "    auto result_ = static_cast<int32_t>({cn}::{method}({args_str}));",
                        method = sm.cpp_method_name,
                    )
                    .unwrap();
                } else {
                    let auto_kw = if rt.cpp_type.ends_with('&') { "auto&" } else { "auto" };
                    writeln!(
                        output,
                        "    {auto_kw} result_ = {cn}::{method}({args_str});",
                        auto_kw = auto_kw,
                        method = sm.cpp_method_name,
```
</details>

<details>
<summary>Reading lines 4380-4430 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4380-L4431">bindings.rs</a></summary>

```
        let other_params: Vec<String> = im
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.name))
            .collect();
        let params = if other_params.is_empty() {
            self_param
        } else {
            format!("{}, {}", self_param, other_params.join(", "))
        };
        let args_str = im
            .params
            .iter()
            .map(|p| p.cpp_arg_expr.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let has_mut_ref_enum = im.params.iter().any(|p| p.mut_ref_enum_cpp_name.is_some());

        let (ret_type_cpp, needs_up) = match &im.return_type {
            Some(rt) => {
                if rt.needs_unique_ptr {
                    (format!("{}*", rt.cpp_type), true)
                } else if rt.enum_cpp_name.is_some() {
                    ("int32_t".to_string(), false)
                } else {
                    (rt.cpp_type.clone(), false)
                }
            }
            None => ("void".to_string(), false),
        };

        writeln!(
            output,
            "extern \"C\" {ret_type_cpp} {fn_name}({params}) {{",
            fn_name = im.ffi_fn_name
        )
        .unwrap();

        // Preamble: create local enum variables from int32_t for &mut enum params
        for p in &im.params {
            if let Some(ref enum_name) = p.mut_ref_enum_cpp_name {
                writeln!(
                    output,
                    "    auto {local} = static_cast<{enum_name}>({param});",
                    local = p.cpp_arg_expr,
                    param = p.name,
                )
                .unwrap();
            }
        }
```
</details>

<details>
<summary>Reading lines 4000-4092 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4000-L4093">bindings.rs</a></summary>

```
                .unwrap();
            }
        } else {
            writeln!(
                output,
                "    self_->{method}({args_str});",
                method = wm.cpp_method_name,
            )
            .unwrap();
        }

        // Emit postamble: write back enum values to int32_t& output params
        for p in &wm.params {
            if let Some(ref _enum_name) = p.mut_ref_enum_cpp_name {
                writeln!(
                    output,
                    "    {param} = static_cast<int32_t>({local});",
                    param = p.cpp_name,
                    local = p.cpp_arg_expr,
                )
                .unwrap();
            }
        }

        // Emit return
        if wm.return_type.is_some() {
            writeln!(output, "    return result_;").unwrap();
        }

        writeln!(output, "}}").unwrap();
    }

    // 4f. Simple wrapper methods (primitives, void, references, etc.)
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::Simple)
    {
        let self_param = if wm.is_const {
            format!("const {cn}* self_")
        } else {
            format!("{cn}* self_")
        };

        let other_params = wm
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
            .collect::<Vec<_>>()
            .join(", ");
        let params = if other_params.is_empty() {
            self_param
        } else {
            format!("{}, {}", self_param, other_params)
        };
        let args_str = wm
            .params
            .iter()
            .map(|p| p.cpp_arg_expr.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        if let Some(ref rt) = wm.return_type {
            writeln!(
                output,
                "extern \"C\" {} {fn_name}({params}) {{",
                rt.cpp_type,
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    return self_->{method}({args_str});",
                method = wm.cpp_method_name
            )
            .unwrap();
        } else {
            writeln!(
                output,
                "extern \"C\" void {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    self_->{method}({args_str});",
                method = wm.cpp_method_name
            )
            .unwrap();
        }
        writeln!(output, "}}").unwrap();
    }

```
</details>

Searched for regex `fn type_to_cpp` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 2 results
- [bindings.rs:584](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L584)
- [bindings.rs:610](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L610)

<details>
<summary>Reading lines 584-660 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L584-L661">bindings.rs</a></summary>

```
fn type_to_cpp(ty: &Type) -> String {
    match ty {
        Type::Void => "void".to_string(),
        Type::Bool => "Standard_Boolean".to_string(),
        Type::I32 => "Standard_Integer".to_string(),
        Type::U32 => "unsigned int".to_string(),
        Type::U16 => "uint16_t".to_string(),
        Type::I64 => "long long".to_string(),
        Type::U64 => "unsigned long long".to_string(),
        Type::Long => "long".to_string(),
        Type::ULong => "unsigned long".to_string(),
        Type::Usize => "size_t".to_string(),
        Type::F32 => "float".to_string(),
        Type::F64 => "Standard_Real".to_string(),
        Type::ConstRef(inner) => format!("const {}&", type_to_cpp(inner)),
        Type::MutRef(inner) => format!("{}&", type_to_cpp(inner)),
        Type::RValueRef(inner) => format!("{}&&", type_to_cpp(inner)),
        Type::ConstPtr(inner) => format!("const {}*", type_to_cpp(inner)),
        Type::MutPtr(inner) => format!("{}*", type_to_cpp(inner)),
        Type::Handle(name) => format!("opencascade::handle<{}>", name),
        Type::Class(name) => name.clone(),
    }
}


/// Convert a Type to C++ parameter type for extern "C" wrapper functions (pointers instead of references)
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

/// Convert a parameter to C++ argument expression for extern "C" wrappers.
/// Dereferences pointers to match C++ method signatures (which take references).
fn param_to_cpp_extern_c_arg(param_name: &str, ty: &Type) -> String {
    match ty {
        Type::ConstRef(_) | Type::MutRef(_) => format!("*{}", param_name),
        _ => param_name.to_string(),
    }
}

/// Context for resolving C++ class names to their public re-exported Rust type
/// paths instead of raw `crate::ffi::` paths.
pub struct ReexportTypeContext<'a> {
    /// Maps C++ class name (original, may contain ::) → (rust_module_name, short_name)
    pub class_public_info: &'a HashMap<String, (String, String)>,
    /// The Rust module name of the class/function currently being generated
    pub current_module_rust: String,
}

impl<'a> ReexportTypeContext<'a> {
    fn resolve_class(&self, cpp_name: &str) -> String {
        if let Some((module_rust, short)) = self.class_public_info.get(cpp_name) {
            if *module_rust == self.current_module_rust {
                short.clone()
            } else {
                format!("crate::{}::{}", module_rust, short)
            }
        } else {
            format!("crate::ffi::{}", Type::ffi_safe_class_name(cpp_name))
        }
    }
}

/// Build the class_public_info map from a list of parsed classes.
/// Maps C++ class name → (rust_module_name, short_name).
pub(crate) fn build_class_public_info(all_classes: &[&ParsedClass]) -> HashMap<String, (String, String)> {
    all_classes
        .iter()
        .map(|c| {
            let ffi_name = c.name.replace("::", "_");
            let module_rust = crate::module_graph::module_to_rust_name(&c.module);
```
</details>

Searched for regex `enum WrapperKind` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:155](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L155)

<details>
<summary>Reading lines 155-175 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L155-L176">bindings.rs</a></summary>

```
pub enum WrapperKind {
    /// Returns a class or handle by value → new T(...) wrapper
    ByValueReturn,
    /// Has const char* parameters → const char* pass-through
    CStringParam,
    /// Returns const char* → const char* pass-through
    CStringReturn,
    /// Uses enum types (params and/or return) → int32_t/static_cast wrapper
    EnumConversion,
    /// Has by-value class/handle parameters → pointer dereference wrapper
    ByValueParam,
    /// Const method returns &mut T — wrapper takes non-const self
    ConstMutReturnFix,
    /// Has &mut enum output parameters → local variable + writeback wrapper
    MutRefEnumParam,
    /// Simple pass-through wrapper (primitives, void, etc.)
    Simple,
}

/// A method that needs a C++ wrapper function.
#[derive(Debug, Clone)]
```
</details>

Searched for regex `WrapperKind::ByValueReturn` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 2 results
- [bindings.rs:564](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L564)
- [bindings.rs:3502](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3502)

<details>
<summary>Reading lines 550-582 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L550-L583">bindings.rs</a></summary>

```
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

    if returns_by_value {
        WrapperKind::ByValueReturn
    } else if has_mut_ref_enum_params(&method.params, all_enums) {
        WrapperKind::MutRefEnumParam
    } else if has_cstring_param {
        WrapperKind::CStringParam
    } else if returns_cstring {
        WrapperKind::CStringReturn
    } else if resolver::has_const_mut_return_mismatch(method) {
        WrapperKind::ConstMutReturnFix
    } else if resolver::method_uses_enum(method, all_enums) {
        WrapperKind::EnumConversion
    } else if has_by_value_class_or_handle_params(&method.params, all_enums) {
        WrapperKind::ByValueParam
    } else {
        // Simple method wrapper (primitives, void, etc.)
        WrapperKind::Simple
    }
}

```
</details>

<details>
<summary>Reading lines 3495-3610 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3495-L3611">bindings.rs</a></summary>

```
        writeln!(output, "}}").unwrap();
    }

    // 2. ByValueReturn wrapper methods
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::ByValueReturn)
    {
        let self_param = if wm.is_const {
            format!("const {cn}* self_")
        } else {
            format!("{cn}* self_")
        };

        let other_params: Vec<String> = wm
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
            .collect();
        let all_params = std::iter::once(self_param)
            .chain(other_params)
            .collect::<Vec<_>>()
            .join(", ");
        let args_str = wm
            .params
            .iter()
            .map(|p| p.cpp_arg_expr.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let ret_cpp = &wm.return_type.as_ref().unwrap().cpp_type;

        writeln!(
            output,
            "extern \"C\" {ret_cpp}* {fn_name}({all_params}) {{",
            fn_name = wm.ffi_fn_name
        )
        .unwrap();
        writeln!(
            output,
            "    return new {ret_cpp}(self_->{method}({args_str}));",
            method = wm.cpp_method_name
        )
        .unwrap();
        writeln!(output, "}}").unwrap();
    }

    // 3. Static method wrappers
    // Note: In the old code, static methods were emitted between by-value and cstring wrappers
    // when you look at the call order in generate_class_wrappers. Actually, the order is:
    // by-value → cstring-param → cstring-return → static. Let me re-check...
    // The actual call order in generate_class_wrappers is:
    //   1. constructor
    //   2. return_by_value
    //   3. c_string_param
    //   4. c_string_return
    //   5. static_method
    //   6. upcast
    //   7. to_owned
    //   8. to_handle
    //   9. handle_upcast
    //   9b. handle_downcast
    //   10. inherited_method

    // 3. CStringParam wrapper methods
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::CStringParam)
    {
        let self_param = if wm.is_const {
            format!("const {cn}* self_")
        } else {
            format!("{cn}* self_")
        };

        let other_params = wm
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
            .collect::<Vec<_>>()
            .join(", ");
        let params = if other_params.is_empty() {
            self_param
        } else {
            format!("{}, {}", self_param, other_params)
        };
        let args_str = wm
            .params
            .iter()
            .map(|p| p.cpp_arg_expr.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        // Determine return behaviour
        let returns_cstring = wm
            .return_type
            .as_ref()
            .map(|rt| rt.cpp_type == "const char*")
            .unwrap_or(false);
        let returns_reference = wm
            .return_type
            .as_ref()
            .map(|rt| rt.cpp_type.contains('&'))
            .unwrap_or(false);

        if returns_cstring {
            writeln!(
                output,
                "extern \"C\" const char* {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(
                output,
                "    return self_->{method}({args_str});",
```
</details>

<details>
<summary>Reading lines 3900-3935 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3900-L3936">bindings.rs</a></summary>

```
            )
            .unwrap();
        }
        writeln!(output, "}}").unwrap();
    }

    // 4e. MutRefEnumParam wrapper methods
    // These have &mut enum output parameters. The wrapper:
    // 1. Takes int32_t& at the FFI boundary
    // 2. Creates local enum variables from the int32_t values
    // 3. Calls the original method
    // 4. Writes back the enum values as int32_t
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::MutRefEnumParam)
    {
        let self_param = if wm.is_const {
            format!("const {cn}* self_")
        } else {
            format!("{cn}* self_")
        };

        let other_params = wm
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
            .collect::<Vec<_>>()
            .join(", ");
        let params = if other_params.is_empty() {
            self_param
        } else {
            format!("{}, {}", self_param, other_params)
        };

        // Determine return type
```
</details>

Searched for regex `fn is_class\b` (`**/crates/opencascade-binding-generator/src/**/*.rs`), 1 result
- [model.rs:491](../crates/opencascade-binding-generator/src/model.rs#L491)

Searched for regex `fn class_ptr_inner_name` (`**/crates/opencascade-binding-generator/src/**/*.rs`), 1 result
- [model.rs:571](../crates/opencascade-binding-generator/src/model.rs#L571)

<details>
<summary>Reading lines 488-600 of <a href="../crates/opencascade-binding-generator/src/model.rs#L488-L601">model.rs</a></summary>

```


    /// Check if this is an OCCT class type (not primitive, not reference/pointer)
    pub fn is_class(&self) -> bool {
        matches!(self, Type::Class(_))
    }

    /// Check if this is a Handle type
    pub fn is_handle(&self) -> bool {
        matches!(self, Type::Handle(_))
    }

    /// Check if this is a reference type (const ref or mutable ref)
    pub fn is_reference(&self) -> bool {
        matches!(self, Type::ConstRef(_) | Type::MutRef(_))
    }

    /// Check if this is a const char* type (C string pointer)
    pub fn is_c_string(&self) -> bool {
        match self {
            Type::ConstPtr(inner) => matches!(inner.as_ref(), Type::Class(name) if name == "char"),
            _ => false,
        }
    }

    /// Check if this is a C++ stream type (Standard_OStream, Standard_IStream, etc.)
    /// These can't be bound through the FFI
    pub fn is_stream(&self) -> bool {
        match self {
            Type::Class(name) => {
                name.contains("OStream")
                    || name.contains("IStream")
                    || name.contains("ostream")
                    || name.contains("istream")
            }
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => inner.is_stream(),
            _ => false,
        }
    }

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
    /// with const char* pass-through wrappers.
    pub fn is_raw_ptr(&self) -> bool {
        match self {
            // const char* is bindable - we generate wrappers
            Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => false,
            Type::ConstPtr(_) | Type::MutPtr(_) => true,
            // References to raw pointers also count as problematic
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => inner.is_raw_ptr(),
            _ => false,
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
            _ => fal
... (truncated, 4413 chars)
```
</details>

Searched for regex `fn map_return_type_in_context` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `fn map_return_type` (`**/crates/opencascade-binding-generator/src/**/*.rs`), 2 results
- [type_mapping.rs:181](../crates/opencascade-binding-generator/src/type_mapping.rs#L181)
- [type_mapping.rs:460](../crates/opencascade-binding-generator/src/type_mapping.rs#L460)

<details>
<summary>Reading lines 181-230 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L181-L231">type_mapping.rs</a></summary>

```
pub fn map_return_type(ty: &Type) -> RustTypeMapping {
    let mut mapping = map_type_to_rust(ty);

    // Return-by-value C++ types are returned as *mut T (heap-allocated)
    if mapping.needs_unique_ptr {
        mapping.rust_type = format!("*mut {}", mapping.rust_type);
    }

    mapping
}

/// Map a type for use in self position
pub fn map_self_type(ty: &Type, is_const: bool) -> RustTypeMapping {
    let inner_mapping = map_type_to_rust(ty);

    if is_const {
        RustTypeMapping {
            rust_type: format!("*const {}", inner_mapping.rust_type),
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

/// Reserved names that would conflict with Rust standard library types.
/// Currently unused — we allow short names like `Vec`, `Box`, `Result` because
/// they live inside module namespaces and don't shadow prelude names.
const FFI_RESERVED_NAMES: &[&str] = &[];

/// Check if a short name is reserved and needs escaping
pub fn is_reserved_name(name: &str) -> bool {
    FFI_RESERVED_NAMES.contains(&name)
}

/// Get the safe Rust name for a short class name.
/// With an empty reserved list, this is effectively a no-op.
pub fn safe_short_name(short_name: &str) -> String {
    if is_reserved_name(short_name) {
        format!("{}_", short_name)
    } else {
        short_name.to_string()
    }
```
</details>

<details>
<summary>Reading lines 460-530 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L460-L531">type_mapping.rs</a></summary>

```
pub fn map_return_type_in_context(ty: &Type, ctx: &TypeContext) -> RustTypeMapping {
    // const char* returns stay as *const c_char
    if ty.is_c_string() {
        return RustTypeMapping {
            rust_type: "*const std::ffi::c_char".to_string(),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: None,
        };
    }
    
    let mut mapping = map_type_in_context(ty, ctx);

    if mapping.needs_unique_ptr {
        mapping.rust_type = format!("*mut {}", mapping.rust_type);
    }

    mapping
}

/// Map a C++ type string directly (for cases where we only have the string)
pub fn map_cpp_type_string(cpp_type: &str) -> RustTypeMapping {
    let cpp_type = cpp_type.trim();

    // Handle primitives
    match cpp_type {
        "void" => return map_type_to_rust(&Type::Void),
        "bool" | "Standard_Boolean" => return map_type_to_rust(&Type::Bool),
        "int" | "Standard_Integer" => return map_type_to_rust(&Type::I32),
        "unsigned int" => return map_type_to_rust(&Type::U32),
        "unsigned short" | "uint16_t" | "char16_t" | "Standard_ExtCharacter" => return map_type_to_rust(&Type::U16),
        "long" => return map_type_to_rust(&Type::Long),
        "unsigned long" => return map_type_to_rust(&Type::ULong),
        "float" => return map_type_to_rust(&Type::F32),
        "double" | "Standard_Real" => return map_type_to_rust(&Type::F64),
        _ => {}
    }

    // Handle const references
    if cpp_type.starts_with("const ") && cpp_type.ends_with('&') {
        let inner = cpp_type[6..cpp_type.len() - 1].trim();
        let inner_mapping = map_cpp_type_string(inner);
        return RustTypeMapping {
            rust_type: format!("*const {}", inner_mapping.rust_type),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: inner_mapping.source_module,
        };
    }

    // Handle mutable references
    if let Some(inner) = cpp_type.strip_suffix('&') {
        let inner = inner.trim();
        let inner_mapping = map_cpp_type_string(inner);
        return RustTypeMapping {
            rust_type: format!("*mut {}", inner_mapping.rust_type),
            needs_unique_ptr: false,
            needs_pin: false,
            source_module: inner_mapping.source_module,
        };
    }

    // Handle Handle types
    if cpp_type.starts_with("Handle(") && cpp_type.ends_with(')') {
        let inner = &cpp_type[7..cpp_type.len() - 1];
        return map_type_to_rust(&Type::Handle(inner.to_string()));
    }

    if cpp_type.starts_with("opencascade::handle<") && cpp_type.ends_with('>') {
        let inner = &cpp_type[20..cpp_type.len() - 1];
        return map_type_to_rust(&Type::Handle(inner.to_string()));
```
</details>

Searched for regex `fn map_type_to_rust` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), 1 result
- [type_mapping.rs:24](../crates/opencascade-binding-generator/src/type_mapping.rs#L24)

<details>
<summary>Reading lines 24-90 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L24-L91">type_mapping.rs</a></summary>

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
```
</details>

<details>
<summary>Reading lines 90-180 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L90-L181">type_mapping.rs</a></summary>

```
            source_module: None,
        },
        Type::F64 => RustTypeMapping {
            rust_type: "f64".to_string(),
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
            let handle_type = format!("Handle{}", class_name.replace("_", ""));
            RustTypeMapping {
                rust_type: handle_type,
                needs_unique_ptr: true, // Returned as *mut T, caller must free
                needs_pin: false,
                source_module,
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
                source_module,
            }
        }
    }
}

/// Map a type for use in return position (returns *mut T for owned objects)
```
</details>

> 💭 Excellent! So the C++ wrapper for MutRef return types uses `Standard_OStream&` as the return type, and Rust sees `*mut Standard_OStream`. This works due to ABI compatibility between references and pointers. So methods returning `Standard_OStream&` WILL work at the C++ level.
> 
> But the important question is: will such methods actually get through the binding filters? Let me think:
> - A method like `Standard_OStream& Dump(Standard_OStream& theS) const`` would haveMutRef(Class("Standard_OStream"))` as return AND MutRef(Class("Standard_OStream")) as a param
> 
> 
> - The "ambiguous lifetime" check would catch this — it has a MutRef return AND reference params, so it gets filtered out anyway.
> 
> For void-returning methods like `void Dump(Standard_OStream& theS) const`, there's no return type issue and they'd pass through. But when a method does return a MutRef, the reexport layer would just expose the raw FFI pointer type directly to the public API, which isn't ideal but is technically workable.
> 
> Now I'm thinking through how the reexport actually handles the common case where we have a const method taking a mutable reference parameter. The FFI layer converts that mutable reference to a raw pointer, and the reexport needs to wrap it back into a safe Rust reference for the public API. I need to verify how the reexport layer handles the type conversion for mutable references to classes—whether it uses the same FFI type or wraps it differently. The key insight from the generated code is that methods like `Dump(Standard_OStream& theS)` get reexported with signatures like `pub fn dump(&self, theS: &mut Standard_OStream)`, so the generator must be handling this conversion somewhere in the `ParamBinding` struct. I'll trace through the implementation when I start coding.
> 
> Now I'm ready to begin implementing—I'll read through the key files and make the necessary changes.

Now I have the complete picture. The C++ return path for MutRef uses `Standard_OStream&` directly (ABI-compatible with `*mut`). Methods returning `Standard_OStream&` with stream params will be caught by the ambiguous lifetime check anyway. Let me implement everything now.

**Todo list:**
- ✅ Research stream type skip logic
- ✅ Design iostream manual bindings
- 🔄 Add manual_types to bindings.toml
- ⬜ Create manual iostream files
- ⬜ Update generator for stream types
- ⬜ Regenerate bindings and build
- ⬜ Write unit test with cout
- ⬜ Update README skip counts
- ⬜ Commit

Let me read the end of bindings.toml to see where to add the new section:

**Terminal:**
```sh
wc -l crates/opencascade-sys/bindings.toml
```
```
     168 crates/opencascade-sys/bindings.toml
```

<details>
<summary>Reading lines 130-168 of <a href="../crates/opencascade-sys/bindings.toml#L130-L169">bindings.toml</a></summary>

```
    "RWGltf_GltfJsonParser.hxx",
    "RWGltf_CafWriter.hxx",
    "RWGltf_GltfAccessor.hxx",

    # BVH template classes: abstract base or complex template type params
    # that the generator can't resolve correctly.
    "BVH_Properties.hxx",

    # Internal-only class: all constructors/destructors are private, accessible only
    # to friend classes TDF_Data and TDF_Label. No public API returns TDF_LabelNode*;
    # it's entirely hidden behind the TDF_Label value-type facade.
    "TDF_LabelNode.hxx",

    # RWObj_MtlReader: constructor takes OSD_OpenFile_Mode not available.
    "RWObj_MtlReader.hxx",

    # Windows-only: includes <windows.h> which causes a fatal error on non-Windows
    # and corrupts clang's type resolution for subsequent headers.
    "OSD_WNT.hxx",
]

# Include specific individual headers (from modules not fully included above).
include_headers = []

# Exclude specific methods/constructors that clang completely misresolves in batch
# parsing mode — both canonical AND display names show the wrong type, so the
# parser's heuristic guards cannot detect it. These methods are excluded
# individually so the rest of each class remains accessible.
# Format: "ClassName::MethodName" for methods, "ClassName::ClassName" for constructors.
exclude_methods = [
    # BRepExtrema_ProximityDistTool::PrjState: constructor takes BVH_PrjState
    # (a template-dependent enum typedef) that clang can't resolve through the
    # template specialization. PrjState is an internal struct not exposed by
    # public methods.
    "BRepExtrema_ProximityDistTool::PrjState::*",
]

[general]
resolve_deps = true
```
</details>

Now I have everything I need. Let me implement all the changes. First, the easy ones: Now I have everything I need. Let me implement all the changes. First, the easy ones: Let me make all the changes. Start with bindings.toml, config.rs, model.rs, resolver.rs, main.rs, and codegen/rust.rs:

<details>
<summary>Edited: <a href="../crates/opencascade-sys/bindings.toml#L169-L178">bindings.toml +9</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "168:bz",
      "insertAfter": true,
      "content": "\n# Opaque types defined in manual/ files. These are not parsed from OCCT headers\n# but referenced by auto-generated bindings. The generator adds them to the known\n# class set so methods using them aren't skipped as \"unknown type\".\n# Format: \"CppTypeName\" = { header = \"Header.hxx\" }\n[manual_types]\nStandard_OStream = { header = \"Standard_OStream.hxx\" }\nStandard_IStream = { header = \"Standard_IStream.hxx\" }\nStandard_SStream = { header = \"Standard_SStream.hxx\" }"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":168,"endLine":177}
```
</details>

Now update `crates/opencascade-binding-generator/src/config.rs` to parse the new section:

<details>
<summary>Reading lines 1-43 of <a href="../crates/opencascade-binding-generator/src/config.rs#L1-L44">config.rs</a></summary>

```
//! Configuration file parsing for the binding generator.
//!
//! Reads a TOML configuration file that specifies which OCCT headers to process.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Top-level configuration structure.
#[derive(Debug, Deserialize)]
pub struct BindingConfig {
    /// General settings.
    #[serde(default)]
    pub general: GeneralConfig,

    /// Include headers from these OCCT modules.
    /// Supports glob patterns: "*" matches all modules, "Geom*" matches
    /// Geom, GeomAdaptor, GeomAPI, etc.
    #[serde(default)]
    pub modules: Vec<String>,

    /// Exclude entire modules from binding generation.
    /// Applied after `modules` expansion (including glob matching).
    /// Supports glob patterns.
    #[serde(default)]
    pub exclude_modules: Vec<String>,

    /// Exclude specific headers, even if their module is included.
    #[serde(default)]
    pub exclude_headers: Vec<String>,

    /// Include specific individual headers (from modules not fully listed in `modules`).
    #[serde(default)]
    pub include_headers: Vec<String>,

    /// Exclude specific methods/constructors from binding generation.
    /// Format: "ClassName::MethodName" for instance/static methods,
    /// or "ClassName::ClassName" for constructors (C++ constructor naming convention).
    /// Methods matching these patterns will be skipped during codegen,
    /// even though the rest of the class is bound.
    #[serde(default)]
    pub exclude_methods: Vec<String>,
}
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/config.rs#L42-L51">config.rs -2+9</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    pub exclude_methods: Vec<String>,\n\n    /// Opaque types defined in manual/ files but referenced by auto-generated bindings.\n    /// The generator adds these to the known class set so methods using them\n    /// aren't skipped as \"unknown type\".\n    /// Format: `TypeName = { header = \"Header.hxx\" }`\n    #[serde(default)]\n    pub manual_types: std::collections::HashMap<String, ManualType>,\n}",
      "filePath": "crates/opencascade-binding-generator/src/config.rs",
      "lineHashes": "42:cx,43:io"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/config.rs","startLine":42,"endLine":50}
```
</details>

Now add the `ManualType` struct:

<details>
<summary>Reading lines 50-60 of <a href="../crates/opencascade-binding-generator/src/config.rs#L50-L61">config.rs</a></summary>

```
}

/// General configuration options.
#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    /// Whether to automatically resolve header dependencies.
    #[serde(default = "default_true")]
    pub resolve_deps: bool,
}

impl Default for GeneralConfig {
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/config.rs#L50-L60">config.rs -3+10</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "}\n\n/// A manually-defined opaque type referenced by auto-generated bindings.\n#[derive(Debug, Deserialize)]\npub struct ManualType {\n    /// The C++ header that defines this type (for wrappers.cpp includes).\n    pub header: String,\n}\n\n/// General configuration options.",
      "filePath": "crates/opencascade-binding-generator/src/config.rs",
      "lineHashes": "50:io,51:dd,52:kz"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/config.rs","startLine":50,"endLine":59}
```
</details>

`crates/opencascade-binding-generator/src/model.rs` - remove  from `is_unbindable()`:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L640-L643">model.rs -3+3</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    pub fn is_unbindable(&self) -> bool {\n        self.is_void_ptr() || self.is_array() || self.is_raw_ptr() || self.is_rvalue_ref() || self.is_unresolved_template_type()\n    }",
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "640:km,641:sa,642:ax"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/model.rs","startLine":640,"endLine":642}
```
</details>

Now update the `collect_types_from_type` in rust.rs — the is_unbindable check there also blocks streams:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L76-L82">rust.rs -5+6</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "fn collect_types_from_type(ty: &Type, collected: &mut CollectedTypes) {\n    // Skip unbindable types (arrays, void ptrs, etc.) — but NOT streams,\n    // which are manually-defined opaque types that we DO want to collect.\n    if ty.is_unbindable() {\n        return;\n    }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "lineHashes": "76:ta,77:ks,78:gm,79:ah,80:ax"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/rust.rs","startLine":76,"endLine":81}
```
</details>

Good — since `is_unbindable()` no longer includes , stream types will now pass through and get collected. Now update the resolver to accept and include manual types:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/resolver.rs#L757-L768">resolver.rs -10+11</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "/// Build the symbol table from parsed headers and module graph\npub fn build_symbol_table(\n    modules: &[&Module],\n    graph: &ModuleGraph,\n    all_classes: &[&ParsedClass],\n    all_enums: &[&ParsedEnum],\n    all_functions: &[&ParsedFunction],\n    collection_type_names: &HashSet<String>,\n    handle_able_classes: &HashSet<String>,\n    manual_type_names: &HashSet<String>,\n) -> SymbolTable {",
      "filePath": "crates/opencascade-binding-generator/src/resolver.rs",
      "lineHashes": "757:tr,758:qa,759:yg,760:ra,761:ue,762:th,763:fe,764:ws,765:kf,766:sm"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/resolver.rs","startLine":757,"endLine":767}
```
</details>

Now add manual types to `all_class_names` right after the collection typedef extension:

<details>
<summary>Reading lines 768-778 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L768-L779">resolver.rs</a></summary>

```
    // Collect all enum and class names first
    let all_enum_names: HashSet<String> = all_enums.iter().map(|e| e.name.clone()).collect();
    let mut all_class_names: HashSet<String> = all_classes.iter().map(|c| c.name.clone()).collect();
    // Collection typedefs are known types for filtering purposes
    all_class_names.extend(collection_type_names.iter().cloned());

    // Start from the pre-computed handle-able classes (transitive closure of inheritance graph)
    let mut handle_able_classes = handle_able_classes.clone();

    // Also add any class name that appears inside Type::Handle(...) in method signatures.
    // If C++ code uses Handle(X) for a type, X must inherit from Standard_Transient,
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/resolver.rs#L771-L775">resolver.rs -2+4</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    // Collection typedefs are known types for filtering purposes\n    all_class_names.extend(collection_type_names.iter().cloned());\n    // Manual types (defined in manual/ files) are also known types\n    all_class_names.extend(manual_type_names.iter().cloned());",
      "filePath": "crates/opencascade-binding-generator/src/resolver.rs",
      "lineHashes": "771:jn,772:jd"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/resolver.rs","startLine":771,"endLine":774}
```
</details>

`crates/opencascade-binding-generator/src/main.rs` to pass manual types through. First, I need to extract manual type names from the config and pass them to `build_symbol_table`:

<details>
<summary>Reading lines 93-145 of <a href="../crates/opencascade-binding-generator/src/main.rs#L93-L146">main.rs</a></summary>

```
    }

    // Determine explicit headers from config file or CLI arguments
    let (explicit_headers, resolve_deps, exclude_set, exclude_modules, exclude_methods) = if let Some(ref config_path) = args.config {
        let cfg = config::load_config(config_path)?;
        let resolve = cfg.general.resolve_deps;

        if args.include_dirs.is_empty() {
            anyhow::bail!("--config requires at least one -I <include_dir>");
        }
        let occt_include_dir = &args.include_dirs[0];

        let headers = config::expand_headers(&cfg, occt_include_dir)?;

        if args.verbose {
            println!("Config file: {:?}", config_path);
            println!("  Modules: {:?}", cfg.modules);
            println!("  Exclude modules: {:?}", cfg.exclude_modules);
            println!("  Include headers: {} entries", cfg.include_headers.len());
            println!("  Exclude headers: {} entries", cfg.exclude_headers.len());
            println!("  Exclude methods: {} entries", cfg.exclude_methods.len());
            println!("  Expanded to {} headers", headers.len());
        }

        println!("Loaded config: {} module patterns, {} exclude module patterns, {} individual headers, {} header exclusions -> {} headers",
            cfg.modules.len(), cfg.exclude_modules.len(), cfg.include_headers.len(), cfg.exclude_headers.len(), headers.len());

        // Parse exclude_methods into (ClassName, MethodName) pairs.
        // Uses rsplit to support nested classes: "Outer::Inner::Method" splits
        // as class="Outer::Inner", method="Method".
        let method_exclusions: HashSet<(String, String)> = cfg.exclude_methods
            .iter()
            .filter_map(|s| {
                if let Some(pos) = s.rfind("::") {
                    let class_name = s[..pos].to_string();
                    let method_name = s[pos + 2..].to_string();
                    Some((class_name, method_name))
                } else {
                    eprintln!("Warning: invalid exclude_methods entry (expected ClassName::MethodName): {}", s);
                    None
                }
            })
            .collect();

        let excludes: std::collections::HashSet<String> = cfg.exclude_headers.into_iter().collect();
        let exclude_mods: Vec<String> = cfg.exclude_modules;
        (headers, resolve, excludes, exclude_mods, method_exclusions)
    } else if !args.headers.is_empty() {
        (args.headers.clone(), args.resolve_deps, std::collections::HashSet::new(), Vec::new(), HashSet::new())
    } else {
        anyhow::bail!("Either --config <file.toml> or positional header arguments are required");
    };

```
</details>

I need to add `manual_types` to the tuple returned from config parsing. Let me update the config extraction:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L96-L97">main.rs -1+1</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L137-L141">-3+4</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L142-L143">-1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    let (explicit_headers, resolve_deps, exclude_set, exclude_modules, exclude_methods, manual_type_names) = if let Some(ref config_path) = args.config {",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "96:jp"
    },
    {
      "content": "        let excludes: std::collections::HashSet<String> = cfg.exclude_headers.into_iter().collect();\n        let exclude_mods: Vec<String> = cfg.exclude_modules;\n        let manual_names: HashSet<String> = cfg.manual_types.keys().cloned().collect();\n        (headers, resolve, excludes, exclude_mods, method_exclusions, manual_names)",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "137:wk,138:pb,139:mp"
    },
    {
      "content": "        (args.headers.clone(), args.resolve_deps, std::collections::HashSet::new(), Vec::new(), HashSet::new(), HashSet::new())",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "141:mc"
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
      "startLine": 96,
      "endLine": 142
    }
  ]
}
```
</details>

Now update the `build_symbol_table` call to pass `manual_type_names`:

<details>
<summary>Reading lines 298-315 of <a href="../crates/opencascade-binding-generator/src/main.rs#L298-L316">main.rs</a></summary>

```

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
    );

    if args.verbose {
        println!("\nSymbol table built:");
        println!("  {} classes", symbol_table.classes.len());
        println!("  {} constructors", symbol_table.constructors.len());
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L302-L312">main.rs -9+10</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    let symbol_table = resolver::build_symbol_table(\n        &ordered_modules,\n        &graph,\n        &all_classes,\n        &all_enums,\n        &all_functions,\n        &collection_type_names,\n        &handle_able_classes,\n        &manual_type_names,\n    );",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "302:gg,303:me,304:ty,305:pc,306:pk,307:ue,308:nb,309:nk,310:pd"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/main.rs","startLine":302,"endLine":311}
```
</details>

Now I also need to update the `lib.rs` test helper if it calls `build_symbol_table`:

Searched for text `build_symbol_table` (`**/crates/opencascade-binding-generator/src/lib.rs`), no results

Searched for text `build_symbol_table` (`**/crates/opencascade-binding-generator/**`), 4 results
- [resolver.rs:758](../crates/opencascade-binding-generator/src/resolver.rs#L758)
- [main.rs:302](../crates/opencascade-binding-generator/src/main.rs#L302)
- [codegen-flow.dot:37](../crates/opencascade-binding-generator/codegen-flow.dot#L37)
- [codegen-flow.svg:94](../crates/opencascade-binding-generator/codegen-flow.svg#L94)

Good — only called from main.rs. Now let me build the binding generator to check for compile errors:

**Todo list:**
- ✅ Research stream type skip logic
- ✅ Design iostream manual bindings
- ✅ Add manual_types to bindings.toml
- ⬜ Create manual iostream files
- 🔄 Update generator for stream types
- ⬜ Regenerate bindings and build
- ⬜ Write unit test with cout
- ⬜ Update README skip counts
- ⬜ Commit

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1; echo "EXIT: $?"
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.70s
real  2.7s
EXIT: 0
```

Generator compiles. Now let me create the manual iostream files. First, the C++ wrapper:

Created [standard_wrappers.cpp](../crates/opencascade-sys/manual/standard_wrappers.cpp)

Now the Rust side. This will be `include!()`d into the generated `standard.rs`:

Created [standard.rs](../crates/opencascade-sys/manual/standard.rs)

Now let me regenerate bindings and build. This is the critical test:

**Todo list:**
- ✅ Research stream type skip logic
- ✅ Design iostream manual bindings
- ✅ Add manual_types to bindings.toml
- ✅ Create manual iostream files
- ✅ Update generator for stream types
- 🔄 Regenerate bindings and build
- ⬜ Write unit test with cout
- ⬜ Update README skip counts
- ⬜ Commit

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1; echo "EXIT: $?"
```
<details>
<summary>Output (319 lines)</summary>

```
Building binding generator...
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `release` profile [optimized] target(s) in 11.82s
Generating bindings from /Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml...
Cleaning /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated...
  Excluded 73 modules via exclude_modules
Loaded config: 1 module patterns, 75 exclude module patterns, 0 individual headers, 10 header exclusions -> 5761 headers
  Excluded 35 headers after dependency resolution
Parsing 5761 headers...
  Clang parse time: 2.32s
  Collected 3641 NCollection/template typedefs
  Supplemented 80 additional NCollection typedefs from header text scan

Timing summary:
  Total clang parse time: 2.32s
  Total AST visit time: 4.20s

Building module dependency graph...

Generating code...

=== Generating FFI output ===

Generating ffi.rs...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi.rs (4400 classes, 1237 functions)
Generating wrappers.cpp...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp
Generating module re-exports...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/t_collection.rs (4 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/standard.rs (43 types, 9 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/t_col_std.rs (24 types, 35 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/message.rs (23 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/quantity.rs (7 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/osd.rs (52 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/top_abs.rs (0 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/gp.rs (38 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/top_loc.rs (5 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/topo_ds.rs (26 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/moni_tool.rs (14 types, 5 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/interface.rs (49 types, 3 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/resource.rs (4 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/step_data.rs (37 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/header_section.rs (4 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/if_graph.rs (11 types, 0 extra)
... (truncated, 32376 chars total)
```
</details>

**Terminal:**
```sh
tail -20 '/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_01Bbwyfn1P5ht65BU1Toa3bd__vscode-1771312785477/content.txt'
```
<details>
<summary>Output (20 lines)</summary>

```
torage.rs (extra types only, 1 types)                                             Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/b_rep
_bnd_lib.rs (function-only module, 4 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/preci
sion.rs (function-only module, 18 functions)                                      Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/hermi
t.rs (function-only module, 3 functions)                                          Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/geom_
proj_lib.rs (function-only module, 8 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/n_col
lection_primes.rs (function-only module, 1 functions)                             Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/step_
to_geom.rs (function-only module, 48 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/rw_ob
j_tools.rs (function-only module, 2 functions)                                    Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/el_s_
lib.rs (function-only module, 76 functions)                                       Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/el_c_
lib.rs (function-only module, 120 functions)                                      Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/lib.r
s                                                                               
Formatting generated Rust code with rustfmt...
  Formatting complete.

Code generation complete!
  263 modules generated

Bindings generated in /Users/sethml/src/opencascade-rs/crates/opencascade-sys/ge
nerated                                                                         real  53.4s
EXIT: 0
```
</details>

Bindings regenerated successfully. Let me check the skip counts and verify the standard module has the stream types:

**Terminal:**
```sh
grep -c '^// SKIPPED:' crates/opencascade-sys/generated/*.rs | awk -F: '{sum+=$2} END {print "Total skips:", sum}'; echo "---"; grep -n 'Standard_OStream\|Standard_IStream' crates/opencascade-sys/generated/ffi.rs | head -10; echo "---"; grep -n 'pub use.*OStream\|pub use.*IStream' crates/opencascade-sys/generated/standard.rs
```
<details>
<summary>Output (11 lines) — exit code 1</summary>

```
Total skips: 1566
---
30974:pub struct Standard_IStream {
30989:pub struct Standard_OStream {
281673:    /// Upcast LDOM_OSStream to Standard_OStream
281674:    pub fn LDOM_OSStream_as_Standard_OStream(
281676:    ) -> *const Standard_OStream;
281677:    /// Upcast LDOM_OSStream to Standard_OStream (mutable)
281678:    pub fn LDOM_OSStream_as_Standard_OStream_mut(
281680:    ) -> *mut Standard_OStream;
---
```
</details>

> 💭 Interesting. Skip count went from 1519 to 1566? That seems wrong - it should have decreased since we removed the stream type filter. Wait... actually, 1566 is more than 1519. That's suspicious. Maybe some methods that were previously skipped as "stream type" are now being skipped as "unknown type" because Standard_OStream is still not properly recognized?
> 
> Wait, let me think. We had 1519 skips before. We removed `is_stream()` from `is_unbindable()`. But also added manual types to `all_class_names`. Let me check:
> 
> 1. `Standard_OStream` and `Standard_IStream` are now in `all_class_names` — so they won't be caught by `type_uses_unknown_type()`
> 
> 
> 2. `Standard_SStream` is also in `manual_types` — so it won't be caught either
> 3. Methods using these types are no longer caught by `is_unbindable()` (is_stream removed)
> 
> But 1566 > 1519 means MORE methods are being skipped now, not fewer. That doesn't make sense if we removed a filter. Maybe `is_stream()` was catching some SStream methods that are now being classified differently, or there's something else going on with how the categorization changed.
> 
> Let me look at the `is_stream()` implementation — it checks for "OStream" or "IStream" in the type name, but `Standard_SStream` has an "S" not an "I" or "O", so it wouldn't match those patterns anyway. That means `Standard_SStream` was never being caught by `is_stream()` in the first place. It must have been handled through `type_uses_unknown_type()` since it's declared as a struct in the FFI bindings.
> 
> Now that I've added `Standard_SStream` to `manual_types`, it's in `all_class_names`, which should actually reduce skips, not increase them. But the count went from 1519 to 1566 — that's 47 more skips. I need to figure out what's causing this increase. Maybe removing `is_stream()` changed how multiple unbindable issues are being counted, or there's something else happening in the unbindability check that I'm not seeing.
> 
> Actually, I think the real issue is that `is_stream()` was catching types like `LDOM_OSStream` that contain "OStream" in their name. Now that I've removed it from `is_unbindable()`, those methods might be getting flagged by different skip reasons instead, or they're no longer being skipped at all when they should be.
> 
> But wait — `LDOM_OSStream` is actually a parsed class, so those methods should be bindable. Removing `is_stream()` would have unblocked them. The skip count increasing suggests something else is happening: maybe methods that had multiple skip reasons (like a stream param plus ambiguous lifetimes) are now hitting the second check instead of the first, getting recategorized under a different skip reason while still counting as one skip total.
> 
> Let me think about the return type handling for `MutRef(Class)` methods — if `Standard_OStream&` returns are now passing the bindability check since the stream filter is gone and `Standard_OStream` is in `all_class_names`, those would have been caught by ambiguous lifetime checks before. And if `LDOM_OSStream` inherits from `Standard_OStream`, that could be affecting the inheritance chain logic.
> 
> Actually, I should check the module-by-module skip counts to see where the increase is coming from. My baseline was 1519 before I started, and now it's 1566 — that's 47 additional skips. The issue might be that when `is_stream()` was part of `is_unbindable()`, methods with stream parameters got caught early by `has_unbindable_types()`, but now they're getting filtered out at a different stage.
> 
> Let me think through this: if a method has only stream parameters that are now in `all_class_names`, it should be bound instead of skipped. But if there are other types with "OStream" or "IStream" in their names that aren't `Standard_OStream` or `Standard_IStream` and aren't in `all_class_names`, those would still be caught by the old `is_stream()` logic and would now be skipped for a different reason.
> 
> The real question is whether the build even works with these changes. I can compare skip counts by module afterward, but first I need to verify the build succeeds. I should check where `is_stream()` is being called from—it might be affecting skip counting through a different code path than `is_unbindable()`. Let me trace through the resolver's skip-reason generation to see if that's where the extra skips are coming from, then I'll just build and debug from there.

The stream types are declared in ffi.rs. But skip count went to 1566 (from 1519) — I need to investigate that after the build. Let me first try building:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1; echo "EXIT: $?"
```
<details>
<summary>Output (184 lines)</summary>

```
   Compiling opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:2592:
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include/RWGltf_GltfOStreamWriter.hxx:1:
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/RWGltf/RWGltf_GltfOStreamWriter.hxx:19:10: fatal error: 'rapidjson/prettywriter.h' file not found
warning: opencascade-sys@0.2.0:    19 | #include <rapidjson/prettywriter.h>
warning: opencascade-sys@0.2.0:       |          ^~~~~~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: 1 error generated.
warning: opencascade-sys@0.2.0: ToolExecError: command did not execute successfully (status code exit status: 1): env -u IPHONEOS_DEPLOYMENT_TARGET LC_ALL="C" "sccache" "c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm64-apple-macosx" "-mmacosx-version-min=26.2" "-I" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include" "-I" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated" "-Wall" "-Wextra" "-std=c++14" "-Wno-unused-function" "-Wno-deprecated-declarations" "-Wno-return-type-c-linkage" "-Wno-delete-non-abstract-non-virtual-dtor" "-Wno-delete-abstract-non-virtual-dtor" "-D_USE_MATH_DEFINES=TRUE" "-o" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/74271c7d28a70e01-wrappers.o" "-c" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp"
error: failed to run custom build command for `opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)`

Caused by:
  process didn't exit successfully: `/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-489f9534c9bc0b7a/build-script-build` (exit status: 1)
  --- stdout
  cargo:rerun-if-env-changed=DEP_OCCT_ROOT
  cargo:rustc-link-search=native=/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/lib
  cargo:rustc-link-lib=static=TKMath
  cargo:rustc-link-lib=static=TKernel
  cargo:rustc-link-lib=static=TKDE
  cargo:rustc-link-lib=static=TKFeat
  cargo:rustc-link-lib=static=TKGeomBase
  cargo:rustc-link-lib=static=TKG2d
  cargo:rustc-link-lib=static=TKG3d
  cargo:rustc-link-lib=static=TKTopAlgo
  cargo:rustc-link-lib=static=TKGeomAlgo
  cargo:rustc-link-lib=static=TKBRep
  cargo:rustc-link-lib=static=TKPrim
  cargo:rustc-link-lib=static=TKDESTEP
  cargo:rustc-link-lib=static=TKDEI
... (truncated, 11599 chars total)
```
</details>

**Terminal:**
```sh
tail -30 '/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_01E8Ez1rhfBsBPBxeua3FkSz__vscode-1771312785480/content.txt'
```
<details>
<summary>Output (30 lines)</summary>

```
  HOST = Some(aarch64-apple-darwin)
  cargo:rerun-if-env-changed=CXXFLAGS
  CXXFLAGS = None
  cargo:rerun-if-env-changed=HOST_CXXFLAGS
  HOST_CXXFLAGS = None
  cargo:rerun-if-env-changed=CXXFLAGS_aarch64_apple_darwin
  CXXFLAGS_aarch64_apple_darwin = None
  cargo:rerun-if-env-changed=CXXFLAGS_aarch64-apple-darwin
  CXXFLAGS_aarch64-apple-darwin = None
  cargo:warning=In file included from /Users/sethml/src/opencascade-rs/crates/op
encascade-sys/generated/wrappers.cpp:2592:                                        cargo:warning=In file included from /Users/sethml/src/opencascade-rs/target/de
bug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include/RWGltf_GltfOStreamWriter.hxx:1:                                                     cargo:warning=/Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/RWGltf
/RWGltf_GltfOStreamWriter.hxx:19:10: fatal error: 'rapidjson/prettywriter.h' file not found                                                                       cargo:warning=   19 | #include <rapidjson/prettywriter.h>
  cargo:warning=      |          ^~~~~~~~~~~~~~~~~~~~~~~~~~
  cargo:warning=1 error generated.
  exit status: 0
  exit status: 0
  exit status: 0
  exit status: 1
  cargo:warning=ToolExecError: command did not execute successfully (status code
 exit status: 1): env -u IPHONEOS_DEPLOYMENT_TARGET LC_ALL="C" "sccache" "c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm64-apple-macosx" "-mmacosx-version-min=26.2" "-I" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include" "-I" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated" "-Wall" "-Wextra" "-std=c++14" "-Wno-unused-function" "-Wno-deprecated-declarations" "-Wno-return-type-c-linkage" "-Wno-delete-non-abstract-non-virtual-dtor" "-Wno-delete-abstract-non-virtual-dtor" "-D_USE_MATH_DEFINES=TRUE" "-o" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/74271c7d28a70e01-wrappers.o" "-c" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp"                                                       
  --- stderr


  error occurred in cc-rs: command did not execute successfully (status code exi
t status: 1): env -u IPHONEOS_DEPLOYMENT_TARGET LC_ALL="C" "sccache" "c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm64-apple-macosx" "-mmacosx-version-min=26.2" "-I" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include" "-I" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated" "-Wall" "-Wextra" "-std=c++14" "-Wno-unused-function" "-Wno-deprecated-declarations" "-Wno-return-type-c-linkage" "-Wno-delete-non-abstract-non-virtual-dtor" "-Wno-delete-abstract-non-virtual-dtor" "-D_USE_MATH_DEFINES=TRUE" "-o" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2
... (truncated, 3268 chars total)
```
</details>

> 💭 The build failed because of `RWGltf_GltfOStreamWriter.hxx` — this file includes `rapidjson/prettywriter.h` which is not available. This is happening because my removal of `is_stream()` from `is_unbindable()` caused `RWGltf_GltfOStreamWriter` to have methods that now reference `Standard_OStream`. But the issue is actually that this header is being included in wrappers.cpp because methods on `RWGltf_GltfOStreamWriter` are now being generated (they were previously skipped as "stream type").
> 
> Wait, actually looking more carefully - the fatal error is in `RWGltf_GltfOStreamWriter.hxx` at line 19: `#include <rapidjson/prettywriter.h>`. 
> 
> The real problem is that now that I'm generating wrapper functions for this class's methods, the header gets pulled in and the missing rapidjson dependency becomes visible. Since wrappers.cpp includes all headers anyway, the header was always there — but before, no wrapper code was actually being generated for these methods, so the compilation never failed. So the real issue is that `RWGltf_GltfOStreamWriter` wasn't generating any wrappers before because all its methods were being filtered out as stream-related, which meant its header never got included in wrappers.cpp. Now that stream methods are bindable, wrappers are actually being generated for this class, forcing the header to be included — and that header depends on rapidjson, which isn't available. The class name itself contains "OStream" which is why it was caught by the stream filtering logic in the first place. The simplest fix is to add this header to the exclude list in bindings.toml since it depends on rapidjson, which isn't available. Looking at the existing excludes for the RWGltf module, I should add `RWGltf_GltfOStreamWriter.hxx` alongside the others that are already listed. The reason this is surfacing now is that previously all its methods were being skipped because they had stream parameters, but now that those are being processed, the missing dependency becomes a problem.
> 
> Looking at the build output more carefully, I notice multiple successful exit statuses before the failure, which suggests wrappers.cpp is being split into multiple compilation units. The error might be happening in a specific chunk rather than affecting the entire build, so I need to understand how the includes are being distributed across these units.

Build failed because `RWGltf_GltfOStreamWriter.hxx` includes `<rapidjson/prettywriter.h>` which isn't available. This class was previously harmless because all its methods (stream-based) were skipped. Let me check if this is a new include:

**Terminal:**
```sh
git show HEAD:crates/opencascade-sys/generated/wrappers.cpp | grep -c 'RWGltf_GltfOStreamWriter'; echo "---"; grep -c 'RWGltf_GltfOStreamWriter' crates/opencascade-sys/generated/wrappers.cpp
```
```
0
---
1
```

> 💭 So `RWGltf_GltfOStreamWriter` was NOT in the previous wrappers.cpp and IS in the new one. This confirms: the class now has generated wrapper code (because its methods are no longer skipped as "stream type"), so its header gets included, and that fails because of the rapidjson dependency.
> 
> The fix: add `RWGltf_GltfOStreamWriter.hxx` to `exclude_headers` in bindings.toml.But wait, I should also check if there are other similar cases — headers that were previously not included because all their methods were stream-type skips, but now they ARE included and might have compilation issues.
> 
> 
> The old `is_stream()` check was a catch-all that filtered out anything with "OStream" or "IStream" in its name — including the actual stream typedefs like `Standard_OStream` and `Standard_IStream`, plus classes like `LDOM_OSStream` that inherit from `std::ostream`, and `RWGltf_GltfOStreamWriter` which just happens to have "OStream" in its name. Now that those typedefs are handled as manual types and the classes are being parsed normally, I need to identify which ones actually have problematic dependencies.
> 
> The real issue is that `RWGltf_GltfOStreamWriter.hxx` includes `rapidjson`, which causes compilation failures when the header gets pulled into wrappers.cpp through the generated wrapper functions. I should exclude just this header from the build rather than trying to prevent the class from being parsed altogether.

The `RWGltf_GltfOStreamWriter.hxx` header is newly included because its stream methods are now bindable, but it requires `rapidjson`. Let me add it to the excluded headers and check for similar issues:

**Terminal:**
```sh
diff <(git show HEAD:crates/opencascade-sys/generated/wrappers.cpp | grep '^#include' | sort) <(grep '^#include' crates/opencascade-sys/generated/wrappers.cpp | sort) | grep '^[><]' | head -30
```
```
> #include <RWGltf_GltfOStreamWriter.hxx>
> #include <Standard_IStream.hxx>
> #include <Standard_OStream.hxx>
```

Only 3 new includes: the two stream headers (which are fine) and `RWGltf_GltfOStreamWriter.hxx` (which needs rapidjson). Let me add it to the exclude list:

<details>
<summary>Reading lines 127-136 of <a href="../crates/opencascade-sys/bindings.toml#L127-L137">bindings.toml</a></summary>

```
    # Third-party dependency (rapidjson): these headers #include rapidjson which
    # is not bundled with our OCCT build.
    "RWGltf_GltfOStreamWriter.hxx",
    "RWGltf_GltfJsonParser.hxx",
    "RWGltf_CafWriter.hxx",
    "RWGltf_GltfAccessor.hxx",

    # BVH template classes: abstract base or complex template type params
    # that the generator can't resolve correctly.
    "BVH_Properties.hxx",
```
</details>

> 💭 Oh! `RWGltf_GltfOStreamWriter.hxx` is ALREADY in the exclude list! It's at line 129. So why is it being included in wrappers.cpp?
> 
> The issue must be that the exclude_headers applies only to the PARSING phase (which headers are parsed by libclang), and the header IS being excluded from parsing. But the wrappers.cpp includes are generated based on... something else.
> 
> Let me think. If the class isn't parsed (because its header is excluded), how did its methods end up in the generated code? They couldn't have. So the wrapper function that references this header must be for a DIFFERENT class that depends on `RWGltf_GltfOStreamWriter`.
>  
> Looking at the actual error message, the include is being pulled in transitively—some other header that's being parsed is including `RWGltf_GltfOStreamWriter.hxx`, which then tries to include the missing rapidjson header. I'm wondering if the header is being pulled in indirectly—maybe another class that was parsed inherits from `RWGltf_GltfOStreamWriter` or includes it, which would explain why it shows up in wrappers.cpp even though the class itself is excluded. Let me check what's actually on line 2592 to see what's triggering this include.

Wait, `RWGltf_GltfOStreamWriter.hxx` is already in the exclude_headers list! Let me check what's actually happening:

**Terminal:**
```sh
sed -n '2590,2595p' crates/opencascade-sys/generated/wrappers.cpp
```
<details>
<summary>Output (6 lines)</summary>

```
#include <RWGltf_GltfLatePrimitiveArray.hxx>
#include <RWGltf_GltfMaterialMap.hxx>
#include <RWGltf_GltfOStreamWriter.hxx>
#include <RWGltf_GltfPrimArrayData.hxx>
#include <RWGltf_GltfPrimitiveMode.hxx>
#include <RWGltf_GltfSceneNodeMap.hxx>
```
</details>

> 💭 So `RWGltf_GltfOStreamWriter.hxx` is explicitly included in wrappers.cpp as a direct include. This means the code generator decided to include it.
> 
> But why? If the header is in `exclude_headers`, the parser shouldn't have parsed it, and no classes from it should exist. Unless another class references it, or the include generation logic adds it independently.
> 
> Let me look at how the includes in wrappers.cpp are generated. Looking at the `generate_wrappers` function in cpp.rs:
> 
> The includes in wrappers.cpp come from the `all_headers` set built in main.rs (line 586-594):
> 
> 
> The code collects headers from every parsed class and function by gathering their `source_header` fields into a set. So the header must be in `all_headers` because some class has `source_header == "RWGltf_GltfOStreamWriter.hxx"`. But if the header was excluded from parsing, how did a class get parsed from it in the first place?
> 
> Looking at the exclude_headers logic in bindings.toml, I see that `RWGltf_GltfOStreamWriter.hxx` is already listed on line 129 as an excluded header. So it should have been filtered out before parsing — yet somehow a class from it still made it through. I'm wondering if the issue is that `RWGltf_GltfOStreamWriter` is being pulled in indirectly through some other mechanism, maybe via a command-line flag or transitive include.
> 
> Wait, I'm checking the code that handles the exclusion filtering. There's a post-resolution step that should catch this — it iterates through the headers and filters out anything matching the exclude_set. So the exclusion should be working. Let me trace through why it might not be. I need to verify whether `RWGltf_GltfOStreamWriter` actually shows up in the parsed classes, and if so, check the generated module file to understand where it's coming from.

**Terminal:**
```sh
grep -rn 'RWGltf_GltfOStreamWriter' crates/opencascade-sys/generated/rw_gltf.rs | head -10
```
<details>
<summary>Output (10 lines)</summary>

```
crates/opencascade-sys/generated/rw_gltf.rs:2034://   Reason: param 'theWriter' uses unknown type 'RWGltf_GltfOStreamWriter*'
crates/opencascade-sys/generated/rw_gltf.rs:2035://   // pub fn flush_glb_buffer_views(&mut self, theWriter: /* RWGltf_GltfOStreamWriter* */, theBinDataBufferId: i32, theBuffViewId: &mut i32);
crates/opencascade-sys/generated/rw_gltf.rs:2039://   Reason: param 'theWriter' uses unknown type 'RWGltf_GltfOStreamWriter*'
crates/opencascade-sys/generated/rw_gltf.rs:2040://   // pub fn flush_glb_images(&mut self, theWriter: /* RWGltf_GltfOStreamWriter* */);
crates/opencascade-sys/generated/rw_gltf.rs:2045://   Reason: param 'theWriter' uses unknown type 'RWGltf_GltfOStreamWriter*'
crates/opencascade-sys/generated/rw_gltf.rs:2046://   // pub fn add_images(&mut self, theWriter: /* RWGltf_GltfOStreamWriter* */, theStyle: &Style, theIsStarted: &mut bool);
crates/opencascade-sys/generated/rw_gltf.rs:2050://   Reason: param 'theWriter' uses unknown type 'RWGltf_GltfOStreamWriter*'
crates/opencascade-sys/generated/rw_gltf.rs:2051://   // pub fn add_material(&mut self, theWriter: /* RWGltf_GltfOStreamWriter* */, theStyle: &Style, theIsStarted: &mut bool);
crates/opencascade-sys/generated/rw_gltf.rs:2055://   Reason: param 'theWriter' uses unknown type 'RWGltf_GltfOStreamWriter*'
crates/opencascade-sys/generated/rw_gltf.rs:2056://   // pub fn add_textures(&mut self, theWriter: /* RWGltf_GltfOStreamWriter* */, theStyle: &Style, theIsStarted: &mut bool);
```
</details>

So `RWGltf_GltfOStreamWriter` is an unknown type referenced by OTHER classes in the rw_gltf module. It's not being parsed — the header IS excluded. But the type name appears in OTHER headers that reference it, and now it's being included because the `collect_referenced_types` function is collecting it (since it's no longer blocked by is_stream).

Let me look at how wrappers.cpp includes work — specifically the `known_headers` filter:

<details>
<summary>Searched for regex <code>include|header|known_header</code> (<code>**/crates/opencascade-binding-generator/src/codegen/cpp.rs</code>), 15 results</summary>

- [cpp.rs:21](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L21)
- [cpp.rs:80](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L80)
- [cpp.rs:81](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L81)
- [cpp.rs:91](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L91)
- [cpp.rs:99](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L99)
- [cpp.rs:102](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L102)
- [cpp.rs:104](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L104)
- [cpp.rs:105](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L105)
- [cpp.rs:106](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L106)
</details>

<details>
<summary>Reading lines 80-135 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L80-L136">cpp.rs</a></summary>

```
/// Collect headers needed for a type
fn collect_type_headers(ty: &Option<Type>, headers: &mut HashSet<String>, known_headers: &HashSet<String>) {
    if let Some(ty) = ty {
        // Skip unbindable types (arrays, streams, void pointers, etc.)
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
                            if known_headers.is_empty() || known_headers.contains(&header) {
                                headers.insert(header);
                            }
                        }
                        return;
                    }
                }
                // Skip types without underscore that aren't Standard* — likely nested types
                // whose qualified name was resolved by clang to just the leaf name
                if !name.contains('_') && !name.starts_with("Standard") {
                    return;
                }
                // Only include headers that actually exist in the OCCT include directory
                let header = format!("{}.hxx", name);
                if known_headers.is_empty() || known_headers.contains(&header) {
                    headers.insert(header);
                }
            }
            Type::Handle(name) => {
                let header = format!("{}.hxx", name);
                if known_headers.is_empty() || known_headers.contains(&header) {
                    headers.insert(header);
                }
                headers.insert("Standard_Handle.hxx".to_string());
            }
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                collect_type_headers(&Some(inner.as_ref().clone()), headers, known_headers);
            }
            _ => {}
        }
    }
```
</details>

Found it! Line 83-87:  also checks `is_unbindable()` and skips unbindable types. Since  is no longer in `is_unbindable()`, stream types now pass through, and  generates headers for them.

For `Standard_OStream`, the header `Standard_OStream.hxx` is fine. But the issue is that types like `RWGltf_GltfOStreamWriter` ALSO have "OStream" in their name and their headers (`RWGltf_GltfOStreamWriter.hxx`) are being collected. The  check passes because the file DOES exist in the OCCT include directory.

The fix needs to be in  — it should not include headers that are in the `exclude_headers` set. Or I need to filter the generated includes against the exclude list. Let me look at how the includes end up in wrappers.cpp:

Searched for regex `pub fn generate_wrappers` (`**/crates/opencascade-binding-generator/src/codegen/cpp.rs`), 1 result
- [cpp.rs:240](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L240)

<details>
<summary>Reading lines 240-310 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L240-L311">cpp.rs</a></summary>

```
pub fn generate_wrappers(
    all_classes: &[&ParsedClass],
    collections: &[super::collections::CollectionInfo],
    known_headers: &HashSet<String>,
    _symbol_table: &SymbolTable,
    all_bindings: &[super::bindings::ClassBindings],
    function_bindings: &[super::bindings::FunctionBinding],
    nested_types: &[super::rust::NestedTypeInfo],
    handle_able_classes: &HashSet<String>,
) -> String {
    let mut output = String::new();

    // Header guard and includes
    writeln!(output, "// Generated by opencascade-binding-generator").unwrap();
    writeln!(output, "// C++ wrappers for all OCCT modules").unwrap();
    writeln!(output).unwrap();
    writeln!(output, "#include <cstdint>").unwrap();
    writeln!(output, "#include <new>").unwrap();
    writeln!(output).unwrap();

    // Collect ALL headers needed
    let headers = collect_all_required_headers(all_classes, known_headers);
    
    for header in &headers {
        writeln!(output, "#include <{}>", header).unwrap();
    }
    writeln!(output).unwrap();

    // Generate Handle typedefs for ALL classes
    let handle_types = collect_handle_types(all_classes, handle_able_classes);
    if !handle_types.is_empty() {
        writeln!(output, "// Handle type aliases").unwrap();
        for (inner_class, handle_name) in &handle_types {
            writeln!(
                output,
                "typedef opencascade::handle<{}> {};",
                inner_class, handle_name
            )
            .unwrap();
        }
        writeln!(output).unwrap();

        // Handle type destructors
        writeln!(output, "// Handle type destructors").unwrap();
        for (_inner_class, handle_name) in &handle_types {
            writeln!(
                output,
                "extern \"C\" void {}_destructor({}* self_) {{ delete self_; }}",
                handle_name, handle_name
            )
            .unwrap();
        }
        writeln!(output).unwrap();
    }

    // Generate wrapper functions for ALL classes from pre-computed ClassBindings
    for bindings in all_bindings {
        output.push_str(&super::bindings::emit_cpp_class(bindings));
    }

    // Generate wrappers for ALL namespace-level free functions
    generate_function_wrappers(&mut output, function_bindings, known_headers);

    // Generate destructors for nested types and extra typedef types (e.g., gp_Vec3f)
    if !nested_types.is_empty() {
        writeln!(output).unwrap();
        writeln!(output, "// Nested type and typedef type destructors").unwrap();
        for nt in nested_types {
            writeln!(
                output,
                "extern \"C\" void {ffi}_destructor({cpp}* self_) {{ delete self_; }}",
```
</details>

Now let me look at `collect_all_required_headers`:

Searched for text `fn collect_all_required_headers` (`**/crates/opencascade-binding-generator/src/codegen/cpp.rs`), 1 result
- [cpp.rs:327](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L327)

<details>
<summary>Reading lines 327-410 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L327-L411">cpp.rs</a></summary>

```
fn collect_all_required_headers(
    classes: &[&ParsedClass],
    known_headers: &HashSet<String>,
) -> Vec<String> {
    let mut headers = HashSet::new();

    for class in classes {
        // Add header for the class itself - use the actual source header
        // (class name doesn't always match header name, e.g. Extrema_GlobOptFuncCCC0 is in Extrema_GlobOptFuncCC.hxx)
        let source_header = &class.source_header;
        if known_headers.is_empty() || known_headers.contains(source_header) {
            headers.insert(source_header.clone());
        } else {
            // Fallback: try class_name.hxx 
            let class_header = format!("{}.hxx", class.name);
            if known_headers.contains(&class_header) {
                headers.insert(class_header);
            }
        }

        // Add headers for types used in methods
        for method in &class.methods {
            collect_type_headers(&method.return_type, &mut headers, known_headers);
            for param in &method.params {
                collect_type_headers(&Some(param.ty.clone()), &mut headers, known_headers);
            }
        }

        for method in &class.static_methods {
            collect_type_headers(&method.return_type, &mut headers, known_headers);
            for param in &method.params {
                collect_type_headers(&Some(param.ty.clone()), &mut headers, known_headers);
            }
        }

        for ctor in &class.constructors {
            for param in &ctor.params {
                collect_type_headers(&Some(param.ty.clone()), &mut headers, known_headers);
            }
        }
    }

    let mut result: Vec<_> = headers.into_iter().collect();
    result.sort();
    result
}

```
</details>

> 💭 I see the issue now. `collect_all_required_headers` calls `collect_type_headers` for ALL methods of ALL parsed classes — not just the bound ones. Before, `collect_type_headers` would skip stream types (because `is_unbindable()` returned true for them). Now that `is_stream()` is removed from `is_unbindable()`, the function processes stream types and generates headers for them.
> 
> The type `RWGltf_GltfOStreamWriter` appears as a parameter type in methods of other RWGltf classes. Even though `RWGltf_GltfOStreamWriter.hxx` is in the exclude_headers list, the `collect_type_headers` function doesn't know about the exclude list — it only checks against `known_headers` (which is the set of all `.hxx` files that exist in the OCCT include dir). Since `RWGltf_GltfOStreamWriter.hxx` exists in the OCCT include dir, it passes the known_headers check.
> 
> 
> The real fix is to only include headers for types that are actually in the binding set. Since `RWGltf_GltfOStreamWriter` isn't in `all_class_names` because its header is excluded, I shouldn't be pulling in its header either. Alternatively, I could filter the final headers list against the exclude_headers set. is to pass the exclude_headers set to `generate_wrappers` and filter out excluded headers from the final include list. But looking deeper, the real issue is that `collect_all_required_headers` pulls headers for ALL methods across parsed classes, even the ones that get skipped later. The `is_unbindable()` check used to filter stream types, but that's too narrow — I need to filter the final headers list against the exclude_headers set from bindings.toml to catch any type that shouldn't be included, regardless of why it's being excluded.
> 
> Now I'm tracing through the call chain to figure out where to pass this parameter. `generate_output` in main.rs calls `generate_wrappers`, which internally calls `collect_all_required_headers`. The cleanest approach is to add the exclude set as a parameter to `generate_wrappers` and filter the headers there.
> 
> Actually, a better solution: I could pass `all_class_names` to `collect_type_headers` and only include headers for types that are actually in the known set. This way I'm not including headers for types that don't have corresponding classes.
> 
> But wait, that might be too restrictive — `collect_type_headers` is designed to be broad and catch any headers that might be needed. The real issue is that I'm collecting headers for ALL methods, including ones that get skipped and never generate wrapper code. I should only be collecting headers for methods that will actually have wrappers generated. Removing excluded headers from `known_headers` before passing it to `generate_output` would prevent excluded types from appearing in the generated code, which is exactly what I want — the header existence checks in `collect_type_headers` and `generate_function_wrappers` would naturally skip them. Implementing the set difference operation...

The issue is that  in `crates/opencascade-binding-generator/src/codegen/cpp.rs` iterates ALL method parameters (not just bound ones), and now that stream types aren't filtered by `is_unbindable()`, it generates include directives for types like `RWGltf_GltfOStreamWriter` that happen to appear in method signatures but whose headers are in the exclude list.

The cleanest fix: remove excluded headers from  so they can't be included in wrappers.cpp:

<details>
<summary>Reading lines 340-372 of <a href="../crates/opencascade-binding-generator/src/main.rs#L340-L373">main.rs</a></summary>

```
        return Ok(());
    }

    // Collect set of known header filenames that actually exist
    // This is used to filter out headers for types that don't have their own header files
    let known_headers: HashSet<String> = if !args.include_dirs.is_empty() {
        let occt_include_dir = &args.include_dirs[0];
        std::fs::read_dir(occt_include_dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter_map(|e| {
                        let path = e.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("hxx") {
                            path.file_name().and_then(|s| s.to_str()).map(|s| s.to_string())
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    } else {
        HashSet::new()
    };

    if args.verbose {
        println!("  Found {} known OCCT headers", known_headers.len());
    }

    // Generate FFI output
    generate_output(&args, &all_classes, &all_functions, &graph, &symbol_table, &known_headers, &exclude_methods, &handle_able_classes)
}
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L365-L371">main.rs -2+6</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    // Remove excluded headers from the known set so they won't be included\n    // in wrappers.cpp (e.g., RWGltf_GltfOStreamWriter.hxx depends on rapidjson\n    // which is not bundled)\n    let known_headers: HashSet<String> = known_headers.difference(&exclude_set).cloned().collect();\n\n    if args.verbose {",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "365:dd,366:av"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/main.rs","startLine":365,"endLine":370}
```
</details>

Now rebuild the binding generator and regenerate:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1; echo "EXIT: $?"
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.00s
real  3.1s
EXIT: 0
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1 | grep -E 'EXIT|error|263 modules' ; echo "EXIT: $?"
```

**Terminal:**
```sh
grep 'RWGltf_GltfOStreamWriter' crates/opencascade-sys/generated/wrappers.cpp
```
```
(base) whitelily:opencascade-rs sethml$
```
**Exit code:** 1

Header excluded. Now let me build:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1; echo "EXIT: $?"
```

Build succeeds. Let me check the skip counts and verify the manual standard module was included:

**Terminal:**
```sh
grep -c '^// SKIPPED:' crates/opencascade-sys/generated/*.rs | awk -F: '{sum+=$2} END {print "Total skips:", sum}'; echo "---"; grep 'Manual bindings' crates/opencascade-sys/generated/standard.rs
```
```
Total skips: 1566
---
// Manual bindings:
// Manual bindings for C++ iostream global objects (std::cout, std::cerr, etc.)
```

> 💭 The manual bindings are properly included. Total skips went from 1519 to 1566 — that's 47 MORE skips. But we should have FEWER skips since stream methods are now bindable. Let me investigate this.
> 
> Wait, 1566 vs 1519 — the increase is 47. Previously we had 546 stream type skips. Now they should mostly be zero. But there might be new skips from other categories. Let me check the skip reason breakdown.
> 
> Actually wait — with the stream types unblocked, methods that were previously skipped as "stream type" might now pass the first check (has_unbindable_types) but THEN get caught by:
> 
> 
> - References to other unknown types besides stream types
> - Ambiguous lifetimes when returning MutRef with reference parameters
> - Various other skip reasons
> 
> So the "stream type" category should drop to nearly zero, but other categories might increase to compensate. Let me check which modules actually changed in skip count, and whether the 1566 total includes the Standard_SStream methods that were previously blocked as unknown types and are now unblocked.
> 
> Before the changes I had 1519 total skipped methods, with 546 in the stream type category and 432 in the unknown/unresolved type category. After removing the is_stream check and adding manual type mappings, the stream type category should drop to zero or close to it, the unknown type category should decrease since Standard_SStream is now manually typed, but many formerly-stream methods might now get caught by ambiguous lifetimes or other unknown type reasons. The net effect should be a decrease, so 1566 being higher than 1519 is unexpected—I need to recount the skip reasons to see what's actually happening.

The skip count went UP by 47 (1519 → 1566). Let me investigate — some stream methods must now be hitting other skip reasons:

**Terminal:**
```sh
grep -h '^//   Reason:' crates/opencascade-sys/generated/*.rs | sort | sed 's/^\/\/   Reason: //' | sed 's/ (.*//' | sort | uniq -c | sort -rn | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
 222 param 'S' uses unknown type 'Standard_OStream&'
 173 returns &mut with reference params
  85 param 'theMessage' uses unknown type 'Standard_SStream&'
  55 param 'OS' uses unknown type 'Standard_OStream&'
  48 param 'o' uses unknown type 'Standard_OStream&'
  46 param 'theDocument' uses unknown type 'const Handle(TDocStd_Document)&'
  46 has unbindable types: param 'S': void pointer
  43 class is abstract
  42 has unbindable types: param 'C': void pointer
  27 param 'theDoc' uses unknown type 'const Handle(TDocStd_Document)&'
  25 has unbindable types: return: unresolved template type
  24 param 'anOStream' uses unknown type 'Standard_OStream&'
  23 param 'IS' uses unknown type 'Standard_IStream&'
  22 has unbindable types: param 'PCurve': void pointer
  16 param 'theStream' uses unknown type 'Standard_OStream&'
  15 has unbindable types: return: raw pointer
  14 return type 'void*' is unknown
  14 param 'theStyle' uses unknown type 'const XCAFPrs_Style&'
  14 has unbindable types: return: void pointer
  13 has unbindable types: param 'ndimen': raw pointer
```
</details>

> 💭 The stream types are STILL being caught as "unknown type"! Look:
> - 222: `param 'S' uses unknown type 'Standard_OStream&'`
> - 85: `param 'theMessage' uses unknown type 'Standard_SStream&'`
> - 55: `param 'OS' uses unknown type 'Standard_OStream&'`
> - 48: `param 'o' uses unknown type 'Standard_OStream&'`- 23: `param 'IS' uses unknown type 'Standard_IStream&'`- etc.They pass the `is_unbindable()` check (good), but then get caught by `type_uses_unknown_type()`. 
> 
> 
> The issue is that `type_uses_unknown_type()` validates against `all_classes`, but I added the manual type names to `all_class_names` in the resolver for symbol resolution. When `compute_all_class_bindings()` creates a `TypeContext`, I need to verify it's actually using the resolver's `all_class_names` instead of just the `all_classes` set.
> 
> Looking at how the bindings function constructs the TypeContext from the symbol table's all_class_names to ensure the manual types are included in the validation.

The stream types are still being caught as "unknown type" even though I added them to `all_class_names` in the resolver. Let me trace how the TypeContext is built in `compute_all_class_bindings`:

<details>
<summary>Searched for regex <code>TypeContext|all_class_names|compute_all_class_bindings</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 15 results</summary>

- [bindings.rs:11](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L11)
- [bindings.rs:496](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L496)
- [bindings.rs:498](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L498)
- [bindings.rs:632](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L632)
- [bindings.rs:639](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L639)
- [bindings.rs:668](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L668)
- [bindings.rs:709](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L709)
- [bindings.rs:874](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L874)
- [bindings.rs:938](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L938)
- [bindings.rs:960](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L960)
- [bindings.rs:1013](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1013)
- [bindings.rs:1102](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1102)
- [bindings.rs:1282](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1282)
</details>

Searched for text `pub fn compute_all_class_bindings` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:3020](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3020)

<details>
<summary>Reading lines 3020-3100 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3020-L3101">bindings.rs</a></summary>

```
pub fn compute_all_class_bindings(
    all_classes: &[&ParsedClass],
    symbol_table: &SymbolTable,
    collection_names: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
    exclude_methods: &HashSet<(String, String)>,
) -> Vec<ClassBindings> {
    // Classes with CppDeletable impls: ParsedClasses (without protected dtor) +
    // the manually-specified known collections (which get generated destructors) +
    // NCollection typedef names from extra_typedef_names (e.g. gp_Vec3f, gp_Pnt2f).
    // Nested types (Parent::Nested) get destructors generated, so include them too.
    let mut deletable_class_names: HashSet<String> = all_classes
        .iter()
        .filter(|c| !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .chain(collection_names.iter().cloned())
        .chain(extra_typedef_names.iter().cloned())
        .collect();

    // Add nested types (those with :: in their name) as deletable
    // since we generate destructors for them
    let known_class_names: HashSet<&str> = all_classes.iter().map(|c| c.name.as_str()).collect();
    for class in all_classes {
        for method in &class.methods {
            if let Some(ref ret) = method.return_type {
                collect_nested_deletable_names(ret, &known_class_names, &mut deletable_class_names);
            }
            for param in &method.params {
                collect_nested_deletable_names(&param.ty, &known_class_names, &mut deletable_class_names);
            }
        }
        for method in &class.static_methods {
            if let Some(ref ret) = method.return_type {
                collect_nested_deletable_names(ret, &known_class_names, &mut deletable_class_names);
            }
            for param in &method.params {
                collect_nested_deletable_names(&param.ty, &known_class_names, &mut deletable_class_names);
            }
        }
    }

    // Full known-type set (for param filtering): adds NCollection template typedefs
    // so methods passing them as params pass the unknown-type filter.
    let mut all_class_names: HashSet<String> =
        all_classes.iter().map(|c| c.name.clone()).collect();
    all_class_names.extend(collection_names.iter().cloned());
    all_class_names.extend(extra_typedef_names.iter().cloned());
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

    let all_classes_by_name: HashMap<String, &ParsedClass> = all_classes
        .iter()
        .map(|c| (c.name.clone(), *c))
        .collect();


    let class_public_info = build_class_public_info(all_classes);

    all_classes
        .iter()
        .map(|class| {
            let reexport_ctx = ReexportTypeContext {
                class_public_info: &class_public_info,
                current_module_rust: crate::module_graph::module_to_rust_name(&class.module),
            };
            compute_class_bindings(class, &ffi_ctx, symbol_table, &handle_able_classes, &all_classes_by_name, Some(&reexport_ctx), exclude_methods)
        })
        .collect()
}
```
</details>

Found it! Lines 3063-3066 build `all_class_names` from:
1. Parsed class names
2. Collection names  
3. Extra typedef names

But NOT from the manual types! The resolver's `all_class_names` is used in the resolver for symbol resolution, but  builds its OWN `all_class_names` separately. I need to pass the manual types here too.

Let me also check `compute_all_function_bindings`:

Searched for text `pub fn compute_all_function_bindings` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:3175](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3175)

<details>
<summary>Reading lines 3175-3250 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3175-L3251">bindings.rs</a></summary>

```
pub fn compute_all_function_bindings(
    symbol_table: &SymbolTable,
    all_classes: &[&ParsedClass],
    collection_names: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
    known_headers: &HashSet<String>,
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
```
</details>

Same pattern — both functions build their own `all_class_names` from parsed classes + collections + extra typedefs, but NOT manual types. I need to add the manual types parameter to both functions.

Let me update both  and  to accept and include manual types:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3020-L3028">bindings.rs -7+8</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3064-L3070">-5+6</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "pub fn compute_all_class_bindings(\n    all_classes: &[&ParsedClass],\n    symbol_table: &SymbolTable,\n    collection_names: &HashSet<String>,\n    extra_typedef_names: &HashSet<String>,\n    exclude_methods: &HashSet<(String, String)>,\n    manual_type_names: &HashSet<String>,\n) -> Vec<ClassBindings> {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "3020:dh,3021:ue,3022:ca,3023:jg,3024:td,3025:rv,3026:ee"
    },
    {
      "content": "    let mut all_class_names: HashSet<String> =\n        all_classes.iter().map(|c| c.name.clone()).collect();\n    all_class_names.extend(collection_names.iter().cloned());\n    all_class_names.extend(extra_typedef_names.iter().cloned());\n    all_class_names.extend(manual_type_names.iter().cloned());\n    let all_enum_names = &symbol_table.all_enum_names;",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "3063:hj,3064:sd,3065:yw,3066:iu,3067:ph"
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
      "file": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "startLine": 3020,
      "endLine": 3069
    }
  ]
}
```
</details>

Now update `compute_all_function_bindings`:

<details>
<summary>Reading lines 3175-3215 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3175-L3216">bindings.rs</a></summary>

```
/// for free functions, using the same `overload_suffix_for_types` / `combine_name_suffix`
/// logic as class methods. The result is shared by all three output generators.
pub fn compute_all_function_bindings(
    symbol_table: &SymbolTable,
    all_classes: &[&ParsedClass],
    collection_names: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
    known_headers: &HashSet<String>,
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
    let all_enum_names = &symbol_table.all_enum_names;

    let handle_able_classes = compute_handle_able_classes(all_classes);

```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3177-L3185">bindings.rs -7+8</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3209-L3215">-5+6</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "pub fn compute_all_function_bindings(\n    symbol_table: &SymbolTable,\n    all_classes: &[&ParsedClass],\n    collection_names: &HashSet<String>,\n    extra_typedef_names: &HashSet<String>,\n    known_headers: &HashSet<String>,\n    manual_type_names: &HashSet<String>,\n) -> (Vec<FunctionBinding>, Vec<SkippedSymbol>) {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "3177:zq,3178:ca,3179:ue,3180:jg,3181:td,3182:ai,3183:qo"
    },
    {
      "content": "    let mut all_class_names: HashSet<String> =\n        all_classes.iter().map(|c| c.name.clone()).collect();\n    all_class_names.extend(collection_names.iter().cloned());\n    all_class_names.extend(extra_typedef_names.iter().cloned());\n    all_class_names.extend(manual_type_names.iter().cloned());\n    let all_enum_names = &symbol_table.all_enum_names;",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "3208:hj,3209:sd,3210:yw,3211:iu,3212:ph"
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
      "file": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "startLine": 3177,
      "endLine": 3214
    }
  ]
}
```
</details>

Now update the call sites in main.rs:

<details>
<summary>Reading lines 600-620 of <a href="../crates/opencascade-binding-generator/src/main.rs#L600-L621">main.rs</a></summary>

```
    let all_headers_list: Vec<String> = all_headers.into_iter().collect();

    // Get all collections
    let all_collections = codegen::collections::all_known_collections();

    // Compute ClassBindings once for ALL classes — shared by all three generators
    let collection_type_names: std::collections::HashSet<String> =
        all_collections.iter().map(|c| c.typedef_name.clone()).collect();
    let extra_typedef_names = parser::get_collected_typedef_names();
    let all_bindings =
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods);

    // Compute FunctionBindings once for ALL free functions — shared by all three generators
    let (all_function_bindings, all_skipped_functions) = codegen::bindings::compute_all_function_bindings(
        symbol_table, all_classes, &collection_type_names, &extra_typedef_names, known_headers,
    );

    // Track generated files for formatting
    let mut generated_rs_files: Vec<PathBuf> = Vec::new();

    // 1. Generate ffi.rs
```
</details>

> 💭 I need to pass `manual_type_names` to both function calls. But `generate_output` doesn't have access to it. Let me check the function signature of `generate_output`:

I need to pass `manual_type_names` to `generate_output`:

<details>
<summary>Reading lines 578-590 of <a href="../crates/opencascade-binding-generator/src/main.rs#L578-L591">main.rs</a></summary>

```
fn generate_output(
    args: &Args,
    all_classes: &[&model::ParsedClass],
    all_functions: &[&model::ParsedFunction],
    graph: &module_graph::ModuleGraph,
    symbol_table: &resolver::SymbolTable,
    known_headers: &HashSet<String>,
    exclude_methods: &HashSet<(String, String)>,
    handle_able_classes: &HashSet<String>,
) -> Result<()> {
    use model::ParsedClass;

    println!("\n=== Generating FFI output ===\n");
```
</details>

<details>
<summary>Reading lines 375-378 of <a href="../crates/opencascade-binding-generator/src/main.rs#L375-L379">main.rs</a></summary>

```
    generate_output(&args, &all_classes, &all_functions, &graph, &symbol_table, &known_headers, &exclude_methods, &handle_able_classes)
}

/// Detect "utility namespace classes" and convert their static methods to free functions.
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L375-L377">main.rs -2+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L578-L589">-10+11</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L610-L612">-2+2</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L614-L617">-3+3</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    generate_output(&args, &all_classes, &all_functions, &graph, &symbol_table, &known_headers, &exclude_methods, &handle_able_classes, &manual_type_names)\n}",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "375:dl,376:io"
    },
    {
      "content": "fn generate_output(\n    args: &Args,\n    all_classes: &[&model::ParsedClass],\n    all_functions: &[&model::ParsedFunction],\n    graph: &module_graph::ModuleGraph,\n    symbol_table: &resolver::SymbolTable,\n    known_headers: &HashSet<String>,\n    exclude_methods: &HashSet<(String, String)>,\n    handle_able_classes: &HashSet<String>,\n    manual_type_names: &HashSet<String>,\n) -> Result<()> {",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "578:ye,579:zb,580:pv,581:mq,582:yb,583:uu,584:ai,585:rv,586:kf,587:gv"
    },
    {
      "content": "    let all_bindings =\n        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods, manual_type_names);",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "609:fy,610:ez"
    },
    {
      "content": "    let (all_function_bindings, all_skipped_functions) = codegen::bindings::compute_all_function_bindings(\n        symbol_table, all_classes, &collection_type_names, &extra_typedef_names, known_headers, manual_type_names,\n    );",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "613:jm,614:la,615:pd"
    }
  ]
}
```
**Output:**
```
{
  "applied": 4,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/main.rs",
      "startLine": 375,
      "endLine": 616
    }
  ]
}
```
</details>

Now rebuild, regenerate, and build:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1; echo "EXIT: $?"
```
<details>
<summary>Output (5 lines)</summary>

```
    Blocking waiting for file lock on build directory
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 9.88s
real  10.0s
EXIT: 0
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1 | grep -E 'EXIT|error|modules generated'; echo "EXIT: $?"
```

**Terminal:**
```sh
grep -c '^// SKIPPED:' crates/opencascade-sys/generated/*.rs | awk -F: '{sum+=$2} END {print "Total skips:", sum}'; echo "---"; grep -h '^//   Reason:' crates/opencascade-sys/generated/*.rs | sed 's/^\/\/   Reason: //' | sed 's/ (.*//' | sort | uniq -c | sort -rn | head -15
```
<details>
<summary>Output (17 lines)</summary>

```
Total skips: 1033
---
 173 returns &mut with reference params
  46 param 'theDocument' uses unknown type 'const Handle(TDocStd_Document)&'
  46 has unbindable types: param 'S': void pointer
  43 class is abstract
  42 has unbindable types: param 'C': void pointer
  27 param 'theDoc' uses unknown type 'const Handle(TDocStd_Document)&'
  25 has unbindable types: return: unresolved template type
  22 has unbindable types: param 'PCurve': void pointer
  15 has unbindable types: return: raw pointer
  14 return type 'void*' is unknown
  14 param 'theStyle' uses unknown type 'const XCAFPrs_Style&'
  14 has unbindable types: return: void pointer
  13 has unbindable types: param 'ndimen': raw pointer
  13 has unbindable types: param 'C1': void pointer
  12 has unbindable types: param 'arg0': raw pointer
```
</details>

1519 → 1033! That's **486 methods unblocked**. The stream type category is completely gone from the top skip reasons. The 173 "returns &mut with reference params" are the stream-returning methods caught by the ambiguous lifetime check — that's expected.

Now let me build:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1; echo "EXIT: $?"
```

> 💭 Three free functions have ambiguous lifetime issues: they return `&mut Standard_OStream` but have multiple reference parameters. The "ambiguous lifetime" check in the generator catches this for INSTANCE methods, but apparently not for FREE FUNCTIONS.
> 
> Looking at the error:
> 1. `t_naming::print_label_ostream` — free function returning `&mut Standard_OStream` with 2 ref params
> 2. `top_ope_b_rep_ds::print_kind_int_ostream_asciistring2` — free function returning `&mut Standard_OStream` with 3 ref params
> 3. `vrml::comment_writer` — free function returning `&mut Standard_OStream` with 2 ref params
> 
> 
> The ambiguous lifetime check is only applied to methods and constructors, not to free functions. I need to find where free functions are being filtered for bindability and add the same lifetime check there. I need to apply that same ambiguous lifetime check to free functions—counting reference parameters and rejecting cases where there's a mutable return type alongside one or more reference parameters.

Three free functions have ambiguous lifetime issues — the generator's ambiguous lifetime check apparently doesn't cover free functions. Let me find the free function filtering code and add the check:

Searched for regex `unbindable_param.*func\.params|free function.*filter|skip_reason.*unbindable` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 2 results
- [bindings.rs:3270](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3270)
- [bindings.rs:3300](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3300)

<details>
<summary>Reading lines 3260-3340 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3260-L3341">bindings.rs</a></summary>

```
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

        let base_rust_name = &func.rust_name;
        let is_overloaded = name_groups.get(base_rust_name).copied().unwrap_or(0) > 1;

```
</details>

I need to add an ambiguous lifetime check for free functions after the return type unknown/unbindable checks. Let me add it after line 3336:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3335-L3361">bindings.rs -2+26</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            }\n        }\n\n        // Ambiguous lifetime check for free functions:\n        // If the function returns &mut and has reference params, Rust can't infer\n        // which param the return borrows from.\n        if let Some(ref ret) = func.return_type {\n            if matches!(&ret.original, Type::MutRef(_)) {\n                let ref_param_count = func.params.iter().filter(|p| {\n                    matches!(&p.ty.original, Type::ConstRef(_) | Type::MutRef(_))\n                }).count();\n                if ref_param_count >= 2 {\n                    skipped.push(SkippedSymbol {\n                        kind: \"function\",\n                        module: func.rust_module.clone(),\n                        cpp_name: format!(\"{}::{}\", func.namespace, func.short_name),\n                        source_header: func.source_header.clone(),\n                        source_line: func.source_line,\n                        doc_comment: func.doc_comment.clone(),\n                        skip_reason: \"returns &mut with reference params \\u{2014} ambiguous lifetime\".to_string(),\n                        stub_rust_decl: generate_function_stub(func),\n                    });\n                    continue;\n                }\n            }\n        }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "3335:uo,3336:cy"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":3335,"endLine":3360}
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.84s
real  3.9s
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```
<details>
<summary>Output (318 lines)</summary>

```
Building binding generator...
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `release` profile [optimized] target(s) in 6.06s
Generating bindings from /Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml...
Cleaning /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated...
  Excluded 73 modules via exclude_modules
Loaded config: 1 module patterns, 75 exclude module patterns, 0 individual headers, 10 header exclusions -> 5761 headers
  Excluded 35 headers after dependency resolution
Parsing 5761 headers...
  Clang parse time: 1.80s
  Collected 3641 NCollection/template typedefs
  Supplemented 80 additional NCollection typedefs from header text scan

Timing summary:
  Total clang parse time: 1.80s
  Total AST visit time: 3.11s

Building module dependency graph...

Generating code...

=== Generating FFI output ===

Generating ffi.rs...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi.rs (4400 classes, 1237 functions)
Generating wrappers.cpp...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp
Generating module re-exports...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/t_collection.rs (4 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/standard.rs (43 types, 9 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/t_col_std.rs (24 types, 35 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/message.rs (23 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/quantity.rs (7 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/osd.rs (52 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/top_abs.rs (0 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/gp.rs (38 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/top_loc.rs (5 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/topo_ds.rs (26 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/moni_tool.rs (14 types, 5 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/interface.rs (49 types, 3 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/resource.rs (4 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/step_data.rs (37 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/header_section.rs (4 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/if_graph.rs (11 types, 0 extra)
... (truncated, 32366 chars total)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep "Total skips" crates/opencascade-sys/generated/skip_report.md
```
```
grep: crates/opencascade-sys/generated/skip_report.md: No such file or directory
```
**Exit code:** 2

**Terminal:**
```sh
grep -c "returns &mut with reference params" crates/opencascade-sys/generated/skipped_symbols.md && grep "^Total" crates/opencascade-sys/generated/skipped_symbols.md
```
```
grep: crates/opencascade-sys/generated/skipped_symbols.md: No such file or directory
```
**Exit code:** 2

**Terminal:**
```sh
grep -r "Total skips\|total skips" crates/opencascade-sys/generated/ 2>/dev/null; find crates/opencascade-sys/generated/ -name "*skip*" 2>/dev/null
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
grep -c "skipped" crates/opencascade-sys/generated/binding_report.md 2>/dev/null || find crates/opencascade-sys/generated/ -name "*report*" 2>/dev/null
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
grep -r "Total\|total\|skipped\|Skipped" crates/opencascade-sys/generated/README.md 2>/dev/null; ls crates/opencascade-sys/generated/*.md 2>/dev/null; ls crates/opencascade-sys/generated/*.txt 2>/dev/null
```
```
(base) whitelily:opencascade-rs sethml$
```
**Exit code:** 1

**Terminal:**
```sh
grep "Skipped\|skipped" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_014xCfu7gTrnpUrvov1vBpnm__vscode-1771312785521/content.txt | tail -5
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
grep -i "skip" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_014xCfu7gTrnpUrvov1vBpnm__vscode-1771312785521/content.txt | wc -l
```
```
       0
```

Let me just build and see if the lifetime errors are fixed:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

> 💭 Only 1 error left (down from 3). The `vrml.rs:21` function `comment_writer` still has the issue. Let me check why - it has 2 reference params (`&str` and `&mut Standard_OStream`). My check counts `ConstRef(_)` and `MutRef(_)` but `&str` might not be represented as `ConstRef` or `MutRef` in the type model. Let me check what type `&str` maps to.
> 
> `&str` likely doesn't match `Type::ConstRef(_)` or `Type::MutRef(_)` - it might be stored as something else. 
> 
> Looking at the actual function signature, I see `&str` is being used for the comment parameter. In the type model, string references probably aren't represented the same way as other const references - they might be a special case or mapped to a different type variant entirely. the semantic meaning of a reference in Rust, so I need to account for how `&str` parameters are represented in the type model—they might be pointers or special string types rather than explicit reference types, which means my current filter is too narrow. a lifetime. So the ambiguity check needs to also count parameters that become borrowed references in Rust, not just those that are references in C++.
> 
> Actually, the issue is simpler: in Rust, the generated code has `aComment: &str` and `anOStream: &mut Standard_OStream`. Both carry lifetimes. The return type `&mut Standard_OStream` can't determine which it borrows from.
> 
> I need to check what types result in `&` or `&mut` in the generated Rust. Let me look at how the Rust type names are generated for function params. The `p.ty.original` types that result in reference types in Rust include:
> - `Type::ConstRef(...)` → `&T`
> - `Type::MutRef(...)` → `&mut T`
> - String types that get mapped to `&str`
> 
> The simplest approach is probably to broaden the ambiguity check: if the return type is a mutable reference and there are multiple parameters with lifetimes—whether they're const refs, mutable refs, or string types that map to `&str`—then I should skip the binding to avoid the ambiguity problem.

One error remains - `vrml.rs:21` with `&str` param. The `&str` type isn't `ConstRef`/`MutRef` in the model. Let me check how the method-level check handles this:

Searched for text `returns &mut with reference params` (`**/crates/opencascade-binding-generator/src/**`), 2 results
- [bindings.rs:890](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L890)
- [bindings.rs:3354](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3354)

<details>
<summary>Reading lines 875-900 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L875-L901">bindings.rs</a></summary>

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
```
</details>

<details>
<summary>Searched for text <code>fn method_needs_explicit_lifetimes</code>, 18 results</summary>

- [claude-conversation-2026-02-05-0e611961.md:2984](../agent-logs/claude-conversation-2026-02-05-0e611961.md#L2984)
- [2026-02-18_18-30-vscode-claude-opus-4.6.md:325](../agent-logs/2026-02-18_18-30-vscode-claude-opus-4.6.md#L325)
- [2026-02-18_19-00-vscode-claude-opus-4.6.md:325](../agent-logs/2026-02-18_19-00-vscode-claude-opus-4.6.md#L325)
- [resolver.rs:581](../crates/opencascade-binding-generator/src/resolver.rs#L581)
- [2026-02-02_21-01_vscode-claude-opus-4.5.json:87660](../agent-logs/2026-02-02_21-01_vscode-claude-opus-4.5.json#L87660)
- [2026-02-02_21-01_vscode-claude-opus-4.5.json:99153](../agent-logs/2026-02-02_21-01_vscode-claude-opus-4.5.json#L99153)
- [2026-02-02_21-01_vscode-claude-opus-4.5.json:167636](../agent-logs/2026-02-02_21-01_vscode-claude-opus-4.5.json#L167636)
- [2026-02-02_21-01_vscode-claude-opus-4.5.json:195080](../agent-logs/2026-02-02_21-01_vscode-claude-opus-4.5.json#L195080)
- [2026-02-02_21-01_vscode-claude-opus-4.5.json:195252](../agent-logs/2026-02-02_21-01_vscode-claude-opus-4.5.json#L195252)
- [2026-02-02_21-01_vscode-claude-opus-4.5.json:195336](../agent-logs/2026-02-02_21-01_vscode-claude-opus-4.5.json#L195336)
- [2026-02-02_21-01_vscode-claude-opus-4.5.json:195719](../agent-logs/2026-02-02_21-01_vscode-claude-opus-4.5.json#L195719)
- [2026-02-02_21-01_vscode-claude-opus-4.5.json:207374](../agent-logs/2026-02-02_21-01_vscode-claude-opus-4.5.json#L207374)
- [2026-02-02_21-01_vscode-claude-opus-4.5.json:210913](../agent-logs/2026-02-02_21-01_vscode-claude-opus-4.5.json#L210913)
- [2026-02-02_21-01_vscode-claude-opus-4.5.json:217884](../agent-logs/2026-02-02_21-01_vscode-claude-opus-4.5.json#L217884)
</details>

<details>
<summary>Reading lines 581-620 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L581-L621">resolver.rs</a></summary>

```
pub fn method_needs_explicit_lifetimes(method: &Method) -> bool {
    // Check if return type is a mutable reference (&mut Self or MutRef)
    let returns_mut_ref = method.return_type.as_ref().map(|ty| {
        matches!(ty, Type::MutRef(_))
    }).unwrap_or(false);
    
    if !returns_mut_ref {
        return false;
    }
    
    // Check if any parameter is a reference (other than self which is handled separately)
    // Also treat const char* as a reference parameter
    method.params.iter().any(|p| {
        matches!(&p.ty, Type::ConstRef(_) | Type::MutRef(_)) || p.ty.is_c_string()
    })
}

/// Check if a const method returns a mutable reference (not allowed at FFI boundary)
/// The FFI requires &mut self when returning &mut, but C++ allows const methods to return non-const refs
pub fn has_const_mut_return_mismatch(method: &Method) -> bool {
    if !method.is_const {
        return false;
    }
    // Check if return type is a mutable reference
    method.return_type.as_ref().map(|ty| {
        matches!(ty, Type::MutRef(_))
    }).unwrap_or(false)
}

/// Check if a method has unsupported by-value parameters.
/// By-value enums (Type::Class) are supported (mapped to i32).
/// By-value classes and Handles are now supported (C++ wrappers accept const T&).
/// MutRef to enums are NOT supported (output params need local variable + writeback).
pub fn method_has_unsupported_by_value_params(_method: &Method, _all_enum_names: &HashSet<String>) -> Option<(String, String)> {
    // MutRef to enum params are now handled via C++ wrapper with local variable + writeback.
    // No remaining unsupported param types.
    None
}

/// Check if a static method has unsupported by-value parameters.
```
</details>

> 💭 The method check at line 594 uses `matches!(&p.ty, Type::ConstRef(_) | Type::MutRef(_)) || p.ty.is_c_string()`. The key is `p.ty.is_c_string()` — this catches the `const char*` type that gets mapped to `&str` in Rust.
> 
> For free functions, `p.ty` is the type itself (not `p.ty.original`). Let me check the free function param types. In `compute_all_function_bindings`, free function params are `func.params` where each param has `p.ty` of type `Type`. 
> 
> I need to verify the data structure — `Param.ty` should be a `Type` directly, but I'm seeing references to `.original` which suggests there might be a `ResolvedType` wrapper involved. Let me trace through whether free function params use the same structure as method params.

Searched for text `struct ParsedFunction` (`**/crates/opencascade-binding-generator/src/**`), 1 result
- [model.rs:26](../crates/opencascade-binding-generator/src/model.rs#L26)

<details>
<summary>Reading lines 26-65 of <a href="../crates/opencascade-binding-generator/src/model.rs#L26-L66">model.rs</a></summary>

```
pub struct ParsedFunction {
    /// Full function name (e.g., "TopoDS::Edge")
    pub name: String,
    /// Namespace name (e.g., "TopoDS")
    pub namespace: String,
    /// Simple function name without namespace (e.g., "Edge")
    pub short_name: String,
    /// Module name derived from namespace
    pub module: String,
    /// Documentation comment
    pub comment: Option<String>,
    /// Source header file name (e.g., "TopoDS.hxx")
    pub source_header: String,
    /// Source line number in the header file
    pub source_line: Option<u32>,
    /// Parameters
    pub params: Vec<Param>,
    /// Return type (None for void)
    pub return_type: Option<Type>,
}

impl ParsedFunction {
    /// Check if this function has any unbindable types
    pub fn has_unbindable_types(&self) -> bool {
        if self.params.iter().any(|p| p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none()) {
            return true;
        }
        if let Some(ref ret) = self.return_type {
            if ret.is_unbindable() {
                return true;
            }
        }
        false
    }
}

/// A parsed C++ enum
#[derive(Debug, Clone)]
pub struct ParsedEnum {
    /// Full enum name (e.g., "TopAbs_ShapeEnum")
```
</details>

Searched for text `struct Param` (`**/crates/opencascade-binding-generator/src/model.rs`), 1 result
- [model.rs:366](../crates/opencascade-binding-generator/src/model.rs#L366)

<details>
<summary>Reading lines 366-390 of <a href="../crates/opencascade-binding-generator/src/model.rs#L366-L391">model.rs</a></summary>

```
pub struct Param {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub ty: Type,
    /// Whether this parameter has a default value in C++
    pub has_default: bool,
    /// The default value as a Rust expression (e.g. "false", "0", "0.0")
    /// Only populated for types we can represent as Rust literals.
    pub default_value: Option<String>,
}

impl Param {
    /// Check if this parameter is a nullable pointer (T* param = NULL or const T* param = NULL).
    /// These are optional parameters that can be bound as Option<&T> / Option<&mut T>.
    pub fn is_nullable_ptr(&self) -> bool {
        if !self.has_default {
            return false;
        }
        match &self.ty {
            // const char* is handled separately (string conversion)
            Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => false,
            Type::ConstPtr(_) | Type::MutPtr(_) => true,
            _ => false,
        }
```
</details>

> 💭 OK so `ParsedFunction.params` is `Vec<Param>`, and `Param.ty` is `Type` (not a `ResolvedType`). 
> 
> Now looking at my check, I wrote:
> ```rust
> matches!(&p.ty.original, Type::ConstRef(_) | Type::MutRef(_))
> ```
> 
> But `p.ty` IS a `Type`, not a struct with an `.original` field. So `p.ty.original` shouldn't compile... unless the `Param` in the `FunctionBinding` context is different.
> 
> Wait, let me look at my actual code again. The `compute_all_function_bindings` iterates over `functions` which are `ParsedFunction`s. 
> 
> Since the binding generator compiled successfully with my code, `p.ty.original` must actually resolve correctly, which means `Type` probably does have an `.original` field or there's some type coercion happening that I'm not immediately seeing.
> 
> Let me check what the actual type of `func` is in that free function loop around line 3268 in bindings.rs to understand what `Param` structure I'm actually working with.

Let me check the actual code I wrote for the free function check:

<details>
<summary>Reading lines 3335-3365 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3335-L3366">bindings.rs</a></summary>

```
            }
        }

        // Ambiguous lifetime check for free functions:
        // If the function returns &mut and has reference params, Rust can't infer
        // which param the return borrows from.
        if let Some(ref ret) = func.return_type {
            if matches!(&ret.original, Type::MutRef(_)) {
                let ref_param_count = func.params.iter().filter(|p| {
                    matches!(&p.ty.original, Type::ConstRef(_) | Type::MutRef(_))
                }).count();
                if ref_param_count >= 2 {
                    skipped.push(SkippedSymbol {
                        kind: "function",
                        module: func.rust_module.clone(),
                        cpp_name: format!("{}::{}", func.namespace, func.short_name),
                        source_header: func.source_header.clone(),
                        source_line: func.source_line,
                        doc_comment: func.doc_comment.clone(),
                        skip_reason: "returns &mut with reference params \u{2014} ambiguous lifetime".to_string(),
                        stub_rust_decl: generate_function_stub(func),
                    });
                    continue;
                }
            }
        }

        let base_rust_name = &func.rust_name;
        let is_overloaded = name_groups.get(base_rust_name).copied().unwrap_or(0) > 1;

        // Compute overload suffix using the same algorithm as class methods
```
</details>

Now let me check - what is  in this context? Let me check the code that sets up the loop:

<details>
<summary>Reading lines 3260-3280 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3260-L3281">bindings.rs</a></summary>

```
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
        if let Some(p) = unbindable_param {
            let reason = if p.ty.original.is_unbindable() {
                format!("param '{}': {}", p.name, describe_unbindable_reason(&p.ty.original))
            } else {
                format!("param '{}' uses unknown type '{}'", p.name, p.ty.original.to_cpp_string())
            };
            skipped.push(SkippedSymbol {
                kind: "function",
```
</details>

> 💭 I see - the `func` here iterates over `all_functions`, which are function binding structs. The `p.ty.original` is used, so `p.ty` must have an `.original` field. So there's a `ResolvedType` or similar wrapper.
> 
> In the method lifetime check in resolver.rs (line 594), `p.ty` is a raw `Type` (from `Param`). But here in `compute_all_function_bindings`, the `func` is a different type - the functions have been resolved/processed, so `p.ty` has an `.original` field.
> 
> My check uses `matches!(&p.ty.original, Type::ConstRef(_) | Type::MutRef(_))` which only catches C++ reference types. But the `&str` parameter comes from `const char*` (a C string pointer), which in the model would be `Type::ConstPtr` with inner `Type::Class("char")` or similar. It's not `ConstRef` or `MutRef`.
>  
> The method-level check in resolver.rs also uses `p.ty.is_c_string()` to catch `const char*` types that map to `&str`. I need to add the same check for `.original.is_c_string()` in my free function validation.

The issue is that my check doesn't account for `const char*` params mapped to `&str`. The method-level check uses `|| p.ty.is_c_string()`. I need to add that:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3343-L3346">bindings.rs -3+3</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "3343:mh,3344:dr,3345:eg",
      "content": "                let ref_param_count = func.params.iter().filter(|p| {\n                    matches!(&p.ty.original, Type::ConstRef(_) | Type::MutRef(_)) || p.ty.original.is_c_string()\n                }).count();"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":3343,"endLine":3345}
```
</details>

Also, the method-level check treats it as ambiguous when there's a MutRef return AND at least 1 reference param (because `&mut self` is the other reference). But for free functions there's no `self`, so the threshold should be 2 — but checking 1 is also needed when there's a `&str` param AND another ref param. Actually, looking at the vrml case: `aComment: &str, anOStream: &mut Standard_OStream` — that's 2 reference-like params, one is a c_string. Let me verify the count would now be right... The `&str` would match `is_c_string()`, and `&mut Standard_OStream` would match `MutRef(_)`. So ref_param_count = 2, and >= 2 triggers the skip. Good.

But wait — what about a function with just ONE ref param and a return `&mut T`? That should be fine (unambiguous). Let me build and test:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1 && time scripts/regenerate-bindings.sh 2>&1 | grep -E "modules generated|error|warning"
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

> 💭 The 3 lifetime errors are fixed!  Now there's a different error: `no method named 'write' found for struct OwnedPtr<opencascade_sys::iges_control::Writer>`. This is in the `opencascade` crate, not `opencascade-sys`. The change in what's bound (e.g., the `write` method may have been renamed due to overload disambiguation since new methods are now included). Let me check what happened.

The lifetime errors are all fixed. The new error is in the `opencascade` crate — method `write` not found on `iges_control::Writer`. New methods being unblocked may have changed overload naming. Let me investigate:

**Terminal:**
```sh
grep -n "pub fn write" crates/opencascade-sys/generated/iges_control.rs
```
```
1883:    pub fn write_ostream_bool(&mut self, S: &mut crate::ffi::Standard_OStream, fnes: bool) -> bool {
1893:    pub fn write_charptr_bool(&mut self, file: &str, fnes: bool) -> bool {
```

> 💭 The `write` method on `iges_control::Writer` was renamed to `write_charptr_bool` and `write_ostream_bool` because there are now two overloads (the ostream one was previously skipped). The `opencascade` crate calls `writer.write(...)` which no longer exists.
> 
> I need to update the call in [crates/opencascade/src/primitives/shape.rs](crates/opencascade/src/primitives/shape.rs#L547) to use `write_charptr_bool`.

<details>
<summary>Reading lines 542-555 of <a href="../crates/opencascade/src/primitives/shape.rs#L542-L556">shape.rs</a></summary>

```
            return Err(Error::IgesWriteFailed);
        }
        writer.compute_model();
        let path_str = path.as_ref().to_string_lossy();
        let fnes = true;
        let success = writer.write(&path_str, fnes);
        if success {
            Ok(())
        } else {
            Err(Error::IgesWriteFailed)
        }
    }

    /// Boolean union: returns a new shape combining `self` and `other`.
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade/src/primitives/shape.rs#L547-L548">shape.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        let success = writer.write_charptr_bool(&path_str, fnes);",
      "filePath": "crates/opencascade/src/primitives/shape.rs",
      "lineHashes": "547:tk"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade/src/primitives/shape.rs","startLine":547,"endLine":547}
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```
<details>
<summary>Output (5 lines)</summary>

```
   Compiling opencascade v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade)
   Compiling examples v0.2.0 (/Users/sethml/src/opencascade-rs/examples)
   Compiling viewer v0.2.0 (/Users/sethml/src/opencascade-rs/crates/viewer)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.64s
real  8.8s
```
</details>

Build succeeds. Now let me update the todo list and proceed with tests and the unit test:

**Todo list:**
- ✅ Config + model changes
- ✅ Manual iostream C++/Rust files
- ✅ Fix lifetime errors in build
- ✅ Fix overload rename breakage
- 🔄 Run binding generator tests
- ⬜ Write iostream unit test
- ⬜ Run full test suite
- ⬜ Commit changes

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (37 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.62s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-3404803fe4cd90de)

running 16 tests
test header_deps::tests::test_is_likely_occt_header ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test module_graph::tests::test_extract_rust_type_name ... ok
test parser::tests::test_extract_module_from_header ... ok
test parser::tests::test_extract_template_arg ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test parser::tests::test_map_standard_type ... ok
test resolver::tests::test_safe_param_name ... ok
test resolver::tests::test_safe_method_name ... ok
test type_mapping::tests::test_map_class ... ok
test type_mapping::tests::test_map_const_ref ... ok
test type_mapping::tests::test_map_return_type ... ok
test type_mapping::tests::test_map_primitives ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/occt_bindgen-eda4fc796a0d21f2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests opencascade_binding_generator

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

real  5.1s
```
</details>

All 16 tests pass. Now let me write the unit test and check existing test patterns:

**Todo list:**
- ✅ Config + model changes
- ✅ Manual iostream C++/Rust files
- ✅ Fix lifetime errors in build
- ✅ Fix overload rename breakage
- ✅ Run binding generator tests
- 🔄 Write iostream unit test
- ⬜ Run full test suite
- ⬜ Commit changes

Searched for regex `#\[cfg\(test\)\]|#\[test\]` (`**/crates/opencascade-sys/src/**`), no results

Searched for regex `#\[cfg\(test\)\]|#\[test\]` (`**/crates/opencascade-sys/**"`), no results

Searched for regex `#\[test\]` (`**/crates/opencascade-sys/**"`), no results

Searched for files matching `**/crates/opencascade-sys/tests/**`, 2 matches
- [pod_structs.rs](../crates/opencascade-sys/tests/pod_structs.rs)
- [golden.rs](../crates/opencascade-sys/tests/golden.rs)

<details>
<summary>Reading lines 1-30 of <a href="../crates/opencascade-sys/tests/pod_structs.rs#L1-L31">pod_structs.rs</a></summary>

```
//! Tests for POD (Plain Old Data) struct bindings.
//!
//! These tests verify that transparent `#[repr(C)]` Rust structs match
//! the C++ layout and can be used to read/write fields directly.

use opencascade_sys::bop_algo::MakePeriodic_PeriodicityParams;

/// Verify that the Rust struct size matches the C++ sizeof for each POD type.
/// This catches layout mismatches from field ordering, alignment, or padding
/// differences between the Rust and C++ definitions.
///
/// PeriodicityParams has: bool[3] + f64[3] + bool[3] + f64[3]
/// = 3*1 + 5(pad) + 3*8 + 3*1 + 5(pad) + 3*8 = 64 bytes
/// (with typical alignment: bools pack to 3 bytes, then 5 padding to align f64)
#[test]
fn periodicity_params_sizeof() {
    let rust_size = std::mem::size_of::<MakePeriodic_PeriodicityParams>();
    // The struct has 4 array fields: bool[3], f64[3], bool[3], f64[3]
    // Minimum size is 3 + 24 + 3 + 24 = 54 bytes, but alignment may add padding
    assert!(
        rust_size >= 54,
        "Rust sizeof ({}) is suspiciously small for PeriodicityParams (expected >= 54)",
        rust_size
    );
}

/// Verify field access works on a default-initialized PeriodicityParams.
#[test]
fn periodicity_params_field_access() {
    let params = MakePeriodic_PeriodicityParams {
```
</details>

Let me check the manual/standard.rs I created to understand what's available:

<details>
<summary>Reading lines 1-60 of <a href="../crates/opencascade-sys/manual/standard.rs#L1-L61">standard.rs</a></summary>

````
// Manual bindings for C++ iostream global objects (std::cout, std::cerr, etc.)
//
// These provide access to C++ standard output/error streams, which can be
// passed to OCCT methods that take Standard_OStream& parameters (e.g., Dump,
// DumpJson, Print).

extern "C" {
    fn iostream_cout() -> *mut crate::ffi::Standard_OStream;
    fn iostream_cerr() -> *mut crate::ffi::Standard_OStream;
    fn iostream_clog() -> *mut crate::ffi::Standard_OStream;
    fn iostream_cin() -> *mut crate::ffi::Standard_IStream;
}

/// Returns a mutable reference to `std::cout` (C++ standard output stream).
///
/// # Safety
///
/// The returned reference points to the global C++ `std::cout` object.
/// The caller must ensure no concurrent writes from multiple threads without
/// synchronization. The reference is valid for the entire program lifetime.
///
/// # Example
///
/// ```no_run
/// let cout = unsafe { opencascade_sys::standard::cout() };
/// // Pass to OCCT methods that take &mut OStream:
/// // some_shape.dump_json(cout, 0);
/// ```
pub unsafe fn cout() -> &'static mut OStream {
    unsafe { &mut *iostream_cout() }
}

/// Returns a mutable reference to `std::cerr` (C++ standard error stream).
///
/// # Safety
///
/// Same safety requirements as [`cout()`].
pub unsafe fn cerr() -> &'static mut OStream {
    unsafe { &mut *iostream_cerr() }
}

/// Returns a mutable reference to `std::clog` (C++ standard log stream).
///
/// # Safety
///
/// Same safety requirements as [`cout()`].
pub unsafe fn clog() -> &'static mut OStream {
    unsafe { &mut *iostream_clog() }
}

/// Returns a mutable reference to `std::cin` (C++ standard input stream).
///
/// # Safety
///
/// Same safety requirements as [`cout()`].
pub unsafe fn cin() -> &'static mut IStream {
    unsafe { &mut *iostream_cin() }
}

````
</details>

**Terminal:**
```sh
grep -n "pub type OStream\|pub type IStream\|pub use" crates/opencascade-sys/manual/standard.rs
```
**Exit code:** 1

> 💭 I need to check if OStream is a type alias or just using the ffi type directly. Let me check the full file. Actually, the file is 59 lines, I already read the whole thing. It references `crate::ffi::Standard_OStream` as the FFI type. But the function returns `OStream` - which needs to be defined or aliased. Let me check if it's defined somewhere.
> 
> Actually, looking at line 29: `pub unsafe fn cout() -> &'static mut OStream {` - this returns `OStream` not `crate::ffi::Standard_OStream`. I need to check if `OStream` is defined somewhere in the file or module. Let me see if it's imported.
> 
> 
> Looking at the file, I don't see `OStream` or `IStream` defined anywhere, which would normally cause a compile error. But since the build succeeded, these types must be coming in through the module structure - probably imported via lib.rs when standard.rs is included in the crate.

Searched for text `standard` (`**/crates/opencascade-sys/src/lib.rs`), no results

<details>
<summary>Searched for text <code>mod</code> (<code>**/crates/opencascade-sys/src/lib.rs</code>), 5 results</summary>

- [lib.rs:8](../crates/opencascade-sys/src/lib.rs#L8)
- [lib.rs:21](../crates/opencascade-sys/src/lib.rs#L21)
- [lib.rs:24](../crates/opencascade-sys/src/lib.rs#L24)
- [lib.rs:26](../crates/opencascade-sys/src/lib.rs#L26)
- [lib.rs:28](../crates/opencascade-sys/src/lib.rs#L28)
</details>

<details>
<summary>Reading lines 1-40 of <a href="../crates/opencascade-sys/src/lib.rs#L1-L41">lib.rs</a></summary>

````
//! OpenCASCADE FFI bindings
//!
//! This crate provides low-level FFI bindings to the OpenCASCADE geometry kernel.
//! The bindings are auto-generated using the `opencascade-binding-generator` crate.
//!
//! # Structure
//!
//! The generated code is organized by OCCT module:
//! - `gp` - Basic geometry types (points, vectors, directions, transforms)
//! - `topo_ds` - Topology data structure (shapes, vertices, edges, faces, etc.)
//! - `collections` - Iterator wrappers for OCCT collection types (ListOfShape, etc.)
//!
//! # Re-generation
//!
//! To regenerate the bindings, run:
//! ```bash
//! ./scripts/regenerate-bindings.sh
//! ```

// Support types for extern "C" FFI
mod owned_ptr;
pub use owned_ptr::*;

// Include the generated modules (including collections)
#[path = "../generated/lib.rs"]
mod generated;

// Re-export all generated modules
pub use generated::*;

````
</details>

> 💭 The `standard` module isn't included in lib.rs — it's in the `manual/` directory. The build succeeded, so either it's not being compiled, or it's included somewhere else. Let me check the generated lib.rs.

**Terminal:**
```sh
grep "standard" crates/opencascade-sys/generated/lib.rs 2>/dev/null; grep "manual" crates/opencascade-sys/generated/lib.rs crates/opencascade-sys/src/lib.rs 2>/dev/null
```
```
pub mod standard;
```
**Exit code:** 1

**Terminal:**
```sh
grep -n "standard" crates/opencascade-sys/generated/lib.rs
```
```
218:pub mod standard;
```

So there's a generated `standard.rs` module. The `manual/standard.rs` I created isn't actually compiled. I need to either merge the manual content into the generated module, or include it separately. Let me check the generated standard.rs:

**Terminal:**
```sh
wc -l crates/opencascade-sys/generated/standard.rs && head -30 crates/opencascade-sys/generated/standard.rs
```
<details>
<summary>Output (31 lines)</summary>

```
   50236 crates/opencascade-sys/generated/standard.rs
//! Standard module re-exports
//!
//! This file was automatically generated by opencascade-binding-generator.
//! Do not edit this file directly.

#![allow(dead_code)]
#![allow(non_snake_case)]

/// **Source:** `Standard.hxx`:94 - `Standard::Purge`
/// Deallocates the storage retained on the free list
/// and clears the list.
/// Returns non-zero if some memory has been actually freed.
pub fn purge() -> i32 {
    unsafe { crate::ffi::Standard_purge() }
}

/// Kind of key in Json string
/// C++ enum: `Standard_JsonKey`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum JsonKey {
    /// < no key
    JsonkeyNone = 0,
    /// < "{"
    JsonkeyOpenchild = 1,
    /// < "}"
    JsonkeyClosechild = 2,
    /// < "["
    JsonkeyOpencontainer = 3,
    /// < "]"
```
</details>

> 💭 OK, so the generated `standard.rs` module exists and is large (50K lines). I should add the manual iostream bindings into this module or into a submodule. The best approach would be to add a manual `pub mod iostream;` to the crate's lib.rs (under the manual directory), since it's separate from the generated standard module.
> 
> Actually, let me think about this differently. The manual/standard_wrappers.cpp is compiled as part of the build (I need to check that), and the manual/standard.rs needs to be included in the crate.
> 
> Let me restructure: instead of having the iostream stuff in `manual/standard.rs`, I'll put it in the lib.rs as a `pub mod iostream` pointing to the manual file. 
> 
> I should verify whether the C++ wrapper file is actually being compiled as part of the build process first.

<details>
<summary>Searched for text <code>manual</code> (<code>**/crates/opencascade-sys/build.rs</code>), 11 results</summary>

- [build.rs:71](../crates/opencascade-sys/build.rs#L71)
- [build.rs:72](../crates/opencascade-sys/build.rs#L72)
- [build.rs:73](../crates/opencascade-sys/build.rs#L73)
- [build.rs:74](../crates/opencascade-sys/build.rs#L74)
- [build.rs:75](../crates/opencascade-sys/build.rs#L75)
- [build.rs:92](../crates/opencascade-sys/build.rs#L92)
- [build.rs:125](../crates/opencascade-sys/build.rs#L125)
- [build.rs:128](../crates/opencascade-sys/build.rs#L128)
- [build.rs:129](../crates/opencascade-sys/build.rs#L129)
</details>

Good — the `crates/opencascade-sys/build.rs` already compiles manual/*.cpp files. Now I need to include the Rust side. Let me add the iostream module to lib.rs:

<details>
<summary>Reading lines 19-30 of <a href="../crates/opencascade-sys/src/lib.rs#L19-L31">lib.rs</a></summary>

```

// Support types for extern "C" FFI
mod owned_ptr;
pub use owned_ptr::*;

// Include the generated modules (including collections)
#[path = "../generated/lib.rs"]
mod generated;

// Re-export all generated modules
pub use generated::*;

```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-sys/src/lib.rs#L28-L34">lib.rs -3+6</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "// Re-export all generated modules\npub use generated::*;\n\n// Manual bindings for iostream global objects\n#[path = \"../manual/standard.rs\"]\npub mod iostream;",
      "filePath": "crates/opencascade-sys/src/lib.rs",
      "lineHashes": "28:sp,29:ps,30:dd"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/src/lib.rs","startLine":28,"endLine":33}
```
</details>

Now I need to make sure `OStream` and `IStream` types are properly defined/aliased in the iostream module. Let me check what the generated FFI types look like:

**Terminal:**
```sh
grep "Standard_OStream\|Standard_IStream\|Standard_SStream" crates/opencascade-sys/generated/ffi.rs | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
pub struct Standard_IStream {
pub struct Standard_OStream {
pub struct Standard_SStream {
        S: *mut Standard_OStream,
        S: *mut Standard_OStream,
        o: *mut Standard_OStream,
        o: *mut Standard_OStream,
        o: *mut Standard_OStream,
        o: *mut Standard_OStream,
        o: *mut Standard_OStream,
        o: *mut Standard_OStream,
        o: *mut Standard_OStream,
    pub fn AppDef_MultiLine_dump(self_: *const AppDef_MultiLine, o: *mut Standard_OStream);
        o: *mut Standard_OStream,
    pub fn AppDef_Variational_dump(self_: *const AppDef_Variational, o: *mut Standard_OStream);
        o: *mut Standard_OStream,
        o: *mut Standard_OStream,
        o: *mut Standard_OStream,
    pub fn Approx_Curve3d_dump(self_: *const Approx_Curve3d, o: *mut Standard_OStream);
        o: *mut Standard_OStream,
```
</details>

The FFI has `pub struct Standard_OStream`, `Standard_IStream`, and `Standard_SStream` as opaque types. Now let me update the manual/standard.rs to properly define `OStream` and `IStream` type aliases:

<details>
<summary>Reading lines 1-59 of <a href="../crates/opencascade-sys/manual/standard.rs#L1-L60">standard.rs</a></summary>

````
// Manual bindings for C++ iostream global objects (std::cout, std::cerr, etc.)
//
// These provide access to C++ standard output/error streams, which can be
// passed to OCCT methods that take Standard_OStream& parameters (e.g., Dump,
// DumpJson, Print).

extern "C" {
    fn iostream_cout() -> *mut crate::ffi::Standard_OStream;
    fn iostream_cerr() -> *mut crate::ffi::Standard_OStream;
    fn iostream_clog() -> *mut crate::ffi::Standard_OStream;
    fn iostream_cin() -> *mut crate::ffi::Standard_IStream;
}

/// Returns a mutable reference to `std::cout` (C++ standard output stream).
///
/// # Safety
///
/// The returned reference points to the global C++ `std::cout` object.
/// The caller must ensure no concurrent writes from multiple threads without
/// synchronization. The reference is valid for the entire program lifetime.
///
/// # Example
///
/// ```no_run
/// let cout = unsafe { opencascade_sys::standard::cout() };
/// // Pass to OCCT methods that take &mut OStream:
/// // some_shape.dump_json(cout, 0);
/// ```
pub unsafe fn cout() -> &'static mut OStream {
    unsafe { &mut *iostream_cout() }
}

/// Returns a mutable reference to `std::cerr` (C++ standard error stream).
///
/// # Safety
///
/// Same safety requirements as [`cout()`].
pub unsafe fn cerr() -> &'static mut OStream {
    unsafe { &mut *iostream_cerr() }
}

/// Returns a mutable reference to `std::clog` (C++ standard log stream).
///
/// # Safety
///
/// Same safety requirements as [`cout()`].
pub unsafe fn clog() -> &'static mut OStream {
    unsafe { &mut *iostream_clog() }
}

/// Returns a mutable reference to `std::cin` (C++ standard input stream).
///
/// # Safety
///
/// Same safety requirements as [`cout()`].
pub unsafe fn cin() -> &'static mut IStream {
    unsafe { &mut *iostream_cin() }
}

````
</details>

I need to add type aliases for `OStream` and `IStream`, and also check the C++ function names match. Let me also check the C++ file:

<details>
<summary>Reading lines 1-15 of <a href="../crates/opencascade-sys/manual/standard_wrappers.cpp#L1-L16">standard_wrappers.cpp</a></summary>

```
// Manual bindings for C++ iostream global objects.
//
// Provides extern "C" accessors for std::cout, std::cerr, std::clog,
// and std::cin as raw pointers to Standard_OStream / Standard_IStream.
// These are global singletons — the returned pointers are valid for
// the entire program lifetime.

#include <Standard_OStream.hxx>
#include <Standard_IStream.hxx>
#include <iostream>

extern "C" Standard_OStream* iostream_cout() {
    return &std::cout;
}

```
</details>

**Terminal:**
```sh
cat crates/opencascade-sys/manual/standard_wrappers.cpp
```
<details>
<summary>Output (26 lines)</summary>

```
// Manual bindings for C++ iostream global objects.
//
// Provides extern "C" accessors for std::cout, std::cerr, std::clog,
// and std::cin as raw pointers to Standard_OStream / Standard_IStream.
// These are global singletons — the returned pointers are valid for
// the entire program lifetime.

#include <Standard_OStream.hxx>
#include <Standard_IStream.hxx>
#include <iostream>

extern "C" Standard_OStream* iostream_cout() {
    return &std::cout;
}

extern "C" Standard_OStream* iostream_cerr() {
    return &std::cerr;
}

extern "C" Standard_OStream* iostream_clog() {
    return &std::clog;
}

extern "C" Standard_IStream* iostream_cin() {
    return &std::cin;
}
```
</details>

Good, the C++ names match (iostream_cout, iostream_cerr, etc.). Now let me add the type aliases to the Rust file:

<details>
<summary>Edited: <a href="../crates/opencascade-sys/manual/standard.rs#L1-L14">standard.rs -7+13</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "// Manual bindings for C++ iostream global objects (std::cout, std::cerr, etc.)\n//\n// These provide access to C++ standard output/error streams, which can be\n// passed to OCCT methods that take Standard_OStream& parameters (e.g., Dump,\n// DumpJson, Print).\n\n/// Opaque type for C++ `std::ostream` (aliased as `Standard_OStream` in OCCT).\npub type OStream = crate::ffi::Standard_OStream;\n\n/// Opaque type for C++ `std::istream` (aliased as `Standard_IStream` in OCCT).\npub type IStream = crate::ffi::Standard_IStream;\n\nextern \"C\" {",
      "filePath": "crates/opencascade-sys/manual/standard.rs",
      "lineHashes": "1:yg,2:ni,3:ln,4:ev,5:re,6:dd,7:bo"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/manual/standard.rs","startLine":1,"endLine":13}
```
</details>

Also update the doc example to use the iostream module name:

<details>
<summary>Reading lines 25-40 of <a href="../crates/opencascade-sys/manual/standard.rs#L25-L41">standard.rs</a></summary>

````
/// The caller must ensure no concurrent writes from multiple threads without
/// synchronization. The reference is valid for the entire program lifetime.
///
/// # Example
///
/// ```no_run
/// let cout = unsafe { opencascade_sys::standard::cout() };
/// // Pass to OCCT methods that take &mut OStream:
/// // some_shape.dump_json(cout, 0);
/// ```
pub unsafe fn cout() -> &'static mut OStream {
    unsafe { &mut *iostream_cout() }
}

/// Returns a mutable reference to `std::cerr` (C++ standard error stream).
///
````
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-sys/manual/standard.rs#L30-L35">standard.rs -5+5</a></summary>

**Input:**
````
{
  "edits": [
    {
      "content": "/// ```no_run\n/// let cout = unsafe { opencascade_sys::iostream::cout() };\n/// // Pass to OCCT methods that take &mut OStream:\n/// // some_shape.dump_json(cout, 0);\n/// ```",
      "filePath": "crates/opencascade-sys/manual/standard.rs",
      "lineHashes": "30:fq,31:pe,32:eu,33:eu,34:qf"
    }
  ]
}
````
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/manual/standard.rs","startLine":30,"endLine":34}
```
</details>

Now let me build to see if the iostream module works:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```
<details>
<summary>Output (40 lines) — exit code 101</summary>

```
   Compiling opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)
error[E0255]: the name `OStream` is defined multiple times
     --> crates/opencascade-sys/src/../generated/../manual/standard.rs:8:1
      |
    8 | pub type OStream = crate::ffi::Standard_OStream;
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `OStream` redefined here
      |
     ::: crates/opencascade-sys/src/../generated/standard.rs:50225:5
      |
50225 |     Standard_OStream as OStream, Standard_PCharacter as PCharacter,
      |     --------------------------- previous import of the type `OStream` here
      |
      = note: `OStream` must be defined only once in the type namespace of this module
help: you can use `as` to change the binding name of the import
     -->   crates/opencascade-sys/src/../generated/standard.rs:50225:26
      |
50225 |     Standard_OStream as OtherOStream, Standard_PCharacter as PCharacter,
      |                          +++++

error[E0255]: the name `IStream` is defined multiple times
     --> crates/opencascade-sys/src/../generated/../manual/standard.rs:11:1
      |
   11 | pub type IStream = crate::ffi::Standard_IStream;
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `IStream` redefined here
      |
     ::: crates/opencascade-sys/src/../generated/standard.rs:50224:28
      |
50224 | ..._Byte as Byte, Standard_IStream as IStream, Standard_JmpBuf as Jmp...
      |                   --------------------------- previous import of the type `IStream` here
      |
      = note: `IStream` must be defined only once in the type namespace of this module
help: you can use `as` to change the binding name of the import
     -->   crates/opencascade-sys/src/../generated/standard.rs:50224:48
      |
50224 |     Standard_Byte as Byte, Standard_IStream as OtherIStream, Standard_JmpBuf as JmpBuf,
      |                                                +++++

For more information about this error, try `rustc --explain E0255`.
error: could not compile `opencascade-sys` (lib) due to 2 previous errors
real  15.8s
```
</details>

> 💭 The generated `standard.rs` already re-exports `Standard_OStream as OStream` and `Standard_IStream as IStream`. Since the iostream module is separate from the standard module, I don't need to re-export them. But the issue is that lib.rs does `pub use generated::*;` which brings the generated `standard` module into scope, and then the `pub mod iostream;` also brings things into scope. But the error says these names conflict at different locations...
> 
> Wait, looking more carefully: the error shows the conflict is between definitions IN the manual/standard.rs file and imports from generated/standard.rs. But manual/standard.rs is the `iostream` module, not the `standard` module. 
> 
> Oh, I see it now - the `#[path]` attribute makes manual/standard.rs into the `iostream` module, so the type aliases defined there are scoped to `iostream`. They shouldn't conflict with the `standard` module unless... actually, looking at the error location, it's pointing to the same file being defined multiple times, which suggests the module is being included or re-exported in a way that creates a duplicate definition.
> 
> The paths in the error are collapsing to the same file through different import paths - the generated/standard.rs is importing `OStream` at line 50225, but then the manual/standard.rs is also defining it at line 8, and somehow both are ending up in the same namespace. I think the issue might be that the generated lib.rs is pulling in both the generated standard module and the manual one through the `#[path]` attribute, causing the same type to be defined twice in the same scope. I'm realizing the path `crates/opencascade-sys/src/../generated/../manual/standard.rs` actually resolves to `crates/opencascade-sys/manual/standard.rs`, which means both the iostream module and the generated standard module might be getting re-exported in a way that creates a naming conflict at the root level. Let me trace through the re-exports in the generated standard.rs file to see what's actually being exposed. I'm realizing the paths in the error message are relative, so both `OStream` definitions appear to be in the same module scope from Rust's perspective. Let me trace through the actual module structure in `lib.rs` to understand how these are being resolved. I'm realizing the real issue might be that the generated lib.rs itself is doing glob re-exports, which could be creating the naming conflict at the crate root level.

**Terminal:**
```sh
tail -20 crates/opencascade-sys/generated/lib.rs
```
<details>
<summary>Output (20 lines)</summary>

```
pub mod top_trans;
pub mod topo_ds;
pub mod topo_ds_to_step;
pub mod transfer;
pub mod transfer_b_rep;
pub mod units;
pub mod units_api;
pub mod units_methods;
pub mod vrml;
pub mod vrml_api;
pub mod vrml_converter;
pub mod vrml_data;
pub mod xcaf_app;
pub mod xcaf_dim_tol_objects;
pub mod xcaf_doc;
pub mod xcaf_note_objects;
pub mod xcaf_prs;
pub mod xcaf_view;
pub mod xs_algo;
pub mod xs_control;
```
</details>

**Terminal:**
```sh
tail -20 crates/opencascade-sys/generated/standard.rs
```
<details>
<summary>Output (20 lines)</summary>

```
}

// ========================
// Additional type re-exports
// ========================

pub use crate::ffi::{
    Standard_Byte as Byte, Standard_IStream as IStream, Standard_JmpBuf as JmpBuf,
    Standard_OStream as OStream, Standard_PCharacter as PCharacter,
    Standard_PExtCharacter as PExtCharacter, Standard_SStream as SStream,
    Standard_ThreadId as ThreadId, Standard_Utf16Char as Utf16Char,
};

// Manual bindings:
// Manual bindings for C++ iostream global objects (std::cout, std::cerr, etc.)
//
// These provide access to C++ standard output/error streams, which can be
// passed to OCCT methods that take Standard_OStream& parameters (e.g., Dump,
// DumpJson, Print).
include!("../manual/standard.rs");
```
</details>

> 💭 The generated `standard.rs` already includes `manual/standard.rs` via `include!()` at the bottom, AND it already re-exports `Standard_OStream as OStream` and `Standard_IStream as IStream`. So:
> 
> 1. The generated standard module already has OStream/IStream
> 2. My manual/standard.rs also defines `pub type OStream = ...;` and `pub type IStream = ...;`
> 3. The `include!` brings the manual content into the generated standard module, causing duplicate definitions
> 
> So I need to:
> 1. Remove the type aliases from manual/standard.rs (they're already in the generated standard.rs re-exports)
> 
> 
> 2. Remove the `pub mod iostream` from lib.rs since the binding generator already handles including manual files through its own mechanism

The generated `standard.rs` already `include!`s the manual file and re-exports the type aliases! I don't need to add the module to lib.rs. Let me remove my additions:

<details>
<summary>Reading lines 28-33 of <a href="../crates/opencascade-sys/src/lib.rs#L28-L34">lib.rs</a></summary>

```
// Re-export all generated modules
pub use generated::*;

// Manual bindings for iostream global objects
#[path = "../manual/standard.rs"]
pub mod iostream;
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-sys/src/lib.rs#L28-L30">lib.rs -6+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "// Re-export all generated modules\npub use generated::*;",
      "filePath": "crates/opencascade-sys/src/lib.rs",
      "lineHashes": "28:sp,29:ps,30:dd,31:xg,32:kr,33:xf"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/src/lib.rs","startLine":28,"endLine":29}
```
</details>

Now remove the type aliases from manual/standard.rs since the generated module already provides them:

<details>
<summary>Reading lines 1-20 of <a href="../crates/opencascade-sys/manual/standard.rs#L1-L21">standard.rs</a></summary>

```
// Manual bindings for C++ iostream global objects (std::cout, std::cerr, etc.)
//
// These provide access to C++ standard output/error streams, which can be
// passed to OCCT methods that take Standard_OStream& parameters (e.g., Dump,
// DumpJson, Print).

/// Opaque type for C++ `std::ostream` (aliased as `Standard_OStream` in OCCT).
pub type OStream = crate::ffi::Standard_OStream;

/// Opaque type for C++ `std::istream` (aliased as `Standard_IStream` in OCCT).
pub type IStream = crate::ffi::Standard_IStream;

extern "C" {
    fn iostream_cout() -> *mut crate::ffi::Standard_OStream;
    fn iostream_cerr() -> *mut crate::ffi::Standard_OStream;
    fn iostream_clog() -> *mut crate::ffi::Standard_OStream;
    fn iostream_cin() -> *mut crate::ffi::Standard_IStream;
}

/// Returns a mutable reference to `std::cout` (C++ standard output stream).
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-sys/manual/standard.rs#L1-L11">standard.rs -13+10</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "// Manual bindings for C++ iostream global objects (std::cout, std::cerr, etc.)\n//\n// These provide access to C++ standard output/error streams, which can be\n// passed to OCCT methods that take Standard_OStream& parameters (e.g., Dump,\n// DumpJson, Print).\n//\n// This file is include!()'d into the generated standard.rs module, which\n// already provides OStream/IStream type aliases via its re-exports section.\n\nextern \"C\" {",
      "filePath": "crates/opencascade-sys/manual/standard.rs",
      "lineHashes": "1:yg,2:ni,3:ln,4:ev,5:re,6:dd,7:gv,8:gi,9:dd,10:mh,11:kw,12:dd,13:bo"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/manual/standard.rs","startLine":1,"endLine":10}
```
</details>

Also update the doc example to use the correct path:

<details>
<summary>Reading lines 15-50 of <a href="../crates/opencascade-sys/manual/standard.rs#L15-L51">standard.rs</a></summary>

````
}

/// Returns a mutable reference to `std::cout` (C++ standard output stream).
///
/// # Safety
///
/// The returned reference points to the global C++ `std::cout` object.
/// The caller must ensure no concurrent writes from multiple threads without
/// synchronization. The reference is valid for the entire program lifetime.
///
/// # Example
///
/// ```no_run
/// let cout = unsafe { opencascade_sys::iostream::cout() };
/// // Pass to OCCT methods that take &mut OStream:
/// // some_shape.dump_json(cout, 0);
/// ```
pub unsafe fn cout() -> &'static mut OStream {
    unsafe { &mut *iostream_cout() }
}

/// Returns a mutable reference to `std::cerr` (C++ standard error stream).
///
/// # Safety
///
/// Same safety requirements as [`cout()`].
pub unsafe fn cerr() -> &'static mut OStream {
    unsafe { &mut *iostream_cerr() }
}

/// Returns a mutable reference to `std::clog` (C++ standard log stream).
///
/// # Safety
///
/// Same safety requirements as [`cout()`].
pub unsafe fn clog() -> &'static mut OStream {
````
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-sys/manual/standard.rs#L28-L29">standard.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "/// let cout = unsafe { opencascade_sys::standard::cout() };",
      "filePath": "crates/opencascade-sys/manual/standard.rs",
      "lineHashes": "28:wc"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/manual/standard.rs","startLine":28,"endLine":28}
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

Build succeeds. Now let me write the unit test. Let me find some OCCT methods that take `&mut Standard_OStream` to use in the test:

**Terminal:**
```sh
grep -n "fn dump\b.*Standard_OStream" crates/opencascade-sys/generated/ffi.rs | head -10
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
grep -n "fn.*_dump.*Standard_OStream" crates/opencascade-sys/generated/ffi.rs | head -10
```
<details>
<summary>Output (10 lines)</summary>

```
38021:    pub fn AppDef_MultiLine_dump(self_: *const AppDef_MultiLine, o: *mut Standard_OStream);
39730:    pub fn AppDef_Variational_dump(self_: *const AppDef_Variational, o: *mut Standard_OStream);
40719:    pub fn Approx_Curve3d_dump(self_: *const Approx_Curve3d, o: *mut Standard_OStream);
48285:    pub fn BOPAlgo_Options_dump_errors(self_: *const BOPAlgo_Options, theOS: *mut Standard_OStream);
61750:    pub fn BRepBlend_AppSurface_dump(self_: *const BRepBlend_AppSurface, o: *mut Standard_OStream);
86904:    pub fn BRepTools_History_dump(self_: *mut BRepTools_History, theS: *mut Standard_OStream);
119189:    pub fn FairCurve_Batten_dump(self_: *const FairCurve_Batten, o: *mut Standard_OStream);
159708:    pub fn GeomTools_Curve2dSet_dump(self_: *const GeomTools_Curve2dSet, OS: *mut Standard_OStream);
159749:    pub fn GeomTools_CurveSet_dump(self_: *const GeomTools_CurveSet, OS: *mut Standard_OStream);
159792:    pub fn GeomTools_SurfaceSet_dump(self_: *const GeomTools_SurfaceSet, OS: *mut Standard_OStream);
```
</details>

Let me find dump methods on simple types that are easy to construct. `gp_Pnt` would be ideal:

**Terminal:**
```sh
grep -n "fn.*gp_Pnt.*Standard_OStream\|fn.*Standard_OStream.*gp_Pnt" crates/opencascade-sys/generated/ffi.rs | head -10
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
grep -rn "fn dump\|fn dump_json" crates/opencascade-sys/generated/gp.rs | head -20
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
grep -rn "OStream" crates/opencascade-sys/generated/gp.rs | head -10
```
```
crates/opencascade-sys/generated/gp.rs:12054:    pub fn print(&self, theStream: &mut crate::ffi::Standard_OStream) {
```

Let me see what type has the `print` method:

<details>
<summary>Reading lines 12045-12068 of <a href="../crates/opencascade-sys/generated/gp.rs#L12045-L12069">gp.rs</a></summary>

```
    ) -> crate::OwnedPtr<crate::ffi::HandlegpVectorWithNullMagnitude> {
        unsafe {
            crate::OwnedPtr::from_raw(crate::ffi::gp_VectorWithNullMagnitude_to_handle(
                obj.into_raw(),
            ))
        }
    }

    /// Inherited: **Source:** `Standard_Failure.hxx`:58 - `Standard_Failure::Print()`
    pub fn print(&self, theStream: &mut crate::ffi::Standard_OStream) {
        unsafe {
            crate::ffi::gp_VectorWithNullMagnitude_inherited_Print(self as *const Self, theStream)
        }
    }

    /// Inherited: **Source:** `Standard_Failure.hxx`:72 - `Standard_Failure::Reraise()`
    pub fn reraise(&mut self) {
        unsafe { crate::ffi::gp_VectorWithNullMagnitude_inherited_Reraise(self as *mut Self) }
    }

    /// Inherited: **Source:** `Standard_Failure.hxx`:112 - `Standard_Failure::Jump()`
    pub fn jump(&mut self) {
        unsafe { crate::ffi::gp_VectorWithNullMagnitude_inherited_Jump(self as *mut Self) }
    }
```
</details>

That's an exception type, not ideal. Let me look for dump or DumpJson on standard types:

**Terminal:**
```sh
grep -rn "OStream" crates/opencascade-sys/generated/b_rep_tools.rs | head -10
```
<details>
<summary>Output (10 lines)</summary>

```
crates/opencascade-sys/generated/b_rep_tools.rs:294:pub fn dump_shape_ostream(Sh: &crate::topo_ds::Shape, S: &mut crate::ffi::Standard_OStream) {
crates/opencascade-sys/generated/b_rep_tools.rs:305:    theStream: &mut crate::ffi::Standard_OStream,
crates/opencascade-sys/generated/b_rep_tools.rs:326:    theStream: &mut crate::ffi::Standard_OStream,
crates/opencascade-sys/generated/b_rep_tools.rs:1533:    pub fn dump(&mut self, theS: &mut crate::ffi::Standard_OStream) {
crates/opencascade-sys/generated/b_rep_tools.rs:3331:    pub fn dump_geometry_ostream(&self, OS: &mut crate::ffi::Standard_OStream) {
crates/opencascade-sys/generated/b_rep_tools.rs:3340:        OS: &mut crate::ffi::Standard_OStream,
crates/opencascade-sys/generated/b_rep_tools.rs:3373:        OS: &mut crate::ffi::Standard_OStream,
crates/opencascade-sys/generated/b_rep_tools.rs:3386:        OS: &mut crate::ffi::Standard_OStream,
crates/opencascade-sys/generated/b_rep_tools.rs:3444:        OS: &mut crate::ffi::Standard_OStream,
crates/opencascade-sys/generated/b_rep_tools.rs:3461:    pub fn dump_polygon3_d(&self, OS: &mut crate::ffi::Standard_OStream) {
```
</details>

`dump_shape_ostream` looks good — takes a `Shape` and `OStream`. Let me also look for a simple BOPAlgo_Options dump:

<details>
<summary>Reading lines 290-300 of <a href="../crates/opencascade-sys/generated/b_rep_tools.rs#L290-L301">b_rep_tools.rs</a></summary>

```
}
/// **Source:** `BRepTools.hxx`:278 - `BRepTools::Dump`
/// Dumps the topological structure and the geometry
/// of <Sh> on the stream <S>.
pub fn dump_shape_ostream(Sh: &crate::topo_ds::Shape, S: &mut crate::ffi::Standard_OStream) {
    unsafe { crate::ffi::BRepTools_dump_shape_ostream(Sh, S) }
}
/// **Source:** `BRepTools.hxx`:285 - `BRepTools::Write`
/// Writes the shape to the stream in an ASCII format TopTools_FormatVersion_VERSION_1.
/// This alias writes shape with triangulation data.
/// @param[in] theShape        the shape to write
```
</details>

Now let me write the test. I'll create a test that uses `BRepPrimAPI_MakeBox` to create a shape, then dumps it to cout:

**Terminal:**
```sh
grep -rn "make_box\|MakeBox" crates/opencascade-sys/generated/ | grep "pub fn new\|pub fn.*make_box" | head -5
```
```
crates/opencascade-sys/generated/b_rep_preview_api.rs:43:    pub fn as_b_rep_prim_api_make_box(&self) -> &crate::b_rep_prim_api::MakeBox {
crates/opencascade-sys/generated/b_rep_preview_api.rs:50:    pub fn as_b_rep_prim_api_make_box_mut(&mut self) -> &mut crate::b_rep_prim_api::MakeBox {
```

**Terminal:**
```sh
grep -rn "pub fn new" crates/opencascade-sys/generated/b_rep_prim_api.rs | head -10
```
<details>
<summary>Output (10 lines)</summary>

```
crates/opencascade-sys/generated/b_rep_prim_api.rs:43:    pub fn new() -> crate::OwnedPtr<Self> {
crates/opencascade-sys/generated/b_rep_prim_api.rs:49:    pub fn new_real3(dx: f64, dy: f64, dz: f64) -> crate::OwnedPtr<Self> {
crates/opencascade-sys/generated/b_rep_prim_api.rs:55:    pub fn new_pnt_real3(P: &crate::gp::Pnt, dx: f64, dy: f64, dz: f64) -> crate::OwnedPtr<Self> {
crates/opencascade-sys/generated/b_rep_prim_api.rs:63:    pub fn new_pnt2(P1: &crate::gp::Pnt, P2: &crate::gp::Pnt) -> crate::OwnedPtr<Self> {
crates/opencascade-sys/generated/b_rep_prim_api.rs:69:    pub fn new_ax2_real3(
crates/opencascade-sys/generated/b_rep_prim_api.rs:271:    pub fn new_real3(R1: f64, R2: f64, H: f64) -> crate::OwnedPtr<Self> {
crates/opencascade-sys/generated/b_rep_prim_api.rs:281:    pub fn new_real4(R1: f64, R2: f64, H: f64, angle: f64) -> crate::OwnedPtr<Self> {
crates/opencascade-sys/generated/b_rep_prim_api.rs:293:    pub fn new_ax2_real3(Axes: &crate::gp::Ax2, R1: f64, R2: f64, H: f64) -> crate::OwnedPtr<Self> {
crates/opencascade-sys/generated/b_rep_prim_api.rs:333:    pub fn new_ax2_real4(
crates/opencascade-sys/generated/b_rep_prim_api.rs:484:    pub fn new_real2(R: f64, H: f64) -> crate::OwnedPtr<Self> {
```
</details>

Let me check how `shape()` is accessed from MakeBox:

**Terminal:**
```sh
grep -n "fn shape\|fn as_shape\|fn as_topo_ds\|Built\|into_shape" crates/opencascade-sys/generated/b_rep_prim_api.rs | head -10
```
<details>
<summary>Output (10 lines)</summary>

```
217:    pub fn shape(&mut self) -> &crate::topo_ds::Shape {
424:    pub fn shape(&mut self) -> &crate::topo_ds::Shape {
631:    pub fn shape(&mut self) -> &crate::topo_ds::Shape {
770:    pub fn shape(&mut self) -> &crate::topo_ds::Shape {
879:    pub fn shape(&mut self) -> &crate::topo_ds::Shape {
1178:    pub fn shape(&mut self) -> &crate::topo_ds::Shape {
1432:    pub fn shape(&mut self) -> &crate::topo_ds::Shape {
1684:    pub fn shape(&mut self) -> &crate::topo_ds::Shape {
1995:    pub fn shape(&mut self) -> &crate::topo_ds::Shape {
2122:    pub fn shape(&mut self) -> &crate::topo_ds::Shape {
```
</details>

Now I can write the test. Let me check what the existing test file structure looks like:

<details>
<summary>Reading lines 1-10 of <a href="../crates/opencascade-sys/tests/pod_structs.rs#L1-L11">pod_structs.rs</a></summary>

```
//! Tests for POD (Plain Old Data) struct bindings.
//!
//! These tests verify that transparent `#[repr(C)]` Rust structs match
//! the C++ layout and can be used to read/write fields directly.

use opencascade_sys::bop_algo::MakePeriodic_PeriodicityParams;

/// Verify that the Rust struct size matches the C++ sizeof for each POD type.
/// This catches layout mismatches from field ordering, alignment, or padding
/// differences between the Rust and C++ definitions.
```
</details>

Created [iostream.rs](../crates/opencascade-sys/tests/iostream.rs)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo test -p opencascade-sys --test iostream 2>&1
```

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_01RcMWsqE6ptppgMmxW7tLc2__vscode-1771312785596/content.txt#1-1), lines 1 to 60

Both tests pass and the dump output shows the box shape topology. Let me now run the full test suite:

**Todo list:**
- ✅ Config + model changes
- ✅ Manual iostream C++/Rust files
- ✅ Fix lifetime errors in build
- ✅ Fix overload rename breakage
- ✅ Run binding generator tests
- ✅ Write iostream unit test
- 🔄 Run full test suite
- ⬜ Commit changes

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo test --workspace --exclude opencascade-binding-generator 2>&1
```

**Terminal:**
```sh
grep -E "test result|FAILED|error" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/6b6b3972-bc9a-42cc-b50c-a28fdd05000e/toolu_016JBaWGhR5pteBGGpVCLhTf__vscode-1771312785599/content.txt
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

All tests pass — 0 failures. Let me also run the binding generator tests:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (36 lines)</summary>

```
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-3404803fe4cd90de)

running 16 tests
test header_deps::tests::test_is_likely_occt_header ... ok
test parser::tests::test_extract_module_from_header ... ok
test module_graph::tests::test_extract_rust_type_name ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test parser::tests::test_extract_template_arg ... ok
test parser::tests::test_map_standard_type ... ok
test type_mapping::tests::test_map_class ... ok
test type_mapping::tests::test_map_const_ref ... ok
test type_mapping::tests::test_map_primitives ... ok
test type_mapping::tests::test_map_return_type ... ok
test resolver::tests::test_safe_param_name ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test resolver::tests::test_safe_method_name ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/occt_bindgen-eda4fc796a0d21f2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests opencascade_binding_generator

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

real  0.9s
```
</details>

All passing. Now let me commit:

**Todo list:**
- ✅ Config + model changes
- ✅ Manual iostream C++/Rust files
- ✅ Fix lifetime errors in build
- ✅ Fix overload rename breakage
- ✅ Run binding generator tests
- ✅ Write iostream unit test
- ✅ Run full test suite
- 🔄 Commit changes

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff --stat
```
<details>
<summary>Output (104 lines)</summary>

```
 .../src/codegen/bindings.rs                        |   28 +
 .../src/codegen/rust.rs                            |    3 +-
 crates/opencascade-binding-generator/src/config.rs |   14 +
 crates/opencascade-binding-generator/src/main.rs   |   19 +-
 crates/opencascade-binding-generator/src/model.rs  |    2 +-
 .../opencascade-binding-generator/src/resolver.rs  |    3 +
 crates/opencascade-sys/bindings.toml               |    9 +
 crates/opencascade-sys/generated/adv_app2_var.rs   |   15 +-
 crates/opencascade-sys/generated/adv_approx.rs     |   26 +-
 .../generated/api_header_section.rs                |   18 +
 crates/opencascade-sys/generated/app_def.rs        |   89 +-
 crates/opencascade-sys/generated/app_par_curves.rs |   51 +-
 crates/opencascade-sys/generated/approx.rs         |   39 +-
 crates/opencascade-sys/generated/b_rep_algo_api.rs |   10 +
 crates/opencascade-sys/generated/b_rep_approx.rs   |   21 +
 crates/opencascade-sys/generated/b_rep_blend.rs    |   13 +-
 .../opencascade-sys/generated/b_rep_builder_api.rs |   18 +-
 crates/opencascade-sys/generated/b_rep_check.rs    |    7 +
 crates/opencascade-sys/generated/b_rep_extrema.rs  |   38 +-
 crates/opencascade-sys/generated/b_rep_feat.rs     |   39 +
 crates/opencascade-sys/generated/b_rep_mesh.rs     |   32 +-
 crates/opencascade-sys/generated/b_rep_tools.rs    |  415 +-
 crates/opencascade-sys/generated/bop_algo.rs       |  250 +-
 crates/opencascade-sys/generated/expr.rs           |  122 +-
 crates/opencascade-sys/generated/expr_intrp.rs     |   20 +-
 crates/opencascade-sys/generated/fair_curve.rs     |   39 +-
 crates/opencascade-sys/generated/ffi.rs            | 4703 +++++++++++++++++++-
 crates/opencascade-sys/generated/g_prop.rs         |   20 +-
 crates/opencascade-sys/generated/gcc_ana.rs        |   20 +-
 crates/opencascade-sys/generated/gcc_ent.rs        |   28 +-
 crates/opencascade-sys/generated/geom.rs           |   42 +-
 crates/opencascade-sys/generated/geom2d.rs         |   42 +-
 crates/opencascade-sys/generated/geom2d_convert.rs |   13 +-
 crates/opencascade-sys/generated/geom2d_gcc.rs     |   20 +-
 crates/opencascade-sys/generated/geom_convert.rs   |   26 +-
 crates/opencascade-sys/generated/geom_int.rs       |   21 +
 crates/opencascade-sys/generated/geom_tools.rs     |  399 +-
 crates/opencascade-sys/generated/gp.rs             |   22 +-
 crates/opencascade-sys/generated/if_select.rs      |  423 +-
 crates/opencascade-sys/generated/iges_
... (truncated, 7940 chars total)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff --stat --cached
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git status --short | grep "^?" | head -10
```
```
?? crates/opencascade-sys/manual/standard.rs
?? crates/opencascade-sys/manual/standard_wrappers.cpp
?? crates/opencascade-sys/tests/iostream.rs
```

Created [commit-msg.txt](../tmp/commit-msg.txt)

**Terminal:**
```sh
rm -f tmp/commit-msg.txt
```
```
(base) whitelily:opencascade-rs sethml$
```

Created [commit-msg.txt](../tmp/commit-msg.txt)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git add -A && git commit -F tmp/commit-msg.txt 2>&1
```
<details>
<summary>Output (5 lines)</summary>

```
[main fc45983] Support iostream/ostream types in generated bindings
 106 files changed, 16958 insertions(+), 3937 deletions(-)
 create mode 100644 crates/opencascade-sys/manual/standard.rs
 create mode 100644 crates/opencascade-sys/manual/standard_wrappers.cpp
 create mode 100644 crates/opencascade-sys/tests/iostream.rs
```
</details>

**Todo list:**
- ✅ Config + model changes
- ✅ Manual iostream C++/Rust files
- ✅ Fix lifetime errors in build
- ✅ Fix overload rename breakage
- ✅ Run binding generator tests
- ✅ Write iostream unit test
- ✅ Run full test suite
- ✅ Commit changes

Committed as `fc45983`. Summary:

**What was done:**
- Removed  from `is_unbindable()` so `Standard_OStream`/`Standard_IStream`/`Standard_SStream` parameters no longer block methods
- Added `manual_types` config in `crates/opencascade-sys/bindings.toml` so stream types are recognized as known types
- Threaded manual type names through resolver →  → `compute_all_function_bindings`
- Added ambiguous lifetime check for free functions (matching the existing class method check) to handle free functions returning `&mut` with multiple reference params
- Created manual/standard_wrappers.cpp with `extern "C"` accessors for , 
- Created manual/standard.rs with `cout()`, `cerr()`, `clog()`, `cin()` functions (included into generated standard module)
- Fixed `write` 
- Created manual/standard.rs with `cout()`, `cerr()`, `clog()`, `cin()` functions (included into generated standard module)
- Fixed `write` → `write_charptr_bool` rename breakage in `crates/opencascade/src/primitives/shape.rs`
- Added iostream test that dumps a box shape to stdout

**Result:** ~486 methods unblocked (1519 → ~1036 skipped symbols). All tests pass.

*2026-02-19 20:24 · 7957s · Words: 17,127 in · 15,748 out · 67,982 ctx · 204 rounds*

---

<a id="user-4"></a>

## User (4)

> I modified the stream accessors to not be unsafe. Update the README to reflect your and my stream changes, then amend commit.

*2026-02-19 21:24*

### Assistant — Claude Opus 4.6

> 💭 Let me first check the current state of the modified files, then update the README and amend the commit.

<details>
<summary>Reading all lines of <a href="../crates/opencascade-sys/manual/standard.rs">standard.rs</a></summary>

````
// Manual bindings for C++ iostream global objects (std::cout, std::cerr, etc.)
//
// These provide access to C++ standard output/error streams, which can be
// passed to OCCT methods that take Standard_OStream& parameters (e.g., Dump,
// DumpJson, Print).
//
// This file is include!()'d into the generated standard.rs module, which
// already provides OStream/IStream type aliases via its re-exports section.

extern "C" {
    fn iostream_cout() -> *mut crate::ffi::Standard_OStream;
    fn iostream_cerr() -> *mut crate::ffi::Standard_OStream;
    fn iostream_clog() -> *mut crate::ffi::Standard_OStream;
    fn iostream_cin() -> *mut crate::ffi::Standard_IStream;
}

/// Returns a mutable reference to `std::cout` (C++ standard output stream).
///
/// # Safety
///
/// The returned reference points to the global C++ `std::cout` object.
/// Concurrent writes from multiple threads are safe but may result in
/// interleaved output. The reference is valid for the entire program lifetime.
///
/// # Example
///
/// ```no_run
/// let cout = opencascade_sys::standard::cout();
/// // Pass to OCCT methods that take &mut OStream:
/// // some_shape.dump_json(cout, 0);
/// ```
pub fn cout() -> &'static mut OStream {
    unsafe { &mut *iostream_cout() }
}

/// Returns a mutable reference to `std::cerr` (C++ standard error stream).
///
/// # Safety
///
/// Same safety requirements as [`cout()`].
pub fn cerr() -> &'static mut OStream {
    unsafe { &mut *iostream_cerr() }
}

/// Returns a mutable reference to `std::clog` (C++ standard log stream).
///
/// # Safety
///
/// Same safety requirements as [`cout()`].
pub fn clog() -> &'static mut OStream {
    unsafe { &mut *iostream_clog() }
}

/// Returns a mutable reference to `std::cin` (C++ standard input stream).
///
/// # Safety
///
/// Same safety requirements as [`cout()`].
pub fn cin() -> &'static mut IStream {
    unsafe { &mut *iostream_cin() }
}

````
</details>

<details>
<summary>Reading all lines of <a href="../crates/opencascade-sys/tests/iostream.rs">iostream.rs</a></summary>

```
//! Tests for iostream (Standard_OStream / Standard_IStream) bindings.
//!
//! These tests verify that the manual iostream bindings work correctly
//! and that OCCT methods accepting Standard_OStream& can be called with
//! the provided cout/cerr accessors.

/// Verify that cout() returns a valid reference and can be passed to
/// an OCCT method that takes &mut Standard_OStream.
#[test]
fn dump_shape_to_cout() {
    // Create a simple box shape
    let mut make_box = opencascade_sys::b_rep_prim_api::MakeBox::new_real3(10.0, 20.0, 30.0);
    let shape = make_box.shape();

    // Get a reference to std::cout
    let cout = opencascade_sys::standard::cout();

    // Dump the shape's topological structure to stdout.
    // This exercises the full path: manual C++ wrapper -> extern "C" -> Rust FFI.
    opencascade_sys::b_rep_tools::dump_shape_ostream(shape, cout);
}

/// Verify that cerr() also works.
#[test]
fn cerr_accessor_works() {
    let cerr = opencascade_sys::standard::cerr();
    // Just verify we can obtain the reference without crashing.
    // The type is Standard_OStream (opaque), so there's not much to assert,
    // but the fact that the extern "C" call resolved and returned a non-null
    // pointer (converted to a reference) proves the C++ wrapper links correctly.
    let _ptr: *const opencascade_sys::standard::OStream = cerr;
}

```
</details>

<details>
<summary>Reading lines 340-410 of <a href="../crates/opencascade-binding-generator/README.md#L340-L411">README.md</a></summary>

````
See `crates/opencascade-sys/manual/` and the comments in `bindings.toml` for the two existing examples (`Transfer_Finder::GetStringAttribute` and `BRepFill_AdvancedEvolved::SetTemporaryDirectory`).

---

## Skipped Symbols

The binding generator skips ~1,519 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Every skipped symbol is documented in the generated per-module `.rs` files as a `// SKIPPED:` comment block including:

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
| 546 | 35.9% | **Stream type** | C++ `std::istream`/`std::ostream` (`Standard_IStream`/`Standard_OStream`) — no Rust equivalent |
| 432 | 28.4% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`Standard_SStream`, `Handle(TDocStd_Document)`, etc.) |
| 199 | 13.1% | **Void pointer** | `Standard_Address` (typedef for `void*`) — cannot be safely expressed in Rust FFI |
| 135 | 8.9% | **Raw pointer** | `T*`/`const T*` returns (static/free fn) or primitive pointer params (`int*`, `double*`) — class pointer returns on instance methods are now bound as `Option<&T>`/`Option<&mut T>` |
| 53 | 3.5% | **Ambiguous lifetimes** | `&mut` return with reference params — Rust lifetime inference is ambiguous |
| 49 | 3.2% | **Unresolved template type** | Template instantiations that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `LDOMBasicString`, etc.) |
| 43 | 2.8% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |
| 18 | 1.2% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |
| 17 | 1.1% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |
| 12 | 0.8% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across FFI |
| 7 | 0.5% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(IGESData_IGESModel)`, `Handle(CDM_MetaData)`, etc.) |
| 5 | 0.3% | **Not CppDeletable** | Return type class has no destructor in the binding set |
| 4 | 0.3% | **&mut enum return** | Mutable reference to enum (cxx limitation) |
| 2 | 0.1% | **Excluded by bindings.toml** | Explicitly excluded in config (e.g., ambiguous overload workarounds) |
| 1 | 0.1% | **Ambiguous overload** | C++ overload that would produce identical wrapper signatures |

### Most Common Unknown Types

The "unknown type" and "unknown Handle type" categories (29% of all skips) are dominated by a few types:

| Count | Type | How to Unblock |
|------:|------|----------------|
| 87 | `Handle(TDocStd_Document)` | Add `TDocStd_Document` — needed for document framework access |
| 86 | `Standard_SStream` | Map `Standard_SStream` (`std::stringstream`) — mainly used in `Raise()` methods on exception classes |
| 19 | `XCAFPrs_Style` | Add `XCAFPrs_Style` — used in XCAF presentation styles |
| 12 | `IMeshData::IEdgeHandle` | Nested handle typedef in meshing internals — low priority |
| 11 | `ShapeProcess::OperationsFlags` | Nested type in ShapeProcess — used in STEP/IGES processing flags |
| 10 | `IMeshData::IFaceHandle` | Nested handle typedef in meshing internals — low priority |

Previously common unknown types that have been resolved:
- ~~`math_Vector`~~ (was 665 skips) — resolved by fixing typedef collection filter that excluded lowercase-prefix OCCT types (`math_*`)
- ~~`Standard_Charac
... (truncated, 4993 chars)
````
</details>

Good — the accessors are now safe (no `unsafe`). Let me get the current skip counts, then update the README.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -c "^// SKIPPED:" crates/opencascade-sys/generated/*.rs | awk -F: '{s+=$2} END {print s}'
```
```
1036
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | sort | uniq -c | sort -rn
```
<details>
<summary>Output (374 lines)</summary>

```
 173 returns &mut with reference params (ambiguous lifetimes)
  46 param 'theDocument' uses unknown type 'const Handle(TDocStd_Document)&'
  43 class is abstract (has unimplemented pure virtual methods)
  41 has unbindable types: param 'S': void pointer (Standard_Address)
  31 has unbindable types: param 'C': void pointer (Standard_Address)
  27 param 'theDoc' uses unknown type 'const Handle(TDocStd_Document)&'
  22 has unbindable types: param 'PCurve': void pointer (const Standard_Address&)
  14 return type 'void*' is unknown
  14 param 'theStyle' uses unknown type 'const XCAFPrs_Style&'
  13 has unbindable types: return: void pointer (Standard_Address)
  11 has unbindable types: param 'C': void pointer (const Standard_Address&)
  10 param 'doc' uses unknown type 'const Handle(TDocStd_Document)&'
  10 has unbindable types: param 'C1': void pointer (const Standard_Address&); param 'C2': void pointer (const Standard_Address&)
   9 has string ref param 'name' of type 'const char*&' (needs manual binding)
   8 param 'theFlags' uses unknown type 'const ShapeProcess::OperationsFlags&'
   8 has unbindable types: param 'theParameters': rvalue reference (XSAlgo_ShapeProcessor::ParameterMap&&)
   8 has unbindable types: param 'Surface': void pointer (const Standard_Address&)
   7 param 'theIStream' uses unknown type 'std::istream&'
   7 has unbindable types: return: raw pointer (const uint16_t*)
   6 return type 'Handle(IMeshData::ListOfPnt2d)' is unknown
   6 return type 'const IMeshData::MapOfInteger&' is unknown
   6 param 'theDFace' uses unknown type 'const IMeshData::IFaceHandle&'
   6 param 'theDEdge' uses unknown type 'const IMeshData::IEdgeHandle&'
   6 has unbindable types: param 'str': void pointer (Standard_Address)
   6 has unbindable types: param 'A': void pointer (Standard_Address)
   5 return type 'const XCAFPrs_Style&' is unknown
   5 param 'theWriter' uses unknown type 'RWGltf_GltfOStreamWriter*'
   5 param 'theDoc' uses unknown type 'Handle(TDocStd_Document)&'
   5 has unbindable types: param 'theStream': stream type (const std::shared_ptr<std::ostream>&)
   5 has unbindable types: param 's': void pointer (Standard_Address)
   5 has unbindable types: param 'S': void pointer (const Standard_Address&)
   4 return type is &mut enum (not representable in extern "C")
   4 return type 'Handle(TDocStd_Document)' is unknown
   4 return type 'Handle(IMeshData::MapOfInteger)' is unknown
   4 return type 'char' is not CppDeletable
   4 param 'tostr' uses unknown type 'Standard_PCharacter&'
   4 param 'theStream' uses unknown type 'std::istream&'
   4 param 'theEdge' uses unknown type 'const IMeshData::IEdgeHandle&'
   4 param 'theDoc' uses unknown Handle type
   4 has unbindable types: return: unresolved template type (std::shared_ptr<std::streambuf>)
   4 has unbindable types: return: unresolved template type (const std::pair<Standard_Real, Standard_Real>&)
   4 has unbindable types: param 'theStream': stre
... (truncated, 43685 chars total)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | sed 's/: .*//' | sed "s/param '.*' uses/param uses/" | sed "s/param '.*':/param:/" | sed 's/return: .*/return type/' | sort | uniq -c | sort -rn | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
 425 has unbindable types
 173 returns &mut with reference params (ambiguous lifetimes)
  89 param uses unknown type 'const Handle(TDocStd_Document)&'
  43 class is abstract (has unimplemented pure virtual methods)
  14 return type 'void*' is unknown
  14 param uses unknown type 'const XCAFPrs_Style&'
  12 param uses unknown type 'std::istream&'
  10 param uses unknown type 'void*'
  10 param uses unknown type 'const IMeshData::IEdgeHandle&'
   9 param uses unknown type 'Handle(TDocStd_Document)&'
   9 param uses unknown type 'const ShapeProcess::OperationsFlags&'
   9 has string ref param 'name' of type 'const char*&' (needs manual binding)
   7 param uses unknown type 'Standard_PCharacter&'
   7 param uses unknown type 'const IMeshData::IFaceHandle&'
   7 param uses unknown Handle type
   6 return type 'Handle(IMeshData::ListOfPnt2d)' is unknown
   6 return type 'const IMeshData::MapOfInteger&' is unknown
   5 return type 'const XCAFPrs_Style&' is unknown
   5 param uses unknown type 'RWGltf_GltfOStreamWriter*'
   4 return type is &mut enum (not representable in extern "C")
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "unbindable.*void"
```
```
200
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "unbindable.*raw pointer"
```
```
135
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "unbindable.*array"
```
```
18
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "unbindable.*rvalue"
```
```
12
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "unbindable.*stream"
```
```
19
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep "stream"
```
<details>
<summary>Output (36 lines)</summary>

```
param 'theIStream' uses unknown type 'std::istream&'
param 'theIStream' uses unknown type 'std::istream&'
param 'anInput' uses unknown type 'std::istream&'
param 'n' uses unknown type 'std::streamsize'
has unbindable types: param 'theStream': stream type (const std::shared_ptr<std::istream>&)
has unbindable types: param 'theStream': stream type (const std::shared_ptr<std::ostream>&)
has unbindable types: param 'theOldStream': stream type (const std::shared_ptr<std::istream>&); return: stream type (std::shared_ptr<std::istream>)
has unbindable types: return: stream type (std::shared_ptr<std::ostream>)
has unbindable types: return: unresolved template type (std::shared_ptr<std::streambuf>)
has unbindable types: param 'theStream': stream type (const std::shared_ptr<std::istream>&)
has unbindable types: param 'theStream': stream type (const std::shared_ptr<std::ostream>&)
has unbindable types: param 'theOldStream': stream type (const std::shared_ptr<std::istream>&); return: stream type (std::shared_ptr<std::istream>)
has unbindable types: return: stream type (std::shared_ptr<std::ostream>)
has unbindable types: return: unresolved template type (std::shared_ptr<std::streambuf>)
has unbindable types: param 'theStream': stream type (const std::shared_ptr<std::istream>&)
has unbindable types: param 'theStream': stream type (const std::shared_ptr<std::ostream>&)
has unbindable types: param 'theOldStream': stream type (const std::shared_ptr<std::istream>&); return: stream type (std::shared_ptr<std::istream>)
has unbindable types: return: stream type (std::shared_ptr<std::ostream>)
has unbindable types: return: unresolved template type (std::shared_ptr<std::streambuf>)
has unbindable types: param 'theStream': stream type (const std::shared_ptr<std::istream>&)
has unbindable types: param 'theStream': stream type (const std::shared_ptr<std::ostream>&)
has unbindable types: return: unresolved template type (std::shared_ptr<std::streambuf>)
param 'theBinFile' uses unknown type 'std::ostream&'
param 'theStream' uses unknown type 'std::istream&'
param 'theStream' uses unknown type 'std::istream&'
param 'theStream' uses unknown type 'std::istream&'
param 'theStream' uses unknown type 'std::istream&'
has unbindable types: param 'theStream': stream type (const std::shared_ptr<std::ostream>&)
param 'theUntilPos' uses unknown type 'std::streampos'
param 'theIStream' uses unknown type 'std::istream&'
param 'theIStream' uses unknown type 'std::istream&'
param 'theOStream' uses unknown type 'std::ostream&'
param 'theIStream' uses unknown type 'std::istream&'
param 'theIStream' uses unknown type 'std::istream&'
param 'theStream' uses unknown type 'std::ostream&'
param 'theIStream' uses unknown type 'std::istream&'
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "unbindable.*unresolved template"
```
```
49
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "unknown type\|unknown Handle"
```
```
257
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "string ref param"
```
```
17
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "Not CppDeletable\|not CppDeletable"
```
```
5
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "excluded\|Excluded"
```
```
2
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "ambiguous overload\|Ambiguous overload"
```
```
1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "&mut enum"
```
```
4
```

Now let me get the remaining unknown type counts for the "Most Common Unknown Types" section.  is now an "unknown type" instead of "stream type" — let me understand the breakdown:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep "unknown type\|unknown Handle" | sed "s/.*unknown type '//" | sed "s/.*unknown Handle type '//" | sed "s/'.*//" | sort | uniq -c | sort -rn | head -15
```
<details>
<summary>Output (15 lines)</summary>

```
  89 const Handle(TDocStd_Document)&
  14 const XCAFPrs_Style&
  12 std::istream&
  10 void*
  10 const IMeshData::IEdgeHandle&
   9 Handle(TDocStd_Document)&
   9 const ShapeProcess::OperationsFlags&
   7 Standard_PCharacter&
   7 param 
   7 const IMeshData::IFaceHandle&
   5 RWGltf_GltfOStreamWriter*
   4 Standard_Byte
   3 std::ostream&
   3 IMeshData::VectorOfInteger&
   3 IMeshData::Array1OfVertexOfDelaun&
```
</details>

Now I have all the numbers. Let me also check what `param` is about and the shared_ptr<stream> count:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep "unknown Handle type" | head -10
```
<details>
<summary>Output (7 lines)</summary>

```
param 'theSet1' uses unknown Handle type
param 'theSet1' uses unknown Handle type
param 'D' uses unknown Handle type
param 'theDoc' uses unknown Handle type
param 'theDoc' uses unknown Handle type
param 'theDoc' uses unknown Handle type
param 'theDoc' uses unknown Handle type
```
</details>

Now let me compute remaining unbindable stream counts (shared_ptr<stream>): 19 unbindable stream + 4 shared_ptr<streambuf> = ~23 remaining from the old "stream" category. The rest moved to "unknown type" (12 std::istream& + 3 std::ostream& = 15) and "ambiguous lifetimes" (about 120 more from the old stream methods). Let me now compute the percentages and update the README.

Total: 1036. Let me verify the sum:
- Unknown/unresolved type: 257 (includes unknown Handle 7)
- Void pointer: 200
- Ambiguous lifetimes: 173
- Raw pointer: 135
- Unresolved template: 49
- Abstract class: 43
- Stream (shared_ptr): 19
- C-style array: 18
- String ref param: 17
- Rvalue reference: 12
- Not CppDeletable: 5
- &mut enum return: 4
- Excluded by config: 2
- Ambiguous overload: 1

Sum: 257 + 200 + 173 + 135 + 49 + 43 + 19 + 18 + 17 + 12 + 5 + 4 + 2 + 1 = 935... That's less than 1036. Let me check what I'm missing.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep "unbindable" | grep -v "void\|raw pointer\|array\|rvalue\|unresolved template\|stream" | head -20
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && echo "=== Verify totals ===" && echo -n "unbindable void: " && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "unbindable.*void" && echo -n "unbindable raw pointer: " && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "unbindable.*raw pointer" && echo -n "unbindable array: " && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "unbindable.*array" && echo -n "unbindable rvalue: " && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "unbindable.*rvalue" && echo -n "unbindable unresolved template: " && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "unbindable.*unresolved template" && echo -n "unbindable stream: " && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "unbindable.*stream" && echo -n "unbindable total: " && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep -c "unbindable" && echo -n "all reasons: " && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | wc -l
```
<details>
<summary>Output (9 lines)</summary>

```
=== Verify totals ===
unbindable void: 200
unbindable raw pointer: 135
unbindable array: 18
unbindable rvalue: 12
unbindable unresolved template: 49
unbindable stream: 19
unbindable total: 425
all reasons:     1044
```
</details>

Hmm, 425 unbindable total, but 200 + 135 + 18 + 12 + 49 + 19 = 433. Some overlap? Let me check: the "unbindable" search also matches "void" items from unknown type context. Let me count differently — non-unbindable categories:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | grep "unbindable" | sed 's/.*unbindable types: //' | sed 's/;.*//' | sed 's/param.*: //' | sed 's/return: //' | sort | uniq -c | sort -rn | head -10
```

OK, the "has unbindable types" count (425) is the total. Some have multiple reasons in one entry (e.g., both param and return). Let me just tabulate the reason kinds properly. The total SKIPPED entries is 1036, but there are 1044 "Reason:" lines because some multi-reason entries have the reason on a separate line. Let me just get the category counts by SKIPPED block.  

Actually, the previous approach of counting the SKIPPED blocks by first-line reason pattern is fine for the README. Let me consolidate: the point is to update the table. Let me compute a clean breakdown:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | awk '
/unbindable.*void/ {void++; next}
/unbindable.*raw pointer/ {raw++; next}
/unbindable.*array/ {array++; next}
/unbindable.*rvalue/ {rvalue++; next}
/unbindable.*unresolved template/ {template++; next}
/unbindable.*stream/ {stream++; next}
/returns &mut with reference params/ {lifetime++; next}
/unknown Handle type/ {unknownhandle++; next}
/unknown type/ {unknown++; next}
/abstract/ {abstract_++; next}
/string ref param/ {stringref++; next}
/Not CppDeletable|not CppDeletable/ {nodestruct++; next}
/&mut enum/ {mutenum++; next}
/excluded|Excluded/ {excluded++; next}
/ambiguous overload|Ambiguous overload/ {ambiguousol++; next}
{other++; print "OTHER: " $0}
END {
  total = void+raw+array+rvalue+template+stream+lifetime+unknownhandle+unknown+abstract_+stringref+nodestruct+mutenum+excluded+ambiguousol+other
  printf "Void pointer: %d (%.1f%%)\n", void, void/total*100
  printf "Unknown type: %d (%.1f%%)\n", unknown, unknown/total*100
  printf "Ambiguous lifetimes: %d (%.1f%%)\n", lifetime, lifetime/total*100
  printf "Raw pointer: %d (%.1f%%)\n", raw, raw/total*100
  printf "Unresolved template: %d (%.1f%%)\n", template, template/total*100
  printf "Abstract class: %d (%.1f%%)\n", abstract_, abstract_/total*100
  printf "Stream (shared_ptr): %d (%.1f%%)\n", stream, stream/total*100
  printf "C-style array: %d (%.1f%%)\n", array, array/total*100
  printf "String ref param: %d (%.1f%%)\n", stringref, stringref/total*100
  printf "Rvalue reference: %d (%.1f%%)\n", rvalue, rvalue/total*100
  printf "Unknown Handle type: %d (%.1f%%)\n", unknownhandle, unknownhandle/total*100
  printf "Not CppDeletable: %d (%.1f%%)\n", nodestruct, nodestruct/total*100
  printf "&mut enum return: %d (%.1f%%)\n", mutenum, mutenum/total*100
  printf "Excluded by config: %d (%.1f%%)\n", excluded, excluded/total*100
  printf "Ambiguous overload: %d (%.1f%%)\n", ambiguousol, ambiguousol/total*100
  printf "Other: %d\n", other
  printf "Total: %d\n", total
}'
```
<details>
<summary>Output (6 lines) — exit code 2</summary>

```
awk: syntax error at source line 18
 context is
printf "Unknown >>>  type: <<<   printf "Unknown type:  printf "Unknown type:  printf "Unknown type:  printf "Unknown type:  printf "nt  printf "Unknown type:  printf ra  printf "Unknown type:  printf "Unkn te  printf "Unknown type:  printf "Utemplate/total*100
awk: illegal statement at source line 18
awk: illegal statement at source line 18
2 missing )'s
```
</details>

Quoting issue. Let me write this as a script instead:

Created [count-skips.awk](../tmp/count-skips.awk)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | awk -f tmp/count-skips.awk
```
<details>
<summary>Output (136 lines)</summary>

```
OTHER: return type 'const BOPAlgo_PPaveFiller&' is unknown
OTHER: return type 'const BOPAlgo_PBuilder&' is unknown
OTHER: return type 'const Handle(BRepExtrema_TriangleSet)&' is unknown
OTHER: return type 'const Handle(BRepExtrema_TriangleSet)&' is unknown
OTHER: return type 'const Handle(BRepExtrema_TriangleSet)&' is unknown
OTHER: return type 'const ProxPnt_Status&' is unknown
OTHER: return type 'const ProxPnt_Status&' is unknown
OTHER: return type 'Handle(IMeshData::ListOfPnt2d)' is unknown
OTHER: return type 'Handle(IMeshData::ListOfPnt2d)' is unknown
OTHER: return type 'const IMeshData::ListOfInteger&' is unknown
OTHER: return type 'const IMeshData::MapOfInteger&' is unknown
OTHER: return type 'const IMeshData::MapOfInteger&' is unknown
OTHER: return type 'Handle(IMeshData::ListOfPnt2d)' is unknown
OTHER: return type 'const IMeshData::IFaceHandle&' is unknown
OTHER: return type 'Handle(IMeshData::MapOfInteger)' is unknown
OTHER: return type 'Handle(IMeshData::MapOfInteger)' is unknown
OTHER: return type 'Handle(IMeshData::MapOfInteger)' is unknown
OTHER: return type 'const Handle(IMeshData::MapOfIEdgePtr)&' is unknown
OTHER: return type 'Handle(IMeshData::MapOfInteger)' is unknown
OTHER: return type 'Handle(IMeshData::ListOfPnt2d)' is unknown
OTHER: return type 'const IMeshData::MapOfInteger&' is unknown
OTHER: return type 'const IMeshData::MapOfInteger&' is unknown
OTHER: return type 'const IMeshData::MapOfInteger&' is unknown
OTHER: return type 'const IMeshData::MapOfInteger&' is unknown
OTHER: return type 'Handle(IMeshData::ListOfPnt2d)' is unknown
OTHER: return type 'Handle(IMeshData::ListOfPnt2d)' is unknown
OTHER: return type 'const IMeshData::IMapOfReal&' is unknown
OTHER: return type 'IMeshData::IMapOfReal&' is unknown
OTHER: return type 'const IMeshData::IMapOfReal&' is unknown
OTHER: return type 'IMeshData::IMapOfReal&' is unknown
OTHER: return type 'const Handle(IMeshData::VectorOfVertex)&' is unknown
OTHER: return type 'Handle(IMeshData::VectorOfVertex)&' is unknown
OTHER: return type 'const IMeshData::ListOfInteger&' is unknown
OTHER: return type 'BOPAlgo_PPaveFiller' is unknown
OTHER: return type 'BOPDS_PDS' is unknown
OTHER: return type 'BOPDS_PDS' is unknown
OTHER: return type 'const BOPDS_PIterator&' is unknown
OTHER: return type 'const Handle(BVH_Properties)&' is unknown
OTHER: *const Standard_SStream,
OTHER: *const Standard_SStream);
OTHER: return type 'const IMeshData::IPCurveHandle&' is unknown
OTHER: return type 'const IMeshData::ICurveHandle&' is unknown
OTHER: return type 'const IMeshData::IWireHandle&' is unknown
OTHER: return type 'const IMeshData::IWireHandle&' is unknown
OTHER: return type 'const IMeshData::IFaceHandle&' is unknown
OTHER: return type 'const IMeshData::IFaceHandle&' is unknown
OTHER: return type 'const IMeshData::IEdgeHandle&' is unknown
OTHER: return type 'const IMeshData::IEdgeHandle&' is unknown
OTHER: return type 'const IMeshData_Face *const&' is
... (truncated, 7341 chars total)
```
</details>

The "OTHER" cases include return type unknowns, Standard_SStream references, not CppDeletable, etc. These are miscategorized because my awk patterns don't catch them. The total is 1044 "Reason:" lines (some SKIPPED blocks have multiple), vs 1036 SKIPPED blocks. Let me adjust — the "OTHER" lines that say "return type ... is unknown" should be counted as unknown type, and some are clearly not-CppDeletable, etc. 

Let me just update the awk script to be more thorough:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && echo "=== SKIPPED blocks ===" && grep -c "^// SKIPPED:" crates/opencascade-sys/generated/*.rs | awk -F: '{s+=$2} END {print "Total SKIPPED blocks: " s}'
```
```
=== SKIPPED blocks ===
Total SKIPPED blocks: 1036
```

The counts don't need to be perfectly exact for the README — close enough. Let me compute the table based on SKIPPED blocks (1036 total) and the categories. Looking at the "OTHER" output, many are "return type ... is unknown" which my script missed because they're on a different line. Let me just use approximate counts that are close:

Based on the analysis:
- Unknown/unresolved type: ~250 (param) + ~75 (return type unknowns from OTHER) = ~325, about 31.4%
- Void pointer: ~200, 19.3%  
- Ambiguous lifetimes: 176, 17.0% (up from 53 since free functions now checked too, and many stream methods were previously skipped as "stream" but now contribute here)
- Raw pointer: 134, 12.9%
- Unresolved template: 48, 4.6%
- Abstract class: 43, 4.1%
- C-style array: 16, 1.5%
- Stream (shared_ptr): 15, 1.4%
- String ref param: 17, 1.6%
- Rvalue reference: 12, 1.2%
- Unknown Handle type: 7, 0.7%
- Not CppDeletable: 5, 0.5%
- &mut enum return: 4, 0.4%
- Excluded by config: 2, 0.2%
- Ambiguous overload: 1, 0.1%

Wait, let me just look at what the block count per category is using a more careful extraction from the actual SKIPPED block's Reason line:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -A1 "^// SKIPPED:" crates/opencascade-sys/generated/*.rs | grep "Reason:" | sed 's/.*Reason: //' | awk -f tmp/count-skips.awk 2>/dev/null | grep -v "^OTHER"
```
<details>
<summary>Output (17 lines)</summary>

```
| 89 | 23.7% | Void pointer |
| 27 | 7.2% | Unknown type |
| 126 | 33.5% | Ambiguous lifetimes |
| 83 | 22.1% | Raw pointer |
| 10 | 2.7% | Unresolved template |
| 8 | 2.1% | Abstract class |
| 0 | 0.0% | Stream (shared_ptr) |
| 0 | 0.0% | C-style array |
| 9 | 2.4% | String ref param |
| 0 | 0.0% | Rvalue reference |
| 1 | 0.3% | Unknown Handle type |
| 0 | 0.0% | Not CppDeletable |
| 3 | 0.8% | &mut enum return |
| 2 | 0.5% | Excluded by config |
| 0 | 0.0% | Ambiguous overload |
Other: 18
Total: 376
```
</details>

The `-A1` approach only gets the *second* line, not the actual "Reason:" line within the block. The skip blocks are multi-line. Let me try a different approach — extract complete reason lines from SKIPPED blocks:

Using "Create File"

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && python3 tmp/count-skips.py
```
<details>
<summary>Output (19 lines)</summary>

```
Total: 1036

| Count | % | Category | Description |
|------:|----:|----------|-------------|
| 356 | 34.4% | **Unknown/unresolved type** |
| 200 | 19.3% | **Void pointer** |
| 176 | 17.0% | **Ambiguous lifetimes** |
| 134 | 12.9% | **Raw pointer** |
| 48 | 4.6% | **Unresolved template** |
| 43 | 4.2% | **Abstract class** |
| 17 | 1.6% | **String ref param** |
| 16 | 1.5% | **C-style array** |
| 15 | 1.4% | **Stream (shared_ptr)** |
| 12 | 1.2% | **Rvalue reference** |
| 7 | 0.7% | **Unknown Handle type** |
| 5 | 0.5% | **Not CppDeletable** |
| 4 | 0.4% | **&mut enum return** |
| 2 | 0.2% | **Excluded by config** |
| 1 | 0.1% | **Ambiguous overload** |
```
</details>

Total 1036 with clean categories. Now let me also check the "Most Common Unknown Types" and important skipped symbols sections. Let me get updated unknown type breakdown:

Created [count-unknown-types.py](../tmp/count-unknown-types.py)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && python3 tmp/count-unknown-types.py
```
<details>
<summary>Output (17 lines)</summary>

```
| Count | Type | How to Unblock |
|------:|------|----------------|
| 90 | `const Handle(TDocStd_Document)&` |
| 24 | `void*` |
| 19 | `const XCAFPrs_Style&` |
| 12 | `const IMeshData::IEdgeHandle&` |
| 12 | `std::istream&` |
| 11 | `const ShapeProcess::OperationsFlags&` |
| 10 | `const IMeshData::IFaceHandle&` |
| 9 | `Handle(TDocStd_Document)&` |
| 7 | `const IMeshData::MapOfInteger&` |
| 7 | `Standard_PCharacter&` |
| 6 | `const Handle(BRepExtrema_TriangleSet)&` |
| 6 | `Handle(IMeshData::ListOfPnt2d)` |
| 6 | `Standard_Byte` |
| 5 | `RWGltf_GltfOStreamWriter*` |
| 4 | `Handle(IMeshData::MapOfInteger)` |
```
</details>

Now let me also get the data exchange / document framework / meshing skip counts for the important skipped symbols section:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && echo "=== Data Exchange ===" && grep -c "^// SKIPPED:" crates/opencascade-sys/generated/step_control.rs crates/opencascade-sys/generated/iges_control.rs crates/opencascade-sys/generated/xs_control.rs crates/opencascade-sys/generated/rw_stl.rs crates/opencascade-sys/generated/igescaf_control.rs crates/opencascade-sys/generated/stepcaf_control.rs 2>/dev/null | awk -F: '{s+=$2; print} END {print "Total: " s}' && echo "=== Document Framework ===" && grep -c "^// SKIPPED:" crates/opencascade-sys/generated/t_doc_std.rs crates/opencascade-sys/generated/tdf.rs crates/opencascade-sys/generated/xcaf_doc.rs crates/opencascade-sys/generated/xcaf_prs.rs 2>/dev/null | awk -F: '{s+=$2; print} END {print "Total: " s}' && echo "=== Meshing ===" && grep -c "^// SKIPPED:" crates/opencascade-sys/generated/b_rep_mesh.rs crates/opencascade-sys/generated/i_mesh_data.rs crates/opencascade-sys/generated/i_mesh_tools.rs 2>/dev/null | awk -F: '{s+=$2; print} END {print "Total: " s}'
```
<details>
<summary>Output (19 lines)</summary>

```
=== Data Exchange ===
crates/opencascade-sys/generated/step_control.rs:5
crates/opencascade-sys/generated/iges_control.rs:3
crates/opencascade-sys/generated/xs_control.rs:22
crates/opencascade-sys/generated/rw_stl.rs:2
crates/opencascade-sys/generated/igescaf_control.rs:6
crates/opencascade-sys/generated/stepcaf_control.rs:19
Total: 57
=== Document Framework ===
crates/opencascade-sys/generated/t_doc_std.rs:36
crates/opencascade-sys/generated/tdf.rs:9
crates/opencascade-sys/generated/xcaf_doc.rs:29
crates/opencascade-sys/generated/xcaf_prs.rs:0
Total: 74
=== Meshing ===
crates/opencascade-sys/generated/b_rep_mesh.rs:76
crates/opencascade-sys/generated/i_mesh_data.rs:14
crates/opencascade-sys/generated/i_mesh_tools.rs:1
Total: 91
```
</details>

Now I have all the data. Let me read the full sections I need to update:

<details>
<summary>Reading lines 344-480 of <a href="../crates/opencascade-binding-generator/README.md#L344-L481">README.md</a></summary>

````
## Skipped Symbols

The binding generator skips ~1,519 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Every skipped symbol is documented in the generated per-module `.rs` files as a `// SKIPPED:` comment block including:

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
| 546 | 35.9% | **Stream type** | C++ `std::istream`/`std::ostream` (`Standard_IStream`/`Standard_OStream`) — no Rust equivalent |
| 432 | 28.4% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`Standard_SStream`, `Handle(TDocStd_Document)`, etc.) |
| 199 | 13.1% | **Void pointer** | `Standard_Address` (typedef for `void*`) — cannot be safely expressed in Rust FFI |
| 135 | 8.9% | **Raw pointer** | `T*`/`const T*` returns (static/free fn) or primitive pointer params (`int*`, `double*`) — class pointer returns on instance methods are now bound as `Option<&T>`/`Option<&mut T>` |
| 53 | 3.5% | **Ambiguous lifetimes** | `&mut` return with reference params — Rust lifetime inference is ambiguous |
| 49 | 3.2% | **Unresolved template type** | Template instantiations that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `LDOMBasicString`, etc.) |
| 43 | 2.8% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |
| 18 | 1.2% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |
| 17 | 1.1% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |
| 12 | 0.8% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across FFI |
| 7 | 0.5% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(IGESData_IGESModel)`, `Handle(CDM_MetaData)`, etc.) |
| 5 | 0.3% | **Not CppDeletable** | Return type class has no destructor in the binding set |
| 4 | 0.3% | **&mut enum return** | Mutable reference to enum (cxx limitation) |
| 2 | 0.1% | **Excluded by bindings.toml** | Explicitly excluded in config (e.g., ambiguous overload workarounds) |
| 1 | 0.1% | **Ambiguous overload** | C++ overload that would produce identical wrapper signatures |

### Most Common Unknown Types

The "unknown type" and "unknown Handle type" categories (29% of all skips) are dominated by a few types:

| Count | Type | How to Unblock |
|------:|------|----------------|
| 87 | `Handle(TDocStd_Document)` | Add `TDocStd_Document` — needed for document framework access |
| 86 | `Standard_SStream` | Map `Standard_SStream` (`std::stringstream`) — mainly used in `Raise()` methods on exception classes |
| 19 | `XCAFPrs_Style` | Add `XCAFPrs_Style` — used in XCAF presentation styles |
| 12 | `IMeshData::IEdgeHandle` | Nested handle typedef in meshing internals — low priority |
| 11 | `ShapeProcess::OperationsFlags` | Nested type in ShapeProcess — used in STEP/IGES processing flags |
| 10 | `IMeshData::IFaceHandle` | Nested handle typedef in meshing internals — low priority |

Previously common unknown types that have been resolved:
- ~~`math_Vector`~~ (was 665 skips) — resolved by fixing typedef collection filter that excluded lowercase-prefix OCCT types (`math_*`)
- ~~`Standard_Character`~~ (was 33 skips) — now mapped as `c_char` (`i8`)
- ~~`Standard_ExtString`~~ (was 26 skips) — now mapped as `*const u16`
- ~~`Standard_ExtCharacter`~~ (was 15 skips) — now mapped as `u16`
- ~~`Interfa
... (truncated, 9581 chars)
````
</details>

<details>
<summary>Reading lines 407-480 of <a href="../crates/opencascade-binding-generator/README.md#L407-L481">README.md</a></summary>

````
### Important Skipped Symbols

Most skipped symbols are in internal, low-use, or specialized modules. However, some affect functionality that users commonly need:

**Data Exchange (40 symbols)** — Reduced from 189 after adding `Transfer_TransientProcess`, `Interface_Protocol`, and `Interface_EntityIterator` to the binding set. Remaining skips in `STEPControl_*` (5), `IGESControl_*` (4), `XSControl_*` (27), and `RWStl` (4) are mostly stream types (`Standard_OStream`/`Standard_IStream`), string ref params (`const char*&` in `XSControl_Vars`), unknown IGES-specific Handle types (`Handle(IGESData_IGESModel)`), and rvalue references (`XSAlgo_ShapeProcessor::ParameterMap&&`). The core `Read()`/`Write()` operations are fully bound. **Remaining unblock opportunity**: add `IGESData_IGESModel` (14 skips).

**Document Framework (86 symbols)** — `TDocStd_*` (38 skipped), `TDF_*` (18 skipped), `XCAFDoc_*` (30 skipped), `XCAFPrs_*` (0 skipped). Previously had 215+ skipped symbols due to `TDF_LabelMap` and `TDF_AttributeMap` being unknown — those are now resolved via the header text scan fallback. Remaining skips are dominated by stream types (`Standard_OStream`), with raw pointers, unknown types (`TDocStd_XLinkPtr`, `TDF_LabelNodePtr`), and XCAFDoc-specific handles.

**Shape Meshing (93 symbols across 3 modules)** — `BRepMesh_*` (78 skipped), `IMeshData_*` (14 skipped), `IMeshTools_*` (1 skipped). The `IMeshData` and `IMeshTools` modules are now in the binding set, but many BRepMesh methods reference internal mesh data types (`IMeshData::IEdgeHandle`, `IMeshData::IFaceHandle`, `IMeshData::MapOfInteger`) that are nested typedefs not yet resolvable. Also includes C-style array params and `std::pair` return types. The core `BRepMesh_IncrementalMesh` meshing API is fully bound.

**Shape Analysis/Fix (0 symbols)** — Reduced from 72 after fixing the OSD_WNT.hxx fatal parse error root cause and adding `ShapeBuild_ReShape` and `GeomAdaptor_Surface`. The 54 legitimate `Standard_Integer&` mode accessors in `ShapeFix_*` are now bound. The remaining 5 `TColgp_SequenceOfPnt` skips were resolved by fixing typedef map pollution (see above).

**Geometry (7 symbols in gp/Geom/Geom2d)** — Raw pointer returns (`gp_XYZ::GetData()`, `Geom_BSplineCurve::Weights()`, `Geom_BSplineSurface::Weights()`) and `Standard_SStream` parameters (exception `Raise()` methods). All core geometry operations are available; only internal data access and exception-raising methods are skipped.

**Poly (15 symbols)** — Raw pointers, ambiguous lifetimes, C-style arrays, stream types, void pointers, and misc others. `Poly_CoherentTriangulation` internal access and `Poly_MakeLoops` helper interfaces. All core triangulation access is available.

### How Skipped Symbols Are Tracked

All filtering decisions happen in two places:
- `codegen/bindings.rs`: `is_method_bindable()`, `is_constructor_bindable()`, `is_static_method_bindable()` return `Result<(), String>` with a human-readable reason on `Err`
- `compute_class_bindings()` and `compute_all_function_bindings()` collect `SkippedSymbol` structs for every rejected symbol

The `emit_reexport_class()` and `emit_skipped_functions()` functions write the skip comments to the generated module files.

---

## Implementation Details

The generated code has two layers: a `pub(crate)` FFI module containing the raw extern "C" bindings, and public per-module re-export files that provide the user-facing API.

### Internal: `ffi.rs` (the FFI declarations)

A single `extern "C"` block declares all types and functions using full C++ names. This is `pub(crate)` — users never interact with it directly.

Types use their full C++ identifiers as Rust names to avoid collisions:

```rust
// generated/ffi.rs (pub(crate), not user-facing)
extern "C" {
    // Opaque C++ types -- full C++ names
    pub fn gp_Pnt_ctor_real3(theXp: f64, theYp: f64, theZp: f64) -> *mut gp_Pnt;
    pub fn gp_Pnt_destructor(self_: *mut gp_Pnt);
    pub fn gp_Pnt_x(self_: *const gp_Pnt) 
... (truncated, 4955 chars)
````
</details>

Now let me also check the "Generated Public API" section to see if there's a iostream/OStream usage example I should add, and also look at the Manual Bindings section:

<details>
<summary>Reading lines 310-342 of <a href="../crates/opencascade-binding-generator/README.md#L310-L343">README.md</a></summary>

```
- `handle.downcast_to_derived()` — type-checked downcast
- `handle.get()` / `handle.get_mut()` — dereference handle to contained object

### Collection Types

NCollection typedefs (e.g., `TopTools_ListOfShape`) get iterator wrappers:
- C++ iterator struct wrapping `const_iterator` or indexed access
- `TypeName_iter()` / `TypeNameIterator_next()` C++ functions
- Rust `Iterator` trait impl yielding `OwnedPtr<Element>`
- Impl methods: `iter()`, `from_iter()`, `append()`, etc.

### Naming Conventions

- **Types in ffi.rs**: Full C++ names (`gp_Pnt`, `TopoDS_Shape`, `BRepPrimAPI_MakeBox`)
- **Types in re-exports**: Short names (`Pnt`, `Shape`, `MakeBox`) via `pub use crate::ffi::X as Y;`
- **Methods**: snake_case (generated by the code generator)
- **Overloads**: Compressed parameter-type suffix (`_real3` not `_real_real_real`, `_pnt2` not `_pnt_pnt`)
- **Enums**: `TopAbs_ShapeEnum` -> `ShapeEnum`, variants `TopAbs_COMPOUND` -> `Compound`
- **Reserved names**: `Vec_` in ffi, re-exported as `Vec`

### Manual Bindings

Some C++ function signatures can't be auto-generated — for example, methods with `const char*&` or `const char* const&` parameters (a reference to a `const char*`). The generator detects these (`ExclusionReason::StringRefParam` in `resolver.rs`) and skips them automatically.

Manual replacements live in `crates/opencascade-sys/manual/`:
- `<module>.rs` — `extern "C"` declarations + `impl` blocks
- `<module>_wrappers.cpp` — C++ wrapper functions

The generator appends `include!("../manual/<module>.rs");` (with a comment explaining why) to the generated module re-export file when a corresponding `manual/<module>.rs` exists. Because `include!()` is a textual insertion, the manual code has full access to the module's type aliases (e.g., `AdvancedEvolved`, `Finder`). The `extern "C"` declarations in manual files are not marked `pub`, so they are private to the module and not exposed as part of the public API. `build.rs` globs `manual/*_wrappers.cpp` and compiles them alongside `generated/wrappers.cpp`. Since Rust allows multiple `impl` blocks for a type, manual methods appear seamlessly alongside the auto-generated ones.

See `crates/opencascade-sys/manual/` and the comments in `bindings.toml` for the two existing examples (`Transfer_Finder::GetStringAttribute` and `BRepFill_AdvancedEvolved::SetTemporaryDirectory`).

---
```
</details>

Now I have all the context. Let me make the README updates. The key changes are:

1. Update the skip count from ~1,519 to ~1,036
2. Replace the "Stream type" row (546 → now just 15 shared_ptr streams), update all counts
3. Add "Standard_OStream/Standard_IStream" to the resolved section 
4. Update the "Most Common Unknown Types" table
5. Update the "Important Skipped Symbols" section counts
6. Add iostream/OStream usage to the "Manual Bindings" section
7. Update the "manual_types" config description  Now let me apply all the README edits:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/README.md#L340-L341">README.md -1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L346-L347">-1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L363-L380">-17+17</a>, <a href="../crates/opencascade-binding-generator/README.md#L383-L384">-1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L385-L394">-8+9</a>, <a href="../crates/opencascade-binding-generator/README.md#L395-L408">-12+13</a>, <a href="../crates/opencascade-binding-generator/README.md#L413-L414">-1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L415-L416">-1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L417-L418">-1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "340:zb",
      "content": "See `crates/opencascade-sys/manual/` and the comments in `bindings.toml` for examples (`Transfer_Finder::GetStringAttribute`, `BRepFill_AdvancedEvolved::SetTemporaryDirectory`, and `standard` iostream accessors)."
    },
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "346:ae",
      "content": "The binding generator skips ~1,036 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Every skipped symbol is documented in the generated per-module `.rs` files as a `// SKIPPED:` comment block including:"
    },
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "363:fa,364:oi,365:nh,366:jz,367:hv,368:vo,369:lm,370:mh,371:pj,372:gc,373:dq,374:ki,375:hg,376:vv,377:bo,378:ma,379:ek",
      "content": "| Count | % | Category | Description |\n|------:|----:|----------|-------------|\n| 356 | 34.4% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`Handle(TDocStd_Document)`, `std::istream&`, `XCAFPrs_Style`, `IMeshData::IEdgeHandle`, etc.) |\n| 200 | 19.3% | **Void pointer** | `Standard_Address` (typedef for `void*`) — cannot be safely expressed in Rust FFI |\n| 176 | 17.0% | **Ambiguous lifetimes** | `&mut` return with reference params — Rust lifetime inference is ambiguous |\n| 134 | 12.9% | **Raw pointer** | `T*`/`const T*` returns (static/free fn) or primitive pointer params (`int*`, `double*`) — class pointer returns on instance methods are now bound as `Option<&T>`/`Option<&mut T>` |\n| 48 | 4.6% | **Unresolved template type** | Template instantiations that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `LDOMBasicString`, etc.) |\n| 43 | 4.2% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |\n| 17 | 1.6% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |\n| 16 | 1.5% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |\n| 15 | 1.4% | **Stream (shared_ptr)** | `std::shared_ptr<std::istream/ostream>` — smart-pointer-wrapped streams not yet bindable |\n| 12 | 1.2% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across FFI |\n| 7 | 0.7% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(IGESData_IGESModel)`, `Handle(CDM_MetaData)`, etc.) |\n| 5 | 0.5% | **Not CppDeletable** | Return type class has no destructor in the binding set |\n| 4 | 0.4% | **&mut enum return** | Mutable reference to enum (cxx limitation) |\n| 2 | 0.2% | **Excluded by bindings.toml** | Explicitly excluded in config (e.g., ambiguous overload workarounds) |\n| 1 | 0.1% | **Ambiguous overload** | C++ overload that would produce identical wrapper signatures |"
    },
    {
      "filePath": "
... (truncated, 7975 chars)
```
**Output:**
```
{
  "applied": 9,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/README.md",
      "startLine": 340,
      "endLine": 417
    }
  ]
}
```
</details>

Now let me also add a section about the iostream/OStream API in the Generated Public API area, and update the Manual Bindings section. Let me also add a "Standard Streams" subsection:

<details>
<summary>Reading lines 310-330 of <a href="../crates/opencascade-binding-generator/README.md#L310-L331">README.md</a></summary>

```
- `handle.downcast_to_derived()` — type-checked downcast
- `handle.get()` / `handle.get_mut()` — dereference handle to contained object

### Collection Types

NCollection typedefs (e.g., `TopTools_ListOfShape`) get iterator wrappers:
- C++ iterator struct wrapping `const_iterator` or indexed access
- `TypeName_iter()` / `TypeNameIterator_next()` C++ functions
- Rust `Iterator` trait impl yielding `OwnedPtr<Element>`
- Impl methods: `iter()`, `from_iter()`, `append()`, etc.

### Naming Conventions

- **Types in ffi.rs**: Full C++ names (`gp_Pnt`, `TopoDS_Shape`, `BRepPrimAPI_MakeBox`)
- **Types in re-exports**: Short names (`Pnt`, `Shape`, `MakeBox`) via `pub use crate::ffi::X as Y;`
- **Methods**: snake_case (generated by the code generator)
- **Overloads**: Compressed parameter-type suffix (`_real3` not `_real_real_real`, `_pnt2` not `_pnt_pnt`)
- **Enums**: `TopAbs_ShapeEnum` -> `ShapeEnum`, variants `TopAbs_COMPOUND` -> `Compound`
- **Reserved names**: `Vec_` in ffi, re-exported as `Vec`

### Manual Bindings
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/README.md#L319-L343">README.md -1+24</a></summary>

**Input:**
````
{
  "edits": [
    {
      "content": "- Impl methods: `iter()`, `from_iter()`, `append()`, etc.\n\n### Standard Streams (iostream)\n\nOCCT uses `Standard_OStream` (typedef for `std::ostream`) and `Standard_IStream` (typedef for `std::istream`) in many debug/dump methods. These are declared as `manual_types` in `bindings.toml` so the generator recognizes them as known types without generating class bindings.\n\nManual bindings in the `standard` module provide access to the global C++ stream objects:\n\n```rust\nuse opencascade_sys::standard;\nuse opencascade_sys::b_rep_tools;\n\n// Get a mutable reference to std::cout (valid for program lifetime)\nlet cout = standard::cout();\n\n// Dump a shape's topological structure to stdout\nlet mut make_box = opencascade_sys::b_rep_prim_api::MakeBox::new_real3(10.0, 20.0, 30.0);\nlet shape = make_box.shape();\nb_rep_tools::dump_shape_ostream(shape, cout);\n\n// Also available: standard::cerr(), standard::clog(), standard::cin()\n```\n\nThe `OStream` and `IStream` type aliases are re-exported from the `standard` module alongside the auto-generated Standard types.",
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "319:gf"
    }
  ]
}
````
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/README.md","startLine":319,"endLine":342}
```
</details>

Now let me also update the Manual Bindings section to mention the iostream manual bindings as a third example:

<details>
<summary>Reading lines 349-365 of <a href="../crates/opencascade-binding-generator/README.md#L349-L366">README.md</a></summary>

```
- **Overloads**: Compressed parameter-type suffix (`_real3` not `_real_real_real`, `_pnt2` not `_pnt_pnt`)
- **Enums**: `TopAbs_ShapeEnum` -> `ShapeEnum`, variants `TopAbs_COMPOUND` -> `Compound`
- **Reserved names**: `Vec_` in ffi, re-exported as `Vec`

### Manual Bindings

Some C++ function signatures can't be auto-generated — for example, methods with `const char*&` or `const char* const&` parameters (a reference to a `const char*`). The generator detects these (`ExclusionReason::StringRefParam` in `resolver.rs`) and skips them automatically.

Manual replacements live in `crates/opencascade-sys/manual/`:
- `<module>.rs` — `extern "C"` declarations + `impl` blocks
- `<module>_wrappers.cpp` — C++ wrapper functions

The generator appends `include!("../manual/<module>.rs");` (with a comment explaining why) to the generated module re-export file when a corresponding `manual/<module>.rs` exists. Because `include!()` is a textual insertion, the manual code has full access to the module's type aliases (e.g., `AdvancedEvolved`, `Finder`). The `extern "C"` declarations in manual files are not marked `pub`, so they are private to the module and not exposed as part of the public API. `build.rs` globs `manual/*_wrappers.cpp` and compiles them alongside `generated/wrappers.cpp`. Since Rust allows multiple `impl` blocks for a type, manual methods appear seamlessly alongside the auto-generated ones.

See `crates/opencascade-sys/manual/` and the comments in `bindings.toml` for examples (`Transfer_Finder::GetStringAttribute`, `BRepFill_AdvancedEvolved::SetTemporaryDirectory`, and `standard` iostream accessors).

---
```
</details>

Good, the manual bindings mention is already updated from the earlier edit. Now let me also update the Wrapper Functions table to mention stream types:

Searched for text `manual_types` (`**/crates/opencascade-binding-generator/README.md`), 2 results
- [README.md:323](../crates/opencascade-binding-generator/README.md#L323)
- [README.md:419](../crates/opencascade-binding-generator/README.md#L419)

Good. Let me now verify the full README builds correctly by reading a broader section around the edits:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build 2>&1 | grep -E "error|warning" | head -5
```
```
(base) whitelily:opencascade-rs sethml$
```

Clean build. Now let me also check the Geometry and Poly important-symbols sections, since those counts may have changed too:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && echo "=== Geometry ===" && grep -c "^// SKIPPED:" crates/opencascade-sys/generated/gp.rs crates/opencascade-sys/generated/geom.rs crates/opencascade-sys/generated/geom2d.rs 2>/dev/null | awk -F: '{s+=$2; print} END {print "Total: " s}' && echo "=== Poly ===" && grep -c "^// SKIPPED:" crates/opencascade-sys/generated/poly.rs 2>/dev/null
```
<details>
<summary>Output (7 lines)</summary>

```
=== Geometry ===
crates/opencascade-sys/generated/gp.rs:2
crates/opencascade-sys/generated/geom.rs:0
crates/opencascade-sys/generated/geom2d.rs:0
Total: 2
=== Poly ===
13
```
</details>

Let me update those sections too:

<details>
<summary>Reading lines 437-445 of <a href="../crates/opencascade-binding-generator/README.md#L437-L446">README.md</a></summary>

```

**Document Framework (74 symbols)** — `TDocStd_*` (36 skipped), `TDF_*` (9 skipped), `XCAFDoc_*` (29 skipped), `XCAFPrs_*` (0 skipped). Previously had 215+ skipped symbols due to `TDF_LabelMap` and `TDF_AttributeMap` being unknown — those are now resolved via the header text scan fallback. Many previous stream-type skips are now unblocked. Remaining skips are dominated by `Handle(TDocStd_Document)` params, with raw pointers, unknown types (`TDocStd_XLinkPtr`, `TDF_LabelNodePtr`), and XCAFDoc-specific handles.

**Shape Meshing (91 symbols across 3 modules)** — `BRepMesh_*` (76 skipped), `IMeshData_*` (14 skipped), `IMeshTools_*` (1 skipped). The `IMeshData` and `IMeshTools` modules are now in the binding set, but many BRepMesh methods reference internal mesh data types (`IMeshData::IEdgeHandle`, `IMeshData::IFaceHandle`, `IMeshData::MapOfInteger`) that are nested typedefs not yet resolvable. Also includes C-style array params and `std::pair` return types. The core `BRepMesh_IncrementalMesh` meshing API is fully bound.

**Shape Analysis/Fix (0 symbols)** — Reduced from 72 after fixing the OSD_WNT.hxx fatal parse error root cause and adding `ShapeBuild_ReShape` and `GeomAdaptor_Surface`. The 54 legitimate `Standard_Integer&` mode accessors in `ShapeFix_*` are now bound. The remaining 5 `TColgp_SequenceOfPnt` skips were resolved by fixing typedef map pollution (see above).

**Geometry (7 symbols in gp/Geom/Geom2d)** — Raw pointer returns (`gp_XYZ::GetData()`, `Geom_BSplineCurve::Weights()`, `Geom_BSplineSurface::Weights()`) and `Standard_SStream` parameters (exception `Raise()` methods). All core geometry operations are available; only internal data access and exception-raising methods are skipped.

```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/README.md#L444-L445">README.md -1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L446-L447">-1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "**Geometry (2 symbols in gp/Geom/Geom2d)** — Down from 7 after stream type support unblocked `Raise()` methods. Remaining: raw pointer returns (`gp_XYZ::GetData()` returning `const double*`, `gp_XYZ::ChangeData()` returning `double*`). All core geometry operations are available.",
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "444:za"
    },
    {
      "content": "**Poly (13 symbols)** — Raw pointers, ambiguous lifetimes, C-style arrays, void pointers, and misc others. `Poly_CoherentTriangulation` internal access and `Poly_MakeLoops` helper interfaces. All core triangulation access is available.",
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "446:xu"
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
      "file": "crates/opencascade-binding-generator/README.md",
      "startLine": 444,
      "endLine": 446
    }
  ]
}
```
</details>

Now let me also check for the `Standard_SStream` mention in the unknown types since it was listed as 86 previously but is no longer in the top unknown types (it's now a manual_type so it's known):

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -c "Standard_SStream" crates/opencascade-sys/generated/*.rs | awk -F: '$2>0' | head -5
```
<details>
<summary>Output (5 lines)</summary>

```
crates/opencascade-sys/generated/b_rep_algo_api.rs:2
crates/opencascade-sys/generated/b_rep_extrema.rs:1
crates/opencascade-sys/generated/bop_algo.rs:6
crates/opencascade-sys/generated/expr_intrp.rs:1
crates/opencascade-sys/generated/expr.rs:6
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -h "Standard_SStream" crates/opencascade-sys/generated/ffi.rs | head -5
```
<details>
<summary>Output (5 lines)</summary>

```
pub struct Standard_SStream {
    pub fn BRepExtrema_UnCompatibleShape_raise_sstream(theMessage: *mut Standard_SStream);
    pub fn Expr_ExprFailure_raise_sstream(theMessage: *mut Standard_SStream);
    pub fn Expr_InvalidAssignment_raise_sstream(theMessage: *mut Standard_SStream);
    pub fn Expr_InvalidFunction_raise_sstream(theMessage: *mut Standard_SStream);
```
</details>

Good — `Standard_SStream` is now a known type and its methods are bound. The `Standard_SStream` entry in the old "Most Common Unknown Types" table is now resolved. Let me now amend the commit:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git add -A && git diff --cached --stat
```
```
 crates/opencascade-binding-generator/README.md | 73 +++++++++++++++++---------
 crates/opencascade-sys/manual/standard.rs      | 14 ++---
 crates/opencascade-sys/tests/iostream.rs       |  4 +-
 3 files changed, 58 insertions(+), 33 deletions(-)
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git commit --amend --no-edit
```
<details>
<summary>Output (6 lines)</summary>

```
[main ad174b8] Support iostream/ostream types in generated bindings
 Date: Thu Feb 19 20:23:58 2026 -0800
 107 files changed, 17007 insertions(+), 3961 deletions(-)
 create mode 100644 crates/opencascade-sys/manual/standard.rs
 create mode 100644 crates/opencascade-sys/manual/standard_wrappers.cpp
 create mode 100644 crates/opencascade-sys/tests/iostream.rs
```
</details>

Amended as `ad174b8`. Changes:

- Updated skip count in README from ~1,519 to ~1,036
- Replaced "Stream type" (546) row with "Stream (shared_ptr)" (15) — only  types remain unbindable
- Updated all category counts and percentages in the breakdown table
- Updated "Most Common Unknown Types" table (removed `Standard_SStream`, added , `void*` returns)
- Added `Standard_OStream`/`Standard_IStream` to the resolved types list
- Added "Standard Streams (iostream)" subsection with usage example showing safe `standard::cout()` API
- Updated important skipped symbol counts (Data Exchange 40→57, Document Framework 86→74, Geometry 7→2, Poly 15→13)
- Updated manual bindings mentions to include iostream accessors
- Incorporated your change making `cout()`/`cerr()`/`clog()`/`cin()` safe (non-`unsafe`) functions

*2026-02-19 21:30 · 366s · Words: 4,214 in · 4,409 out · 109,739 ctx · 50 rounds*

---
