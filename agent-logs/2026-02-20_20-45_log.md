# Reinstate exclusions and build success commit

- **Date:** 2026-02-20 20:23 – 21:44
- **Models:** Claude Opus 4.6, Claude Sonnet 4.6, Gpt 4.1
- **Turns:** 6
- **Tool calls:** 298
- **Thinking blocks:** 16
- **Input words:** 35,313
- **Output words:** 9,001
- **Prompt tokens (last round):** 326,120
- **API rounds:** 159
- **Total elapsed:** 3811s

## Table of Contents

1. [Un-exclude these and then see if you can get the result to build. Commit when you succeed.](#user-1) (Claude Opus 4.6)
2. [Fix the build errors and warnings when building opencascade-sys. Then commit.](#user-2) (Claude Opus 4.6)
3. [What is BRepMeshData for in OCCT?](#user-3) (Claude Sonnet 4.6)
4. [Can you access the BRepMeshData objects, or are they not useful for a library client?](#user-4) (Gpt 4.1)
5. [Can we make these work? Even without new/delete, it should be possible to access these objects, i...](#user-5) (Claude Opus 4.6)
6. [Does this exclude entire classes, or just their desructors? If it&#x27;s just their destructors, then ...](#user-6) (Claude Opus 4.6)

---

<a id="user-1"></a>

## User (1)

> Un-exclude these and then see if you can get the result to build. Commit when you succeed.

*2026-02-20 20:24*

### Assistant — Claude Opus 4.6

> 💭 The user wants me to un-exclude the visualization/rendering modules from `bindings.toml` (lines 27-46 in the selection) and then get the project to build. Let me start by understanding what needs to be done:
> 
> 1. Remove the visualization/rendering modules from `exclude_modules` in bindings.toml
> 2. Regenerate bindings
> 3. Try to build and fix any issues
> 4. Commit when successful
> 
> Let me first read the current state of the file and understand the modules to un-exclude.
> 
> The selected modules to un-exclude are:
> - AIS
> - Aspect
> - D3DHost
> 
> 
> - DsgPrs
> - Graphic3d
> - MeshVS
> - OpenGl
> - Prs3d
> - PrsDim
> - PrsMgr
> - Select3D
> - SelectBasics
> - SelectMgr
> - StdPrs
> - StdSelect
> - TPrsStd
> - V3d
> - XCAFPrs
> 
> Now I'll load the necessary tools and start editing the file to remove these modules from the exclusion list.

**Todo list:**
- 🔄 Remove viz module exclusions
- ⬜ Regenerate bindings
- ⬜ Build and fix errors
- ⬜ Run tests
- ⬜ Commit changes

<details>
<summary>Reading lines 20-50 of <a href="../crates/opencascade-sys/bindings.toml#L20-L51">bindings.toml</a></summary>

```
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
<summary>**Failing: <a href="../crates/opencascade-sys/bindings.toml#L26">bindings.toml -1</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "26:dd\n27:tg\n28:tt\n29:ou\n30:bo\n31:dq\n32:td\n33:ha\n34:ix\n35:bq\n36:fw\n37:oa\n38:ay\n39:sj\n40:vv\n41:ep\n42:ml\n43:yy\n44:cf\n45:zr",
      "content": ""
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":26,"endLine":44}
```
</details>

**Todo list:**
- ✅ Remove viz module exclusions
- 🔄 Regenerate bindings
- ⬜ Build and fix errors
- ⬜ Run tests
- ⬜ Commit changes

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && tail -20 /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01Djyr7nzWmaGKvC4DHKcXeR__vscode-1771608612245/content.txt
```
<details>
<summary>Output (20 lines)</summary>

```
lection_primes.rs (function-only module, 1 functions)                             Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/step_
to_geom.rs (function-only module, 48 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/rw_ob
j_tools.rs (function-only module, 3 functions)                                    Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/xb_re
p_mesh.rs (function-only module, 1 functions)                                     Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/preci
sion.rs (function-only module, 18 functions)                                      Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/el_c_
lib.rs (function-only module, 120 functions)                                      Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/utl.r
s (function-only module, 18 functions)                                            Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/b_rep
_bnd_lib.rs (function-only module, 4 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/geom_
proj_lib.rs (function-only module, 8 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/hermi
t.rs (function-only module, 3 functions)                                          Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/el_s_
lib.rs (function-only module, 76 functions)                                       Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/lib.r
s                                                                               
Formatting generated Rust code with rustfmt...
  Formatting complete.

Code generation complete!
  318 modules generated

Bindings generated in /Users/sethml/src/opencascade-rs/crates/opencascade-sys/ge
nerated                                                                         real  42.8s
```
</details>

Bindings regenerated successfully (318 modules). Now let's build.

**Todo list:**
- ✅ Remove viz module exclusions
- ✅ Regenerate bindings
- 🔄 Build and fix errors
- ⬜ Run tests
- ⬜ Commit changes

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```
<details>
<summary>Output (415 lines) — exit code 101</summary>

```
   Compiling opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)
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
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:18679:11: error: 'SetLocalTransformation' is a protected member of 'AIS_Manipulator'
warning: opencascade-sys@0.2.0:  18679 |     self->SetLocalTransformation(theTrsf);
warning: opencascade-sys@0.2.0:        |     ~~~~~~^~~~~~~~~~~~~~~~~~~~~~
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/AIS/AIS_Manipulator.hxx:455:32: note: declared protected here
warning[
... (truncated, 37807 chars total)
```
</details>

**Terminal:**
```sh
grep -c "^error" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01WRjRjwyf4dGFKJ7ZkbGx4t__vscode-1771608612248/content.txt 2>/dev/null; grep "^error" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01WRjRjwyf4dGFKJ7ZkbGx4t__vscode-1771608612248/content.txt 2>/dev/null
```
```
0
```
**Exit code:** 1

**Terminal:**
```sh
grep -i "error" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01WRjRjwyf4dGFKJ7ZkbGx4t__vscode-1771608612248/content.txt 2>/dev/null
```
<details>
<summary>Output (46 lines)</summary>

```
ade-sys/generated/wrappers.cpp:18679:11: error: 'SetLocalTransformation' is a protected member of 'AIS_Manipulator'                                             warning: opencascade-sys@0.2.0:  18679 |     self->SetLocalTransformation(theTrs
ade-sys/generated/wrappers.cpp:21366:18: error: cannot refer to type member 'DisplayMode' in 'const AIS_PointCloud' with '->'                                   warning: opencascade-sys@0.2.0:  21366 |     return self->DisplayMode();
ade-sys/generated/wrappers.cpp:61777:12: error: unknown type name 'MeshVS_NodePair'                                                                             warning: opencascade-sys@0.2.0:  61777 | extern "C" MeshVS_NodePair* BRepMesh_Ge
ade-sys/generated/wrappers.cpp:61778:16: error: unknown type name 'MeshVS_NodePair'                                                                             warning: opencascade-sys@0.2.0:  61778 |     return new MeshVS_NodePair(BRepMesh
ade-sys/generated/wrappers.cpp:134835:18: error: no matching member function for call to 'Init'                                                                 warning: opencascade-sys@0.2.0:  134835 |     return self->Init(theNbElems, theA
ade-sys/generated/wrappers.cpp:135761:18: error: no matching member function for call to 'Init'                                                                 warning: opencascade-sys@0.2.0:  135761 |     return self->Init(theNbElems, theA
ade-sys/generated/wrappers.cpp:441676:18: error: 'IsHeadlight' is a private member of 'V3d_AmbientLight'                                                        warning: opencascade-sys@0.2.0:  441676 |     return self->IsHeadlight();
ade-sys/generated/wrappers.cpp:441679:18: error: 'Headlight' is a private member of 'V3d_AmbientLight'                                                          warning: opencascade-sys@0.2.0:  441679 |     return self->Headlight();
ade-sys/generated/wrappers.cpp:441682:11: error: 'SetHeadlight' is a private member of 'V3d_AmbientLight'                                                       warning: opencascade-sys@0.2.0:  441682 |     self->SetHeadlight(theValue);
ade-sys/generated/wrappers.cpp:441685:18: error: 'Position' is a private member of 'V3d_AmbientLight'                                                           warning: opencascade-sys@0.2.0:  441685 |     return self->Position();
ade-sys/generated/wrappers.cpp:441688:11: error: 'SetPosition' is a private member of 'V3d_AmbientLight'                                                        warning: opencascade-sys@0.2.0:  441688 |     self->SetPosition(thePosition);
ade-sys/generated/wrappers.cpp:441691:18: error: 'ConstAttenuation' is a private member of 'V3d_AmbientLight'                                                   warning: opencascade-sys@0.2.0:  441691 |     return self->ConstAttenuation();
ade-sys/generated/wrappers.cpp:441694:18: error: 'LinearAttenuation' is a private member of 'V3d_AmbientLight'
... (truncated, 10176 chars total)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_014uWxsD9qd9KiYftHhYnMtg__vscode-1771608612250/content.txt)

> 💭 I see several categories of errors:
> 
> 1. **Protected member access**: `AIS_Manipulator::SetLocalTransformation` is protected
> 2. **Type member vs instance member**: `AIS_PointCloud::DisplayMode` - cannot refer to type member with `->`
> 3. **Unknown type**: `MeshVS_NodePair` - unknown type name
> 4. **No matching member function**: `Init` calls on Graphic3d classes
> 5. **Private member access**: Many V3d_AmbientLight methods are private (inherited from V3d_Light but made private in subclass)
> 
> I need to fix the binding generator to handle these issues, or add exclusions for the problematic methods/classes. Let me look at the patterns:
> 
> 
> The cleanest solution is to add these problematic methods and classes to the exclusion list in bindings.toml rather than trying to fix the binding generator itself. For `V3d_AmbientLight`, all those methods are private, so I need to exclude them entirely. Let me check if the binding generator already has logic to detect and skip private or protected methods, since that would be more efficient than manually excluding each one.

I see 20 compile errors in several categories. Let me analyze them and fix them systematically.

Research binding generator access control

<details>
<summary>Searched for regex <code>protected|private|access</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 20 results</summary>

- [parser.rs:11](../crates/opencascade-binding-generator/src/parser.rs#L11)
- [parser.rs:416](../crates/opencascade-binding-generator/src/parser.rs#L416)
- [parser.rs:461](../crates/opencascade-binding-generator/src/parser.rs#L461)
- [parser.rs:462](../crates/opencascade-binding-generator/src/parser.rs#L462)
- [parser.rs:469](../crates/opencascade-binding-generator/src/parser.rs#L469)
- [parser.rs:470](../crates/opencascade-binding-generator/src/parser.rs#L470)
- [parser.rs:490](../crates/opencascade-binding-generator/src/parser.rs#L490)
- [parser.rs:491](../crates/opencascade-binding-generator/src/parser.rs#L491)
- [parser.rs:492](../crates/opencascade-binding-generator/src/parser.rs#L492)
- [parser.rs:493](../crates/opencascade-binding-generator/src/parser.rs#L493)
- [parser.rs:495](../crates/opencascade-binding-generator/src/parser.rs#L495)
- [parser.rs:497](../crates/opencascade-binding-generator/src/parser.rs#L497)
- [parser.rs:501](../crates/opencascade-binding-generator/src/parser.rs#L501)
- [parser.rs:502](../crates/opencascade-binding-generator/src/parser.rs#L502)
</details>

<details>
<summary>Searched for text <code>exclude_methods</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 20 results</summary>

- [main.rs:96](../crates/opencascade-binding-generator/src/main.rs#L96)
- [main.rs:113](../crates/opencascade-binding-generator/src/main.rs#L113)
- [main.rs:120](../crates/opencascade-binding-generator/src/main.rs#L120)
- [main.rs:123](../crates/opencascade-binding-generator/src/main.rs#L123)
- [main.rs:131](../crates/opencascade-binding-generator/src/main.rs#L131)
- [main.rs:215](../crates/opencascade-binding-generator/src/main.rs#L215)
- [main.rs:376](../crates/opencascade-binding-generator/src/main.rs#L376)
- [main.rs:394](../crates/opencascade-binding-generator/src/main.rs#L394)
- [main.rs:433](../crates/opencascade-binding-generator/src/main.rs#L433)
- [main.rs:434](../crates/opencascade-binding-generator/src/main.rs#L434)
- [main.rs:595](../crates/opencascade-binding-generator/src/main.rs#L595)
- [main.rs:622](../crates/opencascade-binding-generator/src/main.rs#L622)
- [config.rs:42](../crates/opencascade-binding-generator/src/config.rs#L42)
- [bindings.rs:1509](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1509)
- [bindings.rs:1522](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1522)
- [bindings.rs:1523](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1523)
- [bindings.rs:1614](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1614)
- [bindings.rs:1780](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1780)
- [bindings.rs:1916](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1916)
- [bindings.rs:2597](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2597)
</details>

Searched for regex `V3d_AmbientLight|AIS_Manipulator|AIS_PointCloud|MeshVS_NodePair|Graphic3d_Buffer` (`**/crates/opencascade-binding-generator/**`), 1 result
- [README.md:563](../crates/opencascade-binding-generator/README.md#L563)

<details>
<summary>Reading lines 410-560 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L410-L561">parser.rs</a></summary>

```

    // Skip anonymous classes/structs
    if name.is_empty() {
        return Vec::new();
    }

    // Skip internal/private classes (those starting with underscore)
    if name.starts_with('_') {
        return Vec::new();
    }

    // Skip template classes and template specializations
    // Template classes have get_template() returning Some, or get_template_kind() returning Some
    // Also skip if the display name contains angle brackets (indicates template instantiation)
    if entity.get_template().is_some() {
        if verbose {
            println!("    Skipping {} (template class)", name);
        }
        return Vec::new();
    }
    let display_name = entity.get_display_name().unwrap_or_default();
    if display_name.contains('<') {
        if verbose {
            println!("    Skipping {} (template specialization)", display_name);
        }
        return Vec::new();
    }

    // Skip policy/trait classes used as template parameters
    // These are not meant to be instantiated directly
    if name.contains("Inspector") || name.contains("_Hasher") || name.contains("_Traits") {
        if verbose {
            println!("    Skipping {} (policy/trait class)", name);
        }
        return Vec::new();
    }

    // Skip internal node types that use custom allocators (can't be used with std::unique_ptr)
    if name.ends_with("Node") && name.starts_with("NCollection_") {
        if verbose {
            println!("    Skipping {} (internal node type)", name);
        }
        return Vec::new();
    }

    let comment = extract_doxygen_comment(entity);
    let module = extract_module_from_header(source_header);

    // Extract direct base classes for upcast generation
    let base_classes = extract_base_classes(entity);
    
    // Check for protected/private destructor (indicates non-instantiable abstract base class)
    let has_protected_destructor = check_protected_destructor(entity);

    if verbose {
        println!("  Parsing class: {}", name);
        if !base_classes.is_empty() {
            println!("    Base classes: {:?}", base_classes);
        }
        if has_protected_destructor {
            println!("    Has protected destructor (non-instantiable)");
        }
    }

    let mut constructors = Vec::new();
    let mut methods = Vec::new();
    let mut static_methods = Vec::new();
    let mut fields: Vec<ParsedField> = Vec::new();
    let mut has_non_public_fields = false;
    let mut has_virtual_methods = false;
    let mut all_method_names = std::collections::HashSet::new();
    let mut is_abstract = false;
    let mut pure_virtual_methods = std::collections::HashSet::new();
    let mut has_explicit_constructors = false;
    // Track copy constructor: None = no explicit copy ctor seen,
    // Some(true) = public non-deleted copy ctor, Some(false) = deleted/non-public copy ctor
    let mut has_copy_constructor: Option<bool> = None;
    let mut has_move_constructor = false;
    let mut nested_classes: Vec<ParsedClass> = Vec::new();

    // Track current access level for nested type visibility.
    // Default: `class` => private, `struct` => public.
    let default_access = if entity.get_kind() == EntityKind::StructDecl {
        Accessibility::Public
    } else {
        Accessibility::Private
    };
    let current_access = std::cell::Cell::new(default_access);


    entity.visit_children(|child, _| {
        // Track access specifiers (public:/protected:/private: sections)
        if child.get_kind() == EntityKind::AccessSpecifier {
            if let Some(acc) = child.get_accessibility() {
                current_access.set(acc);
            }
            return EntityVisitResult::Continue;
        }

        match child.get_kind() {
            EntityKind::Constructor => {
                // Any explicit constructor means C++ won't generate an implicit default
                has_explicit_constructors = true;

                // Detect copy constructors via libclang
                if child.
... (truncated, 6384 chars)
```
</details>

<details>
<summary>Reading lines 120-170 of <a href="../crates/opencascade-binding-generator/src/model.rs#L120-L171">model.rs</a></summary>

```
/// `#[repr(C)]` Rust struct with real fields instead of an opaque type.
#[derive(Debug, Clone)]
pub struct ParsedClass {
    /// Full class name (e.g., "gp_Pnt", "BRepPrimAPI_MakeBox")
    pub name: String,
    /// Module name extracted from prefix (e.g., "gp", "BRepPrimAPI")
    pub module: String,
    /// Documentation comment from the header
    pub comment: Option<String>,
    /// Source header file name (e.g., "gp_Pnt.hxx")
    pub source_header: String,
    /// Source line number in the header file
    pub source_line: Option<u32>,
    /// Constructors
    pub constructors: Vec<Constructor>,
    /// Instance methods (public only)
    pub methods: Vec<Method>,
    /// Static methods (public only)
    pub static_methods: Vec<StaticMethod>,
    /// All method names in this class (including protected/private) - used for filtering inherited methods
    pub all_method_names: std::collections::HashSet<String>,
    /// Direct base classes (for generating upcast helpers)
    pub base_classes: Vec<String>,
    /// Whether this class has a protected/private destructor (non-instantiable abstract base)
    pub has_protected_destructor: bool,
    /// Whether this class is abstract (has pure virtual methods)
    pub is_abstract: bool,
    /// Names of pure virtual methods declared in this class
    pub pure_virtual_methods: HashSet<String>,
    /// Whether this class has any explicit constructor declarations (public or not).
    /// If true, C++ won't generate an implicit default constructor.
    pub has_explicit_constructors: bool,
    /// Public data members (fields)
    pub fields: Vec<ParsedField>,
    /// Whether this class is a POD struct (all public fields, no virtuals, trivially copyable)
    pub is_pod_struct: bool,
    /// Whether this class has a usable (public, non-deleted) copy constructor.
    /// - `Some(true)`: explicit public, non-deleted copy constructor found
    /// - `Some(false)`: explicit copy constructor found but deleted or non-public
    /// - `None`: no explicit copy constructor (implicit may exist based on C++ rules)
    pub has_copy_constructor: Option<bool>,
    /// Whether this class has an explicit move constructor.
    /// A move constructor suppresses the implicit copy constructor in C++.
    pub has_move_constructor: bool,
}

impl ParsedClass {
    /// Get the class name without the module prefix (e.g., "Pnt" from "gp_Pnt")
    /// For nested classes (e.g., "Poly_CoherentTriangulation::TwoIntegers"),
    /// returns the name after the first underscore, which may contain "::". 
    /// Callers that need correct short names for nested types should use
```
</details>

<details>
<summary>Reading lines 1-80 of <a href="../crates/opencascade-binding-generator/src/config.rs#L1-L81">config.rs</a></summary>

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

    /// Exclude specific classes/types from binding generation.
    /// These classes will not get any bindings (no struct, no methods, no destructor).
    /// For nested types, use the C++ qualified name: "Parent::Nested".
    #[serde(default)]
    pub exclude_classes: Vec<String>,

    /// Opaque types defined in manual/ files but referenced by auto-generated bindings.
    /// The generator adds these to the known class set so methods using them
    /// aren't skipped as "unknown type".
    /// Format: `TypeName = { header = "Header.hxx" }`
    #[serde(default)]
    pub manual_types: std::collections::HashMap<String, ManualType>,
}

/// A manually-defined opaque type referenced by auto-generated bindings.
#[derive(Debug, Deserialize)]
pub struct ManualType {
    /// The C++ header that defines this type (for wrappers.cpp includes).
    pub header: String,
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
```
</details>

<details>
<summary>Reading lines 1500-1630 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1500-L1631">bindings.rs</a></summary>

```
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
        // Record skipped constructors for abstract/protected-destructor classes
        if effectively_abstract {
            for ctor in &class.constructors {
                skipped_symbols.push(SkippedSymbol {
                    kind: "constructor",
                    module: class.module.clone(),
                    cpp_name: format!("{}::{}", class.name, class.name),
                    source_header: class.source_header.clone(),
                    source_line: ctor.source_line,
                    doc_comment: ctor.comment.clone(),
                    skip_reason: "class is abstract (has unimplemented pure virtual methods)".to_string(),
                    stub_rust_decl: generate_ctor_stub(cpp_name, ctor),
                });
            }
        } else if class.has_protected_destructor {
            for ctor in &class.constructors {
                skipped_symbols.push(SkippedSymbol {
                    kind: "constructor",
                    module: class.module.clone(),
                    cpp_name: format!("{}::{
... (truncated, 6377 chars)
```
</details>

<details>
<summary>Reading lines 557-700 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L557-L701">parser.rs</a></summary>

```
            EntityKind::Method => {
                // Check for virtual methods (affects POD detection)
                if child.is_virtual_method() {
                    has_virtual_methods = true;
                }
                // Check if this is a pure virtual method (makes the class abstract)
                if child.is_pure_virtual_method() {
                    is_abstract = true;
                    if let Some(ref method_name) = child.get_name() {
                        pure_virtual_methods.insert(method_name.clone());
                    }
                }

                // Skip destructors, operators, and conversion functions
                if let Some(ref method_name) = child.get_name() {
                    if method_name.starts_with('~')
                        || method_name.starts_with("operator")
                        || method_name == "DumpJson"
                        || method_name == "InitFromJson"
                    {
                        return EntityVisitResult::Continue;
                    }

                    // Always track all method names (even if not public) - used for filtering inherited methods
                    all_method_names.insert(method_name.clone());
                }

                // Skip deprecated methods
                if child.get_availability() == Availability::Deprecated {
                    if verbose {
                        if let Some(ref method_name) = child.get_name() {
                            println!("    Skipping deprecated method {}::{}", name, method_name);
                        }
                    }
                    return EntityVisitResult::Continue;
                }

                if is_public(&child) {
                    if child.is_static_method() {
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
                    for nested in &mut parsed {
                        // Qualify the nested class name with parent: Parent::Nested
                        // Always prepend since multi-level nesting (A::B::C) needs all levels.
                        nested.name = format!("{}::{}", name, nested.name);
                        // nested.module is inherited from source_header
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
            _ => {}
        }
        EntityVisitResult::Continue
    });

    // Only return classes that have something to bind
    if constructo
... (truncated, 5844 chars)
```
</details>

<details>
<summary>Reading lines 2590-2700 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2590-L2701">bindings.rs</a></summary>

```
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
        if let Some(ancestor_class) = symbol_table.class_by_name(ancestor_name) {
            let ancestor_methods = symbol_table.included_methods(ancestor_class);

            // Collect public method names for this ancestor.
            let ancestor_public_names: HashSet<&str> =
                ancestor_methods.iter().map(|m| m.cpp_name.as_str()).collect();
            // Any method declared by this ancestor (in all_method_names) that is
            // NOT publicly exposed has been narrowed (protected/private override).
            // Block it from being inherited from further-up ancestors.
            for method_name in &ancestor_class.all_method_names {
                if !ancestor_public_names.contains(method_name.as_str()) {
                    protected_in_ancestors.insert(method_name.clone());
                }
            }

            for resolved_method in ancestor_methods {
                if existing_method_names.contains(&resolved_method.cpp_name) {
                    continue;
                }
                if class.all_method_names.contains(&resolved_method.cpp_name) {
                    continue;
                }
                if seen_methods.contains(&resolved_method.cpp_name) {
                    continue;
                }
                // Skip methods narrowed to protected/private in an intermediate ancestor.
                // Example: BOPAlgo_PaveFiller overrides BOPAlgo_Options::Clear() as
                // protected; BOPAlgo_CheckerSI must not inherit Clear() from Options.
                if protected_in_ancestors.contains(&resolved_method.cpp_name) {
                    continue;
                }

                // Skip inherited methods that are explicitly excluded for the child
                // class or for the ancestor class (same method, misresolved type).
                if exclude_methods.contains(&(class.name.clone(), resolved_method.cpp_name.clone()))
                    || exclude_methods.contains(&(ancestor_name.clone(), resolved_method.cpp_name.clone()))
                {
                    continue;
                }

                seen_methods.insert(resolved_method.cpp_name.clone());

                // Skip methods with raw pointers (but allow nullable pointer params)
                let uses_raw_pointers = resolved_method.params.iter().any(|p| {
                    (p.ty.rust_ffi_type.contains("*const")
                        || p.ty.rust_ffi_type.contains("*mut"))
                        && !p.is_nullable_ptr()
                        && p.ty.original.class_ptr_inner_name().is_none()
                })
                    || resolved_method
                        .return_type
                        .as_ref()
                        .map(|rt| {
                            (rt.rust_ffi_type.contains("*const")
                                || rt.rust_ffi_type.contains("*mut"))
                                && r
... (truncated, 5322 chars)
```
</details>

<details>
<summary>Reading lines 698-740 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L698-L741">parser.rs</a></summary>

```
/// Check if a class has a protected or private destructor
/// Classes with non-public destructors cannot be directly instantiated via the FFI
fn check_protected_destructor(entity: &Entity) -> bool {
    for child in entity.get_children() {
        if child.get_kind() == EntityKind::Destructor {
            // Check if the destructor is not public
            if let Some(accessibility) = child.get_accessibility() {
                return accessibility != clang::Accessibility::Public;
            }
        }
    }
    false
}

/// Parse an enum declaration
fn parse_enum(entity: &Entity, source_header: &str, verbose: bool) -> Option<ParsedEnum> {
    let name = entity.get_name()?;

    // Skip anonymous enums (empty name or compiler-generated "(unnamed enum at ...)")
    if name.is_empty() || name.starts_with("(unnamed") {
        return None;
    }

    // Skip internal enums
    if name.starts_with('_') {
        return None;
    }

    // Skip nested enums (enums defined inside a class/struct)
    // These are not accessible at global scope
    if let Some(parent) = entity.get_semantic_parent() {
        let parent_kind = parent.get_kind();
        if parent_kind == EntityKind::ClassDecl || parent_kind == EntityKind::StructDecl {
            if verbose {
                println!("    Skipping {} (nested enum inside class)", name);
            }
            return None;
        }
    }

    let comment = extract_doxygen_comment(entity);
    let module = extract_module_from_header(source_header);

```
</details>

<details>
<summary>Searched for text <code>MeshVS_NodePair</code>, 20 results</summary>

- [mesh_vs.rs:9265](../crates/opencascade-sys/generated/mesh_vs.rs#L9265)
- [b_rep_mesh.rs:5347](../crates/opencascade-sys/generated/b_rep_mesh.rs#L5347)
- [ffi.rs:36315](../crates/opencascade-sys/generated/ffi.rs#L36315)
- [ffi.rs:107610](../crates/opencascade-sys/generated/ffi.rs#L107610)
- [ffi.rs:676617](../crates/opencascade-sys/generated/ffi.rs#L676617)
- [ffi.rs:678949](../crates/opencascade-sys/generated/ffi.rs#L678949)
- [ffi.rs:678951](../crates/opencascade-sys/generated/ffi.rs#L678951)
- [wrappers.cpp:61777](../crates/opencascade-sys/generated/wrappers.cpp#L61777)
- [wrappers.cpp:61778](../crates/opencascade-sys/generated/wrappers.cpp#L61778)
- [wrappers.cpp:474111](../crates/opencascade-sys/generated/wrappers.cpp#L474111)
- [MeshVS_NodalColorPrsBuilder.cxx:611](../crates/occt-sys/OCCT/src/MeshVS/MeshVS_NodalColorPrsBuilder.cxx#L611)
- [MeshVS_NodalColorPrsBuilder.cxx:625](../crates/occt-sys/OCCT/src/MeshVS/MeshVS_NodalColorPrsBuilder.cxx#L625)
- [MeshVS_NodalColorPrsBuilder.cxx:630](../crates/occt-sys/OCCT/src/MeshVS/MeshVS_NodalColorPrsBuilder.cxx#L630)
- [MeshVS_SymmetricPairHasher.hxx:20](../crates/occt-sys/OCCT/src/MeshVS/MeshVS_SymmetricPairHasher.hxx#L20)
- [MeshVS_SymmetricPairHasher.hxx:28](../crates/occt-sys/OCCT/src/MeshVS/MeshVS_SymmetricPairHasher.hxx#L28)
- [MeshVS_SymmetricPairHasher.hxx:39](../crates/occt-sys/OCCT/src/MeshVS/MeshVS_SymmetricPairHasher.hxx#L39)
- [MeshVS_MeshPrsBuilder.cxx:413](../crates/occt-sys/OCCT/src/MeshVS/MeshVS_MeshPrsBuilder.cxx#L413)
</details>

Searched for text `fn is_public` (`**/crates/opencascade-binding-generator/src/**`), 1 result
- [parser.rs:920](../crates/opencascade-binding-generator/src/parser.rs#L920)

<details>
<summary>Reading lines 920-945 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L920-L946">parser.rs</a></summary>

```
fn is_public(entity: &Entity) -> bool {
    entity.get_accessibility() == Some(clang::Accessibility::Public)
}

/// Check if a method should be bound based on OCCT conventions
/// Binds: Standard_EXPORT methods OR public methods with doc comments
#[allow(dead_code)]
fn should_bind_method(entity: &Entity) -> bool {
    // Check for Standard_EXPORT by looking at the display name or attributes
    // Standard_EXPORT methods are always bindable
    if has_standard_export(entity) {
        return true;
    }

    // Public methods with documentation comments are also bindable
    // (these are typically inline getters/setters)
    if entity.get_comment().is_some() {
        return true;
    }

    // For now, bind all public methods - can be refined later
    true
}

/// Check if a method has Standard_EXPORT annotation
fn has_standard_export(entity: &Entity) -> bool {
```
</details>

Searched for regex `V3d_AmbientLight|AIS_Manipulator|Graphic3d_Buffer` (`**/crates/opencascade-binding-generator/src/**`), no results

Searched for text `exclude_methods` (`**/crates/opencascade-sys/bindings.toml`), 1 result
- [bindings.toml:86](../crates/opencascade-sys/bindings.toml#L86)

Searched for text `exclude_classes` (`**/crates/opencascade-sys/bindings.toml`), 1 result
- [bindings.toml:98](../crates/opencascade-sys/bindings.toml#L98)

<details>
<summary>Reading lines 86-130 of <a href="../crates/opencascade-sys/bindings.toml#L86-L131">bindings.toml</a></summary>

```
exclude_methods = [
    # C++ `New()` factory method name-collides with constructor wrapper `fn new()`
    "Storage_CallBack::New",
    "Storage_DefaultCallBack::New",

    # `const char**` (double pointer) mapped as `&mut &str` but FFI needs raw pointer
    "XmlObjMgt::GetInteger",
    "XmlObjMgt::GetReal",
]

# Exclude specific classes/types from destructor generation.
# Forward-declared-only nested types cannot be safely deleted.
exclude_classes = [
    "Message_ProgressScope::NullString",  # Forward-declared only, no full definition
]

[general]
resolve_deps = true

# Opaque types defined in manual/ files. These are not parsed from OCCT headers
# but referenced by auto-generated bindings. The generator adds them to the known
# class set so methods using them aren't skipped as "unknown type".
# Format: "CppTypeName" = { header = "Header.hxx" }
[manual_types]
Standard_OStream = { header = "Standard_OStream.hxx" }
Standard_IStream = { header = "Standard_IStream.hxx" }
Standard_SStream = { header = "Standard_SStream.hxx" }

```
</details>

<details>
<summary>Reading lines 2560-2590 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2560-L2591">bindings.rs</a></summary>

```
            } else {
                false
            }
        })
        .map(|derived_class| {
            let derived_handle_name = type_mapping::handle_type_name(derived_class);
            let ffi_fn_name =
                format!("{}_downcast_to_{}", handle_type_name, derived_handle_name);
            // Flatten nested class names (e.g., "Parent::Nested" -> "Parent_Nested")
            // for module extraction heuristic.
            let flattened = derived_class.replace("::", "_");
            let derived_module = if let Some(underscore_pos) = flattened.find('_') {
                flattened[..underscore_pos].to_string()
            } else {
                flattened.clone()
            };

            HandleDowncastBinding {
                derived_handle_name,
                derived_class: derived_class.clone(),
                derived_module,
                base_handle_name: handle_type_name.clone(),
                ffi_fn_name,
            }
        })
        .collect()
}

// ── Inherited method bindings ───────────────────────────────────────────────────
fn compute_inherited_method_bindings(
    class: &ParsedClass,
```
</details>

> 💭 Good, the generator already has logic to skip protected/private methods in the class itself, and even to track "narrowed" inherited methods. But clearly there are edge cases it's missing. Let me understand each error:
> 
> 1. **`AIS_Manipulator::SetLocalTransformation` is protected** -- This must be an inherited method that's protected in AIS_Manipulator but public in an ancestor. The generator's `protected_in_ancestors` logic should catch this, but maybe it's not working for this case. Let me look at the specific class hierarchy.
> 
> 2. **`AIS_PointCloud::DisplayMode` -- cannot refer to type member with '->'** -- `DisplayMode` is a type (enum) in AIS_PointCloud that shadows an inherited method. 
> 
> The generator is incorrectly including it as a method when it's actually a type alias. For the typedef cases like `MeshVS_NodePair`, the generator thinks it's a class but it's really just a `std::pair` alias, so it can't be used as a return type. Then there are overloaded methods like `Graphic3d_Buffer::Init` where the generator picks the wrong signature, and some cases like `V3d_AmbientLight` that I need to examine more closely.
> 
> I think the quickest fix is to add these problematic methods to `exclude_methods` in bindings.toml, but I should also check if there's a generator-level solution for some of these issues. Let me look at the OCCT headers to understand what's actually happening with these specific cases.

Research OCCT headers for errors

Searched for text `SetLocalTransformation` (`**/crates/occt-sys/OCCT/src/AIS/AIS_Manipulator.hxx`), 3 results
- [AIS_Manipulator.hxx:212](../crates/occt-sys/OCCT/src/AIS/AIS_Manipulator.hxx#L212)
- [AIS_Manipulator.hxx:453](../crates/occt-sys/OCCT/src/AIS/AIS_Manipulator.hxx#L453)
- [AIS_Manipulator.hxx:455](../crates/occt-sys/OCCT/src/AIS/AIS_Manipulator.hxx#L455)

Searched for text `DisplayMode` (`**/crates/occt-sys/OCCT/src/AIS/AIS_PointCloud.hxx`), 1 result
- [AIS_PointCloud.hxx:46](../crates/occt-sys/OCCT/src/AIS/AIS_PointCloud.hxx#L46)

Searched for regex `Cells|cells_couple|MeshVS_NodePair` (`**/crates/occt-sys/OCCT/src/BRepMesh/BRepMesh_GeomTool.hxx`), 3 results
- [BRepMesh_GeomTool.hxx:203](../crates/occt-sys/OCCT/src/BRepMesh/BRepMesh_GeomTool.hxx#L203)
- [BRepMesh_GeomTool.hxx:204](../crates/occt-sys/OCCT/src/BRepMesh/BRepMesh_GeomTool.hxx#L204)
- [BRepMesh_GeomTool.hxx:206](../crates/occt-sys/OCCT/src/BRepMesh/BRepMesh_GeomTool.hxx#L206)

<details>
<summary>Searched for text <code>Init</code> (<code>**/crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_Buffer.hxx</code>), 9 results</summary>

- [Graphic3d_Buffer.hxx:45](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_Buffer.hxx#L45)
- [Graphic3d_Buffer.hxx:100](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_Buffer.hxx#L100)
- [Graphic3d_Buffer.hxx:107](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_Buffer.hxx#L107)
- [Graphic3d_Buffer.hxx:113](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_Buffer.hxx#L113)
- [Graphic3d_Buffer.hxx:119](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_Buffer.hxx#L119)
- [Graphic3d_Buffer.hxx:266](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_Buffer.hxx#L266)
- [Graphic3d_Buffer.hxx:304](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_Buffer.hxx#L304)
- [Graphic3d_Buffer.hxx:306](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_Buffer.hxx#L306)
- [Graphic3d_Buffer.hxx:336](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_Buffer.hxx#L336)
</details>

Searched for text `Init` (`**/crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_MutableIndexBuffer.hxx`), no results

Searched for regex `private|protected|public` (`**/crates/occt-sys/OCCT/src/V3d/V3d_AmbientLight.hxx`), 4 results
- [V3d_AmbientLight.hxx:9](../crates/occt-sys/OCCT/src/V3d/V3d_AmbientLight.hxx#L9)
- [V3d_AmbientLight.hxx:23](../crates/occt-sys/OCCT/src/V3d/V3d_AmbientLight.hxx#L23)
- [V3d_AmbientLight.hxx:26](../crates/occt-sys/OCCT/src/V3d/V3d_AmbientLight.hxx#L26)
- [V3d_AmbientLight.hxx:32](../crates/occt-sys/OCCT/src/V3d/V3d_AmbientLight.hxx#L32)

<details>
<summary>Searched for regex <code>private|protected|public</code> (<code>**/crates/occt-sys/OCCT/src/V3d/V3d_PositionLight.hxx</code>), 5 results</summary>

- [V3d_PositionLight.hxx:9](../crates/occt-sys/OCCT/src/V3d/V3d_PositionLight.hxx#L9)
- [V3d_PositionLight.hxx:23](../crates/occt-sys/OCCT/src/V3d/V3d_PositionLight.hxx#L23)
- [V3d_PositionLight.hxx:26](../crates/occt-sys/OCCT/src/V3d/V3d_PositionLight.hxx#L26)
- [V3d_PositionLight.hxx:27](../crates/occt-sys/OCCT/src/V3d/V3d_PositionLight.hxx#L27)
- [V3d_PositionLight.hxx:31](../crates/occt-sys/OCCT/src/V3d/V3d_PositionLight.hxx#L31)
</details>

<details>
<summary>Reading lines 445-465 of <a href="../crates/occt-sys/OCCT/src/AIS/AIS_Manipulator.hxx#L445-L466">AIS_Manipulator.hxx</a></summary>

```
  Standard_EXPORT void setTransformPersistence(const Handle(Graphic3d_TransformPers)& theTrsfPers);

  //! Redefines local transformation management method to inform user of improper use.
  //! @warning this interactive object does not support setting custom local transformation,
  //! this class solely uses this property to implement visual positioning of the manipulator
  //! without need for recomputing presentation.
  //! @warning Invokes debug assertion in debug to catch incompatible usage of the
  //! method, silently does nothing in release mode.
  Standard_EXPORT virtual void setLocalTransformation(const Handle(TopLoc_Datum3D)& theTrsf)
    Standard_OVERRIDE;
  using AIS_InteractiveObject::SetLocalTransformation; // hide visibility

protected: //! @name Auxiliary classes to fill presentation with proper primitives
  class Quadric
  {
  public:
    virtual ~Quadric()
    {
      myTriangulation.Nullify();
      myArray.Nullify();
    }
```
</details>

<details>
<summary>Reading lines 40-70 of <a href="../crates/occt-sys/OCCT/src/AIS/AIS_PointCloud.hxx#L40-L71">AIS_PointCloud.hxx</a></summary>

```
//! hilight mode, e.g. 100);
class AIS_PointCloud : public AIS_InteractiveObject
{
  DEFINE_STANDARD_RTTIEXT(AIS_PointCloud, AIS_InteractiveObject)
public:
  //! Display modes supported by this Point Cloud object
  enum DisplayMode
  {
    DM_Points = 0, //!< display as normal points, default presentation
    DM_BndBox = 2  //!< display as bounding box,  default for highlighting
  };

  //! Selection modes supported by this Point Cloud object
  enum SelectionMode
  {
    SM_Points         = 0, //!< detected by points
    SM_SubsetOfPoints = 1, //!< detect point(s) within Point Cloud rather than object as whole
    SM_BndBox         = 2, //!< detected by bounding box
  };

public:
  //! Constructor.
  Standard_EXPORT AIS_PointCloud();

  //! Sets the points from array of points.
  //! Method will not copy the input data - array will be stored as handle.
  //! @param[in] thePoints  the array of points
  Standard_EXPORT virtual void SetPoints(const Handle(Graphic3d_ArrayOfPoints)& thePoints);

  //! Sets the points with optional colors.
  //! The input data will be copied into internal buffer.
```
</details>

<details>
<summary>Reading lines 195-215 of <a href="../crates/occt-sys/OCCT/src/BRepMesh/BRepMesh_GeomTool.hxx#L195-L216">BRepMesh_GeomTool.hxx</a></summary>

```
  }

  // For better meshing performance we try to estimate the acceleration circles grid structure
  // sizes: For each parametric direction (U, V) we estimate firstly an approximate distance between
  // the future points - this estimation takes into account the required face deflection and the
  // complexity of the face. Particularly, the complexity of the faces based on BSpline curves and
  // surfaces requires much more points. At the same time, for planar faces and linear parts of the
  // arbitrary surfaces usually no intermediate points are necessary. The general idea for each
  // parametric direction: cells_count = 2 ^ log10 ( estimated_points_count ) For linear parametric
  // direction we fall back to the initial vertex count: cells_count = 2 ^ log10 (
  // initial_vertex_count )
  Standard_EXPORT static std::pair<Standard_Integer, Standard_Integer> CellsCount(
    const Handle(Adaptor3d_Surface)&     theSurface,
    const Standard_Integer               theVerticesNb,
    const Standard_Real                  theDeflection,
    const BRepMesh_DefaultRangeSplitter* theRangeSplitter);

private:
  //! Classifies the point in case of coincidence of two vectors.
  //! @param thePoint1 the start point of a segment (base point).
  //! @param thePoint2 the end point of a segment.
```
</details>

<details>
<summary>Reading lines 260-310 of <a href="../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_Buffer.hxx#L260-L311">Graphic3d_Buffer.hxx</a></summary>

```
    Stride       = 0;
    NbElements   = 0;
    NbAttributes = 0;
  }

  //! Allocates new empty array
  bool Init(const Standard_Integer     theNbElems,
            const Graphic3d_Attribute* theAttribs,
            const Standard_Integer     theNbAttribs)
  {
    release();
    Standard_Integer aStride = 0;
    for (Standard_Integer anAttribIter = 0; anAttribIter < theNbAttribs; ++anAttribIter)
    {
      const Graphic3d_Attribute& anAttrib = theAttribs[anAttribIter];
      aStride += anAttrib.Stride();
    }
    if (aStride == 0)
    {
      return false;
    }

    Stride       = aStride;
    NbElements   = theNbElems;
    NbAttributes = theNbAttribs;
    if (NbElements != 0)
    {
      const size_t aDataSize = size_t(Stride) * size_t(NbElements);
      if (!Allocate(aDataSize + sizeof(Graphic3d_Attribute) * NbAttributes))
      {
        release();
        return false;
      }

      mySize = aDataSize;
      for (Standard_Integer anAttribIter = 0; anAttribIter < theNbAttribs; ++anAttribIter)
      {
        ChangeAttribute(anAttribIter) = theAttribs[anAttribIter];
      }
    }
    return true;
  }

  //! Allocates new empty array
  bool Init(const Standard_Integer theNbElems, const Graphic3d_Array1OfAttribute& theAttribs)
  {
    return Init(theNbElems, &theAttribs.First(), theAttribs.Size());
  }

public:
  //! Flag indicating that attributes in the buffer are interleaved; TRUE by default.
```
</details>

<details>
<summary>Reading all lines of <a href="../crates/occt-sys/OCCT/src/V3d/V3d_AmbientLight.hxx">V3d_AmbientLight.hxx</a></summary>

```
// Created on: 1992-01-21
// Created by: GG
// Copyright (c) 1992-1999 Matra Datavision
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

#ifndef _V3d_AmbientLight_HeaderFile
#define _V3d_AmbientLight_HeaderFile

#include <V3d_Light.hxx>

//! Creation of an ambient light source in a viewer.
class V3d_AmbientLight : public Graphic3d_CLight
{
  DEFINE_STANDARD_RTTIEXT(V3d_AmbientLight, Graphic3d_CLight)
public:
  //! Constructs an ambient light source in the viewer.
  //! The default Color of this light source is WHITE.
  Standard_EXPORT V3d_AmbientLight(const Quantity_Color& theColor = Quantity_NOC_WHITE);

  //! @name hidden properties not applicable to ambient light
private:
  using Graphic3d_CLight::Angle;
  using Graphic3d_CLight::Attenuation;
  using Graphic3d_CLight::Concentration;
  using Graphic3d_CLight::ConstAttenuation;
  using Graphic3d_CLight::Direction;
  using Graphic3d_CLight::Headlight;
  using Graphic3d_CLight::IsHeadlight;
  using Graphic3d_CLight::LinearAttenuation;
  using Graphic3d_CLight::Position;
  using Graphic3d_CLight::SetAngle;
  using Graphic3d_CLight::SetAttenuation;
  using Graphic3d_CLight::SetConcentration;
  using Graphic3d_CLight::SetDirection;
  using Graphic3d_CLight::SetHeadlight;
  using Graphic3d_CLight::SetPosition;
  using Graphic3d_CLight::SetSmoothAngle;
  using Graphic3d_CLight::SetSmoothRadius;
  using Graphic3d_CLight::Smoothness;
};

DEFINE_STANDARD_HANDLE(V3d_AmbientLight, Graphic3d_CLight)

#endif // _V3d_AmbientLight_HeaderFile

```
</details>

<details>
<summary>Reading all lines of <a href="../crates/occt-sys/OCCT/src/V3d/V3d_PositionLight.hxx">V3d_PositionLight.hxx</a></summary>

```
// Created on: 1997-11-21
// Created by: ZOV
// Copyright (c) 1997-1999 Matra Datavision
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

#ifndef _V3d_PositionLight_HeaderFile
#define _V3d_PositionLight_HeaderFile

#include <V3d_Light.hxx>

//! Base class for Positional, Spot and Directional Light classes.
class V3d_PositionLight : public Graphic3d_CLight
{
  DEFINE_STANDARD_RTTIEXT(V3d_PositionLight, Graphic3d_CLight)
protected:
  //! Protected constructor.
  Standard_EXPORT V3d_PositionLight(Graphic3d_TypeOfLightSource theType);

  //! @name hidden properties not applicable to positional light
protected:
  using Graphic3d_CLight::Position;
  using Graphic3d_CLight::SetPosition;
};

DEFINE_STANDARD_HANDLE(V3d_PositionLight, Graphic3d_CLight)

#endif // _V3d_PositionLight_HeaderFile

```
</details>

<details>
<summary>Reading all lines of <a href="../crates/occt-sys/OCCT/src/V3d/V3d_SpotLight.hxx">V3d_SpotLight.hxx</a></summary>

```
// Created on: 1992-01-22
// Created by: GG
// Copyright (c) 1992-1999 Matra Datavision
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

#ifndef _V3d_SpotLight_HeaderFile
#define _V3d_SpotLight_HeaderFile

#include <V3d_PositionLight.hxx>
#include <V3d_TypeOfOrientation.hxx>

//! Creation and modification of a spot.
//! The attenuation factor F determines the illumination of a surface:
//! @code
//!   F = 1/(ConstAttenuation() + LinearAttenuation() * Distance)
//! @endcode
//! Where Distance is the distance from the source to the surface.
//! The default values (1.0, 0.0) correspond to a minimum of attenuation.
//! The concentration factor determines the dispersion of the light on the surface, the default
//! value (1.0) corresponds to a minimum of dispersion.
class V3d_SpotLight : public V3d_PositionLight
{
  DEFINE_STANDARD_RTTIEXT(V3d_SpotLight, V3d_PositionLight)
public:
  //! Creates a light source of the Spot type in the viewer with default attenuation factors (1.0,
  //! 0.0), concentration factor 1.0 and spot angle 30 degrees.
  Standard_EXPORT V3d_SpotLight(const gp_Pnt&               thePos,
                                const V3d_TypeOfOrientation theDirection = V3d_XnegYnegZpos,
                                const Quantity_Color&       theColor     = Quantity_NOC_WHITE);

  //! Creates a light source of the Spot type in the viewer with default attenuation factors (1.0,
  //! 0.0), concentration factor 1.0 and spot angle 30 degrees.
  Standard_EXPORT V3d_SpotLight(const gp_Pnt&         thePos,
                                const gp_Dir&         theDirection,
                                const Quantity_Color& theColor = Quantity_NOC_WHITE);

  //! Defines the direction of the light source
  //! according to a predefined directional vector.
  Standard_EXPORT void SetDirection(V3d_TypeOfOrientation theOrientation);
  using Graphic3d_CLight::Position;
  using Graphic3d_CLight::SetDirection;
  using Graphic3d_CLight::SetPosition;
};

DEFINE_STANDARD_HANDLE(V3d_SpotLight, V3d_PositionLight)

#endif // _V3d_SpotLight_HeaderFile

```
</details>

Searched for files matching `**/Graphic3d_MutableIndexBuffer.hxx`, 1 match
- [Graphic3d_MutableIndexBuffer.hxx](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_MutableIndexBuffer.hxx)

Searched for regex `class V3d_Light|typedef|using` (`**/crates/occt-sys/OCCT/src/V3d/V3d_Light.hxx`), 3 results
- [V3d_Light.hxx:22](../crates/occt-sys/OCCT/src/V3d/V3d_Light.hxx#L22)
- [V3d_Light.hxx:24](../crates/occt-sys/OCCT/src/V3d/V3d_Light.hxx#L24)
- [V3d_Light.hxx:26](../crates/occt-sys/OCCT/src/V3d/V3d_Light.hxx#L26)

<details>
<summary>Searched for regex <code>Angle|Attenuation|Concentration|Direction|Headlight|Position|SetAngle|SetAttenuation|SetConcentration|SetDirection|SetHeadlight|SetPosition|SetSmoothAngle|SetSmoothRadius|Smoothness</code> (<code>**/crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx</code>), 50 results</summary>

- [Graphic3d_CLight.hxx:70](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L70)
- [Graphic3d_CLight.hxx:71](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L71)
- [Graphic3d_CLight.hxx:73](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L73)
- [Graphic3d_CLight.hxx:75](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L75)
- [Graphic3d_CLight.hxx:76](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L76)
- [Graphic3d_CLight.hxx:78](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L78)
- [Graphic3d_CLight.hxx:79](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L79)
- [Graphic3d_CLight.hxx:81](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L81)
- [Graphic3d_CLight.hxx:83](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L83)
- [Graphic3d_CLight.hxx:84](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L84)
- [Graphic3d_CLight.hxx:86](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L86)
- [Graphic3d_CLight.hxx:87](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L87)
- [Graphic3d_CLight.hxx:89](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L89)
- [Graphic3d_CLight.hxx:90](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L90)
- [Graphic3d_CLight.hxx:92](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L92)
- [Graphic3d_CLight.hxx:93](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L93)
- [Graphic3d_CLight.hxx:94](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L94)
- [Graphic3d_CLight.hxx:97](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L97)
- [Graphic3d_CLight.hxx:98](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L98)
- [Graphic3d_CLight.hxx:100](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L100)
- [Graphic3d_CLight.hxx:103](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L103)
- [Graphic3d_CLight.hxx:104](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L104)
- [Graphic3d_CLight.hxx:105](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L105)
- [Graphic3d_CLight.hxx:107](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L107)
- [Graphic3d_CLight.hxx:108](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L108)
- [Graphic3d_CLight.hxx:110](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L110)
- [Graphic3d_CLight.hxx:112](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L112)
- [Graphic3d_CLight.hxx:113](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L113)
- [Graphic3d_CLight.hxx:114](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L114)
- [Graphic3d_CLight.hxx:116](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L116)
- [Graphic3d_CLight.hxx:117](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L117)
- [Graphic3d_CLight.hxx:119](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L119)
- [Graphic3d_CLight.hxx:121](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L121)
- [Graphic3d_CLight.hxx:122](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L122)
</details>

Searched for text `Init` (`**/crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_IndexBuffer.hxx`), 3 results
- [Graphic3d_IndexBuffer.hxx:32](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_IndexBuffer.hxx#L32)
- [Graphic3d_IndexBuffer.hxx:52](../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_IndexBuffer.hxx#L52)

<details>
<summary>Reading all lines of <a href="../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_MutableIndexBuffer.hxx">Graphic3d_MutableIndexBuffer.hxx</a></summary>

```
// Copyright (c) 2018 OPEN CASCADE SAS
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

#ifndef _Graphic3d_MutableIndexBuffer_HeaderFile
#define _Graphic3d_MutableIndexBuffer_HeaderFile

#include <Graphic3d_IndexBuffer.hxx>

//! Mutable index buffer.
class Graphic3d_MutableIndexBuffer : public Graphic3d_IndexBuffer
{
  DEFINE_STANDARD_RTTIEXT(Graphic3d_MutableIndexBuffer, Graphic3d_IndexBuffer)
public:
  //! Empty constructor.
  Graphic3d_MutableIndexBuffer(const Handle(NCollection_BaseAllocator)& theAlloc)
      : Graphic3d_IndexBuffer(theAlloc)
  {
  }

  //! Return TRUE if data can be invalidated.
  virtual Standard_Boolean IsMutable() const Standard_OVERRIDE { return Standard_True; }

  //! Return invalidated range.
  virtual Graphic3d_BufferRange InvalidatedRange() const Standard_OVERRIDE
  {
    return myInvalidatedRange;
  }

  //! Reset invalidated range.
  virtual void Validate() Standard_OVERRIDE { myInvalidatedRange.Clear(); }

  //! Invalidate the entire buffer data.
  virtual void Invalidate() Standard_OVERRIDE
  {
    invalidate(Graphic3d_BufferRange(0, (Standard_Integer)mySize));
  }

  //! Invalidate the given indexes (starting from 0)
  void Invalidate(Standard_Integer theIndexLower, Standard_Integer theIndexUpper)
  {
    Standard_OutOfRange_Raise_if(theIndexLower > theIndexUpper,
                                 "Graphic3d_MutableIndexBuffer::Invalidate()");
    invalidate(
      Graphic3d_BufferRange(Stride * theIndexLower, Stride * (theIndexUpper - theIndexLower + 1)));
  }

  //! Invalidate specified sub-range of data (as byte offsets).
  void invalidate(const Graphic3d_BufferRange& theRange) { myInvalidatedRange.Unite(theRange); }

protected:
  Graphic3d_BufferRange myInvalidatedRange; //!< invalidated buffer data range (as byte offsets)
};

#endif // _Graphic3d_MutableIndexBuffer_HeaderFile

```
</details>

<details>
<summary>Reading all lines of <a href="../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_IndexBuffer.hxx">Graphic3d_IndexBuffer.hxx</a></summary>

```
// Copyright (c) 2014 OPEN CASCADE SAS
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

#ifndef _Graphic3d_IndexBuffer_HeaderFile
#define _Graphic3d_IndexBuffer_HeaderFile

#include <Graphic3d_Buffer.hxx>

//! Index buffer.
class Graphic3d_IndexBuffer : public Graphic3d_Buffer
{
  DEFINE_STANDARD_RTTIEXT(Graphic3d_IndexBuffer, Graphic3d_Buffer)
public:
  //! Empty constructor.
  Graphic3d_IndexBuffer(const Handle(NCollection_BaseAllocator)& theAlloc)
      : Graphic3d_Buffer(theAlloc)
  {
  }

  //! Allocates new empty index array
  template <typename IndexType_t>
  bool Init(const Standard_Integer theNbElems)
  {
    release();
    Stride = sizeof(IndexType_t);
    if (Stride != sizeof(unsigned short) && Stride != sizeof(unsigned int))
    {
      return false;
    }

    NbElements   = theNbElems;
    NbAttributes = 0;
    if (NbElements != 0 && !Allocate(size_t(Stride) * size_t(NbElements)))
    {
      release();
      return false;
    }
    return true;
  }

  //! Allocates new empty index array
  bool InitInt32(const Standard_Integer theNbElems) { return Init<int>(theNbElems); }

  //! Access index at specified position
  Standard_Integer Index(const Standard_Integer theIndex) const
  {
    return Stride == sizeof(unsigned short) ? Standard_Integer(Value<unsigned short>(theIndex))
                                            : Standard_Integer(Value<unsigned int>(theIndex));
  }

  //! Change index at specified position
  void SetIndex(const Standard_Integer theIndex, const Standard_Integer theValue)
  {
    if (Stride == sizeof(unsigned short))
    {
      ChangeValue<unsigned short>(theIndex) = (unsigned short)theValue;
    }
    else
    {
      ChangeValue<unsigned int>(theIndex) = (unsigned int)theValue;
    }
  }

  //! Dumps the content of me into the stream
  virtual void DumpJson(Standard_OStream& theOStream,
                        Standard_Integer  theDepth = -1) const Standard_OVERRIDE
  {
    OCCT_DUMP_TRANSIENT_CLASS_BEGIN(theOStream)
    OCCT_DUMP_BASE_CLASS(theOStream, theDepth, Graphic3d_Buffer)
  }
};

DEFINE_STANDARD_HANDLE(Graphic3d_IndexBuffer, Graphic3d_Buffer)

#endif // _Graphic3d_IndexBuffer_HeaderFile

```
</details>

<details>
<summary>Reading lines 60-200 of <a href="../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L60-L201">Graphic3d_CLight.hxx</a></summary>

```
  //! instead it turns it OFF so that it just have no effect.
  Standard_EXPORT void SetEnabled(Standard_Boolean theIsOn);

  //! Return TRUE if shadow casting is enabled; FALSE by default.
  //! Has no effect in Ray-Tracing rendering mode.
  Standard_Boolean ToCastShadows() const { return myToCastShadows; }

  //! Enable/disable shadow casting.
  Standard_EXPORT void SetCastShadows(Standard_Boolean theToCast);

  //! Returns true if the light is a headlight; FALSE by default.
  //! Headlight flag means that light position/direction are defined not in a World coordinate
  //! system, but relative to the camera orientation.
  Standard_Boolean IsHeadlight() const { return myIsHeadlight; }

  //! Alias for IsHeadlight().
  Standard_Boolean Headlight() const { return myIsHeadlight; }

  //! Setup headlight flag.
  Standard_EXPORT void SetHeadlight(Standard_Boolean theValue);

  //! @name positional/spot light properties
public:
  //! Returns location of positional/spot light; (0, 0, 0) by default.
  const gp_Pnt& Position() const { return myPosition; }

  //! Setup location of positional/spot light.
  Standard_EXPORT void SetPosition(const gp_Pnt& thePosition);

  //! Returns location of positional/spot light.
  void Position(Standard_Real& theX, Standard_Real& theY, Standard_Real& theZ) const
  {
    theX = myPosition.X();
    theY = myPosition.Y();
    theZ = myPosition.Z();
  }

  //! Setup location of positional/spot light.
  void SetPosition(Standard_Real theX, Standard_Real theY, Standard_Real theZ)
  {
    SetPosition(gp_Pnt(theX, theY, theZ));
  }

  //! Returns constant attenuation factor of positional/spot light source; 1.0f by default.
  //! Distance attenuation factors of reducing positional/spot light intensity depending on the
  //! distance from its position:
  //! @code
  //!   float anAttenuation = 1.0 / (ConstAttenuation() + LinearAttenuation() * theDistance +
  //!   QuadraticAttenuation() * theDistance * theDistance);
  //! @endcode
  Standard_ShortReal ConstAttenuation() const { return myParams.x(); }

  //! Returns linear attenuation factor of positional/spot light source; 0.0 by default.
  //! Distance attenuation factors of reducing positional/spot light intensity depending on the
  //! distance from its position:
  //! @code
  //!   float anAttenuation = 1.0 / (ConstAttenuation() + LinearAttenuation() * theDistance +
  //!   QuadraticAttenuation() * theDistance * theDistance);
  //! @endcode
  Standard_ShortReal LinearAttenuation() const { return myParams.y(); }

  //! Returns the attenuation factors.
  void Attenuation(Standard_Real& theConstAttenuation, Standard_Real& theLinearAttenuation) const
  {
    theConstAttenuation  = ConstAttenuation();
    theLinearAttenuation = LinearAttenuation();
  }

  //! Defines the coefficients of attenuation; values should be >= 0.0 and their summ should not be
  //! equal to 0.
  Standard_EXPORT void SetAttenuation(Standard_ShortReal theConstAttenuation,
                                      Standard_ShortReal theLinearAttenuation);

  //! @name directional/spot light additional properties
public:
  //! Returns direction of directional/spot light.
  gp_Dir Direction() const { return gp_Dir(myDirection.x(), myDirection.y(), myDirection.z()); }

  //! Sets direction of directional/spot light.
  Standard_EXPORT void SetDirection(const gp_Dir& theDir);

  //! Returns the theVx, theVy, theVz direction of the light source.
  void Direction(Standard_Real& theVx, Standard_Real& theVy, Standard_Real& theVz) const
  {
    theVx = myDirection.x();
    theVy = myDirection.y();
    theVz = myDirection.z();
  }

  //! Sets direction of directional/spot light.
  void SetDirection(Standard_Real theVx, Standard_Real theVy, Standard_Real theVz)
  {
    SetDirection(gp_Dir(theVx, theVy, theVz));
  }

  //! Returns location of positional/spot/directional light, which is the same as returned by
  //! Position().
  const gp_Pnt& DisplayPosition() const { return myPosition; }

  //! Setup lo
... (truncated, 6100 chars)
```
</details>

<details>
<summary>Reading lines 200-210 of <a href="../crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CLight.hxx#L200-L211">Graphic3d_CLight.hxx</a></summary>

```

  //! Modifies the smoothing angle (in radians) of directional light source; should be within range
  //! [0.0, M_PI/2].
  Standard_EXPORT void SetSmoothAngle(Standard_ShortReal theValue);

  //! Returns TRUE if maximum distance of point light source is defined.
  bool HasRange() const { return myDirection.w() != 0.0f; }

  //! Returns maximum distance on which point light source affects to objects and is considered
  //! during illumination calculations. 0.0 means disabling range considering at all without any
  //! distance limits. Has sense only for point light sources (positional and spot).
```
</details>

> 💭 Now I have a clear picture of all the issues. The generator's inherited method logic doesn't handle `using` declarations that change access levels. Let me figure out the best approach:
> 
> 1. **V3d_AmbientLight** - Uses `using Graphic3d_CLight::Method` in `private:` to hide inherited methods → The generator's `protected_in_ancestors` logic already handles re-declared methods IF the `all_method_names` set captures them. But `using` declarations are not regular method declarations — they're `EntityKind::UsingDeclaration` in clang. The parser may not be detecting these.
> 
> 
> 2. **AIS_Manipulator** - Same pattern with `using AIS_InteractiveObject::SetLocalTransformation` in the protected section.
> 3. **AIS_PointCloud** - An enum named `DisplayMode` shadows an inherited method with the same name, which the generator doesn't account for.
> 4. **Graphic3d_IndexBuffer/Graphic3d_MutableIndexBuffer** - Template specialization `Init<T>` hides parent `Init` overloads due to C++ name hiding rules.
> 5. **BRepMesh_GeomTool::CellsCount** - Returns a pair type that gets resolved to `MeshVS_Node`.
> 
> The core issue is that `using` declarations in private/protected sections aren't being tracked in `all_method_names`, so the generator can't detect when access is being narrowed. I could fix this properly by improving the generator's handling of access-narrowing `using` declarations, but that's involved. Instead, I'll take a two-pronged approach: enhance the generator to catch the `using` declaration cases, then add targeted exclusions for the remaining edge cases like the enum shadowing and template hiding. Looking at the specific errors, I see a pattern: `using` declarations in base classes are exposing methods that shouldn't be visible, plus some enum shadowing and template hiding issues. Rather than adding dozens of exclusions, I should fix the generator to properly handle `using` declarations by checking if they're in private or protected sections—that would automatically resolve the V3d_AmbientLight and AIS_Manipulator cases and prevent similar issues going forward. For the enum and template cases, I'll add targeted exclusions since those are more isolated problems. So the main strategy is to fix the `using` declaration handling in the parser first—that should resolve the bulk of the errors across V3d_AmbientLight, AIS_Manipulator, and V3d_PositionLight—then add a few targeted exclusions for the remaining edge cases like the MeshVS_NodePair typedef and a couple of Init methods.

<details>
<summary>Analyze parser using-declaration handling</summary>

Here's the content from lines 400–800 and 900–950 of [parser.rs](crates/opencascade-binding-generator/src/parser.rs). Summary of findings:

### 1. Main loop processing children of a class definition
Lines [500–640](crates/opencascade-binding-generator/src/parser.rs#L500-L640): `entity.visit_children(|child, _| { ... })` iterates over all children. It first checks for `EntityKind::AccessSpecifier` to track public/protected/private sections, then uses a `match child.get_kind()` dispatching on:
- `Constructor`
- `Method`
- `FieldDecl`
- `ClassDecl | StructDecl` (nested classes)
- `Destructor`
- `_ => {}` — everything else is **silently ignored**

### 2. Where `all_method_names` is populated
[Line 480](crates/opencascade-binding-generator/src/parser.rs#L480): initialized as `HashSet::new()`.
[Line 581](crates/opencascade-binding-generator/src/parser.rs#L581): populated inside the `EntityKind::Method` arm with `all_method_names.insert(method_name.clone())` — this happens for **all** methods regardless of visibility, used for filtering inherited methods.

### 3. `EntityKind::UsingDeclaration` handling
**Not handled anywhere.** The `_ => {}` wildcard at [line 637](crates/opencascade-binding-generator/src/parser.rs#L637) silently discards it. This means `using Base::MethodName;` declarations in derived classes (which make a base class method public) are completely ignored. The method name won't be added to `all_method_names`, which could cause issues with inherited method filtering.

### 4. `check_protected_destructor` function
[Lines 700–710](crates/opencascade-binding-generator/src/parser.rs#L700-L710): Iterates over `entity.get_children()`, looks for `EntityKind::Destructor`, and returns `true` if the destructor's accessibility is not `Public`. Returns `false` if no destructor is found (implicit destructor = public).

### 5. How `current_access` Cell is used
[Line 492–497](crates/opencascade-binding-generator/src/parser.rs#L492-L497): Initialized based on whether it's a `StructDecl` (default public) or class (default private).
[Line 504](crates/opencascade-binding-generator/src/parser.rs#L504): Updated when `EntityKind::AccessSpecifier` children are encountered.
[Line 615](crates/opencascade-binding-generator/src/parser.rs#L615): Read in `EntityKind::ClassDecl | StructDecl` arm to determine if a nested class is in a public section: `current_access.get() == Accessibility::Public`. It's a `Cell` because it needs interior mutability inside the `visit_children` closure.

</details>

<details>
<summary>Reading lines 400-800 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L400-L801">parser.rs</a></summary>

```
fn parse_class(entity: &Entity, source_header: &str, verbose: bool) -> Vec<ParsedClass> {
    let name = match entity.get_name() {
        Some(n) => n,
        None => return Vec::new(),
    };

    // Skip forward declarations (no definition)
    if !entity.is_definition() {
        return Vec::new();
    }

    // Skip anonymous classes/structs
    if name.is_empty() {
        return Vec::new();
    }

    // Skip internal/private classes (those starting with underscore)
    if name.starts_with('_') {
        return Vec::new();
    }

    // Skip template classes and template specializations
    // Template classes have get_template() returning Some, or get_template_kind() returning Some
    // Also skip if the display name contains angle brackets (indicates template instantiation)
    if entity.get_template().is_some() {
        if verbose {
            println!("    Skipping {} (template class)", name);
        }
        return Vec::new();
    }
    let display_name = entity.get_display_name().unwrap_or_default();
    if display_name.contains('<') {
        if verbose {
            println!("    Skipping {} (template specialization)", display_name);
        }
        return Vec::new();
    }

    // Skip policy/trait classes used as template parameters
    // These are not meant to be instantiated directly
    if name.contains("Inspector") || name.contains("_Hasher") || name.contains("_Traits") {
        if verbose {
            println!("    Skipping {} (policy/trait class)", name);
        }
        return Vec::new();
    }

    // Skip internal node types that use custom allocators (can't be used with std::unique_ptr)
    if name.ends_with("Node") && name.starts_with("NCollection_") {
        if verbose {
            println!("    Skipping {} (internal node type)", name);
        }
        return Vec::new();
    }

    let comment = extract_doxygen_comment(entity);
    let module = extract_module_from_header(source_header);

    // Extract direct base classes for upcast generation
    let base_classes = extract_base_classes(entity);
    
    // Check for protected/private destructor (indicates non-instantiable abstract base class)
    let has_protected_destructor = check_protected_destructor(entity);

    if verbose {
        println!("  Parsing class: {}", name);
        if !base_classes.is_empty() {
            println!("    Base classes: {:?}", base_classes);
        }
        if has_protected_destructor {
            println!("    Has protected destructor (non-instantiable)");
        }
    }

    let mut constructors = Vec::new();
    let mut methods = Vec::new();
    let mut static_methods = Vec::new();
    let mut fields: Vec<ParsedField> = Vec::new();
    let mut has_non_public_fields = false;
    let mut has_virtual_methods = false;
    let mut all_method_names = std::collections::HashSet::new();
    let mut is_abstract = false;
    let mut pure_virtual_methods = std::collections::HashSet::new();
    let mut has_explicit_constructors = false;
    // Track copy constructor: None = no explicit copy ctor seen,
    // Some(true) = public non-deleted copy ctor, Some(false) = deleted/non-public copy ctor
    let mut has_copy_constructor: Option<bool> = None;
    let mut has_move_constructor = false;
    let mut nested_classes: Vec<ParsedClass> = Vec::new();

    // Track current access level for nested type visibility.
    // Default: `class` => private, `struct` => public.
    let default_access = if entity.get_kind() == EntityKind::StructDecl {
        Accessibility::Public
    } else {
        Accessibility::Private
    };
    let current_access = std::cell::Cell::new(default_access);


    entity.visit_children(|child, _| {
        // Track access specifiers (public:/protected:/private: sections)
        if child.get_kind() == EntityKind::AccessSpecifier {
            if let Some(acc) = child.get_accessibility() {
                current_access.set(acc);
            }
            return EntityVisitResult::Contin
... (truncated, 15451 chars)
```
</details>

<details>
<summary>Reading lines 900-950 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L900-L951">parser.rs</a></summary>

```
/// Extract module name from OCCT header filename (e.g., "gp_Pnt.hxx" -> "gp")
///
/// Module names are derived from the header file, not the class/type name.
/// This avoids mismatches for types like Fortran common blocks (e.g., `maovpar_1_`
/// in `AdvApp2Var_Data.hxx`) and helper classes that don't follow the standard
/// `Module_Class` naming convention.
fn extract_module_from_header(header: &str) -> String {
    // Strip .hxx extension first
    let name = header.strip_suffix(".hxx").unwrap_or(header);
    // OCCT naming convention: ModuleName_ClassName.hxx
    // Examples: gp_Pnt.hxx -> "gp", TopoDS_Shape.hxx -> "TopoDS"
    if let Some(underscore_pos) = name.find('_') {
        name[..underscore_pos].to_string()
    } else {
        // No underscore - single-word module (e.g., "gp.hxx" -> "gp")
        name.to_string()
    }
}

/// Check if a method/constructor is in the public section
fn is_public(entity: &Entity) -> bool {
    entity.get_accessibility() == Some(clang::Accessibility::Public)
}

/// Check if a method should be bound based on OCCT conventions
/// Binds: Standard_EXPORT methods OR public methods with doc comments
#[allow(dead_code)]
fn should_bind_method(entity: &Entity) -> bool {
    // Check for Standard_EXPORT by looking at the display name or attributes
    // Standard_EXPORT methods are always bindable
    if has_standard_export(entity) {
        return true;
    }

    // Public methods with documentation comments are also bindable
    // (these are typically inline getters/setters)
    if entity.get_comment().is_some() {
        return true;
    }

    // For now, bind all public methods - can be refined later
    true
}

/// Check if a method has Standard_EXPORT annotation
fn has_standard_export(entity: &Entity) -> bool {
    // Standard_EXPORT is a macro that expands to __declspec(dllexport) on Windows
    // or __attribute__((visibility("default"))) on other platforms.
    // We can detect it by checking if the method is not inline-only.

    // Methods that are defined inline (have a body in the header) but don't have
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01YZ2c2nPvXm3zL4MNwEDw7V__vscode-1771608612295/content.txt)

<details>
<summary>Reading lines 490-650 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L490-L651">parser.rs</a></summary>

```
    // Track current access level for nested type visibility.
    // Default: `class` => private, `struct` => public.
    let default_access = if entity.get_kind() == EntityKind::StructDecl {
        Accessibility::Public
    } else {
        Accessibility::Private
    };
    let current_access = std::cell::Cell::new(default_access);


    entity.visit_children(|child, _| {
        // Track access specifiers (public:/protected:/private: sections)
        if child.get_kind() == EntityKind::AccessSpecifier {
            if let Some(acc) = child.get_accessibility() {
                current_access.set(acc);
            }
            return EntityVisitResult::Continue;
        }

        match child.get_kind() {
            EntityKind::Constructor => {
                // Any explicit constructor means C++ won't generate an implicit default
                has_explicit_constructors = true;

                // Detect copy constructors via libclang
                if child.is_copy_constructor() {
                    let is_available = child.get_availability() != Availability::Unavailable;
                    let is_pub = is_public(&child);
                    // Also check that the copy ctor takes a const reference (const T&),
                    // not a mutable reference (T&). Our to_owned wrapper uses
                    // `const T*` so non-const copy ctors won't compile.
                    let takes_const_ref = child.get_arguments()
                        .and_then(|args| args.first().and_then(|arg| arg.get_type()))
                        .and_then(|ty| ty.get_pointee_type())
                        .map(|pointee| pointee.is_const_qualified())
                        .unwrap_or(true); // Default to true if we can't determine
                    if is_available && is_pub && takes_const_ref {
                        has_copy_constructor = Some(true);
                    } else if has_copy_constructor.is_none() {
                        // Deleted, non-public, or non-const copy constructor
                        has_copy_constructor = Some(false);
                    }
                    // Don't add copy constructors to the regular constructors list
                    return EntityVisitResult::Continue;
                }

                // Detect move constructors — these suppress implicit copy constructors
                if child.is_move_constructor() {
                    has_move_constructor = true;
                    // Don't add move constructors to the regular constructors list
                    return EntityVisitResult::Continue;
                }

                // Skip deprecated constructors
                if child.get_availability() == Availability::Deprecated {
                    if verbose {
                        println!("    Skipping deprecated constructor for {}", name);
                    }
                    return EntityVisitResult::Continue;
                }

                if is_public(&child) {
                    if let Some(ctor) = parse_constructor(&child, verbose) {
                        constructors.push(ctor);
                    }
                }
            }
            EntityKind::Method => {
                // Check for virtual methods (affects POD detection)
                if child.is_virtual_method() {
                    has_virtual_methods = true;
                }
                // Check if this is a pure virtual method (makes the class abstract)
                if child.is_pure_virtual_method() {
                    is_abstract = true;
                    if let Some(ref method_name) = child.get_name() {
                        pure_virtual_methods.insert(method_name.clone());
                    }
                }

                // Skip destructors, operators, and conversion functions
                if let Some(ref method_name) = child.get_name() {
                    if method_name.starts_with('~')
                        || method_name.starts_with("operator")

... (truncated, 7458 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01TeQnG7jBHQFFktvMV4rqC4__vscode-1771608612298/content.txt#1-1), lines 1 to 160

> 💭 Now I understand the problem clearly. When `using Base::MethodName;` appears in a non-public section of a derived class, it effectively hides that method. But the parser doesn't handle `EntityKind::UsingDeclaration` at all (it falls through to `_ => {}`), so the method name never gets added to `all_method_names`.
> 
> The inheritance logic in bindings.rs checks `all_method_names` to detect narrowed methods. If a method is in `all_method_names` but not in `included_methods()`, it's considered narrowed (re-declared as protected/private). But since `using` declarations don't add to `all_method_names`, the narrowing is not detected.
> 
> 
> I also need to handle cases where other declarations like enums or type aliases shadow inherited methods — for instance, when `AIS_PointCloud::DisplayMode` is a public enum that shadows an inherited method with the same name. So I need to extend the parser to recognize `UsingDeclaration`, `TypedefDecl`, `TypeAliasDecl`, and `EnumDecl` entity kinds and add their names to `all_method_names` when they would shadow inherited methods.
> 
> For `using` declarations, I can extract the method name directly using `child.get_name()` — so `using Graphic3d_CLight::Angle;` gives me "Angle". For enums and type aliases, I'd add their names the same way. The challenge is that I don't know which inherited methods exist at parse time, so the safest approach is to add all enum and type alias names to `all_method_names` to catch any potential shadowing.
> 
> When these names go into `all_method_names`, the inheritance logic will treat them as "claimed" by the child class, preventing those inherited methods from being inherited — which is correct behavior since the enum or type does shadow the method. The same applies to `using` declarations in non-public sections.
> 
> However, there's a subtlety with public `using` declarations like `V3d_SpotLight` re-exposing `Position`: adding the name to `all_method_names` would incorrectly signal that the child class has its own codegen-able method, when really it's just restoring visibility of an inherited one. I need to distinguish between `using` declarations that narrow access versus those that restore it. using declaration just makes the parent's method accessible.
> 
> Hmm, this is tricky. Let me think about this differently.
> 
> Actually, for `using` declarations:
> - In NON-public sections: The name should be added to `all_method_names` to mark it as narrowed
> - In PUBLIC sections: We need to check if the method was narrowed by an intermediate parent and un-narrow it. But the simpler approach is to NOT add it to `all_method_names` for public using — just let the inheritance logic handle it normally.
> 
> Wait, but if a public `using Base::Method;` is present, the method should be inherited from Base. If we don't add it to `all_method_names`, and the intermediate parent narrowed it, the inheritance logic should still properly detect that V3d_PositionLight narrowed `Position`, but V3d_SpotLight did NOT narrow it — so the inherited method would be blocked at V3d_PositionLight level but NOT restored at V3d_SpotLight level.
> 
> The inheritance logic walks ancestors from closest to most distant and once a method is added to `protected_in_ancestors`, it stays blocked. So even if V3d_SpotLight doesn't narrow "Position", it would still be blocked because V3d_PositionLight narrowed it. To support the "re-expose" pattern, I'd need to handle public using declarations specially — they should remove from the blocked set, but that's getting complex.
> 
> For now, I'll focus on what's needed to fix the build errors: handle non-public `using` declarations by adding them to `all_method_names`, and handle enum.
> 
> Now looking at the other errors: template methods like `Graphic3d_IndexBuffer::Init<T>` also hide parent methods and need to be tracked, and there's a type resolution issue with `BRepMesh_GeomTool` returning a typedef for `std::pair`. I need to check if the parser handles `FunctionTemplate` entity kinds.
> 
> Looking at the match arms in the parser, `FunctionTemplate` would fall through to the catch-all case, but in clang's AST, template methods are actually represented as `FunctionTemplate` rather than `Method`, so I need to add handling for that case.
> 
> For the typedef issue, `BRepMesh_GeomTool::CellsCount` returns `std::pair<int,int>`, but the type is being resolved through the `MeshVS_NodePair` typedef. The generator seems to be mishandling this type resolution.
> 
> Looking at the actual error, the generator resolved the return type to the typedef name `MeshVS_NodePair` instead of expanding it to the underlying `std::pair` type. This is a type resolution bug. For now, I'll exclude this method and the Init methods that have similar issues, then focus on the broader generator fixes that will handle most of the errors: handling `UsingDeclaration` in non-public sections and properly tracking `EnumDecl` names.
> 
> Before diving into implementation, I should check if there are many more errors beyond the first 20 reported. The errors I'm seeing break down into patterns—14 from V3d_AmbientLight's private using declarations, one from AIS_Manipulator's protected using declaration, one from AIS_PointCloud's enum shadowing, and a couple from Graphic3d's Init methods—so fixing the generator to handle these declaration types should resolve the bulk of them.
> 
> Fixing the parser to properly handle using declarations would eliminate around 15 errors automatically, and addressing the enum and template issues would clear another 3, leaving just the MeshVS_NodePair type issue and whatever else might surface. Let me start by updating the parser to handle these declaration patterns, then regenerate and rebuild to see what remains.

Now I understand the issues. Let me fix the binding generator to handle C++ `using` declarations that change method accessibility, enum names that shadow methods, and template methods that hide parent overloads. Then I'll handle edge cases via config exclusions.

**Todo list:**
- ✅ Remove viz module exclusions
- ✅ Regenerate bindings
- 🔄 Fix generator: using/enum/template
- ⬜ Add config exclusions for edge cases
- ⬜ Regenerate and build
- ⬜ Fix remaining build errors
- ⬜ Run tests
- ⬜ Commit changes

<details>
<summary>Reading lines 625-640 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L625-L641">parser.rs</a></summary>

```
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
            _ => {}
        }
        EntityVisitResult::Continue
    });
```
</details>

Now I need to add handling for `UsingDeclaration`, `EnumDecl`, and `FunctionTemplate` before the wildcard. These names should be added to `all_method_names` so the inheritance logic knows they shadow inherited methods.

<details>
<summary>**Failing: <a href="../crates/opencascade-binding-generator/src/parser.rs#L636-L666">parser.rs -1+30</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "636:uo\n637:pf",
      "content": "            }\n            EntityKind::UsingDeclaration => {\n                // `using Base::Method;` in a non-public section narrows the\n                // inherited method's access, hiding it from derived classes.\n                // Record the name so the inheritance logic treats it as overridden.\n                if !is_public(&child) {\n                    if let Some(ref using_name) = child.get_name() {\n                        all_method_names.insert(using_name.clone());\n                    }\n                }\n            }\n            EntityKind::EnumDecl => {\n                // A public enum with the same name as an inherited method\n                // shadows that method in C++ (e.g. AIS_PointCloud::DisplayMode\n                // enum hides PrsMgr_PresentableObject::DisplayMode() method).\n                if let Some(ref enum_name) = child.get_name() {\n                    if !enum_name.is_empty() {\n                        all_method_names.insert(enum_name.clone());\n                    }\n                }\n            }\n            EntityKind::FunctionTemplate => {\n                // Template methods hide base class methods with the same name\n                // (C++ name-hiding rule). Record the name so the inheritance\n                // logic blocks the parent's non-template overloads.\n                if let Some(ref tmpl_name) = child.get_name() {\n                    all_method_names.insert(tmpl_name.clone());\n                }\n            }\n            _ => {}"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/parser.rs","startLine":636,"endLine":666}
```
</details>

Now let me also handle the MeshVS_NodePair issue by adding exclusions to bindings.toml:

<details>
<summary>Reading lines 80-105 of <a href="../crates/opencascade-sys/bindings.toml#L80-L106">bindings.toml</a></summary>

```

# Exclude specific methods/constructors that clang completely misresolves in batch
# parsing mode — both canonical AND display names show the wrong type, so the
# parser's heuristic guards cannot detect it. These methods are excluded
# individually so the rest of each class remains accessible.
# Format: "ClassName::MethodName" for methods, "ClassName::ClassName" for constructors.
exclude_methods = [
    # C++ `New()` factory method name-collides with constructor wrapper `fn new()`
    "Storage_CallBack::New",
    "Storage_DefaultCallBack::New",

    # `const char**` (double pointer) mapped as `&mut &str` but FFI needs raw pointer
    "XmlObjMgt::GetInteger",
    "XmlObjMgt::GetReal",
]

# Exclude specific classes/types from destructor generation.
# Forward-declared-only nested types cannot be safely deleted.
exclude_classes = [
    "Message_ProgressScope::NullString",  # Forward-declared only, no full definition
]

[general]
resolve_deps = true

# Opaque types defined in manual/ files. These are not parsed from OCCT headers
```
</details>

<details>
<summary>**Failing: <a href="../crates/opencascade-sys/bindings.toml#L92-L99">bindings.toml -1+7</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    \"XmlObjMgt::GetInteger\",\n    \"XmlObjMgt::GetReal\",\n\n    # Returns MeshVS_NodePair (typedef for std::pair<int,int>) which the generator\n    # misresolves as an opaque class type.\n    \"BRepMesh_GeomTool::CellsCount\",\n]",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "92:vc\n93:vc\n94:cy"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":92,"endLine":100}
```
</details>

**Todo list:**
- ✅ Remove viz module exclusions
- ✅ Regenerate bindings
- ✅ Fix generator: using/enum/template
- ✅ Add config exclusions for edge cases
- 🔄 Regenerate and build
- ⬜ Fix remaining build errors
- ⬜ Run tests
- ⬜ Commit changes

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
grep -E "(complete|error)" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01MCfkocxAZDpiH2Ly5QiJBR__vscode-1771608612306/content.txt | grep -iv "^  Wrote"
```
```
  Formatting complete.
Code generation complete!
```

Good, regeneration succeeded. Now build:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```
<details>
<summary>Output (204 lines) — exit code 101</summary>

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
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:473976:44: error: unknown type name 'MeshVS_NodePair'
warning: opencascade-sys@0.2.0:  473976 | extern "C" void MeshVS_NodePair_destructor(MeshVS_NodePair* self_) { delete self_; }
warning: opencascade-sys@0.2.0:         |                                            ^
warning: o
... (truncated, 15237 chars total)
```
</details>

**Terminal:**
```sh
grep -c "error:" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01QoWsr8rrhHHFMSs5bC6SkL__vscode-1771608612308/content.txt; grep "error:" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01QoWsr8rrhHHFMSs5bC6SkL__vscode-1771608612308/content.txt
```
```
3
ade-sys/generated/wrappers.cpp:473976:44: error: unknown type name 'MeshVS_NodePair'                                                                            warning: opencascade-sys@0.2.0:  473976 | extern "C" void MeshVS_NodePair_destru
lly (status code exit status: 1): env -u IPHONEOS_DEPLOYMENT_TARGET LC_ALL="C" "sccache" "c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm64-apple-macosx" "-mmacosx-version-min=26.2" "-I" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include" "-I" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated" "-Wall" "-Wextra" "-std=c++14" "-Wno-unused-function" "-Wno-deprecated-declarations" "-Wno-return-type-c-linkage" "-Wno-delete-non-abstract-non-virtual-dtor" "-Wno-delete-abstract-non-virtual-dtor" "-D_USE_MATH_DEFINES=TRUE" "-o" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/74271c7d28a70e01-wrappers.o" "-c" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp"                                       error: failed to run custom build command for `opencascade-sys v0.2.0 (/Users/se
d/wrappers.cpp:473976:44: error: unknown type name 'MeshVS_NodePair'              cargo:warning= 473976 | extern "C" void MeshVS_NodePair_destructor(MeshVS_Node
```

> 💭 Down from 20 errors to just 1! The `MeshVS_NodePair` issue. It's a typedef for `std::pair<int,int>` and the generator is treating it as a class. I need to add it to `exclude_classes` in bindings.toml since it's generating a destructor for it.
> 
> Actually, the MeshVS_NodePair is being treated as a class by the generator. It's a typedef, not a real class. I should add it to exclude_classes to prevent destructor generation, or find a better approach.
> 
> Wait, actually - since MeshVS is now included, the generator is seeing `MeshVS_NodePair` as a type (from MeshVS_SymmetricPairHasher.hxx) and trying to generate bindings for it. 
> 
> Since it's just a typedef wrapping std::pair, I can't properly bind it anyway. Let me add it to exclude_classes to stop the generator from creating that destructor.

Down to just 1 error — still the `MeshVS_NodePair` typedef issue. Need to exclude it as a class since it's a typedef for `std::pair<int,int>`, not a real class.

<details>
<summary>Reading lines 101-108 of <a href="../crates/opencascade-sys/bindings.toml#L101-L109">bindings.toml</a></summary>

```
# Forward-declared-only nested types cannot be safely deleted.
exclude_classes = [
    "Message_ProgressScope::NullString",  # Forward-declared only, no full definition
]

[general]
resolve_deps = true

```
</details>

<details>
<summary>**Failing: <a href="../crates/opencascade-sys/bindings.toml#L102-L106">bindings.toml -1+4</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "exclude_classes = [\n    \"Message_ProgressScope::NullString\",  # Forward-declared only, no full definition\n    \"MeshVS_NodePair\",                    # Typedef for std::pair<int,int>, not a real class\n]",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "102:yu\n103:ae\n104:cy"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":102,"endLine":107}
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1 | grep -E "(complete|error|Warning)"
```
```
  Formatting complete.
Code generation complete!
real  43.6s
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

**Terminal:**
```sh
grep -c "error:" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01JReEEYseH5iUGYFKosGPPm__vscode-1771608612313/content.txt 2>/dev/null; grep "Finished\|error\[" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01JReEEYseH5iUGYFKosGPPm__vscode-1771608612313/content.txt 2>/dev/null
```
<details>
<summary>Output (8 lines)</summary>

```
1
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
```
</details>

> 💭 There are Rust compilation errors now. Let me see the details.

C++ compiled successfully but there are Rust type errors. Let me see the details.

**Terminal:**
```sh
grep -A5 "error\[E0308\]" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01JReEEYseH5iUGYFKosGPPm__vscode-1771608612313/content.txt
```
<details>
<summary>Output (48 lines)</summary>

```
error[E0308]: mismatched types
  --> crates/opencascade-sys/src/../generated/n_collection.rs:93:13
   |
91 |       pub fn get_type_name() -> String {
   |                                 ------ expected `NCollection_String` becaus
e of return type                                                                92 |           unsafe {
--
error[E0308]: mismatched types
   --> crates/opencascade-sys/src/../generated/n_collection.rs:298:13
    |
296 |       pub fn get_type_name() -> String {
    |                                 ------ expected `NCollection_String` becau
se of return type                                                               297 |           unsafe {
--
error[E0308]: mismatched types
   --> crates/opencascade-sys/src/../generated/n_collection.rs:519:13
    |
517 |       pub fn get_type_name() -> String {
    |                                 ------ expected `NCollection_String` becau
se of return type                                                               518 |           unsafe {
--
error[E0308]: mismatched types
    --> crates/opencascade-sys/src/../generated/n_collection.rs:1160:13
     |
1158 |       pub fn get_type_name() -> String {
     |                                 ------ expected `NCollection_String` beca
use of return type                                                              1159 |           unsafe {
--
error[E0308]: mismatched types
    --> crates/opencascade-sys/src/../generated/n_collection.rs:1399:13
     |
1397 |       pub fn get_type_name() -> String {
     |                                 ------ expected `NCollection_String` beca
use of return type                                                              1398 |           unsafe {
--
error[E0308]: mismatched types
    --> crates/opencascade-sys/src/../generated/n_collection.rs:1647:13
     |
1645 |       pub fn get_type_name() -> String {
     |                                 ------ expected `NCollection_String` beca
use of return type                                                              1646 |           unsafe {
--
error[E0308]: mismatched types
    --> crates/opencascade-sys/src/../generated/n_collection.rs:1953:13
     |
1951 |       pub fn get_type_name() -> String {
     |                                 ------ expected `NCollection_String` beca
use of return type                                                              1952 |           unsafe {
```
</details>

> 💭 The issue is that `get_type_name()` returns `String` in the Rust FFI but the generated code expects it to return `NCollection_String`. But `NCollection_String` is now an actual type since we included the visualization modules, and there's a name collision with `String` (the type alias for `NCollection_String`).
> 
> Let me look at the generated code to understand the issue better.

<details>
<summary>Reading lines 80-100 of <a href="../crates/opencascade-sys/generated/n_collection.rs#L80-L101">n_collection.rs</a></summary>

```
        unsafe {
            crate::OwnedPtr::from_raw(crate::ffi::NCollection_AccAllocator_ctor_size(theBlockSize))
        }
    }

    /// **Source:** `NCollection_AccAllocator.hxx`:205 - `NCollection_AccAllocator::DynamicType()`
    pub fn dynamic_type(&self) -> &crate::ffi::HandleStandardType {
        unsafe { &*(crate::ffi::NCollection_AccAllocator_dynamic_type(self as *const Self)) }
    }

    /// **Source:** `NCollection_AccAllocator.hxx`:205 - `NCollection_AccAllocator::get_type_name()`
    pub fn get_type_name() -> String {
        unsafe {
            std::ffi::CStr::from_ptr(crate::ffi::NCollection_AccAllocator_get_type_name())
                .to_string_lossy()
                .into_owned()
        }
    }

    /// **Source:** `NCollection_AccAllocator.hxx`:205 - `NCollection_AccAllocator::get_type_descriptor()`
    pub fn get_type_descriptor() -> &'static crate::ffi::HandleStandardType {
```
</details>

**Terminal:**
```sh
grep -n "NCollection_String\|type String\|use.*String" /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/n_collection.rs | head -20
```
```
2136:pub use crate::ffi::NCollection_String as String;
```

> 💭 `NCollection_String` is now being exposed as `String` via `pub use crate::ffi::NCollection_String as String;` in the n_collection module. This shadows the standard Rust `String` type. When the generated code uses `-> String` return type, Rust now resolves it to `NCollection_String` instead of `std::string::String`.
> 
> This is a naming collision — `NCollection_String` gets aliased to `String`, which shadows the std prelude String. The generated code for `get_type_name()` returns `String` (meaning `std::string::String`), but Rust now thinks it means `NCollection_String`.
>  
> I need to fix the code generator to use fully qualified `std::string::String` in return types, or reconsider how the alias is exported to avoid shadowing the standard library type. Let me trace through where the generator emits these return type annotations.

The issue is that `NCollection_String` gets aliased to `String` via `pub use`, which shadows `std::string::String` in the generated module. The `get_type_name()` functions return `String` (meaning `std::string::String`), but Rust now resolves it to `NCollection_String`.

Find get_type_name codegen

Searched for text `get_type_name` (`**/crates/opencascade-binding-generator/src/**`), no results

Searched for text `CStr::from_ptr` (`**/crates/opencascade-binding-generator/src/**`), 2 results
- [rust.rs:702](../crates/opencascade-binding-generator/src/codegen/rust.rs#L702)
- [bindings.rs:4769](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4769)

Searched for regex `pub use.*as String` (`**/crates/opencascade-binding-generator/src/**`), no results

<details>
<summary>Searched for regex <code>type_name|DynamicType|TypeName|typeName</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 20 results</summary>

- [parser.rs:1300](../crates/opencascade-binding-generator/src/parser.rs#L1300)
- [parser.rs:1328](../crates/opencascade-binding-generator/src/parser.rs#L1328)
- [parser.rs:1413](../crates/opencascade-binding-generator/src/parser.rs#L1413)
- [parser.rs:1507](../crates/opencascade-binding-generator/src/parser.rs#L1507)
- [parser.rs:1566](../crates/opencascade-binding-generator/src/parser.rs#L1566)
- [parser.rs:1567](../crates/opencascade-binding-generator/src/parser.rs#L1567)
- [parser.rs:1569](../crates/opencascade-binding-generator/src/parser.rs#L1569)
- [parser.rs:1574](../crates/opencascade-binding-generator/src/parser.rs#L1574)
- [parser.rs:1576](../crates/opencascade-binding-generator/src/parser.rs#L1576)
- [parser.rs:1577](../crates/opencascade-binding-generator/src/parser.rs#L1577)
- [parser.rs:1579](../crates/opencascade-binding-generator/src/parser.rs#L1579)
- [parser.rs:1584](../crates/opencascade-binding-generator/src/parser.rs#L1584)
- [parser.rs:1586](../crates/opencascade-binding-generator/src/parser.rs#L1586)
- [model.rs:206](../crates/opencascade-binding-generator/src/model.rs#L206)
- [model.rs:215](../crates/opencascade-binding-generator/src/model.rs#L215)
- [model.rs:216](../crates/opencascade-binding-generator/src/model.rs#L216)
- [model.rs:218](../crates/opencascade-binding-generator/src/model.rs#L218)
- [model.rs:307](../crates/opencascade-binding-generator/src/model.rs#L307)
</details>

<details>
<summary>Searched for regex <code>as.*String|String.*alias|type_alias.*String</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 20 results</summary>

- [parser.rs:29](../crates/opencascade-binding-generator/src/parser.rs#L29)
- [parser.rs:50](../crates/opencascade-binding-generator/src/parser.rs#L50)
- [parser.rs:51](../crates/opencascade-binding-generator/src/parser.rs#L51)
- [parser.rs:121](../crates/opencascade-binding-generator/src/parser.rs#L121)
- [parser.rs:256](../crates/opencascade-binding-generator/src/parser.rs#L256)
- [parser.rs:335](../crates/opencascade-binding-generator/src/parser.rs#L335)
- [parser.rs:863](../crates/opencascade-binding-generator/src/parser.rs#L863)
- [parser.rs:1428](../crates/opencascade-binding-generator/src/parser.rs#L1428)
- [parser.rs:1539](../crates/opencascade-binding-generator/src/parser.rs#L1539)
- [parser.rs:1562](../crates/opencascade-binding-generator/src/parser.rs#L1562)
- [parser.rs:1598](../crates/opencascade-binding-generator/src/parser.rs#L1598)
- [parser.rs:1601](../crates/opencascade-binding-generator/src/parser.rs#L1601)
- [parser.rs:1602](../crates/opencascade-binding-generator/src/parser.rs#L1602)
- [parser.rs:1623](../crates/opencascade-binding-generator/src/parser.rs#L1623)
- [parser.rs:1625](../crates/opencascade-binding-generator/src/parser.rs#L1625)
- [parser.rs:1626](../crates/opencascade-binding-generator/src/parser.rs#L1626)
- [parser.rs:1627](../crates/opencascade-binding-generator/src/parser.rs#L1627)
- [main.rs:123](../crates/opencascade-binding-generator/src/main.rs#L123)
- [main.rs:127](../crates/opencascade-binding-generator/src/main.rs#L127)
- [main.rs:137](../crates/opencascade-binding-generator/src/main.rs#L137)
</details>

<details>
<summary>Reading lines 685-720 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L685-L721">rust.rs</a></summary>

```
    // Build return type string
    let return_type_str = func.return_type.as_ref()
        .map(|rt| format!(" -> {}", rt.rust_reexport_type))
        .unwrap_or_default();

    // Build call expression
    let call_expr = format!("crate::ffi::{}({})", func.cpp_wrapper_name, args.join(", "));

    // Build body with proper conversions: enum returns, OwnedPtr wrapping, and pointer-to-reference
    let reexport_rt = func.return_type.as_ref().map(|rt| rt.rust_reexport_type.as_str());
    let body = if let Some(ref rt) = func.return_type {
        if let Some(ref rust_type) = rt.enum_rust_type {
            format!("{}::try_from({}).unwrap()", rust_type, call_expr)
        } else if rt.needs_unique_ptr {
            format!("crate::OwnedPtr::from_raw({})", call_expr)
        } else if let Some(rtype) = reexport_rt {
            if rtype == "String" {
                format!("std::ffi::CStr::from_ptr({}).to_string_lossy().into_owned()", call_expr)
            } else if rtype.starts_with("&mut ") {
                format!("&mut *({})", call_expr)
            } else if rtype.starts_with('&') {
                format!("&*({})", call_expr)
            } else {
                call_expr
            }
        } else {
            call_expr
        }
    } else {
        call_expr
    };

    // Generate postamble for &mut enum writeback
    let postamble: String = func.params.iter()
        .filter_map(|p| {
            p.mut_ref_enum_rust_type.as_ref().map(|enum_type| {
```
</details>

<details>
<summary>Reading lines 4750-4800 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4750-L4801">bindings.rs</a></summary>

```
/// Build the body expression for a re-export method call.
/// Handles the conversion from FFI raw pointer returns to Rust references/OwnedPtr.
fn build_reexport_body(raw_call: &str, reexport_type: Option<&str>, is_enum: Option<&str>, needs_owned_ptr: bool, is_class_ptr_return: bool) -> String {
    if is_class_ptr_return {
        // Class pointer returns are bound as Option<&T> / Option<&mut T>.
        // The FFI returns a raw pointer; we null-check and convert.
        if let Some(rt) = reexport_type {
            if rt.starts_with("Option<&mut ") {
                return format!("{{ let ptr = unsafe {{ {} }}; if ptr.is_null() {{ None }} else {{ Some(unsafe {{ &mut *ptr }}) }} }}", raw_call);
            }
        }
        return format!("{{ let ptr = unsafe {{ {} }}; if ptr.is_null() {{ None }} else {{ Some(unsafe {{ &*ptr }}) }} }}", raw_call);
    }
    if let Some(enum_type) = is_enum {
        format!("unsafe {{ {}::try_from({}).unwrap() }}", enum_type, raw_call)
    } else if needs_owned_ptr {
        format!("unsafe {{ crate::OwnedPtr::from_raw({}) }}", raw_call)
    } else if let Some(rt) = reexport_type {
        if rt == "String" {
            format!("unsafe {{ std::ffi::CStr::from_ptr({}).to_string_lossy().into_owned() }}", raw_call)
        } else if rt.starts_with("&mut ") {
            format!("unsafe {{ &mut *({}) }}", raw_call)
        } else if rt.starts_with('&') {
            format!("unsafe {{ &*({}) }}", raw_call)
        } else {
            format!("unsafe {{ {} }}", raw_call)
        }
    } else {
        format!("unsafe {{ {} }}", raw_call)
    }
}

pub fn emit_reexport_class(bindings: &ClassBindings, module_name: &str) -> String {
    let cn = &bindings.cpp_name;
    let short_name = &bindings.short_name;

    let mut output = String::new();

    // Source attribution + doc comment for the class
    let class_source = format_source_attribution(
        &bindings.source_header,
        bindings.source_line,
        cn,
    );
    output.push_str(&format!("/// {}\n", class_source));
    if let Some(ref comment) = bindings.doc_comment {
        for line in comment.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                output.push_str("///\n");
            } else {
```
</details>

<details>
<summary>Searched for regex <code>-&gt; String|return_type.*String|rust_return.*String</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 20 results</summary>

- [parser.rs:33](../crates/opencascade-binding-generator/src/parser.rs#L33)
- [parser.rs:934](../crates/opencascade-binding-generator/src/parser.rs#L934)
- [parser.rs:1566](../crates/opencascade-binding-generator/src/parser.rs#L1566)
- [model.rs:181](../crates/opencascade-binding-generator/src/model.rs#L181)
- [model.rs:201](../crates/opencascade-binding-generator/src/model.rs#L201)
- [model.rs:302](../crates/opencascade-binding-generator/src/model.rs#L302)
- [model.rs:383](../crates/opencascade-binding-generator/src/model.rs#L383)
- [model.rs:498](../crates/opencascade-binding-generator/src/model.rs#L498)
- [model.rs:693](../crates/opencascade-binding-generator/src/model.rs#L693)
- [model.rs:723](../crates/opencascade-binding-generator/src/model.rs#L723)
- [model.rs:735](../crates/opencascade-binding-generator/src/model.rs#L735)
- [model.rs:782](../crates/opencascade-binding-generator/src/model.rs#L782)
- [model.rs:845](../crates/opencascade-binding-generator/src/model.rs#L845)
- [model.rs:856](../crates/opencascade-binding-generator/src/model.rs#L856)
- [model.rs:927](../crates/opencascade-binding-generator/src/model.rs#L927)
- [resolver.rs:690](../crates/opencascade-binding-generator/src/resolver.rs#L690)
- [resolver.rs:700](../crates/opencascade-binding-generator/src/resolver.rs#L700)
- [main.rs:991](../crates/opencascade-binding-generator/src/main.rs#L991)
- [collections.rs:213](../crates/opencascade-binding-generator/src/codegen/collections.rs#L213)
- [collections.rs:280](../crates/opencascade-binding-generator/src/codegen/collections.rs#L280)
</details>

Searched for regex `DynamicType|dynamic_type|get_type_name` (`**/crates/opencascade-binding-generator/src/**`), 3 results
- [bindings.rs:2985](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2985)
- [bindings.rs:3013](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3013)
- [bindings.rs:3022](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3022)

<details>
<summary>Searched for regex <code>rust_reexport_type.*=.*\&quot;String\&quot;|reexport_type.*String|\&quot;String\&quot;\.to_string|char.*String</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 18 results</summary>

- [model.rs:437](../crates/opencascade-binding-generator/src/model.rs#L437)
- [model.rs:517](../crates/opencascade-binding-generator/src/model.rs#L517)
- [model.rs:573](../crates/opencascade-binding-generator/src/model.rs#L573)
- [model.rs:635](../crates/opencascade-binding-generator/src/model.rs#L635)
- [model.rs:653](../crates/opencascade-binding-generator/src/model.rs#L653)
- [model.rs:704](../crates/opencascade-binding-generator/src/model.rs#L704)
- [model.rs:728](../crates/opencascade-binding-generator/src/model.rs#L728)
- [model.rs:750](../crates/opencascade-binding-generator/src/model.rs#L750)
- [model.rs:797](../crates/opencascade-binding-generator/src/model.rs#L797)
- [model.rs:871](../crates/opencascade-binding-generator/src/model.rs#L871)
- [resolver.rs:642](../crates/opencascade-binding-generator/src/resolver.rs#L642)
- [resolver.rs:652](../crates/opencascade-binding-generator/src/resolver.rs#L652)
- [resolver.rs:670](../crates/opencascade-binding-generator/src/resolver.rs#L670)
- [resolver.rs:680](../crates/opencascade-binding-generator/src/resolver.rs#L680)
- [type_mapping.rs:209](../crates/opencascade-binding-generator/src/type_mapping.rs#L209)
- [type_mapping.rs:428](../crates/opencascade-binding-generator/src/type_mapping.rs#L428)
- [type_mapping.rs:511](../crates/opencascade-binding-generator/src/type_mapping.rs#L511)
- [bindings.rs:311](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L311)
</details>

<details>
<summary>Searched for regex <code>pub use|type_alias|re_?export.*alias</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 12 results</summary>

- [rust.rs:310](../crates/opencascade-binding-generator/src/codegen/rust.rs#L310)
- [rust.rs:312](../crates/opencascade-binding-generator/src/codegen/rust.rs#L312)
- [rust.rs:623](../crates/opencascade-binding-generator/src/codegen/rust.rs#L623)
- [rust.rs:751](../crates/opencascade-binding-generator/src/codegen/rust.rs#L751)
- [rust.rs:777](../crates/opencascade-binding-generator/src/codegen/rust.rs#L777)
- [rust.rs:802](../crates/opencascade-binding-generator/src/codegen/rust.rs#L802)
- [rust.rs:858](../crates/opencascade-binding-generator/src/codegen/rust.rs#L858)
- [rust.rs:888](../crates/opencascade-binding-generator/src/codegen/rust.rs#L888)
- [rust.rs:891](../crates/opencascade-binding-generator/src/codegen/rust.rs#L891)
- [bindings.rs:4629](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4629)
- [bindings.rs:4808](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4808)
- [bindings.rs:5187](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5187)
</details>

<details>
<summary>Searched for regex <code>rust_reexport_type</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 40 results</summary>

- [rust.rs:652](../crates/opencascade-binding-generator/src/codegen/rust.rs#L652)
- [rust.rs:661](../crates/opencascade-binding-generator/src/codegen/rust.rs#L661)
- [rust.rs:675](../crates/opencascade-binding-generator/src/codegen/rust.rs#L675)
- [rust.rs:687](../crates/opencascade-binding-generator/src/codegen/rust.rs#L687)
- [rust.rs:694](../crates/opencascade-binding-generator/src/codegen/rust.rs#L694)
- [bindings.rs:311](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L311)
- [bindings.rs:334](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L334)
- [bindings.rs:355](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L355)
- [bindings.rs:375](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L375)
- [bindings.rs:1046](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1046)
- [bindings.rs:1072](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1072)
- [bindings.rs:1088](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1088)
- [bindings.rs:1117](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1117)
- [bindings.rs:1133](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1133)
- [bindings.rs:1162](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1162)
- [bindings.rs:1189](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1189)
- [bindings.rs:1197](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1197)
- [bindings.rs:1217](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1217)
- [bindings.rs:1239](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1239)
- [bindings.rs:1246](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1246)
- [bindings.rs:1257](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1257)
- [bindings.rs:1263](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1263)
- [bindings.rs:1672](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1672)
- [bindings.rs:1673](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1673)
- [bindings.rs:1756](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1756)
- [bindings.rs:1757](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1757)
- [bindings.rs:2771](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2771)
- [bindings.rs:2798](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2798)
- [bindings.rs:2811](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2811)
- [bindings.rs:2838](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2838)
- [bindings.rs:2861](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2861)
- [bindings.rs:2900](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2900)
- [bindings.rs:2923](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2923)
- [bindings.rs:2940](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2940)
- [bindings.rs:2941](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2941)
- [bindings.rs:4647](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4647)
- [bindings.rs:4671](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4671)
</details>

Searched for regex `"String"` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 2 results
- [bindings.rs:652](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L652)
- [bindings.rs:4768](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4768)

Searched for regex `const.*char.*\*.*return|c_char.*return|is_c_string|is_const_char` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 4 results
- [bindings.rs:505](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L505)
- [bindings.rs:509](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L509)
- [bindings.rs:993](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L993)
- [bindings.rs:3399](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3399)

<details>
<summary>Reading lines 640-665 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L640-L666">bindings.rs</a></summary>

```
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
            "String".to_string()
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
```
</details>

<details>
<summary>Reading lines 985-1010 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L985-L1011">bindings.rs</a></summary>

```
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
```
</details>

<details>
<summary>Reading lines 500-520 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L500-L521">bindings.rs</a></summary>

```
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

    if returns_by_value {
```
</details>

<details>
<summary>Reading lines 745-815 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L745-L816">rust.rs</a></summary>

```
    writeln!(output, "}}").unwrap();
}

/// Generate a module re-export file
///
/// This generates a file like `gp.rs` that contains:
/// - `pub use crate::ffi::gp_Pnt as Pnt;` for each type
/// - `impl Pnt { ... }` blocks with constructor and method wrappers
pub fn generate_module_reexports(
    module_name: &str,
    _rust_module_name: &str,
    _classes: &[&ParsedClass],
    collections: &[&super::collections::CollectionInfo],
    symbol_table: &crate::resolver::SymbolTable,
    module_bindings: &[&super::bindings::ClassBindings],
    module_fn_bindings: &[&super::bindings::FunctionBinding],
    module_skipped_fns: &[&super::bindings::SkippedSymbol],
    extra_types: &[(String, String)], // (ffi_name, short_name) for types not covered by ClassBindings
) -> String {
    let mut output = String::new();

    // File header
    output.push_str(&format!(
        "//! {} module re-exports\n//!\n//! This file was automatically generated by opencascade-binding-generator.\n//! Do not edit this file directly.\n\n",
        module_name
    ));

    output.push_str("#![allow(dead_code)]\n");
    output.push_str("#![allow(non_snake_case)]\n\n");

    // Generate re-exports for free functions from pre-computed FunctionBindings.
    for func in module_fn_bindings {
        // All free functions become real wrapper functions (not pub use re-exports)
        // so IDE "go to definition" lands in the public module.
        emit_free_function_wrapper(&mut output, func);
    }

    if !module_fn_bindings.is_empty() {
        output.push('\n');
    }

    // Generate Rust enum definitions for enums in this module
    let rust_module = crate::module_graph::module_to_rust_name(module_name);
    if let Some(enum_ids) = symbol_table.enums_by_module.get(&rust_module) {
        for enum_id in enum_ids {
            if let Some(resolved_enum) = symbol_table.enums.get(enum_id) {
                if !matches!(resolved_enum.status, crate::resolver::BindingStatus::Included) {
                    continue;
                }
                emit_rust_enum(&mut output, resolved_enum);
            }
        }
    }

    // Re-export collection types belonging to this module
    for coll in collections {
        output.push_str(&format!(
            "pub use crate::ffi::{} as {};\n",
            coll.typedef_name, coll.short_name
        ));
    }
    if !collections.is_empty() {
        output.push('\n');
    }

    // Generate impl blocks for collection types
    for coll in collections {
        output.push_str(&emit_collection_impl(coll));
    }

    // Group pre-computed bindings by source header for organized output
```
</details>

<details>
<summary>Reading lines 815-900 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L815-L901">rust.rs</a></summary>

```
    // Group pre-computed bindings by source header for organized output
    use std::collections::BTreeMap;
    let mut bindings_by_header: BTreeMap<String, Vec<&super::bindings::ClassBindings>> =
        BTreeMap::new();
    for b in module_bindings {
        bindings_by_header
            .entry(b.source_header.clone())
            .or_default()
            .push(b);
    }

    // Generate re-exports and impl blocks for classes, grouped by header
    // Collect all handle types that are directly re-exported (derived handles with to_handle),
    // so we can avoid duplicating their re-export when they appear as upcast targets.
    let mut directly_exported_handles: std::collections::HashSet<String> = std::collections::HashSet::new();
    for b in module_bindings {
        if b.has_to_handle || b.has_handle_get {
            let handle_type_name = type_mapping::handle_type_name(&b.cpp_name);
            directly_exported_handles.insert(handle_type_name);
        }
    }

    // Also collect handle types referenced by upcast/downcast methods that need re-exporting.
    // These are handle types for base classes (upcast targets) or derived classes (downcast targets)
    // that external crates need to name.
    let mut base_handle_reexports: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for b in module_bindings {
        for hu in &b.handle_upcasts {
            if !directly_exported_handles.contains(&hu.base_handle_name) {
                base_handle_reexports.insert(hu.base_handle_name.clone());
            }
        }
        for hd in &b.handle_downcasts {
            if !directly_exported_handles.contains(&hd.derived_handle_name) {
                base_handle_reexports.insert(hd.derived_handle_name.clone());
            }
        }
    }

    // Emit base handle type re-exports at the top of the module
    if !base_handle_reexports.is_empty() {
        output.push_str("// Handle type re-exports (targets of handle upcasts/downcasts)\n");
        for handle_name in &base_handle_reexports {
            output.push_str(&format!("pub use crate::ffi::{};\n", handle_name));
        }
        output.push_str("\n");
    }

    for (header, header_bindings) in bindings_by_header {
        // Output section header
        output.push_str("// ========================\n");
        output.push_str(&format!("// From {}\n", header));
        output.push_str("// ========================\n\n");

        for bindings in header_bindings {
            output.push_str(&super::bindings::emit_reexport_class(bindings, module_name));
        }
    }

    // Re-export additional types (handles, opaque references, collection iterators)
    // that appear in ffi.rs but aren't covered by ClassBindings or collections.
    // Skip types already re-exported by ClassBindings (directly_exported_handles or base_handle_reexports).
    if !extra_types.is_empty() {
        let mut extra_lines = Vec::new();
        for (ffi_name, short_name) in extra_types {
            // Skip handle types that are already re-exported by emit_reexport_class (has_to_handle or has_handle_get)
            // or by the base handle re-exports section above.
            if directly_exported_handles.contains(ffi_name.as_str())
                || base_handle_reexports.contains(ffi_name.as_str())
            {
                continue;
            }
            if ffi_name == short_name {
                extra_lines.push(format!("pub use crate::ffi::{};\n", ffi_name));
            } else {
                extra_lines.push(format!(
                    "pub use crate::ffi::{} as {};\n",
                    ffi_name, short_name
                ));
            }
        }
        if !extra_lines.is_empty() {
            output.push_str("// ========================\n");
            output.push_str("// Additional type re-exports\n");
            output.push_str("// ========================\n\n");
            for line in &extra_lines {
```
</details>

<details>
<summary>Reading lines 4800-4830 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4800-L4831">bindings.rs</a></summary>

```
            } else {
                output.push_str(&format!("/// {}\n", trimmed));
            }
        }
    }

    // Type alias re-export
    output.push_str(&format!(
        "pub use crate::ffi::{} as {};\n\n",
        cn, short_name
    ));

    // POD structs are Copy types with real fields.
    // They still need CppDeletable because other classes may return them by pointer.
    if bindings.is_pod_struct {
        output.push_str(&format!(
            "unsafe impl crate::CppDeletable for {} {{\n    unsafe fn cpp_delete(ptr: *mut Self) {{\n        crate::ffi::{}_destructor(ptr);\n    }}\n}}\n\n",
            short_name, cn
        ));
        return output;
    }

    // CppDeletable impl (unless protected destructor)
    if !bindings.has_protected_destructor {
        output.push_str(&format!(
            "unsafe impl crate::CppDeletable for {} {{\n    unsafe fn cpp_delete(ptr: *mut Self) {{\n        crate::ffi::{}_destructor(ptr);\n    }}\n}}\n\n",
            short_name, cn
        ));
    }

    // Build impl methods
```
</details>

<details>
<summary>Reading lines 620-660 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L620-L661">bindings.rs</a></summary>

```
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
            "String".to_string()
        }
        // Class pointer returns -> Option<&T> / Option<&mut T>
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(_)) => {
            if let Type::Class(name) = inner.as_ref() {
                let resolved = if let Some(ctx) = reexport_ctx {
                    ctx.resolve_class(name)
                } else {
                    format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
```
</details>

> 💭 So the issue is:
> 1. The generator emits `pub use crate::ffi::NCollection_String as String;` as a type alias for the `NCollection_String` class
> 2. In the same module, `get_type_name()` methods return `String` which is the std Rust String
> 3. But now `String` in the n_collection module resolves to `NCollection_String`
> 
> The simplest fix is to change the return type to `std::string::String` instead of bare `String` in the generated code where `const char*` returns are mapped. This would be unambiguous regardless of what types are in scope.
> 
> 
> Now I need to locate the relevant code in bindings.rs and rust.rs to implement this fix.

I need to qualify `String` as `std::string::String` in the generated code since `NCollection_String` is now aliased to `String` in the same module. Let me fix the return type mapping.

<details>
<summary>Reading lines 645-660 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L645-L661">bindings.rs</a></summary>

```
        Type::Handle(name) => {
            format!(
                "crate::OwnedPtr<crate::ffi::{}>",
                type_mapping::handle_type_name(name)
            )
        }
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == "char") => {
            "String".to_string()
        }
        // Class pointer returns -> Option<&T> / Option<&mut T>
        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(_)) => {
            if let Type::Class(name) = inner.as_ref() {
                let resolved = if let Some(ctx) = reexport_ctx {
                    ctx.resolve_class(name)
                } else {
                    format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
```
</details>

<details>
<summary>Reading lines 695-710 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L695-L711">rust.rs</a></summary>

```
    let body = if let Some(ref rt) = func.return_type {
        if let Some(ref rust_type) = rt.enum_rust_type {
            format!("{}::try_from({}).unwrap()", rust_type, call_expr)
        } else if rt.needs_unique_ptr {
            format!("crate::OwnedPtr::from_raw({})", call_expr)
        } else if let Some(rtype) = reexport_rt {
            if rtype == "String" {
                format!("std::ffi::CStr::from_ptr({}).to_string_lossy().into_owned()", call_expr)
            } else if rtype.starts_with("&mut ") {
                format!("&mut *({})", call_expr)
            } else if rtype.starts_with('&') {
                format!("&*({})", call_expr)
            } else {
                call_expr
            }
        } else {
```
</details>

<details>
<summary>Reading lines 4762-4775 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4762-L4776">bindings.rs</a></summary>

```
    }
    if let Some(enum_type) = is_enum {
        format!("unsafe {{ {}::try_from({}).unwrap() }}", enum_type, raw_call)
    } else if needs_owned_ptr {
        format!("unsafe {{ crate::OwnedPtr::from_raw({}) }}", raw_call)
    } else if let Some(rt) = reexport_type {
        if rt == "String" {
            format!("unsafe {{ std::ffi::CStr::from_ptr({}).to_string_lossy().into_owned() }}", raw_call)
        } else if rt.starts_with("&mut ") {
            format!("unsafe {{ &mut *({}) }}", raw_call)
        } else if rt.starts_with('&') {
            format!("unsafe {{ &*({}) }}", raw_call)
        } else {
            format!("unsafe {{ {} }}", raw_call)
```
</details>

Now I need to change `"String"` to `"std::string::String"` in both the return type mapping and the body check. Let me update all three locations:

<details>
<summary>**Failing: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L651-L654">bindings.rs -1+3</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4770-L4772">-1+2</a>**  
**Failing: <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L701-L703">rust.rs -1+2</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if name == \"char\") => {\n            \"std::string::String\".to_string()\n        }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "651:ef\n652:qn\n653:cy"
    },
    {
      "content": "        if rt == \"std::string::String\" {\n            format!(\"unsafe {{ std::ffi::CStr::from_ptr({}).to_string_lossy().into_owned() }}\", raw_call)",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "4768:aq\n4769:hm"
    },
    {
      "content": "            if rtype == \"std::string::String\" {\n                format!(\"std::ffi::CStr::from_ptr({}).to_string_lossy().into_owned()\", call_expr)",
      "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "lineHashes": "701:ak\n702:wh"
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
      "startLine": 651,
      "endLine": 4772
    },
    {
      "file": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "startLine": 701,
      "endLine": 703
    }
  ]
}
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1 | grep -E "(complete|error|Warning)"
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

Build succeeds! Only a harmless sprintf deprecation warning from OCCT itself. Now let me run the tests.

**Todo list:**
- ✅ Remove viz module exclusions
- ✅ Regenerate bindings
- ✅ Fix generator: using/enum/template
- ✅ Add config exclusions for edge cases
- ✅ Regenerate and build
- ✅ Fix remaining build errors
- 🔄 Run tests
- ⬜ Commit changes

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (37 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.98s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-3404803fe4cd90de)

running 16 tests
test header_deps::tests::test_is_likely_occt_header ... ok
test module_graph::tests::test_extract_rust_type_name ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test parser::tests::test_extract_module_from_header ... ok
test parser::tests::test_extract_template_arg ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
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

real  5.6s
```
</details>

All 16 tests pass. Now run workspace tests:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo test --workspace --exclude opencascade-binding-generator 2>&1
```

**Terminal:**
```sh
grep -E "(test result|FAILED|error\[)" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01H6xfDpWURNtoYXyJJ28ig5__vscode-1771608612348/content.txt
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

All tests pass. Now let me commit.

**Todo list:**
- ✅ Remove viz module exclusions
- ✅ Regenerate bindings
- ✅ Fix generator: using/enum/template
- ✅ Add config exclusions for edge cases
- ✅ Regenerate and build
- ✅ Fix remaining build errors
- ✅ Run tests
- 🔄 Commit changes

Using "Create File"

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
<details>
<summary>Output (18 lines)</summary>

```
[main e2ab46f] Un-exclude visualization/rendering modules from bindings
 245 files changed, 475326 insertions(+), 142329 deletions(-)
 create mode 100644 crates/opencascade-sys/generated/ais.rs
 create mode 100644 crates/opencascade-sys/generated/dsg_prs.rs
 create mode 100644 crates/opencascade-sys/generated/font.rs
 create mode 100644 crates/opencascade-sys/generated/media.rs
 create mode 100644 crates/opencascade-sys/generated/mesh_vs.rs
 create mode 100644 crates/opencascade-sys/generated/prs3d.rs
 create mode 100644 crates/opencascade-sys/generated/prs_dim.rs
 create mode 100644 crates/opencascade-sys/generated/prs_mgr.rs
 create mode 100644 crates/opencascade-sys/generated/select3_d.rs
 create mode 100644 crates/opencascade-sys/generated/select_basics.rs
 create mode 100644 crates/opencascade-sys/generated/select_mgr.rs
 create mode 100644 crates/opencascade-sys/generated/std_prs.rs
 create mode 100644 crates/opencascade-sys/generated/std_select.rs
 create mode 100644 crates/opencascade-sys/generated/t_prs_std.rs
 create mode 100644 crates/opencascade-sys/generated/v3d.rs
 create mode 100644 crates/opencascade-sys/generated/wnt.rs
```
</details>

**Todo list:**
- ✅ Remove viz module exclusions
- ✅ Regenerate bindings
- ✅ Fix generator: using/enum/template
- ✅ Add config exclusions for edge cases
- ✅ Regenerate and build
- ✅ Fix remaining build errors
- ✅ Run tests
- ✅ Commit changes

Done. The 19 visualization/rendering modules are now un-excluded and building successfully. Here's what was needed:

**Generator fixes** (3 new `EntityKind` handlers in `crates/opencascade-binding-generator/src/parser.rs`):
- `UsingDeclaration` in non-public sections — records narrowed method names so inheritance logic blocks them
- `EnumDecl` — records enum names that shadow inherited methods
- `FunctionTemplate` — records template method names that hide base class overloads

**Type fix** — `const char*` return type now maps to `std::string::String` instead of bare `String` to avoid collision with `NCollection_String`'s type alias

**Config exclusions** — `MeshVS_NodePair` (typedef, not a class) and `BRepMesh_GeomTool::CellsCount` (returns that typedef)

*2026-02-20 20:43 · 1171s · Words: 15,404 in · 3,987 out · 53,811 ctx · 52 rounds*

---

<a id="user-2"></a>

## User (2)

> Fix the build errors and warnings when building opencascade-sys. Then commit.

*2026-02-20 20:47*

### Assistant — Claude Opus 4.6

> 💭 Let me first check what the current build errors and warnings are.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-sys 2>&1
```
<details>
<summary>Output (299 lines) — exit code 101</summary>

```
   Compiling opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:427:
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-d408feb5876848ab/out/../../../../OCCT/build/include/BRepClass3d_BndBoxTree.hxx:1:
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/BRepClass3d/BRepClass3d_BndBoxTree.hxx:19:
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-d408feb5876848ab/out/../../../../OCCT/build/include/NCollection_UBTreeFiller.hxx:1:
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/NCollection/NCollection_UBTreeFiller.hxx:198:3: warning: 'sprintf' is deprecated: This function is provided for compatibility reasons only.  Due to security concerns inherent in the design of sprintf(3), it is highly recommended that you use snprintf(3) instead. [-Wdeprecated-declarations]
warning: opencascade-sys@0.2.0:   198 |   sprintf(buf, "Checking UBTree:%8d leaves, balance =%7.2f", aNumber, aLen1 / aLen0);
warning: opencascade-sys@0.2.0:       |   ^
warning: opencascade-sys@0.2.0: /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:278:1: note: 'sprintf' has been explicitly marked deprecated here
warning: opencascade-sys@0.2.0:   278 | __deprecated_msg("This function is provided for compatibility reasons only.  Due to security concerns inherent in the design of sprintf(3), it is highly recommended that you use snprintf(3) instead.")
warning: opencascade-sys@0.2.0:       | ^
warning: opencascade-sys@0.2.0: /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/cdefs.h:227:48: note: expanded from macro '__deprecated_msg'
warning: opencascade-sys@0.2.0:   227 |         #define __deprecated_msg(_msg) __attribute__((__deprecated__(_msg)))
warning: opencascade-sys@0.2.0:       |                                                       ^
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:63064:12: error: no matching function for call to 'operator new'
warning: opencascade-sys@0.2.0:  63064 |     return new BRepMeshData_Curve(*theAllocator);
warning: opencascade-sys@0.2.0:        |            ^
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/BRepMeshData/BRepMeshData_Curve.hxx:28:3: note: candidate function not viable: requires 2 arguments, but 1 was p
... (truncated, 26213 chars total)
```
</details>

**Terminal:**
```sh
grep -E "^(error|warning)\b" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01T6PomJzN5zqzUfuHTKdi9g__vscode-1771608612356/content.txt | sort | uniq -c | sort -rn
```
```
  12 warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys
   4 warning: opencascade-sys@0.2.0:       |   ^~~~~~~~~~~~~~~~
   2 warning: opencascade-sys@0.2.0: /Applications/Xcode.app/Contents/Developer/Platf
   1 error: failed to run custom build command for `opencascade-sys v0.2.0 (/Users/se
```

**Terminal:**
```sh
grep -E "(error:|warning:.*generated)" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01T6PomJzN5zqzUfuHTKdi9g__vscode-1771608612356/content.txt
```
<details>
<summary>Output (14 lines)</summary>

```
ade-sys/generated/wrappers.cpp:63064:12: error: no matching function for call to 'operator new'                                                                 warning: opencascade-sys@0.2.0:  63064 |     return new BRepMeshData_Curve(*theA
ade-sys/generated/wrappers.cpp:63144:12: error: no matching function for call to 'operator new'                                                                 warning: opencascade-sys@0.2.0:  63144 |     return new BRepMeshData_Edge(*theEd
ade-sys/generated/wrappers.cpp:63273:12: error: no matching function for call to 'operator new'                                                                 warning: opencascade-sys@0.2.0:  63273 |     return new BRepMeshData_Face(*theFa
ade-sys/generated/wrappers.cpp:63544:12: error: no matching function for call to 'operator new'                                                                 warning: opencascade-sys@0.2.0:  63544 |     return new BRepMeshData_Wire(*theWi
ade-sys/generated/wrappers.cpp:103114:117: error: use of undeclared identifier 'theFontAspect_enum_'                                                            warning: opencascade-sys@0.2.0:  103114 |     return new Handle(Font_SystemFont)
ade-sys/generated/wrappers.cpp:103117:70: error: use of undeclared identifier 'theFontAspect_enum_'                                                             warning: opencascade-sys@0.2.0:  103117 |     return new Handle(Font_SystemFont)
                               ^                                                warning: opencascade-sys@0.2.0: 1 warning and 6 errors generated.
error: failed to run custom build command for `opencascade-sys v0.2.0 (/Users/se
d/wrappers.cpp:63064:12: error: no matching function for call to 'operator new'   cargo:warning= 63064 |     return new BRepMeshData_Curve(*theAllocator);
d/wrappers.cpp:63144:12: error: no matching function for call to 'operator new'   cargo:warning= 63144 |     return new BRepMeshData_Edge(*theEdge, *theAllocato
d/wrappers.cpp:63273:12: error: no matching function for call to 'operator new'   cargo:warning= 63273 |     return new BRepMeshData_Face(*theFace, *theAllocato
d/wrappers.cpp:63544:12: error: no matching function for call to 'operator new'   cargo:warning= 63544 |     return new BRepMeshData_Wire(*theWire, theEdgeNb, *
d/wrappers.cpp:103114:117: error: use of undeclared identifier 'theFontAspect_enum_'                                                                              cargo:warning= 103114 |     return new Handle(Font_SystemFont)(self_->FindFont
d/wrappers.cpp:103117:70: error: use of undeclared identifier 'theFontAspect_enum_'                                                                               cargo:warning= 103117 |     return new Handle(Font_SystemFont)(self_->FindFont
```
</details>

**Todo list:**
- 🔄 Fix BRepMeshData operator new errors
- ⬜ Fix Font_SystemFont enum errors
- ⬜ Regenerate and verify build
- ⬜ Run tests and commit

Two categories of errors:
1. **BRepMeshData_Curve/Edge/Face/Wire** — `no matching function for call to 'operator new'` (protected new from NCollection allocator)
2. **Font_SystemFont** — `use of undeclared identifier 'theFontAspect_enum_'` (enum parameter codegen issue)

Let me investigate both.

Research BRepMeshData and Font errors

Searched for text `BRepMeshData` (`**/crates/opencascade-sys/bindings.toml`), no results

Searched for files matching `**/BRepMeshData/BRepMeshData_Curve.hxx`, 1 match
- [BRepMeshData_Curve.hxx](../crates/occt-sys/OCCT/src/BRepMeshData/BRepMeshData_Curve.hxx)

Searched for files matching `**/Font/Font_FontMgr.hxx`, 1 match
- [Font_FontMgr.hxx](../crates/occt-sys/OCCT/src/Font/Font_FontMgr.hxx)

Searched for text `theFontAspect_enum_` (`**/crates/opencascade-sys/generated/**`), 2 results
- [wrappers.cpp:103114](../crates/opencascade-sys/generated/wrappers.cpp#L103114)
- [wrappers.cpp:103117](../crates/opencascade-sys/generated/wrappers.cpp#L103117)

<details>
<summary>Searched for text <code>enum_</code> (<code>**/crates/opencascade-binding-generator/src/codegen/**</code>), 20 results</summary>

- [cpp.rs:206](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L206)
- [rust.rs:158](../crates/opencascade-binding-generator/src/codegen/rust.rs#L158)
- [rust.rs:184](../crates/opencascade-binding-generator/src/codegen/rust.rs#L184)
- [rust.rs:482](../crates/opencascade-binding-generator/src/codegen/rust.rs#L482)
- [rust.rs:496](../crates/opencascade-binding-generator/src/codegen/rust.rs#L496)
- [rust.rs:659](../crates/opencascade-binding-generator/src/codegen/rust.rs#L659)
- [rust.rs:663](../crates/opencascade-binding-generator/src/codegen/rust.rs#L663)
- [rust.rs:678](../crates/opencascade-binding-generator/src/codegen/rust.rs#L678)
- [rust.rs:696](../crates/opencascade-binding-generator/src/codegen/rust.rs#L696)
- [rust.rs:720](../crates/opencascade-binding-generator/src/codegen/rust.rs#L720)
- [rust.rs:721](../crates/opencascade-binding-generator/src/codegen/rust.rs#L721)
- [rust.rs:788](../crates/opencascade-binding-generator/src/codegen/rust.rs#L788)
- [rust.rs:789](../crates/opencascade-binding-generator/src/codegen/rust.rs#L789)
- [rust.rs:790](../crates/opencascade-binding-generator/src/codegen/rust.rs#L790)
- [bindings.rs:317](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L317)
- [bindings.rs:319](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L319)
- [bindings.rs:321](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L321)
</details>

Read [BRepMeshData_Curve.hxx](../crates/occt-sys/OCCT/src/BRepMeshData/BRepMeshData_Curve.hxx)

Read [Font_FontMgr.hxx](../crates/occt-sys/OCCT/src/Font/Font_FontMgr.hxx#1-1), lines 1 to 200

Read [wrappers.cpp](../crates/opencascade-sys/generated/wrappers.cpp#103100-103100), lines 103100 to 103130

Read [cpp.rs](../crates/opencascade-binding-generator/src/codegen/cpp.rs#180-180), lines 180 to 260

Searched for regex `exclude_modules|BRepMesh` (`**/crates/opencascade-sys/bindings.toml`), 3 results
- [bindings.toml:8](../crates/opencascade-sys/bindings.toml#L8)
- [bindings.toml:20](../crates/opencascade-sys/bindings.toml#L20)
- [bindings.toml:87](../crates/opencascade-sys/bindings.toml#L87)

Read [bindings.toml](../crates/opencascade-sys/bindings.toml#18-18), lines 18 to 70

<details>
<summary>Searched for text <code>DEFINE_INC_ALLOC</code> (<code>**/crates/occt-sys/OCCT/src/**/*.hxx</code>), 5 results</summary>

- [IMeshData_Types.hxx:54](../crates/occt-sys/OCCT/src/IMeshData/IMeshData_Types.hxx#L54)
- [BRepMeshData_PCurve.hxx:27](../crates/occt-sys/OCCT/src/BRepMeshData/BRepMeshData_PCurve.hxx#L27)
- [BRepMeshData_Wire.hxx:26](../crates/occt-sys/OCCT/src/BRepMeshData/BRepMeshData_Wire.hxx#L26)
- [BRepMeshData_Face.hxx:27](../crates/occt-sys/OCCT/src/BRepMeshData/BRepMeshData_Face.hxx#L27)
- [BRepMeshData_Edge.hxx:27](../crates/occt-sys/OCCT/src/BRepMeshData/BRepMeshData_Edge.hxx#L27)
</details>

<details>
<summary>Searched for regex <code>mut_ref_enum_cpp_name|mut_ref.*enum|enum.*mut_ref</code> (<code>**/crates/opencascade-binding-generator/src/codegen/**</code>), 20 results</summary>

- [rust.rs:659](../crates/opencascade-binding-generator/src/codegen/rust.rs#L659)
- [rust.rs:678](../crates/opencascade-binding-generator/src/codegen/rust.rs#L678)
- [rust.rs:720](../crates/opencascade-binding-generator/src/codegen/rust.rs#L720)
- [bindings.rs:319](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L319)
- [bindings.rs:321](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L321)
- [bindings.rs:361](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L361)
- [bindings.rs:363](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L363)
- [bindings.rs:486](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L486)
- [bindings.rs:522](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L522)
- [bindings.rs:874](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L874)
- [bindings.rs:995](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L995)
- [bindings.rs:1004](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1004)
- [bindings.rs:1052](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1052)
- [bindings.rs:1053](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1053)
- [bindings.rs:1076](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1076)
- [bindings.rs:1077](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1077)
- [bindings.rs:1121](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1121)
- [bindings.rs:1122](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1122)
- [bindings.rs:1166](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1166)
- [bindings.rs:1167](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1167)
</details>

Read [IMeshData_Types.hxx](../crates/occt-sys/OCCT/src/IMeshData/IMeshData_Types.hxx#50-50), lines 50 to 80

Searched for regex `mut_ref_enum|enum_cpp_name|enum_arg_expr|theFontAspect` (`**/crates/opencascade-binding-generator/src/codegen/cpp.rs`), 1 result
- [cpp.rs:206](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L206)

<details>
<summary>Searched for text <code>DEFINE_NCOLLECTION_ALLOC</code> (<code>**/crates/occt-sys/OCCT/src/**/*.hxx</code>), 5 results</summary>

- [OpenGl_ShadowMap.hxx:109](../crates/occt-sys/OCCT/src/OpenGl/OpenGl_ShadowMap.hxx#L109)
- [Poly_CoherentTriPtr.hxx:103](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L103)
- [IMeshData_Types.hxx:55](../crates/occt-sys/OCCT/src/IMeshData/IMeshData_Types.hxx#L55)
- [NCollection_BaseMap.hxx:42](../crates/occt-sys/OCCT/src/NCollection/NCollection_BaseMap.hxx#L42)
- [NCollection_DynamicArray.hxx:61](../crates/occt-sys/OCCT/src/NCollection/NCollection_DynamicArray.hxx#L61)
</details>

Read [cpp.rs](../crates/opencascade-binding-generator/src/codegen/cpp.rs)

<details>
<summary>Searched for regex <code>emit_cpp_class|emit_cpp|fn emit</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 5 results</summary>

- [bindings.rs:3536](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3536)
- [bindings.rs:4782](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4782)
- [bindings.rs:5279](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5279)
- [bindings.rs:5338](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5338)
- [bindings.rs:5537](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5537)
</details>

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3536-3536), lines 3536 to 3700

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3700-3700), lines 3700 to 3850

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#3850-3850), lines 3850 to 4050

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#4050-4050), lines 4050 to 4150

<details>
<summary>Searched for text <code>_enum_</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 30 results</summary>

- [bindings.rs:319](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L319)
- [bindings.rs:321](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L321)
- [bindings.rs:361](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L361)
- [bindings.rs:363](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L363)
- [bindings.rs:486](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L486)
- [bindings.rs:522](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L522)
- [bindings.rs:884](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L884)
- [bindings.rs:1018](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1018)
- [bindings.rs:1022](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1022)
- [bindings.rs:1049](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1049)
- [bindings.rs:1052](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1052)
- [bindings.rs:1053](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1053)
- [bindings.rs:1062](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1062)
- [bindings.rs:1076](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1076)
- [bindings.rs:1077](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1077)
- [bindings.rs:1121](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1121)
- [bindings.rs:1122](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1122)
- [bindings.rs:1166](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1166)
- [bindings.rs:1167](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1167)
- [bindings.rs:1201](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1201)
- [bindings.rs:1202](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1202)
- [bindings.rs:1210](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1210)
- [bindings.rs:1515](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1515)
- [bindings.rs:1595](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1595)
- [bindings.rs:1645](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1645)
- [bindings.rs:1649](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1649)
- [bindings.rs:1742](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1742)
- [bindings.rs:2088](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2088)
- [bindings.rs:2192](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2192)
- [bindings.rs:2198](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2198)
</details>

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#1010-1010), lines 1010 to 1100

Searched for text `classify_wrapper_kind` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 2 results
- [bindings.rs:504](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L504)
- [bindings.rs:1742](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1742)

Read [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs#486-486), lines 486 to 560

Read [NCollection_DefineAlloc.hxx](../crates/occt-sys/OCCT/src/NCollection/NCollection_DefineAlloc.hxx)

Read [IMeshData_Types.hxx](../crates/occt-sys/OCCT/src/IMeshData/IMeshData_Types.hxx#54-54), lines 54 to 65

Read [BRepMeshData](../crates/occt-sys/OCCT/src/BRepMeshData)

Read [bindings.toml](../crates/opencascade-sys/bindings.toml#68-68), lines 68 to 108

Good. Now let me fix both issues. First, let me look at the `classify_wrapper_kind` function to fix the priority order:

<details>
<summary>Reading lines 500-550 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L500-L551">bindings.rs</a></summary>

```
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


/// Convert a parameter to C++ argument expression for extern "C" wrappers.
/// Dereferences pointers to match C++ method signatures (which take references).
fn param_to_cpp_extern_c_arg(param_name: &str, ty: &Type) -> String {
    match ty {
        Type::ConstRef(_) | Type::MutRef(_) => format!("*{}", param_name),
        _ => param_name.to_string(),
    }
}

/// Context for resolving C++ class names to their public re-exported Rust type
```
</details>

Now I see the issue clearly. The `returns_by_value` check is before `has_mut_ref_enum_params`. I also need to handle the BRepMeshData issue. Let me check if adding it to `exclude_modules` is appropriate, and also look at the existing protection mechanism.

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L520-L524">bindings.rs -4+4</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "520:ww,521:ar,522:rz,523:lc",
      "content": "    if has_mut_ref_enum_params(&method.params, all_enums) {\n        WrapperKind::MutRefEnumParam\n    } else if returns_by_value {\n        WrapperKind::ByValueReturn"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":520,"endLine":523}
```
</details>

Now fix the BRepMeshData issue by adding to exclude_modules. Let me also check whether the existing `BRepMeshData` exclusion comment already covers this:

<details>
<summary>Reading lines 45-65 of <a href="../crates/opencascade-sys/bindings.toml#L45-L66">bindings.toml</a></summary>

```
    # Internal-only class: all constructors/destructors are private, accessible only
    # to friend classes TDF_Data and TDF_Label. No public API returns TDF_LabelNode*;
    # it's entirely hidden behind the TDF_Label value-type facade.
    "TDF_LabelNode.hxx",

    # Deprecated class: Message_ProgressScope should be used instead.
    "Message_ProgressSentry.hxx",

    # Deprecated class (OCCT 7.9): NCollection_Primes should be used instead.
    "TCollection.hxx",

    # Third-party dependency (rapidjson): these headers #include rapidjson which
    # is not bundled with our OCCT build.
    "RWGltf_GltfOStreamWriter.hxx",
    "RWGltf_GltfJsonParser.hxx",
    "RWGltf_CafWriter.hxx",
    "RWGltf_GltfAccessor.hxx",

    # Windows-only: includes <windows.h> which causes a fatal error on non-Windows
    # and corrupts clang's type resolution for subsequent headers.
    "OSD_WNT.hxx",
```
</details>

<details>
<summary>Reading lines 20-42 of <a href="../crates/opencascade-sys/bindings.toml#L20-L43">bindings.toml</a></summary>

```
exclude_modules = [
    # === Platform-specific GUI ===
    "Cocoa",               # macOS Cocoa bindings
    "WNT",                 # Windows native types
    "Wasm",                # WebAssembly bindings
    "Xw",                  # X Window system bindings

    # === Precompiled header stubs (not real modules) ===
    "TKBRep",              # PCH stub
    "TKDEIGES",            # PCH stub
    "TKDESTEP",            # PCH stub
    "TKMath",              # PCH stub
    "TKMesh",              # PCH stub
    "TKV3d",               # PCH stub
    "TKXSBase",            # PCH stub
    "TKernel",             # PCH stub

]

# Exclude specific headers (even if their module is in `modules` above).
exclude_headers = [
    # Protected operator new/delete: class cannot be heap-allocated via std::make_unique.
    "Message_LazyProgressScope.hxx",  # Inherits protected new/delete from Message_ProgressScope
```
</details>

I see the old `BRepMeshData` exclusion was already removed (it used to be there). The BRepMeshData classes have `DEFINE_INC_ALLOC` which hides global operator new. I need to add these individual headers to `exclude_headers`:

<details>
<summary>**Failing: <a href="../crates/opencascade-sys/bindings.toml#L41-L53">bindings.toml -1+12</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    # Protected operator new/delete: class cannot be heap-allocated via std::make_unique.\n    \"Message_LazyProgressScope.hxx\",  # Inherits protected new/delete from Message_ProgressScope\n    \"Poly_CoherentTriPtr.hxx\",         # No suitable operator delete\n\n    # DEFINE_INC_ALLOC macro hides global operator new — these classes can only be\n    # heap-allocated through NCollection_BaseAllocator placement new.\n    \"BRepMeshData_Curve.hxx\",\n    \"BRepMeshData_Edge.hxx\",\n    \"BRepMeshData_Face.hxx\",\n    \"BRepMeshData_Model.hxx\",\n    \"BRepMeshData_PCurve.hxx\",\n    \"BRepMeshData_Wire.hxx\",",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "41:ve\n42:kg"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":41,"endLine":53}
```
</details>

Now regenerate and build:

**Todo list:**
- ✅ Fix BRepMeshData operator new errors
- ✅ Fix Font_SystemFont enum errors
- 🔄 Regenerate and verify build
- ⬜ Run tests and commit

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1 | grep -E "(complete|error|Warning)"
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

**Terminal:**
```sh
grep -E "(error|Finished)" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01F1yXSgJUUKtjAscs7z8gmp__vscode-1771608612398/content.txt
```
```
error[E0106]: missing lifetime specifier
For more information about this error, try `rustc --explain E0106`.
error: could not compile `opencascade-sys` (lib) due to 1 previous error
```

One Rust error. Let me see it:

**Terminal:**
```sh
grep -B2 -A10 "E0106" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01F1yXSgJUUKtjAscs7z8gmp__vscode-1771608612398/content.txt
```
<details>
<summary>Output (27 lines)</summary>

```
warning: opencascade-sys@0.2.0:        |                   ^
warning: opencascade-sys@0.2.0: 2 warnings generated.
error[E0106]: missing lifetime specifier
  --> crates/opencascade-sys/src/../generated/express.rs:11:20
   |
11 | pub fn schema() -> &mut crate::ffi::HandleExpressSchema {
   |                    ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but there is n
o value for it to be borrowed from                                              help: consider using the `'static` lifetime, but this is uncommon unless you're 
returning a borrowed value from a `const` or a `static`                            |
11 | pub fn schema() -> &'static mut crate::ffi::HandleExpressSchema {
   |                     +++++++
--
   |

For more information about this error, try `rustc --explain E0106`.
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencasc
ade-rs/crates/opencascade-sys/generated/wrappers.cpp:427:                       warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencasc
ade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include/BRepClass3d_BndBoxTree.hxx:1:                                     warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencasc
ade-rs/crates/occt-sys/OCCT/src/BRepClass3d/BRepClass3d_BndBoxTree.hxx:19:      warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencasc
ade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include/NCollection_UBTreeFiller.hxx:1:                                   warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys
/OCCT/src/NCollection/NCollection_UBTreeFiller.hxx:198:3: warning: 'sprintf' is deprecated: This function is provided for compatibility reasons only.  Due to security concerns inherent in the design of sprintf(3), it is highly recommended that you use snprintf(3) instead. [-Wdeprecated-declarations]                    warning: opencascade-sys@0.2.0:   198 |   sprintf(buf, "Checking UBTree:%8d leav
es, balance =%7.2f", aNumber, aLen1 / aLen0);                                   warning: opencascade-sys@0.2.0:       |   ^
warning: opencascade-sys@0.2.0: /Applications/Xcode.app/Contents/Developer/Platf
orms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:278:1: note: 'sprintf' has been explicitly marked deprecated here                           warning: opencascade-sys@0.2.0:   278 | __deprecated_msg("This function is provi
ded for compatibility reasons only.  Due to security concerns inherent in the design of sprintf(3), it is highly recommended that you use snprintf(3) instead.")warning: opencascade-sys@0.2.0:       | ^
```
</details>

The error is in `express.rs` — a free function `schema()` returning `&mut` without a lifetime. Express module was supposed to be excluded. Let me check:

**Terminal:**
```sh
grep -n "Express" /Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml
```
**Exit code:** 1

Express module exclusion got lost when the previous edits were done. Let me check the current state of exclude_modules and the generated express.rs:

<details>
<summary>Reading lines 20-38 of <a href="../crates/opencascade-sys/bindings.toml#L20-L39">bindings.toml</a></summary>

```
exclude_modules = [
    # === Platform-specific GUI ===
    "Cocoa",               # macOS Cocoa bindings
    "WNT",                 # Windows native types
    "Wasm",                # WebAssembly bindings
    "Xw",                  # X Window system bindings

    # === Precompiled header stubs (not real modules) ===
    "TKBRep",              # PCH stub
    "TKDEIGES",            # PCH stub
    "TKDESTEP",            # PCH stub
    "TKMath",              # PCH stub
    "TKMesh",              # PCH stub
    "TKV3d",               # PCH stub
    "TKXSBase",            # PCH stub
    "TKernel",             # PCH stub

]

```
</details>

Yes, the Express exclusion was lost during the previous session's edits. I need to add it back:

<details>
<summary>**Failing: <a href="../crates/opencascade-sys/bindings.toml#L35-L40">bindings.toml -1+5</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    \"TKernel\",             # PCH stub\n\n    # === EXPRESS schema parser (internal codegen tool, not user-facing) ===\n    \"Express\",             # EXPRESS schema parser (String type conflicts with Rust String)\n]",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "35:qm\n36:dd\n37:cy"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":35,"endLine":41}
```
</details>

Now regenerate and build again:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1 | grep -E "(complete|error)"
```
```
  Formatting complete.
Code generation complete!
real  45.8s
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

Build succeeds, but there are 6 warnings about empty `Image_Color*` structs not being FFI-safe. These are Image module types that are now included. Let me fix those by excluding these types that have no fields (they're simple color type aliases with `union` layouts that clang doesn't expose as fields):

<details>
<summary>Reading lines 106-120 of <a href="../crates/opencascade-sys/bindings.toml#L106-L121">bindings.toml</a></summary>

```
    "MeshVS_NodePair",                    # Typedef for std::pair<int,int>, not a real class
]

[general]
resolve_deps = true

# Opaque types defined in manual/ files. These are not parsed from OCCT headers
# but referenced by auto-generated bindings. The generator adds them to the known
# class set so methods using them aren't skipped as "unknown type".
# Format: "CppTypeName" = { header = "Header.hxx" }
[manual_types]
Standard_OStream = { header = "Standard_OStream.hxx" }
Standard_IStream = { header = "Standard_IStream.hxx" }
Standard_SStream = { header = "Standard_SStream.hxx" }

```
</details>

<details>
<summary>**Failing: <a href="/Users/sethml/src/opencascade-sys/crates/opencascade-sys/bindings.toml#L106-L117">bindings.toml -1+11</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    \"MeshVS_NodePair\",                    # Typedef for std::pair<int,int>, not a real class\n\n    # Image color union types: clang exposes no fields for these packed unions,\n    # producing empty structs that trigger FFI-safety warnings.\n    \"Image_ColorRGB\",\n    \"Image_ColorRGB32\",\n    \"Image_ColorRGBA\",\n    \"Image_ColorBGR\",\n    \"Image_ColorBGR32\",\n    \"Image_ColorBGRA\",\n]",
      "filePath": "/Users/sethml/src/opencascade-sys/crates/opencascade-sys/bindings.toml",
      "lineHashes": "106:sl\n107:cy"
    }
  ]
}
```
**Output:**
```
{
  "applied": 0,
  "failed": 1,
  "errors": [
    {
      "filePath": "/Users/sethml/src/opencascade-sys/crates/opencascade-sys/bindings.toml",
      "lineHashes": "106:sl\n107:cy",
      "error": "cannot open file:///Users/sethml/src/opencascade-sys/crates/opencascade-sys/bindings.toml. Detail: Unable to read file '/Users/sethml/src/opencascade-sys/crates/opencascade-sys/bindings.toml' (Error: Unable to resolve nonexistent file '/Users/sethml/src/opencascade-sys/crates/opencascade-sys/bindings.toml')"
    }
  ]
}
```
</details>

<details>
<summary>**Failing: <a href="../crates/opencascade-sys/bindings.toml#L106-L117">bindings.toml -1+11</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    \"MeshVS_NodePair\",                    # Typedef for std::pair<int,int>, not a real class\n\n    # Image color union types: clang exposes no fields for these packed unions,\n    # producing empty structs that trigger FFI-safety warnings.\n    \"Image_ColorRGB\",\n    \"Image_ColorRGB32\",\n    \"Image_ColorRGBA\",\n    \"Image_ColorBGR\",\n    \"Image_ColorBGR32\",\n    \"Image_ColorBGRA\",\n]",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "106:sl\n107:cy"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":106,"endLine":117}
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1 | grep -E "(complete|error)"
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

The Image_Color* warnings are still there — the `exclude_classes` config excludes them from the reexport module but they're still being generated as empty structs in ffi.rs with destructors. These come from headers, not classes. Let me exclude the specific headers instead:

**Terminal:**
```sh
grep -rn "Image_ColorRGB\b" /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/Image/ | grep "\.hxx:" | head -10
```
```
/Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/Image/Image_Color.hxx:22:struct Image_ColorRGB
```

They're all defined in `Image_Color.hxx`. Let me exclude that header:

<details>
<summary>Reading lines 70-80 of <a href="../crates/opencascade-sys/bindings.toml#L70-L81">bindings.toml</a></summary>

```
    "RWGltf_GltfOStreamWriter.hxx",
    "RWGltf_GltfJsonParser.hxx",
    "RWGltf_CafWriter.hxx",
    "RWGltf_GltfAccessor.hxx",

    # Windows-only: includes <windows.h> which causes a fatal error on non-Windows
    # and corrupts clang's type resolution for subsequent headers.
    "OSD_WNT.hxx",
]

# Include specific individual headers (from modules not fully included above).
```
</details>

<details>
<summary>**Failing: <a href="../crates/opencascade-sys/bindings.toml#L75-L83">bindings.toml -1+8</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    # Windows-only: includes <windows.h> which causes a fatal error on non-Windows\n    # and corrupts clang's type resolution for subsequent headers.\n    \"OSD_WNT.hxx\",\n\n    # Packed color-component unions: clang exposes no fields, producing empty\n    # opaque structs that trigger Rust FFI-safety warnings.\n    \"Image_Color.hxx\",\n]",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "75:il\n76:cq\n77:fk\n78:cy"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":75,"endLine":85}
```
</details>

Also remove the now-redundant Image_Color entries from exclude_classes:

<details>
<summary>Reading lines 108-125 of <a href="../crates/opencascade-sys/bindings.toml#L108-L126">bindings.toml</a></summary>

```
exclude_classes = [
    "Message_ProgressScope::NullString",  # Forward-declared only, no full definition
    "MeshVS_NodePair",                    # Typedef for std::pair<int,int>, not a real class

    # Image color union types: clang exposes no fields for these packed unions,
    # producing empty structs that trigger FFI-safety warnings.
    "Image_ColorRGB",
    "Image_ColorRGB32",
    "Image_ColorRGBA",
    "Image_ColorBGR",
    "Image_ColorBGR32",
    "Image_ColorBGRA",
]

[general]
resolve_deps = true

# Opaque types defined in manual/ files. These are not parsed from OCCT headers
```
</details>

<details>
<summary>**Failing: <a href="../crates/opencascade-sys/bindings.toml#L108-L112">bindings.toml -1+4</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "exclude_classes = [\n    \"Message_ProgressScope::NullString\",  # Forward-declared only, no full definition\n    \"MeshVS_NodePair\",                    # Typedef for std::pair<int,int>, not a real class\n]",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "108:yu\n109:ae\n110:sl\n111:dd\n112:sm\n113:oa\n114:zh\n115:gz\n116:ik\n117:by\n118:si\n119:ca\n120:cy"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":108,"endLine":123}
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1 | grep -E "(complete|error)"
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

Clean build — no Rust errors or warnings. The only remaining output is the OCCT-internal `sprintf` deprecation warning from Apple's SDK which is not our code. Now let me run tests:

**Todo list:**
- ✅ Fix BRepMeshData operator new errors
- ✅ Fix Font_SystemFont enum errors
- ✅ Regenerate and verify build
- 🔄 Run tests and commit

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (37 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.96s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-3404803fe4cd90de)

running 16 tests
test header_deps::tests::test_is_likely_occt_header ... ok
test module_graph::tests::test_extract_rust_type_name ... ok
test parser::tests::test_extract_module_from_header ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test parser::tests::test_map_standard_type ... ok
test parser::tests::test_extract_template_arg ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
test resolver::tests::test_safe_method_name ... ok
test resolver::tests::test_safe_param_name ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
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

real  5.0s
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo test --workspace --exclude opencascade-binding-generator 2>&1 | grep -E "(test result|FAILED|error)"
```

All tests pass. Now let me commit:

**Terminal:**
```sh
rm /Users/sethml/src/opencascade-rs/tmp/commit-msg.txt 2>/dev/null; true
```

Created [commit-msg.txt](../tmp/commit-msg.txt)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git add -A && git commit -F tmp/commit-msg.txt
```
```
[main 552b5c3] Fix build errors and warnings after un-excluding visualization modules
 21 files changed, 14018 insertions(+), 577 deletions(-)
 create mode 100644 crates/opencascade-sys/generated/ft.rs
```

**Todo list:**
- ✅ Fix BRepMeshData operator new errors
- ✅ Fix Font_SystemFont enum errors
- ✅ Regenerate and verify build
- ✅ Run tests and commit

Done. Three categories of fixes:

1. **BRepMeshData headers excluded** — `DEFINE_INC_ALLOC` macro hides global `operator new`, preventing heap allocation in generated wrappers
2. **classify_wrapper_kind priority fix** — `MutRefEnumParam` now checked before `ByValueReturn` so enum preamble/postamble code is properly generated (fixed `theFontAspect_enum_` undeclared identifier)
3. **Express module + Image_Color.hxx excluded** — restored accidentally lost exclusion; excluded packed union color types that produce empty FFI-unsafe structs

*2026-02-20 21:06 · 1175s · Words: 2,309 in · 1,522 out · 72,996 ctx · 40 rounds*

---

<a id="user-3"></a>

## User (3)

> What is BRepMeshData for in OCCT?

*2026-02-20 21:12*

### Assistant — Claude Sonnet 4.6

> 💭 The user is asking about the purpose of BRepMeshData in OCCT. Let me look at the OCCT documentation/headers to explain this.

`BRepMeshData` is the **concrete implementation layer of OCCT's mesh data model** for BRep shapes.

When OCCT tessellates a BRep solid into a triangle mesh (via `BRepMesh_IncrementalMesh`), it needs to store that mesh topology. The storage is defined by abstract interfaces in the `IMeshData` package (`IMeshData_Face`, `IMeshData_Edge`, etc.), and `BRepMeshData` provides the concrete implementations:

| Class | Role |
|---|---|
| `BRepMeshData_Model` | Root container for the mesh of an entire shape |
| `BRepMeshData_Face` | Mesh data for one BRep face (triangulation) |
| `BRepMeshData_Edge` | Mesh data for one BRep edge (discretized polyline) |
| `BRepMeshData_Wire` | Mesh data for a wire (ordered collection of edges on a face) |
| `BRepMeshData_Curve` | 3D polyline approximation of an edge |
| `BRepMeshData_PCurve` | 2D parametric curve on a face |

All objects in this package are allocated through `NCollection_BaseAllocator` rather than the global heap — that's the `DEFINE_INC_ALLOC` macro. This is a performance optimization: the mesher allocates a pool upfront and creates all mesh objects within it, then frees them all at once when the mesh is finalized.

The resulting triangulation ends up stored in `Poly_Triangulation` nodes attached to each `TopoDS_Face`, which is what you actually use afterward for rendering or export.

*2026-02-20 21:12 · 11s · Words: 6 in · 224 out · 79,369 ctx*

---

<a id="user-4"></a>

## User (4)

> Can you access the BRepMeshData objects, or are they not useful for a library client?

*2026-02-20 21:13*

### Assistant — Gpt 4.1

You generally **cannot access `BRepMeshData` objects directly** as a library client. They are:

- **Internal implementation details** of OCCT's meshing subsystem
- Not exposed via public API: constructors/destructors are protected, and no public OCCT methods return pointers to them
- Allocated via custom allocators (`NCollection_BaseAllocator`), not standard heap

Instead, OCCT exposes mesh results through public types like `Poly_Triangulation` attached to `TopoDS_Face`. All client-facing mesh access is via these public types, not the internal `BRepMeshData` classes.

So for FFI or library clients, `BRepMeshData` is not useful or accessible — you should use the public mesh interfaces (`Poly_Triangulation`, `BRepMesh_IncrementalMesh`, etc.).

*2026-02-20 21:13 · 6s · Words: 15 in · 98 out*

---

<a id="user-5"></a>

## User (5)

> Can we make these work? Even without new/delete, it should be possible to access these objects, if we just leave construction and destruction out of the bound API. Make them work, then commit.

*2026-02-20 21:20*

### Assistant — Claude Opus 4.6

> 💭 The user wants me to make `Message_LazyProgressScope` and `Poly_CoherentTriPtr` work by including them in the bindings but without generating constructors and destructors. Let me understand the current mechanism and figure out how to do this.
> 
> Currently these headers are excluded entirely via `exclude_headers`. The issue is that these classes have protected `operator new/delete`, so the generated code tries to heap-allocate them and fails.
> 
> The generator already has `exclude_classes` which excludes classes from destructor generation. But we need a mechanism to:
> 1. Include the headers (remove from `exclude_headers`)
> 2. Skip constructor generation for these classes
> 
> 
> 3. Skip destructor generation for these classes
> 
> Now I'm examining how the generator currently handles the `exclude_classes` mechanism to see if I can extend it to also skip constructors, or if I need to add a separate configuration option.

**Todo list:**
- 🔄 Understand exclude_classes mechanism
- ⬜ Add no-construct/no-destruct support
- ⬜ Un-exclude the two headers
- ⬜ Regenerate, build, test, commit

Research exclude_classes codegen

<details>
<summary>Searched for text <code>exclude_classes</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 8 results</summary>

- [rust.rs:148](../crates/opencascade-binding-generator/src/codegen/rust.rs#L148)
- [rust.rs:153](../crates/opencascade-binding-generator/src/codegen/rust.rs#L153)
- [main.rs:96](../crates/opencascade-binding-generator/src/main.rs#L96)
- [main.rs:139](../crates/opencascade-binding-generator/src/main.rs#L139)
- [main.rs:376](../crates/opencascade-binding-generator/src/main.rs#L376)
- [main.rs:596](../crates/opencascade-binding-generator/src/main.rs#L596)
- [main.rs:643](../crates/opencascade-binding-generator/src/main.rs#L643)
- [config.rs:48](../crates/opencascade-binding-generator/src/config.rs#L48)
</details>

<details>
<summary>Searched for regex <code>destructor|constructor</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 20 results</summary>

- [bindings.rs:8](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L8)
- [bindings.rs:38](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L38)
- [bindings.rs:43](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L43)
- [bindings.rs:66](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L66)
- [bindings.rs:101](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L101)
- [bindings.rs:102](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L102)
- [bindings.rs:104](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L104)
- [bindings.rs:111](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L111)
- [bindings.rs:118](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L118)
- [bindings.rs:122](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L122)
- [bindings.rs:126](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L126)
- [bindings.rs:129](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L129)
- [bindings.rs:701](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L701)
- [bindings.rs:702](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L702)
- [bindings.rs:750](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L750)
- [bindings.rs:751](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L751)
- [bindings.rs:862](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L862)
- [bindings.rs:864](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L864)
- [bindings.rs:881](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L881)
</details>

Searched for text `exclude_classes` (`**/crates/opencascade-sys/bindings.toml`), 1 result
- [bindings.toml:101](../crates/opencascade-sys/bindings.toml#L101)

<details>
<summary>Reading all lines of <a href="../crates/opencascade-binding-generator/src/config.rs">config.rs</a></summary>

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

    /// Exclude specific classes/types from binding generation.
    /// These classes will not get any bindings (no struct, no methods, no destructor).
    /// For nested types, use the C++ qualified name: "Parent::Nested".
    #[serde(default)]
    pub exclude_classes: Vec<String>,

    /// Opaque types defined in manual/ files but referenced by auto-generated bindings.
    /// The generator adds these to the known class set so methods using them
    /// aren't skipped as "unknown type".
    /// Format: `TypeName = { header = "Header.hxx" }`
    #[serde(default)]
    pub manual_types: std::collections::HashMap<String, ManualType>,
}

/// A manually-defined opaque type referenced by auto-generated bindings.
#[derive(Debug, Deserialize)]
pub struct ManualType {
    /// The C++ header that defines this type (for wrappers.cpp includes).
    pub header: String,
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
         
... (truncated, 9746 chars)
```
</details>

<details>
<summary>Reading lines 90-160 of <a href="../crates/opencascade-binding-generator/src/main.rs#L90-L161">main.rs</a></summary>

```
        if let Some(ref module) = args.module {
            println!("Filtering to module: {}", module);
        }
    }

    // Determine explicit headers from config file or CLI arguments
    let (explicit_headers, resolve_deps, exclude_set, exclude_modules, exclude_methods, exclude_classes, manual_type_names) = if let Some(ref config_path) = args.config {
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
        let exclude_cls: HashSet<String> = cfg.exclude_classes.into_iter().collect();
        let manual_names: HashSet<String> = cfg.manual_types.keys().cloned().collect();
        (headers, resolve, excludes, exclude_mods, method_exclusions, exclude_cls, manual_names)
    } else if !args.headers.is_empty() {
        (args.headers.clone(), args.resolve_deps, std::collections::HashSet::new(), Vec::new(), HashSet::new(), HashSet::new(), HashSet::new())
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
```
</details>

<details>
<summary>Reading lines 370-400 of <a href="../crates/opencascade-binding-generator/src/main.rs#L370-L401">main.rs</a></summary>

```

    if args.verbose {
        println!("  Found {} known OCCT headers", known_headers.len());
    }

    // Generate FFI output
    generate_output(&args, &all_classes, &all_functions, &graph, &symbol_table, &known_headers, &exclude_methods, &exclude_classes, &handle_able_classes, &manual_type_names)
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
    exclude_methods: &HashSet<(String, String)>,
    verbose: bool,
) {
    for header in parsed.iter_mut() {
        let mut functions_to_add = Vec::new();
        let mut classes_to_remove = Vec::new();

```
</details>

<details>
<summary>Reading lines 585-660 of <a href="../crates/opencascade-binding-generator/src/main.rs#L585-L661">main.rs</a></summary>

```
/// - wrappers.hxx: Single C++ header with all wrappers
/// - MODULE.rs: Per-module re-exports with impl blocks
/// - lib.rs: Module declarations
fn generate_output(
    args: &Args,
    all_classes: &[&model::ParsedClass],
    all_functions: &[&model::ParsedFunction],
    graph: &module_graph::ModuleGraph,
    symbol_table: &resolver::SymbolTable,
    known_headers: &HashSet<String>,
    exclude_methods: &HashSet<(String, String)>,
    exclude_classes: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
    manual_type_names: &HashSet<String>,
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
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods, manual_type_names);

    // Compute FunctionBindings once for ALL free functions — shared by all three generators
    let (all_function_bindings, all_skipped_functions) = codegen::bindings::compute_all_function_bindings(
        symbol_table, all_classes, &collection_type_names, &extra_typedef_names, known_headers, manual_type_names,
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
        exclude_classes,
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
```
</details>

<details>
<summary>Reading lines 25-140 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L25-L141">bindings.rs</a></summary>

```
// ── IR Structs ──────────────────────────────────────────────────────────────

/// Computed binding decisions for a single class.
/// All filtering, naming, and conflict resolution happens here ONCE.
#[derive(Debug, Clone)]
pub struct ClassBindings {
    /// Rust-safe name ("::" flattened to "_" for nested types)
    pub cpp_name: String,
    /// Original C++ qualified name (uses "::" for nested types)
    pub cpp_qualified_name: String,
    pub short_name: String,
    pub module: String,
    pub is_abstract: bool,
    pub has_protected_destructor: bool,
    pub doc_comment: Option<String>,
    pub source_header: String,
    pub source_line: Option<u32>,

    pub constructors: Vec<ConstructorBinding>,
    pub direct_methods: Vec<DirectMethodBinding>,
    pub wrapper_methods: Vec<WrapperMethodBinding>,
    pub static_methods: Vec<StaticMethodBinding>,
    pub upcasts: Vec<UpcastBinding>,
    pub has_to_owned: bool,
    pub has_to_handle: bool,
    /// Whether Handle_get/get_mut should be generated (true for all handle types, including abstract)
    pub has_handle_get: bool,
    pub handle_upcasts: Vec<HandleUpcastBinding>,
    pub handle_downcasts: Vec<HandleDowncastBinding>,
    pub inherited_methods: Vec<InheritedMethodBinding>,
    /// Whether this class is a POD struct that can be represented with real fields
    pub is_pod_struct: bool,
    /// Fields for POD structs (only populated when is_pod_struct is true)
    pub pod_fields: Vec<PodFieldBinding>,
    /// Symbols that were skipped during binding generation, with reasons
    pub skipped_symbols: Vec<SkippedSymbol>,
}

/// A symbol that was skipped during binding generation.
#[derive(Debug, Clone)]
pub struct SkippedSymbol {
    /// Kind of symbol ("constructor", "method", "static_method", "function")
    pub kind: &'static str,
    /// Rust module this symbol belongs to
    pub module: String,
    /// C++ name of the symbol
    pub cpp_name: String,
    /// Source header
    pub source_header: String,
    /// Source line number
    pub source_line: Option<u32>,
    /// Documentation comment from C++ header
    pub doc_comment: Option<String>,
    /// Human-readable reason why the symbol was skipped
    pub skip_reason: String,
    /// Best-guess Rust declaration (commented out in output)
    pub stub_rust_decl: String,
}

/// A single field in a POD struct.
#[derive(Debug, Clone)]
pub struct PodFieldBinding {
    /// Field name in Rust (snake_case)
    pub rust_name: String,
    /// Field name in C++ (original)
    pub cpp_name: String,
    /// Rust type string, e.g. "bool" or "f64"
    pub rust_type: String,
    /// Array size if this is a fixed-size array field
    pub array_size: Option<usize>,
    /// Byte offset for offsetof check
    pub offset_index: usize,
    /// Doc comment
    pub doc_comment: Option<String>,
}

/// A constructor that will have a C++ wrapper (std::make_unique),
/// or a Rust-only convenience wrapper that delegates to a full-argument constructor.
#[derive(Debug, Clone)]
pub struct ConstructorBinding {
    /// FFI function name, e.g. "gp_Pnt_ctor_real3"
    pub ffi_fn_name: String,
    /// Impl method name in re-export, e.g. "new_real3"
    pub impl_method_name: String,
    /// Parameters
    pub params: Vec<ParamBinding>,
    /// C++ argument expressions for calling the constructor
    pub cpp_arg_exprs: Vec<String>,
    /// Doc comment
    pub doc_comment: Option<String>,
    /// Source line in C++ header
    pub source_line: Option<u32>,
    /// If this is a convenience wrapper (fewer params with defaults filled in),
    /// contains info about the full-argument constructor it delegates to.
    /// When set, no ffi.rs or wrappers.hxx entry is generated — only a Rust-only
    /// method in the module re-export that calls the full-argument version.
    pub convenience_of: Option<ConvenienceInfo>,
    /// Whether this constructor should be marked `unsafe fn` (has raw pointer params/returns)
    pub is_unsafe: bool,
}

/// Info for a convenien
... (truncated, 4645 chars)
```
</details>

<details>
<summary>Reading lines 840-950 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L840-L951">bindings.rs</a></summary>

```
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
        if let Some(class_name) = p
... (truncated, 5054 chars)
```
</details>

<details>
<summary>Reading lines 95-140 of <a href="../crates/opencascade-sys/bindings.toml#L95-L141">bindings.toml</a></summary>

```
    # misresolves as an opaque class type.
    "BRepMesh_GeomTool::CellsCount",
]

# Exclude specific classes/types from destructor generation.
# Forward-declared-only nested types cannot be safely deleted.
exclude_classes = [
    "Message_ProgressScope::NullString",  # Forward-declared only, no full definition
    "MeshVS_NodePair",                    # Typedef for std::pair<int,int>, not a real class
]

[general]
resolve_deps = true

# Opaque types defined in manual/ files. These are not parsed from OCCT headers
# but referenced by auto-generated bindings. The generator adds them to the known
# class set so methods using them aren't skipped as "unknown type".
# Format: "CppTypeName" = { header = "Header.hxx" }
[manual_types]
Standard_OStream = { header = "Standard_OStream.hxx" }
Standard_IStream = { header = "Standard_IStream.hxx" }
Standard_SStream = { header = "Standard_SStream.hxx" }

```
</details>

<details>
<summary>Reading lines 1-80 of <a href="../crates/opencascade-binding-generator/src/config.rs#L1-L81">config.rs</a></summary>

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

    /// Exclude specific classes/types from binding generation.
    /// These classes will not get any bindings (no struct, no methods, no destructor).
    /// For nested types, use the C++ qualified name: "Parent::Nested".
    #[serde(default)]
    pub exclude_classes: Vec<String>,

    /// Opaque types defined in manual/ files but referenced by auto-generated bindings.
    /// The generator adds these to the known class set so methods using them
    /// aren't skipped as "unknown type".
    /// Format: `TypeName = { header = "Header.hxx" }`
    #[serde(default)]
    pub manual_types: std::collections::HashMap<String, ManualType>,
}

/// A manually-defined opaque type referenced by auto-generated bindings.
#[derive(Debug, Deserialize)]
pub struct ManualType {
    /// The C++ header that defines this type (for wrappers.cpp includes).
    pub header: String,
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
```
</details>

<details>
<summary>Searched for regex <code>exclude_classes|has_protected_destructor|non.?alloc</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 16 results</summary>

- [bindings.rs:38](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L38)
- [bindings.rs:1524](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1524)
- [bindings.rs:1575](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1575)
- [bindings.rs:1593](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1593)
- [bindings.rs:1886](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1886)
- [bindings.rs:1895](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1895)
- [bindings.rs:1962](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1962)
- [bindings.rs:2599](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2599)
- [bindings.rs:3089](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3089)
- [bindings.rs:3248](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3248)
- [bindings.rs:4614](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4614)
- [bindings.rs:4823](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4823)
- [bindings.rs:5357](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5357)
- [bindings.rs:5559](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5559)
- [bindings.rs:5648](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5648)
</details>

<details>
<summary>Reading lines 1510-1620 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1510-L1621">bindings.rs</a></summary>

```
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
        // Record skipped constructors for abstract/protected-destructor classes
        if effectively_abstract {
            for ctor in &class.constructors {
                skipped_symbols.push(SkippedSymbol {
                    kind: "constructor",
                    module: class.module.clone(),
                    cpp_name: format!("{}::{}", class.name, class.name),
                    source_header: class.source_header.clone(),
                    source_line: ctor.source_line,
                    doc_comment: ctor.comment.clone(),
                    skip_reason: "class is abstract (has unimplemented pure virtual methods)".to_string(),
                    stub_rust_decl: generate_ctor_stub(cpp_name, ctor),
                });
            }
        } else if class.has_protected_destructor {
            for ctor in &class.constructors {
                skipped_symbols.push(SkippedSymbol {
                    kind: "constructor",
                    module: class.module.clone(),
                    cpp_name: format!("{}::{}", class.name, class.name),
                    source_header: class.source_header.clone(),
                    source_line: ctor.source_line,
                    doc_comment: ctor.comment.clone(),
                    skip_reason: "class has protected destructor".to_string(),
                    stub_rust_decl: generate_ctor_stub(cpp_name, ctor),
                });
            }
        }
        Vec::new()
    };

    // Coll
... (truncated, 5498 chars)
```
</details>

Searched for text `exclude_classes` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), 2 results
- [rust.rs:148](../crates/opencascade-binding-generator/src/codegen/rust.rs#L148)
- [rust.rs:153](../crates/opencascade-binding-generator/src/codegen/rust.rs#L153)

<details>
<summary>Reading lines 135-170 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L135-L171">rust.rs</a></summary>

```
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
    exclude_classes: &HashSet<String>,
) -> (String, Vec<NestedTypeInfo>) {
    // Get all classes with protected destructors, and merge in excluded classes
    // (excluded classes should not get destructors/CppDeletable either)
    let mut protected_destructor_class_names = symbol_table.protected_destructor_class_names();
    for cls in exclude_classes {
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
```
</details>

Searched for regex `fn compute_all_class_bindings` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:3075](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3075)

<details>
<summary>Reading lines 3075-3120 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3075-L3121">bindings.rs</a></summary>

```
pub fn compute_all_class_bindings(
    all_classes: &[&ParsedClass],
    symbol_table: &SymbolTable,
    collection_names: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
    exclude_methods: &HashSet<(String, String)>,
    manual_type_names: &HashSet<String>,
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
```
</details>

<details>
<summary>Reading lines 3120-3280 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3120-L3281">bindings.rs</a></summary>

```
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

// ── Free function bindings ──────────────────────────────────────────────────

/// Collect nested type names (Parent::Nested) that should be considered deletable.
/// These get destructors generated via the nested type destructor mechanism.
fn collect_nested_deletable_names(ty: &Type, known_classes: &HashSet<&str>, out: &mut HashSet<String>) {
    match ty {
        Type::Class(name) if name.contains("::") => {
            if let Some(parent) = name.split("::").next() {
                if known_classes.contains(parent) {
                    out.insert(name.clone());
                }
            }
        }
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) |
        Type::ConstPtr(inner) | Type::MutPtr(inner) => {
            collect_nested_deletable_names(inner, known_classes, out);
        }
        _ => {}
    }
}

/// Collect C++ headers needed for a type (for #include directives in wrappers.hxx).
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
                        let header = format!("{}.hxx", parent);
                        if known_headers.is_empty() || known_headers.contains(&header) {
                            headers.insert(header);
                        }
                    }
                }
                return;
            }
            if !name.contains('_') && !name.starts_with("Standard") {
                return;
            }
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
      
... (truncated, 6559 chars)
```
</details>

Searched for text `exclude_classes` (`**/crates/opencascade-binding-generator/src/codegen/cpp.rs`), no results

Searched for regex `fn emit_ffi_class` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:5338](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5338)

<details>
<summary>Reading lines 5338-5380 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5338-L5381">bindings.rs</a></summary>

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
```
</details>

**Todo list:**
- ✅ Understand exclude_classes mechanism
- 🔄 Add no-construct/no-destruct support
- ⬜ Un-exclude the two headers
- ⬜ Regenerate, build, test, commit

Good. The existing mechanisms can handle this without any generator code changes:
- `exclude_classes` suppresses destructors
- The parser should already detect `has_protected_destructor` for these classes, which suppresses constructors

Let me verify: the parser's `check_protected_destructor` should detect these. The issue was that these classes inherit protected `operator new/delete`, which isn't the same as a protected destructor. Let me check what actually happens when these headers are included.

Check protected new/delete details

<details>
<summary>Reading all lines of <a href="../crates/occt-sys/OCCT/src/Message/Message_LazyProgressScope.hxx">Message_LazyProgressScope.hxx</a></summary>

```
// Copyright (c) 2017-2021 OPEN CASCADE SAS
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

#ifndef _Message_LazyProgressScope_HeaderFiler
#define _Message_LazyProgressScope_HeaderFiler

#include <Message_ProgressScope.hxx>

//! Progress scope with lazy updates and abort fetches.
//!
//! Although Message_ProgressIndicator implementation is encouraged to spare GUI updates,
//! even optimized implementation might show a noticeable overhead on a very small update step (e.g.
//! per triangle).
//!
//! The class splits initial (displayed) number of overall steps into larger chunks specified in
//! constructor, so that displayed progress is updated at larger steps.
class Message_LazyProgressScope : protected Message_ProgressScope
{
public:
  //! Main constructor.
  //! @param[in] theRange  progress range to scope
  //! @param[in] theName   name of this scope
  //! @param[in] theMax    number of steps within this scope
  //! @param[in] thePatchStep  number of steps to update progress
  //! @param[in] theIsInf  infinite flag
  Message_LazyProgressScope(const Message_ProgressRange& theRange,
                            const char*                  theName,
                            const Standard_Real          theMax,
                            const Standard_Real          thePatchStep,
                            const Standard_Boolean       theIsInf = Standard_False)
      : Message_ProgressScope(theRange, theName, theMax, theIsInf),
        myPatchStep(thePatchStep),
        myPatchProgress(0.0),
        myIsLazyAborted(Standard_False)
  {
  }

  //! Increment progress with 1.
  void Next()
  {
    if (++myPatchProgress < myPatchStep)
    {
      return;
    }

    myPatchProgress = 0.0;
    Message_ProgressScope::Next(myPatchStep);
    IsAborted();
  }

  //! Return TRUE if progress has been aborted - return the cached state lazily updated.
  Standard_Boolean More() const { return !myIsLazyAborted; }

  //! Return TRUE if progress has been aborted - fetches actual value from the Progress.
  Standard_Boolean IsAborted()
  {
    myIsLazyAborted = myIsLazyAborted || !Message_ProgressScope::More();
    return myIsLazyAborted;
  }

protected:
  Standard_Real    myPatchStep;
  Standard_Real    myPatchProgress;
  Standard_Boolean myIsLazyAborted;
};

#endif // _Message_LazyProgressScope_HeaderFiler

```
</details>

<details>
<summary>Reading all lines of <a href="../crates/occt-sys/OCCT/src/Message/Message_ProgressScope.hxx">Message_ProgressScope.hxx</a></summary>

```
// Created on: 2002-02-22
// Created by: Andrey BETENEV
// Copyright (c) 2002-2014 OPEN CASCADE SAS
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

#ifndef _Message_ProgressScope_HeaderFile
#define _Message_ProgressScope_HeaderFile

#include <Standard_Assert.hxx>
#include <Standard_TypeDef.hxx>
#include <Standard_DefineAlloc.hxx>
#include <Standard_Handle.hxx>
#include <Precision.hxx>
#include <TCollection_AsciiString.hxx>

class Message_ProgressRange;
class Message_ProgressIndicator;

//! Message_ProgressScope class provides convenient way to advance progress
//! indicator in context of complex program organized in hierarchical way,
//! where usually it is difficult (or even not possible) to consider process
//! as linear with fixed step.
//!
//! On every level (sub-operation) in hierarchy of operations
//! the local instance of the Message_ProgressScope class is created.
//! It takes a part of the upper-level scope (via Message_ProgressRange) and provides
//! a way to consider this part as independent scale with locally defined range.
//!
//! The position on the local scale may be advanced using the method Next(),
//! which allows iteration-like advancement. This method can take argument to
//! advance by the specified value (with default step equal to 1).
//! This method returns Message_ProgressRange object that takes responsibility
//! of making the specified step, either directly at its destruction or by
//! delegating this task to another sub-scope created from that range object.
//!
//! It is important that sub-scope must have life time less than
//! the life time of its parent scope that provided the range.
//! The usage pattern is to create scope objects as local variables in the
//! functions that do the job, and pass range objects returned by Next() to
//! the functions of the lower level, to allow them creating their own scopes.
//!
//! The scope has a name that can be used in visualization of the progress.
//! It can be null. Note that when C string literal is used as a name, then its
//! value is not copied, just pointer is stored. In other variants (char pointer
//! or a string class) the string is copied, which is additional overhead.
//!
//! The same instance of the progress scope! must not be used concurrently from different threads.
//! For the algorithm running its tasks in parallel threads, a common scope is
//! created before the parallel execution, and the range objects produced by method
//! Next() are used to initialise the data pertinent to each task.
//! Then the progress is advanced within each task using its own range object.
//! See example below.
//!
//! Note that while a range of the scope is specified using Standard_Real
//! (double) parameter, it is expected to be a positive integer value.
//! If the range is not an integer, method Next() shall be called with
//! explicit step argument, and the rounded value returned by method Value()
//! may be not coherent with the step and range.
//!
//! A scope can be created with option "infinite". This is useful when
//! the number of steps is not known by the time of the scope creation.
//! In this case the progress will be advanced logarithmically, approaching
//! the end of the scope at infinite number of steps. The parameter Max
//! for infinite scope indicates number of steps corresponding to mid-range.
//!
//! A progress scope created with empty constructor is not connected to any
//! progress indicator, and 
... (truncated, 23416 chars)
```
</details>

<details>
<summary>Reading all lines of <a href="../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx">Poly_CoherentTriPtr.hxx</a></summary>

```
// Created on: 2007-12-08
// Created by: Alexander GRIGORIEV
// Copyright (c) 2007-2014 OPEN CASCADE SAS
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

#ifndef Poly_CoherentTriPtr_HeaderFile
#define Poly_CoherentTriPtr_HeaderFile

#include <NCollection_BaseAllocator.hxx>
#include <NCollection_DefineAlloc.hxx>

class Poly_CoherentTriangle;

#ifdef _MSC_VER
  #pragma warning(push)
  #pragma warning(disable : 4355) //'this' : used in base member initializer list
#endif

/**
 * Implementation of both list node for Poly_CoherentTriangle type and
 * round double-linked list of these nodes.
 */

class Poly_CoherentTriPtr
{
public:
  /**
   * Iterator class for this list of triangles. Because the list is round,
   * an iteration can be started from any member and it finishes before taking
   * this member again. The iteration sense is always forward (Next).
   */
  class Iterator
  {
  public:
    //! Empty constructor
    inline Iterator()
        : myFirst(0L),
          myCurrent(0L)
    {
    }

    //! Constructor
    inline Iterator(const Poly_CoherentTriPtr& thePtr)
        : myFirst(&thePtr),
          myCurrent(&thePtr)
    {
    }

    //! Query the triangle that started the current iteration.
    inline const Poly_CoherentTriangle* First() const
    {
      return myFirst ? &myFirst->GetTriangle() : 0L;
    }

    //! Query if there is available triangle pointer on this iteration
    inline Standard_Boolean More() const { return myCurrent != 0L; }

    //! Go to the next iteration.
    Standard_EXPORT void Next();

    //! Get the current iterated triangle
    inline const Poly_CoherentTriangle& Value() const { return myCurrent->GetTriangle(); }

    //! Get the current iterated triangle (mutable)
    inline Poly_CoherentTriangle& ChangeValue() const
    {
      return const_cast<Poly_CoherentTriangle&>(myCurrent->GetTriangle());
    }

    //! Get the current iterated pointer to triangle
    inline const Poly_CoherentTriPtr& PtrValue() const { return *myCurrent; }

  private:
    const Poly_CoherentTriPtr* myFirst;
    const Poly_CoherentTriPtr* myCurrent;
  };

  // ---------- PUBLIC METHODS ----------

  /**
   * Constructor.
   */
  inline Poly_CoherentTriPtr(const Poly_CoherentTriangle& theTri)
      : mypTriangle(&theTri),
        myNext(this),
        myPrevious(this)
  {
  }

  /**
   * Operator new for dynamic allocations
   */
  DEFINE_NCOLLECTION_ALLOC

  /**
   * Query the stored pointer to Triangle.
   */
  inline const Poly_CoherentTriangle& GetTriangle() const { return *mypTriangle; }

  /**
   * Initialize this instance with a pointer to triangle.
   */
  inline void SetTriangle(const Poly_CoherentTriangle* pTri) { mypTriangle = pTri; }

  /**
   * Query the next pointer in the list.
   */
  inline Poly_CoherentTriPtr& Next() const { return *myNext; }

  /**
   * Query the previous pointer in the list.
   */
  inline Poly_CoherentTriPtr& Previous() const { return *myPrevious; }

  /**
   * Append a pointer to triangle into the list after the current instance.
   * @param pTri
   *   Triangle that is to be included in the list after this one.
   * @param theA
   *   Allocator where the new pointer instance is created.
   */
  Standard_EXPORT void Append(const Poly_CoherentTriangle*             pTri,
                              const Handle(NCollection_BaseAllocator)& theA);

  /**
   * Prepend a pointer to triangle into the list before the current instance.
   * @par
... (truncated, 5442 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01AvoVuumSV4idibpQ5HuVct__vscode-1771608612451/content.txt#1-1), lines 1 to 500

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01AvoVuumSV4idibpQ5HuVct__vscode-1771608612451/content.txt#500-500), lines 500 to 618

<details>
<summary>Searched for regex <code>DEFINE_STANDARD_ALLOC|DEFINE_NCOLLECTION_ALLOC</code> (<code>**/Standard_DefineAlloc.hxx</code>), 7 results</summary>

- [Standard_DefineAlloc.hxx:22](../crates/occt-sys/OCCT/src/Standard/Standard_DefineAlloc.hxx#L22)
- [Standard_DefineAlloc.hxx:24](../crates/occt-sys/OCCT/src/Standard/Standard_DefineAlloc.hxx#L24)
- [Standard_DefineAlloc.hxx:39](../crates/occt-sys/OCCT/src/Standard/Standard_DefineAlloc.hxx#L39)
- [Standard_DefineAlloc.hxx:45](../crates/occt-sys/OCCT/src/Standard/Standard_DefineAlloc.hxx#L45)
- [Standard_DefineAlloc.hxx:54](../crates/occt-sys/OCCT/src/Standard/Standard_DefineAlloc.hxx#L54)
- [Standard_DefineAlloc.hxx:63](../crates/occt-sys/OCCT/src/Standard/Standard_DefineAlloc.hxx#L63)
- [Standard_DefineAlloc.hxx:64](../crates/occt-sys/OCCT/src/Standard/Standard_DefineAlloc.hxx#L64)
</details>

Searched for text `DEFINE_NCOLLECTION_ALLOC` (`**/NCollection_DefineAlloc.hxx`), 2 results
- [NCollection_DefineAlloc.hxx:25](../crates/occt-sys/OCCT/src/NCollection/NCollection_DefineAlloc.hxx#L25)
- [NCollection_DefineAlloc.hxx:31](../crates/occt-sys/OCCT/src/NCollection/NCollection_DefineAlloc.hxx#L31)

<details>
<summary>Reading all lines of <a href="../crates/occt-sys/OCCT/src/Standard/Standard_DefineAlloc.hxx">Standard_DefineAlloc.hxx</a></summary>

```
// Created on: 2012-01-19
// Created by: Dmitry BOBYLEV
// Copyright (c) 2012-2014 OPEN CASCADE SAS
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

#ifndef _Standard_DefineAlloc_HeaderFile
#define _Standard_DefineAlloc_HeaderFile

// Macro to override new and delete operators for arrays.
// Defined to empty for old SUN compiler
#if defined(__SUNPRO_CC) && (__SUNPRO_CC <= 0x530)
  #define DEFINE_STANDARD_ALLOC_ARRAY
#else
  #define DEFINE_STANDARD_ALLOC_ARRAY                                                              \
    void* operator new[](size_t theSize)                                                           \
    {                                                                                              \
      return Standard::Allocate(theSize);                                                          \
    }                                                                                              \
    void operator delete[](void* theAddress)                                                       \
    {                                                                                              \
      Standard::Free(theAddress);                                                                  \
    }
#endif

// Macro to override placement new and placement delete operators.
// For Borland C and old SUN compilers do not define placement delete
// as it is not supported.
#if defined(__BORLANDC__) || (defined(__SUNPRO_CC) && (__SUNPRO_CC <= 0x530))
  #define DEFINE_STANDARD_ALLOC_PLACEMENT                                                          \
    void* operator new(size_t, void* theAddress)                                                   \
    {                                                                                              \
      return theAddress;                                                                           \
    }
#else
  #define DEFINE_STANDARD_ALLOC_PLACEMENT                                                          \
    void* operator new(size_t, void* theAddress)                                                   \
    {                                                                                              \
      return theAddress;                                                                           \
    }                                                                                              \
    void operator delete(void*, void*) {}
#endif

// Macro to override operators new and delete to use OCC memory manager
#define DEFINE_STANDARD_ALLOC                                                                      \
  void* operator new(size_t theSize)                                                               \
  {                                                                                                \
    return Standard::Allocate(theSize);                                                            \
  }                                                                                                \
  void operator delete(void* theAddress)                                                           \
  {                                                                                                \
    Standard::Free(theAddress);                                                                    \
  }                                                                                                \
  DEFINE_
... (truncated, 5142 chars)
```
</details>

<details>
<summary>Reading all lines of <a href="../crates/occt-sys/OCCT/src/NCollection/NCollection_DefineAlloc.hxx">NCollection_DefineAlloc.hxx</a></summary>

```
// Created on: 2012-01-19
// Created by: Dmitry BOBYLEV
// Copyright (c) 2012-2014 OPEN CASCADE SAS
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

#ifndef _NCollection_DefineAlloc_HeaderFile
#define _NCollection_DefineAlloc_HeaderFile

#include <NCollection_BaseAllocator.hxx>

// Macro to overload placement new and delete operators for NCollection allocators.
// For Borland C and old SUN compilers do not define placement delete
// as it is not supported.
#if defined(__BORLANDC__) || (defined(__SUNPRO_CC) && (__SUNPRO_CC <= 0x530))
  #define DEFINE_NCOLLECTION_ALLOC                                                                 \
    void* operator new(size_t theSize, const Handle(NCollection_BaseAllocator)& theAllocator)      \
    {                                                                                              \
      return theAllocator->Allocate(theSize);                                                      \
    }
#else
  #define DEFINE_NCOLLECTION_ALLOC                                                                 \
    void* operator new(size_t theSize, const Handle(NCollection_BaseAllocator)& theAllocator)      \
    {                                                                                              \
      return theAllocator->Allocate(theSize);                                                      \
    }                                                                                              \
    void operator delete(void* theAddress, const Handle(NCollection_BaseAllocator)& theAllocator)  \
    {                                                                                              \
      theAllocator->Free(theAddress);                                                              \
    }
#endif

#endif

```
</details>

Searched for regex `Message_LazyProgressScope|Message_ProgressScope|Poly_CoherentTriPtr` (`**/bindings.toml`), 5 results
- [bindings.toml:47](../crates/opencascade-sys/bindings.toml#L47)
- [bindings.toml:48](../crates/opencascade-sys/bindings.toml#L48)
- [bindings.toml:55](../crates/opencascade-sys/bindings.toml#L55)
- [bindings.toml:102](../crates/opencascade-sys/bindings.toml#L102)

> 💭 So the situation is:
> 
> 1. **Message_LazyProgressScope** — Protected inheritance hides operator new/delete. The class is low-value. But let's try to make it work anyway since the user asked.
> 
> 2. **Poly_CoherentTriPtr** — It might actually already work. The `DEFINE_NCOLLECTION_ALLOC` adds placement new/delete, but global new/delete should still be usable. Let me just try un-excluding both and see what happens.
> 
> Actually wait - let me re-read the user's request. They said "Can we make these work? Even without new/delete, it should be possible to access these objects, if we just leave construction and destruction out of the bound API."
> 
> 
> So the strategy is to remove both from the exclusion headers, add them to exclude_classes to suppress destructors, and for Message_LazyProgressScope specifically, exclude the constructors by name so we keep the accessor methods like Next, More, and IsAborted available.
> 
> But I'm realizing exclude_classes only handles destructors, not the struct itself. And the protected destructor detection won't catch Message_LazyProgressScope since its destructor is implicitly public—the real problem is that new and delete are protected, which breaks both the constructor wrapper and the destructor wrapper. For Poly_CoherentTriPtr, I should just try it as-is and see what happens.
> 
> Let me take a simpler approach: remove both from exclude_headers, attempt to regenerate and build, then fix based on whatever actual errors surface. For Message_LazyProgressScope specifically, the protected operator new/delete means the constructor wrapper will fail when trying to allocate, and the destructor wrapper will fail when trying to deallocate. I'm tracing through the code to confirm that adding a class to `exclude_classes` automatically suppresses both constructors and destructors—the FFI generation adds it to `protected_destructor_class_names`, and then the bindings generator uses that flag to suppress constructors as well. Let me verify this connection between the two code paths to make sure my approach will work. For `Message_LazyProgressScope`, the parser won't catch the protected destructor since it's implicit, so `has_protected_destructor` stays false and constructors get generated anyway—I'll need `exclude_methods` for the constructors plus `exclude_classes` for the destructor. With `Poly_CoherentTriPtr`, same issue with the parser missing the protected destructor, though the "No suitable operator delete" comment suggests the placement delete might be incompatible with regular new. Rather than patching this with exclusion rules, I should probably fix it properly in the generator so these classes work without new/delete operations.
> 
> Actually, the existing `exclude_classes` and `exclude_methods` mechanism is clean enough—let me just remove both classes from `exclude_headers`, add `Message_LazyProgressScope` to `exclude_classes`, exclude its constructor from `exclude_methods`, and try `Poly_CoherentTriPtr` without any exclusions first to see what the actual compile errors reveal.

Let me try the simplest approach: un-exclude the headers and see what actually breaks, then fix precisely.

<details>
<summary>Reading lines 44-50 of <a href="../crates/opencascade-sys/bindings.toml#L44-L51">bindings.toml</a></summary>

```
# Exclude specific headers (even if their module is in `modules` above).
exclude_headers = [
    # Protected operator new/delete: class cannot be heap-allocated via std::make_unique.
    "Message_LazyProgressScope.hxx",  # Inherits protected new/delete from Message_ProgressScope
    "Poly_CoherentTriPtr.hxx",         # No suitable operator delete

    # Internal-only class: all constructors/destructors are private, accessible only
```
</details>

<details>
<summary>**Failing: <a href="../crates/opencascade-sys/bindings.toml#L44-L47">bindings.toml -1+3</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "# Exclude specific headers (even if their module is in `modules` above).\nexclude_headers = [\n",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "44:nz\n45:ev\n46:ve\n47:kg\n48:tg\n49:dd"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":44,"endLine":51}
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1 | grep -E "(complete|error)"
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-sys 2>&1
```
<details>
<summary>Output (276 lines) — exit code 101</summary>

```
    Blocking waiting for file lock on build directory
   Compiling opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:427:
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-d408feb5876848ab/out/../../../../OCCT/build/include/BRepClass3d_BndBoxTree.hxx:1:
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/BRepClass3d/BRepClass3d_BndBoxTree.hxx:19:
warning: opencascade-sys@0.2.0: In file included from /Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-d408feb5876848ab/out/../../../../OCCT/build/include/NCollection_UBTreeFiller.hxx:1:
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/occt-sys/OCCT/src/NCollection/NCollection_UBTreeFiller.hxx:198:3: warning: 'sprintf' is deprecated: This function is provided for compatibility reasons only.  Due to security concerns inherent in the design of sprintf(3), it is highly recommended that you use snprintf(3) instead. [-Wdeprecated-declarations]
warning: opencascade-sys@0.2.0:   198 |   sprintf(buf, "Checking UBTree:%8d leaves, balance =%7.2f", aNumber, aLen1 / aLen0);
warning: opencascade-sys@0.2.0:       |   ^
warning: opencascade-sys@0.2.0: /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:278:1: note: 'sprintf' has been explicitly marked deprecated here
warning: opencascade-sys@0.2.0:   278 | __deprecated_msg("This function is provided for compatibility reasons only.  Due to security concerns inherent in the design of sprintf(3), it is highly recommended that you use snprintf(3) instead.")
warning: opencascade-sys@0.2.0:       | ^
warning: opencascade-sys@0.2.0: /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/cdefs.h:227:48: note: expanded from macro '__deprecated_msg'
warning: opencascade-sys@0.2.0:   227 |         #define __deprecated_msg(_msg) __attribute__((__deprecated__(_msg)))
warning: opencascade-sys@0.2.0:       |                                                       ^
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp:237666:12: error: 'operator new' is a protected member of 'Message_ProgressScope'
warning: opencascade-sys@0.2.0:  237666 |     return new Message_LazyProgressScope(*theRange, theName, theMax, thePatchStep, theIsInf);
warning: opencascade-sys@0.2.0:         |            ^
warning: opencascade-sys@0.2.0: /Users/sethml/src/opencascade-rs/crates/o
... (truncated, 22705 chars total)
```
</details>

**Terminal:**
```sh
grep "error:" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01Nq626nyS3TUB1rKV3qqKTH__vscode-1771608612463/content.txt | grep -v "cargo:warning" | head -20
```
<details>
<summary>Output (5 lines)</summary>

```
ade-sys/generated/wrappers.cpp:237666:12: error: 'operator new' is a protected member of 'Message_ProgressScope'                                                warning: opencascade-sys@0.2.0:  237666 |     return new Message_LazyProgressSco
ade-sys/generated/wrappers.cpp:237677:90: error: 'operator delete' is a protected member of 'Message_ProgressScope'                                             warning: opencascade-sys@0.2.0:  237677 | extern "C" void Message_LazyProgressSc
ade-sys/generated/wrappers.cpp:246670:12: error: no matching function for call to 'operator new'                                                                warning: opencascade-sys@0.2.0:  246670 |     return new Poly_CoherentTriPtr(*th
ade-sys/generated/wrappers.cpp:246696:78: error: no suitable member 'operator delete' in 'Poly_CoherentTriPtr'                                                  warning: opencascade-sys@0.2.0:  246696 | extern "C" void Poly_CoherentTriPtr_de
error: failed to run custom build command for `opencascade-sys v0.2.0 (/Users/se
```
</details>

Exactly as expected — 4 errors:
1. `Message_LazyProgressScope` constructor: protected `operator new`
2. `Message_LazyProgressScope` destructor: protected `operator delete`
3. `Poly_CoherentTriPtr` constructor: no matching `operator new`
4. `Poly_CoherentTriPtr` destructor: no suitable `operator delete`

Both need constructors and destructors suppressed. I'll use `exclude_classes` (suppresses destructors) + `exclude_methods` (suppresses constructors):

<details>
<summary>Reading lines 82-108 of <a href="../crates/opencascade-sys/bindings.toml#L82-L109">bindings.toml</a></summary>

```
exclude_methods = [
    # C++ `New()` factory method name-collides with constructor wrapper `fn new()`
    "Storage_CallBack::New",
    "Storage_DefaultCallBack::New",

    # `const char**` (double pointer) mapped as `&mut &str` but FFI needs raw pointer
    "XmlObjMgt::GetInteger",
    "XmlObjMgt::GetReal",

    # Returns MeshVS_NodePair (typedef for std::pair<int,int>) which the generator
    # misresolves as an opaque class type.
    "BRepMesh_GeomTool::CellsCount",
]

# Exclude specific classes/types from destructor generation.
# Forward-declared-only nested types cannot be safely deleted.
exclude_classes = [
    "Message_ProgressScope::NullString",  # Forward-declared only, no full definition
    "MeshVS_NodePair",                    # Typedef for std::pair<int,int>, not a real class
]

[general]
resolve_deps = true

# Opaque types defined in manual/ files. These are not parsed from OCCT headers
# but referenced by auto-generated bindings. The generator adds them to the known
# class set so methods using them aren't skipped as "unknown type".
```
</details>

<details>
<summary>**Failing: <a href="../crates/opencascade-sys/bindings.toml#L91-L99">bindings.toml -1+8</a>, <a href="../crates/opencascade-sys/bindings.toml#L103-L112">-1+9</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    # Returns MeshVS_NodePair (typedef for std::pair<int,int>) which the generator\n    # misresolves as an opaque class type.\n    \"BRepMesh_GeomTool::CellsCount\",\n\n    # Protected/hidden operator new — skip constructors but keep methods accessible.\n    \"Message_LazyProgressScope::Message_LazyProgressScope\",\n    \"Poly_CoherentTriPtr::Poly_CoherentTriPtr\",\n]",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "91:hb\n92:ai\n93:ks\n94:cy"
    },
    {
      "content": "# Exclude specific classes/types from destructor generation.\nexclude_classes = [\n    \"Message_ProgressScope::NullString\",  # Forward-declared only, no full definition\n    \"MeshVS_NodePair\",                    # Typedef for std::pair<int,int>, not a real class\n\n    # Protected/hidden operator delete — skip destructors but keep methods accessible.\n    \"Message_LazyProgressScope\",\n    \"Poly_CoherentTriPtr\",\n]",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "96:tk\n97:go\n98:yu\n99:ae\n100:sl\n101:cy"
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
      "file": "crates/opencascade-sys/bindings.toml",
      "startLine": 91,
      "endLine": 116
    }
  ]
}
```
</details>

**Todo list:**
- ✅ Understand exclude_classes mechanism
- ✅ Add no-construct/no-destruct support
- ✅ Un-exclude the two headers
- 🔄 Regenerate, build, test, commit

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1 | grep -E "(complete|error)"
```
```
  Formatting complete.
Code generation complete!
real  44.6s
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

**Terminal:**
```sh
grep -E "(^error|Finished|warning:.*generated [0-9])" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01QUoKKdcY3dZSn8PPtUPdx3__vscode-1771608612469/content.txt
```
**Exit code:** 1

**Terminal:**
```sh
grep "Finished\|error" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01QUoKKdcY3dZSn8PPtUPdx3__vscode-1771608612469/content.txt
```
<details>
<summary>Output (8 lines)</summary>

```
ade-sys/generated/wrappers.cpp:237674:90: error: 'operator delete' is a protected member of 'Message_ProgressScope'                                             warning: opencascade-sys@0.2.0:  237674 | extern "C" void Message_LazyProgressSc
ade-sys/generated/wrappers.cpp:246690:78: error: no suitable member 'operator delete' in 'Poly_CoherentTriPtr'                                                  warning: opencascade-sys@0.2.0:  246690 | extern "C" void Poly_CoherentTriPtr_de
warning: opencascade-sys@0.2.0: 1 warning and 2 errors generated.
lly (status code exit status: 1): env -u IPHONEOS_DEPLOYMENT_TARGET LC_ALL="C" "sccache" "c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm64-apple-macosx" "-mmacosx-version-min=26.2" "-I" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/../../../../OCCT/build/include" "-I" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated" "-Wall" "-Wextra" "-std=c++14" "-Wno-unused-function" "-Wno-deprecated-declarations" "-Wno-return-type-c-linkage" "-Wno-delete-non-abstract-non-virtual-dtor" "-Wno-delete-abstract-non-virtual-dtor" "-D_USE_MATH_DEFINES=TRUE" "-o" "/Users/sethml/src/opencascade-rs/target/debug/build/opencascade-sys-3f85feb8bc2b51d0/out/74271c7d28a70e01-wrappers.o" "-c" "/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp"                                       error: failed to run custom build command for `opencascade-sys v0.2.0 (/Users/se
d/wrappers.cpp:237674:90: error: 'operator delete' is a protected member of 'Message_ProgressScope'                                                               cargo:warning= 237674 | extern "C" void Message_LazyProgressScope_destructor(M
d/wrappers.cpp:246690:78: error: no suitable member 'operator delete' in 'Poly_CoherentTriPtr'                                                                    cargo:warning= 246690 | extern "C" void Poly_CoherentTriPtr_destructor(Poly_Co
  cargo:warning=1 warning and 2 errors generated.
  error occurred in cc-rs: command did not execute successfully (status code exi
```
</details>

Constructors are gone but destructors still being generated. The `exclude_classes` mechanism works by merging into `protected_destructor_class_names` in `rust.rs` (for ffi.rs generation), but the destructor is being emitted in `wrappers.cpp`. Let me check how wrappers.cpp destructors are generated:

Find destructor generation in wrappers

<details>
<summary>Searched for text <code>destructor</code> (<code>**/crates/opencascade-binding-generator/src/codegen/rust.rs</code>), 20 results</summary>

- [rust.rs:39](../crates/opencascade-binding-generator/src/codegen/rust.rs#L39)
- [rust.rs:150](../crates/opencascade-binding-generator/src/codegen/rust.rs#L150)
- [rust.rs:151](../crates/opencascade-binding-generator/src/codegen/rust.rs#L151)
- [rust.rs:152](../crates/opencascade-binding-generator/src/codegen/rust.rs#L152)
- [rust.rs:154](../crates/opencascade-binding-generator/src/codegen/rust.rs#L154)
- [rust.rs:185](../crates/opencascade-binding-generator/src/codegen/rust.rs#L185)
- [rust.rs:190](../crates/opencascade-binding-generator/src/codegen/rust.rs#L190)
- [rust.rs:191](../crates/opencascade-binding-generator/src/codegen/rust.rs#L191)
- [rust.rs:197](../crates/opencascade-binding-generator/src/codegen/rust.rs#L197)
- [rust.rs:201](../crates/opencascade-binding-generator/src/codegen/rust.rs#L201)
- [rust.rs:216](../crates/opencascade-binding-generator/src/codegen/rust.rs#L216)
- [rust.rs:339](../crates/opencascade-binding-generator/src/codegen/rust.rs#L339)
- [rust.rs:340](../crates/opencascade-binding-generator/src/codegen/rust.rs#L340)
- [rust.rs:371](../crates/opencascade-binding-generator/src/codegen/rust.rs#L371)
- [rust.rs:372](../crates/opencascade-binding-generator/src/codegen/rust.rs#L372)
- [rust.rs:444](../crates/opencascade-binding-generator/src/codegen/rust.rs#L444)
- [rust.rs:454](../crates/opencascade-binding-generator/src/codegen/rust.rs#L454)
- [rust.rs:462](../crates/opencascade-binding-generator/src/codegen/rust.rs#L462)
- [rust.rs:472](../crates/opencascade-binding-generator/src/codegen/rust.rs#L472)
</details>

<details>
<summary>Searched for text <code>destructor</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 20 results</summary>

- [bindings.rs:38](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L38)
- [bindings.rs:862](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L862)
- [bindings.rs:864](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L864)
- [bindings.rs:1524](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1524)
- [bindings.rs:1561](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1561)
- [bindings.rs:1575](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1575)
- [bindings.rs:1584](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1584)
- [bindings.rs:1593](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1593)
- [bindings.rs:1886](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1886)
- [bindings.rs:1890](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1890)
- [bindings.rs:1893](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1893)
- [bindings.rs:1895](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1895)
- [bindings.rs:1898](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1898)
- [bindings.rs:1962](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1962)
- [bindings.rs:2599](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2599)
- [bindings.rs:2706](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2706)
- [bindings.rs:3084](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3084)
- [bindings.rs:3086](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3086)
- [bindings.rs:3089](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3089)
</details>

<details>
<summary>Reading lines 140-230 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L140-L231">rust.rs</a></summary>

```
    all_classes: &[&ParsedClass],
    all_headers: &[String],
    collections: &[super::collections::CollectionInfo],
    symbol_table: &crate::resolver::SymbolTable,
    all_bindings: &[super::bindings::ClassBindings],
    function_bindings: &[super::bindings::FunctionBinding],
    handle_able_classes: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
    exclude_classes: &HashSet<String>,
) -> (String, Vec<NestedTypeInfo>) {
    // Get all classes with protected destructors, and merge in excluded classes
    // (excluded classes should not get destructors/CppDeletable either)
    let mut protected_destructor_class_names = symbol_table.protected_destructor_class_names();
    for cls in exclude_classes {
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
```
</details>

<details>
<summary>Reading lines 430-500 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L430-L501">rust.rs</a></summary>

```
    // can be generated even when TColgp_HArray1OfPnt.hxx is excluded.
    for name in extra_handle_able {
        handles.insert(name.clone());
    }

    let mut out = String::new();
    for class_name in &handles {
        let handle_type_name = type_mapping::handle_type_name(class_name);
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
            let handle_type_name = type_mapping::handle_type_name(class_name);
            writeln!(out, "    pub fn {}_destructor(ptr: *mut {});", handle_type_name, handle_type_name).unwrap();
        }
        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();
        for class_name in &extra_handles {
            let handle_type_name = type_mapping::handle_type_name(class_name);
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
```
</details>

Searched for regex `destructor.*wrapper|wrapper.*destructor|wrappers\.cpp.*destructor|destructor.*cpp` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 3 results
- [bindings.rs:2706](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2706)
- [bindings.rs:3545](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3545)
- [bindings.rs:4613](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4613)

<details>
<summary>Reading lines 4600-4660 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4600-L4661">bindings.rs</a></summary>

```
            } else {
                writeln!(
                    output,
                    "    self->{method}({args_str});",
                    method = im.cpp_method_name
                )
                .unwrap();
            }
        }

        writeln!(output, "}}").unwrap();
    }

    // 11. Destructor wrapper
    if !bindings.has_protected_destructor {
        writeln!(
            output,
            "extern \"C\" void {ffi_cn}_destructor({cn}* self_) {{ delete self_; }}"
        )
        .unwrap();
    }

    writeln!(output).unwrap();

    output
}

/// Emit a per-module re-export for a single class from pre-computed ClassBindings.
///
/// Produces the `pub use crate::ffi::X as ShortName;` line and the `impl ShortName { ... }`
/// block with constructor, wrapper, static, upcast, to_owned, and to_handle methods.
/// Convert a param argument for FFI call: add `.into()` if it's a value enum.
fn convert_arg(p: &ParamBinding) -> String {
    if p.is_nullable_ptr {
        if p.rust_ffi_type.starts_with("*const") {
            format!("{}.map_or(std::ptr::null(), |r| r as *const _)", p.rust_name)
        } else {
            format!("{}.map_or(std::ptr::null_mut(), |r| r as *mut _)", p.rust_name)
        }
    } else if p.is_class_ptr {
        if p.rust_ffi_type.starts_with("*const") {
            format!("{} as *const _", p.rust_name)
        } else {
            format!("{} as *mut _", p.rust_name)
        }
    } else if p.mut_ref_enum_rust_type.is_some() {
        format!("&mut {}_i32_", p.rust_name)
    } else if p.rust_reexport_type == "&str" {
        format!("c_{}.as_ptr()", p.rust_name)
    } else if p.enum_rust_type.is_some() {
        format!("{}.into()", p.rust_name)
    } else {
        p.rust_name.clone()
    }
}

fn convert_arg_resolved(name: &str, p: &ResolvedParamBinding) -> String {
    if p.is_nullable_ptr {
        if p.rust_ffi_type.starts_with("*const") {
            format!("{}.map_or(std::ptr::null(), |r| r as *const _)", name)
        } else {
```
</details>

<details>
<summary>Reading lines 2590-2630 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2590-L2631">bindings.rs</a></summary>

```
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
        if let Some(ancestor_class) = symbol_table.class_by_name(ancestor_name) {
            let ancestor_methods = symbol_table.included_methods(ancestor_class);

            // Collect public method names for this ancestor.
            let ancestor_public_names: HashSet<&str> =
                ancestor_methods.iter().map(|m| m.cpp_name.as_str()).collect();
            // Any method declared by this ancestor (in all_method_names) that is
            // NOT publicly exposed has been narrowed (protected/private override).
            // Block it from being inherited from further-up ancestors.
            for method_name in &ancestor_class.all_method_names {
                if !ancestor_public_names.contains(method_name.as_str()) {
                    protected_in_ancestors.insert(method_name.clone());
                }
            }

            for resolved_method in ancestor_methods {
```
</details>

<details>
<summary>Reading lines 3530-3570 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3530-L3571">bindings.rs</a></summary>

```

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
```
</details>

Searched for regex `emit_cpp_class` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:3536](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3536)

Searched for text `emit_cpp_class` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), no results

<details>
<summary>Searched for text <code>emit_cpp_class</code>, 20 results</summary>

- [2026-02-19_21-30-vscode-claude-opus-4.6.md:1181](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L1181)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:1183](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L1183)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:1288](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L1288)
- [2026-02-16_13-00-vscode-claude-opus-4.6.md:76](../agent-logs/2026-02-16_13-00-vscode-claude-opus-4.6.md#L76)
- [2026-02-19_14-15-vscode-claude-opus-4.6.md:2454](../agent-logs/2026-02-19_14-15-vscode-claude-opus-4.6.md#L2454)
- [2026-02-19_14-15-vscode-claude-opus-4.6.md:2456](../agent-logs/2026-02-19_14-15-vscode-claude-opus-4.6.md#L2456)
- [2026-02-18-07-30-vscode-claude-opus-4.6.md:990](../agent-logs/2026-02-18-07-30-vscode-claude-opus-4.6.md#L990)
- [2026-02-15_07-35-vscode-claude-opus-4.6.md:202](../agent-logs/2026-02-15_07-35-vscode-claude-opus-4.6.md#L202)
- [2026-02-15_07-35-vscode-claude-opus-4.6.md:793](../agent-logs/2026-02-15_07-35-vscode-claude-opus-4.6.md#L793)
- [2026-02-17_16-04-vscode-claude-opus-4.6.md:435](../agent-logs/2026-02-17_16-04-vscode-claude-opus-4.6.md#L435)
- [2026-02-17_21-03-vscode-claude-opus-4.6.md:482](../agent-logs/2026-02-17_21-03-vscode-claude-opus-4.6.md#L482)
- [2026-02-17_21-03-vscode-claude-opus-4.6.md:1111](../agent-logs/2026-02-17_21-03-vscode-claude-opus-4.6.md#L1111)
- [2026-02-17_21-03-vscode-claude-opus-4.6.md:1116](../agent-logs/2026-02-17_21-03-vscode-claude-opus-4.6.md#L1116)
- [2026-02-17_21-03-vscode-claude-opus-4.6.md:1129](../agent-logs/2026-02-17_21-03-vscode-claude-opus-4.6.md#L1129)
- [2026-02-17_21-03-vscode-claude-opus-4.6.md:1131](../agent-logs/2026-02-17_21-03-vscode-claude-opus-4.6.md#L1131)
- [2026-02-17_21-03-vscode-claude-opus-4.6.md:1133](../agent-logs/2026-02-17_21-03-vscode-claude-opus-4.6.md#L1133)
- [2026-02-17_21-03-vscode-claude-opus-4.6.md:1148](../agent-logs/2026-02-17_21-03-vscode-claude-opus-4.6.md#L1148)
- [2026-02-17_21-03-vscode-claude-opus-4.6.md:1296](../agent-logs/2026-02-17_21-03-vscode-claude-opus-4.6.md#L1296)
</details>

Searched for text `emit_cpp_class` (`**/crates/opencascade-binding-generator/src/codegen/cpp.rs`), 1 result
- [cpp.rs:298](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L298)

<details>
<summary>Reading lines 275-320 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L275-L321">cpp.rs</a></summary>

```
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
                ffi = nt.ffi_name,
                cpp = nt.cpp_name
            )
            .unwrap();
        }
    }

    // Generate collection wrappers
    if !collections.is_empty() {
```
</details>

<details>
<summary>Reading lines 1-300 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L1-L301">cpp.rs</a></summary>

```
//! C++ wrapper code generation
//!
//! Generates extern "C" wrapper functions for all OCCT methods:
//! - Constructors (using placement new or heap allocation)
//! - Return-by-value methods
//! - Static methods
//! - Overloaded methods
//!
//! All wrapper functions use extern "C" linkage for direct FFI access.

use crate::model::{ParsedClass, Type};
use crate::resolver::SymbolTable;
use std::collections::HashSet;
use std::fmt::Write;

fn collect_handle_types(classes: &[&ParsedClass], handle_able_classes: &HashSet<String>) -> Vec<(String, String)> {
    let mut handles = HashSet::new();

    for class in classes {
        // Add Handle type for classes that are transient (can be wrapped in Handle)
        // Handle types with protected destructors are included because Handle<T>
        // manages lifetime via reference counting, not direct delete.
        if handle_able_classes.contains(&class.name) {
            handles.insert(class.name.clone());
        }

        for method in &class.methods {
            collect_type_handles(&method.return_type, &mut handles);
            for param in &method.params {
                collect_type_handles(&Some(param.ty.clone()), &mut handles);
            }
        }

        for method in &class.static_methods {
            collect_type_handles(&method.return_type, &mut handles);
            for param in &method.params {
                collect_type_handles(&Some(param.ty.clone()), &mut handles);
            }
        }

        for ctor in &class.constructors {
            for param in &ctor.params {
                collect_type_handles(&Some(param.ty.clone()), &mut handles);
            }
        }
    }

    let mut result: Vec<_> = handles
        .into_iter()
        .filter(|inner_class| {
            // Skip pointer/reference types leaked into names, and template types
            // whose instantiated names aren't valid C++ identifiers
            !inner_class.contains('*') && !inner_class.contains('&') && !inner_class.contains('<')
        })
        .map(|inner_class| {
            // Use full class name to match Rust side (e.g., HandleGeom2dCurve not HandleCurve)
            let handle_name = crate::type_mapping::handle_type_name(&inner_class);
            (inner_class, handle_name)
        })
        .collect();
    result.sort();
    result
}


/// Collect Handle type inner classes from a type
fn collect_type_handles(ty: &Option<Type>, handles: &mut HashSet<String>) {
    if let Some(ty) = ty {
        match ty {
            Type::Handle(name) => {
                handles.insert(name.clone());
            }
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                collect_type_handles(&Some(inner.as_ref().clone()), handles);
            }
            _ => {}
        }
    }
}

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
                        // Nested type — include the parent's head
... (truncated, 11725 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/2bb52045-2c8f-416a-a5d2-4bf9f6caee20/toolu_01DFPK7LwdvjdkQSvHkSknfN__vscode-1771608612486/content.txt)

<details>
<summary>Searched for regex <code>all_bindings.*=.*compute|compute_class_bindings</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 5 results</summary>

- [bindings.rs:1502](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1502)
- [bindings.rs:3154](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3154)
- [bindings.rs:5545](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5545)
- [bindings.rs:5606](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5606)
- [bindings.rs:5696](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L5696)
</details>

<details>
<summary>Reading lines 3070-3170 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3070-L3171">bindings.rs</a></summary>

```
// ── Top-level function ──────────────────────────────────────────────────────

/// Compute all binding decisions for every class.
///
/// This is called once and the result is shared by all three output generators.
pub fn compute_all_class_bindings(
    all_classes: &[&ParsedClass],
    symbol_table: &SymbolTable,
    collection_names: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
    exclude_methods: &HashSet<(String, String)>,
    manual_type_names: &HashSet<String>,
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

// ── Free function bindings ──────────────────────────────────────────────────

/// Collect nested type names (Parent::Nested) that should be considered deleta
... (truncated, 4446 chars)
```
</details>

Searched for regex `compute_all_class_bindings\(` (`**/crates/opencascade-binding-generator/src/**`), 2 results
- [main.rs:622](../crates/opencascade-binding-generator/src/main.rs#L622)
- [bindings.rs:3075](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3075)

<details>
<summary>Reading lines 600-660 of <a href="../crates/opencascade-binding-generator/src/main.rs#L600-L661">main.rs</a></summary>

```
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
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods, manual_type_names);

    // Compute FunctionBindings once for ALL free functions — shared by all three generators
    let (all_function_bindings, all_skipped_functions) = codegen::bindings::compute_all_function_bindings(
        symbol_table, all_classes, &collection_type_names, &extra_typedef_names, known_headers, manual_type_names,
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
        exclude_classes,
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
```
</details>

<details>
<summary>Reading lines 660-670 of <a href="../crates/opencascade-binding-generator/src/main.rs#L660-L671">main.rs</a></summary>

```
        &nested_types,
        &handle_able_classes,
    );
    let cpp_path = args.output.join("wrappers.cpp");
    std::fs::write(&cpp_path, &cpp_code)?;
    println!("  Wrote: {}", cpp_path.display());

    // 3. Generate per-module re-export files
    println!("Generating module re-exports...");

    // Index bindings by module for quick lookup
```
</details>

> 💭 So the issue is that `exclude_classes` only suppresses the Rust FFI declaration, but the C++ wrappers.cpp still has the destructor function which uses `delete` and fails to compile. The `has_protected_destructor` field comes from parsing, not from `exclude_classes`.
> 
> I need to propagate `exclude_classes` into the C++ side too. The cleanest fix is to pass `exclude_classes` into `compute_class_bindings()` (or its caller) so that `has_protected_destructor` gets set for those classes.
> 
> Actually, the simplest fix is in rust.rs or wherever the `all_bindings` are computed. 
> 
> Looking at the pipeline, I can see that `all_classes` get parsed, then `all_bindings` are computed from them, and then the C++ wrapper generation checks `has_protected_destructor` when emitting destructors. The Rust side already merges `exclude_classes` into the protected destructor handling. So I need to mark those excluded classes as having protected destructors during the binding computation step to prevent the C++ side from trying to generate delete calls for them.

The issue is that `exclude_classes` suppresses destructors on the Rust side but NOT in the C++ wrappers.cpp. I need to fix the generator to propagate `exclude_classes` to the C++ destructor emission too.

Searched for regex `all_bindings.*compute\|compute.*all_bindings\|exclude_cls` (`**/crates/opencascade-binding-generator/src/main.rs`), no results

Searched for text `exclude_cls` (`**/crates/opencascade-binding-generator/src/main.rs`), 2 results
- [main.rs:139](../crates/opencascade-binding-generator/src/main.rs#L139)
- [main.rs:141](../crates/opencascade-binding-generator/src/main.rs#L141)

Searched for text `generate_output` (`**/crates/opencascade-binding-generator/src/main.rs`), 2 results
- [main.rs:376](../crates/opencascade-binding-generator/src/main.rs#L376)
- [main.rs:588](../crates/opencascade-binding-generator/src/main.rs#L588)

<details>
<summary>Reading lines 588-650 of <a href="../crates/opencascade-binding-generator/src/main.rs#L588-L651">main.rs</a></summary>

```
fn generate_output(
    args: &Args,
    all_classes: &[&model::ParsedClass],
    all_functions: &[&model::ParsedFunction],
    graph: &module_graph::ModuleGraph,
    symbol_table: &resolver::SymbolTable,
    known_headers: &HashSet<String>,
    exclude_methods: &HashSet<(String, String)>,
    exclude_classes: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
    manual_type_names: &HashSet<String>,
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
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods, manual_type_names);

    // Compute FunctionBindings once for ALL free functions — shared by all three generators
    let (all_function_bindings, all_skipped_functions) = codegen::bindings::compute_all_function_bindings(
        symbol_table, all_classes, &collection_type_names, &extra_typedef_names, known_headers, manual_type_names,
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
        exclude_classes,
    );
    let ffi_path = args.output.join("ffi.rs");
    std::fs::write(&ffi_path, ffi_code)?;
    generated_rs_files.push(ffi_path.clone());
    println!("  Wrote: {} ({} classes, {} functions)",
        ffi_path.display(), all_classes.len(), all_functions.len());

```
</details>

<details>
<summary>Reading lines 650-680 of <a href="../crates/opencascade-binding-generator/src/main.rs#L650-L681">main.rs</a></summary>

```

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
```
</details>

I see that `generate_wrappers` doesn't receive `exclude_classes`. I need to pass it through. Let me look at `generate_wrappers` signature and where it checks `has_protected_destructor`:

Searched for text `generate_wrappers` (`**/crates/opencascade-binding-generator/src/codegen/cpp.rs`), 1 result
- [cpp.rs:241](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L241)

<details>
<summary>Reading lines 241-325 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L241-L326">cpp.rs</a></summary>

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
                ffi = nt.ffi_name,
                cpp = nt.cpp_name
            )
            .unwrap();
        }
    }

    // Generate collection wrappers
    if !collections.is_empty() {
        output.push_str(&super::collections::generate_cpp_collections(collections));
    }

    output
}
```
</details>

Line 297-298: it iterates `all_bindings` and calls `emit_cpp_class(bindings)`. Destructors are generated inside `emit_cpp_class` (in bindings.rs) which checks `bindings.has_protected_destructor`. 

The simplest fix: after computing `all_bindings` in main.rs, set `has_protected_destructor = true` for any class in `exclude_classes`. This fixes both the C++ wrappers.cpp side and ensures consistency.

<details>
<summary>Reading lines 617-625 of <a href="../crates/opencascade-binding-generator/src/main.rs#L617-L626">main.rs</a></summary>

```
    // Compute ClassBindings once for ALL classes — shared by all three generators
    let collection_type_names: std::collections::HashSet<String> =
        all_collections.iter().map(|c| c.typedef_name.clone()).collect();
    let extra_typedef_names = parser::get_collected_typedef_names();
    let all_bindings =
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods, manual_type_names);

    // Compute FunctionBindings once for ALL free functions — shared by all three generators
    let (all_function_bindings, all_skipped_functions) = codegen::bindings::compute_all_function_bindings(
```
</details>

<details>
<summary>**Failing: <a href="../crates/opencascade-binding-generator/src/main.rs#L621-L631">main.rs -1+10</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    let mut all_bindings =\n        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods, manual_type_names);\n\n    // Mark exclude_classes as having protected destructors so both the C++ wrappers\n    // (which check has_protected_destructor) and the Rust FFI side skip new/delete.\n    for b in &mut all_bindings {\n        if exclude_classes.contains(&b.class_name) {\n            b.has_protected_destructor = true;\n        }\n    }",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "621:fy\n622:su"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/main.rs","startLine":621,"endLine":631}
```
</details>

Now let me check that `ClassBindings.has_protected_destructor` is a public field:

Searched for text `pub has_protected_destructor` (`**/crates/opencascade-binding-generator/src/codegen/bindings.rs`), 1 result
- [bindings.rs:38](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L38)

Good — it's public. Now let me also verify the generator compiles:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo check -p opencascade-binding-generator 2>&1
```
<details>
<summary>Output (12 lines) — exit code 101</summary>

```
    Blocking waiting for file lock on build directory
    Checking opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
error[E0609]: no field `class_name` on type `&mut ClassBindings`
   --> crates/opencascade-binding-generator/src/main.rs:627:40
    |
627 |         if exclude_classes.contains(&b.class_name) {
    |                                        ^^^^^^^^^^ unknown field
    |
    = note: available fields are: `cpp_name`, `cpp_qualified_name`, `short_name`, `module`, `is_abstract` ... and 18 others

For more information about this error, try `rustc --explain E0609`.
error: could not compile `opencascade-binding-generator` (bin "occt-bindgen") due to 1 previous error
```
</details>

Need to use the correct field name. It's `cpp_name`:

<details>
<summary>Reading lines 20-42 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L20-L43">bindings.rs</a></summary>

```
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
    "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final",
    "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

// ── IR Structs ──────────────────────────────────────────────────────────────

/// Computed binding decisions for a single class.
/// All filtering, naming, and conflict resolution happens here ONCE.
#[derive(Debug, Clone)]
pub struct ClassBindings {
    /// Rust-safe name ("::" flattened to "_" for nested types)
    pub cpp_name: String,
    /// Original C++ qualified name (uses "::" for nested types)
    pub cpp_qualified_name: String,
    pub short_name: String,
    pub module: String,
    pub is_abstract: bool,
    pub has_protected_destructor: bool,
    pub doc_comment: Option<String>,
    pub source_header: String,
    pub source_line: Option<u32>,

```
</details>

<details>
<summary>Reading lines 621-632 of <a href="../crates/opencascade-binding-generator/src/main.rs#L621-L633">main.rs</a></summary>

```
    let mut all_bindings =
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods, manual_type_names);

    // Mark exclude_classes as having protected destructors so both the C++ wrappers
    // (which check has_protected_destructor) and the Rust FFI side skip new/delete.
    for b in &mut all_bindings {
        if exclude_classes.contains(&b.class_name) {
            b.has_protected_destructor = true;
        }
    }

    // Compute FunctionBindings once for ALL free functions — shared by all three generators
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L627-L628">main.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        if exclude_classes.contains(&b.cpp_name) {",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "627:sj"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/main.rs","startLine":627,"endLine":627}
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo check -p opencascade-binding-generator 2>&1
```
```
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Checking opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.04s
```

Now regenerate and build:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1 | grep -E "(complete|error)"
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

Clean build — no errors or Rust warnings. Let me verify the types are actually usable with methods:

**Terminal:**
```sh
grep -n "Message_LazyProgressScope\|Poly_CoherentTriPtr" /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/message.rs /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/poly.rs 2>/dev/null | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/message.rs:3839:// From Message_LazyProgressScope.hxx
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/message.rs:3842:/// **Source:** `Message_LazyProgressScope.hxx`:27 - `Message_LazyProgressScope`
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/message.rs:3851:pub use crate::ffi::Message_LazyProgressScope as LazyProgressScope;
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/message.rs:3854:    /// **Source:** `Message_LazyProgressScope.hxx`:49 - `Message_LazyProgressScope::Next()`
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/message.rs:3857:        unsafe { crate::ffi::Message_LazyProgressScope_next(self as *mut Self) }
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/message.rs:3860:    /// **Source:** `Message_LazyProgressScope.hxx`:62 - `Message_LazyProgressScope::More()`
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/message.rs:3863:        unsafe { crate::ffi::Message_LazyProgressScope_more(self as *const Self) }
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/message.rs:3866:    /// **Source:** `Message_LazyProgressScope.hxx`:65 - `Message_LazyProgressScope::IsAborted()`
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/message.rs:3869:        unsafe { crate::ffi::Message_LazyProgressScope_is_aborted(self as *mut Self) }
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/message.rs:3874:// SKIPPED: **Source:** `Message_LazyProgressScope.hxx`:36 - `Message_LazyProgressScope::Message_LazyProgressScope`
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/poly.rs:895:// From Poly_CoherentTriPtr.hxx
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/poly.rs:898:/// **Source:** `Poly_CoherentTriPtr.hxx`:34 - `Poly_CoherentTriPtr`
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/poly.rs:902:pub use crate::ffi::Poly_CoherentTriPtr as CoherentTriPtr;
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/poly.rs:905:    /// **Source:** `Poly_CoherentTriPtr.hxx`:108 - `Poly_CoherentTriPtr::GetTriangle()`
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/poly.rs:909:        unsafe { &*(crate::ffi::Poly_CoherentTriPtr_get_triangle(self as *const Self)) }
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/poly.rs:912:    /// **Source:** `Poly_CoherentTriPtr.hxx`:113 - `Poly_CoherentTriPtr::SetTriangle()`
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/poly.rs:916:        unsafe { crate::ffi::Poly_CoherentTriPtr_set_triangle(self as *mut Self, pTri as *const _) }
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/poly.rs:919:    /// **Source:** `Poly_CoherentTriPtr.hxx`:118 - `Poly_CoherentTriPtr::Next()`
/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/poly.rs:923:        unsa
... (truncated, 3235 chars total)
```
</details>

Both types have their methods bound:
- `Message_LazyProgressScope`: `Next()`, `More()`, `IsAborted()` (constructors skipped)
- `Poly_CoherentTriPtr`: `GetTriangle()`, `SetTriangle()`, `Next()`, `Previous()`, etc. (constructors skipped)

Now run tests:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (37 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.79s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-3404803fe4cd90de)

running 16 tests
test parser::tests::test_extract_module_from_header ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test module_graph::tests::test_extract_rust_type_name ... ok
test parser::tests::test_extract_template_arg ... ok
test header_deps::tests::test_is_likely_occt_header ... ok
test parser::tests::test_map_standard_type ... ok
test type_mapping::tests::test_map_class ... ok
test type_mapping::tests::test_map_const_ref ... ok
test type_mapping::tests::test_map_primitives ... ok
test resolver::tests::test_safe_param_name ... ok
test resolver::tests::test_safe_method_name ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
test type_mapping::tests::test_map_return_type ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/occt_bindgen-eda4fc796a0d21f2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests opencascade_binding_generator

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

real  3.1s
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo test --workspace --exclude opencascade-binding-generator 2>&1 | grep -E "(test result|FAILED|error)"
```

All tests pass. Now commit:

**Terminal:**
```sh
rm -f /Users/sethml/src/opencascade-rs/tmp/commit-msg.txt
```

Created [commit-msg.txt](../tmp/commit-msg.txt)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git add -A && git commit -F tmp/commit-msg.txt
```
```
[main e0fd0bc] Bind Message_LazyProgressScope and Poly_CoherentTriPtr without new/delete
 6 files changed, 419 insertions(+), 36 deletions(-)
```

**Todo list:**
- ✅ Understand exclude_classes mechanism
- ✅ Add no-construct/no-destruct support
- ✅ Un-exclude the two headers
- ✅ Regenerate, build, test, commit

Done. Both types are now bound with their public methods accessible:

- **`Message_LazyProgressScope`**: `Next()`, `More()`, `IsAborted()` — no constructors/destructors (protected operator new/delete from protected inheritance)
- **`Poly_CoherentTriPtr`**: `GetTriangle()`, `SetTriangle()`, `Next()`, `Previous()`, etc. — no constructors/destructors (hidden standard operator new/delete from `DEFINE_NCOLLECTION_ALLOC`)

Also fixed a latent bug: `exclude_classes` now properly propagates to the C++ side by setting `has_protected_destructor = true` on matching `ClassBindings` entries in main.rs, so both the Rust FFI declarations and C++ wrappers.cpp are consistent.

*2026-02-20 21:37 · 1011s · Words: 16,843 in · 2,251 out · 103,900 ctx · 42 rounds*

---

<a id="user-6"></a>

## User (6)

> Does this exclude entire classes, or just their desructors? If it&#x27;s just their destructors, then update the comment and rename the field.

*2026-02-20 21:37*

### Assistant — Claude Opus 4.6

> 💭 The user wants to know what `exclude_classes` actually does. From my earlier research, I know that `exclude_classes` only suppresses destructor generation (and by extension, `CppDeletable` impl). After my fix, it also sets `has_protected_destructor = true` which suppresses constructors too. But the name is misleading.
> 
> Let me verify exactly what it does now:
> 
> 1. In `rust.rs`: merges into `protected_destructor_class_names` → skips destructor FFI declaration and `CppDeletable` impl
> 2. In `main.rs` (my new code): sets `has_protected_destructor = true` → which in `bindings.rs` skips:
>    - Destructors in wrappers.cpp
>    - Constructors
>    - ToOwned/HandleGet impls
> 
> So `exclude_classes` now suppresses both constructors and destructors, while still generating the struct, methods, and static methods. But wait, there's redundancy here—constructors are also being excluded via `exclude_methods` with `ClassName::ClassName`, so the `has_protected_destructor` flag is making those entries unnecessary.
> 
> Let me verify: the check at line 1524 does suppress constructors when there's a protected destructor. So `exclude_classes` effectively handles both construction and destruction, not just destructors. The user wants me to rename this field to better reflect that it suppresses new/delete operations, not just destruction. I think `non_allocatable_classes` is clearer since it describes what these classes can't do—they can't be heap-allocated with new/delete, though you can still work with pointers to them. Now I need to find all the places this field is referenced and update them to use the new name.

It suppresses constructors and destructors (plus `CppDeletable`/`ToOwned`), but keeps the struct and all methods. Let me trace all usages, update the comment, and rename the field.

<details>
<summary>Searched for text <code>exclude_classes</code>, 20 results</summary>

- [bindings.toml:101](../crates/opencascade-sys/bindings.toml#L101)
- [2026-02-20_20-45-vscode-claude-opus-4.6.md:117](../agent-logs/2026-02-20_20-45-vscode-claude-opus-4.6.md#L117)
- [2026-02-20_20-45-vscode-claude-opus-4.6.md:321](../agent-logs/2026-02-20_20-45-vscode-claude-opus-4.6.md#L321)
- [2026-02-20_20-10-vscode-claude-opus-4.6.md:49](../agent-logs/2026-02-20_20-10-vscode-claude-opus-4.6.md#L49)
- [2026-02-20_20-10-vscode-claude-opus-4.6.md:291](../agent-logs/2026-02-20_20-10-vscode-claude-opus-4.6.md#L291)
- [2026-02-20_20-10-vscode-claude-opus-4.6.md:311](../agent-logs/2026-02-20_20-10-vscode-claude-opus-4.6.md#L311)
- [2026-02-20_20-10-vscode-claude-opus-4.6.md:320](../agent-logs/2026-02-20_20-10-vscode-claude-opus-4.6.md#L320)
- [2026-02-20_20-10-vscode-claude-opus-4.6.md:326](../agent-logs/2026-02-20_20-10-vscode-claude-opus-4.6.md#L326)
- [2026-02-20_20-10-vscode-claude-opus-4.6.md:383](../agent-logs/2026-02-20_20-10-vscode-claude-opus-4.6.md#L383)
- [2026-02-20_20-10-vscode-claude-opus-4.6.md:392](../agent-logs/2026-02-20_20-10-vscode-claude-opus-4.6.md#L392)
- [2026-02-20_20-10-vscode-claude-opus-4.6.md:398](../agent-logs/2026-02-20_20-10-vscode-claude-opus-4.6.md#L398)
- [2026-02-20_20-10-vscode-claude-opus-4.6.md:405](../agent-logs/2026-02-20_20-10-vscode-claude-opus-4.6.md#L405)
- [2026-02-20_20-10-vscode-claude-opus-4.6.md:418](../agent-logs/2026-02-20_20-10-vscode-claude-opus-4.6.md#L418)
- [2026-02-20_20-10-vscode-claude-opus-4.6.md:425](../agent-logs/2026-02-20_20-10-vscode-claude-opus-4.6.md#L425)
- [2026-02-20_20-10-vscode-claude-opus-4.6.md:438](../agent-logs/2026-02-20_20-10-vscode-claude-opus-4.6.md#L438)
- [2026-02-20_20-10-vscode-claude-opus-4.6.md:458](../agent-logs/2026-02-20_20-10-vscode-claude-opus-4.6.md#L458)
- [2026-02-20_20-10-vscode-claude-opus-4.6.md:465](../agent-logs/2026-02-20_20-10-vscode-claude-opus-4.6.md#L465)
- [2026-02-20_20-10-vscode-claude-opus-4.6.md:485](../agent-logs/2026-02-20_20-10-vscode-claude-opus-4.6.md#L485)
- [2026-02-20_20-10-vscode-claude-opus-4.6.md:492](../agent-logs/2026-02-20_20-10-vscode-claude-opus-4.6.md#L492)
- [2026-02-20_20-10-vscode-claude-opus-4.6.md:505](../agent-logs/2026-02-20_20-10-vscode-claude-opus-4.6.md#L505)
</details>

<details>
<summary>Searched for text <code>exclude_classes</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 10 results</summary>

- [main.rs:96](../crates/opencascade-binding-generator/src/main.rs#L96)
- [main.rs:139](../crates/opencascade-binding-generator/src/main.rs#L139)
- [main.rs:376](../crates/opencascade-binding-generator/src/main.rs#L376)
- [main.rs:596](../crates/opencascade-binding-generator/src/main.rs#L596)
- [main.rs:624](../crates/opencascade-binding-generator/src/main.rs#L624)
- [main.rs:627](../crates/opencascade-binding-generator/src/main.rs#L627)
- [main.rs:651](../crates/opencascade-binding-generator/src/main.rs#L651)
- [rust.rs:148](../crates/opencascade-binding-generator/src/codegen/rust.rs#L148)
- [rust.rs:153](../crates/opencascade-binding-generator/src/codegen/rust.rs#L153)
- [config.rs:48](../crates/opencascade-binding-generator/src/config.rs#L48)
</details>

Now let me read the relevant lines in each file to prepare accurate edits:

<details>
<summary>Reading lines 40-55 of <a href="../crates/opencascade-binding-generator/src/config.rs#L40-L56">config.rs</a></summary>

```
    /// even though the rest of the class is bound.
    #[serde(default)]
    pub exclude_methods: Vec<String>,

    /// Exclude specific classes/types from binding generation.
    /// These classes will not get any bindings (no struct, no methods, no destructor).
    /// For nested types, use the C++ qualified name: "Parent::Nested".
    #[serde(default)]
    pub exclude_classes: Vec<String>,

    /// Opaque types defined in manual/ files but referenced by auto-generated bindings.
    /// The generator adds these to the known class set so methods using them
    /// aren't skipped as "unknown type".
    /// Format: `TypeName = { header = "Header.hxx" }`
    #[serde(default)]
    pub manual_types: std::collections::HashMap<String, ManualType>,
```
</details>

<details>
<summary>Reading lines 93-100 of <a href="../crates/opencascade-binding-generator/src/main.rs#L93-L101">main.rs</a></summary>

```
    }

    // Determine explicit headers from config file or CLI arguments
    let (explicit_headers, resolve_deps, exclude_set, exclude_modules, exclude_methods, exclude_classes, manual_type_names) = if let Some(ref config_path) = args.config {
        let cfg = config::load_config(config_path)?;
        let resolve = cfg.general.resolve_deps;

        if args.include_dirs.is_empty() {
```
</details>

<details>
<summary>Reading lines 136-145 of <a href="../crates/opencascade-binding-generator/src/main.rs#L136-L146">main.rs</a></summary>

```

        let excludes: std::collections::HashSet<String> = cfg.exclude_headers.into_iter().collect();
        let exclude_mods: Vec<String> = cfg.exclude_modules;
        let exclude_cls: HashSet<String> = cfg.exclude_classes.into_iter().collect();
        let manual_names: HashSet<String> = cfg.manual_types.keys().cloned().collect();
        (headers, resolve, excludes, exclude_mods, method_exclusions, exclude_cls, manual_names)
    } else if !args.headers.is_empty() {
        (args.headers.clone(), args.resolve_deps, std::collections::HashSet::new(), Vec::new(), HashSet::new(), HashSet::new(), HashSet::new())
    } else {
        anyhow::bail!("Either --config <file.toml> or positional header arguments are required");
```
</details>

<details>
<summary>Reading lines 373-380 of <a href="../crates/opencascade-binding-generator/src/main.rs#L373-L381">main.rs</a></summary>

```
    }

    // Generate FFI output
    generate_output(&args, &all_classes, &all_functions, &graph, &symbol_table, &known_headers, &exclude_methods, &exclude_classes, &handle_able_classes, &manual_type_names)
}

/// Detect "utility namespace classes" and convert their static methods to free functions.
///
```
</details>

<details>
<summary>Reading lines 588-600 of <a href="../crates/opencascade-binding-generator/src/main.rs#L588-L601">main.rs</a></summary>

```
fn generate_output(
    args: &Args,
    all_classes: &[&model::ParsedClass],
    all_functions: &[&model::ParsedFunction],
    graph: &module_graph::ModuleGraph,
    symbol_table: &resolver::SymbolTable,
    known_headers: &HashSet<String>,
    exclude_methods: &HashSet<(String, String)>,
    exclude_classes: &HashSet<String>,
    handle_able_classes: &HashSet<String>,
    manual_type_names: &HashSet<String>,
) -> Result<()> {
    use model::ParsedClass;
```
</details>

<details>
<summary>Reading lines 621-660 of <a href="../crates/opencascade-binding-generator/src/main.rs#L621-L661">main.rs</a></summary>

```
    let mut all_bindings =
        codegen::bindings::compute_all_class_bindings(all_classes, symbol_table, &collection_type_names, &extra_typedef_names, exclude_methods, manual_type_names);

    // Mark exclude_classes as having protected destructors so both the C++ wrappers
    // (which check has_protected_destructor) and the Rust FFI side skip new/delete.
    for b in &mut all_bindings {
        if exclude_classes.contains(&b.cpp_name) {
            b.has_protected_destructor = true;
        }
    }

    // Compute FunctionBindings once for ALL free functions — shared by all three generators
    let (all_function_bindings, all_skipped_functions) = codegen::bindings::compute_all_function_bindings(
        symbol_table, all_classes, &collection_type_names, &extra_typedef_names, known_headers, manual_type_names,
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
        exclude_classes,
    );
    let ffi_path = args.output.join("ffi.rs");
    std::fs::write(&ffi_path, ffi_code)?;
    generated_rs_files.push(ffi_path.clone());
    println!("  Wrote: {} ({} classes, {} functions)",
        ffi_path.display(), all_classes.len(), all_functions.len());

    // 2. Generate wrappers.cpp
    println!("Generating wrappers.cpp...");
```
</details>

<details>
<summary>Reading lines 140-160 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L140-L161">rust.rs</a></summary>

```
    all_classes: &[&ParsedClass],
    all_headers: &[String],
    collections: &[super::collections::CollectionInfo],
    symbol_table: &crate::resolver::SymbolTable,
    all_bindings: &[super::bindings::ClassBindings],
    function_bindings: &[super::bindings::FunctionBinding],
    handle_able_classes: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
    exclude_classes: &HashSet<String>,
) -> (String, Vec<NestedTypeInfo>) {
    // Get all classes with protected destructors, and merge in excluded classes
    // (excluded classes should not get destructors/CppDeletable either)
    let mut protected_destructor_class_names = symbol_table.protected_destructor_class_names();
    for cls in exclude_classes {
        protected_destructor_class_names.insert(cls.clone());
    }

    // All enum names (needed for opaque type filtering)
    let all_enum_names = &symbol_table.all_enum_names;

    // Collect collection type names to exclude from class generation
```
</details>

Now I have all the locations. The rename:  → `non_allocatable_classes`. This better describes the semantics: the struct and methods are kept, but new/delete are suppressed.

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L96-L97">main.rs -1+1</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L376-L377">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L596-L597">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L651-L652">-1+1</a>  
**Failing: <a href="../crates/opencascade-binding-generator/src/config.rs#L44-L51">config.rs -1+7</a>**  
**Failing: <a href="../crates/opencascade-binding-generator/src/main.rs#L139-L142">main.rs -1+3</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L626-L634">-1+8</a>**  
**Failing: <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L148-L156">rust.rs -1+8</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    /// Non-allocatable classes: suppress constructors and destructors (including\n    /// CppDeletable/ToOwned), but keep the opaque struct and all methods.\n    /// Use for classes with protected/hidden operator new/delete that can still\n    /// be used via pointers obtained from other APIs.\n    /// For nested types, use the C++ qualified name: \"Parent::Nested\".\n    #[serde(default)]\n    pub non_allocatable_classes: Vec<String>,",
      "filePath": "crates/opencascade-binding-generator/src/config.rs",
      "lineHashes": "44:pl\n45:oy\n46:iz\n47:lb\n48:cy"
    },
    {
      "content": "    let (explicit_headers, resolve_deps, exclude_set, exclude_modules, exclude_methods, non_allocatable_classes, manual_type_names) = if let Some(ref config_path) = args.config {",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "96:wv"
    },
    {
      "content": "        let non_alloc_cls: HashSet<String> = cfg.non_allocatable_classes.into_iter().collect();\n        let manual_names: HashSet<String> = cfg.manual_types.keys().cloned().collect();\n        (headers, resolve, excludes, exclude_mods, method_exclusions, non_alloc_cls, manual_names)",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "139:gj\n140:cf\n141:ob"
    },
    {
      "content": "    generate_output(&args, &all_classes, &all_functions, &graph, &symbol_table, &known_headers, &exclude_methods, &non_allocatable_classes, &handle_able_classes, &manual_type_names)",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "376:uk"
    },
    {
      "content": "    non_allocatable_classes: &HashSet<String>,",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "596:ne"
    },
    {
      "content": "    // Mark non-allocatable classes as having protected destructors so both the\n    // C++ wrappers (which check has_protected_destructor) and the Rust FFI side\n    // skip constructors, destructors, CppDeletable, and ToOwned.\n    for b in &mut all_bindings {\n        if non_allocatable_classes.contains(&b.cpp_name) {\n            b.has_protected_destructor = true;\n        }\n    }",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "624:rh\n625:lc\n626:sk\n627:rw\n628:aj\n629:cy\n630:ax"
    },
    {
      "content": "        non_allocatable_classes,",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "651:ea"
    },
    {
      "content": "    non_allocatable_classes: &HashSet<String>,\n) -> (String, Vec<NestedTypeInfo>) {\n    // Get all classes with protected destructors, and merge in non-allo
... (truncated, 3515 chars)
```
**Output:**
```
{
  "applied": 8,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/config.rs",
      "startLine": 44,
      "endLine": 54
    },
    {
      "file": "crates/opencascade-binding-generator/src/main.rs",
      "startLine": 96,
      "endLine": 660
    },
    {
      "file": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "startLine": 148,
      "endLine": 162
    }
  ]
}
```
</details>

Now update bindings.toml:

<details>
<summary>Reading lines 99-115 of <a href="../crates/opencascade-sys/bindings.toml#L99-L116">bindings.toml</a></summary>

```

# Exclude specific classes/types from destructor generation.
exclude_classes = [
    "Message_ProgressScope::NullString",  # Forward-declared only, no full definition
    "MeshVS_NodePair",                    # Typedef for std::pair<int,int>, not a real class

    # Protected/hidden operator delete — skip destructors but keep methods accessible.
    "Message_LazyProgressScope",
    "Poly_CoherentTriPtr",
]

[general]
resolve_deps = true

# Opaque types defined in manual/ files. These are not parsed from OCCT headers
# but referenced by auto-generated bindings. The generator adds them to the known
# class set so methods using them aren't skipped as "unknown type".
```
</details>

<details>
<summary>**Failing: <a href="../crates/opencascade-sys/bindings.toml#L100-L110">bindings.toml -1+10</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "# Non-allocatable classes: suppress constructors 和 destructors (including\n# CppDeletable/ToOwned) but keep the opaque struct and all methods.\n# Use for classes with protected/hidden operator new/delete, forward-declared\n# nested types, or typedefs that aren't real classes.\nnon_allocatable_classes = [\n    \"Message_ProgressScope::NullString\",  # Forward-declared only, no full definition\n    \"MeshVS_NodePair\",                    # Typedef for std::pair<int,int>, not a real class\n    \"Message_LazyProgressScope\",           # Protected operator new/delete (protected inheritance)\n    \"Poly_CoherentTriPtr\",                 # Hidden operator new/delete (DEFINE_NCOLLECTION_ALLOC)\n]",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "100:tk\n101:yu\n102:ae\n103:sl\n104:dd\n105:xz\n106:et\n107:zm\n108:cy"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":100,"endLine":117}
```
</details>

Wait, I used a Unicode character accidentally. Let me fix that:

<details>
<summary>Reading lines 100-102 of <a href="../crates/opencascade-sys/bindings.toml#L100-L103">bindings.toml</a></summary>

```
# Non-allocatable classes: suppress constructors 和 destructors (including
# CppDeletable/ToOwned) but keep the opaque struct and all methods.
# Use for classes with protected/hidden operator new/delete, forward-declared
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-sys/bindings.toml#L100-L101">bindings.toml -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "# Non-allocatable classes: suppress constructors and destructors (including",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "100:zf"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":100,"endLine":100}
```
</details>

Now verify everything compiles:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo check -p opencascade-binding-generator 2>&1
```

Generator compiles. Let me do a full regenerate + build + test cycle:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1 | grep -E "(complete|error)"
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1 | grep -E "(error|Finished|warning:.*generated)"
```

Clean build — only the OCCT sprintf deprecation warning. Let me run the tests:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/run-binding-generator-tests.sh 2>&1 | grep -E "(test result|FAILED)"
```
```
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
real  5.9s
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo test --workspace --exclude opencascade-binding-generator 2>&1 | grep -E "(test result|FAILED|error)"
```

All pass. Now amend the All pass. Now amend the previous commit since this is a refinement:

**Terminal:**
```sh
rm -f /Users/sethml/src/opencascade-rs/tmp/commit-msg.txt
```

Created [commit-msg.txt](../tmp/commit-msg.txt)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git add -A && git commit -F tmp/commit-msg.txt
```
```
[main a9aea16] Rename exclude_classes to non_allocatable_classes
 5 files changed, 972 insertions(+), 22 deletions(-)
```

Committed as `a9aea16`.

*2026-02-20 21:44 · 437s · Words: 736 in · 919 out · 16,044 ctx · 23 rounds*

---
