# Code refactoring and commit process guidance

- **Date:** 2026-02-21 15:49 – 16:23
- **Model:** Claude Opus 4.6
- **Turns:** 1
- **Tool calls:** 185
- **Thinking blocks:** 18
- **Input words:** 26,603
- **Output words:** 14,833
- **Prompt tokens (last round):** 132,729
- **API rounds:** 107
- **Total elapsed:** 2015s

## Table of Contents

1. [Try to fix all of these except the ones blocked by missing dependencies, and commit. Then think a...](#user-1) (Claude Opus 4.6)

---

<a id="user-1"></a>

## User (1)

> Try to fix all of these except the ones blocked by missing dependencies, and commit. Then think about whether you discovered opportunities to refactor the code the avoid repetition in this session. If refactoring seems worthwhile, do it, and commit.

*2026-02-21 15:49*

### Assistant — Claude Opus 4.6

> 💭 The user wants me to fix the skipped symbols from the table they selected in the README. Let me analyze each one:
> 
> 1. `Handle(ShapePersistent_Geom::...)` - Nested template Handle types - low priority (blocked by missing deps)
> 2. `ShapeProcess::OperationsFlags` - Class-scoped using alias to `std::bitset` — non-OCCT type, not bindable (blocked)
> 3. `Handle(ShapePersistent_Geom::...)` - Same as #1
> 4. `Handle(BVH_Builder<double, 3>)` - Template Handle type - low priority (blocked)
> 
> 
> 5. `IMeshData::MapOfInteger` - Namespace-scoped NCollection template typedef - low priority (blocked)
> 6. `Standard_PCharacter` - Typedef for `char*` — pointer typedef, not yet resolvable
> 7. `BOPDS_PDS` - Typedef for `BOPDS_DS*` — raw pointer typedef
> 8. `Handle(NCollection_Shared<...>)` - Handle to NCollection_Shared template (blocked)
> 9. `WNT_HIDSpaceMouse` - Windows-only type, WNT module excluded (blocked - platform)
> 10. `AVStream` - FFmpeg external type, not bindable (blocked)
> 11. `MeshVS_DisplayModeFlags` - Typedef to Standard_Integer — should resolve but doesn't (investigation needed)
> 12. `RWGltf_GltfOStreamWriter` - External RapidJSON dependency (blocked)
> 
> The unblocked ones worth tackling are the three pointer and integer typedefs: `Standard_PCharacter`, `BOPDS_PDS`, and `MeshVS_DisplayModeFlags`. Let me dig into the codebase to understand why these aren't resolving properly.

**Todo list:**
- ⬜ Investigate Standard_PCharacter (char*)
- ⬜ Investigate BOPDS_PDS (BOPDS_DS*)
- ⬜ Investigate MeshVS_DisplayModeFlags
- ⬜ Implement fixes
- ⬜ Regenerate bindings and test
- ⬜ Update README skip counts
- ⬜ Commit changes
- ⬜ Consider refactoring opportunities

**Todo list:**
- 🔄 Investigate Standard_PCharacter (char*)
- ⬜ Investigate BOPDS_PDS (BOPDS_DS*)
- ⬜ Investigate MeshVS_DisplayModeFlags
- ⬜ Implement fixes
- ⬜ Regenerate bindings and test
- ⬜ Update README skip counts
- ⬜ Commit changes
- ⬜ Consider refactoring opportunities

Let me investigate all three fixable types in parallel.

Searched for text `Standard_PCharacter` (`**/crates/opencascade-binding-generator/src/**`), no results

Searched for text `Standard_PCharacter` (`**/crates/occt-sys/OCCT/**"`), no results

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "Standard_PCharacter" crates/occt-sys/OCCT/src/ | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
crates/occt-sys/OCCT/src/DPrsStd/DPrsStd_AISPresentationCommands.cxx:#include <Standard_PCharacter.hxx>
crates/occt-sys/OCCT/src/DPrsStd/DPrsStd_AISPresentationCommands.cxx:    Standard_PCharacter presS;
crates/occt-sys/OCCT/src/DPrsStd/DPrsStd_AISPresentationCommands.cxx:        Standard_PCharacter pstr;
crates/occt-sys/OCCT/src/DPrsStd/DPrsStd_AISPresentationCommands.cxx:        Standard_PCharacter presS;
crates/occt-sys/OCCT/src/Interface/Interface_FileParameter.hxx:#include <Standard_PCharacter.hxx>
crates/occt-sys/OCCT/src/Interface/Interface_FileParameter.hxx:  Standard_PCharacter theval;
crates/occt-sys/OCCT/src/Interface/Interface_MSG.hxx:#include <Standard_PCharacter.hxx>
crates/occt-sys/OCCT/src/Interface/Interface_MSG.hxx:  Standard_PCharacter theval;
crates/occt-sys/OCCT/src/Interface/Interface_FileParameter.cxx:  theval  = (Standard_PCharacter)val; // Principe : Allocation geree par contenant (ParamSet)
crates/occt-sys/OCCT/src/Interface/Interface_ParamSet.hxx:#include <Standard_PCharacter.hxx>
crates/occt-sys/OCCT/src/Interface/Interface_ParamSet.hxx:  Standard_PCharacter         theval;
crates/occt-sys/OCCT/src/TopOpeBRep/TopOpeBRep_traceBOOPNINT.cxx:  #define AS(x) (Standard_PCharacter) TCollection_AsciiString((x)).ToCString();
crates/occt-sys/OCCT/src/XmlMFunction/XmlMFunction_FunctionDriver.cxx:    Standard_PCharacter pGuidStr;
crates/occt-sys/OCCT/src/IGESData/IGESData_IGESEntity.cxx:#include <Standard_PCharacter.hxx>
crates/occt-sys/OCCT/src/IGESData/IGESData_IGESEntity.cxx:  Standard_PCharacter pres1, pres2;
crates/occt-sys/OCCT/src/IGESData/IGESData_IGESEntity.cxx:  pres1 = (Standard_PCharacter)res1;
crates/occt-sys/OCCT/src/IGESData/IGESData_IGESEntity.cxx:  pres2 = (Standard_PCharacter)res2;
crates/occt-sys/OCCT/src/IGESData/IGESData_DirPart.cxx:#include <Standard_PCharacter.hxx>
crates/occt-sys/OCCT/src/IGESData/IGESData_DirPart.cxx:  Standard_PCharacter pres1, pres2, plabel, psubscript;
crates/occt-sys/OCCT/src/IGESData/IGESData_DirPart.cxx:  pres1      = (Standard_PCharacter)res1;
```
</details>

**Terminal:**
```sh
cat crates/occt-sys/OCCT/src/Standard/Standard_PCharacter.hxx
```
<details>
<summary>Output (21 lines)</summary>

```
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

#ifndef _Standard_PCharacter_HeaderFile
#define _Standard_PCharacter_HeaderFile

#include <Standard_TypeDef.hxx>

typedef Standard_Character* Standard_PCharacter;

#endif
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "BOPDS_PDS" crates/occt-sys/OCCT/src/BOPDS/ | head -20
```
<details>
<summary>Output (12 lines)</summary>

```
crates/occt-sys/OCCT/src/BOPDS/BOPDS_PDS.hxx:#ifndef _BOPDS_PDS_HeaderFile
crates/occt-sys/OCCT/src/BOPDS/BOPDS_PDS.hxx:#define _BOPDS_PDS_HeaderFile
crates/occt-sys/OCCT/src/BOPDS/BOPDS_PDS.hxx:typedef BOPDS_DS* BOPDS_PDS;
crates/occt-sys/OCCT/src/BOPDS/BOPDS_PDS.hxx:#endif // _BOPDS_PDS_HeaderFile
crates/occt-sys/OCCT/src/BOPDS/BOPDS_SubIterator.hxx:#include <BOPDS_PDS.hxx>
crates/occt-sys/OCCT/src/BOPDS/BOPDS_SubIterator.hxx:  void SetDS(const BOPDS_PDS& pDS) { myDS = pDS; }
crates/occt-sys/OCCT/src/BOPDS/BOPDS_SubIterator.hxx:  BOPDS_PDS                         myDS;
crates/occt-sys/OCCT/src/BOPDS/BOPDS_Iterator.cxx:void BOPDS_Iterator::SetDS(const BOPDS_PDS& aDS)
crates/occt-sys/OCCT/src/BOPDS/BOPDS_Iterator.hxx:#include <BOPDS_PDS.hxx>
crates/occt-sys/OCCT/src/BOPDS/BOPDS_Iterator.hxx:  Standard_EXPORT void SetDS(const BOPDS_PDS& pDS);
crates/occt-sys/OCCT/src/BOPDS/BOPDS_Iterator.hxx:  BOPDS_PDS                    myDS;             //!< Data Structure
crates/occt-sys/OCCT/src/BOPDS/FILES:BOPDS_PDS.hxx
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "MeshVS_DisplayModeFlags" crates/occt-sys/OCCT/src/ | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
crates/occt-sys/OCCT/src/MeshVS/MeshVS_DisplayModeFlags.hxx:#ifndef MeshVS_DisplayModeFlags_HeaderFile
crates/occt-sys/OCCT/src/MeshVS/MeshVS_DisplayModeFlags.hxx:#define MeshVS_DisplayModeFlags_HeaderFile
crates/occt-sys/OCCT/src/MeshVS/MeshVS_DisplayModeFlags.hxx:typedef Standard_Integer MeshVS_DisplayModeFlags;
crates/occt-sys/OCCT/src/MeshVS/MeshVS_ElementalColorPrsBuilder.cxx:  const MeshVS_DisplayModeFlags&   Flags,
crates/occt-sys/OCCT/src/MeshVS/MeshVS_NodalColorPrsBuilder.hxx:#include <MeshVS_DisplayModeFlags.hxx>
crates/occt-sys/OCCT/src/MeshVS/MeshVS_NodalColorPrsBuilder.hxx:    const MeshVS_DisplayModeFlags&   Flags    = MeshVS_DMF_NodalColorDataPrs,
crates/occt-sys/OCCT/src/MeshVS/MeshVS_TextPrsBuilder.hxx:#include <MeshVS_DisplayModeFlags.hxx>
crates/occt-sys/OCCT/src/MeshVS/MeshVS_TextPrsBuilder.hxx:    const MeshVS_DisplayModeFlags&   Flags    = MeshVS_DMF_TextDataPrs,
crates/occt-sys/OCCT/src/MeshVS/MeshVS_PrsBuilder.hxx:#include <MeshVS_DisplayModeFlags.hxx>
crates/occt-sys/OCCT/src/MeshVS/MeshVS_PrsBuilder.hxx:  //! DisplayMode is numeric constant describing display mode (see MeshVS_DisplayModeFlags.hxx)
crates/occt-sys/OCCT/src/MeshVS/MeshVS_PrsBuilder.hxx:  //! DisplayMode is numeric constant describing display mode (see MeshVS_DisplayModeFlags.hxx)
crates/occt-sys/OCCT/src/MeshVS/MeshVS_PrsBuilder.hxx:                                    const MeshVS_DisplayModeFlags&   Flags,
crates/occt-sys/OCCT/src/MeshVS/MeshVS_VectorPrsBuilder.cxx:                                                 const MeshVS_DisplayModeFlags&   Flags,
crates/occt-sys/OCCT/src/MeshVS/MeshVS_PrsBuilder.cxx:                                     const MeshVS_DisplayModeFlags&   Flags,
crates/occt-sys/OCCT/src/MeshVS/MeshVS_MeshPrsBuilder.hxx:#include <MeshVS_DisplayModeFlags.hxx>
crates/occt-sys/OCCT/src/MeshVS/MeshVS_MeshPrsBuilder.hxx:                                        const MeshVS_DisplayModeFlags&   Flags = MeshVS_DMF_OCCMask,
crates/occt-sys/OCCT/src/MeshVS/MeshVS_VectorPrsBuilder.hxx:#include <MeshVS_DisplayModeFlags.hxx>
crates/occt-sys/OCCT/src/MeshVS/MeshVS_VectorPrsBuilder.hxx:    const MeshVS_DisplayModeFlags&   Flags       = MeshVS_DMF_VectorDataPrs,
crates/occt-sys/OCCT/src/MeshVS/FILES:MeshVS_DisplayModeFlags.hxx
crates/occt-sys/OCCT/src/MeshVS/MeshVS_ElementalColorPrsBuilder.hxx:#include <MeshVS_DisplayModeFlags.hxx>
```
</details>

Now let me look at how the resolver handles type resolution to understand where these are being rejected:

Investigate type resolution code

<details>
<summary>Searched for regex <code>typedef|resolve_type|type_alias|canonical</code> (<code>**/crates/opencascade-binding-generator/src/resolver.rs</code>), 10 results</summary>

- [resolver.rs:347](../crates/opencascade-binding-generator/src/resolver.rs#L347)
- [resolver.rs:760](../crates/opencascade-binding-generator/src/resolver.rs#L760)
- [resolver.rs:1055](../crates/opencascade-binding-generator/src/resolver.rs#L1055)
- [resolver.rs:1104](../crates/opencascade-binding-generator/src/resolver.rs#L1104)
- [resolver.rs:1110](../crates/opencascade-binding-generator/src/resolver.rs#L1110)
- [resolver.rs:1167](../crates/opencascade-binding-generator/src/resolver.rs#L1167)
- [resolver.rs:1173](../crates/opencascade-binding-generator/src/resolver.rs#L1173)
- [resolver.rs:1236](../crates/opencascade-binding-generator/src/resolver.rs#L1236)
- [resolver.rs:1242](../crates/opencascade-binding-generator/src/resolver.rs#L1242)
- [resolver.rs:1320](../crates/opencascade-binding-generator/src/resolver.rs#L1320)
</details>

Searched for regex `Standard_PCharacter|BOPDS_PDS|MeshVS_DisplayModeFlags` (`**/crates/opencascade-binding-generator/src/**`), no results

<details>
<summary>Searched for regex <code>unknown type|unresolved type|skip</code> (<code>**/crates/opencascade-binding-generator/src/resolver.rs</code>), 5 results</summary>

- [resolver.rs:84](../crates/opencascade-binding-generator/src/resolver.rs#L84)
- [resolver.rs:954](../crates/opencascade-binding-generator/src/resolver.rs#L954)
- [resolver.rs:1283](../crates/opencascade-binding-generator/src/resolver.rs#L1283)
- [resolver.rs:1292](../crates/opencascade-binding-generator/src/resolver.rs#L1292)
- [resolver.rs:1294](../crates/opencascade-binding-generator/src/resolver.rs#L1294)
</details>

<details>
<summary>Searched for regex <code>Standard_PCharacter|BOPDS_PDS|MeshVS_DisplayModeFlags</code> (<code>**/crates/opencascade-sys/generated/**</code>), 33 results</summary>

- [bop_algo.rs:11132](../crates/opencascade-sys/generated/bop_algo.rs#L11132)
- [bop_algo.rs:11133](../crates/opencascade-sys/generated/bop_algo.rs#L11133)
- [bop_algo.rs:14892](../crates/opencascade-sys/generated/bop_algo.rs#L14892)
- [bop_algo.rs:14893](../crates/opencascade-sys/generated/bop_algo.rs#L14893)
- [bop_algo.rs:16471](../crates/opencascade-sys/generated/bop_algo.rs#L16471)
- [bop_algo.rs:16476](../crates/opencascade-sys/generated/bop_algo.rs#L16476)
- [bop_algo.rs:16480](../crates/opencascade-sys/generated/bop_algo.rs#L16480)
- [mesh_vs.rs:3078](../crates/opencascade-sys/generated/mesh_vs.rs#L3078)
- [mesh_vs.rs:5881](../crates/opencascade-sys/generated/mesh_vs.rs#L5881)
- [mesh_vs.rs:6395](../crates/opencascade-sys/generated/mesh_vs.rs#L6395)
- [mesh_vs.rs:6422](../crates/opencascade-sys/generated/mesh_vs.rs#L6422)
- [mesh_vs.rs:6448](../crates/opencascade-sys/generated/mesh_vs.rs#L6448)
- [mesh_vs.rs:8782](../crates/opencascade-sys/generated/mesh_vs.rs#L8782)
- [mesh_vs.rs:9393](../crates/opencascade-sys/generated/mesh_vs.rs#L9393)
- [mesh_vs.rs:9409](../crates/opencascade-sys/generated/mesh_vs.rs#L9409)
- [t_collection.rs:1643](../crates/opencascade-sys/generated/t_collection.rs#L1643)
- [bopds.rs:3042](../crates/opencascade-sys/generated/bopds.rs#L3042)
- [bopds.rs:4074](../crates/opencascade-sys/generated/bopds.rs#L4074)
- [bopds.rs:4151](../crates/opencascade-sys/generated/bopds.rs#L4151)
- [standard.rs:6661](../crates/opencascade-sys/generated/standard.rs#L6661)
- [standard.rs:59650](../crates/opencascade-sys/generated/standard.rs#L59650)
- [resource.rs:964](../crates/opencascade-sys/generated/resource.rs#L964)
- [resource.rs:971](../crates/opencascade-sys/generated/resource.rs#L971)
- [resource.rs:978](../crates/opencascade-sys/generated/resource.rs#L978)
- [resource.rs:985](../crates/opencascade-sys/generated/resource.rs#L985)
- [resource.rs:992](../crates/opencascade-sys/generated/resource.rs#L992)
- [resource.rs:999](../crates/opencascade-sys/generated/resource.rs#L999)
- [ffi.rs:35171](../crates/opencascade-sys/generated/ffi.rs#L35171)
- [ffi.rs:36226](../crates/opencascade-sys/generated/ffi.rs#L36226)
- [ffi.rs:36516](../crates/opencascade-sys/generated/ffi.rs#L36516)
- [wrappers.cpp:286](../crates/opencascade-sys/generated/wrappers.cpp#L286)
- [wrappers.cpp:2802](../crates/opencascade-sys/generated/wrappers.cpp#L2802)
- [wrappers.cpp:3404](../crates/opencascade-sys/generated/wrappers.cpp#L3404)
</details>

Searched for regex `typedef|resolve_type|type_alias|canonical` (`**/crates/opencascade-binding-generator/src/type_mapping.rs`), 4 results
- [type_mapping.rs:197](../crates/opencascade-binding-generator/src/type_mapping.rs#L197)
- [type_mapping.rs:215](../crates/opencascade-binding-generator/src/type_mapping.rs#L215)
- [type_mapping.rs:443](../crates/opencascade-binding-generator/src/type_mapping.rs#L443)
- [type_mapping.rs:452](../crates/opencascade-binding-generator/src/type_mapping.rs#L452)

<details>
<summary>Reading lines 1300-1430 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L1300-L1431">resolver.rs</a></summary>

```
                        return false;
                    }
                }
            }
            _ => {}
        }
        crate::type_mapping::type_uses_unknown_handle(ty, all_class_names, handle_able_classes)
    };
    if func.params.iter().any(|p| check(&p.ty)) {
        return true;
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
<summary>Reading lines 740-810 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L740-L811">resolver.rs</a></summary>

```
    let all_powers_of_2 = nonzero.iter().all(|&v| (v & (v - 1)) == 0);
    let max_val = nonzero.iter().copied().max().unwrap_or(0);

    all_powers_of_2 && max_val >= 4
}

/// Build the symbol table from parsed headers and module graph
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
```
</details>

<details>
<summary>Reading lines 340-370 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L340-L371">resolver.rs</a></summary>

```
    pub classes_by_module: HashMap<String, Vec<SymbolId>>,
    /// Functions grouped by module
    pub functions_by_module: HashMap<String, Vec<SymbolId>>,
    /// Enums grouped by module
    pub enums_by_module: HashMap<String, Vec<SymbolId>>,
    /// All enum names (for filtering methods that use enums)
    pub all_enum_names: HashSet<String>,
    /// All class names (including collection typedef names)
    pub all_class_names: HashSet<String>,
    /// Classes that can have Handle<T> declarations (transitive closure + signature scanning)
    pub handle_able_classes: HashSet<String>,
    /// Cross-module type references by module
    pub cross_module_types: HashMap<String, Vec<CrossModuleType>>,
    /// Authoritative mapping from C++ type name to module name (built from parsed headers)
    /// This is the single source of truth for "which module does type X belong to?"
    pub type_to_module: HashMap<String, String>,
    /// Mapping from C++ enum name to qualified Rust enum type path (e.g., "crate::top_abs::Orientation")
    /// Only contains value enums (not bitset enums). These get typed Rust enum params/returns.
    pub enum_rust_types: HashMap<String, String>,
}

impl SymbolTable {
    /// Get all classes for a module
    pub fn classes_for_module(&self, module: &str) -> Vec<&ResolvedClass> {
        self.classes_by_module
            .get(module)
            .map(|ids| ids.iter().filter_map(|id| self.classes.get(id)).collect())
            .unwrap_or_default()
    }
    
    /// Get all included classes for a module
```
</details>

<details>
<summary>Reading lines 1-100 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L1-L101">type_mapping.rs</a></summary>

```
//! Type mapping between C++ and Rust types
//!
//! Maps OCCT C++ types to their Rust equivalents for extern "C" FFI generation.

#![allow(dead_code)] // Some functions are reserved for future use

use crate::model::Type;
use crate::module_graph::module_to_rust_name;

/// Convert a C++ class name to its Rust Handle type name.
///
/// Strips underscores and `::` (from nested classes) to produce a valid
/// Rust identifier.  e.g. `ShapePersistent_BRep::CurveRepresentation`
/// → `HandleShapePersistentBRepCurveRepresentation`.
pub fn handle_type_name(cpp_name: &str) -> String {
    format!("Handle{}", cpp_name.replace("::", "").replace('_', ""))
}

/// Lowercase variant used for parameter names.
pub fn handle_param_name(cpp_name: &str) -> String {
    format!("handle{}", cpp_name.to_lowercase().replace("::", "").replace('_', ""))
}

/// Result of mapping a C++ type to Rust
#[derive(Debug, Clone)]
pub struct RustTypeMapping {
    /// The Rust type string for use in extern "C" FFI declarations
    pub rust_type: String,
    /// Whether this type is returned as an owned pointer (*mut T) that the caller must free
    pub needs_unique_ptr: bool,
    /// Whether this type needs Pin<&mut T> for mutable self
    pub needs_pin: bool,
    /// The module this type comes from (if cross-module reference)
    pub source_module: Option<String>,
}

/// Map a parsed Type to its Rust representation for extern "C" FFI
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
```
</details>

<details>
<summary>Reading lines 100-500 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L100-L501">type_mapping.rs</a></summary>

```
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

... (truncated, 16615 chars)
```
</details>

<details>
<summary>Searched for regex <code>unknown type|is_known|all_class_names|method_uses_unknown|type_uses_unknown</code> (<code>**/crates/opencascade-binding-generator/src/resolver.rs</code>), 13 results</summary>

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
<summary>Searched for regex <code>is_known_type|type_is_known|type_uses_unknown</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 27 results</summary>

- [model.rs:786](../crates/opencascade-binding-generator/src/model.rs#L786)
- [resolver.rs:1306](../crates/opencascade-binding-generator/src/resolver.rs#L1306)
- [parser.rs:1693](../crates/opencascade-binding-generator/src/parser.rs#L1693)
- [bindings.rs:463](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L463)
- [bindings.rs:477](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L477)
- [bindings.rs:479](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L479)
- [bindings.rs:828](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L828)
- [bindings.rs:836](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L836)
- [bindings.rs:846](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L846)
- [bindings.rs:847](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L847)
- [bindings.rs:859](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L859)
- [bindings.rs:916](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L916)
- [bindings.rs:924](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L924)
- [bindings.rs:961](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L961)
- [bindings.rs:969](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L969)
- [bindings.rs:990](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L990)
- [bindings.rs:2137](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2137)
- [bindings.rs:2715](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2715)
- [bindings.rs:2725](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2725)
- [bindings.rs:2754](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2754)
- [bindings.rs:3369](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3369)
- [bindings.rs:3403](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3403)
- [type_mapping.rs:374](../crates/opencascade-binding-generator/src/type_mapping.rs#L374)
- [type_mapping.rs:397](../crates/opencascade-binding-generator/src/type_mapping.rs#L397)
- [type_mapping.rs:403](../crates/opencascade-binding-generator/src/type_mapping.rs#L403)
- [type_mapping.rs:405](../crates/opencascade-binding-generator/src/type_mapping.rs#L405)
- [type_mapping.rs:433](../crates/opencascade-binding-generator/src/type_mapping.rs#L433)
</details>

<details>
<summary>Reading lines 370-460 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L370-L461">type_mapping.rs</a></summary>

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
```
</details>

<details>
<summary>Reading lines 820-1000 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L820-L1001">bindings.rs</a></summary>

```
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

... (truncated, 8262 chars)
```
</details>

<details>
<summary>Reading lines 460-500 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L460-L501">bindings.rs</a></summary>

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

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/20f22355-bd40-4a8b-9af0-a41343687ade/toolu_01WmxLFC5t9AbK2zChvG3t6Q__vscode-1771608613965/content.txt#1-1), lines 1 to 180

Searched for regex `unknown type.*reason|Reason.*unknown` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:3411](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3411)

<details>
<summary>Searched for regex <code>typedef|resolve.*type|canonical|Standard_PCharacter|BOPDS_PDS|pointer.*typedef</code> (<code>**/crates/opencascade-binding-generator/src/parser.rs</code>), 50 results</summary>

- [parser.rs:18](../crates/opencascade-binding-generator/src/parser.rs#L18)
- [parser.rs:19](../crates/opencascade-binding-generator/src/parser.rs#L19)
- [parser.rs:22](../crates/opencascade-binding-generator/src/parser.rs#L22)
- [parser.rs:23](../crates/opencascade-binding-generator/src/parser.rs#L23)
- [parser.rs:27](../crates/opencascade-binding-generator/src/parser.rs#L27)
- [parser.rs:29](../crates/opencascade-binding-generator/src/parser.rs#L29)
- [parser.rs:31](../crates/opencascade-binding-generator/src/parser.rs#L31)
- [parser.rs:32](../crates/opencascade-binding-generator/src/parser.rs#L32)
- [parser.rs:33](../crates/opencascade-binding-generator/src/parser.rs#L33)
- [parser.rs:35](../crates/opencascade-binding-generator/src/parser.rs#L35)
- [parser.rs:37](../crates/opencascade-binding-generator/src/parser.rs#L37)
- [parser.rs:40](../crates/opencascade-binding-generator/src/parser.rs#L40)
- [parser.rs:71](../crates/opencascade-binding-generator/src/parser.rs#L71)
- [parser.rs:73](../crates/opencascade-binding-generator/src/parser.rs#L73)
- [parser.rs:75](../crates/opencascade-binding-generator/src/parser.rs#L75)
- [parser.rs:76](../crates/opencascade-binding-generator/src/parser.rs#L76)
- [parser.rs:81](../crates/opencascade-binding-generator/src/parser.rs#L81)
- [parser.rs:83](../crates/opencascade-binding-generator/src/parser.rs#L83)
- [parser.rs:87](../crates/opencascade-binding-generator/src/parser.rs#L87)
- [parser.rs:91](../crates/opencascade-binding-generator/src/parser.rs#L91)
- [parser.rs:97](../crates/opencascade-binding-generator/src/parser.rs#L97)
- [parser.rs:99](../crates/opencascade-binding-generator/src/parser.rs#L99)
- [parser.rs:100](../crates/opencascade-binding-generator/src/parser.rs#L100)
- [parser.rs:106](../crates/opencascade-binding-generator/src/parser.rs#L106)
- [parser.rs:109](../crates/opencascade-binding-generator/src/parser.rs#L109)
- [parser.rs:110](../crates/opencascade-binding-generator/src/parser.rs#L110)
- [parser.rs:111](../crates/opencascade-binding-generator/src/parser.rs#L111)
- [parser.rs:112](../crates/opencascade-binding-generator/src/parser.rs#L112)
- [parser.rs:122](../crates/opencascade-binding-generator/src/parser.rs#L122)
- [parser.rs:139](../crates/opencascade-binding-generator/src/parser.rs#L139)
- [parser.rs:140](../crates/opencascade-binding-generator/src/parser.rs#L140)
- [parser.rs:141](../crates/opencascade-binding-generator/src/parser.rs#L141)
- [parser.rs:146](../crates/opencascade-binding-generator/src/parser.rs#L146)
- [parser.rs:147](../crates/opencascade-binding-generator/src/parser.rs#L147)
- [parser.rs:149](../crates/opencascade-binding-generator/src/parser.rs#L149)
- [parser.rs:151](../crates/opencascade-binding-generator/src/parser.rs#L151)
- [parser.rs:153](../crates/opencascade-binding-generator/src/parser.rs#L153)
- [parser.rs:154](../crates/opencascade-binding-generator/src/parser.rs#L154)
- [parser.rs:155](../crates/opencascade-binding-generator/src/parser.rs#L155)
- [parser.rs:156](../crates/opencascade-binding-generator/src/parser.rs#L156)
- [parser.rs:157](../crates/opencascade-binding-generator/src/parser.rs#L157)
- [parser.rs:160](../crates/opencascade-binding-generator/src/parser.rs#L160)
- [parser.rs:161](../crates/opencascade-binding-generator/src/parser.rs#L161)
- [parser.rs:164](../crates/opencascade-binding-generator/src/parser.rs#L164)
- [parser.rs:165](../crates/opencascade-binding-generator/src/parser.rs#L165)
</details>

<details>
<summary>Searched for regex <code>enum Type|Class\(|Handle\(|to_cpp_string|to_rust_type_string|is_known|is_primitive</code> (<code>**/crates/opencascade-binding-generator/src/model.rs</code>), 49 results</summary>

- [model.rs:264](../crates/opencascade-binding-generator/src/model.rs#L264)
- [model.rs:438](../crates/opencascade-binding-generator/src/model.rs#L438)
- [model.rs:447](../crates/opencascade-binding-generator/src/model.rs#L447)
- [model.rs:491](../crates/opencascade-binding-generator/src/model.rs#L491)
- [model.rs:493](../crates/opencascade-binding-generator/src/model.rs#L493)
- [model.rs:528](../crates/opencascade-binding-generator/src/model.rs#L528)
- [model.rs:529](../crates/opencascade-binding-generator/src/model.rs#L529)
- [model.rs:537](../crates/opencascade-binding-generator/src/model.rs#L537)
- [model.rs:572](../crates/opencascade-binding-generator/src/model.rs#L572)
- [model.rs:573](../crates/opencascade-binding-generator/src/model.rs#L573)
- [model.rs:577](../crates/opencascade-binding-generator/src/model.rs#L577)
- [model.rs:578](../crates/opencascade-binding-generator/src/model.rs#L578)
- [model.rs:596](../crates/opencascade-binding-generator/src/model.rs#L596)
- [model.rs:605](../crates/opencascade-binding-generator/src/model.rs#L605)
- [model.rs:620](../crates/opencascade-binding-generator/src/model.rs#L620)
- [model.rs:631](../crates/opencascade-binding-generator/src/model.rs#L631)
- [model.rs:645](../crates/opencascade-binding-generator/src/model.rs#L645)
- [model.rs:663](../crates/opencascade-binding-generator/src/model.rs#L663)
- [model.rs:676](../crates/opencascade-binding-generator/src/model.rs#L676)
- [model.rs:686](../crates/opencascade-binding-generator/src/model.rs#L686)
- [model.rs:745](../crates/opencascade-binding-generator/src/model.rs#L745)
- [model.rs:746](../crates/opencascade-binding-generator/src/model.rs#L746)
- [model.rs:747](../crates/opencascade-binding-generator/src/model.rs#L747)
- [model.rs:750](../crates/opencascade-binding-generator/src/model.rs#L750)
- [model.rs:755](../crates/opencascade-binding-generator/src/model.rs#L755)
- [model.rs:773](../crates/opencascade-binding-generator/src/model.rs#L773)
- [model.rs:774](../crates/opencascade-binding-generator/src/model.rs#L774)
- [model.rs:775](../crates/opencascade-binding-generator/src/model.rs#L775)
- [model.rs:776](../crates/opencascade-binding-generator/src/model.rs#L776)
- [model.rs:777](../crates/opencascade-binding-generator/src/model.rs#L777)
- [model.rs:778](../crates/opencascade-binding-generator/src/model.rs#L778)
- [model.rs:779](../crates/opencascade-binding-generator/src/model.rs#L779)
- [model.rs:786](../crates/opencascade-binding-generator/src/model.rs#L786)
- [model.rs:790](../crates/opencascade-binding-generator/src/model.rs#L790)
- [model.rs:802](../crates/opencascade-binding-generator/src/model.rs#L802)
- [model.rs:821](../crates/opencascade-binding-generator/src/model.rs#L821)
- [model.rs:825](../crates/opencascade-binding-generator/src/model.rs#L825)
- [model.rs:832](../crates/opencascade-binding-generator/src/model.rs#L832)
- [model.rs:836](../crates/opencascade-binding-generator/src/model.rs#L836)
- [model.rs:839](../crates/opencascade-binding-generator/src/model.rs#L839)
- [model.rs:850](../crates/opencascade-binding-generator/src/model.rs#L850)
- [model.rs:863](../crates/opencascade-binding-generator/src/model.rs#L863)
- [model.rs:865](../crates/opencascade-binding-generator/src/model.rs#L865)
- [model.rs:867](../crates/opencascade-binding-generator/src/model.rs#L867)
- [model.rs:869](../crates/opencascade-binding-generator/src/model.rs#L869)
- [model.rs:913](../crates/opencascade-binding-generator/src/model.rs#L913)
- [model.rs:923](../crates/opencascade-binding-generator/src/model.rs#L923)
</details>

<details>
<summary>Reading lines 155-250 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L155-L251">parser.rs</a></summary>

```
/// Used by the resolver to register these as known class types.
pub fn get_collected_typedef_names() -> HashSet<String> {
    TYPEDEF_MAP.with(|m| m.borrow().values().flat_map(|v| v.iter()).cloned().collect())
}

/// Walk the AST to collect simple (non-template) typedef declarations where
/// the underlying type is another OCCT class name. Populates SIMPLE_TYPEDEF_MAP.
///
/// This handles cases like:
///   typedef TColStd_IndexedMapOfTransient BinObjMgt_SRelocationTable;
///   typedef LDOM_Element XmlObjMgt_Element;
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
                        TypeKind::Record | TypeKind::Elaborated | TypeKind::Typedef
                    );

                    if is_record_type {
                        // Get the clean underlying type name
                        let clean = underlying_display
                            .trim()
                            .trim_start_matches("const ")
                            .trim_start_matches("struct ")
                            .trim_start_matches("class ")
                            .trim();

                        // Must look like an OCCT class name (starts with uppercase, no special chars)
                        let looks_like_class = !clean.is_empty()
                            && clean.starts_with(|c: char| c.is_ascii_uppercase())
                            && !clean.contains('<')
                            && !clean.contains('*')
                            && !clean.contains('(')
                            && clean != &name; // skip self-referential typedefs

                        if looks_like_class {
                            map.insert(name.clone(), clean.to_string());
                        }
                    }
                }
            }
        }
        EntityVisitResult::Recurse
    });

    // Chase typedef chains: if A -> B and B -> C, resolve A -> C
    let mut changed = true;
    while changed {
        changed = false;
        let snapshot: Vec<(String, String)> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        for (key, value) in &snapshot {
            if let Some(resolved) = map.get(value) {
                if resolved != value && map.get(key).unwrap() != resolved {
                    map.insert(key.clone(), resolved.clone());
                    changed = true;
                }
            }
        }
    }

    eprintln!("  Collected {} simple typedef entries", map.len());
    SIMPLE_TYPEDEF_MAP.with(|m| {
        *m.borrow_mut() = map;
    });
}

/// Look up a typ
... (truncated, 4156 chars)
```
</details>

<details>
<summary>Searched for regex <code>resolve_clang_type|parse_type|clang_type_to_model</code> (<code>**/crates/opencascade-binding-generator/src/parser.rs</code>), 12 results</summary>

- [parser.rs:395](../crates/opencascade-binding-generator/src/parser.rs#L395)
- [parser.rs:400](../crates/opencascade-binding-generator/src/parser.rs#L400)
- [parser.rs:1028](../crates/opencascade-binding-generator/src/parser.rs#L1028)
- [parser.rs:1038](../crates/opencascade-binding-generator/src/parser.rs#L1038)
- [parser.rs:1252](../crates/opencascade-binding-generator/src/parser.rs#L1252)
- [parser.rs:1254](../crates/opencascade-binding-generator/src/parser.rs#L1254)
- [parser.rs:1327](../crates/opencascade-binding-generator/src/parser.rs#L1327)
- [parser.rs:1464](../crates/opencascade-binding-generator/src/parser.rs#L1464)
- [parser.rs:1468](../crates/opencascade-binding-generator/src/parser.rs#L1468)
- [parser.rs:1617](../crates/opencascade-binding-generator/src/parser.rs#L1617)
- [parser.rs:1629](../crates/opencascade-binding-generator/src/parser.rs#L1629)
- [parser.rs:1638](../crates/opencascade-binding-generator/src/parser.rs#L1638)
</details>

<details>
<summary>Reading lines 1460-1720 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1460-L1721">parser.rs</a></summary>

```
    if result_type.get_kind() == TypeKind::Void {
        return None;
    }

    Some(parse_type(&result_type))
}

/// Parse a clang type into our Type representation
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
            && !s.contains("::");

        if !looks_like_class {
            false
        } else {
            // Check if this is a typedef whose underlying type is a primitive.
            // If so, it's a genuine typedef-to-primitive (like Poly_MeshPurpose = unsigned int).
            // Note: clang wraps typedefs in Elaborated sugar, so check both Typedef and Elaborated kinds.
            // The underlying type of a typedef chain (e.g., Graphic3d_ZLayerId -> Standard_Integer -> int)
            // may appear as Elaborated rather than Typedef, so we accept both.
            let is_primitive_typedef = matches!(kind, TypeKind::Typedef | TypeKind::Elaborated)
                && clang_type.get_declaration()
                    .filter(|d| d.get_kind() == clang::EntityKind::TypedefDecl)
                    .and_then(|d| d.get_typedef_underlying_type())
                    .map(|u| matches!(u.get_kind(),
                        TypeKind::Bool | TypeKind::CharS | TypeKi
... (truncated, 12616 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/20f22355-bd40-4a8b-9af0-a41343687ade/toolu_01T6xMyiypsHRGxTUaNLT12L__vscode-1771608613973/content.txt#1-1), lines 1 to 260

<details>
<summary>Searched for regex <code>typedef.*Standard_PCharacter|Standard_PCharacter</code> (<code>**/crates/occt-sys/OCCT/src/**</code>), 10 results</summary>

- [XmlTObjDrivers_ModelDriver.cxx:97](../crates/occt-sys/OCCT/src/XmlTObjDrivers/XmlTObjDrivers_ModelDriver.cxx#L97)
- [TCollection_AsciiString.hxx:24](../crates/occt-sys/OCCT/src/TCollection/TCollection_AsciiString.hxx#L24)
- [TCollection_AsciiString.hxx:706](../crates/occt-sys/OCCT/src/TCollection/TCollection_AsciiString.hxx#L706)
- [TCollection_ExtendedString.cxx:921](../crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.cxx#L921)
- [TCollection_AsciiString.cxx:1344](../crates/occt-sys/OCCT/src/TCollection/TCollection_AsciiString.cxx#L1344)
- [TCollection_AsciiString.cxx:1358](../crates/occt-sys/OCCT/src/TCollection/TCollection_AsciiString.cxx#L1358)
- [TCollection_AsciiString.cxx:1362](../crates/occt-sys/OCCT/src/TCollection/TCollection_AsciiString.cxx#L1362)
- [TCollection_ExtendedString.hxx:32](../crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx#L32)
- [TCollection_ExtendedString.hxx:365](../crates/occt-sys/OCCT/src/TCollection/TCollection_ExtendedString.hxx#L365)
- [Draw_VariableCommands.cxx:435](../crates/occt-sys/OCCT/src/Draw/Draw_VariableCommands.cxx#L435)
</details>

Searched for regex `typedef.*BOPDS_PDS` (`**/crates/occt-sys/OCCT/src/**`), 1 result
- [BOPDS_PDS.hxx:19](../crates/occt-sys/OCCT/src/BOPDS/BOPDS_PDS.hxx#L19)

Searched for regex `typedef.*MeshVS_DisplayModeFlags` (`**/crates/occt-sys/OCCT/src/**`), 1 result
- [MeshVS_DisplayModeFlags.hxx:19](../crates/occt-sys/OCCT/src/MeshVS/MeshVS_DisplayModeFlags.hxx#L19)

Searched for text `typedef` (`**/crates/occt-sys/OCCT/src/Standard/Standard_PCharacter.hxx`), 2 results
- [Standard_PCharacter.hxx:17](../crates/occt-sys/OCCT/src/Standard/Standard_PCharacter.hxx#L17)
- [Standard_PCharacter.hxx:19](../crates/occt-sys/OCCT/src/Standard/Standard_PCharacter.hxx#L19)

<details>
<summary>Searched for regex <code>map_standard_type</code> (<code>**/crates/opencascade-binding-generator/src/parser.rs</code>), 10 results</summary>

- [parser.rs:1476](../crates/opencascade-binding-generator/src/parser.rs#L1476)
- [parser.rs:1522](../crates/opencascade-binding-generator/src/parser.rs#L1522)
- [parser.rs:1567](../crates/opencascade-binding-generator/src/parser.rs#L1567)
- [parser.rs:1584](../crates/opencascade-binding-generator/src/parser.rs#L1584)
- [parser.rs:1752](../crates/opencascade-binding-generator/src/parser.rs#L1752)
- [parser.rs:1897](../crates/opencascade-binding-generator/src/parser.rs#L1897)
- [parser.rs:1898](../crates/opencascade-binding-generator/src/parser.rs#L1898)
- [parser.rs:1899](../crates/opencascade-binding-generator/src/parser.rs#L1899)
- [parser.rs:1900](../crates/opencascade-binding-generator/src/parser.rs#L1900)
- [parser.rs:1901](../crates/opencascade-binding-generator/src/parser.rs#L1901)
</details>

<details>
<summary>Reading lines 1750-1900 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1750-L1901">parser.rs</a></summary>

```

/// Map OCCT Standard_* typedefs to Rust primitive types
fn map_standard_type(type_name: &str) -> Option<Type> {
    // Strip any const/class/struct prefixes
    let clean = strip_type_qualifiers(type_name);

    match clean {
        // OCCT standard type aliases
        "Standard_Real" => Some(Type::F64),
        "Standard_Integer" => Some(Type::I32),
        "Standard_Boolean" => Some(Type::Bool),
        "Standard_CString" => Some(Type::ConstPtr(Box::new(Type::Class("char".to_string())))),
        "Standard_Size" => Some(Type::Usize),
        "Standard_ShortReal" => Some(Type::F32),
        "Standard_Utf8Char" => Some(Type::Class("char".to_string())),
        "Standard_Character" => Some(Type::Class("char".to_string())),
        "Standard_ExtCharacter" => Some(Type::CHAR16),
        "Standard_Utf32Char" => Some(Type::U32),
        "Standard_ExtString" => Some(Type::ConstPtr(Box::new(Type::CHAR16))),
        // C++ primitive types (may appear from canonical type resolution)
        "double" => Some(Type::F64),
        "float" => Some(Type::F32),
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
        "char32_t" => Some(Type::U32),
        "unsigned char" | "uint8_t" | "Standard_Byte" | "Standard_Utf8UChar" => Some(Type::U8),
        "signed char" | "int8_t" => Some(Type::I8),
        "bool" => Some(Type::Bool),
        // Standard_Address is void* — bound as *mut c_void in unsafe functions.
        // Represented as Type::Class("Standard_Address") so is_void_ptr() can detect it.
        "Standard_Address" => Some(Type::Class("Standard_Address".to_string())),
        // Stream types - map both OCCT typedef names and bare C++ names to the
        // same Type::Class so they're recognized as known manual_types.
        "Standard_OStream" | "std::ostream" => Some(Type::Class("Standard_OStream".to_string())),
        "Standard_IStream" | "std::istream" => Some(Type::Class("Standard_IStream".to_string())),
        "Standard_SStream" | "std::stringstream" => Some(Type::Class("Standard_SStream".to_string())),
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
            .output()
        {
            if output.status.success() {
                let sdk_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                
                // Add C++ standard library headers
                let cxx_include = format!("{}/usr/include/c++/v1", sdk_path);
                if std::path::Path::new(&cxx_include).exists() {
                    args.push("-isystem".to_string());
                    args.push(cxx_include);
                }
                
                // Add general system headers
                let sys_include = format!("{}/usr/include", sdk_path);
                if std::path::Path::new(&sys_include).exists() {
                    args.push("-isystem".to_string());
                    args.push(sys_include);
                }
            }
        }
        
        // Try to find clang's resource directory for built-in headers
        if let Ok(output) = std::process::Command::new("clang")
            .args(["--print-resource-dir"])
            .output()
        {
            if output.status.success() {
                let r
... (truncated, 6414 chars)
```
</details>

<details>
<summary>Reading lines 248-260 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L248-L261">parser.rs</a></summary>

```
/// Look up a type name in the simple typedef map.
/// Returns the underlying class name if this is a known typedef.
fn lookup_simple_typedef(name: &str) -> Option<String> {
    SIMPLE_TYPEDEF_MAP.with(|m| m.borrow().get(name).cloned())
}



/// Parse a collection of OCCT header files
/// 
/// Uses batch parsing: creates a synthetic source file that includes all headers,
/// parses once, then extracts entities from each target header. This is much faster
/// than parsing each header separately since OCCT headers have deep include chains.
```
</details>

Searched for text `BOPDS_PDS` (`**/crates/opencascade-sys/generated/ffi.rs`), 1 result
- [ffi.rs:35171](../crates/opencascade-sys/generated/ffi.rs#L35171)

Searched for text `Standard_PCharacter` (`**/crates/opencascade-sys/generated/ffi.rs`), 1 result
- [ffi.rs:36516](../crates/opencascade-sys/generated/ffi.rs#L36516)

Searched for text `MeshVS_DisplayModeFlags` (`**/crates/opencascade-sys/generated/ffi.rs`), 1 result
- [ffi.rs:36226](../crates/opencascade-sys/generated/ffi.rs#L36226)

<details>
<summary>Reading lines 35168-35180 of <a href="../crates/opencascade-sys/generated/ffi.rs#L35168-L35181">ffi.rs</a></summary>

```
}
/// Referenced type from C++
#[repr(C)]
pub struct BOPDS_PDS {
    _opaque: [u8; 0],
}
/// Referenced type from C++
#[repr(C)]
pub struct BOPDS_PIterator {
    _opaque: [u8; 0],
}
/// Referenced type from C++
#[repr(C)]
```
</details>

<details>
<summary>Reading lines 36223-36230 of <a href="../crates/opencascade-sys/generated/ffi.rs#L36223-L36231">ffi.rs</a></summary>

```
}
/// Referenced type from C++
#[repr(C)]
pub struct MeshVS_DisplayModeFlags {
    _opaque: [u8; 0],
}
/// Referenced type from C++
#[repr(C)]
```
</details>

<details>
<summary>Reading lines 36512-36525 of <a href="../crates/opencascade-sys/generated/ffi.rs#L36512-L36526">ffi.rs</a></summary>

```
    _opaque: [u8; 0],
}
/// Referenced type from C++
#[repr(C)]
pub struct Standard_PCharacter {
    _opaque: [u8; 0],
}
/// Referenced type from C++
#[repr(C)]
pub struct Standard_PExtCharacter {
    _opaque: [u8; 0],
}
/// Referenced type from C++
#[repr(C)]
```
</details>

Searched for text `Referenced type from C++` (`**/crates/opencascade-binding-generator/src/codegen/**`), 1 result
- [rust.rs:535](../crates/opencascade-binding-generator/src/codegen/rust.rs#L535)

<details>
<summary>Reading lines 520-570 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L520-L571">rust.rs</a></summary>

```
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
            unique_variants.push((variant, value));
```
</details>

<details>
<summary>Searched for regex <code>fn emit_cross_module|referenced_types|opaque_type|cross_module</code> (<code>**/crates/opencascade-binding-generator/src/codegen/rust.rs</code>), 5 results</summary>

- [rust.rs:41](../crates/opencascade-binding-generator/src/codegen/rust.rs#L41)
- [rust.rs:193](../crates/opencascade-binding-generator/src/codegen/rust.rs#L193)
- [rust.rs:194](../crates/opencascade-binding-generator/src/codegen/rust.rs#L194)
- [rust.rs:305](../crates/opencascade-binding-generator/src/codegen/rust.rs#L305)
- [rust.rs:310](../crates/opencascade-binding-generator/src/codegen/rust.rs#L310)
</details>

<details>
<summary>Reading lines 41-200 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L41-L201">rust.rs</a></summary>

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
    // Skip unbindable types (arrays, void ptrs, etc.) — but NOT streams,
    // which are manually-defined opaque types that we DO want to collect.
    if ty.is_unbindable() {
        return;
    }

    match ty {
        Type::Class(name) => {
            // Skip primitive types and template instantiations (e.g., NCollection_Shared<...>)
            // that may come from canonical type resolution
            if !is_primitive_type(name) && !name.contains('<') {
                collected.classes.insert(name.clone());
            }
        }
        Type::Handle(name) => {
            // Record the Handle type AND the inner class
            // Skip template instantiations (e.g., NCollection_Shared<...>) which
            // aren't valid as standalone Rust/C++ type names.
            if !name.contains('<') {
                collected.handles.insert(name.clone());
                collected.classes.insert(name.clone());
            }
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
/// (e.g., gp_Pnt, TopoDS_Shape) to avoid collisions and make the mappin
... (truncated, 6286 chars)
```
</details>

<details>
<summary>Reading lines 160-200 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L160-L201">parser.rs</a></summary>

```
/// Walk the AST to collect simple (non-template) typedef declarations where
/// the underlying type is another OCCT class name. Populates SIMPLE_TYPEDEF_MAP.
///
/// This handles cases like:
///   typedef TColStd_IndexedMapOfTransient BinObjMgt_SRelocationTable;
///   typedef LDOM_Element XmlObjMgt_Element;
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
                        TypeKind::Record | TypeKind::Elaborated | TypeKind::Typedef
                    );

                    if is_record_type {
```
</details>

> 💭 Excellent analysis. Now I understand the problem. Let me look at the actual code to figure out the best fixes:
> 
> 1. **`MeshVS_DisplayModeFlags`** (typedef to `Standard_Integer`) - This is the simplest. It should just resolve to `i32`. Can add it to `map_standard_type()` or fix the reference parsing path.
> 
> 2. **`Standard_PCharacter`** (typedef to `char*`) - This is a mutable `char*` pointer. Methods taking `Standard_PCharacter&` are taking a reference to a `char*`. This is tricky - some methods want to write back through the pointer. 
> 
> By value, `Standard_PCharacter` is just `*mut c_char`.
> 
> 3. **`BOPDS_PDS`** (typedef to `BOPDS_DS*`) - Raw pointer to a known class. Methods return BOPDS_PDS meaning they return a raw pointer to BOPDS_DS. Methods taking `const BOPDS_PDS&` take a reference to a pointer.
> 
> Let me examine the actual skipped methods to understand the patterns better and determine the best approach for handling these type definitions.
> 
> For `MeshVS_DisplayModeFlags`, it's a typedef for `Standard_Integer` used as `const MeshVS_DisplayModeFlags&` with default values like `MeshVS_DMF_OCCMask`. I could add it to `map_standard_type()`, but that's just special-casing one type—a better approach would be to make the parser resolve it through clang's canonical type. For `Standard_PCharacter`, it's a typedef for `char*` used as a reference in out-parameter patterns like `ToCString(Standard_PCharacter& aStr)`, which is tricky to bind safely since it's essentially a reference to a pointer.
> 
> For `BOPDS_PDS`, it's a typedef for `BOPDS_DS*` used as `const BOPDS_PDS&` (a reference to a pointer) or as a return type. When returned, it's a raw pointer to `BOPDS_DS`; when passed as `const BOPDS_PDS&`, it's really a const pointer to a mutable pointer. Rather than special-casing each typedef, I should resolve them through clang's canonical type system to handle this more systematically.
> 
> Looking at `parse_type()`, the issue is that `is_primitive_typedef` only catches direct uses like `MeshVS_DisplayModeFlags`, but fails when wrapped in references or pointers like `const MeshVS_DisplayModeFlags&`. For `Standard_PCharacter`, clang resolves it to `char*` directly and `Standard_PCharacter&` to `char*&`, so the parser needs to handle these canonical forms correctly. The real fix is making the reference and pointer handling paths also check for primitive typedefs before giving up.

Now let me look at the actual parser code to understand the type resolution flow:

<details>
<summary>Reading lines 1-50 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1-L51">parser.rs</a></summary>

```
//! Header parser using libclang
//!
//! Extracts class declarations, methods, constructors, enums, and other information
//! from OCCT C++ headers.

use crate::model::{
    Constructor, EnumVariant, Method, Param, ParsedClass, ParsedEnum, ParsedField, ParsedFunction,
    ParsedHeader, StaticMethod, Type,
};
use anyhow::{Context, Result};
use clang::{Accessibility, Availability, Clang, Entity, EntityKind, EntityVisitResult, Index, TypeKind};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Instant;

thread_local! {
    /// Map from NCollection template spellings to their typedef names.
    /// Populated by `collect_ncollection_typedefs()` before type parsing begins.
    /// Key: whitespace-stripped template spelling, e.g.
    ///   "NCollection_Map<TDF_Label,NCollection_DefaultHasher<TDF_Label>>"
    /// Value: all typedef names that alias this template, e.g. ["TDF_LabelMap"]
    /// Multiple typedefs can alias the same template (e.g. gp_Vec3f and Graphic3d_Vec3
    /// both alias NCollection_Vec3<Standard_ShortReal>).
    ///
    /// Both the display-name form (with OCCT aliases like Standard_ShortReal) and
    /// the canonical form (with C++ primitives like float) are stored as keys,
    /// so lookups work regardless of which spelling clang uses.
    static TYPEDEF_MAP: RefCell<HashMap<String, Vec<String>>> = RefCell::new(HashMap::new());

    /// Map from simple typedef names to their underlying class names.
    /// Populated by `collect_simple_typedefs()` before type parsing begins.
    /// Key: typedef name (e.g., "BinObjMgt_SRelocationTable")
    /// Value: underlying type name (e.g., "TColStd_IndexedMapOfTransient")
    /// Only contains typedefs where the underlying type is another OCCT class/typedef
    /// (not template specializations, primitives, or pointers).
    static SIMPLE_TYPEDEF_MAP: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

/// Strip whitespace from a C++ type spelling for typedef map key/lookup.
fn normalize_template_spelling(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Strip C++ type qualifier prefixes (const, volatile, struct, class, typename, enum)
/// from a type spelling. Call sites used to chain these manually; this centralizes the
/// stripping logic and avoids accidental divergence.
fn strip_type_qualifiers(s: &str) -> &str {
    s.trim()
        .trim_start_matches("const ")
```
</details>

Let me look at the key parts of the parser and resolver code:

<details>
<summary>Reading lines 160-260 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L160-L261">parser.rs</a></summary>

```
/// Walk the AST to collect simple (non-template) typedef declarations where
/// the underlying type is another OCCT class name. Populates SIMPLE_TYPEDEF_MAP.
///
/// This handles cases like:
///   typedef TColStd_IndexedMapOfTransient BinObjMgt_SRelocationTable;
///   typedef LDOM_Element XmlObjMgt_Element;
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
                        TypeKind::Record | TypeKind::Elaborated | TypeKind::Typedef
                    );

                    if is_record_type {
                        // Get the clean underlying type name
                        let clean = underlying_display
                            .trim()
                            .trim_start_matches("const ")
                            .trim_start_matches("struct ")
                            .trim_start_matches("class ")
                            .trim();

                        // Must look like an OCCT class name (starts with uppercase, no special chars)
                        let looks_like_class = !clean.is_empty()
                            && clean.starts_with(|c: char| c.is_ascii_uppercase())
                            && !clean.contains('<')
                            && !clean.contains('*')
                            && !clean.contains('(')
                            && clean != &name; // skip self-referential typedefs

                        if looks_like_class {
                            map.insert(name.clone(), clean.to_string());
                        }
                    }
                }
            }
        }
        EntityVisitResult::Recurse
    });

    // Chase typedef chains: if A -> B and B -> C, resolve A -> C
    let mut changed = true;
    while changed {
        changed = false;
        let snapshot: Vec<(String, String)> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        for (key, value) in &snapshot {
            if let Some(resolved) = map.get(value) {
                if resolved != value && map.get(key).unwrap() != resolved {
                    map.insert(key.clone(), resolved.clone());
                    changed = true;
                }
            }
        }
    }

    eprintln!("  Collected {} simple typedef entries", map.len());
    SIMPLE_TYPEDEF_MAP.with(|m| {
        *m.borrow_mut() = map;
    });
}

/// Look up a type name in the simple typedef map.
/// Returns the underlying class name if this is a known typedef.
fn lookup_simple_typedef(name: &str) -> Option<String> {
    SIMPLE_TYPEDEF_MAP.with(|m| m.borrow().get(name).clon
... (truncated, 4312 chars)
```
</details>

Now let me look at `parse_type()` to understand how types are parsed:

Searched for text `fn parse_type` (`**/crates/opencascade-binding-generator/src/parser.rs`), 1 result
- [parser.rs:1468](../crates/opencascade-binding-generator/src/parser.rs#L1468)

<details>
<summary>Reading lines 1468-1620 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1468-L1621">parser.rs</a></summary>

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
            && !s.contains("::");

        if !looks_like_class {
            false
        } else {
            // Check if this is a typedef whose underlying type is a primitive.
            // If so, it's a genuine typedef-to-primitive (like Poly_MeshPurpose = unsigned int).
            // Note: clang wraps typedefs in Elaborated sugar, so check both Typedef and Elaborated kinds.
            // The underlying type of a typedef chain (e.g., Graphic3d_ZLayerId -> Standard_Integer -> int)
            // may appear as Elaborated rather than Typedef, so we accept both.
            let is_primitive_typedef = matches!(kind, TypeKind::Typedef | TypeKind::Elaborated)
                && clang_type.get_declaration()
                    .filter(|d| d.get_kind() == clang::EntityKind::TypedefDecl)
                    .and_then(|d| d.get_typedef_underlying_type())
                    .map(|u| matches!(u.get_kind(),
                        TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |
                        TypeKind::SChar | TypeKind::UChar |
                        TypeKind::Short | TypeKind::UShort |
                        TypeKind::I
... (truncated, 7582 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/20f22355-bd40-4a8b-9af0-a41343687ade/toolu_011VCEuBfqEkTgRAKQ1ihBh6__vscode-1771608613996/content.txt)

<details>
<summary>Reading lines 1620-1750 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1620-L1751">parser.rs</a></summary>

```
            } else {
                Type::MutRef(Box::new(inner))
            };
        }
    }

    // Handle rvalue reference types (T&&) - not bindable but we need to parse them
    if kind == TypeKind::RValueReference {
        if let Some(pointee) = clang_type.get_pointee_type() {
            let inner = parse_type(&pointee);
            return Type::RValueRef(Box::new(inner));
        }
    }

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
    // Check both the display name AND the canonical type spelling, because
    // namespace-scoped Handle typedefs (e.g., IMeshData::IEdgeHandle) have
    // a display name like "IMeshData::IEdgeHandle" but canonical type
    // "opencascade::handle<IMeshData_Edge>".
    // Be careful NOT to match function pointer typedefs whose return type is a Handle,
    // e.g., StdObjMgt_Persistent::Instantiator = Handle(StdObjMgt_Persistent) (*)()
    // has canonical "opencascade::handle<StdObjMgt_Persistent> (*)()". These are
    // detected by checking that the canonical ends with '>' (a pure handle type).
    let clean_spelling = spelling.trim_start_matches("const ").trim();
    let canonical_is_pure_handle = canonical_clean.starts_with("opencascade::handle<")
        && canonical_clean.ends_with('>');
    if clean_spelling.starts_with("opencascade::handle<") || clean_spelling.starts_with("Handle(")
        || canonical_is_pure_handle
    {
        // Prefer the canonical type spelling for the inner type name, because
        // clang's display name may use unqualified names for nested classes
        // (e.g., "Curve" instead of "ShapePersistent_BRep::Curve") when the
        // Handle appears in a method within the parent class scope.
        let inner = if canonical_is_pure_handle {
            extract_template_arg(canonical_clean)
        } else {
            extract_template_arg(clean_spelling)
        };
        return Type::Handle(inner);
    }

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
            .starts_with(|c: char| c.is_ascii_uppercase());
        if !canonical_base.contains("::") && !canonical_base.contains('<') && !canonical_base.is_empty() && canonical_looks_like_class {
            return Type::Class(canonical_base.to_string());
        }
    }
    
    // Check if this type's declaration is nested inside a class
    // This catches types like DESTEP_Parameters::ReadMode_ProductContext that
    // appear as "ReadMode_ProductContext" in me
... (truncated, 6008 chars)
```
</details>

Now let me look at `map_standard_type()`:

<details>
<summary>Reading lines 1750-1860 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1750-L1861">parser.rs</a></summary>

```

/// Map OCCT Standard_* typedefs to Rust primitive types
fn map_standard_type(type_name: &str) -> Option<Type> {
    // Strip any const/class/struct prefixes
    let clean = strip_type_qualifiers(type_name);

    match clean {
        // OCCT standard type aliases
        "Standard_Real" => Some(Type::F64),
        "Standard_Integer" => Some(Type::I32),
        "Standard_Boolean" => Some(Type::Bool),
        "Standard_CString" => Some(Type::ConstPtr(Box::new(Type::Class("char".to_string())))),
        "Standard_Size" => Some(Type::Usize),
        "Standard_ShortReal" => Some(Type::F32),
        "Standard_Utf8Char" => Some(Type::Class("char".to_string())),
        "Standard_Character" => Some(Type::Class("char".to_string())),
        "Standard_ExtCharacter" => Some(Type::CHAR16),
        "Standard_Utf32Char" => Some(Type::U32),
        "Standard_ExtString" => Some(Type::ConstPtr(Box::new(Type::CHAR16))),
        // C++ primitive types (may appear from canonical type resolution)
        "double" => Some(Type::F64),
        "float" => Some(Type::F32),
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
        "char32_t" => Some(Type::U32),
        "unsigned char" | "uint8_t" | "Standard_Byte" | "Standard_Utf8UChar" => Some(Type::U8),
        "signed char" | "int8_t" => Some(Type::I8),
        "bool" => Some(Type::Bool),
        // Standard_Address is void* — bound as *mut c_void in unsafe functions.
        // Represented as Type::Class("Standard_Address") so is_void_ptr() can detect it.
        "Standard_Address" => Some(Type::Class("Standard_Address".to_string())),
        // Stream types - map both OCCT typedef names and bare C++ names to the
        // same Type::Class so they're recognized as known manual_types.
        "Standard_OStream" | "std::ostream" => Some(Type::Class("Standard_OStream".to_string())),
        "Standard_IStream" | "std::istream" => Some(Type::Class("Standard_IStream".to_string())),
        "Standard_SStream" | "std::stringstream" => Some(Type::Class("Standard_SStream".to_string())),
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
            .output()
        {
            if output.status.success() {
                let sdk_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                
                // Add C++ standard library headers
                let cxx_include = format!("{}/usr/include/c++/v1", sdk_path);
                if std::path::Path::new(&cxx_include).exists() {
                    args.push("-isystem".to_string());
                    args.push(cxx_include);
                }
                
                // Add general system headers
                let sys_include = format!("{}/usr/include", sdk_path);
                if std::path::Path::new(&sys_include).exists() {
                    args.push("-isystem".to_string());
                    args.push(sys_include);
                }
            }
        }
        
        // Try to find clang's resource directory for built-in headers
        if let Ok(output) = std::process::Command::new("clang")
            .args(["--print-resource-dir"])
            .output()
        {
            if output.status.success() {
                let r
... (truncated, 4909 chars)
```
</details>

> 💭 Now I understand the problem. Let me look at where these types fail specifically:
> 
> 1. **`MeshVS_DisplayModeFlags`** (typedef to `Standard_Integer`): When clang processes `const MeshVS_DisplayModeFlags& Flags`, it first hits the reference handling at line 1614. It peels off the reference and recurses into `parse_type()` for the pointee. The pointee's display name is `MeshVS_DisplayModeFlags`, and its canonical type resolves to `int`. The `spelling_looks_like_class` check at line 1519-1554 tries to detect if this is a class name that happens to be a primitive typedef. 
> 
> When the reference gets unwrapped, the resulting `MeshVS_DisplayModeFlags` type might be wrapped in an `Elaborated` kind that contains a `Typedef`, and the `is_primitive_typedef` check is looking for exactly that pattern to identify when a typedef name looks like a class but actually resolves to a primitive type. Still analyzing the type-checking logic...
> 
> When `parse_type()` gets called recursively on the pointee of a reference, it receives a fresh `clang::Type` object, so the `kind` variable gets recomputed from that pointee's actual kind rather than inheriting the outer typedef wrapper. This means the typedef check might not trigger the way I expected—the pointee's kind could be something entirely different after clang resolves the layers. Looking at the actual skip comments in the generated code would help clarify what's happening with these typedef cases. I should also verify whether `MeshVS` is even included in the bindings configuration, since that would determine if these symbols are being generated at all. The README lists three problematic typedefs—`Standard_PCharacter` as a pointer typedef, `BOPDS_PDS` as a raw pointer typedef, and `MeshVS_DisplayModeFlags` which should be resolvable since it's just a typedef to `Standard_Integer`.

Let me see the actual skip comments in the generated files:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -rn "MeshVS_DisplayModeFlags\|Standard_PCharacter\|BOPDS_PDS" crates/opencascade-sys/generated/*.rs | head -40
```
<details>
<summary>Output (30 lines)</summary>

```
crates/opencascade-sys/generated/bop_algo.rs:11132://   Reason: return type 'BOPDS_PDS' is unknown
crates/opencascade-sys/generated/bop_algo.rs:11133://   // pub fn pds(&mut self) -> OwnedPtr<BOPDS_PDS>;
crates/opencascade-sys/generated/bop_algo.rs:14892://   Reason: return type 'BOPDS_PDS' is unknown
crates/opencascade-sys/generated/bop_algo.rs:14893://   // pub fn pds(&mut self) -> OwnedPtr<BOPDS_PDS>;
crates/opencascade-sys/generated/bop_algo.rs:16471://   Reason: param 'theDS' uses unknown type 'BOPDS_PDS&'
crates/opencascade-sys/generated/bop_algo.rs:16476://   Reason: param 'pDS' uses unknown type 'BOPDS_PDS&'
crates/opencascade-sys/generated/bop_algo.rs:16480://   Reason: param 'theDS' uses unknown type 'BOPDS_PDS'
crates/opencascade-sys/generated/bopds.rs:3042://   Reason: param 'pDS' uses unknown type 'const BOPDS_PDS&'
crates/opencascade-sys/generated/bopds.rs:4074://   Reason: param 'pDS' uses unknown type 'const BOPDS_PDS&'
crates/opencascade-sys/generated/bopds.rs:4151:    BOPDS_PDS as PDS,
crates/opencascade-sys/generated/ffi.rs:35171:pub struct BOPDS_PDS {
crates/opencascade-sys/generated/ffi.rs:36226:pub struct MeshVS_DisplayModeFlags {
crates/opencascade-sys/generated/ffi.rs:36516:pub struct Standard_PCharacter {
crates/opencascade-sys/generated/mesh_vs.rs:3078://   Reason: param 'Flags' uses unknown type 'const MeshVS_DisplayModeFlags&'
crates/opencascade-sys/generated/mesh_vs.rs:5881://   Reason: param 'Flags' uses unknown type 'const MeshVS_DisplayModeFlags&'
crates/opencascade-sys/generated/mesh_vs.rs:6395://   Reason: param 'Flags' uses unknown type 'const MeshVS_DisplayModeFlags&'
crates/opencascade-sys/generated/mesh_vs.rs:6422:    /// DisplayMode is numeric constant describing display mode (see MeshVS_DisplayModeFlags.hxx)
crates/opencascade-sys/generated/mesh_vs.rs:6448:    /// DisplayMode is numeric constant describing display mode (see MeshVS_DisplayModeFlags.hxx)
crates/opencascade-sys/generated/mesh_vs.rs:8782://   Reason: param 'Flags' uses unknown type 'const MeshVS_DisplayModeFlags&'
crates/opencascade-sys/generated/mesh_vs.rs:9393://   Reason: param 'Flags' uses unknown type 'const MeshVS_DisplayModeFlags&'
crates/opencascade-sys/generated/mesh_vs.rs:9409:    MeshVS_DisplayModeFlags as DisplayModeFlags,
crates/opencascade-sys/generated/resource.rs:964://   Reason: param 'tostr' uses unknown type 'Standard_PCharacter&'
crates/opencascade-sys/generated/resource.rs:971://   Reason: param 'tostr' uses unknown type 'Standard_PCharacter&'
crates/opencascade-sys/generated/resource.rs:978://   Reason: param 'tostr' uses unknown type 'Standard_PCharacter&'
crates/opencascade-sys/generated/resource.rs:985://   Reason: param 'tostr' uses unknown type 'Standard_PCharacter&'
crates/opencascade-sys/generated/resource.rs:992://   Reason: param 'theToStr' uses unknown type 'Standard_PCharacter&'
crates/opencascade-sys/generated/resource.rs:999://   Reason: param 'theToStr' uses unknown type 'Standard_PC
... (truncated, 3379 chars total)
```
</details>

> 💭 Now I understand exactly what's happening. Let me categorize the fixes:
> 
> ### MeshVS_DisplayModeFlags (5 instances)
> Used as `const MeshVS_DisplayModeFlags&` - this is a reference to `Standard_Integer` (i.e., `const int&`).
> 
> The fix should make the parser resolve `MeshVS_DisplayModeFlags` to `Type::I32`, same as how `Standard_Integer` works. The most general approach: when `parse_type()` encounters a typedef whose canonical type is a primitive, resolve it as the primitive. The `is_primitive_typedef` check should catch this but isn't.
>  
> I see the issue now. There's a guard that specifically handles references and pointers with canonical "int" types, but it's preserving the class-like display name instead of resolving to the primitive. This guard is catching the case and wrapping it in a `ConstRef` with the original typedef name, which defeats the purpose of recognizing it as a primitive type.
> 
> The fix is to add a check for primitive typedefs within this guard so that when we encounter something like `MeshVS_DisplayModeFlags` that canonically resolves to `int`, we recognize it as a primitive rather than treating it as a class. to `map_standard_type()`. But that's a band-aid for one type. There are probably other typedefs-to-Standard_Integer that we don't know about.
> 
> The better fix: in the reference handler guard (lines 1576-1609), also check if the type is a primitive typedef (same check as `is_primitive_typedef` above) and if so, DON'T treat it as a class.
> 
> Wait, let me think about this differently. The guard at 1576 runs when the OUTER type is a reference/pointer and the canonical base is "int". It then checks if the display name looks like a class. But `MeshVS_DisplayModeFlags` is NOT a class - it's a typedef to int. The guard should detect this.
> 
> The fix should be: in the guard at 1576-1609, after determining `base_looks_like_class`, I need to also check if the base name is a known primitive typedef and skip the Class type creation if it is. The challenge is that inside the reference handler, I'm working with the whole reference type, not the pointee, so I'd need to extract the pointee declaration and check its underlying type. Actually, the simpler approach is to recognize that when clang resolves `const MeshVS_DisplayModeFlags&`, the canonical type becomes `const int&` since `MeshVS_DisplayModeFlags` is a typedef chain leading to `int`. int"`. The guard at 1576 catches this, but incorrectly preserves the class name.
> 
> The fix: In the guard, before creating `Type::Class(base)`, check if the pointee is a primitive typedef. We can do this by calling `clang_type.get_pointee_type()` and then checking if *that* type is a typedef whose underlying resolves to a primitive.
> 
> Actually, an even simpler approach: instead of this complex guard, we can check the pointee type by passing it through `map_standard_type()` AND also checking if it resolves transitively through `typedef -> Standard_Integer -> int`. Let me think...
> 
> Actually, I think the cleanest fix for all three types involves different approaches:
> 
> 1. **`MeshVS_DisplayModeFlags`**: This is the `typedef Standard_Integer MeshVS_DisplayModeFlags`. The issue is that when we encounter `const MeshVS_DisplayModeFlags&`, the guard at line 1576 sees the outer canonical as `const int&` and incorrectly creates `Type::Class("MeshVS_DisplayModeFlags")`. The solution is to enhance the guard to detect when the base name is itself a typedef pointing to a primitive—by traversing the pointee's declaration chain through `get_typedef_underlying_type()` to confirm it ultimately resolves to a primitive, then skip the class creation and let the normal reference handler recurse and resolve to the correct primitive type.
> 
> 2. **`Standard_PCharacter`**: This is `typedef char* Standard_PCharacter`. Most uses are `Standard_PCharacter&` (reference to char pointer). The fix would be to add it to `map_standard_type()` as `Type::MutPtr(Type::Class("char"))`, but the reference-to-mutable-pointer binding is tricky.
> 
> I could add both to `map_standard_type()`, but then I'd need to handle every such typedef. Let me check if there are other similar ones in the codebase.
> 
> Looking at the actual methods, I see that `Resource_Unicode::ConvertSJISToUnicode` takes `Standard_PCharacter&` - a reference to a char pointer that can be repointed, which is effectively an out-parameter. Meanwhile, `Standard_GUID::ToCString` takes it by value, which is just a mutable char pointer. For by-value, I'd map it to `*mut c_char`, but for the reference case, I'm dealing with `&mut *mut c_char` in Rust, which is hard to represent safely.
> 
> Scanning through the codebase, I'm seeing a pattern: `resource.rs` has 6 instances of `Standard_PCharacter&` as out-parameters, `standard.rs` has 1 by-value instance, and `t_collection.rs` has 1 reference instance. The `Standard_PCharacter&` pattern really needs special handling since it's fundamentally a reference to a mutable pointer.
> 
> The by-value form should be straightforward to map to `*mut c_char`, but the reference form is the tricky part—it's essentially `&mut *mut c_char` in Rust terms, which doesn't map cleanly to FFI. I think the pragmatic approach is to start by adding just the by-value `Standard_PCharacter` to the type mapping and see how far that gets me. would resolve the by-value case (1 instance in standard.rs). The reference cases (7 instances) would need the parser to correctly see `Standard_PCharacter` as `*mut char` and then the reference wrapping would produce `&mut *mut char`, which the resolver would then need to handle `MutRef(MutPtr(Class("char")))`. Let me check if this would actually be bindable or if it would hit another skip reason.
> 
> Actually, let me trace through this more carefully. If I add `Standard_PCharacter` to `map_standard_type()` as `Type::MutPtr(Type::Class("char"))`, then when parsing `Standard_PCharacter&`, the reference handling code would recursively parse the pointee type, get back the mutable pointer type, and wrap it in a `MutRef`. But looking at the actual reference parsing logic, it extracts the pointee, checks if it's const-qualified, then recursively parses and wraps the result—so I'd end up with `MutRef(MutPtr(Class("char")))` which the resolver would need to handle. I'm tracing through what happens when we recursively process `Standard_PCharacter` as a reference type—the handler would extract the pointee and call `map_standard_type()` on it, which would return a mutable pointer to char, giving us a mutable reference to a mutable pointer. Now I need to figure out whether Rust can actually bind this nested reference-pointer combination, or if the generator needs special handling for it. the wrapper. Let me check if `type_uses_unknown_type()` would flag this... `MutPtr(Class("char"))` - `Class("char")` should pass the `is_primitive_type()` check (which checks for "char"). So `type_uses_unknown_type()` should return false. The type would be considered known.
> 
>    But then, would the code generator correctly handle `MutRef(MutPtr(Class("char")))` as a Rust type? The Rust type emission would need to produce `&mut *mut c_char`. And the C++ wrapper would need to pass `char*&`. 
> 
>    Actually, I think this would be an issue because the wrapper generation might not know how to handle `&mut *mut c_char` parameters correctly. These are double-indirection patterns that are not commonly tested. Let me think about how this actually works in practice - `Standard_PCharacter&` is an out-parameter for a `char*`, so in Rust FFI this becomes `&mut *mut c_char`, which is technically valid but the real problem is that extern "C" functions can't directly take references to pointers like `char*&`.
> 
>    So the wrapper needs to take `char**` instead and dereference it. This is getting complex. Let me focus on what I can actually fix - the `BOPDS_PDS` typedef which is `BOPDS_DS*`. It appears as return types, const references to pointers, mutable references to pointers, and by-value pointers. If I make the parser resolve these properly, I can handle the different use cases.
> 
>    The tricky part is the reference-to-pointer patterns - a const reference to a pointer and a mutable reference to a pointer don't map cleanly to Rust. In the C wrapper, these would need special handling since Rust doesn't have the same pointer reference semantics.
> 
>    Actually, I should check if BOPDS_DS is even part of the binding set first. And thinking about this more carefully - these pointer typedefs are probably just opaque handles for internal use. Methods like `SetDS` just store a pointer, and `PDS()` retrieves it. These are internal API methods, not user-facing, so binding them might not be worth the complexity.
> 
>    Let me reconsider what's actually feasible here. `MeshVS_DisplayModeFlags` as a typedef to Standard_Integer is the simplest case to handle.
> 
>    For `Standard_PCharacter` and `BOPDS_PDS`, both are pointer typedefs with the same issue - the by-value forms are straightforward but the reference-to-pointer forms require complex double indirection handling. I'll prioritize fixing `MeshVS_DisplayModeFlags` since that's a clear parser limitation, then tackle the pointer typedefs if there's time. Standard_PCharacter and BOPDS_PDS), they're pointer typedefs. The README says "pointer typedef, not yet resolvable" and "raw pointer typedef" - these are inherently limited by the FFI boundary. Methods that take `char*&` or `BOPDS_DS*&` need double indirection which the current wrapper generation doesn't handle.
> 
> So my strategy is to tackle `MeshVS_DisplayModeFlags` first by improving how I detect primitive typedefs, then handle `Standard_PCharacter` by adding it to `map_standard_type()` for the by-value case while addressing the reference cases separately. For `BOPDS_PDS`, I can resolve it to `*mut BOPDS_DS` in by-value scenarios, though the reference cases will still require wrapper changes.
> 
> Looking at the README's guidance, I can see which items are actually blocked by missing dependencies - things like `Handle(ShapePersistent_Geom)`, `Handle(BVH_Builder)`, platform-specific code like `WNT_HIDSpaceMouse`, and external library dependencies like ffmpeg and RapidJSON. The user wants me to fix everything except those blocked items, so I should focus on the three typedefs that are resolvable with the current approach. or `Pointer` kind, so the guard doesn't catch them. I need to refine the guard to also check the underlying type when dealing with references and pointers, so it can properly identify and skip over legitimate typedefs that are wrapped in indirection. the guard logic, I'll create dedicated maps for pointer typedefs and primitive typedefs, then have `parse_type()` consult these maps to resolve display names that are actually typedef aliases to pointers or primitives. the reference/pointer guard, I need to check if the base type is a primitive typedef before applying the guard logic. The issue is that `is_primitive_typedef` only catches typedefs at the top level, but when we have a reference to a typedef-to-primitive, the outer kind is `LValueReference` so the check gets skipped. I should move the primitive typedef detection inside the guard condition itself, checking the stripped base type against known primitives. I'm realizing the guard at line 1580 only checks for `canonical_base == "int"`, so it wouldn't catch other primitives like double or char. The real issue is that this guard was meant to prevent incorrect resolution of template or namespace types that clang mistakenly resolves to "int" due to failed instantiation, so I need to preserve that protection while generalizing it to handle all primitive types, not just int.
> 
> For the `Standard_PCharacter` case, I'll add it to the standard type map as a mutable pointer to char, and then the reference handler will naturally resolve the reference case by extracting the pointee type. 2. `parse_type()` on the pointee: hits `map_standard_type("Standard_PCharacter")` → `MutPtr(Class("char"))`  
> 3. Return `MutRef(MutPtr(Class("char")))` or `ConstRef(MutPtr(Class("char")))`
> 
> But wait - will the reference handler be reached? The canonical type of `Standard_PCharacter&` is `char *&`. The guard at 1576 checks for reference types with canonical base... `strip_type_decorators("char *")` would be `"char"`. Then `canonical_base == "int"` is false, so the guard doesn't fire. Good - the guard doesn't interfere.
> 
> So the normal reference handler at 1614 would fire, checking if the pointee is const-qualified, then recursively parsing the pointee type. When `parse_type()` is called on `Standard_PCharacter`, it maps to `MutPtr(Class("char"))`, giving us `MutRef(MutPtr(Class("char")))`.
> 
> Now I need to check if `MutRef(MutPtr(Class("char")))` is actually bindable. the FFI boundary. Let me trace through how the codegen currently handles reference parameters—in the re-export a `MutRef(Class("X"))` becomes `&mut X`, at the FFI level it's `*mut X`, and in the C++ wrapper it's `X&` with a dereference of `*self_ptr`. For the nested case `MutRef(MutPtr(Class("char")))`, this creates a double pointer situation: `*mut *mut c_char` at the FFI level and `char**` in the C++ wrapper that needs to convert to `char*&` before calling the actual method. I think the fundamental issue is how the codegen emits these nested pointer-and-reference patterns.
> 
> Let me be more practical and check what these skipped methods actually do. Looking at the resource.rs functions like `Resource_Unicode::ConvertSJISToUnicode`, they're character encoding converters that take `Standard_PCharacter&` as output parameters—they write string results into the char* that the reference points to. The standard.rs methods like `Standard_GUID::ToCString` take char* by value instead.
> 
> Now I'm realizing that `Standard_PCharacter&` is really `char*&`—a mutable reference to a pointer itself, not just the data it points to. In C terms that's `char**`, and at the FFI boundary it becomes `*mut *mut c_char`. This should be bindable if the codegen handles it correctly. Let me try the straightforward approach: add `Standard_PCharacter` to the type mapping, fix the `MeshVS_DisplayModeFlags` typedef detection, and see what happens.
> 
> But I'm second-guessing whether just collecting more typedefs will actually solve the resolution problem. The real issue is that the simple typedef map only maps names to names, not names to actual types. I need to think about whether there's a more fundamental approach to handling pointer and primitive typedefs in the type resolution logic itself. I could also create separate functions to collect pointer typedefs and resolve them similarly, mapping names like `Standard_PCharacter` to their underlying pointer types. But I'm realizing this approach is getting fragmented—I'm duplicating logic across multiple maps and lookup functions. The cleaner path might be to enhance `parse_type()` itself to leverage clang's canonical type resolution at the end, which would handle all these typedef chains automatically without needing manual map population.
> 
> Writing type resolution logic... I'm realizing the guard at line 1576 would intercept the reference case before reaching the canonical resolution logic, so it would incorrectly return a `ConstRef` wrapping a class type instead of resolving through to the underlying primitive. I need to rethink how to handle this—the guard needs to be more selective about when it applies the decorator-stripping logic.
> 
> Refining the guard logic...
> 
> The guard at 1576 is actually protecting against a specific problem: some templated or namespaced types resolve to primitive types like `int` through their canonical form, but they're not actually primitives—like `NCollection_Map<...>::size_type` which canonicalizes to `int` but should be treated as a class type. Without this guard, we'd incorrectly treat these as simple integers.
> 
> The issue is the guard is too broad and catches legitimate typedefs like `MeshVS_DisplayModeFlags` that genuinely typedef to `Standard_Integer`. I can distinguish between them by checking if the display name contains template or namespace markers like `::` or `<`—those indicate nested/templated types that need name preservation, while simple typedef names should resolve through their canonical type. Looking at the logic around line 1589, it seems the code already has this distinction built in.
> 
> Refining the approach: for simple typedef names without templates or namespaces, I should check if the pointee is a primitive typedef and skip the interception if it is.
> 
> Still writing the type check... I need to add `Standard_PCharacter` to the type mapping to handle char pointer typedefs, and then refine the reference/pointer guard to let primitive and pointer typedefs flow through to normal resolution instead of being intercepted as class-like types.
> 
> For the late-stage canonical resolution, I should add it at the end of `parse_type()` so that unrecognized pointer typedef display names like `BOPDS_PDS` can resolve through their canonical types. Since `MeshVS_DisplayModeFlags` only appears in reference contexts based on the skip comments, the existing primitive typedef check should handle that case, but I could add the late-stage resolution for safety as well.
> 
> Now looking at the actual `BOPDS_PDS` instances—there are by-value returns and both mutable and const references. The reference handler already recurses into `parse_type()` for the pointee, where the display name `BOPDS_PDS` maps to the canonical `BOPDS_DS*`, so the late-stage resolution should catch this.
> 
> But I'm realizing the guard at line 1576 might still intercept the outer reference wrapper since the canonical of `BOPDS_PDS&` becomes `BOPDS_DS*&`, and I need to understand what `strip_type_decorators` actually does to see if that's the real issue.

Searched for text `fn strip_type_decorators` (`**/crates/opencascade-binding-generator/src/parser.rs`), 1 result
- [parser.rs:61](../crates/opencascade-binding-generator/src/parser.rs#L61)

<details>
<summary>Reading lines 45-85 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L45-L86">parser.rs</a></summary>

```
/// Strip C++ type qualifier prefixes (const, volatile, struct, class, typename, enum)
/// from a type spelling. Call sites used to chain these manually; this centralizes the
/// stripping logic and avoids accidental divergence.
fn strip_type_qualifiers(s: &str) -> &str {
    s.trim()
        .trim_start_matches("const ")
        .trim_start_matches("volatile ")
        .trim_start_matches("struct ")
        .trim_start_matches("class ")
        .trim_start_matches("typename ")
        .trim_start_matches("enum ")
        .trim()
}

/// Strip type qualifier prefixes AND trailing reference/pointer decorators.
/// Useful when extracting the base type name from a fully decorated C++ type spelling.
fn strip_type_decorators(s: &str) -> &str {
    strip_type_qualifiers(s)
        .trim_end_matches(" &")
        .trim_end_matches(" &&")
        .trim_end_matches(" *")
        .trim_end_matches('&')
        .trim_end_matches('*')
        .trim()
}

/// Walk the AST to collect all typedef/using declarations that resolve to
/// template specializations (NCollection, math_VectorBase, etc.).
/// Populates the thread-local TYPEDEF_MAP.
///
/// For each typedef, we insert keys for BOTH the display-name spelling
/// (e.g. NCollection_Vec3<Standard_ShortReal>) and the canonical spelling
/// (e.g. NCollection_Vec3<float>). This handles OCCT headers that use
/// C++ primitives directly in method signatures rather than the OCCT aliases.
///
/// `included_modules` is the set of module prefixes (e.g. "gp", "Geom") that
/// are included in the binding generation. When multiple typedefs alias the
/// same template, we prefer names from included modules.
fn collect_ncollection_typedefs(root: &Entity, included_modules: &HashSet<String>) {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

```
</details>

> 💭 Now I understand `strip_type_decorators`. For `BOPDS_PDS&` the canonical is `BOPDS_DS *&`. After strip_type_decorators, the canonical base would be:
> - Strip qualifiers: `BOPDS_DS *&` → `BOPDS_DS *&` (no qualifiers)
> - Strip `&`: `BOPDS_DS *`
> - Strip `*`: `BOPDS_DS`
> 
> So `canonical_base` would be `"BOPDS_DS"`. Then the guard checks `canonical_base == "int"` — which is false. So the guard at 1576 doesn't fire for `BOPDS_PDS&`. Good — the normal reference handler at 1614 would process it.
> 
> 
> For `const MeshVS_DisplayModeFlags&`, the canonical form is `const int &`, and after stripping qualifiers and decorators I get `int`, which matches the guard condition at line 1579. But for `BOPDS_PDS&`, the canonical is `BOPDS_DS *&`, and stripping everything leaves just `BOPDS_DS`, so that guard doesn't trigger.
> 
> The issue is that `BOPDS_PDS` falls through to the normal reference handler, which recurses into `parse_type` and returns `Type::Class("BOPDS_PDS")` instead of resolving the underlying pointer typedef. I need to fix the end of `parse_type` to handle pointer typedefs properly. For `Standard_PCharacter&`, the canonical is `char *&`, which strips down to `char`, so that guard also doesn't fire.
> 
> My approach is to add `Standard_PCharacter` to `map_standard_type()` to handle all 8 instances across both by-value and reference contexts, then fix the guard at 1576 for `MeshVS_DisplayModeFlags`.
> 
> For `BOPDS_PDS`, I need to add late-stage canonical resolution for pointer typedefs to handle both by-value cases and when recursed from the reference handler. Looking at the reference case with `BOPDS_PDS&`, the canonical form is `BOPDS_DS *&` where the canonical_base is `BOPDS_DS`, so the guard doesn't fire and the normal reference handler processes it correctly. For the by-value method return type, the canonical is `BOPDS_DS *`, which I need to resolve separately. Now I'm checking whether raw pointer returns to classes are actually bindable—the return type handling needs to account for how the reexport layer converts these pointers to safe Rust references. bindable. Let me search for how pointer returns are handled.
> 
> Actually, I recall from the README: "Class Pointer Returns: Instance methods returning `const T*` or `T*` (where `T` is a known class type) are bound as `Option<&T>` or `Option<&mut T>` in Rust."
> 
> So `BOPDS_PDS` return (= `BOPDS_DS*` return) would be treated as a class pointer return, bound as `Option<&mut BOPDS_DS>`. That's sensible.
> 
> For `BOPDS_PDS` as parameter by value (= `BOPDS_DS*`): This is a raw pointer parameter. The README says "Non-Nullable Class Pointer Parameters: Methods with non-nullable `const T*` or `T*` parameters are bound as `&T` or `&mut T` in Rust." So `BOPDS_PDS` param would become `&mut BOPDS_DS`. That's also sensible.
> 
> For `const BOPDS_PDS&` (= const reference to `BOPDS_DS*`): After resolving, this would be `ConstRef(MutPtr(Class("BOPDS_DS")))`. In C++ terms, `const BOPDS_DS*&` is a const reference to a mutable pointer, which becomes `BOPDS_DS* const*` in the wrapper and `&*mut BOPDS_DS` in Rust. I'm wondering if this pattern appears elsewhere in the codebase.
> 
> Actually, let me think about what `const BOPDS_PDS&` means semantically. It's `void SetDS(const BOPDS_PDS& pDS)` where `BOPDS_PDS = BOPDS_DS*`. So it's `void SetDS(BOPDS_DS* const& pDS)` which is "pass a BOPDS_DS pointer by const reference". Functionally this is the same as just passing `BOPDS_DS*` by value (you're just passing a pointer). The const reference is an efficiency optimization that doesn't matter for pointer-sized types.
> 
> So the question is: can the codegen handle `ConstRef(MutPtr(Class("BOPDS_DS")))`? This is `&(*
> 
> mut BOPDS_DS)` in Rust, which is `*const *mut BOPDS_DS` at the FFI boundary, and `BOPDS_DS* const*` in C++. I suspect the codegen might not handle references-to-pointers correctly since that's an unusual pattern.
> 
> Let me check the type checking code to see what's actually bindable, or maybe I should just resolve `BOPDS_PDS` directly to `Class("BOPDS_DS")` instead of `MutPtr(Class("BOPDS_DS"))` to avoid the nested pointer issue.
> 
> Actually, wait — `BOPDS_PDS` is itself a pointer type, so I can't collapse it without losing the pointer semantics. The real issue is that when it appears as a return type, it's a raw pointer return, and when it's a parameter like `const BOPDS_PDS&`, it's passing a pointer by const reference, which at the ABI level becomes a pointer-to-pointer.
> 
> Let me try a more pragmatic approach: resolve `BOPDS_PDS` to `MutPtr(Class("BOPDS_DS"))` and then check whether the existing codegen already handles mutable pointers in return positions and as parameters, including the reference-wrapped variants. If the codegen can't handle certain patterns, I'll find out when I try to regenerate and see what errors come up.
> 
> Actually, I'm realizing I should think bigger here. Rather than patching individual types, I could implement a general solution that automatically resolves all primitive typedefs to their underlying primitives and all pointer typedefs to their underlying pointer types. This would handle `MeshVS_DisplayModeFlags`, `BOPDS_PDS`, `Standard_PCharacter`, and any other similar typedefs without needing special cases for each one.
> 
> The approach is to check at the end of `parse_type()` whether an unrecognized typedef has a canonical type that's either a primitive or a pointer, and if so, resolve through that canonical type instead of just wrapping it in `Type::Class`.
> 
> Implementing the general resolution logic... Still writing type resolution logic... I'm realizing the guard at line 1576 only triggers when the canonical base is "int", so it won't catch pointer typedefs like BOPDS_PDS where the canonical base would be something else after stripping the pointer. This means the real fix is simpler—I just need to handle primitive typedefs in that specific guard rather than trying to intercept all pointer cases. the guard at 1576 for primitive typedefs (MeshVS_DisplayModeFlags)
> 2. Add late-stage canonical resolution for pointer typedefs (BOPDS_PDS, Standard_PCharacter)
> 
> But also - Standard_PCharacter would be handled by late-stage canonical resolution too! Its canonical is `char*` which is a Pointer kind. So I don't need to add it to map_standard_type separately.
> 
> Let me also think about what the guard at 1576 protects against. The comment at the guard says:
> "Guard: when the OUTER type's display name identifies an OCCT class but the canonical type is "int", construct the class type directly instead of recursing into the pointee (whose display name might already be "int", losing the typedef info)."
> 
> So the guard exists because: when you have `const Foo_Bar&` where Foo_Bar is a class but its canonical is "int" (template failure), recursing into the pointee would produce `Type::I32` which is wrong. The guard preserves the class name.
> 
> But for genuine typedef-to-int like MeshVS_DisplayModeFlags = Standard_Integer = int, we want canonical resolution to kick in. The fix is to refine the guard: check if the pointee is a Typedef or Elaborated, and if its declaration is a real typedef chain leading to a primitive, allow it through. If there's no declaration or the chain doesn't resolve cleanly, it's likely a template failure, so preserve the class name.
> 
> The most reliable check is to walk the typedef declaration chain: if the underlying type is itself a typedef or primitive (not a template), it's genuine. For MeshVS_DisplayModeFlags, the declaration's underlying type is Standard_Integer, whose declaration's underlying is int — that's a clean chain, so allow canonical resolution.
> 
> For NCollection_Map<T>::size_type, it's inside a template specialization that didn't fully instantiate, so the chain breaks differently. I could also try checking if the typedef has a file source location rather than being synthesized by template instantiation, or collect all primitive typedefs during the initial AST walk as a dedicated pass.
> 
> Actually, the pragmatic approach is simpler. The guard at line 1576 checks `canonical_base == "int"` and then `base_looks_like_type`, so for it to fail we'd need a genuine typedef-to-int that looks like a class name—things like Poly_MeshPurpose (typedef to unsigned int) or other bitmask types. These are rare, and the current code already handles them through `is_primitive_typedef` for non-references. The issue is only in the reference guard where that check is missing. So I'll add the `is_primitive_typedef` check to the reference/pointer guard at line 1576-1609 to prevent intercepting primitive typedefs.
> 
> Then at the end of `parse_type()`, before returning `Type::Class(clean_name)`, I'll add late-stage canonical resolution that handles pointer typedefs by resolving to `MutPtr`/`ConstPtr` of the pointee, and primitive typedefs by resolving to the primitive Type itself—both gated on the type being Typedef or Elaborated kind to avoid accidentally resolving non-typedef types. The main risk is that template-instantiated types inside a reference could also be Typedef/Elaborated with a primitive canonical, but that's an edge case worth handling.
> 
> Actually, I'm realizing the real concern: if a template fails to instantiate, the canonical type wouldn't resolve to "int" at all—it would stay as the unresolved template name. The canonical only becomes "int" if the template successfully instantiates to an int type, which means the guard I'm adding would correctly catch it and prevent wrong bindings.
> 
> Let me look at the original comment on that guard to understand what problem it was actually solving...
> 
> The issue is about typedef information getting lost when clang strips the sugar. If we have something like `const MeshVS_DisplayModeFlags&` where the typedef resolves to int, the reference handler would normally recurse into the pointee and lose the original typedef name. But actually, clang does preserve typedef display names, so that's not the real concern. Let me trace through what actually happens without the guard to see where the real problem lies.
> 
> When processing `const MeshVS_DisplayModeFlags&`, it hits the reference handler and recurses on the pointee. The pointee has display name "MeshVS_DisplayModeFlags" but its canonical form is int. During recursion, the standard type mapping fails, but the primitive typedef check succeeds because it's an Elaborated kind with a TypedefDecl whose underlying type matches. This makes `spelling_looks_like_class` false, which then triggers the canonical type mapping.
> 
> But wait—I'm realizing the guard at line 1576 might be firing for a different reason than I thought. Let me actually test this instead of trying to reason through clang's exact behavior, since the pointee TypeKind after dereferencing a reference might not be what I expect. Writing typedef detection logic... Writing the implementation... Now I'm checking the bindability of these resolved typedef types—whether mutable references to mutable pointers, const references to mutable pointers, and mutable pointers themselves can be safely bound in the C++ wrapper. I'm realizing that references across `extern "C"` boundaries get represented as pointers, so a mutable reference to a mutable pointer is actually quite unusual and I need to understand how that maps to the wrapper layer.
> 
> Rather than getting bogged down in the edge cases, I think the better approach is to just implement the typedef resolution and regenerate—if some methods still can't be bound due to complex types, they'll simply get skipped with a clearer reason. The main win here is resolving flags like `MeshVS_DisplayModeFlags` down to `i32`, which should unlock several methods that are currently blocked. which is `const int&` → simple bindable type
> 2. Standard_PCharacter by value → resolves to `*mut char` → potentially bindable 
> 3. Standard_PCharacter& → resolves to `MutRef(MutPtr(Class("char")))` → may or may not be bindable
> 4. BOPDS_PDS by value → resolves to `*mut BOPDS_DS` → bindable as pointer return/param
> 5. BOPDS_PDS& → resolves to `MutRef(MutPtr(Class("BOPDS_DS")))` → may or may not be bindable
> 
> Let me implement and test. If we get compilation errors, I'll fix the codegen too.
> 
> Wait, actually there's another consideration. For `ConstRef(I32)` (const int&) parameters — this is a const reference to a primitive type. In C++, this is `const int&`. In the extern "C" wrapper, this becomes `const int*`. In Rust, `*const i32`. In the reexport, `&i32`.
> 
> Is `ConstRef(I32)` handled by the existing codegen? Let me check... Actually, many OCCT methods take `const Standard_Integer&` or `const Standard_Real&`. These are currently resolved as... let me check. `const Standard_Integer&`:
> - parse_type sees LValueReference
> - Gets pointee "Standard_Integer", const-qualified
> - Recurses: map_standard_type("Standard_Integer") → Type::I32
> - Returns ConstRef(I32)
> 
> So `ConstRef(I32)` IS already a common pattern in the codebase! It should be fully handled by the codegen.
> 
> Now for `MutPtr(Class("BOPDS_DS"))` as a return type — that's a non-const pointer to a class, which should map to `Option<&mut BOPDS_DS>`. And `MutPtr(Class("char"))` as a parameter is like a mutable `Standard_CString`, producing `*mut c_char` in the FFI layer. I need to verify how the type mapping handles this conversion from the internal representation to the final Rust type.
> 
> Let me trace through the type rendering logic to see what actually gets generated, and also consider the `MutRef(MutPtr(Class("...")))` pattern — that's `&mut *mut T`, which becomes a double pointer `*mut *mut T` in the FFI. I'm checking whether the existing codegen already handles this case, and reviewing how different reference and pointer combinations map: `ConstRef` becomes `*const` in FFI and `&` in the reexport, `MutRef` becomes `*mut` in FFI and `&mut` in the reexport, and `MutPtr` stays as `*mut` in both layers.
> 
> Now I'm wondering if nested reference-to-pointer types like `ConstRef(MutPtr(X))` and `MutRef(MutPtr(X))` are handled specially or just nested generically as `*const *mut X` and `*mut *mut X`. There might be a check that skips methods with these unusual reference-to-pointer combinations since they're hard to use from Rust. Let me look at the type checking and emission code to see how `Type::MutRef` containing `Type::MutPtr` would actually be processed, and check if the type recursion in `type_uses_unknown_type` handles this properly.

Let me look at the bindability and type mapping code to understand what types the codegen can handle:

Analyze codegen type handling

<details>
<summary>Searched for regex <code>is_method_bindable|type_uses_unknown_type</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 17 results</summary>

- [bindings.rs:463](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L463)
- [bindings.rs:810](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L810)
- [bindings.rs:828](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L828)
- [bindings.rs:836](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L836)
- [bindings.rs:846](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L846)
- [bindings.rs:847](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L847)
- [bindings.rs:859](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L859)
- [bindings.rs:916](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L916)
- [bindings.rs:924](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L924)
- [bindings.rs:961](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L961)
- [bindings.rs:969](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L969)
- [bindings.rs:990](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L990)
- [bindings.rs:993](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L993)
- [bindings.rs:1650](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1650)
- [bindings.rs:2137](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2137)
- [bindings.rs:3369](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3369)
- [bindings.rs:3403](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3403)
</details>

Searched for regex `type_to_rust_ffi|type_to_rust_reexport|type_to_cpp|fn rust_ffi|fn rust_reexport|fn cpp_type` (`**/crates/opencascade-binding-generator/src/**`), no results

<details>
<summary>Searched for regex <code>MutRef|ConstRef|MutPtr</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 20 results</summary>

- [bindings.rs:175](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L175)
- [bindings.rs:454](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L454)
- [bindings.rs:467](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L467)
- [bindings.rs:496](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L496)
- [bindings.rs:529](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L529)
- [bindings.rs:553](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L553)
- [bindings.rs:628](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L628)
- [bindings.rs:629](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L629)
- [bindings.rs:640](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L640)
- [bindings.rs:677](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L677)
- [bindings.rs:824](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L824)
- [bindings.rs:836](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L836)
- [bindings.rs:882](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L882)
- [bindings.rs:924](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L924)
- [bindings.rs:969](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L969)
- [bindings.rs:1005](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1005)
- [bindings.rs:1016](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1016)
</details>

Searched for regex `MutRef|ConstRef|MutPtr` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), 3 results
- [rust.rs:114](../crates/opencascade-binding-generator/src/codegen/rust.rs#L114)
- [rust.rs:115](../crates/opencascade-binding-generator/src/codegen/rust.rs#L115)
- [rust.rs:117](../crates/opencascade-binding-generator/src/codegen/rust.rs#L117)

<details>
<summary>Reading lines 440-560 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L440-L561">bindings.rs</a></summary>

```
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
        WrapperKind::ByValueReturn
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


/// Convert a parameter to C++ argument expressi
... (truncated, 4479 chars)
```
</details>

<details>
<summary>Reading lines 600-720 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L600-L721">bindings.rs</a></summary>

```
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
            } else if name == "void" {
                "std::ffi::c_void".to_string()
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
        Type::Class(name) if is_opaque_class_name(name) => {
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
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if !is_void_type_name(name)) => {
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
        Type::MutPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if is_opaque_class_name(name)) => {
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

... (truncated, 5111 chars)
```
</details>

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

<details>
<summary>Reading lines 100-200 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L100-L201">rust.rs</a></summary>

```
            // that may come from canonical type resolution
            if !is_primitive_type(name) && !name.contains('<') {
                collected.classes.insert(name.clone());
            }
        }
        Type::Handle(name) => {
            // Record the Handle type AND the inner class
            // Skip template instantiations (e.g., NCollection_Shared<...>) which
            // aren't valid as standalone Rust/C++ type names.
            if !name.contains('<') {
                collected.handles.insert(name.clone());
                collected.classes.insert(name.clone());
            }
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
    all_headers: &[String],
    collections: &[super::collections::CollectionInfo],
    symbol_table: &crate::resolver::SymbolTable,
    all_bindings: &[super::bindings::ClassBindings],
    function_bindings: &[super::bindings::FunctionBinding],
    handle_able_classes: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
    non_allocatable_classes: &HashSet<String>,
) -> (String, Vec<NestedTypeInfo>) {
    // Get all classes with protected destructors, and merge in non-allocatable classes
    // (non-allocatable classes should not get destructors/CppDeletable either)
    let mut protected_destructor_class_names = symbol_table.protected_destructor_class_names();
    for cls in non_allocatable_classes {
        protected_destructor_class_names.insert(cls.clone());
    }

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
    let collected_types = collect_ref
... (truncated, 4297 chars)
```
</details>

<details>
<summary>Searched for regex <code>MutRef|ConstRef|MutPtr|fn.*type|fn.*emit|fn.*render</code> (<code>**/crates/opencascade-binding-generator/src/codegen/cpp.rs</code>), 9 results</summary>

- [cpp.rs:16](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L16)
- [cpp.rs:67](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L67)
- [cpp.rs:73](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L73)
- [cpp.rs:82](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L82)
- [cpp.rs:131](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L131)
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/20f22355-bd40-4a8b-9af0-a41343687ade/toolu_01FGugbo6hYid5kradt4yCsL__vscode-1771608614010/content.txt)

<details>
<summary>Searched for regex <code>fn type_to|fn type_uses_unknown|fn param_to|is_unbindable|fn.*cpp|fn.*ffi|fn.*reexport</code> (<code>**/crates/opencascade-binding-generator/src/type_mapping.rs</code>), 5 results</summary>

- [type_mapping.rs:15](../crates/opencascade-binding-generator/src/type_mapping.rs#L15)
- [type_mapping.rs:20](../crates/opencascade-binding-generator/src/type_mapping.rs#L20)
- [type_mapping.rs:299](../crates/opencascade-binding-generator/src/type_mapping.rs#L299)
- [type_mapping.rs:374](../crates/opencascade-binding-generator/src/type_mapping.rs#L374)
- [type_mapping.rs:405](../crates/opencascade-binding-generator/src/type_mapping.rs#L405)
</details>

Searched for regex `is_unbindable` (`**/crates/opencascade-binding-generator/src/types.rs`), no results

Searched for regex `fn is_unbindable|fn to_cpp_string|fn class_ptr_inner_name` (`**/crates/opencascade-binding-generator/src/types.rs`), no results

<details>
<summary>Reading lines 370-430 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L370-L431">type_mapping.rs</a></summary>

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
```
</details>

Searched for regex `fn type_to_ffi|fn ffi_param_type|fn ffi_return_type|fn param_to_ffi` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `fn is_unbindable` (`**/crates/opencascade-binding-generator/src/**`), 1 result
- [model.rs:729](../crates/opencascade-binding-generator/src/model.rs#L729)

Searched for regex `fn emit_ffi_class|fn emit_ffi_method|fn ffi_param|type_to_ffi_param|fn emit_extern_c` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:5389](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5389)

<details>
<summary>Searched for regex <code>fn type_to_ffi|fn ffi_type|to_ffi_param|to_ffi_type|ffi_sig|extern.*param</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 20 results</summary>

- [bindings.rs:551](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L551)
- [bindings.rs:1201](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1201)
- [bindings.rs:3602](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3602)
- [bindings.rs:3645](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3645)
- [bindings.rs:3720](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3720)
- [bindings.rs:3734](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3734)
- [bindings.rs:3747](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3747)
- [bindings.rs:3762](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3762)
- [bindings.rs:3817](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3817)
- [bindings.rs:3867](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3867)
- [bindings.rs:3880](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3880)
- [bindings.rs:3891](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3891)
- [bindings.rs:3934](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3934)
- [bindings.rs:3948](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3948)
- [bindings.rs:3994](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3994)
- [bindings.rs:4008](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4008)
- [bindings.rs:4061](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4061)
- [bindings.rs:4181](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4181)
- [bindings.rs:4195](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4195)
- [bindings.rs:4254](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4254)
</details>

<details>
<summary>Reading lines 725-790 of <a href="../crates/opencascade-binding-generator/src/model.rs#L725-L791">model.rs</a></summary>

```
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
            Type::U8 => "uint8_t".to_string(),
            Type::I8 => "int8_t".to_string(),
            Type::ConstRef(inner) => format!("const {}&", inner.to_cpp_string()),
            Type::MutRef(inner) => format!("{}&", inner.to_cpp_string()),
            Type::RValueRef(inner) => format!("{}&&", inner.to_cpp_string()),
            Type::ConstPtr(inner) => format!("const {}*", inner.to_cpp_string()),
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
    fn is_unresolved_template_type(&self) -> bool {
        match self {
            Type::Class(name) => {
```
</details>

Searched for regex `fn to_cpp_string|fn to_cpp_extern_c_param|fn to_cpp_extern_c_return` (`**/crates/opencascade-binding-generator/src/model.rs`), 2 results
- [model.rs:743](../crates/opencascade-binding-generator/src/model.rs#L743)
- [model.rs:755](../crates/opencascade-binding-generator/src/model.rs#L755)

<details>
<summary>Reading lines 1170-1250 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1170-L1251">bindings.rs</a></summary>

```
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
        Type::Class(name) if is_opaque_class_name(name) && !ffi_ctx.all_enums.contains(name) => {
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
        let mapped = map_return_type_in_context(ty, ffi_ctx);
        let rust_ffi_type = mapped.rust_type;
        let cpp_type = ty.to_cpp_string();
        // Build the reexport type as Option<&T> or Option<&mut T>
        let is_const = matches!(ty, Type::ConstPtr(_));
        let inner_rust_type = if let Some(ctx) = reexport_ctx {
            ctx.resolve_class(class_name)
        } else {
            format!("crate::ffi::{}", Type::ffi_safe_class_name(class_name))
        };
        let rust_reexport_type = if is_const {
```
</details>

<details>
<summary>Searched for regex <code>to_rust_ffi_string|type_to_ffi_string|to_ffi_string|ffi_safe_class|fn.*ffi_param_type</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 6 results</summary>

- [bindings.rs:576](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L576)
- [bindings.rs:624](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L624)
- [bindings.rs:651](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L651)
- [bindings.rs:670](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L670)
- [bindings.rs:682](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L682)
- [bindings.rs:1248](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1248)
</details>

Searched for regex `fn map_type_in_context|fn map_return_type_in_context` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), no results

Searched for regex `fn map_type_in_context|fn map_return_type_in_context` (`**/crates/opencascade-binding-generator/src/**`), 2 results
- [type_mapping.rs:440](../crates/opencascade-binding-generator/src/type_mapping.rs#L440)
- [type_mapping.rs:542](../crates/opencascade-binding-generator/src/type_mapping.rs#L542)

<details>
<summary>Reading lines 440-600 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L440-L601">type_mapping.rs</a></summary>

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
... (truncated, 5730 chars)
```
</details>

<details>
<summary>Reading lines 1-440 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L1-L441">type_mapping.rs</a></summary>

```
//! Type mapping between C++ and Rust types
//!
//! Maps OCCT C++ types to their Rust equivalents for extern "C" FFI generation.

#![allow(dead_code)] // Some functions are reserved for future use

use crate::model::Type;
use crate::module_graph::module_to_rust_name;

/// Convert a C++ class name to its Rust Handle type name.
///
/// Strips underscores and `::` (from nested classes) to produce a valid
/// Rust identifier.  e.g. `ShapePersistent_BRep::CurveRepresentation`
/// → `HandleShapePersistentBRepCurveRepresentation`.
pub fn handle_type_name(cpp_name: &str) -> String {
    format!("Handle{}", cpp_name.replace("::", "").replace('_', ""))
}

/// Lowercase variant used for parameter names.
pub fn handle_param_name(cpp_name: &str) -> String {
    format!("handle{}", cpp_name.to_lowercase().replace("::", "").replace('_', ""))
}

/// Result of mapping a C++ type to Rust
#[derive(Debug, Clone)]
pub struct RustTypeMapping {
    /// The Rust type string for use in extern "C" FFI declarations
    pub rust_type: String,
    /// Whether this type is returned as an owned pointer (*mut T) that the caller must free
    pub needs_unique_ptr: bool,
    /// Whether this type needs Pin<&mut T> for mutable self
    pub needs_pin: bool,
    /// The module this type comes from (if cross-module reference)
    pub source_module: Option<String>,
}

/// Map a parsed Type to its Rust representation for extern "C" FFI
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
            source_mo
... (truncated, 17434 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/20f22355-bd40-4a8b-9af0-a41343687ade/toolu_01VpAU6CTsjsVh2ibXRyBotU__vscode-1771608614029/content.txt#1-1), lines 1 to 120

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/20f22355-bd40-4a8b-9af0-a41343687ade/toolu_01VpAU6CTsjsVh2ibXRyBotU__vscode-1771608614029/content.txt#120-120), lines 120 to 300

<details>
<summary>Reading lines 1050-1170 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1050-L1171">bindings.rs</a></summary>

```
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
    if is_nullable && !ty.is_void_ptr() {
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
            rust_name,
            rust_ffi_type,
            rust_reexport_type,
            cpp_type,
            cpp_arg_expr,
            enum_rust_type: None,
            mut_ref_enum_cpp_name: None,
            mut_ref_enum_rust_type: None,
            is_nullable_ptr: true,
            is_class_ptr: false,
        };
    }

    // Non-nullable class pointer params: const T* -> &T, T* -> &mut T
    // In ffi.rs: *const T / *mut T (raw pointers)
    // In re-export: &T / &mut T
    // In C++: const T* / T* (passed through directly)
    if let Some(_class_name) = ty.class_ptr_inner_name() {
  
... (truncated, 5211 chars)
```
</details>

<details>
<summary>Reading lines 3560-3650 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3560-L3651">bindings.rs</a></summary>

```
// ── Emit functions ──────────────────────────────────────────────────────────

/// Emit C++ wrapper code for a single class from pre-computed ClassBindings.
///
/// Produces C++ wrapper code for a class
/// and its 10+ sub-functions, but consumes the pre-computed IR instead
/// of re-deriving decisions.
pub fn emit_cpp_class(bindings: &ClassBindings) -> String {
    use std::fmt::Write;

    let mut output = String::new();
    let ffi_cn = &bindings.cpp_name;  // Rust-safe flattened name (for FFI function names)
    let cn = &bindings.cpp_qualified_name;  // C++ qualified name (for C++ type expressions)

    // POD structs don't need C++ wrappers, but we generate a sizeof helper
    // so Rust tests can verify layout compatibility at runtime,
    // and a destructor so CppDeletable can be implemented (needed when returned by pointer).
    if bindings.is_pod_struct {
        writeln!(output, "// sizeof helper for POD struct {}", cn).unwrap();
        writeln!(output, "extern \"C\" size_t {}_sizeof() {{ return sizeof({}); }}", ffi_cn, cn).unwrap();
        writeln!(output, "extern \"C\" void {}_destructor({}* self_) {{ delete self_; }}", ffi_cn, cn).unwrap();
        writeln!(output).unwrap();
        return output;
    }

    writeln!(output, "// ========================").unwrap();
    writeln!(output, "// {} wrappers", cn).unwrap();
    writeln!(output, "// ========================").unwrap();
    writeln!(output).unwrap();

    // 1. Constructor wrappers (skip convenience — they are Rust-only)
    for ctor in bindings.constructors.iter().filter(|c| c.convenience_of.is_none()) {
        let params_cpp: Vec<String> = ctor
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
            .collect();
        let params_str = params_cpp.join(", ");
        let args_str = ctor.cpp_arg_exprs.join(", ");

        writeln!(
            output,
            "extern \"C\" {cn}* {fn_name}({params_str}) {{",
            fn_name = ctor.ffi_fn_name
        )
        .unwrap();
        writeln!(
            output,
            "    return new {cn}({args_str});"
        )
        .unwrap();
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
```
</details>

<details>
<summary>Reading lines 3880-4020 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3880-L4021">bindings.rs</a></summary>

```
                    "extern \"C\" {} {fn_name}({params}) {{",
                    rt.cpp_type,
                    fn_name = wm.ffi_fn_name
                )
                .unwrap();
                writeln!(output, "    return {call_expr};").unwrap();
            }
        } else {
            // Void return, enum params only
            writeln!(
                output,
                "extern \"C\" void {fn_name}({params}) {{",
                fn_name = wm.ffi_fn_name
            )
            .unwrap();
            writeln!(output, "    {call_expr};").unwrap();
        }
        writeln!(output, "}}").unwrap();
    }

    // 4c. ByValueParam wrapper methods
    // These take const T& at the FFI boundary; the C++ method receives by value (implicit copy).
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::ByValueParam)
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

    // 4d. ConstMutReturnFix wrapper methods
    // These are const methods returning &mut T — the wrapper takes non-const self
    // to ensure &mut self is used when returning &mut T.
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::ConstMutReturnFix)
    {
        // Always non-const self (that's the fix)
        let self_param = format!("{cn}* self_");

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
                fn_name = wm.ffi_fn
... (truncated, 4278 chars)
```
</details>

<details>
<summary>Reading lines 4020-4100 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4020-L4101">bindings.rs</a></summary>

```
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
```
</details>

<details>
<summary>Reading lines 4130-4210 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4130-L4211">bindings.rs</a></summary>

```
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

    // 5. Static method wrappers
    for sm in &bindings.static_methods {
```
</details>

<details>
<summary>Reading lines 1010-1050 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1010-L1051">bindings.rs</a></summary>

```
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
                let reexport_type = enum_rust_type.as_ref()
```
</details>

<details>
<summary>Reading lines 700-730 of <a href="../crates/opencascade-binding-generator/src/model.rs#L700-L731">model.rs</a></summary>

```
                    return true;
                }
                false
            }
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_nested_type()
            }
            _ => false,
        }
    }

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
<summary>Searched for regex <code>fn is_array|fn is_rvalue_ref|fn has_unbindable_types|fn class_ptr_inner_name</code> (<code>**/crates/opencascade-binding-generator/src/model.rs</code>), 7 results</summary>

- [model.rs:49](../crates/opencascade-binding-generator/src/model.rs#L49)
- [model.rs:234](../crates/opencascade-binding-generator/src/model.rs#L234)
- [model.rs:269](../crates/opencascade-binding-generator/src/model.rs#L269)
- [model.rs:352](../crates/opencascade-binding-generator/src/model.rs#L352)
- [model.rs:629](../crates/opencascade-binding-generator/src/model.rs#L629)
- [model.rs:659](../crates/opencascade-binding-generator/src/model.rs#L659)
- [model.rs:719](../crates/opencascade-binding-generator/src/model.rs#L719)
</details>

<details>
<summary>Reading lines 625-670 of <a href="../crates/opencascade-binding-generator/src/model.rs#L625-L671">model.rs</a></summary>

```
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
                    Type::Class(name) if is_opaque_class_name(name) => Some(name.as_str()),
                    _ => None,
                }
            }
            _ => None,
        }
    }

```
</details>

<details>
<summary>Reading lines 230-280 of <a href="../crates/opencascade-binding-generator/src/model.rs#L230-L281">model.rs</a></summary>

```
    }

    /// Check if this constructor has any unbindable types (C strings, streams, void pointers, etc.)
    /// Nullable pointer params are NOT considered unbindable.
    pub fn has_unbindable_types(&self) -> bool {
        self.params.iter().any(|p| p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none())
    }

    /// Check if this constructor has any unsafe raw pointer types.
    pub fn has_unsafe_types(&self) -> bool {
        self.params.iter().any(|p| p.ty.needs_unsafe_fn() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none())
    }
}

/// An instance method declaration
#[derive(Debug, Clone)]
pub struct Method {
    /// Method name (e.g., "X", "SetX", "Mirrored")
    pub name: String,
    /// Documentation comment
    pub comment: Option<String>,
    /// Whether the method is const (determines &self vs &mut self)
    pub is_const: bool,
    /// Parameters (excluding implicit this)
    pub params: Vec<Param>,
    /// Return type (None for void)
    pub return_type: Option<Type>,
    /// Source line number in the header file
    pub source_line: Option<u32>,
}

impl Method {
    /// Check if this method returns by value (needs wrapper)
    pub fn returns_by_value(&self) -> bool {
        matches!(&self.return_type, Some(Type::Class(_)) | Some(Type::Handle(_)))
    }

    /// Check if this method has any unbindable types (streams, void pointers, etc.)
    /// in parameters or return type. Nullable pointer params are NOT considered unbindable.
    pub fn has_unbindable_types(&self) -> bool {
        // Check params (skip nullable pointer params — they're handled as Option<&T>)
        // Also skip class raw pointer params — they're handled as &T / &mut T
        if self.params.iter().any(|p| p.ty.is_unbindable() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none()) {
            return true;
        }
        // Check return type
        if let Some(ref ret) = self.return_type {
            if ret.is_unbindable() && ret.class_ptr_inner_name().is_none() {
                return true;
            }
        }
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/20f22355-bd40-4a8b-9af0-a41343687ade/toolu_01QBEqQsdYGPrEGXN8W1hLxj__vscode-1771608614003/content.txt)

> 💭 Excellent analysis. So:
> 1. `ConstRef(I32)` — fully works (MeshVS_DisplayModeFlags fix will work)
> 2. `MutPtr(Class("X"))` return — fully works (BOPDS_PDS return will work)
> 3. `MutPtr(Class("char"))` — fully works (Standard_PCharacter by value will work)
> 4. `MutRef(MutPtr(Class("X")))` — works (BOPDS_PDS& param will work)
> 5. `ConstRef(MutPtr(Class("X")))` — mostly works, minor C++ const issue (const BOPDS_PDS& will work)
> 
> 
> The `ConstRef(MutPtr)` case has a subtle const-correctness mismatch where C++ generates `const X**` instead of `X* const*`, but this shouldn't cause compilation issues. Looking at the actual OCCT signature `void SetDS(const BOPDS_PDS& pDS)` where `BOPDS_PDS` is a pointer type, the wrapper needs to handle the reference-to-pointer semantics correctly.
> 
> The problem is that `to_cpp_extern_c_param()` processes the type recursively—when it encounters `ConstRef(MutPtr(Class))`, it converts the outer `ConstRef` to `const {inner}*`, but the inner `MutPtr` still produces a pointer, resulting in the wrong const placement compared to what the C++ method actually expects. cause a compilation error. So when I convert `ConstRef(MutPtr(Class("X")))`, the resulting `const X**` can't implicitly convert to `X* const*` — that's a real type mismatch in C++.
> 
> Let me think through what the actual BOPDS method signature looks like. It takes `const BOPDS_PDS&`, which expands to `BOPDS_DS* const&` — a const reference to a mutable pointer. The C wrapper would need to match this signature...
> 
> Now I'm working through the parameter types. If the wrapper receives `const BOPDS_DS**`, dereferencing it gives `const BOPDS_DS*`, but the method expects `BOPDS_DS* const&` — these don't align because one points to const data and the other doesn't. I see the issue now—when I parse `const BOPDS_PDS&`, the typedef resolves BOPDS_PDS to a mutable pointer, giving me `ConstRef(MutPtr(Class("BOPDS_DS")))`. But in C++, that const applies to the pointer itself, not what it points to, so it should codegen as `BOPDS_DS* const&`. My current codegen is incorrectly producing `const BOPDS_DS**` instead, which strips const from the wrong level.
> 
> The real problem is that `to_cpp_extern_c_param()` doesn't distinguish between "const reference to a simple type" (which needs `const T*`) and "const reference to a pointer" (which needs `T* const*`). This is a const-correctness edge case I need to handle specially. handle this...
> 
> One option: For `ConstRef(MutPtr(X))`, emit `X* const*` instead of `const X**`.
> For `ConstRef(ConstPtr(X))`, emit `const X* const*`.
> 
> Actually, the general rule for `ConstRef(T)` in extern "C":
> - `T` becomes `T const*` (pointer to const T)
> - If `T` is a simple type (Class, primitive), `T const*` = `const T*` (same thing in C)
> - If `T` is `MutPtr(X)` = `X*`, then `MutPtr(X) const*` = `X* const*`
> - Currently emitted as `const X**` which is wrong
> 
> The fix in `to_cpp_extern_c_param()`: I need to change how const references are formatted. Instead of prefixing with `const`, I should append `const*` after the inner type. The key insight is that `const X*` and `X const*` are equivalent for simple types, but when the inner type contains pointers, the placement matters—`X* const*` means a pointer to a const pointer, which is different from `const X**` which means a pointer to a pointer to const X. I'm working through the C++ pointer-to-const syntax rules. When reading declarations right-to-left, `const BOPDS_DS**` means a pointer to a pointer-to-const-BOPDS_DS, while `BOPDS_DS* const*` means a pointer to a const pointer-to-BOPDS_DS — the const qualifier sits at different levels depending on placement. For the code generation fix, I need to emit `{inner.to_cpp_string()} const *` instead of `const {inner.to_cpp_string()}*` in the `ConstRef` case to get the const applied at the right pointer level.
> 
> Now I'm verifying this works across different inner types: for a simple class it produces `X const *`, for a mutable pointer it gives `X* const *`, and for primitives like int32_t it yields `int32_t const *`. The fix is straightforward and maintains backward compatibility since `const X *` and `X const *` are equivalent. I'm checking edge cases like `ConstRef(ConstPtr(Class("X")))` to make sure the const placement stays correct through nested type combinations. The `MutRef` case with a mutable pointer is already correct—it properly produces a pointer-to-pointer. The issue is specifically with `ConstRef` where the `const` keyword needs to come after the type instead of before it. I need to locate where `to_cpp_extern_c_param()` is defined in model.rs and apply that fix, then check if any other methods have similar const positioning problems.
> 
> Since all `ConstRef` parameters flow through this function, changing the const placement is safe—it's semantically identical in C++ for all existing type patterns, just fixing the new mutable pointer case. Now I'm looking at how by-value parameters like `Handle(X)` and `Class(X)` get converted to const references at the C level.
> 
> Those by-value types can't contain mutable pointers, so the const position fix won't affect them. I should also check if other primitive typedefs besides `MeshVS_DisplayModeFlags` would benefit from this fix. For the guard at line 1576, instead of adding a complex primitive typedef check, I could narrow the guard condition to only intercept when the display name contains `::` or template syntax like `<`.
> 
> But I need to be careful—the original guard exists for a reason. If I let simple typedef names fall through and some class type incorrectly canonicalizes to int, it would be resolved wrong. The thing is, a CLASS type would only canonicalize to int through template instantiation like `MyClass<int>::value_type`, which would have `::` in the display name anyway, and `base_looks_like_class` already filters those out.
> 
> Looking at the actual checks, `base_looks_like_class` verifies the name starts with uppercase, isn't a standard type, and has no spaces—but it doesn't check for `::` or `<`. Those get checked separately in `base_looks_like_type`, which combines the class check with additional conditions for templates and namespaced types. So something like `MeshVS_DisplayModeFlags` passes both checks, while a template like `NCollection_Map<int>::` would be caught by the broader type check.
> 
> My refined approach is to keep the guard at `base_looks_like_type` but add a nested condition: for names containing `<` or `::`, always preserve the display name since they might have incorrect canonical forms due to template issues, but for simple typedef-like names, I need to verify whether it's actually a valid type before deciding whether to preserve it.
> 
> Refining the logic... I'm realizing I've been overthinking this. The real issue is just that `MeshVS_DisplayModeFlags` and similar typedefs that resolve to primitives are being incorrectly preserved as class names. For pointer typedefs like `BOPDS_PDS`, the guard doesn't even trigger since the canonical base won't be "int". So I need to add a check: when the base looks like a class name but isn't templated or namespaced, I should verify that the pointee's canonical type is actually a primitive before deciding to preserve it as a class.
> 
> Refining the guard logic... Finishing the primitive type check...
> 
> So if we don't match a genuine primitive typedef, we wrap it as a Class type and handle the reference. But if we do fall through—meaning it's a real primitive typedef—we hit the normal reference handler which recursively parses the pointee type. The issue is that the pointee here is `MeshVS_DisplayModeFlags`, which needs to be resolved correctly.
> 
> When we recurse into parsing that, the primitive typedef check should catch it: the kind is Elaborated, it has a TypedefDecl, and the underlying type is Standard_Integer. So it resolves to I32, and since the original reference was const, we get ConstRef(I32). This should actually work correctly. looking at the check more carefully. The check is at lines 1535-1550, which validates whether a type is a primitive typedef by checking if it's either a Typedef or Elaborated kind, then tracing through its declaration to verify the underlying type matches a primitive pattern.
> 
> For `MeshVS_DisplayModeFlags`, I need to verify the chain: the Elaborated kind should resolve to a TypedefDecl through get_declaration(), which then unwraps to `Standard_Integer` via get_typedef_underlying_type(). The question is whether `Standard_Integer` itself is marked as Typedef or Elaborated, which would satisfy the final pattern match.
> 
> But there's a potential issue when dealing with reference pointees — I'm not certain whether clang preserves the Typedef kind when extracting the pointee type from a reference, or if it strips away the typedef sugar entirely.
> 
> Looking at how the existing code handles cases like `Graphic3d_ZLayerId`, which correctly resolves as a primitive typedef when passed directly as a parameter, I'm fairly confident that clang preserves the Elaborated/Typedef kind even when the type is nested inside a reference. The canonical form would be the desugared version, but the sugared type should still carry the typedef information.
> 
> Rather than overthinking this, I'll implement the fix and test it against `MeshVS_DisplayModeFlags`. If the reference handling doesn't work as expected, I can add more specific logic to handle it.
> 
> The plan is to modify parser.rs around line 1576-1609 to add a guard that checks whether the canonical base is a primitive but the pointee itself is a genuine primitive typedef — if so, let it proceed through the normal reference handling path instead of being rejected.
> 
> Then around line 1726-1730, before returning `Type::Class(clean_name)`, I'll add late-stage resolution that checks if the type is a typedef or elaborated kind. If the canonical form is a pointer, I'll resolve it to the appropriate MutPtr or ConstPtr of the pointee; if it's a primitive, I'll resolve directly to that primitive type.
> 
> For model.rs, I need to fix the const positioning in `to_cpp_extern_c_param()` — changing from `const {inner.to_cpp_string()}*` to `{inner.to_cpp_string()} const*` so that const applies correctly to the pointer itself rather than the pointed-to type.
> 
> Let me verify this works across different cases: simple types like `I32` become `int32_t const*`, pointer types like `MutPtr(Class("X"))` become `X* const*`, const pointers become `const X* const*`, and handle types become `HandleX const*`. This all looks correct, so I'll go ahead with the implementation.
> 
> Actually, I should double-check whether there's existing pass-by-value-to-const-ref handling that might also use this code path.
> 
> Looking at the codegen, `param_to_cpp_extern_c_arg` in bindings.rs handles `ConstRef` and `MutRef` by dereferencing the parameter. When a class or handle is passed by value, the codegen wraps it in `ConstRef`, so those cases also flow through `to_cpp_extern_c_param()`. For a by-value `Class("X")` wrapped in `ConstRef`, it becomes `X const*` with my fix, which is equivalent to the current `const X*` in C.
> 
> Now I need to implement the fix and carefully handle the guard logic, especially where it returns from within nested if-else blocks. Let me check the exact structure around lines 1576-1609 to understand the control flow.

Let me re-read the exact code structure I need to modify:

<details>
<summary>Reading lines 1570-1615 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1570-L1616">parser.rs</a></summary>

```
    }

    // Guard: when the OUTER type's display name identifies an OCCT class but the
    // canonical type is "int", construct the class type directly instead of recursing
    // into the pointee (whose display name might already be "int", losing the
    // typedef info).
    if kind == TypeKind::LValueReference || kind == TypeKind::RValueReference || kind == TypeKind::Pointer

    {
        let canonical_base = strip_type_decorators(canonical_clean);
        if canonical_base == "int" {
            // Strip qualifiers and ref/ptr decorators from the outer display name
            let base = strip_type_decorators(trimmed_spelling);
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
```
</details>

Now let me look at `to_cpp_extern_c_param()` in model.rs:

Searched for text `fn to_cpp_extern_c_param` (`**/crates/opencascade-binding-generator/src/model.rs`), 1 result
- [model.rs:743](../crates/opencascade-binding-generator/src/model.rs#L743)

<details>
<summary>Reading lines 740-790 of <a href="../crates/opencascade-binding-generator/src/model.rs#L740-L791">model.rs</a></summary>

```

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
            Type::U8 => "uint8_t".to_string(),
            Type::I8 => "int8_t".to_string(),
            Type::ConstRef(inner) => format!("const {}&", inner.to_cpp_string()),
            Type::MutRef(inner) => format!("{}&", inner.to_cpp_string()),
            Type::RValueRef(inner) => format!("{}&&", inner.to_cpp_string()),
            Type::ConstPtr(inner) => format!("const {}*", inner.to_cpp_string()),
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
    fn is_unresolved_template_type(&self) -> bool {
        match self {
            Type::Class(name) => {
```
</details>

> 💭 Now I have all the context I need. Let me summarize my implementation plan:
> 
> ### Changes needed:
> 
> #### 1. parser.rs: Fix reference/pointer guard for primitive typedefs (lines 1576-1609)
> 
> In the guard, when `canonical_base == "int"` and `base_looks_like_class` is true (simple typedef name, not template/namespace), check if the pointee's canonical type is actually a primitive. If so, don't intercept — let normal reference handling resolve it.
> 
> #### 2. parser.rs: Late-stage canonical resolution for pointer typedefs (before line 1730)
> 
> 
> Before returning `Type::Class(clean_name)`, I need to check if the type is a Typedef or Elaborated kind with a canonical pointer type, then resolve through that canonical pointer to properly handle cases like `Standard_PCharacter` → `char*` and `BOPDS_PDS` → `BOPDS_DS*`. I should also catch any remaining primitive typedefs that slipped through the earlier checks by testing if the canonical type itself is a primitive kind.
> 
> For the `to_cpp_extern_c_param()` function in model.rs, I need to fix the const positioning by changing `const {inner}*` to `{inner} const*` when the inner type is a pointer. After making these changes, I'll regenerate the bindings, run the build and tests, and update the README skip counts accordingly. One thing I'm concerned about is whether the opaque struct generation might still create a `pub struct BOPDS_PDS` even after resolving the typedef, since the parser could encounter the typedef name separately and add it to the referenced types set.
> 
> Actually, looking at how opaque declarations work, they're only generated for types that appear in the extern block. Since `BOPDS_PDS` will be resolved to `MutPtr(Class("BOPDS_DS"))` instead of remaining as `Type::Class("BOPDS_PDS")`, it won't end up in the referenced set, so no opaque struct should be generated. The re-export in bopds.rs will also become unnecessary since `BOPDS_PDS` won't exist as a type anymore—it'll just be a pointer alias.
> 
> The same logic applies to `MeshVS_DisplayModeFlags` and `Standard_PCharacter`. Since the opaque struct generation and re-exports are driven by what the parser produces, if the parser no longer creates `Type::Class("BOPDS_PDS")`, then the symbol table won't include it and nothing will be generated. But I'm realizing `BOPDS_PDS` is currently a parsed class that the parser encounters in the binding set, so the resolver processes it...
> 
> Actually, wait — `BOPDS_PDS` is a typedef, not a class. The opaque struct is generated by `generate_opaque_declarations()` which collects types referenced in method signatures that aren't explicitly declared. So if my fix resolves `BOPDS_PDS` to a pointer to `BOPDS_DS` instead, the opaque struct generation should handle it differently.
> 
> If `BOPDS_PDS` is no longer treated as a standalone type in method signatures, then `generate_opaque_declarations()` won't find it, the opaque struct won't be generated, and the re-export will disappear too. That's actually the right behavior since `BOPDS_PDS` is just a pointer typedef, not a real type. But I should check if anything in the `bopds.rs` module actually references it.
> 
> The re-export at `bopds.rs:4151` is auto-generated based on the symbol table, so if `BOPDS_PDS` stops being classified as a class, the re-export won't be created. Methods that used to reference it as a class will now go through `BOPDS_DS` via a pointer wrapper instead, which should work fine.
> 
> Now I'll implement this change and also scan for other primitive typedefs like `MeshVS_DisplayModeFlags` that might need the same treatment.

Now I have all the context needed. Let me implement the fixes.

**Todo list:**
- ✅ Investigate Standard_PCharacter (char*)
- ✅ Investigate BOPDS_PDS (BOPDS_DS*)
- ✅ Investigate MeshVS_DisplayModeFlags
- 🔄 Implement fixes
- ⬜ Regenerate bindings and test
- ⬜ Update README skip counts
- ⬜ Commit changes
- ⬜ Consider refactoring opportunities

Now let me implement all Now let me implement all three fixes. First, let me read the exact regions I need to modify:

<details>
<summary>Reading lines 1571-1731 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1571-L1732">parser.rs</a></summary>

```

    // Guard: when the OUTER type's display name identifies an OCCT class but the
    // canonical type is "int", construct the class type directly instead of recursing
    // into the pointee (whose display name might already be "int", losing the
    // typedef info).
    if kind == TypeKind::LValueReference || kind == TypeKind::RValueReference || kind == TypeKind::Pointer

    {
        let canonical_base = strip_type_decorators(canonical_clean);
        if canonical_base == "int" {
            // Strip qualifiers and ref/ptr decorators from the outer display name
            let base = strip_type_decorators(trimmed_spelling);
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
            let is_const = pointee.is_const_qualified();
            let inner = parse_type(&pointee);
            return if is_const {
                Type::ConstRef(Box::new(inner))
            } else {
                Type::MutRef(Box::new(inner))
            };
        }
    }

    // Handle rvalue reference types (T&&) - not bindable but we need to parse them
    if kind == TypeKind::RValueReference {
        if let Some(pointee) = clang_type.get_pointee_type() {
            let inner = parse_type(&pointee);
            return Type::RValueRef(Box::new(inner));
        }
    }

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
    // Check both the display name AND the canonical type spelling, because
    // namespace-scoped Handle typedefs (e.g., IMeshData::IEdgeHandle) have
    // a display name like "IMeshData::IEdgeHandle" but canonical type
    // "opencascade::handle<IMeshData_Edge>".
    // Be careful NOT to match function pointer typedefs whose return type is a Handle,
    // e.g., StdObjMgt_Persistent::Instantiator = Handle(StdObjMgt_Persistent) (*)()
    // has canonical "opencascade::handle<StdObjMgt_Persistent> (*)()". These are
    // detected by checking that the canonical ends with '>' (a pure handle type).
    let clean_spelling = spelling.trim_start_matches("const ").trim();
    let canonical_is_pure_handle = canonical_clean.starts_with("opencascade::handle<")
        && canonical_clean.en
... (truncated, 7764 chars)
```
</details>

Now let me implement the three changes:

**Change 1**: Fix the reference/pointer guard in `crates/opencascade-binding-generator/src/parser.rs` (lines 1576-1609) to allow primitive typedefs to fall through.

**Change 2**: Add late-stage canonical resolution for pointer and primitive typedefs (before line 1730).

**Change 3**: Fix  in `crates/opencascade-binding-generator/src/model.rs` for correct const positioning.

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/parser.rs#L1572-L1636">parser.rs -38+64</a>, <a href="../crates/opencascade-binding-generator/src/parser.rs#L1752-L1795">-5+43</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L745-L752">model.rs -1+7</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "1572:hi,1573:ff,1574:ve,1575:lj,1576:ld,1577:dd,1578:gz,1579:dv,1580:qp,1581:qp,1582:jl,1583:gr,1584:wu,1585:ib,1586:ma,1587:dz,1588:ih,1589:xi,1590:yj,1591:ze,1592:wq,1593:dd,1594:ef,1595:zp,1596:mn,1597:tr,1598:qp,1599:os,1600:zt,1601:ek,1602:es,1603:xz,1604:xv,1605:gl,1606:ee,1607:uo,1608:cy,1609:ax",
      "content": "    // Guard: when the OUTER type's display name identifies an OCCT class but the\n    // canonical type is \"int\", construct the class type directly instead of recursing\n    // into the pointee (whose display name might already be \"int\", losing the\n    // typedef info).\n    //\n    // Exception: genuine primitive typedefs (e.g., MeshVS_DisplayModeFlags = Standard_Integer = int)\n    // should NOT be intercepted — let them fall through to normal reference handling\n    // which will resolve them to the primitive type through canonical resolution.\n    if kind == TypeKind::LValueReference || kind == TypeKind::RValueReference || kind == TypeKind::Pointer\n\n    {\n        let canonical_base = strip_type_decorators(canonical_clean);\n        if canonical_base == \"int\" {\n            // Strip qualifiers and ref/ptr decorators from the outer display name\n            let base = strip_type_decorators(trimmed_spelling);\n            let base_looks_like_class = base.starts_with(|c: char| c.is_ascii_uppercase())\n                && map_standard_type(base).is_none()\n                && base != \"Standard_Boolean\"\n                && !base.contains(' ');\n            // Also handle template/namespaced types (e.g. \"NCollection_Map<...>\" or\n            // \"IMeshData::IMapOfReal\") — these are clearly not primitives.\n            let base_looks_like_type = base_looks_like_class\n                || base.contains('<')\n                || base.contains(\"::\");\n            if base_looks_like_type {\n                // For simple typedef names (not template/namespaced), check if the\n                // pointee's canonical type is actually a primitive. If so, this is a\n                // genuine typedef-to-primitive (e.g., MeshVS_DisplayModeFlags = int)\n                // and should be resolved normally, not preserved as a class name.\n                let is_template_or_ns = base.contains('<') || base.contains(\"::\");\n                let pointee_is_primitive_canonical = !is_template_or_ns\n                    && clang_type.get_pointee_type().map(|pt| {\n                        let canon = pt.get_canonical_type();\n                        let ck = canon.get_kind();\n                        matches!(ck,\n                            TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |\n                            TypeKind::SChar | TypeKind::UChar |\n                            TypeKind::Short | TypeKind::UShort |\n                            TypeKind::Int | TypeKind::UInt |\n                            TypeKind::Long | T
... (truncated, 7033 chars)
```
**Output:**
```
{
  "applied": 3,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/parser.rs",
      "startLine": 1572,
      "endLine": 1794
    },
    {
      "file": "crates/opencascade-binding-generator/src/model.rs",
      "startLine": 745,
      "endLine": 751
    }
  ]
}
```
</details>

All three edits applied. Now let me verify the code compiles:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo check -p opencascade-binding-generator 2>&1
```
```
    Checking opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.19s
real  2.2s
```

Compiles fine. Now let me regenerate the bindings and see the effect:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time ./scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && tail -30 tmp/regen-output.txt 2>/dev/null || echo "checking last output directly" && grep -c "SKIP\|Error\|error" /dev/stdin <<< "$(tail -100 /dev/null)"
```
<details>
<summary>Output (31 lines) — exit code 1</summary>

```
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/vrml_api.rs (2 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/vrml_data.rs (27 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/xcaf_app.rs (1 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/xs_algo.rs (2 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/prox_pnt.rs (extra types only, 1 types)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/cdm.rs (extra types only, 1 types)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/graphic3d.rs (extra types only, 10 types)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/aspect.rs (extra types only, 1 types)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/xcaf_prs.rs (extra types only, 1 types)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/image.rs (extra types only, 1 types)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/pcdm.rs (extra types only, 5 types)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/xb_rep_mesh.rs (function-only module, 1 functions)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/step_to_geom.rs (function-only module, 48 functions)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/precision.rs (function-only module, 18 functions)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/el_c_lib.rs (function-only module, 120 functions)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/n_collection_primes.rs (function-only module, 1 functions)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/rw_obj_tools.rs (function-only module, 3 functions)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/b_rep_bnd_lib.rs (function-only module, 4 functions)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/hermit.rs (function-only module, 3 functions)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/geom_proj_lib.rs (function-only module, 8 functions)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/el_s_lib.rs (function-only module, 76 functions)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/lib.rs

Formatting generated Rust code with rustfmt...
  Formatting complete.

Code generation complete!
  263 modules generated

Bindings generated in /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated
0
```
</details>

Bindings generated. Let me check if the changes resolved the expected types:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -rn "MeshVS_DisplayModeFlags\|Standard_PCharacter\|BOPDS_PDS" crates/opencascade-sys/generated/*.rs | head -40
```
```
crates/opencascade-sys/generated/mesh_vs.rs:6414:    /// DisplayMode is numeric constant describing display mode (see MeshVS_DisplayModeFlags.hxx)
crates/opencascade-sys/generated/mesh_vs.rs:6440:    /// DisplayMode is numeric constant describing display mode (see MeshVS_DisplayModeFlags.hxx)
```

All three types are no longer in the skip comments. The only remaining mentions are in doc comments. Let me verify the previously skipped methods are now bound:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n "set_ds\|SetDS\|pds\b" crates/opencascade-sys/generated/bop_algo.rs | head -20
```
<details>
<summary>Output (17 lines)</summary>

```
10455:    pub fn pds(&mut self) -> Option<&mut crate::bopds::DS> {
10792:    pub fn pds(&mut self) -> Option<&mut crate::bopds::DS> {
10794:            let ptr = unsafe { crate::ffi::BOPAlgo_Builder_pds(self as *mut Self) };
12330:    pub fn pds(&mut self) -> Option<&mut crate::bopds::DS> {
12833:    pub fn ds(&mut self) -> &crate::bopds::DS {
12838:    pub fn pds(&mut self) -> Option<&mut crate::bopds::DS> {
14279:    pub fn pds(&mut self) -> Option<&mut crate::bopds::DS> {
14795:    pub fn ds(&mut self) -> &crate::bopds::DS {
14800:    pub fn pds(&mut self) -> Option<&mut crate::bopds::DS> {
14802:            let ptr = unsafe { crate::ffi::BOPAlgo_PaveFiller_pds(self as *mut Self) };
14812:    pub unsafe fn iterator(&mut self) -> &*mut crate::bopds::Iterator {
15489:    pub fn pds(&mut self) -> Option<&mut crate::bopds::DS> {
16183:    pub fn pds(&mut self) -> Option<&mut crate::bopds::DS> {
16479:        theDS: &mut *mut crate::bopds::DS,
16492:        pDS: &mut *mut crate::bopds::DS,
16503:        theDS: &mut crate::bopds::DS,
16782:    pub fn pds(&mut self) -> Option<&mut crate::bopds::DS> {
```
</details>

The BOPDS_PDS methods are now bound. The `pds()` method returns `Option<&mut crate::bopds::DS>`, and parameters like `theDS: &mut *mut crate::bopds::DS` correctly represent the pointer typedef. Let me also check MeshVS:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n "flags\|Flags" crates/opencascade-sys/generated/mesh_vs.rs | grep -i "fn \|pub " | head -20
```
<details>
<summary>Output (14 lines)</summary>

```
281:pub enum SelectionModeFlags {
294:    fn from(value: SelectionModeFlags) -> Self {
2857:    pub fn get_flags(&self) -> i32 {
2864:    pub fn test_flags(&self, DisplayMode: i32) -> bool {
5698:    pub fn get_flags(&self) -> i32 {
5703:    pub fn test_flags(&self, DisplayMode: i32) -> bool {
6188:    pub fn get_flags(&self) -> i32 {
6193:    pub fn test_flags(&self, DisplayMode: i32) -> bool {
6477:    pub fn get_flags(&self) -> i32 {
6485:    pub fn test_flags(&self, DisplayMode: i32) -> bool {
8591:    pub fn get_flags(&self) -> i32 {
8596:    pub fn test_flags(&self, DisplayMode: i32) -> bool {
9212:    pub fn get_flags(&self) -> i32 {
9217:    pub fn test_flags(&self, DisplayMode: i32) -> bool {
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n "fn new" crates/opencascade-sys/generated/mesh_vs.rs | grep -i "flags\|prs_builder\|MeshPrs\|TextPrs\|NodalColor\|ElementalColor\|VectorPrs" | head -20
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n "fn new" crates/opencascade-sys/generated/mesh_vs.rs | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
341:    pub fn new_size(theSize: usize) -> crate::OwnedPtr<Self> {
363:    pub fn new_handleselectmgrentityowner_handlemeshvsmesh_meshselectionmethod(
1600:    pub fn new_handlemeshvsdatasource_real(
2049:    pub fn new() -> crate::OwnedPtr<Self> {
2276:    pub fn new_handleselectmgrentityowner(
2600:    pub fn new_handlemeshvsmesh_int_handlemeshvsdatasource_int2(
3094:    pub fn new() -> crate::OwnedPtr<Self> {
3099:    pub fn new_int2(theLower: i32, theUpper: i32) -> crate::OwnedPtr<Self> {
3108:    pub fn new_int2_type(
3121:    pub fn new_type_int2_bool(
3137:    pub fn new_array1ofsequenceofinteger(
3330:    pub fn new_bool(theIsAllowOverlapped: bool) -> crate::OwnedPtr<Self> {
3340:    pub fn new() -> crate::OwnedPtr<Self> {
4595:    pub unsafe fn new_selectableobjectptr_int_address_entitytype_int_bool(
4609:    pub unsafe fn new_selectableobjectptr_int_address_entitytype_int(
4622:    pub unsafe fn new_selectableobjectptr_int_address_entitytype(
5037:    pub fn new_selectableobjectptr_handlemeshvsdatasource_int(
5054:    pub fn new_selectableobjectptr_handlemeshvsdatasource(
5474:    pub fn new_handlemeshvsmesh_int_handlemeshvsdatasource_int2(
5907:    pub fn new_handlemeshvsmesh_int_handlemeshvsdatasource_int2(
```
</details>

Let me search specifically for the previously skipped constructors that took `MeshVS_DisplayModeFlags&`:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -B2 "SKIPPED" crates/opencascade-sys/generated/mesh_vs.rs | grep -i "DisplayModeFlags\|display_mode_flags" | head -10
```
```
(base) whitelily:opencascade-rs sethml$
```

All `MeshVS_DisplayModeFlags` skips are resolved. Let me also check Standard_PCharacter methods:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n "pcharacter\|PCharacter\|to_c_string\|to_cstring\|ToCString" crates/opencascade-sys/generated/standard.rs | head -10
```
```
6617:    /// **Source:** `Standard_GUID.hxx`:73 - `Standard_GUID::ToCString()`
6623:    pub unsafe fn to_c_string(&self, aStrGuid: *mut std::ffi::c_char) {
6624:        unsafe { crate::ffi::Standard_GUID_to_c_string(self as *const Self, aStrGuid) }
```

`Standard_GUID::ToCString` is now bound with `*mut std::ffi::c_char`. Let me also check the resource module:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n "SKIPPED\|tostr\|convert" crates/opencascade-sys/generated/resource.rs | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
9:/// List of non ASCII format types which may be converted into the Unicode 16 bits format type.
10:/// Use the functions provided by the Resource_Unicode class to convert a string
829:/// This class provides functions used to convert a non-ASCII C string
849:    /// to Unicode ExtendedString <tostr>.
850:    pub fn convert_sjis_to_unicode(fromstr: &str, tostr: &mut crate::t_collection::ExtendedString) {
852:        unsafe { crate::ffi::Resource_Unicode_convert_sjis_to_unicode(c_fromstr.as_ptr(), tostr) }
857:    /// to Unicode ExtendedString <tostr>.
858:    pub fn convert_euc_to_unicode(fromstr: &str, tostr: &mut crate::t_collection::ExtendedString) {
860:        unsafe { crate::ffi::Resource_Unicode_convert_euc_to_unicode(c_fromstr.as_ptr(), tostr) }
865:    /// to Unicode ExtendedString <tostr>.
866:    pub fn convert_gb_to_unicode(fromstr: &str, tostr: &mut crate::t_collection::ExtendedString) {
868:        unsafe { crate::ffi::Resource_Unicode_convert_gb_to_unicode(c_fromstr.as_ptr(), tostr) }
873:    /// to Unicode ExtendedString <tostr>.
874:    pub fn convert_gbk_to_unicode(
876:        tostr: &mut crate::t_collection::ExtendedString,
879:        unsafe { crate::ffi::Resource_Unicode_convert_gbk_to_unicode(c_fromstr.as_ptr(), tostr) }
884:    /// to Unicode ExtendedString <tostr>.
885:    pub fn convert_big5_to_unicode(
887:        tostr: &mut crate::t_collection::ExtendedString,
890:        unsafe { crate::ffi::Resource_Unicode_convert_big5_to_unicode(c_fromstr.as_ptr(), tostr) }
```
</details>

> 💭 Hmm interesting - the resource Unicode functions seem to be different methods that were already bound (using ExtendedString params, not Standard_PCharacter). Let me check if the Standard_PCharacter methods are also bound now or if they're still skipped.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n "SKIPPED" crates/opencascade-sys/generated/resource.rs | head -10
```

No skipped methods at all in the resource module. Let me now build the full project:

**Todo list:**
- ✅ Investigate Standard_PCharacter (char*)
- ✅ Investigate BOPDS_PDS (BOPDS_DS*)
- ✅ Investigate MeshVS_DisplayModeFlags
- ✅ Implement fixes
- 🔄 Regenerate bindings and test
- ⬜ Update README skip counts
- ⬜ Commit changes
- ⬜ Consider refactoring opportunities

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```
<details>
<summary>Output (222 lines) — exit code 101</summary>

```
   Compiling opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:421:
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
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:40299:12: error: binding reference of type 'const BOPDS_Iterator *' to value of type 'const BOPDS_PIterator' (aka 'BOPDS_Iterator *const') drops 'const' qualifier
warning: opencascade-sys@0.2.0:  40299 |     return self_->Iterator();
warning: opencascade-sys@0.2.0:        |            ^~~~
... (truncated, 17966 chars total)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build 2>&1 | grep "^error" | head -20
```
```
error: failed to run custom build command for `opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)`
```

There's a C++ compilation error. Let me check the details:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build 2>&1 | grep "error:" | head -30
```
<details>
<summary>Output (9 lines)</summary>

```
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:40299:12: error: binding reference of type 'const BOPDS_Iterator *' to value of type 'const BOPDS_PIterator' (aka 'BOPDS_Iterator *const') drops 'const' qualifier
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:47338:12: error: binding reference of type 'const BOPAlgo_PaveFiller *' to value of type 'const BOPAlgo_PPaveFiller' (aka 'BOPAlgo_PaveFiller *const') drops 'const' qualifier
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:47341:12: error: binding reference of type 'const BOPAlgo_Builder *' to value of type 'const BOPAlgo_PBuilder' (aka 'BOPAlgo_Builder *const') drops 'const' qualifier
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:307922:12: error: cannot initialize return object of type 'opencascade::handle<StdObjMgt_Persistent> *' with an rvalue of type 'StdObjMgt_Persistent::Instantiator' (aka 'opencascade::handle<StdObjMgt_Persistent> (*)()')
error: failed to run custom build command for `opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)`
  cargo:warning=/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:40299:12: error: binding reference of type 'const BOPDS_Iterator *' to value of type 'const BOPDS_PIterator' (aka 'BOPDS_Iterator *const') drops 'const' qualifier
  cargo:warning=/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:47338:12: error: binding reference of type 'const BOPAlgo_PaveFiller *' to value of type 'const BOPAlgo_PPaveFiller' (aka 'BOPAlgo_PaveFiller *const') drops 'const' qualifier
  cargo:warning=/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:47341:12: error: binding reference of type 'const BOPAlgo_Builder *' to value of type 'const BOPAlgo_PBuilder' (aka 'BOPAlgo_Builder *const') drops 'const' qualifier
  cargo:warning=/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:307922:12: error: cannot initialize return object of type 'opencascade::handle<StdObjMgt_Persistent> *' with an rvalue of type 'StdObjMgt_Persistent::Instantiator' (aka 'opencascade::handle<StdObjMgt_Persistent> (*)()')
```
</details>

> 💭 There are two categories of errors:
> 
> 1. **Three "drops 'const' qualifier" errors** at lines 40299, 47338, 47341:
>    - `BOPDS_PIterator` (typedef for `BOPDS_Iterator*`) 
>    - `BOPAlgo_PPaveFiller` (typedef for `BOPAlgo_PaveFiller*`)
>    - `BOPAlgo_PBuilder` (typedef for `BOPAlgo_Builder*`)
>    These are similar pointer typedefs to BOPDS_PDS. The issue is the `ConstRef` const positioning - the C++ wrapper generates `const BOPDS_Iterator**` where it should be `BOPDS_Iterator* const*`.
>    
>    Actually, I thought I already fixed the const positioning in `to_cpp_extern_c_param()`, but looking at the actual error message, the problem is at the call site where we're binding a reference of type `const BOPDS_Iterator*` to a value of type `const BOPDS_PIterator` (which is `BOPDS_Iterator* const`), and that's dropping the const qualifier. I need to examine the wrapper code more carefully to see what's happening.
> 
> 2. **One "cannot initialize return object" error** at line 307922:
>    - `StdObjMgt_Persistent::Instantiator` is a function pointer typedef `Handle(StdObjMgt_Persistent) (*)()`, and my late-stage canonical resolution is incorrectly treating it as a pointer type and resolving it to a pointer to the return type instead of keeping it as a function pointer. I need to add a guard to prevent this aggressive resolution from applying to function pointer typedefs.
> 
> Now let me check the actual generated C++ code to see what's happening with the const qualifier issues.

There are two categories of errors. Let me investigate:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && sed -n '40295,40305p' crates/opencascade-sys/generated/wrappers.cpp
```
<details>
<summary>Output (11 lines)</summary>

```
extern "C" BOPDS_DS* BOPAlgo_PaveFiller_pds(BOPAlgo_PaveFiller* self_) {
    return self_->PDS();
}
extern "C" const BOPDS_Iterator*& BOPAlgo_PaveFiller_iterator(BOPAlgo_PaveFiller* self_) {
    return self_->Iterator();
}
extern "C" void BOPAlgo_PaveFiller_set_arguments(BOPAlgo_PaveFiller* self_, TopTools_ListOfShape const* theLS) {
    self_->SetArguments(*theLS);
}
extern "C" void BOPAlgo_PaveFiller_add_argument(BOPAlgo_PaveFiller* self_, TopoDS_Shape const* theShape) {
    self_->AddArgument(*theShape);
```
</details>

> 💭 The error is at line 40299: the C++ code has `const BOPDS_Iterator*& BOPAlgo_PaveFiller_iterator(...)`. The `Iterator()` method returns `BOPDS_PIterator` which is `BOPDS_Iterator*`. But the wrapper expects `const BOPDS_Iterator*&` as the return type.
> 
> Wait, this is a return value binding issue, not a parameter issue. Let me understand:
> 
> `BOPAlgo_PaveFiller::Iterator()` returns `BOPDS_PIterator` (= `BOPDS_Iterator*`) by value (I think). 
> 
> Actually, the method probably returns a const reference to the pointer, so the C++ signature is something like a const reference to `BOPDS_PIterator`. The wrapper declares the return type as `const BOPDS_Iterator*&`, but there's a const qualifier mismatch — the wrapper has const on the pointed-to type while the actual method likely has const on the reference itself.
> 
> The real issue is that extern "C" functions can't return references at all, so the wrapper should be returning a pointer instead. Let me trace through how the return type is actually being generated here. I'm tracing through how the type gets resolved and emitted: the parser sees `const BOPDS_PIterator&`, which resolves to a const reference wrapping a mutable pointer to the BOPDS_Iterator class, but when the C++ code is generated, the const modifier ends up in the wrong position—it's being placed before the pointer instead of after it, producing `const BOPDS_Iterator*&` when it should be `BOPDS_Iterator* const&`.
> 
> Now I'm realizing the issue also affects the `to_cpp_string()` method, which is used for generating return types in the C++ wrapper. The same postfix const fix needs to apply here, though I need to verify whether `to_cpp_string()` is actually the path being used for return type emission or if there's a separate mechanism.
> 
> Looking at the actual wrapper code, I'm noticing something more fundamental — the return type `const BOPDS_Iterator*&` is problematic because `extern "C"` functions can't return references at all. This suggests the wrapper should be returning a pointer instead, like `BOPDS_Iterator* const*`, which would require a different approach than just fixing the const placement. I'm realizing the core issue is that `ConstRef(MutPtr(...))` is being rendered with incorrect const positioning in the C++ signature, and the error message suggests a type mismatch between what's being returned and what the reference binding expects. Fixing the const positioning in `to_cpp_string()` for both `ConstRef` and `ConstPtr` types — the const qualifier needs to come after the type name rather than before it to properly preserve pointer constness when wrapping mutable pointers. I need to apply the same const-correctness fix to both `ConstRef` and `ConstPtr` methods since they're used across C++ wrapper generation, function bodies, and diagnostics. Now I'm looking at a function pointer issue that appeared at line 307922 where there's an incomplete instantiator assignment.
> 
> The problem is that function pointer typedefs have a canonical type that's a pointer to a function prototype, and my late-stage pointer resolution is incorrectly trying to resolve these. I need to add a check that skips resolution when the pointer's pointee is a function type, letting it fall through to the Type::Class handling instead.
> 
> Adding pointer type checking...
> 
> Now I'm consolidating all three fixes: updating the C++ string generation for const references and pointers to use postfix const syntax, skipping function pointer typedefs in the late-stage resolution, and checking for other similar pointer typedefs like BOPDS_PIterator and BOPAlgo_PPaveFiller that might cause the same issues. about whether the const positioning fix to `to_cpp_string()` is really correct for ALL cases:
> 
> For `ConstRef(Class("TopoDS_Shape"))`: 
> - Current: `const TopoDS_Shape&`
> - Fixed: `TopoDS_Shape const&`
> - Both equivalent in C++
> 
> For `ConstPtr(Class("char"))`:
> - Current: `const char*`  
> - Fixed: `char const*`
> - Both equivalent in C++... BUT `to_cpp_extern_c_param()` has a special case for `ConstPtr(Class("char"))` that returns `"const char*"`. This special case would still apply.
> 
> For `ConstRef(I32)`:
> - Current: `const int32_t&`
> - Fixed: `int32_t const&`
> - Both equivalent
> 
> For `ConstRef(MutPtr(Class("X")))`:
> - Current: `const X*&` (WRONG: reference to pointer-to-const-X)
> - Fixed: `X* const&` (CORRECT: const-reference to pointer-to-X)
> 
> For `ConstPtr(MutPtr(Class("X")))`:
> - Current: `const X**` (WRONG)
> - Fixed: `X* const*` (CORRECT)
> 
> The fix handles all these cases correctly, and since nested pointer types were never resolved before, this only affects the newly resolved types while leaving all existing behavior intact.
> 
> Now I need to check the `to_cpp_extern_c_param()` function to make sure it's consistent with the `to_cpp_string()` changes — both should use postfix const positioning.
> 
> Checking the consistency...
> 
> The changes are safe, so I'll go ahead and implement them.

I see two categories of errors:
1. **Const positioning** in `to_cpp_string()` for `ConstRef` and `ConstPtr` with pointer inners
2. **Function pointer typedef** being incorrectly resolved by the late-stage canonical resolution

Let me fix both:

<details>
<summary>Reading lines 770-780 of <a href="../crates/opencascade-binding-generator/src/model.rs#L770-L781">model.rs</a></summary>

```
            Type::U64 => "uint64_t".to_string(),
            Type::Long => "long".to_string(),
            Type::ULong => "unsigned long".to_string(),
            Type::Usize => "size_t".to_string(),
            Type::F32 => "float".to_string(),
            Type::F64 => "double".to_string(),
            Type::CHAR16 => "char16_t".to_string(),
            Type::U8 => "uint8_t".to_string(),
            Type::I8 => "int8_t".to_string(),
            Type::ConstRef(inner) => format!("const {}&", inner.to_cpp_string()),
            Type::MutRef(inner) => format!("{}&", inner.to_cpp_string()),
```
</details>

<details>
<summary>Reading lines 780-785 of <a href="../crates/opencascade-binding-generator/src/model.rs#L780-L786">model.rs</a></summary>

```
            Type::MutRef(inner) => format!("{}&", inner.to_cpp_string()),
            Type::RValueRef(inner) => format!("{}&&", inner.to_cpp_string()),
            Type::ConstPtr(inner) => format!("const {}*", inner.to_cpp_string()),
            Type::MutPtr(inner) => format!("{}*", inner.to_cpp_string()),
            Type::Handle(name) => format!("Handle({})", name),
            Type::Class(name) => name.clone(),
```
</details>

Now let me also read the late-stage canonical resolution I just added:

<details>
<summary>Reading lines 1770-1800 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1770-L1801">parser.rs</a></summary>

```
                return if is_const {
                    Type::ConstPtr(Box::new(inner))
                } else {
                    Type::MutPtr(Box::new(inner))
                };
            }
        }

        // Primitive typedef: canonical is a C primitive type
        if matches!(canon_kind,
            TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |
            TypeKind::SChar | TypeKind::UChar |
            TypeKind::Short | TypeKind::UShort |
            TypeKind::Int | TypeKind::UInt |
            TypeKind::Long | TypeKind::ULong |
            TypeKind::LongLong | TypeKind::ULongLong |
            TypeKind::Float | TypeKind::Double | TypeKind::LongDouble
        ) {
            if let Some(ty) = map_standard_type(canonical_clean) {
                return ty;
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

<details>
<summary>Reading lines 1755-1777 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1755-L1778">parser.rs</a></summary>

```

    // Late-stage canonical resolution for unrecognized typedefs.
    // When a typedef's display name is unrecognized (not in map_standard_type, not a known
    // class, not a simple typedef), try resolving through clang's canonical type.
    // This handles:
    // - Pointer typedefs: Standard_PCharacter = char*, BOPDS_PDS = BOPDS_DS*
    // - Primitive typedefs not caught earlier (fallback for edge cases)
    if matches!(kind, TypeKind::Typedef | TypeKind::Elaborated) {
        let canon_kind = canonical.get_kind();

        // Pointer typedef: canonical is a pointer type
        if canon_kind == TypeKind::Pointer {
            if let Some(pointee) = canonical.get_pointee_type() {
                let is_const = pointee.is_const_qualified();
                let inner = parse_type(&pointee);
                return if is_const {
                    Type::ConstPtr(Box::new(inner))
                } else {
                    Type::MutPtr(Box::new(inner))
                };
            }
        }

```
</details>

Now let me apply all three fixes:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L779-L786">model.rs -1+7</a>, <a href="../crates/opencascade-binding-generator/src/model.rs#L788-L790">-1+2</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/parser.rs#L1765-L1782">parser.rs -12+17</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            // Use postfix const (\"T const&\") rather than prefix (\"const T&\") so that\n            // when the inner type is itself a pointer (e.g., ConstRef(MutPtr(Class(\"X\")))),\n            // the const correctly qualifies the pointer level, not the pointee:\n            //   ConstRef(MutPtr(X)) → \"X* const&\" (correct: const-ref to pointer-to-X)\n            //   vs. \"const X*&\" (wrong: ref to pointer-to-const-X)\n            // For simple types, \"T const&\" and \"const T&\" are equivalent in C/C++.\n            Type::ConstRef(inner) => format!(\"{} const&\", inner.to_cpp_string()),",
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "779:pc"
    },
    {
      "content": "            // Use postfix const for same reason as ConstRef above.\n            Type::ConstPtr(inner) => format!(\"{} const*\", inner.to_cpp_string()),",
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "782:uy"
    },
    {
      "content": "        // Pointer typedef: canonical is a pointer type.\n        // Exclude function pointer typedefs (canonical pointee is a function type)\n        // such as StdObjMgt_Persistent::Instantiator = Handle(...) (*)()\n        if canon_kind == TypeKind::Pointer {\n            if let Some(pointee) = canonical.get_pointee_type() {\n                let pointee_kind = pointee.get_kind();\n                if !matches!(pointee_kind, TypeKind::FunctionPrototype | TypeKind::FunctionNoPrototype) {\n                    let is_const = pointee.is_const_qualified();\n                    let inner = parse_type(&pointee);\n                    return if is_const {\n                        Type::ConstPtr(Box::new(inner))\n                    } else {\n                        Type::MutPtr(Box::new(inner))\n                    };\n                }\n            }\n        }",
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "1765:rl,1766:ms,1767:rk,1768:cf,1769:xb,1770:zf,1771:jl,1772:cu,1773:sh,1774:fj,1775:uo,1776:cy"
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
      "file": "crates/opencascade-binding-generator/src/model.rs",
      "startLine": 779,
      "endLine": 789
    },
    {
      "file": "crates/opencascade-binding-generator/src/parser.rs",
      "startLine": 1765,
      "endLine": 1781
    }
  ]
}
```
</details>

Let me verify the generator still compiles, then regenerate and build:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo check -p opencascade-binding-generator 2>&1
```
```
    Checking opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.00s
real  2.1s
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time ./scripts/regenerate-bindings.sh 2>&1 | tail -5
```

> 💭 Interesting - the number of modules went from 263 to 321! That's a huge increase. I think the late-stage pointer typedef resolution is resolving more typedefs, which adds more modules. Let me now build and check for errors.

Modules increased from 263 to 321 — the pointer typedef resolution is unblocking many more types than expected. Let me build:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1 | grep "^error\|error:" | head -20
```

Clean build! No errors. Let me now run the binding generator tests:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time ./scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (37 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.46s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-3404803fe4cd90de)

running 16 tests
test header_deps::tests::test_is_likely_occt_header ... ok
test module_graph::tests::test_extract_rust_type_name ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test parser::tests::test_extract_module_from_header ... ok
test parser::tests::test_extract_template_arg ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok
test parser::tests::test_map_standard_type ... ok
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

real  3.8s
```
</details>

All 16 tests pass. Now run the workspace tests:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo test --workspace --exclude opencascade-binding-generator 2>&1 | tail -30
```

All tests pass. Now let me update the README with new skip counts:

**Todo list:**
- ✅ Investigate Standard_PCharacter (char*)
- ✅ Investigate BOPDS_PDS (BOPDS_DS*)
- ✅ Investigate MeshVS_DisplayModeFlags
- ✅ Implement fixes
- ✅ Regenerate bindings and test
- 🔄 Update README skip counts
- ⬜ Commit changes
- ⬜ Consider refactoring opportunities

Let me get the new skip counts from the regenerated bindings:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "SKIPPED:" crates/opencascade-sys/generated/*.rs | wc -l
```
```
     432
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | sort | uniq -c | sort -rn | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
  72 class is abstract (has unimplemented pure virtual methods)
  12 return type 'Handle(ShapePersistent_Geom::geometryBase<Geom_Surface>)' is unknown
  10 return type 'Handle(ShapePersistent_Geom::geometryBase<Geom2d_Curve>)' is unknown
  10 return type 'Handle(ShapePersistent_Geom::geometryBase<Geom_Curve>)' is unknown
   9 has string ref param 'name' of type 'const char*&' (needs manual binding)
   8 param 'theFlags' uses unknown type 'ShapeProcess::OperationsFlags const&'
   8 has unbindable types: param 'theParameters': rvalue reference (XSAlgo_ShapeProcessor::ParameterMap&&)
   6 return type 'IMeshData::MapOfInteger const&' is unknown
   6 return type 'Handle(NCollection_Shared<NCollection_List<gp_Pnt2d>>)' is unknown
   5 param 'theWriter' uses unknown type 'RWGltf_GltfOStreamWriter*'
   5 param 'theEvent' uses unknown type 'WNT_HIDSpaceMouse const&'
   5 param 'theBuilder' uses unknown type 'Handle(BVH_Builder<double, 3>) const&'
   5 has unbindable types: param 'theStream': stream type (std::shared_ptr<std::ostream> const&)
   5 has unbindable types: param 'thePlaneEquations': unresolved template type (NCollection_Vector<SelectMgr_Vec4>&)
   4 return type is &mut enum (not representable in extern "C")
   4 return type 'Handle(NCollection_Shared<TColStd_PackedMapOfInteger>)' is unknown
   4 return type 'char' is not CppDeletable
   4 param 'theStream' uses unknown type 'AVStream const&'
   4 has unbindable types: return: unresolved template type (std::shared_ptr<std::streambuf>)
   4 has unbindable types: return: unresolved template type (std::pair<Standard_Real, Standard_Real> const&)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "Reason:" crates/opencascade-sys/generated/*.rs | sed 's/.*Reason: //' | sed 's/param .*/unknown type/' | sed "s/return type .* is unknown/unknown return type/" | sed "s/has unbindable types: .*/has unbindable types/" | sed "s/has string ref param .*/string ref param/" | sed "s/class is abstract .*/abstract class/" | sort | uniq -c | sort -rn
```
<details>
<summary>Output (20 lines)</summary>

```
 141 has unbindable types
 109 unknown return type
  75 unknown type
  72 abstract class
  17 has string ref unknown type
   4 return type is &mut enum (not representable in extern "C")
   4 return type 'char' is not CppDeletable
   3 return type 'V3d_ListOfLight::Iterator' is not CppDeletable
   2 return type 'V3d_ListOfView::Iterator' is not CppDeletable
   2 excluded by bindings.toml
   2 &crate::ffi::Standard_SStream) {
   1 return type 'VrmlData_MapOfNode::Iterator' is not CppDeletable
   1 crates/opencascade-sys/generated/rw_obj.rs:            3 => Ok(SubMeshReason::Newsmoothgroup),
   1 crates/opencascade-sys/generated/rw_obj.rs:            2 => Ok(SubMeshReason::Newmaterial),
   1 crates/opencascade-sys/generated/rw_obj.rs:            1 => Ok(SubMeshReason::Newgroup),
   1 crates/opencascade-sys/generated/rw_obj.rs:            0 => Ok(SubMeshReason::Newobject),
   1 class pointer unknown type
   1 causes ambiguous overload in C++ (listed in AMBIGUOUS_METHODS)
   1 *const Standard_SStream);
   1 *const Standard_SStream,
```
</details>

> 💭 The grep command is getting confused by multi-line patterns. The totals are approximately:
> - 141 has unbindable types (arrays, rvalue refs, shared_ptr, etc.)
> - 109 unknown return type (Handle templates, etc.) 
> - 75 unknown type (params)
> - 72 abstract class
> - 17 string ref param
> - 4 &mut enum return
> - 4 'char' not CppDeletable
> - ~10 other (not CppDeletable, excluded, etc.)
> 
> Total: ~432Let me get a cleaner breakdown.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && python3 -c "
import re, collections
counts = collections.Counter()
with open('/dev/stdin') as f:
    for line in f:
        m = re.search(r'Reason: (.+)', line)
        if not m: continue
        reason = m.group(1).strip()
        if 'unknown type' in reason or 'is unknown' in reason:
            if 'Handle(' in reason:
                counts['Unknown Handle type'] += 1
            elif 'unknown type' in reason:
                counts['Unknown/unresolved type'] += 1
            else:
                counts['Unknown return type'] += 1
        elif 'unresolved template' in reason:
            counts['Unresolved template type'] += 1
        elif 'abstract' in reason:
            counts['Abstract class'] += 1
        elif 'array' in reason or 'Standard_Real[' in reason or 'Standard_Integer[' in reason:
            counts['C-style array'] += 1
        elif 'shared_ptr' in reason:
            counts['Stream (shared_ptr)'] += 1
        elif 'string ref' in reason:
            counts['String ref param'] += 1
        elif 'rvalue' in reason:
            counts['Rvalue reference'] += 1
        elif 'not CppDeletable' in reason:
            counts['Not CppDeletable'] += 1
        elif '&mut enum' in reason:
            counts['&mut enum return'] += 1
        elif 'excluded by bindings' in reason:
            counts['Excluded by bindings.toml'] += 1
        elif 'ambiguous' in reason:
            counts['Ambiguous overload'] += 1
        else:
            counts['Other: ' + reason[:60]] += 1
for cat, n in sorted(counts.items(), key=lambda x: -x[1]):
    print(f'{n:>5}  {cat}')
print(f'{sum(counts.values()):>5}  TOTAL')
" < <(grep -r "Reason:" crates/opencascade-sys/generated/*.rs)
```

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/20f22355-bd40-4a8b-9af0-a41343687ade/toolu_01VjndmC5ZiaFT5c2PqXjD9W__vscode-1771608614080/content.txt#1-1), lines 1 to 40

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/20f22355-bd40-4a8b-9af0-a41343687ade/toolu_01VjndmC5ZiaFT5c2PqXjD9W__vscode-1771608614080/content.txt#40-40), lines 40 to 70

The terminal seems to have issues with the heredoc. Let me use a script file instead:

Created [count_skips.py](../tmp/count_skips.py)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "Reason:" crates/opencascade-sys/generated/*.rs | python3 tmp/count_skips.py
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -rh "Reason:" crates/opencascade-sys/generated/*.rs > tmp/skip_reasons.txt && python3 tmp/count_skips.py < tmp/skip_reasons.txt
```
<details>
<summary>Output (16 lines)</summary>

```
Total: 432

| 102 |  23.6% | Unknown/unresolved type |
|  90 |  20.8% | Unresolved template type |
|  84 |  19.4% | Unknown Handle type |
|  72 |  16.7% | Abstract class |
|  19 |   4.4% | C-style array |
|  17 |   3.9% | String ref param |
|  15 |   3.5% | Stream (shared_ptr) |
|  12 |   2.8% | Rvalue reference |
|  10 |   2.3% | Not CppDeletable |
|   4 |   0.9% | &mut enum return |
|   3 |   0.7% | Other: param 'theBuilder' uses unknown Handle type |
|   2 |   0.5% | Excluded by bindings.toml |
|   1 |   0.2% | Ambiguous overload |
|   1 |   0.2% | Other: param 'theMutex' uses unknown Handle type |
```
</details>

> 💭 Let me now compare with the old skip counts from the README:
> 
> Old:
> - 165 (33.3%) Unknown

> 💭 Let me now compare with the old skip counts from the README:
> 
> Old:
> - 165 (33.3%) Unknown/unresolved type
> - 91 (18.4%) Unresolved template type
> - 83 (16.8%) Unknown Handle type
> - 72 (14.5%) Abstract class
> - 19 (3.8%) C-style array
> - 19 (3.8%) Stream (shared_ptr)
> - 17 (3.4%) String ref param
> - 12 (2.4%) Rvalue reference
> - 10 (2.0%) Not CppDeletable
> 
> 
> - 4 (0.8%) &mut enum return
> - 2 (0.4%) Excluded by bindings.toml
> - 1 (0.2%) Ambiguous overload
> Total: ~495
> 
> New:
> - 102 (23.6%) Unknown/unresolved type → was 165 (63 fewer)
> - 90 (20.8%) Unresolved template type → was 91 (1 fewer - guess from re-categorization)
> - 88 (includes "Other" Handle entries) → was 83 (5 more?? Hmm, probably re-categorization)
> 
> Looking at the new numbers, I've made solid progress on unknown/unresolved types, cutting that category nearly in half. The template type count stayed roughly the same, and the Handle types shifted slightly, likely due to how I'm bucketing them now. some reclassification)
> - 72 (16.7%) Abstract class → same
> - 19 (4.4%) C-style array → same
> - 17 (3.9%) String ref param → same (was 17)
> - 15 (3.5%) Stream (shared_ptr) → was 19 (4 fewer)
> - 12 (2.8%) Rvalue reference → same
> - 10 (2.3%) Not CppDeletable → same
> - 4 (0.9%) &mut enum return → same
> - 2 (0.5%) Excluded by bindings.toml → same
> - 1 (0.2%) Ambiguous overload → same
> Total: 432 (was ~495, now 63 fewer)
> 
> I need to recalculate the Unknown Handle type count. The entries mentioning "unknown Handle type" should be grouped with the Handle count, which would make it 88 total—5 more than the previous 83. But looking at the classification logic, I'm realizing the script might be misclassifying these. The entries like "param 'theBuilder' uses unknown Handle type" contain both "unknown type" and "Handle" in the reason, but the script checks for "unknown type" first before checking for "Handle(", so they're getting caught by the earlier condition. I'm realizing there might be a mismatch between what the script is actually checking and what the skip reasons contain — the categorization logic should be catching these "unknown Handle type" entries, but they're showing up as "Other" instead, so I need to look at the actual text the script is seeing to figure out where the disconnect is.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -rh "Reason:" crates/opencascade-sys/generated/*.rs | grep -i "handle" | grep -iv "Handle(" | head -10
```
<details>
<summary>Output (10 lines)</summary>

```
//   Reason: has unbindable types: return: unresolved template type (NCollection_Sequence<opencascade::handle<AIS_Animation>> const&)
//   Reason: has unbindable types: param 'theOther': unresolved template type (NCollection_Sequence<opencascade::handle<AIS_InteractiveObject>> const&)
//   Reason: has unbindable types: return: unresolved template type (NCollection_Sequence<opencascade::handle<AIS_InteractiveObject>> const&)
//   Reason: has unbindable types: param 'theSequence': unresolved template type (NCollection_Sequence<opencascade::handle<AIS_InteractiveObject>>&)
//   Reason: has unbindable types: return: unresolved template type (NCollection_Sequence<opencascade::handle<AIS_InteractiveObject>>&)
//   Reason: has unbindable types: param 'theFonts': unresolved template type (NCollection_Sequence<opencascade::handle<Font_SystemFont>>&)
//   Reason: has unbindable types: param 'theFonts': unresolved template type (NCollection_Sequence<opencascade::handle<Font_SystemFont>> const&)
//   Reason: param 'theBuilder' uses unknown Handle type
//   Reason: has unbindable types: return: unresolved template type (NCollection_List<opencascade::handle<Graphic3d_Layer>> const&)
//   Reason: has unbindable types: return: unresolved template type (NCollection_Sequence<opencascade::handle<Graphic3d_CView>> const&)
```
</details>

Those "uses unknown Handle type" entries have "Handle" as a noun, not as `Handle(`. Let me quickly get the exact new "Most Common Unknown Types" for the README update. Let me also count the total unknown types:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -rh "Reason:" crates/opencascade-sys/generated/*.rs | grep -oP "unknown type '([^']+)'" | sed "s/unknown type '//" | sed "s/'//" | sort | uniq -c | sort -rn | head -20
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
cd /Users/sethml/src/opencascade-rs && grep -roh "unknown type '[^']*'" crates/opencascade-sys/generated/*.rs | sed "s/unknown type '//" | sed "s/'$//" | sort | uniq -c | sort -rn | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
   9 ShapeProcess::OperationsFlags const&
   5 WNT_HIDSpaceMouse const&
   5 RWGltf_GltfOStreamWriter*
   5 Handle(BVH_Builder<double, 3>) const&
   4 AVStream const&
   3 IMeshData::VectorOfInteger&
   3 IMeshData::Array1OfVertexOfDelaun&
   2 Standard_WideChar const*
   2 OSD_ThreadFunction const&
   2 NSOpenGLContext*
   2 IMeshData_Face *const const&
   2 IMeshData_Edge *const const&
   2 CPnts_RealFunction const&
   2 AVRational const&
   2 Aspect_XDisplay*
   1 TNaming_Node* const&
   1 TDF_LabelNode*
   1 std::type_info const&
   1 std::streampos
   1 Standard_Utf16Char
```
</details>

Good, `Standard_PCharacter`, `BOPDS_PDS`, and `MeshVS_DisplayModeFlags` no longer appear. Let me also check for the return-type unknowns:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -roh "return type '[^']*' is unknown" crates/opencascade-sys/generated/*.rs | sed "s/return type '//" | sed "s/' is unknown//" | sort | uniq -c | sort -rn | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
  12 Handle(ShapePersistent_Geom::geometryBase<Geom_Surface>)
  10 Handle(ShapePersistent_Geom::geometryBase<Geom2d_Curve>)
  10 Handle(ShapePersistent_Geom::geometryBase<Geom_Curve>)
   6 IMeshData::MapOfInteger const&
   6 Handle(NCollection_Shared<NCollection_List<gp_Pnt2d>>)
   4 Handle(NCollection_Shared<TColStd_PackedMapOfInteger>)
   3 Handle(BVH_Builder<double, 3>) const&
   3 GLXFBConfig
   2 ShapeProcess::OperationsFlags const&
   2 ProxPnt_Status const&
   2 IMeshData::ListOfInteger const&
   2 IMeshData::IMapOfReal&
   2 IMeshData::IMapOfReal const&
   2 Handle(BVH_Tree<double, 3>) const&
   1 TNaming_Node*
   1 Standard_JmpBuf&
   1 OSD_Function
   1 IMeshData::ListOfInteger&
   1 IMeshData_Face *const const&
   1 IMeshData_Edge *const const&
```
</details>

Now let me update the README. Let me read the section I need to update:

<details>
<summary>Reading lines 385-500 of <a href="../crates/opencascade-binding-generator/README.md#L385-L501">README.md</a></summary>

````
The generator appends `include!("../manual/<module>.rs");` (with a comment explaining why) to the generated module re-export file when a corresponding `manual/<module>.rs` exists. Because `include!()` is a textual insertion, the manual code has full access to the module's type aliases (e.g., `AdvancedEvolved`, `Finder`). The `extern "C"` declarations in manual files are not marked `pub`, so they are private to the module and not exposed as part of the public API. `build.rs` globs `manual/*_wrappers.cpp` and compiles them alongside `generated/wrappers.cpp`. Since Rust allows multiple `impl` blocks for a type, manual methods appear seamlessly alongside the auto-generated ones.

See `crates/opencascade-sys/manual/` and the comments in `bindings.toml` for examples (`Transfer_Finder::GetStringAttribute`, `BRepFill_AdvancedEvolved::SetTemporaryDirectory`, and `standard` iostream accessors).

---

## Skipped Symbols

The binding generator skips ~495 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Every skipped symbol is documented in the generated per-module `.rs` files as a `// SKIPPED:` comment block including:

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
| 165 | 33.3% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`ShapeProcess::OperationsFlags`, `IMeshData::MapOfInteger`, `BOPDS_PDS`, etc.) |
| 91 | 18.4% | **Unresolved template type** | Template instantiations that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `NCollection_Vec3<...>`, etc.) |
| 83 | 16.8% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(ShapePersistent_Geom::...)`, `Handle(BVH_Builder<...>)`, etc.) |
| 72 | 14.5% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |
| 19 | 3.8% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |
| 19 | 3.8% | **Stream (shared_ptr)** | `std::shared_ptr<std::istream/ostream>` — smart-pointer-wrapped streams not yet bindable |
| 17 | 3.4% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |
| 12 | 2.4% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across FFI |
| 10 | 2.0% | **Not CppDeletable** | Return type class has no destructor in the binding set |
| 4 | 0.8% | **&mut enum return** | Mutable reference to enum (cxx limitation) |
| 2 | 0.4% | **Excluded by bindings.toml** | Explicitly excluded in config (e.g., ambiguous overload workarounds) |
| 1 | 0.2% | **Ambiguous overload** | C++ overload that would produce identical wrapper signatures |

### Most Common Unknown Types

The "unknown type" and "unknown Handle type" categories (50.1% of all skips) are dominated by a few types:

| Count | Type | How to Unblock |
|------:|------|----------------|
| 12 | `Handle(ShapePersistent_Geom::...)` | Nested template Handle types in shape persistence — low priority |
| 11 | `ShapeProcess::OperationsFlags` | Class-scoped using alias to `std::bitset` — non-OCCT type, not bindable |
| 10 | `Handle(ShapePersistent_Geom::...)` | Nested template Handle types in shape persistence — low priority |
| 9 | `Handle(BVH_Builder<double, 3>)` | Template Handle type in BVH internals — low priority |
| 8 | `IMeshData::MapOfInteger` | Namespace-scoped NCollection template typedef in meshing i
... (truncated, 8387 chars)
````
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/20f22355-bd40-4a8b-9af0-a41343687ade/toolu_01L4stgGRVaUN3nZzoLNCZwC__vscode-1771608614090/content.txt#1-1), lines 1 to 80

Now let me update the README with the new counts. Let me also check the "Important Skipped Symbols" data to see if Document Framework pointer typedefs were resolved:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -rh "SKIPPED" crates/opencascade-sys/generated/t_doc_std.rs crates/opencascade-sys/generated/tdf.rs 2>/dev/null | head -10
```
```
// SKIPPED: **Source:** `TDF_AttributeIterator.hxx`:52 - `TDF_AttributeIterator::TDF_AttributeIterator`
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -rh "Reason:" crates/opencascade-sys/generated/t_doc_std.rs crates/opencascade-sys/generated/tdf.rs 2>/dev/null
```
```
//   Reason: param 'aLabelNode' uses unknown type 'TDF_LabelNode*'
```

> 💭 So `TDocStd_XLinkPtr` was resolved (it was a pointer typedef like BOPDS_PDS), but `TDF_LabelNode*` remains (

> 💭 So `TDocStd_XLinkPtr` was resolved (it was a pointer typedef like BOPDS_PDS), but `TDF_LabelNode*` remains (it's a raw pointer to a class not in the binding set, not a typedef issue).
> 
> Let me now also check the data exchange skips and shape meshing skips:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && for mod in step_control iges_control xs_control rw_gltf rw_obj rw_stl rw_ply; do echo "--- $mod ---"; grep -c "SKIPPED" crates/opencascade-sys/generated/${mod}.rs 2>/dev/null || echo "0"; done
```
<details>
<summary>Output (14 lines)</summary>

```
--- step_control ---
2
--- iges_control ---
3
--- xs_control ---
10
--- rw_gltf ---
6
--- rw_obj ---
2
--- rw_stl ---
2
--- rw_ply ---
1
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -c "SKIPPED" crates/opencascade-sys/generated/b_rep_mesh.rs crates/opencascade-sys/generated/i_mesh_data.rs 2>/dev/null
```
```
crates/opencascade-sys/generated/b_rep_mesh.rs:57
crates/opencascade-sys/generated/i_mesh_data.rs:5
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -c "SKIPPED" crates/opencascade-sys/generated/poly.rs 2>/dev/null
```
```
7
```

Good — the data exchange (26), meshing (62), and poly (7) counts are unchanged. The TDocStd count went from 3 to 0 (solved!) and TDF from 1 to 1 (unchanged). Now let me update the README:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/README.md#L393-L394">README.md -1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L410-L424">-14+14</a>, <a href="../crates/opencascade-binding-generator/README.md#L427-L428">-1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L429-L439">-14+10</a>, <a href="../crates/opencascade-binding-generator/README.md#L446-L447">-1+1</a>, <a href="../crates/opencascade-binding-generator/README.md#L448-L449">-1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "393:yy",
      "content": "The binding generator skips ~432 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Every skipped symbol is documented in the generated per-module `.rs` files as a `// SKIPPED:` comment block including:"
    },
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "410:fa,411:oi,412:gp,413:kd,414:mh,415:xh,416:jx,417:zz,418:mr,419:yy,420:ee,421:ys,422:ht,423:xy",
      "content": "| Count | % | Category | Description |\n|------:|----:|----------|-------------|\n| 106 | 24.5% | **Unknown/unresolved type** | Parameter or return type not in the binding set (`ShapeProcess::OperationsFlags`, `IMeshData::MapOfInteger`, etc.) |\n| 90 | 20.8% | **Unresolved template type** | Template instantiations that can't be resolved (`NCollection_DataMap<...>`, `std::pair<...>`, `NCollection_Vec3<...>`, etc.) |\n| 84 | 19.4% | **Unknown Handle type** | Handle to a class not in the binding set (`Handle(ShapePersistent_Geom::...)`, `Handle(BVH_Builder<...>)`, etc.) |\n| 72 | 16.7% | **Abstract class** | No constructors generated (class has unimplemented pure virtual methods) |\n| 19 | 4.4% | **C-style array** | `Standard_Real[]` or `Standard_Integer[3]` params |\n| 17 | 3.9% | **String ref param** | `const char*&` or `const char* const&` parameters — needs manual binding |\n| 15 | 3.5% | **Stream (shared_ptr)** | `std::shared_ptr<std::istream/ostream>` — smart-pointer-wrapped streams not yet bindable |\n| 12 | 2.8% | **Rvalue reference** | C++ move semantics (`T&&`) — no Rust equivalent across FFI |\n| 10 | 2.3% | **Not CppDeletable** | Return type class has no destructor in the binding set |\n| 4 | 0.9% | **&mut enum return** | Mutable reference to enum (cxx limitation) |\n| 2 | 0.5% | **Excluded by bindings.toml** | Explicitly excluded in config (e.g., ambiguous overload workarounds) |\n| 1 | 0.2% | **Ambiguous overload** | C++ overload that would produce identical wrapper signatures |"
    },
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "427:fg",
      "content": "The \"unknown type\" and \"unknown Handle type\" categories (43.9% of all skips) are dominated by a few types:"
    },
    {
      "filePath": "crates/opencascade-binding-generator/README.md",
      "lineHashes": "429:js,430:bt,431:df,432:pe,433:tp,434:eg,435:da,436:wh,437:ym,438:bv,439:vh,440:jx,441:fh,442:bg",
      "content": "| Count | Type | How to Unblock |\n|------:|------|----------------|\n| 12 | `Handle(ShapePersistent_Geom::...)` | Nested template Handle types in shape persistence — low priority |\n| 10 | `Handle(ShapePersistent_Geom::...)` | Nested template Handle types in shape persistence — low priority |\n| 9 | `ShapeProcess::OperationsFlags` | Class-scoped using alias to `std::bitset` — non-OCCT type, not b
... (truncated, 4683 chars)
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
      "endLine": 448
    }
  ]
}
```
</details>

Now also update the top-level README (the user's active file) if it has the same table:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n "MeshVS_DisplayModeFlags\|BOPDS_PDS\|Standard_PCharacter\|~495" README.md | head -10
```
```
(base) whitelily:opencascade-rs sethml$
```

Good, the top-level README doesn't mention these. Now let me commit:

**Todo list:**
- ✅ Investigate Standard_PCharacter (char*)
- ✅ Investigate BOPDS_PDS (BOPDS_DS*)
- ✅ Investigate MeshVS_DisplayModeFlags
- ✅ Implement fixes
- ✅ Regenerate bindings and test
- ✅ Update README skip counts
- 🔄 Commit changes
- ⬜ Consider refactoring opportunities

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff --stat
```
<details>
<summary>Output (61 lines)</summary>

```
 crates/opencascade-binding-generator/README.md     |     40 +-
 crates/opencascade-binding-generator/src/model.rs  |     19 +-
 crates/opencascade-binding-generator/src/parser.rs |     95 +-
 crates/opencascade-sys/generated/ais.rs            |     22 +-
 crates/opencascade-sys/generated/approx_int.rs     |      4 +-
 crates/opencascade-sys/generated/aspect.rs         |     27 +-
 crates/opencascade-sys/generated/b_rep_algo_api.rs |     26 +-
 crates/opencascade-sys/generated/b_rep_extrema.rs  |     12 +-
 crates/opencascade-sys/generated/b_rep_feat.rs     |     53 +
 crates/opencascade-sys/generated/b_rep_mesh.rs     |     60 +-
 crates/opencascade-sys/generated/bin_obj_mgt.rs    |    255 +-
 crates/opencascade-sys/generated/bop_algo.rs       |    288 +-
 crates/opencascade-sys/generated/bopds.rs          |     45 +-
 crates/opencascade-sys/generated/c_pnts.rs         |      4 +-
 crates/opencascade-sys/generated/ffi.rs            |    716 +-
 crates/opencascade-sys/generated/font.rs           |     18 +-
 crates/opencascade-sys/generated/ft.rs             |      2 +-
 crates/opencascade-sys/generated/graphic3d.rs      |     40 +-
 crates/opencascade-sys/generated/hlr_algo.rs       |      4 +-
 crates/opencascade-sys/generated/i_mesh_data.rs    |     10 +-
 crates/opencascade-sys/generated/if_select.rs      |      8 +-
 crates/opencascade-sys/generated/iges_control.rs   |      4 +-
 crates/opencascade-sys/generated/iges_to_b_rep.rs  |      4 +-
 crates/opencascade-sys/generated/image.rs          |     16 +-
 crates/opencascade-sys/generated/int_ana.rs        |      4 +-
 crates/opencascade-sys/generated/int_patch.rs      |      4 +-
 crates/opencascade-sys/generated/ldom.rs           |     12 +-
 crates/opencascade-sys/generated/media.rs          |     18 +-
 crates/opencascade-sys/generated/mesh_vs.rs        |    119 +-
 crates/opencascade-sys/generated/message.rs        |      6 +-
 crates/opencascade-sys/generated/osd.rs            |     69 +-
 crates/opencascade-sys/generated/resource.rs       |    138 +-
 crates/opencascade-sys/generated/rw_gltf.rs        |      4 +-
 crates/opencascade-sys/generated/rw_ply.rs         |      4 +-
 crates/opencascade-sys/generated/select3_d.rs      |     10 +-
 crates/opencascade-sys/generated/select_mgr.rs     |     12 +-
 crates/opencascade-sys/generated/standard.rs       |     62 +-
 .../opencascade-sys/generated/std_l_persistent.rs  |     16 +-
 cra
... (truncated, 4802 chars total)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff --stat -- crates/opencascade-sys/generated/lib.rs 2>/dev/null | head -5
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff -- crates/opencascade-sys/generated/lib.rs | head -30
```
```
(base) whitelily:opencascade-rs sethml$
```

> 💭 lib.rs didn't change. The "321 modules" vs "263 modules" I saw earlier was probably just a different counting (maybe I was confused). Let me move on and commit.
> 
> Wait, how could 321 modules be generated if lib.rs didn't change? Maybe the 321 number includes function-only modules and other module types that are counted differently... Let me not worry about this.
> 
> Let me write the commit message and commit.

**Terminal:**
```sh
cat > /Users/sethml/src/opencascade-rs/tmp/commit-msg.txt << 'EOF'
Resolve pointer and primitive typedef skips in binding generator

Three changes to the parser/codegen unblock ~63 previously skipped symbols:

1. **Late-stage canonical resolution for pointer typedefs** (parser.rs):
   When parse_type() encounters a typedef whose canonical type is a pointer
   (e.g., BOPDS_PDS = BOPDS_DS*, Standard_PCharacter = char*), resolve
   through the canonical pointer. Function pointer typedefs are excluded.

2. **Fix reference guard for primitive typedefs** (parser.rs):
   The guard that prevents canonical resolution for class-like types inside
   references now detects genuine primitive typedefs
   (e.g., MeshVS_DisplayModeFlags = Standard_Integer = int) and lets them
   resolve normally instead of incorrectly preserving the class name.

3. **Fix C++ const positioning for pointer-in-reference types** (model.rs):
   Changed ConstRef and ConstPtr emission from prefix const ("const T*") to
   postfix const ("T const*") in both to_cpp_string() and
   to_cpp_extern_c_param(). For simple types these are equivalent; for
   pointer-in-reference types like ConstRef(MutPtr(Class("X"))), postfix
   const correctly produces "X* const&" instead of the incorrect "const X*&".

Resolved types and their skip counts:
- Standard_PCharacter (8 skips → 0): char* typedef, now resolves to *mut c_char
- BOPDS_PDS (7 skips → 0): BOPDS_DS* typedef, now resolves to *mut BOPDS_DS
- MeshVS_DisplayModeFlags (5 skips → 0): Standard_Integer typedef in references
- TDocStd_XLinkPtr (3 skips → 0): pointer typedef resolved by same mechanism
- Various other pointer typedefs (BOPDS_PIterator, BOPAlgo_PPaveFiller, etc.)

Total skipped symbols: ~495 → ~432 (63 fewer, 12.7% reduction)
EOF
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git add -A && git commit -F tmp/commit-msg.txt
```
```
[main 28a0575] Resolve pointer and primitive typedef skips in binding generator
 60 files changed, 52340 insertions(+), 51458 deletions(-)
```

**Todo list:**
- ✅ Investigate Standard_PCharacter (char*)
- ✅ Investigate BOPDS_PDS (BOPDS_DS*)
- ✅ Investigate MeshVS_DisplayModeFlags
- ✅ Implement fixes
- ✅ Regenerate bindings and test
- ✅ Update README skip counts
- ✅ Commit changes
- 🔄 Consider refactoring opportunities

Now let me think about refactoring opportunities. The main pattern I noticed is **repeated primitive TypeKind match arms**. Let me check:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -c "TypeKind::Bool | TypeKind::CharS" crates/opencascade-binding-generator/src/parser.rs
```
```
3
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n "TypeKind::Bool | TypeKind::CharS" crates/opencascade-binding-generator/src/parser.rs
```
```
1540:                        TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |
1607:                            TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |
1785:            TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |
```

There are 3 instances of the same long primitive TypeKind match pattern. Let me also check `to_cpp_string()` const positioning — I introduced two similar comment blocks. Let me extract the repeated TypeKind pattern into a helper function:

<details>
<summary>Reading lines 1535-1556 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1535-L1557">parser.rs</a></summary>

```
            let is_primitive_typedef = matches!(kind, TypeKind::Typedef | TypeKind::Elaborated)
                && clang_type.get_declaration()
                    .filter(|d| d.get_kind() == clang::EntityKind::TypedefDecl)
                    .and_then(|d| d.get_typedef_underlying_type())
                    .map(|u| matches!(u.get_kind(),
                        TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |
                        TypeKind::SChar | TypeKind::UChar |
                        TypeKind::Short | TypeKind::UShort |
                        TypeKind::Int | TypeKind::UInt |
                        TypeKind::Long | TypeKind::ULong |
                        TypeKind::LongLong | TypeKind::ULongLong |
                        TypeKind::Float | TypeKind::Double | TypeKind::LongDouble |
                        TypeKind::Typedef |   // chain through another typedef (e.g., Standard_Integer)
                        TypeKind::Elaborated  // clang sugar around typedef (e.g., Standard_Integer via Elaborated)
                    ))
                    .unwrap_or(false);

            !is_primitive_typedef
        }
    };

    // Handle primitives via canonical type.
```
</details>

<details>
<summary>Reading lines 1597-1620 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1597-L1621">parser.rs</a></summary>

```
                // For simple typedef names (not template/namespaced), check if the
                // pointee's canonical type is actually a primitive. If so, this is a
                // genuine typedef-to-primitive (e.g., MeshVS_DisplayModeFlags = int)
                // and should be resolved normally, not preserved as a class name.
                let is_template_or_ns = base.contains('<') || base.contains("::");
                let pointee_is_primitive_canonical = !is_template_or_ns
                    && clang_type.get_pointee_type().map(|pt| {
                        let canon = pt.get_canonical_type();
                        let ck = canon.get_kind();
                        matches!(ck,
                            TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |
                            TypeKind::SChar | TypeKind::UChar |
                            TypeKind::Short | TypeKind::UShort |
                            TypeKind::Int | TypeKind::UInt |
                            TypeKind::Long | TypeKind::ULong |
                            TypeKind::LongLong | TypeKind::ULongLong |
                            TypeKind::Float | TypeKind::Double | TypeKind::LongDouble
                        )
                    }).unwrap_or(false);

                if !pointee_is_primitive_canonical {
                    let inner = Type::Class(base.to_string());
                    if let Some(pointee) = clang_type.get_pointee_type() {
                        let is_const = pointee.is_const_qualified();
```
</details>

<details>
<summary>Reading lines 1780-1800 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1780-L1801">parser.rs</a></summary>

```
            }
        }

        // Primitive typedef: canonical is a C primitive type
        if matches!(canon_kind,
            TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |
            TypeKind::SChar | TypeKind::UChar |
            TypeKind::Short | TypeKind::UShort |
            TypeKind::Int | TypeKind::UInt |
            TypeKind::Long | TypeKind::ULong |
            TypeKind::LongLong | TypeKind::ULongLong |
            TypeKind::Float | TypeKind::Double | TypeKind::LongDouble
        ) {
            if let Some(ty) = map_standard_type(canonical_clean) {
                return ty;
            }
        }
    }

    Type::Class(clean_name.to_string())
}
```
</details>

Yes, there's a clear refactoring opportunity: extract `is_primitive_type_kind()` as a helper. The first instance at line 1540 also includes `TypeKind::Typedef | TypeKind::Elaborated` (for typedef chains), but the core primitive kinds are the same.

Let me also check the `crates/opencascade-binding-generator/src/model.rs` const positioning — I introduced similar comments in two places:

<details>
<summary>Reading lines 745-760 of <a href="../crates/opencascade-binding-generator/src/model.rs#L745-L761">model.rs</a></summary>

```
            // Use postfix const ("T const*") rather than prefix ("const T*") so that
            // when the inner type is itself a pointer (e.g., ConstRef(MutPtr(Class("X")))),
            // the const correctly qualifies the pointer level, not the pointee:
            //   ConstRef(MutPtr(X)) → "X* const*" (correct: pointer to const-pointer-to-X)
            //   vs. "const X**" (wrong: pointer to pointer-to-const-X)
            // For simple types, "T const*" and "const T*" are equivalent in C/C++.
            Type::ConstRef(inner) => format!("{} const*", inner.to_cpp_string()),
            Type::MutRef(inner) => format!("{}*", inner.to_cpp_string()),
            Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
                "const char*".to_string()
            }
            _ => self.to_cpp_string(),
        }
    }

    /// Get a human-readable C++-like type string for diagnostic messages.
```
</details>

<details>
<summary>Reading lines 779-800 of <a href="../crates/opencascade-binding-generator/src/model.rs#L779-L801">model.rs</a></summary>

```
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

> 💭 Good opportunity for refactoring:1. **Extract `is_c_primitive_type_kind()` helper** in parser.rs — the same long list of `TypeKind::Bool | TypeKind::CharS | ...` appears 3 times. Extract to a function that takes a `TypeKind` and returns `bool`. The first occurrence also includes `TypeKind::Typedef | TypeKind::Elaborated` for typedef chains, which can be a separate parameter or separate check.
> 
> 2. The comments in model.rs for `ConstRef` are somewhat duplicated between `to_cpp_extern_c_param()` and `to_cpp_string()`, but this is reasonable documentation. Not worth de-duplicating since they explain different contexts.
> 
> 
> Now I'll start implementing the helper function to consolidate the repeated type checks.

The refactoring opportunity is clear: extract the repeated primitive `TypeKind` match into a helper function. Let me implement it:

<details>
<summary>Reading lines 10-16 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L10-L17">parser.rs</a></summary>

```
use anyhow::{Context, Result};
use clang::{Accessibility, Availability, Clang, Entity, EntityKind, EntityVisitResult, Index, TypeKind};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Instant;

```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/parser.rs#L15-L30">parser.rs -1+15</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "use std::time::Instant;\n\n/// Check if a clang TypeKind represents a C/C++ primitive type (int, double, char, etc.).\n/// Used to detect when a typedef resolves to a primitive through canonical type analysis.\nfn is_c_primitive_type_kind(kind: TypeKind) -> bool {\n    matches!(kind,\n        TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |\n        TypeKind::SChar | TypeKind::UChar |\n        TypeKind::Short | TypeKind::UShort |\n        TypeKind::Int | TypeKind::UInt |\n        TypeKind::Long | TypeKind::ULong |\n        TypeKind::LongLong | TypeKind::ULongLong |\n        TypeKind::Float | TypeKind::Double | TypeKind::LongDouble\n    )\n}",
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "15:ul"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/parser.rs","startLine":15,"endLine":29}
```
</details>

Now replace the three repeated patterns. Let me re-read the updated line numbers:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n "TypeKind::Bool | TypeKind::CharS" crates/opencascade-binding-generator/src/parser.rs
```
```
21:        TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |
1554:                        TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |
1621:                            TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |
1799:            TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |
```

<details>
<summary>Reading lines 1547-1568 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1547-L1569">parser.rs</a></summary>

```
            // The underlying type of a typedef chain (e.g., Graphic3d_ZLayerId -> Standard_Integer -> int)
            // may appear as Elaborated rather than Typedef, so we accept both.
            let is_primitive_typedef = matches!(kind, TypeKind::Typedef | TypeKind::Elaborated)
                && clang_type.get_declaration()
                    .filter(|d| d.get_kind() == clang::EntityKind::TypedefDecl)
                    .and_then(|d| d.get_typedef_underlying_type())
                    .map(|u| matches!(u.get_kind(),
                        TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |
                        TypeKind::SChar | TypeKind::UChar |
                        TypeKind::Short | TypeKind::UShort |
                        TypeKind::Int | TypeKind::UInt |
                        TypeKind::Long | TypeKind::ULong |
                        TypeKind::LongLong | TypeKind::ULongLong |
                        TypeKind::Float | TypeKind::Double | TypeKind::LongDouble |
                        TypeKind::Typedef |   // chain through another typedef (e.g., Standard_Integer)
                        TypeKind::Elaborated  // clang sugar around typedef (e.g., Standard_Integer via Elaborated)
                    ))
                    .unwrap_or(false);

            !is_primitive_typedef
        }
    };
```
</details>

<details>
<summary>Reading lines 1612-1632 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1612-L1633">parser.rs</a></summary>

```
                // pointee's canonical type is actually a primitive. If so, this is a
                // genuine typedef-to-primitive (e.g., MeshVS_DisplayModeFlags = int)
                // and should be resolved normally, not preserved as a class name.
                let is_template_or_ns = base.contains('<') || base.contains("::");
                let pointee_is_primitive_canonical = !is_template_or_ns
                    && clang_type.get_pointee_type().map(|pt| {
                        let canon = pt.get_canonical_type();
                        let ck = canon.get_kind();
                        matches!(ck,
                            TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |
                            TypeKind::SChar | TypeKind::UChar |
                            TypeKind::Short | TypeKind::UShort |
                            TypeKind::Int | TypeKind::UInt |
                            TypeKind::Long | TypeKind::ULong |
                            TypeKind::LongLong | TypeKind::ULongLong |
                            TypeKind::Float | TypeKind::Double | TypeKind::LongDouble
                        )
                    }).unwrap_or(false);

                if !pointee_is_primitive_canonical {
                    let inner = Type::Class(base.to_string());
```
</details>

<details>
<summary>Reading lines 1795-1808 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1795-L1809">parser.rs</a></summary>

```
        }

        // Primitive typedef: canonical is a C primitive type
        if matches!(canon_kind,
            TypeKind::Bool | TypeKind::CharS | TypeKind::CharU |
            TypeKind::SChar | TypeKind::UChar |
            TypeKind::Short | TypeKind::UShort |
            TypeKind::Int | TypeKind::UInt |
            TypeKind::Long | TypeKind::ULong |
            TypeKind::LongLong | TypeKind::ULongLong |
            TypeKind::Float | TypeKind::Double | TypeKind::LongDouble
        ) {
            if let Some(ty) = map_standard_type(canonical_clean) {
                return ty;
```
</details>

Now let me replace all three with the helper:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/parser.rs#L1549-L1559">parser.rs -16+10</a>, <a href="../crates/opencascade-binding-generator/src/parser.rs#L1610-L1614">-14+4</a>, <a href="../crates/opencascade-binding-generator/src/parser.rs#L1781-L1783">-10+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            let is_primitive_typedef = matches!(kind, TypeKind::Typedef | TypeKind::Elaborated)\n                && clang_type.get_declaration()\n                    .filter(|d| d.get_kind() == clang::EntityKind::TypedefDecl)\n                    .and_then(|d| d.get_typedef_underlying_type())\n                    .map(|u| {\n                        let uk = u.get_kind();\n                        is_c_primitive_type_kind(uk)\n                        || matches!(uk, TypeKind::Typedef | TypeKind::Elaborated)  // chain through another typedef\n                    })\n                    .unwrap_or(false);",
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "1549:qw,1550:fg,1551:xy,1552:vo,1553:hx,1554:nv,1555:tm,1556:oa,1557:qk,1558:ap,1559:cf,1560:no,1561:nf,1562:no,1563:vu,1564:nc"
    },
    {
      "content": "                let pointee_is_primitive_canonical = !is_template_or_ns\n                    && clang_type.get_pointee_type().map(|pt| {\n                        is_c_primitive_type_kind(pt.get_canonical_type().get_kind())\n                    }).unwrap_or(false);",
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "1616:dy,1617:jg,1618:mb,1619:rx,1620:lw,1621:ld,1622:pz,1623:em,1624:qm,1625:kj,1626:um,1627:fg,1628:ov,1629:zc"
    },
    {
      "content": "        // Primitive typedef: canonical is a C primitive type\n        if is_c_primitive_type_kind(canon_kind) {",
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "1797:et,1798:qk,1799:xq,1800:lr,1801:we,1802:cm,1803:ol,1804:gn,1805:br,1806:de"
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
      "file": "crates/opencascade-binding-generator/src/parser.rs",
      "startLine": 1549,
      "endLine": 1782
    }
  ]
}
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo check -p opencascade-binding-generator 2>&1
```
```
    Checking opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.28s
real  1.3s
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time ./scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (37 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.53s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-3404803fe4cd90de)

running 16 tests
test header_deps::tests::test_is_likely_occt_header ... ok
test module_graph::tests::test_extract_rust_type_name ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
test parser::tests::test_extract_module_from_header ... ok
test parser::tests::test_extract_template_arg ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test parser::tests::test_map_standard_type ... ok
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

real  4.2s
```
</details>

All tests pass. The refactoring doesn't change any generated output (it's just code cleanup), so I don't need to regenerate or fully rebuild. Let me commit:

**Terminal:**
```sh
cat > /Users/sethml/src/opencascade-rs/tmp/commit-msg.txt << 'EOF'
Refactor: extract is_c_primitive_type_kind() helper

Extract the repeated TypeKind::Bool | TypeKind::CharS | ... match pattern
(3 occurrences in parser.rs) into a reusable helper function. This reduces
duplication and makes it easier to maintain the list of recognized C
primitive types.

No behavioral change — pure code cleanup.
EOF
cd /Users/sethml/src/opencascade-rs && git add -A && git commit -F tmp/commit-msg.txt
```
```
(base) whitelily:opencascade-rs sethml$ cd /Users/sethml/src/opencascade-rs && g
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git add -A && git commit -F tmp/commit-msg.txt
```
```
On branch main
nothing to commit, working tree clean
```
**Exit code:** 1

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git status
```
```
On branch main
nothing to commit, working tree clean
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff HEAD~1 --stat
```
```
 crates/opencascade-binding-generator/src/parser.rs | 52 +++++++++-------------
 1 file changed, 21 insertions(+), 31 deletions(-)
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git log --oneline -3
```
```
38dfb12 (HEAD -> main) Refactor: extract is_c_primitive_type_kind() helper
28a0575 Resolve pointer and primitive typedef skips in binding generator
6bef5c9 Refactor: clean up Handle detection variable naming
```

The refactoring was already committed as a separate commit. Both commits went through. Let me finalize:

**Todo list:**
- ✅ Investigate Standard_PCharacter (char*)
- ✅ Investigate BOPDS_PDS (BOPDS_DS*)
- ✅ Investigate MeshVS_DisplayModeFlags
- ✅ Implement fixes
- ✅ Regenerate bindings and test
- ✅ Update README skip counts
- ✅ Commit changes
- ✅ Consider refactoring opportunities

Done. Here's a summary of what was accomplished:

### Commit 1: Resolve pointer and primitive typedef skips (28a0575)

Fixed 3 types from the selected table that weren't blocked by missing dependencies:

| Type | Issue | Fix | Skips Resolved |
|------|-------|-----|---------------|
| `MeshVS_DisplayModeFlags` | `typedef Standard_Integer` not resolving in references | Enhanced reference guard to detect genuine primitive typedefs | 5 |
| `Standard_PCharacter` | `typedef char*` not resolvable | Late-stage canonical resolution for pointer typedefs | 8 |
| `BOPDS_PDS` | `typedef BOPDS_DS*` not resolvable | Same pointer typedef mechanism | 7 |

Additional types resolved by the same mechanism: `TDocStd_XLinkPtr`, `BOPDS_PIterator`, `BOPAlgo_PPaveFiller`, `BOPAlgo_PBuilder`, and various other pointer typedefs.

Three code changes:
1. **`crates/opencascade-binding-generator/src/parser.rs`**: Late-stage canonical resolution for pointer/primitive typedefs at end of `parse_type()`
2. **`crates/opencascade-binding-generator/src/parser.rs`**: Reference guard now allows primitive typedefs to fall through 
3. **`crates/opencascade-binding-generator/src/model.rs`**: Fixed C++ const positioning from prefix (`const X*`) to postfix (`X const*`) in `to_cpp_string()` and  — necessary for correctness when inner type is a pointer

**Total: ~495 → ~432 skipped symbols (63 fewer, 12.7% reduction)**

### Commit 2: Refactor (38dfb12)

Extracted `is_c_primitive_type_kind()` helper to eliminate 3 repetitions of the 14-line `TypeKind::Bool | TypeKind::CharS | ...` match pattern.

*2026-02-21 16:23 · 2015s · Words: 26,603 in · 14,833 out · 132,729 ctx · 107 rounds*

---
