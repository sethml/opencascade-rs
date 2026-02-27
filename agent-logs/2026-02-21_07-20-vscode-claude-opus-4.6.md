# Name Collision Disambiguation Fix Request

- **Date:** 2026-02-21 06:54 – 07:38
- **Model:** Claude Opus 4.6
- **Turns:** 2
- **Tool calls:** 55
- **Thinking blocks:** 12
- **Input words:** 4,535
- **Output words:** 3,312
- **Prompt tokens (last round):** 124,766
- **API rounds:** 55
- **Total elapsed:** 719s

## Table of Contents

1. [I think our name collision disambiguation should work here, and the ::New functions should end up...](#user-1) (Claude Opus 4.6)
2. [Explain this binding. In C++, Storage_DefaultCallback::New() returns a handle&lt;Standard_Persistent...](#user-2) (Claude Opus 4.6)

---

<a id="user-1"></a>

## User (1)

> I think our name collision disambiguation should work here, and the ::New functions should end up _new2 or something. Fix it so collision disambiguation takes the constructor and methods into account, remove these, and commit.

*2026-02-21 06:55*

### Assistant — Claude Opus 4.6

> 💭 The user wants me to fix the name collision disambiguation so that `Storage_CallBack::New` and `Storage_DefaultCallBack::New` don't need to be excluded. The issue is that when we generate Rust bindings, the C++ `New()` factory method name-collides with the constructor wrapper `fn new()`. The user wants the disambiguation logic to handle this by renaming one of them (e.g., `new2`).
> 
> Let me first understand the current code structure and how name collision disambiguation works.

**Todo list:**
- 🔄 Understand collision disambiguation code
- ⬜ Fix disambiguation for constructor+method collisions
- ⬜ Remove exclude_methods entries
- ⬜ Regenerate bindings and test
- ⬜ Commit changes

Searching codebase for "name collision disambiguation rust method name dedup rename"

> 💭 Now I need to understand how the method name disambiguation works, specifically how constructors and methods interact. Let me look at the key functions: `compute_direct_method_names` and the code that generates constructor wrappers (named `new`).
> 
> The issue is: `Storage_CallBack::New` is a C++ method that gets mapped to `new` in Rust (since `New` → `new` via snake_case conversion), which collides with the constructor wrapper `fn new()`. The disambiguation system should handle overloaded methods with the same Rust name, but it probably doesn't consider that constructors also use the name `new`.
> 
> 
> I'm examining the relevant code sections...

Let me look at the disambiguation code and how constructors interact with methods.

Searched for text `compute_direct_method_names` (`**/*.rs`), 2 results
- [bindings.rs:1276](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1276)
- [bindings.rs:1653](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1653)

<details>
<summary>Reading lines 1276-1420 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1276-L1421">bindings.rs</a></summary>

```
fn compute_direct_method_names(methods: &[&Method]) -> Vec<String> {
    let mut name_counts: HashMap<String, usize> = HashMap::new();
    for method in methods {
        *name_counts.entry(method.name.clone()).or_insert(0) += 1;
    }

    let mut seen_names: HashMap<String, usize> = HashMap::new();

    methods
        .iter()
        .map(|method| {
            let needs_suffix = name_counts.get(&method.name).copied().unwrap_or(0) > 1;
            let base_suffix = if needs_suffix {
                let base_suffix = method.overload_suffix();
                let same_suffix_diff_const = methods.iter().any(|m| {
                    m.name == method.name
                        && m.overload_suffix() == base_suffix
                        && m.is_const != method.is_const
                });
                if same_suffix_diff_const && !method.is_const {
                    format!("{}_mut", base_suffix)
                } else {
                    base_suffix
                }
            } else {
                String::new()
            };

            let base_rust_name = safe_method_name(&method.name);
            let candidate_name = if base_suffix.is_empty() {
                base_rust_name.clone()
            } else {
                format!("{}{}", base_rust_name, base_suffix)
            };

            let count = seen_names.entry(candidate_name.clone()).or_insert(0);
            *count += 1;
            if *count > 1 {
                let suffix = format!("{}_{}", base_suffix, count);
                if suffix.is_empty() {
                    base_rust_name
                } else {
                    format!("{}{}", base_rust_name, suffix)
                }
            } else if base_suffix.is_empty() {
                base_rust_name
            } else {
                format!("{}{}", base_rust_name, base_suffix)
            }
        })
        .collect()
}

/// Compute overload suffix with const/mut disambiguation for wrapper methods.
/// Returns the base fn_name (without class prefix) for each method.
fn compute_wrapper_method_names(methods: &[&Method]) -> Vec<String> {
    let mut name_counts: HashMap<String, usize> = HashMap::new();
    for method in methods {
        *name_counts.entry(method.name.clone()).or_insert(0) += 1;
    }

    // Pass 1: resolve C++ overloads (same C++ name, different params)
    let mut names: Vec<String> = methods
        .iter()
        .map(|method| {
            let base_name = safe_method_name(&method.name);
            let needs_suffix = name_counts.get(&method.name).copied().unwrap_or(0) > 1;
            if needs_suffix {
                let base_suffix = method.overload_suffix();
                let same_suffix_diff_const = methods.iter().any(|m| {
                    m.name == method.name
                        && m.overload_suffix() == base_suffix
                        && m.is_const != method.is_const
                });
                let suffix = if same_suffix_diff_const && !method.is_const {
                    format!("{}_mut", base_suffix)
                } else {
                    base_suffix
                };
                combine_name_suffix(&base_name, &suffix)
            } else {
                base_name
            }
        })
        .collect();

    // Pass 2: resolve cross-name collisions (different C++ names that produce
    // the same snake_case name, e.g. SetInteger/setInteger → set_integer).
    // Append _2, _3, ... to later duplicates.
    let mut seen: HashMap<String, usize> = HashMap::new();
    for name in &mut names {
        let count = seen.entry(name.clone()).or_insert(0);
        *count += 1;
        if *count > 1 {
            *name = format!("{}_{}", name, count);
        }
    }

    names
}

/// Compute static method names with 3-level conflict resolution.
/// Returns (ffi_fn_name_base, impl_method_name) for each method.
fn compute_static_method_names(
    cpp_name: &str,
    methods: &[&StaticMethod],
    reserved_names: &Has
... (truncated, 5555 chars)
```
</details>

<details>
<summary>Reading lines 1630-1720 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1630-L1721">bindings.rs</a></summary>

```
                module: class.module.clone(),
                cpp_name: format!("{}::{}", class.name, method.name),
                source_header: class.source_header.clone(),
                source_line: method.source_line,
                doc_comment: method.comment.clone(),
                skip_reason: reason,
                stub_rust_decl: generate_method_stub(cpp_name, method),
            });
            continue;
        }
        bindable_methods.push(method);
    }

    // Partition into direct vs wrapper
    let direct_methods_raw: Vec<&Method> = bindable_methods.iter()
        .filter(|m| !needs_wrapper_function(m, all_enum_names))
        .copied()
        .collect();
    let wrapper_methods_raw: Vec<&Method> = bindable_methods.iter()
        .filter(|m| needs_wrapper_function(m, all_enum_names))
        .copied()
        .collect();

    let direct_method_names = compute_direct_method_names(&direct_methods_raw);
    let direct_methods: Vec<DirectMethodBinding> = direct_methods_raw
        .iter()
        .zip(direct_method_names.iter())
        .map(|(method, rust_name)| {
            let params: Vec<ParamBinding> = method
                .params
                .iter()
                .map(|p| build_param_binding(&p.name, &p.ty, p.is_nullable_ptr(), ffi_ctx, reexport_ctx))
                .collect();
            let mut return_type = method
                .return_type
                .as_ref()
                .map(|ty| build_return_type_binding(ty, ffi_ctx, reexport_ctx));

            // If the method is const (&self) and returns a class pointer,
            // downgrade Option<&mut T> to Option<&T> to avoid unsound &self -> &mut T.
            if method.is_const {
                if let Some(ref mut rt) = return_type {
                    if rt.is_class_ptr_return && rt.rust_reexport_type.starts_with("Option<&mut ") {
                        rt.rust_reexport_type = rt.rust_reexport_type.replace("Option<&mut ", "Option<&");
                    }
                }
            }

            DirectMethodBinding {
                rust_name: rust_name.clone(),
                cxx_name: method.name.clone(),
                is_const: method.is_const,
                params,
                return_type,
                doc_comment: method.comment.clone(),
                source_line: method.source_line,
                is_unsafe: method.has_unsafe_types(),
            }
        })
        .collect();

    let wrapper_fn_names = compute_wrapper_method_names(&wrapper_methods_raw);

    // Build reserved_names set for static method conflict detection
    let mut reserved_names: HashSet<String> = HashSet::new();
    for fn_name in &wrapper_fn_names {
        reserved_names.insert(format!("{}_{}", cpp_name, fn_name));
    }

    // Build FFI method names set (for re-export conflict detection)
    let cxx_method_names: HashSet<String> = direct_methods_raw
        .iter()
        .map(|m| safe_method_name(&m.name))
        .collect();

    // Build all_instance_method_names (direct + wrapper impl names + constructor names)
    let mut all_instance_method_names: HashSet<String> = cxx_method_names.clone();
    // Include constructor impl_method_names so static methods don't collide with them
    for ctor in &constructors {
        all_instance_method_names.insert(ctor.impl_method_name.clone());
    }

    let wrapper_methods: Vec<WrapperMethodBinding> = wrapper_methods_raw
        .iter()
        .zip(wrapper_fn_names.iter())
        .map(|(method, fn_name)| {
            let ffi_fn_name = format!("{}_{}", cpp_name, fn_name);

            // Compute impl_method_name: may differ if fn_name conflicts with a direct method
            let impl_method_name = if cxx_method_names.contains(fn_name) {
                let suffix = method.overload_suffix();
```
</details>

> 💭 I can see the code structure. Now I need to understand how constructors are handled and how the wrapper method names interact with them. The key issue is:
> 
> 1. `Storage_CallBack::New` is a C++ static method that becomes `new` in snake_case Rust
> 2. The constructor wrapper also uses `new` as the method name
> 3. The disambiguation needs to account for constructor names when resolving method names
> 
> Let me look at how constructors are built and named, and where they interact with methods.

Now let me look at how constructors are built and what name they get, and how static methods check for collisions:

<details>
<summary>Reading lines 1420-1460 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1420-L1461">bindings.rs</a></summary>

```
                if all_instance_method_names.contains(&ffi_fn_name_base) {
                    let suffix = method.overload_suffix();
                    if suffix.is_empty() {
                        format!("{}_static", ffi_fn_name_base)
                    } else {
                        combine_name_suffix(&base_name, &suffix)
                    }
                } else {
                    ffi_fn_name_base.clone()
                };

            (ffi_fn_name_base, impl_method_name)
        })
        .collect();

    // Pass 2: resolve cross-name collisions (different C++ names that produce
    // the same snake_case name). Append _2, _3, ... to later duplicates.
    let mut seen_ffi: HashMap<String, usize> = HashMap::new();
    let mut seen_impl: HashMap<String, usize> = HashMap::new();
    for (ffi_name, impl_name) in &mut results {
        let ffi_count = seen_ffi.entry(ffi_name.clone()).or_insert(0);
        *ffi_count += 1;
        if *ffi_count > 1 {
            *ffi_name = format!("{}_{}", ffi_name, ffi_count);
        }
        let impl_count = seen_impl.entry(impl_name.clone()).or_insert(0);
        *impl_count += 1;
        if *impl_count > 1 {
            *impl_name = format!("{}_{}", impl_name, impl_count);
        }
    }

    results
}

// ── Abstract class detection ────────────────────────────────────────────────

/// Check if a class is effectively abstract by walking the inheritance chain.
///
/// A class is effectively abstract if:
/// 1. It declares pure virtual methods itself (`is_abstract` flag), OR
```
</details>

Searched for regex `impl_method_name.*new` (`**/bindings.rs`), 2 results
- [bindings.rs:1550](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1550)
- [bindings.rs:5621](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5621)

<details>
<summary>Reading lines 1475-1630 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1475-L1631">bindings.rs</a></summary>

```
    let mut all_concrete: HashSet<String> = HashSet::new();

    for ancestor_name in symbol_table.get_all_ancestors_by_name(&class.name) {
        if let Some(ancestor) = all_classes_by_name.get(&ancestor_name) {
            all_pvms.extend(ancestor.pure_virtual_methods.iter().cloned());
            // Concrete = all methods minus pure virtual declarations
            for m in &ancestor.all_method_names {
                if !ancestor.pure_virtual_methods.contains(m) {
                    all_concrete.insert(m.clone());
                }
            }
        }
    }

    // This class's own methods are concrete (is_abstract is false)
    all_concrete.extend(class.all_method_names.iter().cloned());

    // If any pure virtual method is not overridden, the class is abstract
    all_pvms.iter().any(|pvm| !all_concrete.contains(pvm))
}

// ── Main compute function ───────────────────────────────────────────────────

/// Compute all binding decisions for a class.
///
/// This is the SINGLE place where filtering, naming, overload suffixes,
/// and used_names conflict resolution happen.
pub fn compute_class_bindings(
    class: &ParsedClass,
    ffi_ctx: &TypeContext,
    symbol_table: &SymbolTable,
    handle_able_classes: &HashSet<String>,
    all_classes_by_name: &HashMap<String, &ParsedClass>,
    reexport_ctx: Option<&ReexportTypeContext>,
    exclude_methods: &HashSet<(String, String)>,
) -> ClassBindings {
    // Flatten C++ nested class names (e.g., "Parent::Child" -> "Parent_Child")
    // for use as valid Rust identifiers in ffi.rs
    let cpp_name = class.name.replace("::", "_");
    let cpp_name = &cpp_name;
    let all_enum_names = ffi_ctx.all_enums;

    let effectively_abstract = is_effectively_abstract(class, all_classes_by_name, symbol_table);

    let mut skipped_symbols: Vec<SkippedSymbol> = Vec::new();

    // ── Constructors ────────────────────────────────────────────────────────────
    let exclude_ctors = exclude_methods.contains(&(class.name.clone(), class.name.clone()))
        || exclude_methods.contains(&(class.name.clone(), "*".to_string()));
    let constructors = if !effectively_abstract && !class.has_protected_destructor {
        let mut ctors = compute_constructor_bindings(class, ffi_ctx, handle_able_classes, reexport_ctx);
        if exclude_ctors {
            // Record excluded constructors from bindings.toml
            for ctor in &class.constructors {
                skipped_symbols.push(SkippedSymbol {
                    kind: "constructor",
                    module: class.module.clone(),
                    cpp_name: format!("{}::{}", class.name, class.name),
                    source_header: class.source_header.clone(),
                    source_line: ctor.source_line,
                    doc_comment: ctor.comment.clone(),
                    skip_reason: "excluded by bindings.toml".to_string(),
                    stub_rust_decl: generate_ctor_stub(cpp_name, ctor),
                });
            }
            ctors.clear();
        }
        // If no bindable constructors AND no explicit constructors at all,
        // generate a synthetic default constructor (uses C++ implicit default).
        // We must NOT generate synthetic constructors when:
        // - The class has explicit constructors (even if filtered out) — C++ won't
        //   generate an implicit default constructor in that case
        if ctors.is_empty() && !class.has_explicit_constructors {
            ctors.push(ConstructorBinding {
                ffi_fn_name: format!("{}_ctor", cpp_name),
                impl_method_name: "new".to_string(),
                params: Vec::new(),
                cpp_arg_exprs: Vec::new(),
                doc_comment: Some("Default constructor".to_string()),
                source_line: None,
                convenience_of: None,
                is_unsafe: false,
            });
        }
        ctors
    } else {
        // Record skipped constructors for abstract/protected-destru
... (truncated, 7355 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/0c601d13-57cb-4b43-8289-968929687961/toolu_018DtsqrsazZ5RT53AbacLBM__vscode-1771608612604/content.txt)

> 💭 Now I need to see how constructors are computed and named, and specifically how `New` methods become `new` in the static method context. The key issue is:
> 
> 1. Constructors get `impl_method_name: "new"` (or "new2", "new3", etc.)
> 2. Static methods go through `compute_static_method_names` which checks against `reserved_names` (from wrapper methods) and `all_instance_method_names` (from direct + wrapper + constructors)
> 3. But `New` is a C++ static method. When converted to snake_case, `New` → `new`. So the static method name resolution should detect the collision with constructor `new`.
> 
> 
> 4. I need to verify that `compute_static_method_names` is actually checking constructor names when it builds the collision detection set.

Let me look at how static methods are handled and how `compute_static_method_names` interacts with constructors:

<details>
<summary>Reading lines 1720-1780 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1720-L1781">bindings.rs</a></summary>

```
                let suffix = method.overload_suffix();
                if suffix.is_empty() {
                    format!("{}_wrapper", fn_name)
                } else {
                    let base_name = safe_method_name(&method.name);
                    combine_name_suffix(&base_name, &suffix)
                }
            } else {
                fn_name.clone()
            };

            all_instance_method_names.insert(impl_method_name.clone());

            let params: Vec<ParamBinding> = method
                .params
                .iter()
                .map(|p| build_param_binding(&p.name, &p.ty, p.is_nullable_ptr(), ffi_ctx, reexport_ctx))
                .collect();
            let mut return_type = method
                .return_type
                .as_ref()
                .map(|ty| build_return_type_binding(ty, ffi_ctx, reexport_ctx));
            let wrapper_kind = classify_wrapper_kind(method, all_enum_names);

            // For ConstMutReturnFix, the wrapper takes non-const self even though
            // the C++ method is const. This ensures methods returning &mut use &mut self.
            let effective_is_const = if wrapper_kind == WrapperKind::ConstMutReturnFix {
                false
            } else {
                method.is_const
            };

            // If the method is const (&self) and returns a class pointer,
            // downgrade Option<&mut T> to Option<&T> to avoid unsound &self -> &mut T.
            if effective_is_const {
                if let Some(ref mut rt) = return_type {
                    if rt.is_class_ptr_return && rt.rust_reexport_type.starts_with("Option<&mut ") {
                        rt.rust_reexport_type = rt.rust_reexport_type.replace("Option<&mut ", "Option<&");
                    }
                }
            }

            WrapperMethodBinding {
                ffi_fn_name,
                impl_method_name,
                is_const: effective_is_const,
                params,
                return_type,
                wrapper_kind,
                cpp_method_name: method.name.clone(),
                doc_comment: method.comment.clone(),
                source_line: method.source_line,
                is_unsafe: method.has_unsafe_types(),
            }
        })
        .collect();

    // ── Static methods ──────────────────────────────────────────────────────────
    let mut static_methods_raw: Vec<&StaticMethod> = Vec::new();
    for method in &class.static_methods {
        if exclude_methods.contains(&(class.name.clone(), method.name.clone())) {
```
</details>

<details>
<summary>Reading lines 1780-1900 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1780-L1901">bindings.rs</a></summary>

```
        if exclude_methods.contains(&(class.name.clone(), method.name.clone())) {
            skipped_symbols.push(SkippedSymbol {
                kind: "static_method",
                module: class.module.clone(),
                cpp_name: format!("{}::{}", class.name, method.name),
                source_header: class.source_header.clone(),
                source_line: method.source_line,
                doc_comment: method.comment.clone(),
                skip_reason: "excluded by bindings.toml".to_string(),
                stub_rust_decl: generate_static_method_stub(cpp_name, method),
            });
            continue;
        }
        if let Err(reason) = is_static_method_bindable(method, ffi_ctx) {
            skipped_symbols.push(SkippedSymbol {
                kind: "static_method",
                module: class.module.clone(),
                cpp_name: format!("{}::{}", class.name, method.name),
                source_header: class.source_header.clone(),
                source_line: method.source_line,
                doc_comment: method.comment.clone(),
                skip_reason: reason,
                stub_rust_decl: generate_static_method_stub(cpp_name, method),
            });
            continue;
        }
        static_methods_raw.push(method);
    }

    let static_method_names = compute_static_method_names(
        cpp_name,
        &static_methods_raw,
        &reserved_names,
        &all_instance_method_names,
    );

    let static_methods: Vec<StaticMethodBinding> = static_methods_raw
        .iter()
        .zip(static_method_names.iter())
        .map(|(method, (ffi_fn_name_base, impl_method_name))| {
            let ffi_fn_name = format!("{}_{}", cpp_name, ffi_fn_name_base);

            let params: Vec<ParamBinding> = method
                .params
                .iter()
                .map(|p| build_param_binding(&p.name, &p.ty, p.is_nullable_ptr(), ffi_ctx, reexport_ctx))
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
                return_type,
                cpp_method_name: method.name.clone(),
                needs_static_lifetime,
                doc_comment: method.comment.clone(),
                source_line: method.source_line,
                is_unsafe: method.has_unsafe_types(),
            }
        })
        .collect();

    // ── Upcasts ─────────────────────────────────────────────────────────
    let upcasts = compute_upcast_bindings(class, symbol_table);

    // ── to_owned ──────────────────────────────────────────────────────────────────────────────
    // Detect copyability using libclang copy/move constructor detection.
    // has_copy_constructor: Some(true) = explicit usable copy ctor,
    //                       Some(false) = explicitly deleted/private,
    //                       None = no explicit copy ctor (implicit may exist)
    //
    // Handle-able classes (inheriting from Standard_Transient) always use to_handle()
    // instead of to_owned(), even if they have explicit copy constructors.
    //
    // For None (no explicit copy ctor), we fall back to a conservative module
    // allowlist because implicit copy constructors can be silently deleted when
    // a class has non-copyable members (e.g., algorithm classes with Extrema solvers).
    // Clang does not enumerate implicitly-deleted copy constructors.
    let is_handl
... (truncated, 5678 chars)
```
</details>

> 💭 Now I understand the flow. Looking at how static methods interact with constructors:
> 
> 1. At line 1705-1710, `all_instance_method_names` is built from direct methods + wrapper methods + constructor impl_method_names.
> 2. `compute_static_method_names` at Level 3 (line 1418-1429) checks if a static method's impl_name collides with `all_instance_method_names`. If it does, it appends `_static`.
> 
> So the flow is:
> 1. Constructors are computed → their `impl_method_name` values (like "new", "new2", etc.) are put into `all_instance_method_names`
> 
> 
> 2. When computing static method names, Level 3 checks against `all_instance_method_names`
> 
> Now I need to trace through what happens with `Storage_CallBack::New` — it's a C++ static factory method that converts to snake_case as "new", which should collide with the constructor's impl_method_name. Let me check what `safe_method_name("New")` actually produces to confirm this collision detection is working.

Let me check what  produces for "New":

Searched for text `fn safe_method_name` (`**/bindings.rs`), 1 result
- [bindings.rs:419](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L419)

<details>
<summary>Reading lines 419-470 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L419-L471">bindings.rs</a></summary>

```
fn safe_method_name(name: &str) -> String {
    let snake_name = name.to_snake_case();
    if RUST_KEYWORDS.contains(&snake_name.as_str()) {
        format!("{}_", snake_name)
    } else {
        snake_name
    }
}

fn safe_param_name(name: &str) -> String {
    // In Rust, function parameters are patterns, so tuple variant names from
    // the prelude (Ok, Err, Some, None) cannot be used as parameter names —
    // they conflict as enum variant patterns. Append '_' to avoid E0530.
    const RESERVED_PATTERNS: &[&str] = &["Ok", "Err", "Some", "None"];
    if RUST_KEYWORDS.contains(&name) || RESERVED_PATTERNS.contains(&name) {
        format!("{}_", name)
    } else {
        name.to_string()
    }
}

// ── Filtering predicates ────────────────────────────────────────────────────


fn param_uses_unknown_handle(ty: &Type, handle_able_classes: &HashSet<String>) -> bool {
    match ty {
        Type::Handle(class_name) => !handle_able_classes.contains(class_name),
        Type::ConstRef(inner) | Type::MutRef(inner) => {
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
```
</details>

> 💭 So `safe_method_name("New")` → `"New".to_snake_case()` → `"new"`. And constructors get `impl_method_name: "new"`.
> 
> Now, is `Storage_CallBack::New` a static method? Let me check what kind of method it is. The comment says "C++ `New()` factory method", so it's likely a static method that returns a Handle.
> 
> Let me look at the flow:
> 1. Constructor bindings are computed → "new" is added to `all_instance_method_names` (line 1709)
> 2. Static method `New` gets `safe_method_name("New")` = "new"
> 
> 
> 3. When checking static method names, Level 3 compares against `all_instance_method_names` — "new" is already there from constructors, so it would get renamed to "new_static" using the format string with the FFI function name base and "_static" suffix.
> 
> But I need to figure out what `overload_suffix()` returns for `New` based on its parameter types to understand the full renaming logic. Tracing through the collision logic at Level 3, when `ffi_fn_name_base` is "new" and it collides with an existing instance method name, the code appends "_static" to produce "new_static" as the final `impl_method_name`. But I'm noticing something about how `ffi_fn_name_base` flows through this logic...
> 
> Actually, let me reconsider the Level 2 check more carefully. The `reserved_names` set only includes wrapper methods, so if `New` is a static method rather than an instance method, it wouldn't be in that set at all, which means the collision detection might work differently than I initially thought. I'm realizing the issue now — `ffi_fn_name_base` is the FFI function name component (like "new"), but I need to check whether it actually matches any of the constructor impl method names that are stored in `all_instance_method_names`. The problem is that for a constructor with no overload, the FFI base becomes "new" in lowercase, and that should be found in the instance method names list which includes constructor names. and regenerating to see if that resolves it. If the exclusions weren't needed, the issue should disappear. But I should double-check whether `New` is actually parsed as a static method in the model, since that's the crux of the collision with the constructor wrapper.

Let me check how Storage_CallBack is parsed and whether `New` is a static method:

**Terminal:**
```sh
find /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src -name "Storage_CallBack.hxx" -o -name "Storage_DefaultCallBack.hxx" | head -5
```
```
/Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/Storage/Storage_DefaultCallBack.hxx
/Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/Storage/Storage_CallBack.hxx
```

**Terminal:**
```sh
grep -n "New\|DEFINE_STANDARD_RTTIEXT\|Standard_Transient\|class Storage_CallBack" /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/Storage/Storage_CallBack.hxx
```
<details>
<summary>Output (6 lines)</summary>

```
23:#include <Standard_Transient.hxx>
28:class Storage_CallBack;
29:DEFINE_STANDARD_HANDLE(Storage_CallBack, Standard_Transient)
31:class Storage_CallBack : public Standard_Transient
34:  Standard_EXPORT virtual Handle(Standard_Persistent) New() const = 0;
47:  DEFINE_STANDARD_RTTIEXT(Storage_CallBack, Standard_Transient)
```
</details>

> 💭 So `Storage_CallBack::New()` is a **virtual instance method** (not static!), and it's `const` and `= 0` (pure virtual). So `Storage_CallBack` is abstract.
> 
> If it's abstract, then it has no constructors, and `all_instance_method_names` won't contain "new" from a constructor. But then `New()` → `new` as a method name should also be fine since there's no constructor to collide with.
> 
> But wait — the class is abstract, so constructors are skipped. But the method `New()` is still parsed as an instance method. And when converted to Rust, `new` becomes an instance method on the class. 
> 
> Now I'm wondering if the re-export code is trying to generate a `fn new()` constructor somewhere anyway, which would conflict with the `New()` method. Let me look at `Storage_DefaultCallBack` to see if that's where the actual constructor gets generated.

**Terminal:**
```sh
grep -n "New\|DEFINE_STANDARD_RTTIEXT\|Standard_Transient\|class Storage_DefaultCallBack" /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/Storage/Storage_DefaultCallBack.hxx
```
```
27:class Storage_DefaultCallBack;
31:class Storage_DefaultCallBack : public Storage_CallBack
36:  Standard_EXPORT Handle(Standard_Persistent) New() const Standard_OVERRIDE;
49:  DEFINE_STANDARD_RTTIEXT(Storage_DefaultCallBack, Storage_CallBack)
```

> 💭 So `Storage_CallBack::New()` is a pure virtual (abstract) const instance method, and `Storage_DefaultCallBack::New()` is a concrete override.
> 
> `Storage_DefaultCallBack` is NOT abstract (it overrides `New()`), so it will get constructors. And `New()` → `new` will be an instance method. So the constructor `new()` and instance method `new()` will collide.
> 
> For `Storage_CallBack`, it's abstract (pure virtual `New()` = 0), so NO constructors are generated, and the instance method `new()` won't collide with anything.
> 
> So the real collision is only in `Storage_DefaultCallBack`. 
> 
> The bug is that the method naming logic disambiguates overloads among methods themselves, but never checks whether a method name conflicts with a constructor name like "new". Constructors get computed first and assigned their `impl_method_name`, then direct methods are named without considering what the constructors already claimed. The issue is that `needs_wrapper_function` checks for Handle return types, which means `New()` would be treated as a wrapper method. So the real problem is in `compute_wrapper_method_names` — I need to add a Pass 3 that checks wrapper method names against constructor names and renames collisions with a suffix. The cleaner approach would be to collect constructor names into a set upfront and check for collisions when computing wrapper method names around line 1691-1775.
> 
> I'm considering three options: passing constructor names as a reserved set into `compute_wrapper_method_names`, post-processing wrapper names afterward, or adding a collision-checking pass in `compute_class_bindings`. Option A seems cleanest since it prevents collisions at the source. I should also check `compute_direct_method_names` for the same issue, though direct methods like `New()` returning `Handle(Standard_Persistent)` would actually be wrappers, not direct methods.
> 
> Now I'm realizing the naming gets more complex—direct methods use CXX naming where the C++ name stays as-is, and the `rust_name` gets mapped via `#[cxx_name]`. I need to trace through how direct versus wrapper methods are actually emitted in the re-exports to understand where collisions could occur.
> 
> Looking at the re-export structure, both direct and wrapper methods end up in the same `impl` block as constructors, so if a constructor is named `new` and a method also resolves to `new`, they'll conflict in the same block.
> 
> The cleanest solution is to pre-populate the name tracking maps with constructor names before processing methods, so any collision automatically gets a numeric suffix appended. I'll apply this to both the direct and wrapper method naming functions.

Now I understand the issue. `Storage_CallBack::New()` is a virtual instance method (not static), and `Storage_DefaultCallBack::New()` overrides it. `New()` →  in snake_case, colliding with the constructor . The current disambiguation in `compute_wrapper_method_names` and  handles method-vs-method collisions but doesn't check for method-vs-constructor collisions.

Let me also check how inherited methods handle this:

Searched for regex `impl_method_name.*safe_method_name` (`**/bindings.rs`), no results

Searched for regex `safe_method_name.*inherited` (`**/bindings.rs`), no results

Searched for regex `inherited.*impl_method` (`**/bindings.rs`), no results

<details>
<summary>Reading lines 2740-2770 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2740-L2771">bindings.rs</a></summary>

```
                        false
                    }
                });
                if class_ptr_uses_unknown {
                    continue;
                }

                // Skip class pointer returns whose inner type is unknown.
                if let Some(ref rt) = resolved_method.return_type {
                    if let Some(class_name) = rt.original.class_ptr_inner_name() {
                        if !all_class_names.contains(class_name) && !all_enum_names.contains(class_name) {
                            continue;
                        }
                    }
                }

                let ffi_fn_name = format!(
                    "{}_inherited_{}",
                    class.name.replace("::", "_"), resolved_method.cpp_name
                );
                let impl_method_name =
                    safe_method_name(&resolved_method.cpp_name);

                let params: Vec<ResolvedParamBinding> = resolved_method
                    .params
                    .iter()
                    .map(|p| {
                        let is_nullable = p.is_nullable_ptr();

                        // Nullable pointer params: pass through as raw pointers
                        if is_nullable {
```
</details>

> 💭 So inherited methods also produce `impl_method_name = safe_method_name(&resolved_method.cpp_name)` without checking for constructor name collisions. But inherited methods are less likely to collide since they're from parent classes. The `New()` method IS inherited by `Storage_DefaultCallBack` from `Storage_CallBack`, so this is actually the path that matters.
> 
> Wait no — `Storage_DefaultCallBack` has `New()` as its own method (it overrides it). So it would be processed as its own method, not inherited.
> 
> Let me look at where inherited method names get deduped:

Let me see how inherited methods handle name collisions:

Searched for regex `inherited_methods.*dedup|inherited.*used_names|used_impl_names` (`**/bindings.rs`), no results

<details>
<summary>Reading lines 1900-1980 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1900-L1981">bindings.rs</a></summary>

```

    // ── Handle upcasts ──────────────────────────────────────────────────
    let handle_upcasts = if has_handle_get {
        compute_handle_upcast_bindings(class, symbol_table, handle_able_classes)
    } else {
        Vec::new()
    };
    // ── Handle downcasts ─────────────────────────────────────────────
    let handle_downcasts = if has_handle_get {
        compute_handle_downcast_bindings(class, symbol_table, handle_able_classes)
    } else {
        Vec::new()
    };

    // ── Inherited methods ───────────────────────────────────────────────
    let inherited_methods_raw =
        compute_inherited_method_bindings(class, symbol_table, handle_able_classes, ffi_ctx.all_classes, ffi_ctx.all_enums, ffi_ctx.deletable_class_names, reexport_ctx, exclude_methods);
    // Filter out inherited methods whose Rust name conflicts with a constructor or direct method
    let ctor_and_method_names: std::collections::HashSet<&str> = constructors
        .iter()
        .map(|c| c.impl_method_name.as_str())
        .chain(direct_methods.iter().map(|m| m.rust_name.as_str()))
        .chain(wrapper_methods.iter().map(|m| m.impl_method_name.as_str()))
        .chain(static_methods.iter().map(|m| m.impl_method_name.as_str()))
        .collect();
    let mut inherited_methods: Vec<InheritedMethodBinding> = inherited_methods_raw
        .into_iter()
        .filter(|im| !ctor_and_method_names.contains(im.impl_method_name.as_str()))
        .collect();
    // Dedup inherited methods against each other (different C++ names that
    // produce the same snake_case, e.g. GetChildLabel/getChildLabel).
    // Also dedup the FFI function names (C++ wrappers) to avoid link-time
    // collisions.
    {
        let mut seen_impl: HashMap<String, usize> = HashMap::new();
        let mut seen_ffi: HashMap<String, usize> = HashMap::new();
        for im in &mut inherited_methods {
            let impl_count = seen_impl.entry(im.impl_method_name.clone()).or_insert(0);
            *impl_count += 1;
            if *impl_count > 1 {
                im.impl_method_name = format!("{}_{}", im.impl_method_name, impl_count);
            }
            let ffi_count = seen_ffi.entry(im.ffi_fn_name.clone()).or_insert(0);
            *ffi_count += 1;
            if *ffi_count > 1 {
                im.ffi_fn_name = format!("{}_{}", im.ffi_fn_name, ffi_count);
            }
        }
    }
    // ── POD struct fields ────────────────────────────────────────────────
    let pod_fields = if class.is_pod_struct {
        compute_pod_field_bindings(&class.fields)
    } else {
        Vec::new()
    };

    ClassBindings {
        cpp_name: cpp_name.clone(),
        cpp_qualified_name: class.name.clone(),
        short_name: crate::type_mapping::safe_short_name(&crate::type_mapping::short_name_for_module(cpp_name, &class.module)),
        module: class.module.clone(),
        is_abstract: effectively_abstract,
        has_protected_destructor: class.has_protected_destructor,
        doc_comment: class.comment.clone(),
        source_header: class.source_header.clone(),
        source_line: class.source_line,
        constructors,
        direct_methods,
        wrapper_methods,
        static_methods,
        upcasts,
        has_to_owned,
        has_to_handle,
        has_handle_get,
        handle_upcasts,
        handle_downcasts,
        inherited_methods,
        is_pod_struct: class.is_pod_struct,
        pod_fields,
        skipped_symbols,
    }
```
</details>

> 💭 Now I see the full picture. Looking at lines 1917-1928:
> 
> ```rust
> // Filter out inherited methods whose Rust name conflicts with a constructor or direct method
> let ctor_and_method_names: std::collections::HashSet<&str> = constructors
>     .iter()
>     .map(|c| c.impl_method_name.as_str())
>     .chain(direct_methods.iter().map(|m| m.rust_name.as_str()))
>     .chain(wrapper_methods.iter().map(|m| m.impl_method_name.as_str()))
> 
> 
> ```
> 
> Building a HashSet of all constructor and method names...
> 
> The code then filters out inherited methods entirely if their names collide with any of these, which seems overly aggressive—they should probably be renamed instead of dropped. But the real problem is how this applies to own methods.
> 
> For direct and wrapper methods, there's no collision check against constructor names at all. The fix would involve passing constructor names as reserved identifiers to both `compute_wrapper_method_names` and `compute_direct_method_names`, pre-seeding their tracking maps so collisions automatically get suffixed with `_2`.
> 
> Though with direct methods, there's a complication—they're used in the CXX bridge where the Rust binding name needs to match the C++ method name, typically via `#[cxx_name]` attributes. I need to understand how that mapping actually works before deciding on the right approach.
> 
> ```
> Looking at how direct methods are emitted...
> ```

Let me check how direct methods are emitted in the CXX and re-export code:

Searched for regex `DirectMethodBinding` (`**/bindings.rs`), 4 results
- [bindings.rs:44](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L44)
- [bindings.rs:138](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L138)
- [bindings.rs:1654](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1654)
- [bindings.rs:1678](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1678)

<details>
<summary>Reading lines 138-160 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L138-L161">bindings.rs</a></summary>

```
pub struct DirectMethodBinding {
    /// Rust method name (snake_case, possibly with overload suffix)
    pub rust_name: String,
    /// Original C++ method name (for #[cxx_name])
    pub cxx_name: String,
    /// Whether this is a const method
    pub is_const: bool,
    /// Parameters
    pub params: Vec<ParamBinding>,
    /// Return type (None for void)
    pub return_type: Option<ReturnTypeBinding>,
    /// Doc comment
    pub doc_comment: Option<String>,
    /// Source line in C++ header
    pub source_line: Option<u32>,
    /// Whether this method should be marked `unsafe fn` (has raw pointer params/returns)
    pub is_unsafe: bool,
}

/// What kind of C++ wrapper is needed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WrapperKind {
    /// Returns a class or handle by value → new T(...) wrapper
```
</details>

> 💭 Good. So `DirectMethodBinding` has `rust_name` and `cxx_name`. The `rust_name` is used in CXX bridge declarations, and `cxx_name` is used in `#[cxx_name]` attribute. So changing `rust_name` doesn't affect the C++ binding, just the Rust name.
> 
> Now I need my plan:
> 
> 1. Modify `compute_direct_method_names` to accept a set of reserved names (constructor names) and pre-seed the `seen_names` map
> 2. Modify `compute_wrapper_method_names` similarly
> 3. Pass constructor `impl_method_name`s to both functions
> 
> 
> 4. Remove the `Storage_CallBack::New` and `Storage_DefaultCallBack::New` entries from `exclude_methods` in bindings.toml
> 5. Regenerate bindings and test
> 
> With the pre-seeding approach, when a method collides with a constructor, it gets renamed by incrementing a counter—so `new` becomes `new_2` if there's already a constructor named `new`. I also need to reconsider how inherited methods are handled since they're currently filtered out when they collide with constructors.
> 
> Looking at the specific case: `Storage_DefaultCallBack` overrides `New()`, so it has its own method that would collide with the constructor, while inherited `New()` methods from parent classes get filtered out. The real fix is ensuring that both direct and wrapper method naming accounts for constructor names.

Now I have a clear picture. The fix is to make  and `compute_wrapper_method_names` aware of constructor names, so method names like  (from C++ `New()`) get renamed to `new_2` when there's a constructor .

**Todo list:**
- ✅ Understand collision disambiguation code
- 🔄 Fix disambiguation for constructor+method collisions
- ⬜ Remove exclude_methods entries
- ⬜ Regenerate bindings and test
- ⬜ Commit changes

Let me see the current function signatures and how they're called:Now let me implement the fix. I need to:

1. Add a  parameter to both  and `compute_wrapper_method_names`
2. Pre-seed the "seen" maps with constructor names
3. Update the call sites to pass constructor names

<details>
<summary>Reading lines 1274-1280 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1274-L1281">bindings.rs</a></summary>

```
/// Compute overload suffix with const/mut disambiguation for direct methods.
/// Returns (rust_name, suffix_used) for each method in the list.
fn compute_direct_method_names(methods: &[&Method]) -> Vec<String> {
    let mut name_counts: HashMap<String, usize> = HashMap::new();
    for method in methods {
        *name_counts.entry(method.name.clone()).or_insert(0) += 1;
    }
```
</details>

Now let me implement the changes:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1274-L1279">bindings.rs -3+5</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1284-L1289">-1+5</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1274:uq,1275:sj,1276:sp",
      "content": "/// Compute overload suffix with const/mut disambiguation for direct methods.\n/// Returns (rust_name, suffix_used) for each method in the list.\n/// `constructor_names` contains the impl_method_names of constructors (e.g. \"new\", \"new_2\")\n/// so that methods whose snake_case name collides with a constructor get a suffix.\nfn compute_direct_method_names(methods: &[&Method], constructor_names: &HashSet<String>) -> Vec<String> {"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1282:ss",
      "content": "    let mut seen_names: HashMap<String, usize> = HashMap::new();\n    // Pre-seed with constructor names so methods colliding with them get _2 suffix\n    for name in constructor_names {\n        seen_names.insert(name.clone(), 1);\n    }"
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
      "startLine": 1274,
      "endLine": 1288
    }
  ]
}
```
</details>

Now update `compute_wrapper_method_names`:

<details>
<summary>Reading lines 1332-1370 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1332-L1371">bindings.rs</a></summary>

```
        .collect()
}

/// Compute overload suffix with const/mut disambiguation for wrapper methods.
/// Returns the base fn_name (without class prefix) for each method.
fn compute_wrapper_method_names(methods: &[&Method]) -> Vec<String> {
    let mut name_counts: HashMap<String, usize> = HashMap::new();
    for method in methods {
        *name_counts.entry(method.name.clone()).or_insert(0) += 1;
    }

    // Pass 1: resolve C++ overloads (same C++ name, different params)
    let mut names: Vec<String> = methods
        .iter()
        .map(|method| {
            let base_name = safe_method_name(&method.name);
            let needs_suffix = name_counts.get(&method.name).copied().unwrap_or(0) > 1;
            if needs_suffix {
                let base_suffix = method.overload_suffix();
                let same_suffix_diff_const = methods.iter().any(|m| {
                    m.name == method.name
                        && m.overload_suffix() == base_suffix
                        && m.is_const != method.is_const
                });
                let suffix = if same_suffix_diff_const && !method.is_const {
                    format!("{}_mut", base_suffix)
                } else {
                    base_suffix
                };
                combine_name_suffix(&base_name, &suffix)
            } else {
                base_name
            }
        })
        .collect();

    // Pass 2: resolve cross-name collisions (different C++ names that produce
    // the same snake_case name, e.g. SetInteger/setInteger → set_integer).
    // Append _2, _3, ... to later duplicates.
```
</details>

<details>
<summary>Reading lines 1370-1385 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1370-L1386">bindings.rs</a></summary>

```
    // Append _2, _3, ... to later duplicates.
    let mut seen: HashMap<String, usize> = HashMap::new();
    for name in &mut names {
        let count = seen.entry(name.clone()).or_insert(0);
        *count += 1;
        if *count > 1 {
            *name = format!("{}_{}", name, count);
        }
    }

    names
}

/// Compute static method names with 3-level conflict resolution.
/// Returns (ffi_fn_name_base, impl_method_name) for each method.
fn compute_static_method_names(
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1335-L1340">bindings.rs -3+5</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1372-L1378">-2+6</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "/// Compute overload suffix with const/mut disambiguation for wrapper methods.\n/// Returns the base fn_name (without class prefix) for each method.\n/// `constructor_names` contains the impl_method_names of constructors (e.g. \"new\", \"new_2\")\n/// so that methods whose snake_case name collides with a constructor get a suffix.\nfn compute_wrapper_method_names(methods: &[&Method], constructor_names: &HashSet<String>) -> Vec<String> {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1335:wm,1336:js,1337:aa"
    },
    {
      "content": "    // Append _2, _3, ... to later duplicates.\n    // Pre-seed with constructor names so methods colliding with them get _2 suffix.\n    let mut seen: HashMap<String, usize> = HashMap::new();\n    for name in constructor_names {\n        seen.insert(name.clone(), 1);\n    }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1370:le,1371:xs"
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
      "startLine": 1335,
      "endLine": 1377
    }
  ]
}
```
</details>

Now update the call sites. Let me find and update where these functions are called:

<details>
<summary>Reading lines 1655-1706 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1655-L1707">bindings.rs</a></summary>

```
    // Partition into direct vs wrapper
    let direct_methods_raw: Vec<&Method> = bindable_methods.iter()
        .filter(|m| !needs_wrapper_function(m, all_enum_names))
        .copied()
        .collect();
    let wrapper_methods_raw: Vec<&Method> = bindable_methods.iter()
        .filter(|m| needs_wrapper_function(m, all_enum_names))
        .copied()
        .collect();

    let direct_method_names = compute_direct_method_names(&direct_methods_raw);
    let direct_methods: Vec<DirectMethodBinding> = direct_methods_raw
        .iter()
        .zip(direct_method_names.iter())
        .map(|(method, rust_name)| {
            let params: Vec<ParamBinding> = method
                .params
                .iter()
                .map(|p| build_param_binding(&p.name, &p.ty, p.is_nullable_ptr(), ffi_ctx, reexport_ctx))
                .collect();
            let mut return_type = method
                .return_type
                .as_ref()
                .map(|ty| build_return_type_binding(ty, ffi_ctx, reexport_ctx));

            // If the method is const (&self) and returns a class pointer,
            // downgrade Option<&mut T> to Option<&T> to avoid unsound &self -> &mut T.
            if method.is_const {
                if let Some(ref mut rt) = return_type {
                    if rt.is_class_ptr_return && rt.rust_reexport_type.starts_with("Option<&mut ") {
                        rt.rust_reexport_type = rt.rust_reexport_type.replace("Option<&mut ", "Option<&");
                    }
                }
            }

            DirectMethodBinding {
                rust_name: rust_name.clone(),
                cxx_name: method.name.clone(),
                is_const: method.is_const,
                params,
                return_type,
                doc_comment: method.comment.clone(),
                source_line: method.source_line,
                is_unsafe: method.has_unsafe_types(),
            }
        })
        .collect();

    let wrapper_fn_names = compute_wrapper_method_names(&wrapper_methods_raw);

    // Build reserved_names set for static method conflict detection
    let mut reserved_names: HashSet<String> = HashSet::new();
```
</details>

Now I need to build a set of constructor names before computing method names:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1665-L1673">bindings.rs -1+8</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1710-L1711">-1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    // Build set of constructor impl_method_names so that method name disambiguation\n    // can avoid collisions (e.g. C++ `New()` → `new` colliding with constructor `new()`).\n    let constructor_names: HashSet<String> = constructors\n        .iter()\n        .map(|c| c.impl_method_name.clone())\n        .collect();\n\n    let direct_method_names = compute_direct_method_names(&direct_methods_raw, &constructor_names);",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1665:ot"
    },
    {
      "content": "    let wrapper_fn_names = compute_wrapper_method_names(&wrapper_methods_raw, &constructor_names);",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1703:zt"
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
      "startLine": 1665,
      "endLine": 1710
    }
  ]
}
```
</details>

Now let me check if there are any tests that call these functions directly:

<details>
<summary>Searched for regex <code>compute_direct_method_names\(|compute_wrapper_method_names\(</code> (<code>**/bindings.rs</code>), 5 results</summary>

- [bindings.rs:1278](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1278)
- [bindings.rs:1339](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1339)
- [bindings.rs:1672](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1672)
- [bindings.rs:1710](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1710)
- [bindings.rs:5765](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5765)
</details>

There's a test call at line 5765. Let me fix it:

<details>
<summary>Reading lines 5750-5810 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5750-L5811">bindings.rs</a></summary>

```
                name: "Mirror".to_string(),
                comment: None,
                is_const: false,
                params: vec![Param {
                    name: "A1".to_string(),
                    ty: Type::ConstRef(Box::new(Type::Class("gp_Ax1".to_string()))),
                    has_default: false,
                    default_value: None,
                }],
                return_type: None,
                source_line: Some(20),
            },
        ];

        let method_refs: Vec<&Method> = methods.iter().collect();
        let names = compute_wrapper_method_names(&method_refs);

        // Should get different suffixes based on param types
        assert_ne!(names[0], names[1]);
        assert!(names[0].starts_with("mirror"));
        assert!(names[1].starts_with("mirror"));
    }

    /// Test static method 3-level conflict resolution
    #[test]
    fn test_static_method_conflict_resolution() {
        use crate::model::{StaticMethod, Type};

        let methods = vec![StaticMethod {
            name: "Origin".to_string(),
            comment: None,
            params: Vec::new(),
            return_type: Some(Type::Class("gp_Pnt".to_string())),
            source_line: Some(10),
        }];

        let method_refs: Vec<&StaticMethod> = methods.iter().collect();

        // Case 1: No conflicts
        let reserved: HashSet<String> = HashSet::new();
        let instance: HashSet<String> = HashSet::new();
        let names =
            compute_static_method_names("gp_Pnt", &method_refs, &reserved, &instance);
        assert_eq!(names[0].0, "origin"); // ffi base
        assert_eq!(names[0].1, "origin"); // impl name

        // Case 2: Conflict with wrapper reserved name
        let reserved: HashSet<String> = ["gp_Pnt_origin".to_string()].into();
        let instance: HashSet<String> = HashSet::new();
        let names =
            compute_static_method_names("gp_Pnt", &method_refs, &reserved, &instance);
        assert_eq!(names[0].0, "origin_static"); // ffi base adds _static
        assert_eq!(names[0].1, "origin_static"); // impl follows

        // Case 3: Conflict with instance method names
        let reserved: HashSet<String> = HashSet::new();
        let instance: HashSet<String> = ["origin".to_string()].into();
        let names =
            compute_static_method_names("gp_Pnt", &method_refs, &reserved, &instance);
        assert_eq!(names[0].0, "origin"); // ffi base stays
        assert_eq!(names[0].1, "origin_static"); // impl adds _static
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5765-L5766">bindings.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        let names = compute_wrapper_method_names(&method_refs, &HashSet::new());",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "5765:xu"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":5765,"endLine":5765}
```
</details>

Now let me verify the code compiles:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo check -p opencascade-binding-generator 2>&1
```

**Todo list:**
- ✅ Understand collision disambiguation code
- ✅ Fix disambiguation for constructor+method collisions
- 🔄 Remove exclude_methods entries
- ⬜ Regenerate bindings and test
- ⬜ Commit changes

Now remove the exclude_methods entries from bindings.toml:

<details>
<summary>Reading lines 76-96 of <a href="../crates/opencascade-sys/bindings.toml#L76-L97">bindings.toml</a></summary>

```

# Exclude specific methods/constructors that can't be bound. These methods are
# excluded individually so the rest of each class remains accessible.
# Format: "ClassName::MethodName" for methods, "ClassName::ClassName" for constructors.
exclude_methods = [
    # C++ `New()` factory method name-collides with constructor wrapper `fn new()`
    "Storage_CallBack::New",
    "Storage_DefaultCallBack::New",

    # These take a const char *&, and advance the pointer after parsing.
    "XmlObjMgt::GetInteger",
    "XmlObjMgt::GetReal",

    # Protected/hidden operator new — skip constructors but keep methods accessible.
    "Message_LazyProgressScope::Message_LazyProgressScope",
    "Poly_CoherentTriPtr::Poly_CoherentTriPtr",
]

# Non-allocatable classes: suppress constructors and destructors (including
# CppDeletable/ToOwned) but keep the opaque struct and all methods.
# Use for classes with protected/hidden operator new/delete, forward-declared
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-sys/bindings.toml#L80-L84">bindings.toml -7+4</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "exclude_methods = [\n    # These take a const char *&, and advance the pointer after parsing.\n    \"XmlObjMgt::GetInteger\",\n    \"XmlObjMgt::GetReal\",",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "80:aj,81:ka,82:so,83:ff,84:dd,85:ni,86:vc"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":80,"endLine":83}
```
</details>

**Todo list:**
- ✅ Understand collision disambiguation code
- ✅ Fix disambiguation for constructor+method collisions
- ✅ Remove exclude_methods entries
- 🔄 Regenerate bindings and test
- ⬜ Commit changes

Now regenerate bindings and build:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && tail -5 /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/0c601d13-57cb-4b43-8289-968929687961/toolu_01HyYKKkzGEg5jTm8rKtRkhX__vscode-1771608612637/content.txt
```
<details>
<summary>Output (5 lines)</summary>

```
Code generation complete!
  321 modules generated

Bindings generated in /Users/sethml/src/opencascade-rs/crates/opencascade-sys/ge
nerated                                                                         real  48.7s
```
</details>

Let me verify the generated code for Storage_DefaultCallBack to see that `New()` got renamed properly:

**Terminal:**
```sh
grep -n "new\|New" /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/storage.rs | grep -i "storage_default\|storage_call_back"
```
```
1749:    /// **Source:** `Storage_DefaultCallBack.hxx`:36 - `Storage_DefaultCallBack::New()`
1752:            crate::OwnedPtr::from_raw(crate::ffi::Storage_DefaultCallBack_new_2(
```

The `New()` method got renamed to `new_2` rather than being excluded. Let me also check the wrapper declaration:

**Terminal:**
```sh
grep -B2 -A5 "new_2\|Storage_DefaultCallBack_New\|Storage_CallBack_New" /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/storage.rs
```
<details>
<summary>Output (9 lines)</summary>

```

    /// **Source:** `Storage_DefaultCallBack.hxx`:36 - `Storage_DefaultCallBack::New()`
    pub fn new_2(&self) -> crate::OwnedPtr<crate::ffi::HandleStandardPersistent> {
        unsafe {
            crate::OwnedPtr::from_raw(crate::ffi::Storage_DefaultCallBack_new_2(
                self as *const Self,
            ))
        }
    }
```
</details>

The disambiguation is working correctly. Now let's build everything:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

Build succeeded. Now run the binding generator tests:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (37 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.51s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-3404803fe4cd90de)

running 16 tests
test header_deps::tests::test_is_likely_occt_header ... ok
test module_graph::tests::test_extract_rust_type_name ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test parser::tests::test_extract_module_from_header ... ok
test parser::tests::test_extract_template_arg ... ok
test parser::tests::test_map_standard_type ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
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

real  6.4s
```
</details>

All 16 tests pass. Now run the workspace tests:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo test --workspace --exclude opencascade-binding-generator 2>&1
```

**Terminal:**
```sh
grep "test result:" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/0c601d13-57cb-4b43-8289-968929687961/toolu_01KXnaRQGvFWdbACd3Ttjdet__vscode-1771608612643/content.txt
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

All tests pass. Now let me commit:

**Todo list:**
- ✅ Understand collision disambiguation code
- ✅ Fix disambiguation for constructor+method collisions
- ✅ Remove exclude_methods entries
- ✅ Regenerate bindings and test
- 🔄 Commit changes

Created [commit-msg.txt](../tmp/commit-msg.txt)

**Terminal:**
```sh
rm /Users/sethml/src/opencascade-rs/tmp/commit-msg.txt
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
[main b3a6990] Fix method-vs-constructor name collision disambiguation
 5 files changed, 55 insertions(+), 26 deletions(-)
```

**Todo list:**
- ✅ Understand collision disambiguation code
- ✅ Fix disambiguation for constructor+method collisions
- ✅ Remove exclude_methods entries
- ✅ Regenerate bindings and test
- ✅ Commit changes

Done. The fix pre-seeds the "seen names" maps in  and  with constructor s. Now when a C++ method like `New()` maps to snake_case  and a constructor already claims that name, the dedup logic automatically produces `new_2`. 

`Storage_DefaultCallBack::New()` now generates `fn new_2(&self)` instead of being excluded entirely. Removed both `Storage_CallBack::New` and `Storage_DefaultCallBack::New` from `exclude_methods` in bindings.toml.

*2026-02-21 07:06 · 681s · Words: 4,429 in · 3,122 out · 64,353 ctx · 53 rounds*

---

<a id="user-2"></a>

## User (2)

> Explain this binding. In C++, Storage_DefaultCallback::New() returns a handle&lt;Standard_Persistent&gt;. The wrapper appears to construct a new Handle around the existing one - why?

*2026-02-21 07:38*

### Assistant — Claude Opus 4.6

> 💭 The user is asking about the generated wrapper for `Storage_DefaultCallBack::New()`. Let me look at the specific lines they're viewing and understand the wrapper pattern.

<details>
<summary>Reading lines 400575-400590 of <a href="../crates/opencascade-sys/generated/wrappers.cpp#L400575-L400591">wrappers.cpp</a></summary>

```
// ========================

extern "C" Storage_DefaultCallBack* Storage_DefaultCallBack_ctor() {
    return new Storage_DefaultCallBack();
}
extern "C" Handle(Standard_Persistent)* Storage_DefaultCallBack_new_2(const Storage_DefaultCallBack* self_) {
    return new Handle(Standard_Persistent)(self_->New());
}
extern "C" void Storage_DefaultCallBack_add(const Storage_DefaultCallBack* self_, const Handle(Standard_Persistent)* thePers, const Handle(Storage_Schema)* theSchema) {
    self_->Add(*thePers, *theSchema);
}
extern "C" void Storage_DefaultCallBack_write(const Storage_DefaultCallBack* self_, const Handle(Standard_Persistent)* thePers, const Handle(Storage_BaseDriver)* theDriver, const Handle(Storage_Schema)* theSchema) {
    self_->Write(*thePers, *theDriver, *theSchema);
}
extern "C" void Storage_DefaultCallBack_read(const Storage_DefaultCallBack* self_, const Handle(Standard_Persistent)* thePers, const Handle(Storage_BaseDriver)* theDriver, const Handle(Storage_Schema)* theSchema) {
    self_->Read(*thePers, *theDriver, *theSchema);
```
</details>

The C++ method `New()` returns a  **by value** (a stack-local smart pointer). The FFI boundary is `extern "C"`, which can only pass pointers — not C++ objects by value.

So the wrapper does  which:

1. Calls `self_->New()` — returns a  by value (temporary on the stack)
2.  — heap-allocates a new  object, copy/move-constructing it from the temporary

This is the standard pattern the generator uses for any C++ method that returns a class/handle by value: it heap-allocates the return value so it can be passed across the FFI boundary as a raw pointer. On the Rust side, the `*mut Handle<Standard_Persistent>` is wrapped in an `OwnedPtr` which will call `delete` when dropped.

The "new Handle around the existing one" isn't wrapping a handle in another handle — it's **heap-allocating** the same handle so it can cross the `extern "C"` boundary as a pointer. The inner  and outer  are the same type;  is just C++ for "heap-allocate a `T` initialized from `value`".

*2026-02-21 07:38 · 39s · Words: 106 in · 190 out · 60,413 ctx · 2 rounds*

---
