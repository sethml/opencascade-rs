# Resolving cross-platform typedef differences in Git

- **Date:** 2026-02-23 22:42 – 2026-02-24 08:52
- **Models:** Claude Opus 4.6, Claude Sonnet 4.6
- **Turns:** 5
- **Tool calls:** 234
- **Thinking blocks:** 39
- **Input words:** 15,307
- **Output words:** 13,747
- **Prompt tokens (last round):** 353,966
- **API rounds:** 224
- **Total elapsed:** 3070s

## Table of Contents

1. [The state of git HEAD had bindings generated on a Linux box, while the CWD changes are generated ...](#user-1) (Claude Opus 4.6)
2. [That&#x27;s not quite what I had in mind. I&#x27;d imagined that in Rust aspect::rendering_context would be...](#user-2) (Claude Opus 4.6)
3. [I probably should have asked before starting this refactoring: How does one get an Aspect_Renderi...](#user-3) (Claude Sonnet 4.6)
4. [I see. OK, revert back to the *mut std::ffi::c_void approach. We shouldn&#x27;t need a pub struct Aspe...](#user-4) (Claude Sonnet 4.6)
5. [Commit. Then Move this into bindings.toml. Decide on a good way to represent custom mappings in b...](#user-5) (Claude Sonnet 4.6)

---

<a id="user-1"></a>

## User (1)

> The state of git HEAD had bindings generated on a Linux box, while the CWD changes are generated on macos. There are some differences, which I would like to make go away. In particular, there&#x27;s a typedef Aspect_RenderingContext that translates to void* or NSOpenGLContext* - can we make it an opaque type instead?

*2026-02-23 22:46*

### Assistant — Claude Opus 4.6

> 💭 Let me understand the problem. The user has bindings that differ between Linux and macOS because `Aspect_RenderingContext` is a typedef that resolves to `void*` on Linux and `NSOpenGLContext*` on macOS. They want to make it an opaque type instead so the bindings are consistent across platforms.
> 
> Let me first look at the git diff to understand the differences, and then find where this type is handled in the binding generator.

**Todo list:**
- 🔄 Examine git diff for platform differences
- ⬜ Find Aspect_RenderingContext in OCCT headers
- ⬜ Find how binding generator handles typedefs
- ⬜ Make Aspect_RenderingContext opaque
- ⬜ Regenerate and verify bindings

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff HEAD --stat | head -30
```
<details>
<summary>Output (5 lines)</summary>

```
 crates/opencascade-sys/generated/ffi.rs       | 20 +++++-----------
 crates/opencascade-sys/generated/graphic3d.rs | 30 +++++++----------------
 crates/opencascade-sys/generated/v3d.rs       | 34 +++++++++------------------
 crates/opencascade-sys/generated/wrappers.cpp |  8 +------
 4 files changed, 27 insertions(+), 65 deletions(-)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff HEAD -- crates/opencascade-sys/generated/ | head -200
```
<details>
<summary>Output (193 lines)</summary>

```
diff --git a/crates/opencascade-sys/generated/ffi.rs b/crates/opencascade-sys/generated/ffi.rs
index 03c8bba..a03b34b 100644
--- a/crates/opencascade-sys/generated/ffi.rs
+++ b/crates/opencascade-sys/generated/ffi.rs
@@ -36679,6 +36679,11 @@ pub struct NCollection_Vector_opencascade_handle_Standard_Transient {
 }
 /// Referenced type from C++
 #[repr(C)]
+pub struct NSOpenGLContext {
+    _opaque: [u8; 0],
+}
+/// Referenced type from C++
+#[repr(C)]
 pub struct OSD_Function {
     _opaque: [u8; 0],
 }
@@ -209521,13 +209526,6 @@ extern "C" {
         self_: *mut Graphic3d_CView,
         theDrawToFrontBuffer: bool,
     ) -> bool;
-    /// **Source:** `Graphic3d_CView.hxx`:283 - `Graphic3d_CView::SetWindow()`
-    pub fn Graphic3d_CView_set_window(
-        self_: *mut Graphic3d_CView,
-        theParentVIew: *const HandleGraphic3dCView,
-        theWindow: *const HandleAspectWindow,
-        theContext: *mut std::ffi::c_void,
-    );
     /// **Source:** `Graphic3d_CView.hxx`:288 - `Graphic3d_CView::Window()`
     pub fn Graphic3d_CView_window(self_: *const Graphic3d_CView) -> *mut HandleAspectWindow;
     /// **Source:** `Graphic3d_CView.hxx`:291 - `Graphic3d_CView::IsDefined()`
@@ -641517,14 +641515,8 @@ extern "C" {
     ) -> *mut V3d_View;
     /// **Source:** `V3d_View.hxx`:58 - `V3d_View::DynamicType()`
     pub fn V3d_View_dynamic_type(self_: *const V3d_View) -> *const HandleStandardType;
-    /// **Source:** `V3d_View.hxx`:76 - `V3d_View::SetWindow()`
-    pub fn V3d_View_set_window_handleaspectwindow_voidptr(
-        self_: *mut V3d_View,
-        theWindow: *const HandleAspectWindow,
-        theContext: *mut std::ffi::c_void,
-    );
     /// **Source:** `V3d_View.hxx`:93 - `V3d_View::SetWindow()`
-    pub fn V3d_View_set_window_handlev3dview_vec2d_typeoftriedronposition_vec2d_vec2i(
+    pub fn V3d_View_set_window(
         self_: *mut V3d_View,
         theParentView: *const HandleV3dView,
         theSize: *const Graphic3d_Vec2d,
diff --git a/crates/opencascade-sys/generated/graphic3d.rs b/crates/opencascade-sys/generated/graphic3d.rs
index 2d9971c..43135a3 100644
--- a/crates/opencascade-sys/generated/graphic3d.rs
+++ b/crates/opencascade-sys/generated/graphic3d.rs
@@ -17967,27 +17967,6 @@ impl CView {
         }
     }
 
-    /// **Source:** `Graphic3d_CView.hxx`:283 - `Graphic3d_CView::SetWindow()`
-    /// Creates and maps rendering window to the view.
-    /// @param[in] theParentVIew parent view or NULL
-    /// @param[in] theWindow the window
-    /// @param[in] theContext the rendering context; if NULL the context will be created internally
-    pub unsafe fn set_window(
-        &mut self,
-        theParentVIew: &crate::ffi::HandleGraphic3dCView,
-        theWindow: &crate::ffi::HandleAspectWindow,
-        theContext: *mut std::ffi::c_void,
-    ) {
-        unsafe {
-            crate::ffi::Graphic3d_CView_set_window(
-                self as *mut Self,
... (truncated, 9831 chars total)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code - Insiders/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/ea54976d-0c4a-4387-8833-79fd31749c73/toolu_01LHkXwdiC89p6j4uBXWVeHd__vscode-1771913780866/content.txt)

**Todo list:**
- ✅ Examine git diff for platform differences
- 🔄 Find Aspect_RenderingContext in OCCT headers
- ⬜ Find how binding generator handles typedefs
- ⬜ Make Aspect_RenderingContext opaque
- ⬜ Regenerate and verify bindings

Now I see the issue clearly. On Linux, `Aspect_RenderingContext` is `void*`, so those methods compile fine. On macOS, it resolves to `NSOpenGLContext*`, which the generator doesn't know about and skips the methods. Let me find how this typedef is defined and how the binding generator handles it.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -rn 'Aspect_RenderingContext' crates/occt-sys/OCCT/src/ | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
crates/occt-sys/OCCT/src/D3DHost/D3DHost_View.cxx:128:                             const Aspect_RenderingContext  theContext)
crates/occt-sys/OCCT/src/D3DHost/D3DHost_View.hxx:49:                                         const Aspect_RenderingContext  theContext)
crates/occt-sys/OCCT/src/Aspect/Aspect_RenderingContext.hxx:15:/*==== Title: Aspect_RenderingContext.hxx                                     */
crates/occt-sys/OCCT/src/Aspect/Aspect_RenderingContext.hxx:22:#ifndef _Aspect_RenderingContext_HeaderFile
crates/occt-sys/OCCT/src/Aspect/Aspect_RenderingContext.hxx:23:#define _Aspect_RenderingContext_HeaderFile
crates/occt-sys/OCCT/src/Aspect/Aspect_RenderingContext.hxx:35:typedef EAGLContext* Aspect_RenderingContext;
crates/occt-sys/OCCT/src/Aspect/Aspect_RenderingContext.hxx:42:Standard_DISABLE_DEPRECATION_WARNINGS typedef NSOpenGLContext* Aspect_RenderingContext;
crates/occt-sys/OCCT/src/Aspect/Aspect_RenderingContext.hxx:46:typedef void* Aspect_RenderingContext; // GLXContext under UNIX
crates/occt-sys/OCCT/src/Aspect/Aspect_RenderingContext.hxx:49:#endif /* _Aspect_RenderingContext_HeaderFile */
crates/occt-sys/OCCT/src/Aspect/FILES:40:Aspect_RenderingContext.hxx
crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CView.hxx:17:#include <Aspect_RenderingContext.hxx>
crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CView.hxx:285:                         const Aspect_RenderingContext  theContext) = 0;
crates/occt-sys/OCCT/src/V3d/V3d_View.cxx:190:                         const Aspect_RenderingContext theContext)
crates/occt-sys/OCCT/src/V3d/V3d_View.hxx:77:                                 const Aspect_RenderingContext theContext = NULL);
crates/occt-sys/OCCT/src/OpenGl/OpenGl_Context.cxx:273:  myGContext = (Aspect_RenderingContext)EGL_NO_CONTEXT;
crates/occt-sys/OCCT/src/OpenGl/OpenGl_Context.cxx:945:  myGContext = (Aspect_RenderingContext)eglGetCurrentContext();
crates/occt-sys/OCCT/src/OpenGl/OpenGl_Context.cxx:949:  myGContext = (Aspect_RenderingContext)wglGetCurrentContext();
crates/occt-sys/OCCT/src/OpenGl/OpenGl_Context.cxx:952:  myGContext = (Aspect_RenderingContext)glXGetCurrentContext();
crates/occt-sys/OCCT/src/OpenGl/OpenGl_Context.cxx:973:                                      const Aspect_RenderingContext theContext,
crates/occt-sys/OCCT/src/OpenGl/OpenGl_GraphicDriver.cxx:192:  myEglContext = (Aspect_RenderingContext)EGL_NO_CONTEXT;
```
</details>

Find binding generator type handling

<details>
<summary>Searched for files matching <code>**/*opencascade-binding-generator*/**</code>, 19 matches</summary>

- [codegen-flow.svg](../crates/opencascade-binding-generator/codegen-flow.svg)
- [Cargo.toml](../crates/opencascade-binding-generator/Cargo.toml)
- [model.rs](../crates/opencascade-binding-generator/src/model.rs)
- [parser.rs](../crates/opencascade-binding-generator/src/parser.rs)
- [resolver.rs](../crates/opencascade-binding-generator/src/resolver.rs)
- [main.rs](../crates/opencascade-binding-generator/src/main.rs)
- [collections.rs](../crates/opencascade-binding-generator/src/codegen/collections.rs)
- [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs)
- [rust.rs](../crates/opencascade-binding-generator/src/codegen/rust.rs)
- [mod.rs](../crates/opencascade-binding-generator/src/codegen/mod.rs)
- [cpp.rs](../crates/opencascade-binding-generator/src/codegen/cpp.rs)
- [header_deps.rs](../crates/opencascade-binding-generator/src/header_deps.rs)
- [type_mapping.rs](../crates/opencascade-binding-generator/src/type_mapping.rs)
- [lib.rs](../crates/opencascade-binding-generator/src/lib.rs)
- [config.rs](../crates/opencascade-binding-generator/src/config.rs)
- [module_graph.rs](../crates/opencascade-binding-generator/src/module_graph.rs)
- [README.md](../crates/opencascade-binding-generator/README.md)
- [codegen-flow.png](../crates/opencascade-binding-generator/codegen-flow.png)
- [codegen-flow.dot](../crates/opencascade-binding-generator/codegen-flow.dot)
</details>

<details>
<summary>Searched for regex <code>typedef|opaque|skip|exclude|Aspect_RenderingContext|void\*</code>, 200 results</summary>

- [CLAUDE.md:11](../CLAUDE.md#L11)
- [AGENTS.md:11](../AGENTS.md#L11)
- [writing_bindings.md:25](../docs/writing_bindings.md#L25)
- [writing_bindings.md:35](../docs/writing_bindings.md#L35)
- [LICENSE:410](../LICENSE#L410)
- [README.md:17](../README.md#L17)
- [README.md:19](../README.md#L19)
- [README.md:96](../README.md#L96)
- [README.md:138](../README.md#L138)
- [zbox_case_failure.txt:1](../zbox_case_failure.txt#L1)
- [compute-skip-stats.py:9](../scripts/compute-skip-stats.py#L9)
- [compute-skip-stats.py:14](../scripts/compute-skip-stats.py#L14)
- [compute-skip-stats.py:24](../scripts/compute-skip-stats.py#L24)
- [compute-skip-stats.py:25](../scripts/compute-skip-stats.py#L25)
- [compute-skip-stats.py:43](../scripts/compute-skip-stats.py#L43)
- [compute-skip-stats.py:44](../scripts/compute-skip-stats.py#L44)
- [compute-skip-stats.py:51](../scripts/compute-skip-stats.py#L51)
- [compute-skip-stats.py:66](../scripts/compute-skip-stats.py#L66)
- [compute-skip-stats.py:90](../scripts/compute-skip-stats.py#L90)
- [compute-skip-stats.py:95](../scripts/compute-skip-stats.py#L95)
- [compute-skip-stats.py:117](../scripts/compute-skip-stats.py#L117)
- [compute-skip-stats.py:124](../scripts/compute-skip-stats.py#L124)
- [compute-skip-stats.py:153](../scripts/compute-skip-stats.py#L153)
- [compute-skip-stats.py:173](../scripts/compute-skip-stats.py#L173)
- [wasm.rs:9](../crates/model-api/src/wasm.rs#L9)
- [wasm.rs:10](../crates/model-api/src/wasm.rs#L10)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:90](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L90)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:102](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L102)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:107](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L107)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:145](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L145)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:149](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L149)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:158](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L158)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:196](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L196)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:200](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L200)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:202](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L202)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:340](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L340)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:370](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L370)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:376](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L376)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:389](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L389)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:453](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L453)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:468](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L468)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:476](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L476)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:480](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L480)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:486](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L486)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:495](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L495)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:497](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L497)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:507](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L507)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:520](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L520)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:527](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L527)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:579](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L579)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:589](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L589)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:595](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L595)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:597](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L597)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:703](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L703)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:704](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L704)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:717](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L717)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:966](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L966)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:986](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L986)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:997](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L997)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1003](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1003)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1010](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1010)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1026](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1026)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1037](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1037)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1042](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1042)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1059](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1059)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1153](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1153)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1155](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1155)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1239](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1239)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1241](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1241)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1312](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1312)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1432](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1432)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1433](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1433)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1445](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1445)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1447](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1447)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1467](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1467)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1469](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1469)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1470](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1470)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1481](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1481)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1483](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1483)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1502](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1502)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1534](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1534)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1617](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1617)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1654](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1654)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1669](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1669)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1750](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1750)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1754](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1754)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1792](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1792)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1799](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1799)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1849](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1849)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1970](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1970)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2006](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2006)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2144](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2144)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2292](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2292)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2298](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2298)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2435](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2435)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2455](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2455)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2486](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2486)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2506](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2506)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2623](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2623)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2692](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2692)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2709](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2709)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2713](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2713)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2731](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2731)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2769](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2769)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2771](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2771)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2773](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2773)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2777](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2777)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2783](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2783)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2785](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2785)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2787](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2787)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2789](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2789)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2791](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2791)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2797](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2797)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2801](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2801)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2810](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2810)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2818](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2818)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2838](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2838)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2871](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2871)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2975](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2975)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2984](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2984)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2993](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2993)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:2999](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L2999)
</details>

Read [parser.rs](../crates/opencascade-binding-generator/src/parser.rs#1-1), lines 1 to 300

Read [content.txt](/Users/sethml/Library/Application Support/Code - Insiders/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/ea54976d-0c4a-4387-8833-79fd31749c73/toolu_01U4n3zKsfXVbbNmYBBHgiLf__vscode-1771913780870/content.txt#1-1), lines 1 to 300

**Todo list:**
- ✅ Examine git diff for platform differences
- ✅ Find Aspect_RenderingContext in OCCT headers
- 🔄 Find how binding generator handles typedefs
- ⬜ Make Aspect_RenderingContext opaque
- ⬜ Regenerate and verify bindings

Let me look at how specific types are resolved in the parser and find where types are mapped to Rust types — particularly how `void*` gets handled and where the "unknown type" skip logic lives.

Search complete: Find type resolution and skip logic

<details>
<summary>Searched for files matching <code>**/*.toml</code>, 12 matches</summary>

- [Cargo.toml](../examples/Cargo.toml)
- [rustfmt.toml](../rustfmt.toml)
- [Cargo.toml](../crates/kicad-parser/Cargo.toml)
- [Cargo.toml](../crates/opencascade-binding-generator/Cargo.toml)
- [Cargo.toml](../Cargo.toml)
- [bindings.toml](../crates/opencascade-sys/bindings.toml)
- [Cargo.toml](../crates/viewer/Cargo.toml)
- [Cargo.toml](../crates/model-api/Cargo.toml)
- [Cargo.toml](../crates/opencascade/Cargo.toml)
- [Cargo.toml](../crates/opencascade-sys/Cargo.toml)
- [Cargo.toml](../crates/occt-sys/Cargo.toml)
- [Cargo.toml](../crates/wasm-example/Cargo.toml)
</details>

<details>
<summary>Searched for regex <code>resolve_type|map_type|unknown type|opaque_types|type_overrides</code>, 200 results</summary>

- [compute-skip-stats.py:36](../scripts/compute-skip-stats.py#L36)
- [compute-skip-stats.py:78](../scripts/compute-skip-stats.py#L78)
- [compute-skip-stats.py:111](../scripts/compute-skip-stats.py#L111)
- [compute-skip-stats.py:169](../scripts/compute-skip-stats.py#L169)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1258](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1258)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1981](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1981)
- [parser.rs:61](../crates/opencascade-binding-generator/src/parser.rs#L61)
- [parser.rs:76](../crates/opencascade-binding-generator/src/parser.rs#L76)
- [parser.rs:379](../crates/opencascade-binding-generator/src/parser.rs#L379)
- [parser.rs:384](../crates/opencascade-binding-generator/src/parser.rs#L384)
- [parser.rs:399](../crates/opencascade-binding-generator/src/parser.rs#L399)
- [parser.rs:407](../crates/opencascade-binding-generator/src/parser.rs#L407)
- [resolver.rs:970](../crates/opencascade-binding-generator/src/resolver.rs#L970)
- [resolver.rs:1019](../crates/opencascade-binding-generator/src/resolver.rs#L1019)
- [resolver.rs:1025](../crates/opencascade-binding-generator/src/resolver.rs#L1025)
- [resolver.rs:1081](../crates/opencascade-binding-generator/src/resolver.rs#L1081)
- [resolver.rs:1087](../crates/opencascade-binding-generator/src/resolver.rs#L1087)
- [resolver.rs:1149](../crates/opencascade-binding-generator/src/resolver.rs#L1149)
- [resolver.rs:1155](../crates/opencascade-binding-generator/src/resolver.rs#L1155)
- [resolver.rs:1238](../crates/opencascade-binding-generator/src/resolver.rs#L1238)
- [main.rs:105](../crates/opencascade-binding-generator/src/main.rs#L105)
- [main.rs:154](../crates/opencascade-binding-generator/src/main.rs#L154)
- [main.rs:164](../crates/opencascade-binding-generator/src/main.rs#L164)
- [main.rs:176](../crates/opencascade-binding-generator/src/main.rs#L176)
- [main.rs:191](../crates/opencascade-binding-generator/src/main.rs#L191)
- [main.rs:257](../crates/opencascade-binding-generator/src/main.rs#L257)
- [2026-02-14_15-20-vscode-claude-opus-4.6.md:594](../agent-logs/2026-02-14_15-20-vscode-claude-opus-4.6.md#L594)
- [2026-02-14_15-20-vscode-claude-opus-4.6.md:596](../agent-logs/2026-02-14_15-20-vscode-claude-opus-4.6.md#L596)
- [2026-02-14_15-20-vscode-claude-opus-4.6.md:598](../agent-logs/2026-02-14_15-20-vscode-claude-opus-4.6.md#L598)
- [2026-02-14_15-20-vscode-claude-opus-4.6.md:600](../agent-logs/2026-02-14_15-20-vscode-claude-opus-4.6.md#L600)
- [2026-02-14_15-20-vscode-claude-opus-4.6.md:604](../agent-logs/2026-02-14_15-20-vscode-claude-opus-4.6.md#L604)
- [2026-02-14_15-20-vscode-claude-opus-4.6.md:882](../agent-logs/2026-02-14_15-20-vscode-claude-opus-4.6.md#L882)
- [2026-02-14_20-25-vscode-claude-opus-4.6.md:256](../agent-logs/2026-02-14_20-25-vscode-claude-opus-4.6.md#L256)
- [2026-02-14_20-25-vscode-claude-opus-4.6.md:258](../agent-logs/2026-02-14_20-25-vscode-claude-opus-4.6.md#L258)
- [bindings.rs:11](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L11)
- [bindings.rs:626](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L626)
- [bindings.rs:932](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L932)
- [bindings.rs:937](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L937)
- [bindings.rs:968](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L968)
- [bindings.rs:1221](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1221)
- [bindings.rs:1232](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1232)
- [bindings.rs:1266](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1266)
- [bindings.rs:1277](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1277)
- [bindings.rs:1317](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1317)
- [bindings.rs:2959](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2959)
- [bindings.rs:2969](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2969)
- [bindings.rs:2996](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2996)
- [bindings.rs:3030](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3030)
- [bindings.rs:3040](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3040)
- [bindings.rs:3135](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3135)
- [bindings.rs:3589](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3589)
- [2026-02-22_09-11-vscode-claude-opus-4.6.md:1](../agent-logs/2026-02-22_09-11-vscode-claude-opus-4.6.md#L1)
- [2026-02-22_09-11-vscode-claude-opus-4.6.md:44](../agent-logs/2026-02-22_09-11-vscode-claude-opus-4.6.md#L44)
- [2026-02-22_09-11-vscode-claude-opus-4.6.md:193](../agent-logs/2026-02-22_09-11-vscode-claude-opus-4.6.md#L193)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:314](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L314)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:339](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L339)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:353](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L353)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:696](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L696)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:740](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L740)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:742](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L742)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:767](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L767)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:772](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L772)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:848](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L848)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:881](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L881)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:1163](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L1163)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:1171](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L1171)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:1189](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L1189)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:1191](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L1191)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:1193](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L1193)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:1406](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L1406)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:1446](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L1446)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:1464](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L1464)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:1664](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L1664)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:1746](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L1746)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:2402](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L2402)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:2414](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L2414)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:2416](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L2416)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:2422](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L2422)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:2446](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L2446)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:2463](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L2463)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:2474](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L2474)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:2500](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L2500)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:2511](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L2511)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:2535](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L2535)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:2570](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L2570)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:2610](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L2610)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:2636](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L2636)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:2646](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L2646)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:2656](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L2656)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:2732](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L2732)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:2738](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L2738)
- [2026-02-19_21-30-vscode-claude-opus-4.6.md:2749](../agent-logs/2026-02-19_21-30-vscode-claude-opus-4.6.md#L2749)
- [config.rs:73](../crates/opencascade-binding-generator/src/config.rs#L73)
- [config.rs:79](../crates/opencascade-binding-generator/src/config.rs#L79)
- [config.rs:89](../crates/opencascade-binding-generator/src/config.rs#L89)
- [2026-02-02_19-59_vscode-gemini-3-pro-preview.md:1442](../agent-logs/2026-02-02_19-59_vscode-gemini-3-pro-preview.md#L1442)
- [2026-02-02_19-59_vscode-gemini-3-pro-preview.md:1569](../agent-logs/2026-02-02_19-59_vscode-gemini-3-pro-preview.md#L1569)
- [2026-02-02_19-59_vscode-gemini-3-pro-preview.md:1573](../agent-logs/2026-02-02_19-59_vscode-gemini-3-pro-preview.md#L1573)
- [2026-02-02_19-59_vscode-gemini-3-pro-preview.md:1783](../agent-logs/2026-02-02_19-59_vscode-gemini-3-pro-preview.md#L1783)
- [2026-02-02_19-59_vscode-gemini-3-pro-preview.md:2055](../agent-logs/2026-02-02_19-59_vscode-gemini-3-pro-preview.md#L2055)
- [type_mapping.rs:38](../crates/opencascade-binding-generator/src/type_mapping.rs#L38)
- [type_mapping.rs:137](../crates/opencascade-binding-generator/src/type_mapping.rs#L137)
- [type_mapping.rs:146](../crates/opencascade-binding-generator/src/type_mapping.rs#L146)
- [type_mapping.rs:169](../crates/opencascade-binding-generator/src/type_mapping.rs#L169)
- [type_mapping.rs:178](../crates/opencascade-binding-generator/src/type_mapping.rs#L178)
- [type_mapping.rs:187](../crates/opencascade-binding-generator/src/type_mapping.rs#L187)
- [type_mapping.rs:236](../crates/opencascade-binding-generator/src/type_mapping.rs#L236)
- [type_mapping.rs:252](../crates/opencascade-binding-generator/src/type_mapping.rs#L252)
- [type_mapping.rs:264](../crates/opencascade-binding-generator/src/type_mapping.rs#L264)
- [type_mapping.rs:455](../crates/opencascade-binding-generator/src/type_mapping.rs#L455)
- [type_mapping.rs:497](../crates/opencascade-binding-generator/src/type_mapping.rs#L497)
- [type_mapping.rs:528](../crates/opencascade-binding-generator/src/type_mapping.rs#L528)
- [type_mapping.rs:537](../crates/opencascade-binding-generator/src/type_mapping.rs#L537)
- [type_mapping.rs:556](../crates/opencascade-binding-generator/src/type_mapping.rs#L556)
- [type_mapping.rs:573](../crates/opencascade-binding-generator/src/type_mapping.rs#L573)
- [type_mapping.rs:588](../crates/opencascade-binding-generator/src/type_mapping.rs#L588)
- [type_mapping.rs:589](../crates/opencascade-binding-generator/src/type_mapping.rs#L589)
- [type_mapping.rs:590](../crates/opencascade-binding-generator/src/type_mapping.rs#L590)
- [type_mapping.rs:596](../crates/opencascade-binding-generator/src/type_mapping.rs#L596)
- [type_mapping.rs:601](../crates/opencascade-binding-generator/src/type_mapping.rs#L601)
- [2026-02-20_21-45-vscode-claude-opus-4.6.md:33](../agent-logs/2026-02-20_21-45-vscode-claude-opus-4.6.md#L33)
- [2026-02-20_21-45-vscode-claude-opus-4.6.md:35](../agent-logs/2026-02-20_21-45-vscode-claude-opus-4.6.md#L35)
- [README.md:448](../crates/opencascade-binding-generator/README.md#L448)
- [claude-conversation-2026-02-13-a06f7def.md:363](../agent-logs/claude-conversation-2026-02-13-a06f7def.md#L363)
- [claude-conversation-2026-02-13-a06f7def.md:487](../agent-logs/claude-conversation-2026-02-13-a06f7def.md#L487)
- [claude-conversation-2026-02-05-0e611961.md:806](../agent-logs/claude-conversation-2026-02-05-0e611961.md#L806)
- [claude-conversation-2026-02-05-0e611961.md:807](../agent-logs/claude-conversation-2026-02-05-0e611961.md#L807)
- [claude-conversation-2026-02-05-0e611961.md:2148](../agent-logs/claude-conversation-2026-02-05-0e611961.md#L2148)
- [claude-conversation-2026-02-05-0e611961.md:2149](../agent-logs/claude-conversation-2026-02-05-0e611961.md#L2149)
- [claude-conversation-2026-02-05-0e611961.md:4631](../agent-logs/claude-conversation-2026-02-05-0e611961.md#L4631)
- [claude-conversation-2026-02-05-0e611961.md:4773](../agent-logs/claude-conversation-2026-02-05-0e611961.md#L4773)
- [claude-conversation-2026-02-05-0e611961.md:4774](../agent-logs/claude-conversation-2026-02-05-0e611961.md#L4774)
- [claude-conversation-2026-02-05-0e611961.md:4793](../agent-logs/claude-conversation-2026-02-05-0e611961.md#L4793)
- [claude-conversation-2026-02-05-0e611961.md:4794](../agent-logs/claude-conversation-2026-02-05-0e611961.md#L4794)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:33](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L33)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:37](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L37)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:72](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L72)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:107](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L107)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:144](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L144)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:150](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L150)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:172](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L172)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:222](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L222)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:247](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L247)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:300](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L300)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:302](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L302)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:394](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L394)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:516](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L516)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:525](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L525)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:534](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L534)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:605](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L605)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:1210](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L1210)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:1212](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L1212)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:1221](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L1221)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:1223](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L1223)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:1239](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L1239)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:1240](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L1240)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:1299](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L1299)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:1405](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L1405)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:1409](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L1409)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:1425](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L1425)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:1435](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L1435)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:1461](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L1461)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:1481](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L1481)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:1489](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L1489)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:1491](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L1491)
- [2026-02-21_07-50-vscode-claude-opus-4.6.md:1645](../agent-logs/2026-02-21_07-50-vscode-claude-opus-4.6.md#L1645)
- [2026-02-18_19-00-vscode-claude-opus-4.6.md:425](../agent-logs/2026-02-18_19-00-vscode-claude-opus-4.6.md#L425)
- [2026-02-18_19-00-vscode-claude-opus-4.6.md:454](../agent-logs/2026-02-18_19-00-vscode-claude-opus-4.6.md#L454)
- [2026-02-18_19-00-vscode-claude-opus-4.6.md:459](../agent-logs/2026-02-18_19-00-vscode-claude-opus-4.6.md#L459)
- [2026-02-18_19-00-vscode-claude-opus-4.6.md:684](../agent-logs/2026-02-18_19-00-vscode-claude-opus-4.6.md#L684)
- [2026-02-18_19-00-vscode-claude-opus-4.6.md:1700](../agent-logs/2026-02-18_19-00-vscode-claude-opus-4.6.md#L1700)
- [2026-02-18_19-00-vscode-claude-opus-4.6.md:1740](../agent-logs/2026-02-18_19-00-vscode-claude-opus-4.6.md#L1740)
- [2026-02-18_19-00-vscode-claude-opus-4.6.md:1838](../agent-logs/2026-02-18_19-00-vscode-claude-opus-4.6.md#L1838)
- [2026-02-18_19-00-vscode-claude-opus-4.6.md:1852](../agent-logs/2026-02-18_19-00-vscode-claude-opus-4.6.md#L1852)
- [2026-02-18_19-00-vscode-claude-opus-4.6.md:1854](../agent-logs/2026-02-18_19-00-vscode-claude-opus-4.6.md#L1854)
</details>

Read [type_mapping.rs](../crates/opencascade-binding-generator/src/type_mapping.rs#1-1), lines 1 to 100

Read [content.txt](/Users/sethml/Library/Application Support/Code - Insiders/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/ea54976d-0c4a-4387-8833-79fd31749c73/toolu_01Tx9BEjMc3Ne6TGrj4DCgjT__vscode-1771913780876/content.txt#1-1), lines 1 to 300

Now let me find how the parser resolves `Aspect_RenderingContext` — specifically, how typedef resolution works for types that resolve to `void*` vs platform-specific pointers.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -rn 'Aspect_RenderingContext\|opaque_type\|opaque_types\|type_overrides\|platform_opaque\|platform.indep\|void_ptr_typedef' crates/opencascade-binding-generator/src/ | head -30
```
<details>
<summary>Output (13 lines)</summary>

```
crates/opencascade-binding-generator/src/config.rs:89:    pub occt_alias_type_overrides: std::collections::HashMap<String, String>,
crates/opencascade-binding-generator/src/codegen/rust.rs:173:    let (opaque_type_decls, nested_types) = generate_opaque_declarations(
crates/opencascade-binding-generator/src/codegen/rust.rs:284:    if !opaque_type_decls.is_empty() {
crates/opencascade-binding-generator/src/codegen/rust.rs:289:        out.push_str(&opaque_type_decls);
crates/opencascade-binding-generator/src/main.rs:105:    occt_alias_type_overrides: HashMap<String, String>,
crates/opencascade-binding-generator/src/main.rs:154:        let occt_alias_overrides = cfg.occt_alias_type_overrides;
crates/opencascade-binding-generator/src/main.rs:164:            occt_alias_type_overrides: occt_alias_overrides,
crates/opencascade-binding-generator/src/main.rs:176:            occt_alias_type_overrides: HashMap::new(),
crates/opencascade-binding-generator/src/main.rs:191:        occt_alias_type_overrides,
crates/opencascade-binding-generator/src/main.rs:257:        &occt_alias_type_overrides,
crates/opencascade-binding-generator/src/parser.rs:379:fn set_occt_alias_type_overrides(overrides: &HashMap<String, String>) {
crates/opencascade-binding-generator/src/parser.rs:399:    occt_alias_type_overrides: &HashMap<String, String>,
crates/opencascade-binding-generator/src/parser.rs:407:    set_occt_alias_type_overrides(occt_alias_type_overrides);
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -rn 'opaque_declarations\|generate_opaque\|referenced_types\|unknown_type\|unknown type' crates/opencascade-binding-generator/src/ | head -30
```
<details>
<summary>Output (23 lines)</summary>

```
crates/opencascade-binding-generator/src/config.rs:73:    /// aren't skipped as "unknown type".
crates/opencascade-binding-generator/src/config.rs:79:    /// as opaque types so methods using them aren't skipped as "unknown type".
crates/opencascade-binding-generator/src/codegen/rust.rs:41:pub fn collect_referenced_types(
crates/opencascade-binding-generator/src/codegen/rust.rs:172:    let collected_types = collect_referenced_types(all_classes, all_functions, handle_able_classes);
crates/opencascade-binding-generator/src/codegen/rust.rs:173:    let (opaque_type_decls, nested_types) = generate_opaque_declarations(
crates/opencascade-binding-generator/src/codegen/rust.rs:464:fn generate_opaque_declarations(
crates/opencascade-binding-generator/src/codegen/bindings.rs:497:fn type_uses_unknown_type(ty: &Type, ctx: &TypeContext) -> bool {
crates/opencascade-binding-generator/src/codegen/bindings.rs:932:/// Check parameters for unknown types, nullable pointer inner types, and class pointer inner types.
crates/opencascade-binding-generator/src/codegen/bindings.rs:936:    if let Some(p) = params.iter().find(|p| type_uses_unknown_type(&p.ty, ctx)) {
crates/opencascade-binding-generator/src/codegen/bindings.rs:937:        return Err(format!("param '{}' uses unknown type '{}'", p.name, p.ty.to_cpp_string()));
crates/opencascade-binding-generator/src/codegen/bindings.rs:943:                Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_type(inner, ctx),
crates/opencascade-binding-generator/src/codegen/bindings.rs:953:    // We check all_classes directly (not type_uses_unknown_type) because nested types
crates/opencascade-binding-generator/src/codegen/bindings.rs:954:    // like Parent::Nested are considered "known" by type_uses_unknown_type if the parent
crates/opencascade-binding-generator/src/codegen/bindings.rs:968:/// Check a return type for unknown types, CppDeletable, class pointer inner types, and &mut enum.
crates/opencascade-binding-generator/src/codegen/bindings.rs:971:    if type_uses_unknown_type(ret, ctx) {
crates/opencascade-binding-generator/src/codegen/bindings.rs:2291:        .any(|p| type_uses_unknown_type(&p.ty, ctx))
crates/opencascade-binding-generator/src/codegen/bindings.rs:2867:                let uses_unknown_type = resolved_method.params.iter().any(|p| {
crates/opencascade-binding-generator/src/codegen/bindings.rs:2887:                if uses_unknown_type {
crates/opencascade-binding-generator/src/codegen/bindings.rs:3583:            p.ty.original.is_unbindable() || type_uses_unknown_type(&p.ty.original, &ffi_ctx)
crates/opencascade-binding-generator/src/codegen/bindings.rs:3589:                format!("param '{}' uses unknown type '{}'", p.name, p.ty.original.to_cpp_string())
crates/opencascade-binding-generator/src/codegen/bindings.rs:3617:            if type_uses_unknown_type(&ret.original, &ffi_ctx) {
crates/opencascade-binding-generator/src/main.rs:1098:    let collected_types = codegen::ru
... (truncated, 3227 chars total)
```
</details>

Now let me read the `type_uses_unknown_type` function and the config structure to understand the full picture:

<details>
<summary>Reading lines 490-570 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L490-L571">bindings.rs</a></summary>

```
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
        // Std bitmask types are known (mapped to their FFI integer type)
        Type::Class(name) if std_bitmask_ffi_type(name).is_some() => return false,
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => {
            if let Type::Class(name) = inner.as_ref() {
                if ctx.all_enums.contains(name) {
                    return false;
                }
                if std_bitmask_ffi_type(name).is_some() {
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

/// Check if a method has by-value class or handle parameters (not enums, not bitmask types).
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
```
</details>

<details>
<summary>Reading lines 60-130 of <a href="../crates/opencascade-binding-generator/src/config.rs#L60-L131">config.rs</a></summary>

```
    #[serde(default)]
    pub ambiguous_methods: Vec<String>,

    /// Non-allocatable classes: suppress constructors and destructors (including
    /// CppDeletable/ToOwned), but keep the opaque struct and all methods.
    /// Use for classes with protected/hidden operator new/delete that can still
    /// be used via pointers obtained from other APIs.
    /// For nested types, use the C++ qualified name: "Parent::Nested".
    #[serde(default)]
    pub non_allocatable_classes: Vec<String>,

    /// Opaque types defined in manual/ files but referenced by auto-generated bindings.
    /// The generator adds these to the known class set so methods using them
    /// aren't skipped as "unknown type".
    /// Format: `TypeName = { header = "Header.hxx" }`
    #[serde(default)]
    pub manual_types: std::collections::HashMap<String, ManualType>,

    /// Template instantiation aliases: declare specific C++ template instantiations
    /// as opaque types so methods using them aren't skipped as "unknown type".
    /// The generator creates C++ typedefs, Rust opaque types, and Handle wrappers.
    /// Format: `"Template<Args>" = { header = "Header.hxx", module = "Module", handle = true }`
    #[serde(default)]
    pub template_instantiations: std::collections::HashMap<String, TemplateInstantiation>,

    /// Canonical OCCT type spellings that should be rewritten to public aliases during parsing.
    /// This is used when clang reports internal template spellings that differ from public typedef names.
    /// Format: "CanonicalTypeSpelling" = "PublicAliasName"
    #[serde(default)]
    pub occt_alias_type_overrides: std::collections::HashMap<String, String>,
}

/// A manually-defined opaque type referenced by auto-generated bindings.
#[derive(Debug, Deserialize)]
pub struct ManualType {
    /// The C++ header that defines this type (for wrappers.cpp includes).
    pub header: String,
}

/// A C++ template instantiation declared as an opaque Rust type.
#[derive(Debug, Deserialize)]
pub struct TemplateInstantiation {
    /// The C++ header that defines the template (for wrappers.cpp includes).
    pub header: String,
    /// The OCCT module this type belongs to (for re-export file placement).
    pub module: String,
    /// Whether this instantiation inherits from Standard_Transient and needs Handle support.
    #[serde(default)]
    pub handle: bool,
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
```
</details>

Now let me look at how the parser resolves typedef types — specifically how `void*` vs `NSOpenGLContext*` would flow through:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n 'type_uses_unknown_class\|type_uses_unknown_handle' crates/opencascade-binding-generator/src/type_mapping.rs | head -20
```
<details>
<summary>Output (8 lines)</summary>

```
399:/// Shared logic for type_uses_unknown_class and type_uses_unknown_handle.
425:pub fn type_uses_unknown_class(ty: &Type, all_classes: &std::collections::HashSet<String>) -> bool {
429:        Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_class(inner, all_classes),
430:        Type::FixedArray(inner, _) => type_uses_unknown_class(inner, all_classes),
436:/// This is more strict than type_uses_unknown_class - it checks that Handle types are for
438:pub fn type_uses_unknown_handle(
447:            type_uses_unknown_handle(inner, all_classes, handle_able_classes)
449:        Type::FixedArray(inner, _) => type_uses_unknown_handle(inner, all_classes, handle_able_classes),
```
</details>

<details>
<summary>Reading lines 395-460 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L395-L461">type_mapping.rs</a></summary>

```

/// Check if a type references an unknown class/handle
/// Returns true if the type uses a Handle or Class that is not in all_classes
/// Check if a class name is unknown (not in the known type set).
/// Shared logic for type_uses_unknown_class and type_uses_unknown_handle.
fn is_class_name_unknown(class_name: &str, all_classes: &std::collections::HashSet<String>) -> bool {
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
    // Standard library bitmask types (e.g., std::ios_base::openmode) are known
    if crate::model::std_bitmask_ffi_type(class_name).is_some() {
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

pub fn type_uses_unknown_class(ty: &Type, all_classes: &std::collections::HashSet<String>) -> bool {
    match ty {
        Type::Handle(class_name) => !all_classes.contains(class_name),
        Type::Class(class_name) => is_class_name_unknown(class_name, all_classes),
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => type_uses_unknown_class(inner, all_classes),
        Type::FixedArray(inner, _) => type_uses_unknown_class(inner, all_classes),
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
        Type::Class(class_name) => is_class_name_unknown(class_name, all_classes),
        Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
            type_uses_unknown_handle(inner, all_classes, handle_able_classes)
        }
        Type::FixedArray(inner, _) => type_uses_unknown_handle(inner, all_classes, handle_able_classes),
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
```
</details>

Now I need to understand how typedef resolution works in the parser — when it encounters `Aspect_RenderingContext` (which is `void*` on Linux but `NSOpenGLContext*` on macOS), how does it resolve the type?

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n 'resolve_type\|parse_type\|fn parse_method_param\|fn parse_param\|get_canonical_type\|get_result_type\|is_void_type_name\|Standard_Address' crates/opencascade-binding-generator/src/parser.rs | head -30
```
<details>
<summary>Output (25 lines)</summary>

```
212:                        let canonical = underlying.get_canonical_type().get_display_name();
533:    // This must happen before class/method parsing so parse_type() can
538:    // This must happen before class/method parsing so parse_type() can resolve
1179:    let result_type = entity.get_result_type()?;
1180:    let return_type = parse_type(&result_type);
1190:                ty: parse_type(&param_type),
1404:        (parse_type(&element_type), Some(size))
1406:        (parse_type(&field_type), None)
1452:fn parse_params(entity: &Entity) -> Vec<Param> {
1479:                ty: parse_type(&param_type),
1609:    let result_type = entity.get_result_type()?;
1616:    Some(parse_type(&result_type))
1629:fn parse_type(clang_type: &clang::Type) -> Type {
1685:    let canonical = clang_type.get_canonical_type();
1773:                        is_c_primitive_type_kind(pt.get_canonical_type().get_kind())
1809:                    let elem_ty = parse_type(&elem);
1839:            let inner = parse_type(&pointee);
1851:            let inner = parse_type(&pointee);
1860:            let inner = parse_type(&pointee);
1876:            let inner = parse_type(&elem);
1885:            let inner = parse_type(&elem);
1990:                    let inner = parse_type(&pointee);
2073:        // Standard_Address is void* — bound as *mut c_void in unsafe functions.
2074:        // Represented as Type::Class("Standard_Address") so is_void_ptr() can detect it.
2075:        "Standard_Address" => Some(Type::Class("Standard_Address".to_string())),
```
</details>

<details>
<summary>Reading lines 1629-2100 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L1629-L2101">parser.rs</a></summary>

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

    // Check for fixed-width integer types BEFORE canonical resolution.
    // On LP64 Linux, uint64_t is a typedef for "unsigned long", but on macOS it's
    // "unsigned long long". We want uint64_t to always map to u64/i64 regardless
    // of platform, rather than letting canonical resolution turn it into c_ulong.
    // (We intentionally do NOT remap "unsigned long" itself, since it's 32 bits
    // on some platforms.)
    if let Some(fixed) = map_fixed_width_integer(trimmed_spelling) {
        return fixed;
    }

    // Check if this is a typedef to size_t or a fixed-width integer by examining
    // the declaration. This catches cases where get_display_name() returns the
    // canonical type but the declaration still carries the original typedef name.
    if let Some(decl) = clang_type.get_declaration() {
        if let Some(decl_name) = decl.get_name() {
            if decl_name == "size_t" || decl_name.ends_with("_Size") {
                return Type::Usize;
            }
            if let Some(fixed) = map_fixed_width_integer(&decl_name) {
                return fixed;
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
            return class_type(&typedef_name);
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
            // The underlying
... (truncated, 22977 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code - Insiders/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/ea54976d-0c4a-4387-8833-79fd31749c73/toolu_01QkZZb2jWMyF6BNucrRRDiV__vscode-1771913780888/content.txt)

**Todo list:**
- ✅ Examine git diff for platform differences
- ✅ Find Aspect_RenderingContext in OCCT headers
- ✅ Find how binding generator handles typedefs
- 🔄 Make Aspect_RenderingContext opaque
- ⬜ Regenerate and verify bindings

> 💭 Now I understand the full picture:
> 
> 1. `Aspect_RenderingContext` is a typedef that resolves to:
>    - `void*` on Linux
>    - `NSOpenGLContext*` on macOS
>    - `EAGLContext*` on iOS
> 
> 2. On Linux, `parse_type()` sees `Aspect_RenderingContext` → canonical is `void*` → it resolves through the late-stage pointer typedef handling (line 1985) to `Type::MutPtr(Type::Class("void"))` which becomes `*mut std::ffi::c_void`.
>    
> 3. On macOS, the same typedef resolves to `NSOpenGLContext*`, which `parse_type()` treats as an unknown type and skips methods for. The real issue is that I need to handle this at the typedef name level rather than waiting for the canonical type to resolve, since the canonical form differs per platform. I should intercept `Aspect_RenderingContext` directly in `map_standard_type()` as an opaque type, similar to how `Standard_Address` is handled.
> 
> Looking at the actual flow: when `parse_type()` encounters `Aspect_RenderingContext`, it checks `map_standard_type()` first (which returns None), then falls through to canonical resolution. On macOS that gives `NSOpenGLContext*`, which eventually becomes `MutPtr(Class("NSOpenGLContext"))` — but since `NSOpenGLContext` isn't in `all_classes`, it's treated as unknown and methods get skipped. On Linux the canonical is `void*` instead, so the same problem occurs with different underlying types.
> 
> The user actually wants an opaque struct type that works cross-platform, not a void pointer. So I should map `Aspect_RenderingContext` in `map_standard_type()` to return an opaque type definition that both platforms can use consistently. I'm seeing how `Aspect_RenderingContext` is actually used as a value parameter in these method signatures, where it's already a pointer type from the typedef. The question is whether to map it to a simple `*mut c_void` for consistency across platforms, or create a proper opaque struct in the Rust bindings to match what the user expects. the same regardless of platform. It's platform-independent, preserves type safety beyond just `void*`, and prevents methods from being skipped. I should add `Aspect_RenderingContext` to `map_standard_type()` to return it as its own class type, then register it in `manual_types` to avoid unknown type filtering. The C++ parameter is a const pointer, so I need to think through how that translates...
> 
> When clang processes `Aspect_RenderingContext`, it sees the typedef resolving to a pointer type. If I intercept it in `map_standard_type()`, I could represent it as an opaque pointer class or just treat it as a class type directly. The cleanest approach is probably to return `Type::Class("Aspect_RenderingContext")` and handle it as a special opaque type.
> 
> Looking at how `Standard_Address` works, it maps to a class type in `map_standard_type()`, then in `type_mapping.rs` there's a special case that converts it to a raw `c_void` pointer in Rust. I should follow the same pattern for `Aspect_RenderingContext`. pattern is: intercept the typedef name in `map_standard_type()` to return a Type::Class with a known name, and handle that name in the type mapping.
> 
> I could add a special case for `Aspect_RenderingContext` that maps it to `*mut std::ffi::c_void` across all platforms, intercepting it before canonical resolution to avoid platform differences. But the user wants an opaque type, so maybe I should map it to an opaque `Aspect_RenderingContext` struct instead and treat it like a pointer on the Rust side. The problem is that if the C++ wrapper emits this type directly, it would need the platform-specific type available—like `NSOpenGLContext*` on macOS—which creates a dependency issue.
> 
> Actually, the key insight is that the C++ wrapper file won't have any problem because the OCCT headers already define `Aspect_RenderingContext` appropriately for each platform. The real issue is just on the Rust side where I need to figure out what type to generate. Since `Aspect_RenderingContext` is already a pointer type (a `void*`), I should intercept it and map it directly to `*mut c_void` without wrapping it in another pointer layer.
> 
> I can follow the same pattern as `Standard_Address` — treat `Aspect_RenderingContext` as a class type that maps to an opaque pointer in the FFI layer.
> 
> Now I need to check the `is_void_type_name` function since it determines which types are recognized as void-like, which affects how `is_class_name_unknown` classifies them.

Now I understand the full picture. On Linux, `Aspect_RenderingContext` is `typedef void*` which resolves cleanly to `*mut c_void`. On macOS, it's `typedef NSOpenGLContext*` which becomes an unknown type. The fix follows the same pattern as `Standard_Address` (which is also `typedef void*`): intercept the typedef name before canonical resolution.

Let me find `is_void_type_name` and the C++ wrapper generation to make sure the fix is complete:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n 'is_void_type_name\|is_void_ptr' crates/opencascade-binding-generator/src/model.rs | head -20
```
<details>
<summary>Output (6 lines)</summary>

```
420:pub fn is_void_type_name(name: &str) -> bool {
440:    name != "char" && !is_void_type_name(name) && std_bitmask_ffi_type(name).is_none()
589:    pub fn is_void_ptr(&self) -> bool {
591:            Type::Class(name) => is_void_type_name(name),
593:                inner.is_void_ptr()
725:        self.is_void_ptr() || self.is_raw_ptr()
```
</details>

<details>
<summary>Reading lines 418-435 of <a href="../crates/opencascade-binding-generator/src/model.rs#L418-L436">model.rs</a></summary>

```
/// Standard_Address is a typedef for void*, and "void" is the parsed
/// form of literal void* parameters.
pub fn is_void_type_name(name: &str) -> bool {
    name == "Standard_Address" || name == "void"
}

/// Check if a class name represents a C++ standard library bitmask type.
/// These types are integer-compatible bitmasks that need explicit `static_cast`
/// in C++ wrapper code because on some platforms (e.g., Linux/libstdc++),
/// they are proper enum types that don't implicitly convert from integers.
/// Returns the corresponding FFI integer type if it is a bitmask type.
pub fn std_bitmask_ffi_type(name: &str) -> Option<Type> {
    match name {
        "std::ios_base::openmode" => Some(Type::U32),
        _ => None,
    }
}

```
</details>

Now let me check how the C++ wrapper handles `Standard_Address`/void* types to understand what C++ type it uses in the wrapper function signature:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n 'Standard_Address\|void_type_name\|is_void_ptr\|cpp_type_for' crates/opencascade-binding-generator/src/codegen/cpp.rs | head -30
```
```
93:                // Also skip Standard_Address which is defined in Standard_TypeDef.hxx, not its own file
96:                    "void" | "size_t" | "Standard_Address"
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n 'to_cpp_string\|fn cpp_param_type\|fn ffi_param_type\|fn wrapper_param' crates/opencascade-binding-generator/src/model.rs | head -20
```
<details>
<summary>Output (13 lines)</summary>

```
740:                    format!("{} const*", elem.to_cpp_string())
742:                    format!("{} const*", inner.to_cpp_string())
747:                    format!("{}*", elem.to_cpp_string())
749:                    format!("{}*", inner.to_cpp_string())
755:            _ => self.to_cpp_string(),
760:    pub fn to_cpp_string(&self) -> String {
784:            Type::ConstRef(inner) => format!("{} const&", inner.to_cpp_string()),
785:            Type::MutRef(inner) => format!("{}&", inner.to_cpp_string()),
786:            Type::RValueRef(inner) => format!("{}&&", inner.to_cpp_string()),
788:            Type::ConstPtr(inner) => format!("{} const*", inner.to_cpp_string()),
789:            Type::MutPtr(inner) => format!("{}*", inner.to_cpp_string()),
791:            Type::FixedArray(inner, size) => format!("{}[{}]", inner.to_cpp_string(), size),
873:            format!("/* {} */", self.to_cpp_string())
```
</details>

<details>
<summary>Reading lines 755-800 of <a href="../crates/opencascade-binding-generator/src/model.rs#L755-L801">model.rs</a></summary>

```
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
            Type::FixedArray(inner, size) => format!("{}[{}]", inner.to_cpp_string(), size),
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

Now I have the full picture. The fix is straightforward, following the exact pattern of `Standard_Address`:

1. In `parser.rs` `map_standard_type()`: Add `"Aspect_RenderingContext"` → `Type::Class("Aspect_RenderingContext")`
2. In `model.rs` `is_void_type_name()`: Add `"Aspect_RenderingContext"` so it's treated as known
3. In `type_mapping.rs` `map_type_to_rust()` and `map_type_in_context()`: Add special case for `"Aspect_RenderingContext"` → `*mut std::ffi::c_void`
4. In `codegen/cpp.rs`: Add it to the skip-include list (it's defined via OCCT headers, not its own standalone file)

Let me also check the C++ wrapper side to see how `Standard_Address` appears in C++ output:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n 'Standard_Address' crates/opencascade-sys/generated/wrappers.cpp | head -5
```
<details>
<summary>Output (5 lines)</summary>

```
29157:extern "C" Standard_Address* Adaptor3d_TopolTool_edge(const Adaptor3d_TopolTool* self_) {
29158:    return new Standard_Address(self_->Edge());
49639:extern "C" BRepApprox_TheMultiLineOfApprox* BRepApprox_TheMultiLineOfApprox_ctor_handlebrepapproxapproxline_address_int2_bool2_real7_bool_int2(Handle(BRepApprox_ApproxLine) const* line, Standard_Address PtrSvSurfaces, int32_t NbP3d, int32_t NbP2d, bool ApproxU1V1, bool ApproxU2V2, double xo, double yo, double zo, double u1o, double v1o, double u2o, double v2o, bool P2DOnFirst, int32_t IndMin, int32_t IndMax) {
66551:extern "C" Standard_Address* BRepPrimAPI_MakeCone_one_axis(BRepPrimAPI_MakeCone* self_) {
66552:    return new Standard_Address(self_->OneAxis());
```
</details>

I see — `Standard_Address` appears as its original C++ typedef in the wrapper. That's exactly what `to_cpp_string()` does for `Type::Class(name)` — it outputs the name directly. So if I map `Aspect_RenderingContext` the same way, it will emit `Aspect_RenderingContext` in C++, which is correct because OCCT headers define it appropriately per platform.

Now let me check line 96 of cpp.rs more carefully:

<details>
<summary>Reading lines 85-110 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L85-L111">cpp.rs</a></summary>

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
                            if known_headers.is_empty() || known_headers.contains(&header) {
                                headers.insert(header);
                            }
                        }
                        return;
```
</details>

Now I have everything I need. Let me implement the fix:Now let me make all the edits. I need to modify 4 files:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/parser.rs#L2076-L2080">parser.rs +4</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L421-L422">model.rs -1+1</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L96-L97">cpp.rs -1+1</a>  
**Failing: <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L231-L241">type_mapping.rs +10</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/parser.rs",
      "lineHashes": "2075:gp",
      "insertAfter": true,
      "content": "        // Aspect_RenderingContext is a platform-dependent typedef:\n        // void* on Linux, NSOpenGLContext* on macOS, EAGLContext* on iOS.\n        // Intercept before canonical resolution to get consistent cross-platform output.\n        \"Aspect_RenderingContext\" => Some(Type::Class(\"Aspect_RenderingContext\".to_string())),"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "421:ls",
      "content": "    name == \"Standard_Address\" || name == \"void\" || name == \"Aspect_RenderingContext\""
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "230:gp",
      "insertAfter": true,
      "content": "        Type::Class(class_name) if class_name == \"Aspect_RenderingContext\" => {\n            // Aspect_RenderingContext is a platform-dependent pointer typedef\n            // (void* on Linux, NSOpenGLContext* on macOS). Map to *mut c_void for consistency.\n            RustTypeMapping {\n                rust_type: \"*mut std::ffi::c_void\".to_string(),\n                needs_unique_ptr: false,\n                needs_pin: false,\n                source_module: None,\n            }\n        }"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "96:bs",
      "content": "                    \"void\" | \"size_t\" | \"Standard_Address\" | \"Aspect_RenderingContext\""
    }
  ]
}
```
**Output:**
```
{
  "applied": 3,
  "failed": 1,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/parser.rs",
      "startLine": 2075,
      "endLine": 2079
    },
    {
      "file": "crates/opencascade-binding-generator/src/model.rs",
      "startLine": 421,
      "endLine": 421
    },
    {
      "file": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "startLine": 96,
      "endLine": 96
    }
  ],
  "errors": [
    {
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "230:gp",
      "error": "hash mismatch at line 230: file has 'nk', edit specified 'gp'"
    }
  ]
}
```
</details>

The type_mapping.rs edit failed due to hash mismatch. Let me read the correct area:

<details>
<summary>Reading lines 220-240 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L220-L241">type_mapping.rs</a></summary>

```
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
        Type::Class(class_name) if crate::model::std_bitmask_ffi_type(class_name).is_some() => {
            // Standard library bitmask types (e.g., std::ios_base::openmode)
            // mapped to their FFI integer type
            map_type_to_rust(&crate::model::std_bitmask_ffi_type(class_name).unwrap())
        }
        Type::Class(class_name) => {
            let source_module = extract_module_from_class(class_name);
            RustTypeMapping {
```
</details>

I see, I need to insert after the `Standard_Address` case. Let me find it:

<details>
<summary>Reading lines 200-225 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L200-L226">type_mapping.rs</a></summary>

```
                needs_unique_ptr: true,
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
        Type::Class(class_name) if class_name == "void" => {
            // C++ void type (pointee of void*) — just c_void, pointer wrapping handled by outer type
            RustTypeMapping {
                rust_type: "std::ffi::c_void".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: None,
            }
        }
        Type::Class(class_name) if class_name == "char" => {
            // C++ char resolved from canonical types (e.g., Standard_Character)
            // FFI supports c_char but not Rust's char (which is 4-byte Unicode)
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L214-L224">type_mapping.rs +10</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::Class(class_name) if class_name == \"Aspect_RenderingContext\" => {\n            // Aspect_RenderingContext is a platform-dependent pointer typedef\n            // (void* on Linux, NSOpenGLContext* on macOS). Map to *mut c_void for consistency.\n            RustTypeMapping {\n                rust_type: \"*mut std::ffi::c_void\".to_string(),\n                needs_unique_ptr: false,\n                needs_pin: false,\n                source_module: None,\n            }\n        }",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "insertAfter": true,
      "lineHashes": "213:cy"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/type_mapping.rs","startLine":213,"endLine":223}
```
</details>

Now I also need to add the same case in `map_type_in_context`:

<details>
<summary>Reading lines 465-500 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L465-L501">type_mapping.rs</a></summary>

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
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L485-L495">type_mapping.rs +10</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::Class(class_name) if class_name == \"Aspect_RenderingContext\" => {\n            // Aspect_RenderingContext is a platform-dependent pointer typedef\n            // (void* on Linux, NSOpenGLContext* on macOS). Map to *mut c_void for consistency.\n            RustTypeMapping {\n                rust_type: \"*mut std::ffi::c_void\".to_string(),\n                needs_unique_ptr: false,\n                needs_pin: false,\n                source_module: None,\n            }\n        }",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "insertAfter": true,
      "lineHashes": "484:cy"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/type_mapping.rs","startLine":484,"endLine":494}
```
</details>

Now let me verify the edits look correct: Now let me verify the edits look correct:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo check -p opencascade-binding-generator 2>&1 | tail -5
```
```
    Blocking waiting for file lock on build directory
    Checking opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 19.29s
```

Compiles cleanly. Now let me regenerate the bindings and build:

**Todo list:**
- ✅ Examine git diff for platform differences
- ✅ Find Aspect_RenderingContext in OCCT headers
- ✅ Find how binding generator handles typedefs
- ✅ Make Aspect_RenderingContext opaque
- 🔄 Regenerate and verify bindings

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff HEAD --stat -- crates/opencascade-sys/generated/
```
<details>
<summary>Output (6 lines)</summary>

```
 crates/opencascade-sys/generated/aspect.rs    | 8 ++++----
 crates/opencascade-sys/generated/ffi.rs       | 7 ++++++-
 crates/opencascade-sys/generated/graphic3d.rs | 2 +-
 crates/opencascade-sys/generated/v3d.rs       | 6 +++---
 crates/opencascade-sys/generated/wrappers.cpp | 4 ++--
 5 files changed, 16 insertions(+), 11 deletions(-)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff HEAD -- crates/opencascade-sys/generated/
```
<details>
<summary>Output (98 lines)</summary>

```
diff --git a/crates/opencascade-sys/generated/aspect.rs b/crates/opencascade-sys/generated/aspect.rs
index f28157a..af64a60 100644
--- a/crates/opencascade-sys/generated/aspect.rs
+++ b/crates/opencascade-sys/generated/aspect.rs
@@ -9092,8 +9092,8 @@ impl HandleAspectXRSession {
 
 pub use crate::ffi::{
     Aspect_FrustumLRBT_Standard_Real as FrustumLRBT_Standard_Real,
-    Aspect_FrustumLRBT_double as FrustumLRBT_double, Aspect_SequenceOfColor as SequenceOfColor,
-    Aspect_TouchMap as TouchMap, Aspect_TrackedDevicePoseArray as TrackedDevicePoseArray,
-    Aspect_XDisplay as XDisplay, Aspect_XRActionMap as XRActionMap,
-    Aspect_XVisualInfo as XVisualInfo,
+    Aspect_FrustumLRBT_double as FrustumLRBT_double, Aspect_RenderingContext as RenderingContext,
+    Aspect_SequenceOfColor as SequenceOfColor, Aspect_TouchMap as TouchMap,
+    Aspect_TrackedDevicePoseArray as TrackedDevicePoseArray, Aspect_XDisplay as XDisplay,
+    Aspect_XRActionMap as XRActionMap, Aspect_XVisualInfo as XVisualInfo,
 };
diff --git a/crates/opencascade-sys/generated/ffi.rs b/crates/opencascade-sys/generated/ffi.rs
index 03c8bba..78c1d20 100644
--- a/crates/opencascade-sys/generated/ffi.rs
+++ b/crates/opencascade-sys/generated/ffi.rs
@@ -35261,6 +35261,11 @@ pub struct Aspect_FrustumLRBT_double {
 }
 /// Referenced type from C++
 #[repr(C)]
+pub struct Aspect_RenderingContext {
+    _opaque: [u8; 0],
+}
+/// Referenced type from C++
+#[repr(C)]
 pub struct Aspect_SequenceOfColor {
     _opaque: [u8; 0],
 }
@@ -641518,7 +641523,7 @@ extern "C" {
     /// **Source:** `V3d_View.hxx`:58 - `V3d_View::DynamicType()`
     pub fn V3d_View_dynamic_type(self_: *const V3d_View) -> *const HandleStandardType;
     /// **Source:** `V3d_View.hxx`:76 - `V3d_View::SetWindow()`
-    pub fn V3d_View_set_window_handleaspectwindow_voidptr(
+    pub fn V3d_View_set_window_handleaspectwindow_renderingcontext(
         self_: *mut V3d_View,
         theWindow: *const HandleAspectWindow,
         theContext: *mut std::ffi::c_void,
diff --git a/crates/opencascade-sys/generated/graphic3d.rs b/crates/opencascade-sys/generated/graphic3d.rs
index 2d9971c..cb5ee90 100644
--- a/crates/opencascade-sys/generated/graphic3d.rs
+++ b/crates/opencascade-sys/generated/graphic3d.rs
@@ -17976,7 +17976,7 @@ impl CView {
         &mut self,
         theParentVIew: &crate::ffi::HandleGraphic3dCView,
         theWindow: &crate::ffi::HandleAspectWindow,
-        theContext: *mut std::ffi::c_void,
+        theContext: crate::ffi::Aspect_RenderingContext,
     ) {
         unsafe {
             crate::ffi::Graphic3d_CView_set_window(
diff --git a/crates/opencascade-sys/generated/v3d.rs b/crates/opencascade-sys/generated/v3d.rs
index d6a7543..1fa8a70 100644
--- a/crates/opencascade-sys/generated/v3d.rs
+++ b/crates/opencascade-sys/generated/v3d.rs
... (truncated, 5740 chars total)
```
</details>

The diff looks good. Now:
- `Aspect_RenderingContext` is a proper opaque struct in the Rust FFI
- The C++ wrappers use `Aspect_RenderingContext` (the OCCT typedef) which is correct per-platform
- Both `V3d_View::SetWindow` and `Graphic3d_CView::SetWindow` methods are now generated (previously skipped on macOS)
- The `NSOpenGLContext` opaque struct is no longer needed (it was macOS-only)

However, I notice an issue: in `crates/opencascade-sys/generated/graphic3d.rs`, the parameter type changed from `*mut std::ffi::c_void` to `crate::ffi::Aspect_RenderingContext` — but `Aspect_RenderingContext` is defined as a zero-size opaque struct. Since the C++ side treats it as a pointer, we need the Rust parameter to also be a pointer (`*mut Aspect_RenderingContext`), not the struct by value. Let me check if this is a problem with how the type mapping works...

Actually wait — looking at it more carefully: `Aspect_RenderingContext` IS a pointer type (it's `typedef void* Aspect_RenderingContext`). The C++ parameter is `Aspect_RenderingContext theContext` which is passing a pointer by value. The Rust side has `theContext: crate::ffi::Aspect_RenderingContext` — but this is a zero-size struct, not a pointer. This won't match the ABI.

The issue is that `Type::Class("Aspect_RenderingContext")` in the type mapping is treated as an opaque class that needs `*mut` wrapping via `needs_unique_ptr: true`. But in the FFI declaration in ffi.rs, it appears as `*mut std::ffi::c_void` (from `map_type_to_rust`), while the re-export module uses `map_type_in_context` which doesn't , the parameter type changed from `*mut std::ffi::c_void` to `crate::ffi::Aspect_RenderingContext` — but `Aspect_RenderingContext` is defined as a zero-size opaque struct. Since the C++ side treats it as a pointer, we need the Rust parameter to also be a pointer (`*mut Aspect_RenderingContext`), not the struct by value. Let me check if this is a problem with how the type mapping works...

Actually wait — looking at it more carefully: `Aspect_RenderingContext` IS a pointer type (it's `typedef void* Aspect_RenderingContext`). The C++ parameter is `Aspect_RenderingContext theContext` which is passing a pointer by value. The Rust side has `theContext: crate::ffi::Aspect_RenderingContext` — but this is a zero-size struct, not a pointer. This won't match the ABI.

The issue is that `Type::Class("Aspect_RenderingContext")` in the type mapping is treated as an opaque class that needs `*mut` wrapping via `needs_unique_ptr: true`. But in the FFI declaration in ffi.rs, it appears as `*mut std::ffi::c_void` (from `map_type_to_rust`), while the re-export module uses `map_type_in_context` which doesn't add the pointer wrapper.

Let me look more carefully at the ffi.rs output and graphic3d.rs output:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -A5 'Graphic3d_CView_set_window' crates/opencascade-sys/generated/ffi.rs
```
<details>
<summary>Output (6 lines)</summary>

```
    pub fn Graphic3d_CView_set_window(
        self_: *mut Graphic3d_CView,
        theParentVIew: *const HandleGraphic3dCView,
        theWindow: *const HandleAspectWindow,
        theContext: *mut std::ffi::c_void,
    );
```
</details>

OK so the ffi.rs has `*mut std::ffi::c_void` (from `map_type_to_rust` which explicitly maps `Aspect_RenderingContext` to that). But in graphic3d.rs (the re-export), it shows `theContext: crate::ffi::Aspect_RenderingContext` — that's the zero-size opaque struct, not a pointer. This is an ABI mismatch.

The problem is that `map_type_in_context` returns `needs_unique_ptr: true` for `Aspect_RenderingContext` (via the generic `Type::Class` arm), which means the re-export layer would normally wrap it with some pointer type. But looking at the actual generated code, it doesn't seem to be doing that.

Wait, actually, let me re-check. The `graphic3d.rs` shows `theContext: crate::ffi::Aspect_RenderingContext` — but this is the Rust wrapper method, not the FFI. The wrapper calls the FFI function. Let me look at the full context:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -B5 -A15 'fn set_window' crates/opencascade-sys/generated/graphic3d.rs | head -40
```
<details>
<summary>Output (20 lines)</summary>

```
    /// **Source:** `Graphic3d_CView.hxx`:283 - `Graphic3d_CView::SetWindow()`
    /// Creates and maps rendering window to the view.
    /// @param[in] theParentVIew parent view or NULL
    /// @param[in] theWindow the window
    /// @param[in] theContext the rendering context; if NULL the context will be created internally
    pub unsafe fn set_window(
        &mut self,
        theParentVIew: &crate::ffi::HandleGraphic3dCView,
        theWindow: &crate::ffi::HandleAspectWindow,
        theContext: crate::ffi::Aspect_RenderingContext,
    ) {
        unsafe {
            crate::ffi::Graphic3d_CView_set_window(
                self as *mut Self,
                theParentVIew,
                theWindow,
                theContext,
            )
        }
    }
```
</details>

There's a type mismatch: the FFI function expects `*mut std::ffi::c_void` but the wrapper passes `crate::ffi::Aspect_RenderingContext` (a zero-size struct) directly. This won't compile.

The root cause is that `Aspect_RenderingContext` should map to `*mut c_void` everywhere, not just in `map_type_to_rust`. The issue is that `map_type_in_context` falls through to the generic `Type::Class` arm which uses the struct name. I already added the special case there but let me verify the ordering is correct:

<details>
<summary>Reading lines 484-505 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L484-L506">type_mapping.rs</a></summary>

```
        }
        Type::Class(class_name) if class_name == "Aspect_RenderingContext" => {
            // Aspect_RenderingContext is a platform-dependent pointer typedef
            // (void* on Linux, NSOpenGLContext* on macOS). Map to *mut c_void for consistency.
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
```
</details>

Good, the `Aspect_RenderingContext` case is there in `map_type_in_context`. It should return `*mut std::ffi::c_void` with `needs_unique_ptr: false`. Let me try building to see if there's a compilation error:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

> 💭 As expected, there's a type mismatch. The FFI layer (ffi.rs) uses `*mut std::ffi::c_void` (from `map_type_to_rust`), but the wrapper layer (graphic3d.rs, v3d.rs) uses `Aspect_RenderingContext` (the opaque struct). 
> 
> The issue is that `map_type_in_context` maps it to `*mut std::ffi::c_void`, but the code generator for the module wrapper uses `Aspect_RenderingContext` as the Rust type. Let me look more carefully at how the codegen uses these mappings.
>  
> I'm realizing the wrapper layer might be pulling parameter types from a different code path entirely—not from `map_type_to_rust` or `map_type_in_context`. Let me trace where the wrapper actually gets its parameter types from. I need to trace where wrapper parameter types actually get generated—my `map_type_in_context` fix might not even be called for these cases, or the wrapper generation is using a different code path entirely that bypasses my changes.

The type mismatch confirms my concern. The FFI uses `*mut c_void` but the wrapper uses the opaque struct. Let me find how wrapper parameter types are generated:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n 'map_type_in_context\|wrapper.*param\|method_param_type\|safe_param_type\|fn emit_method\|fn generate_method\|fn format_param' crates/opencascade-binding-generator/src/codegen/bindings.rs | head -30
```
<details>
<summary>Output (12 lines)</summary>

```
11:use crate::type_mapping::{self, map_return_type, map_return_type_in_context, map_type_in_context, map_type_to_rust, TypeContext};
117:    /// If this is a convenience wrapper (fewer params with defaults filled in),
318:    /// C++ type for wrappers.hxx parameter (e.g. "Standard_Real", "const gp_Pnt&")
626:    let elem_ffi = map_type_in_context(elem_ty, ffi_ctx).rust_type;
877:fn generate_method_stub(_class_name: &str, method: &Method) -> String {
1221:                let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
1232:                let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
1266:                let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
1277:                let inner_ffi = map_type_in_context(inner, ffi_ctx).rust_type;
1317:    let mapped = map_type_in_context(&effective_ty, ffi_ctx);
2443:    // convenience wrappers with fewer params. These are Rust-only wrappers that
5604:fn format_params(params: &[ParamBinding]) -> String {
```
</details>

<details>
<summary>Reading lines 1190-1330 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1190-L1331">bindings.rs</a></summary>

```

    // Check if this parameter is a standard library bitmask type (e.g., std::ios_base::openmode).
    // These are represented as Type::Class but should be passed as integers with a static_cast.
    // On macOS/libc++, openmode is a typedef for unsigned int (implicit conversion works),
    // but on Linux/libstdc++, it's a proper enum requiring explicit cast.
    if let Some((bitmask_cpp_name, ffi_ty)) = extract_bitmask_type(ty) {
        let rust_type_str = ffi_ty.to_rust_type_string();
        let cpp_type_str = ffi_ty.to_cpp_string();
        return ParamBinding {
            cpp_name,
            rust_name,
            rust_ffi_type: rust_type_str.clone(),
            rust_reexport_type: rust_type_str,
            cpp_type: cpp_type_str,
            cpp_arg_expr: format!("static_cast<{}>({})", bitmask_cpp_name, name),
            enum_rust_type: None,
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
              
... (truncated, 5778 chars)
```
</details>

> 💭 Now I see the issue. At line 1308, `is_opaque_class_name` is checked. If `Aspect_RenderingContext` matches that function, it would be wrapped in `ConstRef(…)`, becoming `&Aspect_RenderingContext`. Let me check what `is_opaque_class_name` does.
> 
> But more importantly, the `rust_reexport_type` comes from `type_to_rust_string(&effective_ty, reexport_ctx)` (line 1319), which is a different path than `map_type_in_context`. Let me find `type_to_rust_string`.

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n 'fn type_to_rust_string\|fn is_opaque_class_name' crates/opencascade-binding-generator/src/codegen/bindings.rs | head -10
```
```
719:fn type_to_rust_string(ty: &Type, reexport_ctx: Option<&ReexportTypeContext>) -> String {
```

<details>
<summary>Reading lines 719-870 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L719-L871">bindings.rs</a></summary>

```
fn type_to_rust_string(ty: &Type, reexport_ctx: Option<&ReexportTypeContext>) -> String {
    match ty {
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
            } else if name == "void" {
                "std::ffi::c_void".to_string()
            } else if let Some(ffi_ty) = std_bitmask_ffi_type(name) {
                ffi_ty.to_rust_type_string()
            } else if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            }
        }
        Type::Handle(name) => format!("crate::ffi::{}", type_mapping::handle_type_name(name)),
        Type::FixedArray(inner, size) => {
            format!("[{}; {}]", type_to_rust_string(inner, reexport_ctx), size)
        }
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
   
... (truncated, 6572 chars)
```
</details>

Now I see! In `type_to_rust_string` (line 740-741), there's a special case for `Standard_Address` → `*mut std::ffi::c_void`, but no corresponding case for `Aspect_RenderingContext`. I need to add it there too.

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L740-L741">bindings.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "            } else if name == \"Standard_Address\" || name == \"Aspect_RenderingContext\" {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "740:sq"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":740,"endLine":740}
```
</details>

Now let me also check `is_opaque_class_name` — since `Aspect_RenderingContext` contains an underscore, it might match that and cause the by-value-to-const-ref conversion at line 1308:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n 'fn is_opaque_class_name' crates/opencascade-binding-generator/src/model.rs
```
```
439:pub fn is_opaque_class_name(name: &str) -> bool {
```

<details>
<summary>Reading lines 435-460 of <a href="../crates/opencascade-binding-generator/src/model.rs#L435-L461">model.rs</a></summary>

```

/// Check if a class name is a real opaque C++ class (not a primitive
/// mapped to a special Rust type like char or void pointer types,
/// and not a standard library bitmask type).
pub fn is_opaque_class_name(name: &str) -> bool {
    name != "char" && !is_void_type_name(name) && std_bitmask_ffi_type(name).is_none()
}

impl Type {
    /// Get a short name for this type (for generating overload suffixes)
    pub fn short_name(&self) -> String {
        match self {
            Type::Void => "void".to_string(),
            Type::Bool => "bool".to_string(),
            Type::I32 => "int".to_string(),
            Type::U32 => "uint".to_string(),
            Type::U16 => "u16".to_string(),
            Type::I16 => "i16".to_string(),
            Type::I64 => "longlong".to_string(),
            Type::U64 => "ulonglong".to_string(),
            Type::Long => "long".to_string(),
            Type::ULong => "ulong".to_string(),
            Type::Usize => "size".to_string(),
            Type::F32 => "float".to_string(),
            Type::F64 => "real".to_string(),
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => inner.short_name(),
```
</details>

Good — `is_opaque_class_name` checks `!is_void_type_name(name)`, and I already added `"Aspect_RenderingContext"` to `is_void_type_name`. So `is_opaque_class_name("Aspect_RenderingContext")` returns `false`, which means it won't be wrapped in `ConstRef` at line 1308. And it won't be treated as a regular class requiring by-reference passing.

Now let me also check `is_void_ptr()` which might affect things:

<details>
<summary>Reading lines 585-610 of <a href="../crates/opencascade-binding-generator/src/model.rs#L585-L611">model.rs</a></summary>

```
    }

    /// Check if this is a void pointer type (Standard_Address = void*, or literal void*)
    /// Methods with these types are bound as `unsafe fn` with `*mut c_void` types.
    pub fn is_void_ptr(&self) -> bool {
        match self {
            Type::Class(name) => is_void_type_name(name),
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_void_ptr()
            }
            _ => false,
        }
    }

    /// Check if this type is a C-style array (e.g., gp_Pnt[8] or fixed-size array refs)
    pub fn is_array(&self) -> bool {
        match self {
            Type::FixedArray(_, _) => true,
            Type::Class(name) => name.contains('[') && name.contains(']'),
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                inner.is_array()
            }
            _ => false,
        }
    }

```
</details>

OK, `is_void_ptr` returns `true` for `Type::Class("Aspect_RenderingContext")` because `is_void_type_name("Aspect_RenderingContext")` is now true. This also means `is_unbindable` (line 869) would mark void pointer types as unbindable. Let me check:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n 'fn is_unbindable' crates/opencascade-binding-generator/src/model.rs
```
```
714:    pub fn is_unbindable(&self) -> bool {
```

<details>
<summary>Reading lines 714-740 of <a href="../crates/opencascade-binding-generator/src/model.rs#L714-L741">model.rs</a></summary>

```
    pub fn is_unbindable(&self) -> bool {
        (self.is_array() && !self.is_fixed_array_ref() && !self.is_fixed_array_param())
            || self.is_rvalue_ref()
            || self.is_unresolved_template_type()
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
            // Use postfix const ("T const*") rather than prefix ("const T*") so that
            // when the inner type is itself a pointer (e.g., ConstRef(MutPtr(Class("X")))),
            // the const correctly qualifies the pointer level, not the pointee:
            //   ConstRef(MutPtr(X)) → "X* const*" (correct: pointer to const-pointer-to-X)
            //   vs. "const X**" (wrong: pointer to pointer-to-const-X)
            // For simple types, "T const*" and "const T*" are equivalent in C/C++.
            Type::ConstRef(inner) => {
                if let Type::FixedArray(elem, _) = inner.as_ref() {
                    format!("{} const*", elem.to_cpp_string())
```
</details>

Good — `is_unbindable` does NOT check `is_void_ptr`. The void pointer makes functions `unsafe` but doesn't skip them. So that's fine.

Now let me regenerate and rebuild:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```
<details>
<summary>Output (377 lines)</summary>

```
Building binding generator...
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `release` profile [optimized] target(s) in 5.94s
Generating bindings from /Users/sethml/src/opencascade-rs/crates/opencascade-sys/bindings.toml...
Cleaning /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated...
  Excluded 14 modules via exclude_modules
Loaded config: 1 module patterns, 14 exclude module patterns, 0 individual headers, 9 header exclusions -> 6809 headers
  Excluded 2 headers after dependency resolution
Parsing 6809 headers...
  Clang parse time: 1.76s
  Collected 5070 NCollection/template typedef entries (4869 unique template spellings)
  Collected 144 simple typedef entries

Timing summary:
  Total clang parse time: 1.76s
  Total AST visit time: 2.69s
  Rewrote 28 template Handle type references to alias names
  Auto-detected 56 template type instantiations
  Rewrote 121 template Class type references to alias names

Building module dependency graph...

Generating code...

=== Generating FFI output ===

Generating ffi.rs...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/ffi.rs (5120 classes, 1394 functions)
Generating wrappers.cpp...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/wrappers.cpp
Generating module re-exports...
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/t_collection.rs (4 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/standard.rs (43 types, 7 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/gp.rs (38 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/bnd.rs (14 types, 3 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/n_collection.rs (17 types, 53 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/quantity.rs (7 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/image.rs (10 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/font.rs (8 types, 1 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/media.rs (9 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/osd.rs (52 types, 2 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/t_col_std.rs (24 types, 37 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/top_loc.rs (5 types, 0 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/graphic3d.rs (95 types, 25 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/aspect.rs (31 types, 9 extra)
  Wrote: /Users/sethml/src/opencascade-rs/crates/openc
... (truncated, 38667 chars total)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff HEAD --stat -- crates/opencascade-sys/generated/
```
<details>
<summary>Output (5 lines)</summary>

```
 crates/opencascade-sys/generated/aspect.rs    | 8 ++++----
 crates/opencascade-sys/generated/ffi.rs       | 7 ++++++-
 crates/opencascade-sys/generated/v3d.rs       | 4 ++--
 crates/opencascade-sys/generated/wrappers.cpp | 4 ++--
 4 files changed, 14 insertions(+), 9 deletions(-)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff HEAD -- crates/opencascade-sys/generated/
```
<details>
<summary>Output (84 lines)</summary>

```
diff --git a/crates/opencascade-sys/generated/aspect.rs b/crates/opencascade-sys/generated/aspect.rs
index f28157a..af64a60 100644
--- a/crates/opencascade-sys/generated/aspect.rs
+++ b/crates/opencascade-sys/generated/aspect.rs
@@ -9092,8 +9092,8 @@ impl HandleAspectXRSession {
 
 pub use crate::ffi::{
     Aspect_FrustumLRBT_Standard_Real as FrustumLRBT_Standard_Real,
-    Aspect_FrustumLRBT_double as FrustumLRBT_double, Aspect_SequenceOfColor as SequenceOfColor,
-    Aspect_TouchMap as TouchMap, Aspect_TrackedDevicePoseArray as TrackedDevicePoseArray,
-    Aspect_XDisplay as XDisplay, Aspect_XRActionMap as XRActionMap,
-    Aspect_XVisualInfo as XVisualInfo,
+    Aspect_FrustumLRBT_double as FrustumLRBT_double, Aspect_RenderingContext as RenderingContext,
+    Aspect_SequenceOfColor as SequenceOfColor, Aspect_TouchMap as TouchMap,
+    Aspect_TrackedDevicePoseArray as TrackedDevicePoseArray, Aspect_XDisplay as XDisplay,
+    Aspect_XRActionMap as XRActionMap, Aspect_XVisualInfo as XVisualInfo,
 };
diff --git a/crates/opencascade-sys/generated/ffi.rs b/crates/opencascade-sys/generated/ffi.rs
index 03c8bba..78c1d20 100644
--- a/crates/opencascade-sys/generated/ffi.rs
+++ b/crates/opencascade-sys/generated/ffi.rs
@@ -35261,6 +35261,11 @@ pub struct Aspect_FrustumLRBT_double {
 }
 /// Referenced type from C++
 #[repr(C)]
+pub struct Aspect_RenderingContext {
+    _opaque: [u8; 0],
+}
+/// Referenced type from C++
+#[repr(C)]
 pub struct Aspect_SequenceOfColor {
     _opaque: [u8; 0],
 }
@@ -641518,7 +641523,7 @@ extern "C" {
     /// **Source:** `V3d_View.hxx`:58 - `V3d_View::DynamicType()`
     pub fn V3d_View_dynamic_type(self_: *const V3d_View) -> *const HandleStandardType;
     /// **Source:** `V3d_View.hxx`:76 - `V3d_View::SetWindow()`
-    pub fn V3d_View_set_window_handleaspectwindow_voidptr(
+    pub fn V3d_View_set_window_handleaspectwindow_renderingcontext(
         self_: *mut V3d_View,
         theWindow: *const HandleAspectWindow,
         theContext: *mut std::ffi::c_void,
diff --git a/crates/opencascade-sys/generated/v3d.rs b/crates/opencascade-sys/generated/v3d.rs
index d6a7543..bc83e60 100644
--- a/crates/opencascade-sys/generated/v3d.rs
+++ b/crates/opencascade-sys/generated/v3d.rs
@@ -4178,13 +4178,13 @@ impl View {
     /// Otherwise an internal graphic context is created.
     /// Warning: The view is centered and resized to preserve
     /// the height/width ratio of the window.
-    pub fn set_window_handleaspectwindow_voidptr(
+    pub unsafe fn set_window_handleaspectwindow_renderingcontext(
         &mut self,
         theWindow: &crate::ffi::HandleAspectWindow,
         theContext: *mut std::ffi::c_void,
     ) {
         unsafe {
-            crate::ffi::V3d_View_set_window_handleaspectwindow_voidptr(
+            crate::ffi::V3d_View_set_window_h
... (truncated, 5032 chars total)
```
</details>

The diff looks clean now. The `graphic3d.rs` diff no longer appears (it matches HEAD, meaning it generates identically). The remaining diffs are:

1. **ffi.rs**: Added `Aspect_RenderingContext` opaque struct; renamed the V3d overload from `_voidptr` to `_renderingcontext` 
2. **v3d.rs**: Same rename, and the method is now `unsafe` (because it takes a void pointer)
3. **wrappers.cpp**: Uses `Aspect_RenderingContext` instead of `void*` in C++ signatures
4. **aspect.rs**: Simply re-exports `Aspect_RenderingContext`

Let me build:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

Build succeeds. Now let me run the binding generator tests and workspace tests:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (38 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.13s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-3404803fe4cd90de)

running 17 tests
test header_deps::tests::test_is_likely_occt_header ... ok
test module_graph::tests::test_extract_rust_type_name ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test parser::tests::test_extract_module_from_header ... ok
test parser::tests::test_extract_template_arg ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test parser::tests::test_is_occt_namespace_chain ... ok
test resolver::tests::test_safe_param_name ... ok
test resolver::tests::test_safe_method_name ... ok
test parser::tests::test_map_standard_type ... ok
test type_mapping::tests::test_map_class ... ok
test type_mapping::tests::test_map_primitives ... ok
test type_mapping::tests::test_map_const_ref ... ok
test type_mapping::tests::test_map_return_type ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/occt_bindgen-eda4fc796a0d21f2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests opencascade_binding_generator

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

real  5.4s
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo test --workspace --exclude opencascade-binding-generator 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo test --workspace --exclude opencascade-binding-generator 2>&1 | grep -E 'test result|error|FAILED'
```
<details>
<summary>Output (18 lines)</summary>

```
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 19 passed; 0 failed; 4 ignored; 0 measured; 0 filtered out; finished in 0.95s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.41s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.50s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
</details>

All tests pass. Let me update the todo list:

**Todo list:**
- ✅ Examine git diff for platform differences
- ✅ Find Aspect_RenderingContext in OCCT headers
- ✅ Find how binding generator handles typedefs
- ✅ Make Aspect_RenderingContext opaque
- ✅ Regenerate and verify bindings

Done. Here's a summary of the changes:

**Problem:** `Aspect_RenderingContext` is a platform-dependent typedef (`void*` on Linux, `NSOpenGLContext*` on macOS, `EAGLContext*` on iOS). This caused the generated bindings to differ between platforms — on Linux methods using it worked fine (resolved to `*mut c_void`), while on macOS the generator couldn't resolve `NSOpenGLContext` and skipped those methods.

**Fix:** Treated `Aspect_RenderingContext` the same way as `Standard_Address` (another `void*` typedef):

1. `crates/opencascade-binding-generator/src/parser.rs` — `map_standard_type()`: Intercept `"Aspect_RenderingContext"` before canonical resolution to return `Type::Class("Aspect_RenderingContext")`
2. `crates/opencascade-binding-generator/src/model.rs` — `is_void_type_name()`: Added `"Aspect_RenderingContext"` so it's recognized as a known void-pointer type
3. `crates/opencascade-binding-generator/src/type_mapping.rs` — Both `map_type_to_rust()` and `map_type_in_context()`: Map to `*mut std::ffi::c_void`
4. `crates/opencascade-binding-generator/src/codegen/bindings.rs` — `type_to_rust_string()`: Map to `*mut std::ffi::c_void` for re-export wrappers
5. `crates/opencascade-binding-generator/src/codegen/cpp.rs`: Skip header inclusion for the type name

The C++ wrappers now use `Aspect_RenderingContext` (the OCCT typedef, correct per-platform) while Rust always sees `*mut c_void` — consistent and platform-independent.

*2026-02-23 22:58 · 760s · Words: 6,909 in · 3,798 out · 83,716 ctx · 61 rounds*

---

<a id="user-2"></a>

## User (2)

> That&#x27;s not quite what I had in mind. I&#x27;d imagined that in Rust aspect::rendering_context would be an opaque pass-by-value type whose size would be sizeof(void *) that would get cast to/from C++ void * in ffi.rs and then to Aspect_RenderingContext in wrappers.cpp.

*2026-02-24 07:56*

### Assistant — Claude Opus 4.6

> 💭 The user wants `Aspect_RenderingContext` to be a proper opaque type that:
> 1. Has size `sizeof(void*)` (so it can be passed by value)
> 2. In Rust, it's an opaque struct of pointer size
> 3. At the FFI boundary (ffi.rs), it appears as this opaque type
> 4. In wrappers.cpp, it gets used as `Aspect_RenderingContext` (which is the platform-specific pointer type)
> 
> So rather than mapping it to `*mut c_void` everywhere, we want:
> - A Rust struct `Aspect_RenderingContext` that is `#[repr(C)]` and has the size of a pointer (e.g., `[u8; std::mem::size_of::<*const ()>()]` or just a `*mut c_void` field)
> 
> 
> - Methods that take `Aspect_RenderingContext` by value, matching how the C++ wrapper accepts it
> - The key insight is that `Aspect_RenderingContext` should be a pointer-sized opaque value passed by value, not a zero-sized struct, so I need to stop mapping it to `*mut c_void` in the type mapping functions and instead define it as its own struct type with pointer size
> 
> Now I'm reconsidering the implementation strategy: keep the `map_standard_type` interception to consistently get `Type::Class("Aspect_RenderingContext")` across platforms, but remove it from the void type and opaque class checks so it gets treated as a regular class that can be passed by value, and make sure the generated FFI struct has the correct pointer-sized representation. name` - it's not a void type, it's its own opaque type.
> 3. The struct needs to be pointer-sized. I need to change how its opaque declaration is generated.
> 4. In type mappings, it should just map as a normal class - `Aspect_RenderingContext` in Rust FFI.
> 5. Methods taking `Aspect_RenderingContext` should pass it by value (it IS a pointer, so by-value is correct).
> 6. The C++ wrappers should use `Aspect_RenderingContext` as the parameter type.
> 
> The challenge is making the opaque struct pointer-sized. Currently the "referenced types" section generates zero-sized opaque structs. I need a way to specify that this particular type should be pointer-sized.
> 
> Actually, the simplest approach might be to define `Aspect_RenderingContext` as a POD type with a single `*mut c_void` field instead of a zero-sized opaque struct. This way it's pointer-sized and can be passed by value. To implement this, I need to undo the special-case handling I added for void types and type mappings, keep the standard type interception and the cpp.rs skip-include change, and then add logic to mark `Aspect_RenderingContext` as a known, non-opaque type that gets the pointer field treatment. a pointer-sized struct declaration
> 
> Actually, let me reconsider. The way the binding generator works:
> - `is_opaque_class_name` returns true for most OCCT classes → they get passed by const-ref
> - If it returns false, the type is passed by value
> - For the struct declaration, it ends up in the "referenced types" section as a zero-sized opaque
> 
> I need `Aspect_RenderingContext` to:
> - Be passed by value (not by reference) → `is_opaque_class_name` should return false
> - Have a pointer-sized struct → need special handling in the declaration
> 
> For `is_opaque_class_name` to return false, I can keep it in `is_void_type_name` BUT then not map it to `*mut c_void`. But `is_void_type_name` is used in other places too, so that won't work cleanly.
> 
> The cleaner approach is to introduce a new category for opaque pointer-sized types. I'd keep the `map_standard_type` interception so it stays as `Type::Class("Aspect_RenderingContext")`, add a dedicated check like `is_opaque_pointer_type_name` that identifies this type, make sure `is_opaque_class_name` returns false so it passes by value, and then generate a special struct with a `*mut c_void` field inside while keeping the C++ type as-is. Generating the FFI bindings... Writing the Rust wrapper...
> 
> Now I'm identifying the key requirements: the opaque type needs to be pointer-sized with an actual `_ptr` field rather than zero-sized, passed by value everywhere, and have its own named type while keeping methods marked unsafe since they work with opaque pointers. I need to figure out where the opaque declarations are generated to make these changes.
> 
> Looking at the code generation flow, `Aspect_RenderingContext` gets placed in the "Referenced types" section through `generate_opaque_declarations` in the Rust codegen. The issue is that my current approach using `is_void_type_name` triggers too much void-pointer special-casing. Instead, I should create a separate predicate like `is_pointer_sized_opaque_type` to handle this case distinctly.
> 
> My cleaner strategy is to revert all the scattered changes and consolidate: keep just the `map_standard_type` interception in the parser and the cpp.rs skip, then add `Aspect_RenderingContext` to a new `is_pointer_opaque_type_name` category in model.rs. This way `is_opaque_class_name` returns false for it (enabling by-value passing), and I can mark it appropriately for unsafe function generation without the void-pointer mappings.
> 
> Now I'm reviewing the actual edits I've made across the codebase to figure out what to keep and what to undo.
> 
> I need to create a new helper function in model.rs to identify pointer-opaque types, then update the opaque declarations generator to use a pointer-sized struct with proper derives instead of the zero-sized array approach.
> 
> Let me check the generate_opaque_declarations function to see how to integrate this change.
> 
> Actually, I should verify that `Aspect_RenderingContext` gets properly recognized as a known type — it needs to be in the `all_classes` set from the referenced types collection so methods using it don't get skipped. If I remove the void type bypass from `is_class_name_unknown`, the type must be collected during the reference scanning phase since it appears in method signatures.
> 
> Let me trace through `collect_referenced_types` to make sure it picks this up, then I'll implement the changes. On second thought about the `needs_unsafe_fn` aspect — the user described this as an opaque pass-by-value type wrapping a platform pointer, which is type-safe compared to raw void pointers, but since the original OCCT methods use raw pointers, marking it `unsafe` makes sense for the Rust binding.
> 
> Looking back at the previous diff, methods like `SetWindow` were already being generated with `void*` mapped to `*mut c_void` and marked unsafe on Linux, but skipped on macOS. With my new approach, the parameter will be the opaque `Aspect_RenderingContext` struct instead of a raw pointer, but it should still be unsafe since the underlying value is a raw pointer. I need to verify that `needs_unsafe_fn` correctly identifies this type as unsafe — currently it checks `is_void_ptr()` or `is_raw_ptr()`, so I'm checking whether `Type::Class("Aspect_RenderingContext")` satisfies either condition.
> 
> Since `is_void_ptr()` will return false after I remove it from `is_void_type_name`, and `is_raw_ptr()` is false for Class types, I need to add a new check for pointer-opaque types to `needs_unsafe_fn`. I'm planning to add an `is_pointer_opaque_type_name` function, remove `"Aspect_RenderingContext"` from `is_void_type_name`, update `is_opaque_class_name` to exclude pointer-opaque types, and modify `needs_unsafe_fn` to include the pointer-opaque check.
> 
> For the type mapping, I'll remove the special cases from `map_type_to_rust` and `map_type_in_context`, and make sure `is_class_name_unknown` treats pointer-opaque types as known. I also need to undo the `type_to_rust_string` change in bindings.rs, update the opaque declaration generation to handle pointer-sized bodies for pointer-opaque types, and keep the cpp.rs skip-include as-is.
> 
> For `is_class_name_unknown`, I'm checking if I can use the same pattern as void types...
> 
> Checking pointer-opaque types...
> 
> Actually, the referenced types should get collected automatically from method signatures when `Aspect_RenderingContext` appears as a parameter, which means it'll be in `all_classes` when `type_uses_unknown_type` checks it. But I'll add the safety check in `is_class_name_unknown` anyway.
> 
> Now I need to look at how `generate_opaque_declarations` works to modify it for pointer-opaque types.

I understand - you want `Aspect_RenderingContext` to be a proper pointer-sized opaque type, not mapped to `*mut c_void`. Let me rework the approach.

**Todo list:**
- 🔄 Undo *mut c_void mapping approach
- ⬜ Add pointer-opaque type category
- ⬜ Generate pointer-sized struct decl
- ⬜ Regenerate and verify bindings

First, let me understand the opaque declaration generation:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -n 'generate_opaque_declarations' crates/opencascade-binding-generator/src/codegen/rust.rs
```
```
173:    let (opaque_type_decls, nested_types) = generate_opaque_declarations(
464:fn generate_opaque_declarations(
```

<details>
<summary>Reading lines 464-540 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L464-L541">rust.rs</a></summary>

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
```
</details>

Now I see exactly where the opaque structs are generated (line 516). I need to make that line generate a pointer-sized struct for `Aspect_RenderingContext`. Let me now read the current state of all my edited files to understand what to undo/change:

<details>
<summary>Reading lines 418-425 of <a href="../crates/opencascade-binding-generator/src/model.rs#L418-L426">model.rs</a></summary>

```
/// Standard_Address is a typedef for void*, and "void" is the parsed
/// form of literal void* parameters.
pub fn is_void_type_name(name: &str) -> bool {
    name == "Standard_Address" || name == "void" || name == "Aspect_RenderingContext"
}

/// Check if a class name represents a C++ standard library bitmask type.
/// These types are integer-compatible bitmasks that need explicit `static_cast`
```
</details>

<details>
<summary>Reading lines 436-445 of <a href="../crates/opencascade-binding-generator/src/model.rs#L436-L446">model.rs</a></summary>

```
/// Check if a class name is a real opaque C++ class (not a primitive
/// mapped to a special Rust type like char or void pointer types,
/// and not a standard library bitmask type).
pub fn is_opaque_class_name(name: &str) -> bool {
    name != "char" && !is_void_type_name(name) && std_bitmask_ffi_type(name).is_none()
}

impl Type {
    /// Get a short name for this type (for generating overload suffixes)
    pub fn short_name(&self) -> String {
```
</details>

Now let me read the current state of type_mapping.rs for my two inserted blocks:

<details>
<summary>Reading lines 212-234 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L212-L235">type_mapping.rs</a></summary>

```
            }
        }
        Type::Class(class_name) if class_name == "Aspect_RenderingContext" => {
            // Aspect_RenderingContext is a platform-dependent pointer typedef
            // (void* on Linux, NSOpenGLContext* on macOS). Map to *mut c_void for consistency.
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
        Type::Class(class_name) if class_name == "char" => {
            // C++ char resolved from canonical types (e.g., Standard_Character)
```
</details>

<details>
<summary>Reading lines 491-510 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L491-L511">type_mapping.rs</a></summary>

```
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
```
</details>

Now let me check the second inserted block:

<details>
<summary>Reading lines 483-505 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L483-L506">type_mapping.rs</a></summary>

```
            }
        }
        Type::Class(class_name) if class_name == "Aspect_RenderingContext" => {
            // Aspect_RenderingContext is a platform-dependent pointer typedef
            // (void* on Linux, NSOpenGLContext* on macOS). Map to *mut c_void for consistency.
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
```
</details>

Now let me check bindings.rs:

<details>
<summary>Reading lines 738-745 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L738-L746">bindings.rs</a></summary>

```
            if name == "char" {
                "std::ffi::c_char".to_string()
            } else if name == "Standard_Address" || name == "Aspect_RenderingContext" {
                "*mut std::ffi::c_void".to_string()
            } else if name == "void" {
                "std::ffi::c_void".to_string()
            } else if let Some(ffi_ty) = std_bitmask_ffi_type(name) {
                ffi_ty.to_rust_type_string()
```
</details>

Good. Now I also need to check `needs_unsafe_fn` and related:

<details>
<summary>Reading lines 720-730 of <a href="../crates/opencascade-binding-generator/src/model.rs#L720-L731">model.rs</a></summary>

```
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
```
</details>

Now let me also check `is_class_name_unknown` in type_mapping.rs:

<details>
<summary>Reading lines 400-425 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L400-L426">type_mapping.rs</a></summary>

```
    /// protected_destructor + the 91 manually-specified known collections).
    /// Methods returning a bare `Class(name)` value (generating `OwnedPtr<name>`) are
    /// only allowed if `name` is in this set. If `None`, no constraint is applied.
    pub deletable_class_names: Option<&'a std::collections::HashSet<String>>,
}

/// Check if a type references an unknown class/handle
/// Returns true if the type uses a Handle or Class that is not in all_classes
/// Check if a class name is unknown (not in the known type set).
/// Shared logic for type_uses_unknown_class and type_uses_unknown_handle.
fn is_class_name_unknown(class_name: &str, all_classes: &std::collections::HashSet<String>) -> bool {
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
    // Standard library bitmask types (e.g., std::ios_base::openmode) are known
    if crate::model::std_bitmask_ffi_type(class_name).is_some() {
        return false;
    }
```
</details>

Good, now I have a complete picture. Let me also check `describe_unbindable_reason` to see if void ptr types appear there:

<details>
<summary>Reading lines 866-878 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L866-L879">bindings.rs</a></summary>

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

/// Generate a best-guess stub Rust declaration for a skipped method.
fn generate_method_stub(_class_name: &str, method: &Method) -> String {
    let self_param = if method.is_const { "&self" } else { "&mut self" };
```
</details>

`describe_unbindable_reason` mentions void ptr, but this is for diagnostics only — `is_unbindable()` doesn't check `is_void_ptr()`, so void-pointer methods aren't skipped.

Now let me implement the new approach. I'll make all the edits together:

**Todo list:**
- 🔄 Undo *mut c_void mapping approach
- ⬜ Add pointer-opaque type category
- ⬜ Generate pointer-sized struct decl
- ⬜ Regenerate and verify bindings

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L421-L430">model.rs -1+9</a>, <a href="../crates/opencascade-binding-generator/src/model.rs#L448-L449">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/model.rs#L732-L734">-1+2</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L740-L741">bindings.rs -1+1</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L516-L522">rust.rs -1+6</a>  
**Failing: <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L214">type_mapping.rs -1</a>, <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L413-L415">-1+2</a>, <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L485">-1</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "421:ma",
      "content": "    name == \"Standard_Address\" || name == \"void\"\n}\n\n/// Check if a type name is a platform-dependent pointer typedef that should be\n/// represented as an opaque pointer-sized type in Rust. These types are pointer-sized\n/// values (like void*) but get their own named opaque Rust type instead of *mut c_void,\n/// so that the same generated code works across platforms.\npub fn is_pointer_opaque_type_name(name: &str) -> bool {\n    name == \"Aspect_RenderingContext\""
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "440:pj",
      "content": "    name != \"char\" && !is_void_type_name(name) && !is_pointer_opaque_type_name(name) && std_bitmask_ffi_type(name).is_none()"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "724:iq",
      "content": "    pub fn needs_unsafe_fn(&self) -> bool {\n        self.is_void_ptr() || self.is_raw_ptr() || matches!(self, Type::Class(name) if is_pointer_opaque_type_name(name))"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "214:ht\n215:fj\n216:an\n217:mu\n218:mi\n219:wh\n220:rl\n221:nk\n222:uo\n223:cy",
      "content": ""
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "485:ht\n486:fj\n487:an\n488:mu\n489:mi\n490:wh\n491:rl\n492:nk\n493:uo\n494:cy",
      "content": ""
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "414:ne\n415:kw",
      "content": "    // Void pointer types — Standard_Address (void*) and literal \"void\" — are known\n    if crate::model::is_void_type_name(class_name) || crate::model::is_pointer_opaque_type_name(class_name) {"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "740:vj",
      "content": "            } else if name == \"Standard_Address\" {"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "lineHashes": "516:qf",
      "content": "        if crate::model::is_pointer_opaque_type_name(&safe_name) {\n            writeln!(out, \"#[derive(Debug, Clone, Copy)]\").unwrap();\n            writeln!(out, \"pub struct {} {{ _ptr: *mut std::ffi::c_void }}\", safe_name).unwrap();\n        } else {\n            writeln!(out, \"pub struct {} {{ _opaque: [u8; 0] }}\", safe_name).unwrap();\n        }"
    }
  ]
}
```
**Output:**
```
{
  "applied": 8,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/model.rs",
      "startLine": 421,
      "endLine": 733
    },
    {
      "file": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "startLine": 214,
      "endLine": 493
    },
    {
      "file": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "startLine": 740,
      "endLine": 740
    },
    {
      "file": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "startLine": 516,
      "endLine": 521
    }
  ]
}
```
</details>

All edits applied. Let me verify compilation:

**Todo list:**
- ✅ Undo *mut c_void mapping approach
- ✅ Add pointer-opaque type category
- ✅ Generate pointer-sized struct decl
- 🔄 Regenerate and verify bindings

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo check -p opencascade-binding-generator 2>&1
```

The old `needs_unsafe_fn` body wasn't fully replaced - there's still a duplicate line. Let me check:

<details>
<summary>Reading lines 728-740 of <a href="../crates/opencascade-binding-generator/src/model.rs#L728-L741">model.rs</a></summary>

```
    /// Check if this type involves raw pointers that require the containing
    /// function to be marked `unsafe`. True for void pointers (Standard_Address)
    /// and raw T*/const T* pointers (excluding const char* which is handled
    /// as C strings).
    pub fn needs_unsafe_fn(&self) -> bool {
        self.is_void_ptr() || self.is_raw_ptr() || matches!(self, Type::Class(name) if is_pointer_opaque_type_name(name))
        self.is_void_ptr() || self.is_raw_ptr()
    }

    /// Convert this type to a C++ parameter type for extern "C" wrapper functions.
    /// References become pointers (const T& → const T*, T& → T*).
    pub fn to_cpp_extern_c_param(&self) -> String {
        match self {
```
</details>

The old line 734 wasn't replaced. Let me fix it:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L734">model.rs -1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "",
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "734:jf"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/model.rs","startLine":734,"endLine":734}
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo check -p opencascade-binding-generator 2>&1
```

Now regenerate and build:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff HEAD -- crates/opencascade-sys/generated/
```
<details>
<summary>Output (112 lines)</summary>

```
diff --git a/crates/opencascade-sys/generated/aspect.rs b/crates/opencascade-sys/generated/aspect.rs
index f28157a..af64a60 100644
--- a/crates/opencascade-sys/generated/aspect.rs
+++ b/crates/opencascade-sys/generated/aspect.rs
@@ -9092,8 +9092,8 @@ impl HandleAspectXRSession {
 
 pub use crate::ffi::{
     Aspect_FrustumLRBT_Standard_Real as FrustumLRBT_Standard_Real,
-    Aspect_FrustumLRBT_double as FrustumLRBT_double, Aspect_SequenceOfColor as SequenceOfColor,
-    Aspect_TouchMap as TouchMap, Aspect_TrackedDevicePoseArray as TrackedDevicePoseArray,
-    Aspect_XDisplay as XDisplay, Aspect_XRActionMap as XRActionMap,
-    Aspect_XVisualInfo as XVisualInfo,
+    Aspect_FrustumLRBT_double as FrustumLRBT_double, Aspect_RenderingContext as RenderingContext,
+    Aspect_SequenceOfColor as SequenceOfColor, Aspect_TouchMap as TouchMap,
+    Aspect_TrackedDevicePoseArray as TrackedDevicePoseArray, Aspect_XDisplay as XDisplay,
+    Aspect_XRActionMap as XRActionMap, Aspect_XVisualInfo as XVisualInfo,
 };
diff --git a/crates/opencascade-sys/generated/ffi.rs b/crates/opencascade-sys/generated/ffi.rs
index 03c8bba..a423449 100644
--- a/crates/opencascade-sys/generated/ffi.rs
+++ b/crates/opencascade-sys/generated/ffi.rs
@@ -35261,6 +35261,12 @@ pub struct Aspect_FrustumLRBT_double {
 }
 /// Referenced type from C++
 #[repr(C)]
+#[derive(Debug, Clone, Copy)]
+pub struct Aspect_RenderingContext {
+    _ptr: *mut std::ffi::c_void,
+}
+/// Referenced type from C++
+#[repr(C)]
 pub struct Aspect_SequenceOfColor {
     _opaque: [u8; 0],
 }
@@ -209526,7 +209532,7 @@ extern "C" {
         self_: *mut Graphic3d_CView,
         theParentVIew: *const HandleGraphic3dCView,
         theWindow: *const HandleAspectWindow,
-        theContext: *mut std::ffi::c_void,
+        theContext: Aspect_RenderingContext,
     );
     /// **Source:** `Graphic3d_CView.hxx`:288 - `Graphic3d_CView::Window()`
     pub fn Graphic3d_CView_window(self_: *const Graphic3d_CView) -> *mut HandleAspectWindow;
@@ -641518,10 +641524,10 @@ extern "C" {
     /// **Source:** `V3d_View.hxx`:58 - `V3d_View::DynamicType()`
     pub fn V3d_View_dynamic_type(self_: *const V3d_View) -> *const HandleStandardType;
     /// **Source:** `V3d_View.hxx`:76 - `V3d_View::SetWindow()`
-    pub fn V3d_View_set_window_handleaspectwindow_voidptr(
+    pub fn V3d_View_set_window_handleaspectwindow_renderingcontext(
         self_: *mut V3d_View,
         theWindow: *const HandleAspectWindow,
-        theContext: *mut std::ffi::c_void,
+        theContext: Aspect_RenderingContext,
     );
     /// **Source:** `V3d_View.hxx`:93 - `V3d_View::SetWindow()`
     pub fn V3d_View_set_window_handlev3dview_vec2d_typeoftriedronposition_vec2d_vec2i(
diff --git a/crates/opencascade-sys/generated/graphic3d.rs b/crates/opencascade-sys/generated/graphic3d.rs
... (truncated, 6488 chars total)
```
</details>

This looks exactly right:

- **ffi.rs**: `Aspect_RenderingContext` is `#[derive(Debug, Clone, Copy)]` with `_ptr: *mut c_void` (pointer-sized, pass-by-value)
- **ffi.rs extern "C"**: Parameters use `Aspect_RenderingContext` by value, not `*mut c_void`
- **graphic3d.rs & v3d.rs**: Wrappers use `crate::ffi::Aspect_RenderingContext` by value
- **wrappers.cpp**: C++ uses `Aspect_RenderingContext` (the OCCT typedef), which resolves to the platform-appropriate pointer type

Let me build:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

Builds clean. Now run the tests:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (38 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.28s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-3404803fe4cd90de)

running 17 tests
test header_deps::tests::test_is_likely_occt_header ... ok
test module_graph::tests::test_extract_rust_type_name ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test parser::tests::test_extract_module_from_header ... ok
test parser::tests::test_extract_template_arg ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
test parser::tests::test_is_occt_namespace_chain ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test parser::tests::test_map_standard_type ... ok
test resolver::tests::test_safe_method_name ... ok
test resolver::tests::test_safe_param_name ... ok
test type_mapping::tests::test_map_class ... ok
test type_mapping::tests::test_map_const_ref ... ok
test type_mapping::tests::test_map_primitives ... ok
test type_mapping::tests::test_map_return_type ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/occt_bindgen-eda4fc796a0d21f2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests opencascade_binding_generator

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

real  6.5s
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo test --workspace --exclude opencascade-binding-generator 2>&1 | grep -E 'test result|error|FAILED'
```

All tests pass.

**Todo list:**
- ✅ Undo *mut c_void mapping approach
- ✅ Add pointer-opaque type category
- ✅ Generate pointer-sized struct decl
- ✅ Regenerate and verify bindings

Done. `Aspect_RenderingContext` is now a pointer-sized opaque pass-by-value type:

**Rust struct** (in ffi.rs):
```rust
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Aspect_RenderingContext {
    _ptr: *mut std::ffi::c_void,
}
```

**FFI signatures** use it by value (`theContext: Aspect_RenderingContext`), and **C++ wrappers** use the OCCT typedef `Aspect_RenderingContext` which resolves to the correct platform pointer type.

The changes introduce a new `is_pointer_opaque_type_name()` predicate in `crates/opencascade-binding-generator/src/model.rs` that marks types as pointer-sized opaques — not treated as void pointers (so they get their own named type), not treated as opaque classes (so they're passed by value, not by reference), but still marked `unsafe` since they wrap raw pointers.

*2026-02-24 08:05 · 523s · Words: 1,429 in · 2,028 out · 113,006 ctx · 26 rounds*

---

<a id="user-3"></a>

## User (3)

> I probably should have asked before starting this refactoring: How does one get an Aspect_RenderingContext in OCCT? Is there an API to generate one, or is the caller expected to generate one through other means?

*2026-02-24 08:08*

### Assistant — Claude Sonnet 4.6

> 💭 The user is asking about how `Aspect_RenderingContext` is obtained in OCCT - whether there's an API to generate one or if the caller is expected to provide it through other means. Let me search the OCCT headers and source code to find out.

**Terminal:**
```sh
grep -r "Aspect_RenderingContext" crates/occt-sys/OCCT/src/ --include="*.hxx" --include="*.cxx" -l | head -30
```
<details>
<summary>Output (14 lines)</summary>

```
crates/occt-sys/OCCT/src/D3DHost/D3DHost_View.cxx
crates/occt-sys/OCCT/src/D3DHost/D3DHost_View.hxx
crates/occt-sys/OCCT/src/Aspect/Aspect_RenderingContext.hxx
crates/occt-sys/OCCT/src/Graphic3d/Graphic3d_CView.hxx
crates/occt-sys/OCCT/src/V3d/V3d_View.cxx
crates/occt-sys/OCCT/src/V3d/V3d_View.hxx
crates/occt-sys/OCCT/src/OpenGl/OpenGl_Context.cxx
crates/occt-sys/OCCT/src/OpenGl/OpenGl_GraphicDriver.cxx
crates/occt-sys/OCCT/src/OpenGl/OpenGl_Window.cxx
crates/occt-sys/OCCT/src/OpenGl/OpenGl_View.cxx
crates/occt-sys/OCCT/src/OpenGl/OpenGl_Window.hxx
crates/occt-sys/OCCT/src/OpenGl/OpenGl_View.hxx
crates/occt-sys/OCCT/src/OpenGl/OpenGl_Context.hxx
crates/occt-sys/OCCT/src/OpenGl/OpenGl_GraphicDriver.hxx
```
</details>

**Terminal:**
```sh
grep -n "Aspect_RenderingContext" crates/occt-sys/OCCT/src/V3d/V3d_View.hxx | head -20
echo "---"
grep -n "Aspect_RenderingContext" crates/occt-sys/OCCT/src/OpenGl/OpenGl_Context.hxx | head -20
echo "---"
grep -n "Aspect_RenderingContext" crates/occt-sys/OCCT/src/OpenGl/OpenGl_GraphicDriver.hxx | head -20
```
```
77:                                 const Aspect_RenderingContext theContext = NULL);
```

**Terminal:**
```sh
grep -n "Aspect_RenderingContext" crates/occt-sys/OCCT/src/OpenGl/OpenGl_Context.hxx
echo "---"
grep -n "RenderingContext\|theContext\|InitFromGLContext\|InitEGL\|WrapFBO\|CreateContext\|MakeCurrent\|ShareCtx" crates/occt-sys/OCCT/src/OpenGl/OpenGl_Context.hxx | head -40
```
<details>
<summary>Output (9 lines)</summary>

```
22:#include <Aspect_RenderingContext.hxx>
228:  //!   )theEglDisp,  (Aspect_RenderingContext )theEglCtx);
239:  //!   (Aspect_RenderingContext )theGContext);
250:  //!   (Aspect_RenderingContext )theGlxCtx);
262:                                        const Aspect_RenderingContext theContext,
272:  Aspect_RenderingContext RenderingContext() const { return myGContext; }
284:                (Aspect_RenderingContext)theGContext,
295:                (Aspect_RenderingContext)theGContext,
1118:  Aspect_RenderingContext myGContext; //!< rendering context EGLContext | HGLRC | GLXContext | EAGLContext* | NSOpenGLContext*
```
</details>

**Terminal:**
```sh
sed -n '220,300p' crates/occt-sys/OCCT/src/OpenGl/OpenGl_Context.hxx
```
<details>
<summary>Output (80 lines)</summary>

```
  //! EGL:
  //! @code
  //!   Handle(Aspect_Window) theAspWin;
  //!   EGLSurface theEglSurf = eglCreateWindowSurface (theEglDisp, anEglConfig,
  //!   (EGLNativeWindowType )theAspWin->NativeHandle(), NULL); EGLDisplay theEglDisp =
  //!   eglGetDisplay (EGL_DEFAULT_DISPLAY); EGLContext theEglCtx  = eglCreateContext ((EGLDisplay
  //!   )theEglDisp, anEglConfig, EGL_NO_CONTEXT, anEglCtxAttribs); Handle(OpenGl_Context) aGlCtx =
  //!   new OpenGl_Context(); aGlCtx->Init ((Aspect_Drawable )theEglSurf, (Aspect_Display
  //!   )theEglDisp,  (Aspect_RenderingContext )theEglCtx);
  //! @endcode
  //!
  //! Windows (Win32):
  //! @code
  //!   Handle(WNT_Window) theAspWin;
  //!   HWND  theWindow   = (HWND )theAspWin->NativeHandle();
  //!   HDC   theDevCtx   = GetDC(theWindow);
  //!   HGLRC theGContext = wglCreateContext (theDevCtx);
  //!   Handle(OpenGl_Context) aGlCtx = new OpenGl_Context();
  //!   aGlCtx->Init ((Aspect_Drawable )theWindow, (Aspect_Display )theDevCtx,
  //!   (Aspect_RenderingContext )theGContext);
  //! @endcode
  //!
  //! Linux (Xlib):
  //! @code
  //!   Handle(Xw_Window) theAspWin;
  //!   Window     theXWindow = (Window )theAspWin->NativeHandle();
  //!   Display*   theXDisp   = (Display* )theAspWin->DisplayConnection()->GetDisplayAspect();
  //!   GLXContext theGlxCtx  = glXCreateContext (theXDisp, aVis.get(), NULL, GL_TRUE);
  //!   Handle(OpenGl_Context) aGlCtx = new OpenGl_Context();
  //!   aGlCtx->Init ((Aspect_Drawable )theXWindow, (Aspect_Display )theXDisp,
  //!   (Aspect_RenderingContext )theGlxCtx);
  //! @endcode
  //!
  //! @param[in] theSurface  surface / window          (EGLSurface | HWND  | GLXDrawable/Window)
  //! @param[in] theDisplay  display or device context (EGLDisplay | HDC   | Display*)
  //! @param[in] theContext  rendering context         (EGLContext | HGLRC | GLXContext |
  //! EAGLContext* | NSOpenGLContext*)
  //! @param[in] theIsCoreProfile  flag indicating that passed GL rendering context has been created
  //! with Core Profile
  //! @return false if OpenGL context can not be bound to specified surface
  Standard_EXPORT Standard_Boolean Init(const Aspect_Drawable         theSurface,
                                        const Aspect_Display          theDisplay,
                                        const Aspect_RenderingContext theContext,
                                        const Standard_Boolean theIsCoreProfile = Standard_False);

  //! Return window handle currently bound to this OpenGL context (EGLSurface | HWND | GLXDrawable).
  Aspect_Drawable Window() const { return myWindow; }

  //! Return display / window device context (EGLDisplay | HDC | Display*).
  Aspect_Display GetDisplay() const { return myDisplay; }

  //! Return rendering context (EGLContext | HGLRC | GLXContext | EAGLContext* | NSOpenGLContext*).
  Aspect_RenderingContext RenderingContext() const { return myGContext; }

#if defined(__APPLE__) && !defined(HAVE_
... (truncated, 3954 chars total)
```
</details>

> 💭 Now I have a clear picture. The `Aspect_RenderingContext` is a platform-specific type alias for an OpenGL context handle:
> - Linux/X11: `GLXContext` (created with `glXCreateContext`)
> - Windows: `HGLRC` (created with `wglCreateContext`)
> - macOS: `NSOpenGLContext*` (created with Cocoa/AppKit APIs)
> - EGL: `EGLContext` (created with `eglCreateContext`)
> - iOS: `EAGLContext*`
> 
> It is NOT something OCCT creates — it's something the caller creates using platform-specific OpenGL APIs, and then passes to OCCT as a type-erased pointer. The `SetWindow` method on `V3d_View` accepts it so you can share an existing GL context with OCCT.
>  
> In typical usage, OCCT generates its own rendering context internally when you call `SetWindow` without providing one, but if you already have an OpenGL context from another library like winit or SDL, you can pass it to OCCT to reuse it. So the caller is responsible for creating the context through platform-specific means and casting it to `Aspect_RenderingContext` — OCCT doesn't provide an API for that.

`Aspect_RenderingContext` is **not created by OCCT** — it's an existing platform OpenGL context handle that the *caller* creates through platform-specific APIs, then casts to this type-erased typedef before passing to OCCT:

- **Linux/X11**: `GLXContext` created via `glXCreateContext()`
- **Windows**: `HGLRC` created via `wglCreateContext()`
- **macOS**: `NSOpenGLContext*` created via AppKit APIs
- **EGL**: `EGLContext` created via `eglCreateContext()`
- **iOS**: `EAGLContext*`

The `SetWindow` method takes an `Aspect_RenderingContext` so you can **share an existing GL context** with OCCT. In normal usage you just pass `NULL` (the default), and OCCT creates its own context internally.

So `Aspect_RenderingContext` is really an escape hatch for context-sharing — if you already have a GL context from e.g. glutin/winit and want OCCT to reuse it. Most OCCT users never pass a non-null value here.

Given that, the pointer-sized opaque type we implemented is arguably overkill — the only callers who would use `SetWindow` with a non-null context are doing platform-specific work anyway and would need to `unsafe`-cast their platform handle into the type. The `unsafe` marker on the generated method is appropriate. Whether it's worth the complexity depends on whether you want Rust users to be able to share their GL context with OCCT at all.

*2026-02-24 08:10 · 118s · Words: 35 in · 536 out · 42,101 ctx · 5 rounds*

---

<a id="user-4"></a>

## User (4)

> I see. OK, revert back to the *mut std::ffi::c_void approach. We shouldn&#x27;t need a pub struct Aspect_RenderingContext in ffi.rs at all, I think.

*2026-02-24 08:15*

### Assistant — Claude Sonnet 4.6

> 💭 The user wants to revert back to the `*mut std::ffi::c_void` approach for `Aspect_RenderingContext`. This means:
> 
> 1. Remove `is_pointer_opaque_type_name()` and related changes from `model.rs`
> 2. Add `Aspect_RenderingContext` back to `is_void_type_name()` in `model.rs`
> 3. Update `type_mapping.rs` to map it to `*mut c_void`
> 4. Update `bindings.rs` to map it to `*mut std::ffi::c_void`
> 5. Revert `codegen/rust.rs` changes
> 
> 
> 6. Check if the `parser.rs` intercept is still needed or if handling it in `is_void_type_name` is sufficient
> 7. Regenerate the bindings
> 
> Let me check the current state of these files to see what's actually changed.

**Todo list:**
- ⬜ Read current state of model.rs
- ⬜ Read type_mapping.rs, bindings.rs, cpp.rs, rust.rs
- ⬜ Apply all revert edits
- ⬜ Regenerate bindings and build

**Todo list:**
- 🔄 Read current state of all affected files
- ⬜ Apply all revert edits
- ⬜ Regenerate bindings and build

Searched for regex `is_pointer_opaque_type_name\|is_void_type_name\|Aspect_RenderingContext` (`**/crates/opencascade-binding-generator/src/**/*.rs`), no results

Searched for regex `is_pointer_opaque_type_name\|is_void_type_name\|Aspect_RenderingContext` (`**/*.rs`), no results

**Terminal:**
```sh
grep -rn "is_pointer_opaque_type_name\|is_void_type_name\|Aspect_RenderingContext" crates/opencascade-binding-generator/src/ 2>&1
```
<details>
<summary>Output (15 lines)</summary>

```
crates/opencascade-binding-generator/src/type_mapping.rs:405:    if crate::model::is_void_type_name(class_name) || crate::model::is_pointer_opaque_type_name(class_name) {
crates/opencascade-binding-generator/src/codegen/cpp.rs:96:                    "void" | "size_t" | "Standard_Address" | "Aspect_RenderingContext"
crates/opencascade-binding-generator/src/codegen/rust.rs:516:        if crate::model::is_pointer_opaque_type_name(&safe_name) {
crates/opencascade-binding-generator/src/codegen/bindings.rs:8:use crate::model::{Constructor, Method, Param, ParsedClass, ParsedField, StaticMethod, Type, is_void_type_name, is_opaque_class_name, std_bitmask_ffi_type};
crates/opencascade-binding-generator/src/codegen/bindings.rs:793:        Type::ConstPtr(inner) if matches!(inner.as_ref(), Type::Class(name) if !is_void_type_name(name)) => {
crates/opencascade-binding-generator/src/codegen/bindings.rs:987:        if !is_void_type_name(name) && !super::rust::is_primitive_type(name) {
crates/opencascade-binding-generator/src/codegen/bindings.rs:3632:                if !is_void_type_name(name) && !super::rust::is_primitive_type(name) {
crates/opencascade-binding-generator/src/parser.rs:2076:        // Aspect_RenderingContext is a platform-dependent typedef:
crates/opencascade-binding-generator/src/parser.rs:2079:        "Aspect_RenderingContext" => Some(Type::Class("Aspect_RenderingContext".to_string())),
crates/opencascade-binding-generator/src/model.rs:420:pub fn is_void_type_name(name: &str) -> bool {
crates/opencascade-binding-generator/src/model.rs:428:pub fn is_pointer_opaque_type_name(name: &str) -> bool {
crates/opencascade-binding-generator/src/model.rs:429:    name == "Aspect_RenderingContext"
crates/opencascade-binding-generator/src/model.rs:448:    name != "char" && !is_void_type_name(name) && !is_pointer_opaque_type_name(name) && std_bitmask_ffi_type(name).is_none()
crates/opencascade-binding-generator/src/model.rs:599:            Type::Class(name) => is_void_type_name(name),
crates/opencascade-binding-generator/src/model.rs:733:        self.is_void_ptr() || self.is_raw_ptr() || matches!(self, Type::Class(name) if is_pointer_opaque_type_name(name))
```
</details>

> 💭 Now I have a clear picture of the current state. I need to:
> 
> 1. **`model.rs`**: 
>    - Remove `is_pointer_opaque_type_name()` function
>    - Add `Aspect_RenderingContext` to `is_void_type_name()` 
>    - Restore `is_opaque_class_name()` to not check `is_pointer_opaque_type_name`
>    - Revert `needs_unsafe_fn()` to not check `is_pointer_opaque_type_name`
> 
> 2. **`type_mapping.rs`**: 
>    - Remove the check for `is_pointer_opaque_type_name` since `is_void_type_name` will now handle `Aspect_RenderingContext`
>    - Add `*mut c_void` mapping for `Aspect_RenderingContext` in the type mapping functions
> 
> 3. **`bindings.rs`**: 
>    - Add `Aspect_RenderingContext` to the `*mut std::ffi::c_void` branch in `type_to_rust_string()`
> 
> 4. **`codegen/rust.rs`**: 
>    - Remove the `is_pointer_opaque_type_name` special case
> 
> 5. **`codegen/cpp.rs`**: 
>    - Keep the skip-include entry for `Aspect_RenderingContext` since we still need to avoid including the platform-specific header
> 
> 6. **`parser.rs`**: 
>    - I need to reconsider whether the intercept in `map_standard_type()` is still necessary if `Aspect_RenderingContext` is now in `is_void_type_name`
> 
> On Linux where it's a `typedef void*`, clang would resolve it through canonical types, but on macOS it resolves to `NSOpenGLContext*` which is unknown, so the intercept is still needed to prevent that canonicalization. The key issue is that both `map_type_to_rust` and `map_type_in_context` need to consistently map `Type::Class("Aspect_RenderingContext")` to `*mut c_void`, but there was a type mismatch when one function returned the opaque struct instead.

Now let me read the specific code sections I need to modify:

**Terminal:**
```sh
grep -n "is_pointer_opaque\|is_void_type\|Aspect_Rendering\|Standard_Address\|void_ptr\|c_void" crates/opencascade-binding-generator/src/model.rs | head -40
echo "==="
grep -n "is_pointer_opaque\|is_void_type\|Aspect_Rendering\|Standard_Address\|c_void" crates/opencascade-binding-generator/src/type_mapping.rs | head -40
echo "==="
grep -n "is_pointer_opaque\|is_void_type\|Aspect_Rendering\|Standard_Address\|c_void" crates/opencascade-binding-generator/src/codegen/rust.rs | head -20
```
<details>
<summary>Output (14 lines)</summary>

```
418:/// Standard_Address is a typedef for void*, and "void" is the parsed
420:pub fn is_void_type_name(name: &str) -> bool {
421:    name == "Standard_Address" || name == "void"
426:/// values (like void*) but get their own named opaque Rust type instead of *mut c_void,
428:pub fn is_pointer_opaque_type_name(name: &str) -> bool {
429:    name == "Aspect_RenderingContext"
448:    name != "char" && !is_void_type_name(name) && !is_pointer_opaque_type_name(name) && std_bitmask_ffi_type(name).is_none()
595:    /// Check if this is a void pointer type (Standard_Address = void*, or literal void*)
596:    /// Methods with these types are bound as `unsafe fn` with `*mut c_void` types.
597:    pub fn is_void_ptr(&self) -> bool {
599:            Type::Class(name) => is_void_type_name(name),
601:                inner.is_void_ptr()
729:    /// function to be marked `unsafe`. True for void pointers (Standard_Address)
733:        self.is_void_ptr() || self.is_raw_ptr() || matches!(self, Type::Class(name) if is_pointer_opaque_type_name(name))
```
</details>

**Terminal:**
```sh
sed -n '418,435p' crates/opencascade-binding-generator/src/model.rs
echo "==="
sed -n '443,452p' crates/opencascade-binding-generator/src/model.rs
echo "==="
sed -n '725,738p' crates/opencascade-binding-generator/src/model.rs
```
<details>
<summary>Output (18 lines)</summary>

```
/// Standard_Address is a typedef for void*, and "void" is the parsed
/// form of literal void* parameters.
pub fn is_void_type_name(name: &str) -> bool {
    name == "Standard_Address" || name == "void"
}

/// Check if a type name is a platform-dependent pointer typedef that should be
/// represented as an opaque pointer-sized type in Rust. These types are pointer-sized
/// values (like void*) but get their own named opaque Rust type instead of *mut c_void,
/// so that the same generated code works across platforms.
pub fn is_pointer_opaque_type_name(name: &str) -> bool {
    name == "Aspect_RenderingContext"
}

/// Check if a class name represents a C++ standard library bitmask type.
/// These types are integer-compatible bitmasks that need explicit `static_cast`
/// in C++ wrapper code because on some platforms (e.g., Linux/libstdc++),
/// they are proper enum types that don't implicitly convert from integers.
```
</details>

**Terminal:**
```sh
sed -n '440,452p' crates/opencascade-binding-generator/src/model.rs
echo "==="
# Also get type_mapping.rs lines around the relevant sections
sed -n '400,415p' crates/opencascade-binding-generator/src/type_mapping.rs
echo "==="
# Get the map_type_to_rust / map_type_in_context sections  
grep -n "Standard_Address\|map_type_to_rust\|map_type_in_context\|fn map_type\|Aspect_Rendering" crates/opencascade-binding-generator/src/type_mapping.rs | head -30
```
<details>
<summary>Output (13 lines)</summary>

```
        _ => None,
    }
}

/// Check if a class name is a real opaque C++ class (not a primitive
/// mapped to a special Rust type like char or void pointer types,
/// and not a standard library bitmask type).
pub fn is_opaque_class_name(name: &str) -> bool {
    name != "char" && !is_void_type_name(name) && !is_pointer_opaque_type_name(name) && std_bitmask_ffi_type(name).is_none()
}

impl Type {
    /// Get a short name for this type (for generating overload suffixes)
```
</details>

**Terminal:**
```sh
grep -n "Standard_Address\|fn map_type_to_rust\|fn map_type_in_context\|fn map_return_type\|is_void_type\|c_void" crates/opencascade-binding-generator/src/type_mapping.rs | head -40
```
<details>
<summary>Output (16 lines)</summary>

```
38:pub fn map_type_to_rust(ty: &Type) -> RustTypeMapping {
205:        Type::Class(class_name) if class_name == "Standard_Address" => {
206:            // Standard_Address is a typedef for void* — map to raw c_void pointer
208:                rust_type: "*mut std::ffi::c_void".to_string(),
215:            // C++ void type (pointee of void*) — just c_void, pointer wrapping handled by outer type
217:                rust_type: "std::ffi::c_void".to_string(),
251:pub fn map_return_type(ty: &Type) -> RustTypeMapping {
404:    // Void pointer types — Standard_Address (void*) and literal "void" — are known
405:    if crate::model::is_void_type_name(class_name) || crate::model::is_pointer_opaque_type_name(class_name) {
455:pub fn map_type_in_context(ty: &Type, ctx: &TypeContext) -> RustTypeMapping {
466:        Type::Class(class_name) if class_name == "Standard_Address" => {
467:            // Standard_Address is a typedef for void* — map to raw c_void pointer
469:                rust_type: "*mut std::ffi::c_void".to_string(),
476:            // C++ void type (pointee of void*) — just c_void, pointer wrapping handled by outer type
478:                rust_type: "std::ffi::c_void".to_string(),
562:pub fn map_return_type_in_context(ty: &Type, ctx: &TypeContext) -> RustTypeMapping {
```
</details>

**Terminal:**
```sh
sed -n '200,230p' crates/opencascade-binding-generator/src/type_mapping.rs
echo "==="
sed -n '460,490p' crates/opencascade-binding-generator/src/type_mapping.rs
```
<details>
<summary>Output (31 lines)</summary>

```
                needs_unique_ptr: true,
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
        Type::Class(class_name) if class_name == "void" => {
            // C++ void type (pointee of void*) — just c_void, pointer wrapping handled by outer type
            RustTypeMapping {
                rust_type: "std::ffi::c_void".to_string(),
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
```
</details>

**Terminal:**
```sh
sed -n '461,495p' crates/opencascade-binding-generator/src/type_mapping.rs
```
<details>
<summary>Output (35 lines)</summary>

```
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
            // Standard library bitmask types (e.g., std::ios_base::openmode)
            // are mapped to their FFI integer type (e.g., u32)
```
</details>

> 💭 Now I have a clear picture. Let me also read the rust.rs section to understand what to change there.

Now I have everything I need. Let me also check the rust.rs section:

**Terminal:**
```sh
sed -n '508,530p' crates/opencascade-binding-generator/src/codegen/rust.rs
```
<details>
<summary>Output (23 lines)</summary>

```
        // Avoid duplicate opaque declarations (flattened nested name might collide
        // with an existing class name or another nested type)
        if defined_classes.contains(&safe_name) || !emitted.insert(safe_name.clone()) {
            continue;
        }

        writeln!(out, "/// Referenced type from C++").unwrap();
        writeln!(out, "#[repr(C)]").unwrap();
        if crate::model::is_pointer_opaque_type_name(&safe_name) {
            writeln!(out, "#[derive(Debug, Clone, Copy)]").unwrap();
            writeln!(out, "pub struct {} {{ _ptr: *mut std::ffi::c_void }}", safe_name).unwrap();
        } else {
            writeln!(out, "pub struct {} {{ _opaque: [u8; 0] }}", safe_name).unwrap();
        }

        // Track types needing destructor generation (skip protected destructor types)
        // This includes nested types (Parent::Nested) and extra typedef types (gp_Vec3f)
        let is_extra_typedef = extra_typedef_names.contains(type_name);
        if (is_nested || is_extra_typedef) && !has_protected_dtor {
            nested_types.push(NestedTypeInfo {
                cpp_name: type_name.clone(),
                ffi_name: safe_name,
            });
```
</details>

> 💭 Now I have a complete picture. Here's what I need to do:
> 
> 1. **`model.rs`**:
>    - Remove `is_pointer_opaque_type_name()` function (lines ~426-430)
>    - Add `Aspect_RenderingContext` to `is_void_type_name()` 
>    - Restore `is_opaque_class_name()` to not check `is_pointer_opaque_type_name`
>    - Revert `needs_unsafe_fn()` to not check `is_pointer_opaque_type_name`
>    
> 2. **`type_mapping.rs`**:
>    - Add `Aspect_RenderingContext` to the `*mut c_void` cases in both `map_type_to_rust` and `map_type_in_context`
>    - Remove `is_pointer_opaque_type_name` from `is_class_name_unknown` (since `is_void_type_name` now covers it)
> 
> 3. **`bindings.rs`**:
>    - Add `Aspect_RenderingContext` to the `*mut std::ffi::c_void` branch in `type_to_rust_string()`
> 
> 4. **`codegen/rust.rs`**:
>    - Remove the `is_pointer_opaque_type_name` special case
> 
> 5. **`codegen/cpp.rs`**: 
>    - Keep as is (still need to avoid including Aspect_RenderingContext.hxx)
> 
> 6. **`parser.rs`**: 
>    - Keep as is (still need to intercept before clang canonicalization)
> 
> But I'm realizing there's a potential issue here. If `Aspect_RenderingContext` goes into `is_void_type_name`, then `is_void_ptr()` returns true for it, which means `is_unbindable()` would also return true. That would cause methods with this type to be skipped entirely. However, the conversation summary shows the first approach actually worked and tests passed, so `has_unbindable_types()` must be allowing void pointer types through somehow. Let me trace through the logic more carefully to understand how `Standard_Address` methods are currently being generated despite being void pointers.

**Terminal:**
```sh
grep -n "fn is_unbindable\|fn is_void_ptr\|fn has_unbindable\|is_void_ptr\|is_stream\|is_array\|is_rvalue" crates/opencascade-binding-generator/src/model.rs | head -30
```
<details>
<summary>Output (15 lines)</summary>

```
49:    pub fn has_unbindable_types(&self) -> bool {
207:    pub fn has_unbindable_types(&self) -> bool {
242:    pub fn has_unbindable_types(&self) -> bool {
298:    pub fn has_unbindable_types(&self) -> bool {
556:    pub fn is_stream(&self) -> bool {
564:            Type::ConstRef(inner) | Type::MutRef(inner) | Type::RValueRef(inner) => inner.is_stream(),
597:    pub fn is_void_ptr(&self) -> bool {
601:                inner.is_void_ptr()
608:    pub fn is_array(&self) -> bool {
613:                inner.is_array()
712:    pub fn is_rvalue_ref(&self) -> bool {
722:    pub fn is_unbindable(&self) -> bool {
723:        (self.is_array() && !self.is_fixed_array_ref() && !self.is_fixed_array_param())
724:            || self.is_rvalue_ref()
733:        self.is_void_ptr() || self.is_raw_ptr() || matches!(self, Type::Class(name) if is_pointer_opaque_type_name(name))
```
</details>

> 💭 Good - `is_unbindable()` does NOT include `is_void_ptr()`. It only includes `is_array()`, `is_rvalue_ref()`, and some other things. `is_void_ptr()` only feeds into `needs_unsafe_fn()`. So adding `Aspect_RenderingContext` to `is_void_type_name` won't cause methods to be skipped - it will just mark them as `unsafe`.
> 
> Now let me also check `has_unbindable_types()` more carefully for Method:

**Terminal:**
```sh
sed -n '205,220p' crates/opencascade-binding-generator/src/model.rs
```
<details>
<summary>Output (16 lines)</summary>

```
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
```
</details>

> 💭 Good. `is_unbindable()` does not include `is_void_ptr()`, so methods with `Standard_Address` (or `Aspect_RenderingContext` if we put it in `is_void_type_name`) won't be skipped. They'll just be `unsafe`.
> 
> Now let me also check what `type_uses_unknown_class` does with void type names:
> 
> Looking at type_mapping.rs line 404-405:
> if crate::model::is_void_type_name(class_name) || crate::model::is_pointer_opaque_type_name(class_name) {

Good. `is_unbindable()` does NOT include `is_void_ptr()`, so void-pointer methods get generated (just as `unsafe`). Now I'll make all the changes:

<details>
<summary>Reading lines 418-450 of <a href="../crates/opencascade-binding-generator/src/model.rs#L418-L451">model.rs</a></summary>

```
/// Standard_Address is a typedef for void*, and "void" is the parsed
/// form of literal void* parameters.
pub fn is_void_type_name(name: &str) -> bool {
    name == "Standard_Address" || name == "void"
}

/// Check if a type name is a platform-dependent pointer typedef that should be
/// represented as an opaque pointer-sized type in Rust. These types are pointer-sized
/// values (like void*) but get their own named opaque Rust type instead of *mut c_void,
/// so that the same generated code works across platforms.
pub fn is_pointer_opaque_type_name(name: &str) -> bool {
    name == "Aspect_RenderingContext"
}

/// Check if a class name represents a C++ standard library bitmask type.
/// These types are integer-compatible bitmasks that need explicit `static_cast`
/// in C++ wrapper code because on some platforms (e.g., Linux/libstdc++),
/// they are proper enum types that don't implicitly convert from integers.
/// Returns the corresponding FFI integer type if it is a bitmask type.
pub fn std_bitmask_ffi_type(name: &str) -> Option<Type> {
    match name {
        "std::ios_base::openmode" => Some(Type::U32),
        _ => None,
    }
}

/// Check if a class name is a real opaque C++ class (not a primitive
/// mapped to a special Rust type like char or void pointer types,
/// and not a standard library bitmask type).
pub fn is_opaque_class_name(name: &str) -> bool {
    name != "char" && !is_void_type_name(name) && !is_pointer_opaque_type_name(name) && std_bitmask_ffi_type(name).is_none()
}

```
</details>

<details>
<summary>Reading lines 728-736 of <a href="../crates/opencascade-binding-generator/src/model.rs#L728-L737">model.rs</a></summary>

```
    /// Check if this type involves raw pointers that require the containing
    /// function to be marked `unsafe`. True for void pointers (Standard_Address)
    /// and raw T*/const T* pointers (excluding const char* which is handled
    /// as C strings).
    pub fn needs_unsafe_fn(&self) -> bool {
        self.is_void_ptr() || self.is_raw_ptr() || matches!(self, Type::Class(name) if is_pointer_opaque_type_name(name))
    }

    /// Convert this type to a C++ parameter type for extern "C" wrapper functions.
```
</details>

<details>
<summary>Reading lines 400-410 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L400-L411">type_mapping.rs</a></summary>

```
fn is_class_name_unknown(class_name: &str, all_classes: &std::collections::HashSet<String>) -> bool {
    if all_classes.contains(class_name) {
        return false;
    }
    // Void pointer types — Standard_Address (void*) and literal "void" — are known
    if crate::model::is_void_type_name(class_name) || crate::model::is_pointer_opaque_type_name(class_name) {
        return false;
    }
    // Primitive types mapped as Type::Class (e.g., "char" from Standard_Character)
    if crate::codegen::rust::is_primitive_type(class_name) {
        return false;
```
</details>

<details>
<summary>Reading lines 202-220 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L202-L221">type_mapping.rs</a></summary>

```
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
        Type::Class(class_name) if class_name == "void" => {
            // C++ void type (pointee of void*) — just c_void, pointer wrapping handled by outer type
            RustTypeMapping {
                rust_type: "std::ffi::c_void".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: None,
```
</details>

<details>
<summary>Reading lines 464-483 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L464-L484">type_mapping.rs</a></summary>

```
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
```
</details>

<details>
<summary>Reading lines 743-754 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L743-L755">bindings.rs</a></summary>

```
                "std::ffi::c_void".to_string()
            } else if let Some(ffi_ty) = std_bitmask_ffi_type(name) {
                ffi_ty.to_rust_type_string()
            } else if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
                format!("crate::ffi::{}", Type::ffi_safe_class_name(name))
            }
        }
        Type::Handle(name) => format!("crate::ffi::{}", type_mapping::handle_type_name(name)),
        Type::FixedArray(inner, size) => {
            format!("[{}; {}]", type_to_rust_string(inner, reexport_ctx), size)
```
</details>

<details>
<summary>Reading lines 728-745 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L728-L746">bindings.rs</a></summary>

```
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
            } else if let Some(ffi_ty) = std_bitmask_ffi_type(name) {
                ffi_ty.to_rust_type_string()
```
</details>

<details>
<summary>Reading lines 512-527 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L512-L528">rust.rs</a></summary>

```
        }

        writeln!(out, "/// Referenced type from C++").unwrap();
        writeln!(out, "#[repr(C)]").unwrap();
        if crate::model::is_pointer_opaque_type_name(&safe_name) {
            writeln!(out, "#[derive(Debug, Clone, Copy)]").unwrap();
            writeln!(out, "pub struct {} {{ _ptr: *mut std::ffi::c_void }}", safe_name).unwrap();
        } else {
            writeln!(out, "pub struct {} {{ _opaque: [u8; 0] }}", safe_name).unwrap();
        }

        // Track types needing destructor generation (skip protected destructor types)
        // This includes nested types (Parent::Nested) and extra typedef types (gp_Vec3f)
        let is_extra_typedef = extra_typedef_names.contains(type_name);
        if (is_nested || is_extra_typedef) && !has_protected_dtor {
            nested_types.push(NestedTypeInfo {
```
</details>

> 💭 Now I have all the read data I need. Let me apply all the changes using `hashline_edit`:
> 
> 1. **`model.rs`** changes:
>    - Line 421: Change `is_void_type_name` to add `Aspect_RenderingContext`
>    - Lines 424-430: Remove the `is_pointer_opaque_type_name` function block
>    - Line 448: Fix `is_opaque_class_name` to remove `!is_pointer_opaque_type_name(name)` 
>    - Line 733: Fix `needs_unsafe_fn` to remove `|| matches!(self, Type::Class(name) if is_pointer_opaque_type_name(name))`
>    
> 2. **`type_mapping.rs`** changes:
>    - Line 205: Change `Standard_Address` guard to also include `Aspect_RenderingContext`
>    - Line 466: Same for `map_type_in_context`
>    - Line 405: Remove `|| crate::model::is_pointer_opaque_type_name(class_name)`
> 
> 3. **`bindings.rs`** changes:
>    - Line 740: Change `Standard_Address` to also include `Aspect_RenderingContext`
> 
> 4. **`codegen/rust.rs`** changes:
>    - Lines 516-521: Updating type handling logic...
> 
> Now I'm planning the batch edits. For `model.rs`, I need to update `is_void_type_name` to include `Aspect_RenderingContext`, remove the `is_pointer_opaque_type_name` function entirely (lines 424-431), update `is_opaque_class_name`, and fix the `needs_unsafe_fn` method. For the type mapping changes, I'll add the `Aspect_RenderingContext` check using `||` pattern guards. I'll delete lines 424-431 to remove the now-unused function while keeping proper spacing, then apply all the remaining updates across the files.

Now I have all the context I need. Let me apply all changes in one batch: model.rs (4 changes), type_mapping.rs (3 changes), bindings.rs (1 change), rust.rs (1 change):

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L418-L420">model.rs -2+2</a>, <a href="../crates/opencascade-binding-generator/src/model.rs#L421-L422">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/model.rs#L424">-8</a>, <a href="../crates/opencascade-binding-generator/src/model.rs#L440-L441">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/model.rs#L725-L726">-1+1</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L205-L206">type_mapping.rs -1+1</a>, <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L405-L406">-1+1</a>, <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L466-L467">-1+1</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L740-L741">bindings.rs -1+1</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L516-L517">rust.rs -6+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "418:uc,419:ex",
      "content": "/// Standard_Address and Aspect_RenderingContext are typedefs for void* or platform pointers,\n/// and \"void\" is the parsed form of literal void* parameters."
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "421:ls",
      "content": "    name == \"Standard_Address\" || name == \"void\" || name == \"Aspect_RenderingContext\""
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "424:wo,425:an,426:we,427:ta,428:up,429:ma,430:io,431:dd",
      "content": ""
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "448:ra",
      "content": "    name != \"char\" && !is_void_type_name(name) && std_bitmask_ffi_type(name).is_none()"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "733:wp",
      "content": "        self.is_void_ptr() || self.is_raw_ptr()"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "205:by",
      "content": "        Type::Class(class_name) if class_name == \"Standard_Address\" || class_name == \"Aspect_RenderingContext\" => {"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "405:kc",
      "content": "    if crate::model::is_void_type_name(class_name) {"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "466:by",
      "content": "        Type::Class(class_name) if class_name == \"Standard_Address\" || class_name == \"Aspect_RenderingContext\" => {"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "740:sq",
      "content": "            } else if name == \"Standard_Address\" || name == \"Aspect_RenderingContext\" {"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "lineHashes": "516:oy,517:tc,518:pn,519:ag,520:mk,521:cy",
      "content": "        writeln!(out, \"pub struct {} {{ _opaque: [u8; 0] }}\", safe_name).unwrap();"
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
      "file": "crates/opencascade-binding-generator/src/model.rs",
      "startLine": 418,
      "endLine": 725
    },
    {
      "file": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "startLine": 205,
      "endLine": 466
    },
    {
      "file": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "startLine": 740,
      "endLine": 740
    },
    {
      "file": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "startLine": 516,
      "endLine": 516
    }
  ]
}
```
</details>

10/10 applied. Now check for any remaining references to `is_pointer_opaque_type_name`:

**Terminal:**
```sh
grep -rn "is_pointer_opaque_type_name" crates/opencascade-binding-generator/src/ 2>&1
```
**Exit code:** 1

Clean. Now build check and regenerate:

**Todo list:**
- ✅ Read current state of all affected files
- ✅ Apply all revert edits
- 🔄 Regenerate bindings and build

**Terminal:**
```sh
time cargo check -p opencascade-binding-generator 2>&1
```
```
    Blocking waiting for file lock on build directory
    Checking opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 30.86s
real  31.0s
```

**Terminal:**
```sh
time scripts/regenerate-bindings.sh 2>&1 | tail -5
```

**Terminal:**
```sh
git diff HEAD -- crates/opencascade-sys/generated/ffi.rs | grep "Aspect_RenderingContext\|^\+" | head -20
```
<details>
<summary>Output (7 lines)</summary>

```
+++ b/crates/opencascade-sys/generated/ffi.rs
+pub struct Aspect_RenderingContext {
+    _opaque: [u8; 0],
+}
+/// Referenced type from C++
+#[repr(C)]
+    pub fn V3d_View_set_window_handleaspectwindow_renderingcontext(
```
</details>

> 💭 Hmm, there's still a `pub struct Aspect_Rendering

> 💭 Hmm, there's still a `pub struct Aspect_RenderingContext` being generated. This means the type is still being treated as an opaque class (not a void type). Let me investigate why.
> 
> Looking at the diff: there's still a `pub struct Aspect_RenderingContext { _opaque: [u8; 0], }` being generated. This code is generated by `generate_opaque_declarations()` which is called for classes that appear in the opaque type list.
> 
> But wait - if `Aspect_RenderingContext` is now in `is_void_type_name`, then `is_opaque_class_name` returns false for it.

**Terminal:**
```sh
git diff HEAD -- crates/opencascade-sys/generated/ffi.rs | grep -A3 -B3 "Aspect_RenderingContext"
```
<details>
<summary>Output (7 lines)</summary>

```
 }
 /// Referenced type from C++
 #[repr(C)]
+pub struct Aspect_RenderingContext {
+    _opaque: [u8; 0],
+}
+/// Referenced type from C++
```
</details>

**Terminal:**
```sh
grep -n "Aspect_RenderingContext\|renderingcontext\|rendering_context" crates/opencascade-sys/generated/ffi.rs | head -20
```
```
35264:pub struct Aspect_RenderingContext {
641526:    pub fn V3d_View_set_window_handleaspectwindow_renderingcontext(
```

**Terminal:**
```sh
sed -n '641522,641535p' crates/opencascade-sys/generated/ffi.rs
```
<details>
<summary>Output (14 lines)</summary>

```
    ) -> *mut V3d_View;
    /// **Source:** `V3d_View.hxx`:58 - `V3d_View::DynamicType()`
    pub fn V3d_View_dynamic_type(self_: *const V3d_View) -> *const HandleStandardType;
    /// **Source:** `V3d_View.hxx`:76 - `V3d_View::SetWindow()`
    pub fn V3d_View_set_window_handleaspectwindow_renderingcontext(
        self_: *mut V3d_View,
        theWindow: *const HandleAspectWindow,
        theContext: *mut std::ffi::c_void,
    );
    /// **Source:** `V3d_View.hxx`:93 - `V3d_View::SetWindow()`
    pub fn V3d_View_set_window_handlev3dview_vec2d_typeoftriedronposition_vec2d_vec2i(
        self_: *mut V3d_View,
        theParentView: *const HandleV3dView,
        theSize: *const Graphic3d_Vec2d,
```
</details>

> 💭 The `set_window` function is correctly using `*mut std::ffi::c_void` - that's correct. But there's still a `pub struct Aspect_RenderingContext` being generated. This struct is being generated even though the type is now mapped to `*mut c_void` in the function signatures.
> 
> The question is - why is the struct still being generated? Let me look at what feeds `generate_opaque_declarations()` - it seems to collect all referenced types from the classes, and `Aspect_RenderingContext` might still be added to the "referenced types" list even though it's mapped to `*mut c_void` in function signatures.

The function signature is correct (`*mut std::ffi::c_void`) but the struct is still being emitted. Let me find where it's getting added to the referenced types list:

**Terminal:**
```sh
grep -n "referenced_types\|opaque_decl\|generate_opaque\|extra_class\|referenced_class" crates/opencascade-binding-generator/src/codegen/rust.rs | head -30
```
```
41:pub fn collect_referenced_types(
172:    let collected_types = collect_referenced_types(all_classes, all_functions, handle_able_classes);
173:    let (opaque_type_decls, nested_types) = generate_opaque_declarations(
464:fn generate_opaque_declarations(
```

**Terminal:**
```sh
sed -n '41,130p' crates/opencascade-binding-generator/src/codegen/rust.rs
```
<details>
<summary>Output (90 lines)</summary>

```
pub fn collect_referenced_types(
    classes: &[&ParsedClass],
    functions: &[&crate::model::ParsedFunction],
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

        crate::model::for_each_type_in_class(class, &mut |ty| {
            collect_types_from_type(ty, &mut result);
        });
    }

    for func in functions {
        crate::model::for_each_type_in_function(func, &mut |ty| {
            collect_types_from_type(ty, &mut result);
        });
    }

    result
}

/// Recursively collect OCCT class and Handle types from a type.
/// Uses `visit_inner` to unwrap reference/pointer wrappers.
fn collect_types_from_type(ty: &Type, collected: &mut CollectedTypes) {
    // Skip unbindable types (arrays, rvalue refs, etc.) — but NOT streams,
    // which are manually-defined opaque types that we DO want to collect.
    if ty.is_unbindable() {
        return;
    }

    ty.visit_inner(&mut |inner| {
        match inner {
            Type::Class(name) => {
                // Skip primitive types and template instantiations (e.g., NCollection_Shared<...>)
                if !is_primitive_type(name) && !name.contains('<') {
                    collected.classes.insert(name.clone());
                }
            }
            Type::Handle(name) => {
                // Record the Handle type AND the inner class
                if !name.contains('<') {
                    collected.handles.insert(name.clone());
                    collected.classes.insert(name.clone());
                }
            }
            _ => {}
        }
    });
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
//
... (truncated, 3433 chars total)
```
</details>

> 💭 I see the issue now. In `collect_types_from_type`, when it encounters `Type::Class("Aspect_RenderingContext")`, it checks if it's a primitive type, and since it's not, it adds it to `collected.classes`. This then causes `generate_opaque_declarations` to emit a struct for it.
> 
> The fix is to also check `is_void_type_name` in `collect_types_from_type`:
> 
> Type::Class(name) => {
>     if !is_primitive_type(name) && !name.contains('<') && !crate::model::is_void_type_name(name) {

The issue is in `collect_types_from_type` — it adds `Type::Class` names to the opaque declarations without checking `is_void_type_name`. Let me fix it:

<details>
<summary>Reading lines 81-94 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L81-L95">rust.rs</a></summary>

```

    ty.visit_inner(&mut |inner| {
        match inner {
            Type::Class(name) => {
                // Skip primitive types and template instantiations (e.g., NCollection_Shared<...>)
                if !is_primitive_type(name) && !name.contains('<') {
                    collected.classes.insert(name.clone());
                }
            }
            Type::Handle(name) => {
                // Record the Handle type AND the inner class
                if !name.contains('<') {
                    collected.handles.insert(name.clone());
                    collected.classes.insert(name.clone());
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L85-L87">rust.rs -2+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "                // Skip primitive types, void pointer types, and template instantiations (e.g., NCollection_Shared<...>)\n                if !is_primitive_type(name) && !name.contains('<') && !crate::model::is_void_type_name(name) {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "lineHashes": "85:ew,86:np"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/rust.rs","startLine":85,"endLine":86}
```
</details>

**Terminal:**
```sh
time cargo check -p opencascade-binding-generator 2>&1
```
```
    Checking opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.00s
real  1.0s
```

**Terminal:**
```sh
time scripts/regenerate-bindings.sh 2>&1 | tail -5
```

**Terminal:**
```sh
grep -n "Aspect_RenderingContext" crates/opencascade-sys/generated/ffi.rs | head -10
echo "==="
grep -n "Aspect_RenderingContext\|renderingcontext" crates/opencascade-sys/generated/v3d.rs | head -10
echo "==="
grep -n "Aspect_RenderingContext" crates/opencascade-sys/generated/aspect.rs | head -5
```

**Terminal:**
```sh
grep -n "renderingcontext\|c_void" crates/opencascade-sys/generated/ffi.rs | grep -i "window\|v3d\|rendering" | head -10
echo "==="
grep -rn "renderingcontext" crates/opencascade-sys/generated/ | head -10
```
```
641511:    pub fn V3d_View_set_window_handleaspectwindow_renderingcontext(
```

**Terminal:**
```sh
sed -n '641508,641520p' crates/opencascade-sys/generated/ffi.rs
```
<details>
<summary>Output (13 lines)</summary>

```
    /// **Source:** `V3d_View.hxx`:58 - `V3d_View::DynamicType()`
    pub fn V3d_View_dynamic_type(self_: *const V3d_View) -> *const HandleStandardType;
    /// **Source:** `V3d_View.hxx`:76 - `V3d_View::SetWindow()`
    pub fn V3d_View_set_window_handleaspectwindow_renderingcontext(
        self_: *mut V3d_View,
        theWindow: *const HandleAspectWindow,
        theContext: *mut std::ffi::c_void,
    );
    /// **Source:** `V3d_View.hxx`:93 - `V3d_View::SetWindow()`
    pub fn V3d_View_set_window_handlev3dview_vec2d_typeoftriedronposition_vec2d_vec2i(
        self_: *mut V3d_View,
        theParentView: *const HandleV3dView,
        theSize: *const Graphic3d_Vec2d,
```
</details>

No `Aspect_RenderingContext` struct in ffi.rs, and `theContext` is `*mut std::ffi::c_void`. Now build and test:

**Terminal:**
```sh
time cargo build 2>&1 | grep -E "^error|warning.*unused|Compiling opencascade|Finished"
```
<details>
<summary>Output (5 lines)</summary>

```
   Compiling opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
   Compiling opencascade v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 02s
real  62.9s
```
</details>

**Terminal:**
```sh
time scripts/run-binding-generator-tests.sh 2>&1 | tail -5
```
<details>
<summary>Output (6 lines)</summary>

```

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

real  5.4s
```
</details>

**Terminal:**
```sh
time cargo test --workspace --exclude opencascade-binding-generator 2>&1 | grep -E "test result|error|FAILED"
```

All passing. Here's a summary of what was done:

**Todo list:**
- ✅ Read current state of all affected files
- ✅ Apply all revert edits
- ✅ Regenerate bindings and build

Done. The changes made:

- **`model.rs`**: Removed `is_pointer_opaque_type_name()`, added `"Aspect_RenderingContext"` to `is_void_type_name()`, simplified `is_opaque_class_name()` and `needs_unsafe_fn()` accordingly
- **`type_mapping.rs`**: Added `Aspect_RenderingContext` to the `*mut c_void` arm in both `map_type_to_rust` and `map_type_in_context`; simplified `is_class_name_unknown` to just use `is_void_type_name`
- **`codegen/bindings.rs`**: Added `Aspect_RenderingContext` to the `*mut std::ffi::c_void` branch in `type_to_rust_string()`
- **`codegen/rust.rs`**: Removed the pointer-opaque struct special case; also fixed Done. The changes made:

- **`model.rs`**: Removed `is_pointer_opaque_type_name()`, added `"Aspect_RenderingContext"` to `is_void_type_name()`, simplified `is_opaque_class_name()` and `needs_unsafe_fn()` accordingly
- **`type_mapping.rs`**: Added `Aspect_RenderingContext` to the `*mut c_void` arm in both `map_type_to_rust` and `map_type_in_context`; simplified `is_class_name_unknown` to just use `is_void_type_name`
- **`codegen/bindings.rs`**: Added `Aspect_RenderingContext` to the `*mut std::ffi::c_void` branch in `type_to_rust_string()`
- **`codegen/rust.rs`**: Removed the pointer-opaque struct special case; also fixed `collect_types_from_type` to skip void type names (preventing the spurious struct from being emitted)

The generated `ffi.rs` now has no `Aspect_RenderingContext` struct — `SetWindow`'s `theContext` parameter is simply `*mut std::ffi::c_void`.

*2026-02-24 08:27 · 701s · Words: 909 in · 2,322 out · 71,402 ctx · 45 rounds*

---

<a id="user-5"></a>

## User (5)

> Commit. Then Move this into bindings.toml. Decide on a good way to represent custom mappings in bindings.toml and implement it. Then commit. Then think about whether you encountered code that deserves refactoring in this session. If so, refactor and commit.

*2026-02-24 08:35*

### Assistant — Claude Sonnet 4.6

> 💭 Let me understand what the user wants:
> 
> 1. **Commit the current state** - commit all the recent changes (the `*mut c_void` approach)
> 2. **Move the `Aspect_RenderingContext` → `*mut c_void` mapping into `bindings.toml`** - rather than hardcoded in the generator, let it be configurable
> 3. **Design and implement a good way to represent custom type mappings in bindings.toml**
> 4. **Commit that change**
> 5. **Think about refactoring opportunities from this session** and potentially refactor + commit
> 
> Let me start by understanding the current state and the bindings.toml format.

**Todo list:**
- ⬜ Commit current state
- ⬜ Read bindings.toml and generator config structures
- ⬜ Design custom_type_mappings in bindings.toml
- ⬜ Implement custom_type_mappings in generator
- ⬜ Regenerate, build, test, commit
- ⬜ Identify and apply refactoring opportunities

**Todo list:**
- 🔄 Commit current state
- ⬜ Read bindings.toml and generator config structures
- ⬜ Design custom_type_mappings in bindings.toml
- ⬜ Implement custom_type_mappings in generator
- ⬜ Regenerate, build, test, commit
- ⬜ Identify and apply refactoring opportunities

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git status --short 2>&1 | head -30
```
<details>
<summary>Output (11 lines)</summary>

```
 ? crates/occt-sys/OCCT
 M crates/opencascade-binding-generator/src/codegen/bindings.rs
 M crates/opencascade-binding-generator/src/codegen/cpp.rs
 M crates/opencascade-binding-generator/src/codegen/rust.rs
 M crates/opencascade-binding-generator/src/model.rs
 M crates/opencascade-binding-generator/src/parser.rs
 M crates/opencascade-binding-generator/src/type_mapping.rs
 M crates/opencascade-sys/generated/ffi.rs
 M crates/opencascade-sys/generated/standard.rs
 M crates/opencascade-sys/generated/v3d.rs
 M crates/opencascade-sys/generated/wrappers.cpp
```
</details>

**Terminal:**
```sh
cat > tmp/commit1.txt << 'EOF'
Handle Aspect_RenderingContext as *mut c_void in FFI

Aspect_RenderingContext is a platform-dependent typedef:
- void* on Linux
- NSOpenGLContext* on macOS
- EAGLContext* on iOS

Previously the generator would canonicalize it through clang, which
produced an unresolvable type on macOS, causing SetWindow methods on
V3d_View and Graphic3d_CView to be skipped.

Fix by intercepting "Aspect_RenderingContext" in map_standard_type()
before canonical resolution, treating it as a void-pointer type
alongside Standard_Address. It maps to *mut std::ffi::c_void at the
FFI boundary, which produces correct unsafe wrappers on all platforms.

Changes:
- parser.rs: intercept Aspect_RenderingContext in map_standard_type()
- model.rs: add to is_void_type_name() so is_void_ptr() detects it
- type_mapping.rs: map to *mut c_void in both mapping functions;
  is_class_name_unknown() recognizes it as known via is_void_type_name
- codegen/bindings.rs: type_to_rust_string() maps to *mut c_void
- codegen/cpp.rs: skip generating #include for Aspect_RenderingContext
- codegen/rust.rs: skip emitting opaque struct for void type names
- generated/: V3d_View::SetWindow and Graphic3d_CView::SetWindow now
  appear with *mut std::ffi::c_void parameter
EOF
git add crates/opencascade-binding-generator/src/ crates/opencascade-sys/generated/
git commit -F tmp/commit1.txt
```

> 💭 The heredoc got confused. Let me write the commit message to a file using Python instead.

The heredoc got confused by the terminal. Let me write the file differently:

Created [commit1.txt](../tmp/commit1.txt)

**Terminal:**
```sh
git add crates/opencascade-binding-generator/src/ crates/opencascade-sys/generated/
git commit -F tmp/commit1.txt
```
```
(base) whitelily:opencascade-rs sethml$ git commit -F tmp/commit1.txt
```

**Terminal:**
```sh
git log --oneline -3
```
```
1c50ffc (HEAD -> main) Handle Aspect_RenderingContext as *mut c_void in FFI
3c30e2b Make golden tests numeric-tolerant and restore golden artifacts
09d1426 (origin/main, origin/HEAD) Rewrite top-level README for auto-generated binding architecture
```

**Todo list:**
- ✅ Commit current state
- 🔄 Read bindings.toml and generator config structures
- ⬜ Design custom_type_mappings in bindings.toml
- ⬜ Implement custom_type_mappings in generator
- ⬜ Regenerate, build, test, commit
- ⬜ Identify and apply refactoring opportunities

Now let me study bindings.toml and the config parsing code:

**Terminal:**
```sh
wc -l crates/opencascade-sys/bindings.toml
head -80 crates/opencascade-sys/bindings.toml
```
```
     130 crates/opencascade-sys/bindings.toml
```

Read [bindings.toml](../crates/opencascade-sys/bindings.toml#1-1), lines 1 to 130

> 💭 Now I need to understand the config structures in the generator code. Let me find where BindingsConfig is defined and how it's parsed.

Now let me find the config struct and how it's loaded:

**Terminal:**
```sh
grep -rn "BindingsConfig\|struct.*Config\|fn load\|toml::from\|manual_types\|occt_alias" crates/opencascade-binding-generator/src/ | grep -v "\.toml" | head -40
```
<details>
<summary>Output (19 lines)</summary>

```
crates/opencascade-binding-generator/src/config.rs:29:pub struct BindingConfig {
crates/opencascade-binding-generator/src/config.rs:76:    pub manual_types: std::collections::HashMap<String, ManualType>,
crates/opencascade-binding-generator/src/config.rs:89:    pub occt_alias_type_overrides: std::collections::HashMap<String, String>,
crates/opencascade-binding-generator/src/config.rs:112:pub fn load_config(path: &Path) -> Result<BindingConfig> {
crates/opencascade-binding-generator/src/config.rs:116:        toml::from_str(&content).with_context(|| format!("Failed to parse config file: {}", path.display()))?;
crates/opencascade-binding-generator/src/main.rs:105:    occt_alias_type_overrides: HashMap<String, String>,
crates/opencascade-binding-generator/src/main.rs:152:        let manual_names: HashSet<String> = cfg.manual_types.keys().cloned().collect();
crates/opencascade-binding-generator/src/main.rs:154:        let occt_alias_overrides = cfg.occt_alias_type_overrides;
crates/opencascade-binding-generator/src/main.rs:164:            occt_alias_type_overrides: occt_alias_overrides,
crates/opencascade-binding-generator/src/main.rs:176:            occt_alias_type_overrides: HashMap::new(),
crates/opencascade-binding-generator/src/main.rs:191:        occt_alias_type_overrides,
crates/opencascade-binding-generator/src/main.rs:257:        &occt_alias_type_overrides,
crates/opencascade-binding-generator/src/parser.rs:74:fn normalize_occt_alias_type_name(type_name: &str) -> String {
crates/opencascade-binding-generator/src/parser.rs:85:    Type::Class(normalize_occt_alias_type_name(type_name))
crates/opencascade-binding-generator/src/parser.rs:379:fn set_occt_alias_type_overrides(overrides: &HashMap<String, String>) {
crates/opencascade-binding-generator/src/parser.rs:399:    occt_alias_type_overrides: &HashMap<String, String>,
crates/opencascade-binding-generator/src/parser.rs:407:    set_occt_alias_type_overrides(occt_alias_type_overrides);
crates/opencascade-binding-generator/src/parser.rs:1914:        return Type::Handle(normalize_occt_alias_type_name(&inner));
crates/opencascade-binding-generator/src/parser.rs:2081:        // same Type::Class so they're recognized as known manual_types.
```
</details>

Read [config.rs](../crates/opencascade-binding-generator/src/config.rs#1-1), lines 1 to 130

> 💭 Now I understand the config structure. I need to:
> 
> 1. Add a `custom_type_mappings` section to `bindings.toml` and `BindingConfig`
> 2. This section should map C++ type names to Rust types with specific handling
> 3. For `Aspect_RenderingContext`, we want to map it to `*mut std::ffi::c_void`
> 
> The key design question is: what information should each mapping capture?
> 
> Looking at the existing implementation, there are several things the generator needs to know about a type:
> 1. The Rust FFI type string (e.g., `*mut std::ffi::c_void`)
> 
> 
> 2. Whether it's a "void type" (for `is_void_ptr()`, `is_void_type_name()`)
> 3. Whether to skip the #include in wrappers.cpp
> 4. Whether to skip generating an opaque struct in ffi.rs
> 5. Whether methods using this type should be `unsafe`
> 
> Looking at how `Standard_Address` is handled, it's returned as `Type::Class("Standard_Address")` from `map_standard_type()`, marked as void via `is_void_type_name()`, makes functions unsafe via `is_void_ptr()`, maps to `*mut std::ffi::c_void` in type_mapping.rs, and is excluded from opaque struct and #include generation. I want `Aspect_RenderingContext` to behave the same way.
> 
> The simplest design is a `void_pointer_types` list in bindings.toml that captures exactly this behavior: these types map to `*mut c_void`, make methods unsafe, and skip struct/include generation. This is more self-documenting than a generic `custom_type_mappings` table, and it matches the actual use case.
> 
> For implementation, I need to add a `void_pointer_types: Vec<String>` field to `BindingConfig`, update bindings.toml with the new entry, and modify `is_void_type_name()` to check this list. Since the existing code already uses a thread-local global for `OCCT_ALIAS_OVERRIDES`, I can follow the same pattern to make the config accessible throughout the generator.

Now let me look at main.rs and how config flows into the generator, plus how global state is set:

**Terminal:**
```sh
grep -n "occt_alias\|void_pointer\|is_void\|VOID\|manual_types\|set.*override\|GlobalState\|static\|OnceLock\|thread_local" crates/opencascade-binding-generator/src/main.rs | head -40
```
<details>
<summary>Output (31 lines)</summary>

```
105:    occt_alias_type_overrides: HashMap<String, String>,
152:        let manual_names: HashSet<String> = cfg.manual_types.keys().cloned().collect();
154:        let occt_alias_overrides = cfg.occt_alias_type_overrides;
164:            occt_alias_type_overrides: occt_alias_overrides,
176:            occt_alias_type_overrides: HashMap::new(),
191:        occt_alias_type_overrides,
257:        &occt_alias_type_overrides,
345:    // (class name == module name), only static methods, and no instance methods/constructors.
347:    // Convert their static methods to free functions so they appear as module-level
499:    // additional Handle types by scanning method signatures (including static methods)
507:        println!("  {} static methods", symbol_table.static_methods.len());
515:        let included_static = symbol_table.static_methods.values().filter(|m| m.status.is_included()).count();
522:        println!("    {} static methods (of {})", included_static, symbol_table.static_methods.len());
589:/// method params, return types, constructor params, static method params/returns,
722:/// Detect "utility namespace classes" and convert their static methods to free functions.
724:/// OCCT has a pattern where some packages use a class with only static methods instead of
725:/// a C++ namespace (e.g., `class gp { static const gp_Pnt& Origin(); ... }`). These are
730:/// - Has ONLY static methods (no instance methods)
733:/// Conversion: static methods → ParsedFunction entries in the same header,
751:            // Must have static methods
752:            if class.static_methods.is_empty() {
767:            // This is a utility class — convert static methods to functions
770:                    "  Detected utility class '{}' with {} static methods → converting to module-level functions",
772:                    class.static_methods.len()
776:            for sm in &class.static_methods {
777:                // Check exclude_methods for this static method
887:            let statics = table.included_static_methods(class);
888:            let all_statics: Vec<_> = class.static_methods.iter()
889:                .filter_map(|id| table.static_methods.get(id))
891:            if !all_statics.is_empty() {
892:                println!("    Static methods: {}/{} included", statics.len(), all_statics.len());
```
</details>

**Terminal:**
```sh
sed -n '140,200p' crates/opencascade-binding-generator/src/main.rs
```
<details>
<summary>Output (61 lines)</summary>

```
        }

        println!("Loaded config: {} module patterns, {} exclude module patterns, {} individual headers, {} header exclusions -> {} headers",
            cfg.modules.len(), cfg.exclude_modules.len(), cfg.include_headers.len(), cfg.exclude_headers.len(), headers.len());

        let method_exclusions = parse_class_method_pairs(&cfg.exclude_methods, "exclude_methods");
        let ambiguous_method_exclusions =
            parse_class_method_pairs(&cfg.ambiguous_methods, "ambiguous_methods");

        let excludes: std::collections::HashSet<String> = cfg.exclude_headers.into_iter().collect();
        let exclude_mods: Vec<String> = cfg.exclude_modules;
        let non_alloc_cls: HashSet<String> = cfg.non_allocatable_classes.into_iter().collect();
        let manual_names: HashSet<String> = cfg.manual_types.keys().cloned().collect();
        let tmpl_inst = cfg.template_instantiations;
        let occt_alias_overrides = cfg.occt_alias_type_overrides;
        SelectionInputs {
            explicit_headers: headers,
            exclude_set: excludes,
            exclude_modules: exclude_mods,
            exclude_methods: method_exclusions,
            ambiguous_methods: ambiguous_method_exclusions,
            non_allocatable_classes: non_alloc_cls,
            manual_type_names: manual_names,
            template_instantiations: tmpl_inst,
            occt_alias_type_overrides: occt_alias_overrides,
        }
    } else if !args.headers.is_empty() {
        SelectionInputs {
            explicit_headers: args.headers.clone(),
            exclude_set: std::collections::HashSet::new(),
            exclude_modules: Vec::new(),
            exclude_methods: HashSet::new(),
            ambiguous_methods: HashSet::new(),
            non_allocatable_classes: HashSet::new(),
            manual_type_names: HashSet::new(),
            template_instantiations: HashMap::new(),
            occt_alias_type_overrides: HashMap::new(),
        }
    } else {
        anyhow::bail!("Either --config <file.toml> or positional header arguments are required");
    };

    let SelectionInputs {
        explicit_headers,
        exclude_set,
        exclude_modules,
        exclude_methods,
        ambiguous_methods,
        non_allocatable_classes,
        manual_type_names,
        template_instantiations,
        occt_alias_type_overrides,
    } = inputs;

    // Resolve header dependencies when include directories are available.
    let headers_to_process = if !args.include_dirs.is_empty() {
        // Use first include dir as OCCT include root
        let occt_include_dir = &args.include_dirs[0];

        if args.verbose {
            println!("\nResolving header dependencies...");
```
</details>

**Terminal:**
```sh
sed -n '90,115p' crates/opencascade-binding-generator/src/main.rs
```
<details>
<summary>Output (26 lines)</summary>

```
                None
            }
        })
        .collect()
}

struct SelectionInputs {
    explicit_headers: Vec<PathBuf>,
    exclude_set: std::collections::HashSet<String>,
    exclude_modules: Vec<String>,
    exclude_methods: HashSet<(String, String)>,
    ambiguous_methods: HashSet<(String, String)>,
    non_allocatable_classes: HashSet<String>,
    manual_type_names: HashSet<String>,
    template_instantiations: HashMap<String, config::TemplateInstantiation>,
    occt_alias_type_overrides: HashMap<String, String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("OCCT Binding Generator");
        println!("======================");
        println!("Include directories: {:?}", args.include_dirs);
        println!("Output directory: {:?}", args.output);
```
</details>

**Terminal:**
```sh
sed -n '240,270p' crates/opencascade-binding-generator/src/main.rs
```
<details>
<summary>Output (31 lines)</summary>

```
                    if exclude_modules.iter().any(|pattern| config::module_matches_pattern(module, pattern)) {
                        return false;
                    }
                }
                true
            })
            .collect();
        if filtered.len() < before {
            println!("  Excluded {} headers after dependency resolution", before - filtered.len());
        }
        filtered
    };

    println!("Parsing {} headers...", headers_to_process.len());
    let mut parsed = parser::parse_headers(
        &headers_to_process,
        &args.include_dirs,
        &occt_alias_type_overrides,
        args.verbose,
    )?;

    // Rewrite template Handle types to alias names based on template_instantiations config.
    // This transforms e.g. Type::Handle("BVH_Builder<double, 3>") into
    // Type::Handle("BVH_Builder_double_3") so downstream code sees clean alias names.
    let template_alias_map: HashMap<String, String> = template_instantiations
        .iter()
        .map(|(spelling, _)| (spelling.clone(), config::template_alias_name(spelling)))
        .collect();
    if !template_alias_map.is_empty() {
        let rewritten = rewrite_template_types(&mut parsed, &template_alias_map);
        if rewritten > 0 {
```
</details>

> 💭 Now I have a clear picture. Let me look at how `model.rs`'s `is_void_type_name` is currently used - I need to understand whether I can make it dynamic (take a set as parameter) or keep it hardcoded.
> 
> Actually, looking at the existing precedent: `parser.rs` has a thread-local global `OCCT_ALIAS_OVERRIDES` for `occt_alias_type_overrides`. The same pattern could work for `void_pointer_types`.
> 
> But wait, `is_void_type_name` in `model.rs` is called from many places (type_mapping.rs, codegen/rust.rs, codegen/cpp.rs, codegen/bindings.rs). 
> 
> I'm thinking the cleanest solution is to follow the existing pattern and add a global `OnceLock` for extra void types in `model.rs`, then initialize it before binding generation starts. This avoids threading parameters through all the call sites while keeping the logic centralized.

**Terminal:**
```sh
sed -n '55,90p' crates/opencascade-binding-generator/src/parser.rs
```
<details>
<summary>Output (36 lines)</summary>

```
    /// These are tracked separately so callers can selectively add them to
    /// known-type sets without importing every collected template typedef name.
    static NAMESPACE_TYPEDEF_NAMES: RefCell<HashSet<String>> = RefCell::new(HashSet::new());

    /// Map of canonical OCCT type spellings to public alias names loaded from config.
    /// Keys are whitespace-normalized to match `normalize_template_spelling()` lookups.
    static OCCT_ALIAS_TYPE_OVERRIDES: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

/// Strip whitespace from a C++ type spelling for typedef map key/lookup.
fn normalize_template_spelling(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Normalize known OCCT internal template spellings to their public typedef aliases.
///
/// Some persistence APIs expose handles to aliases like `ShapePersistent_Geom::Curve`,
/// but clang may report canonical inner names like `ShapePersistent_Geom::geometryBase<Geom_Curve>`.
/// Rewriting these to alias names keeps them bindable against the known-type set.
fn normalize_occt_alias_type_name(type_name: &str) -> String {
    let key = normalize_template_spelling(type_name);
    OCCT_ALIAS_TYPE_OVERRIDES.with(|m| {
        m.borrow()
            .get(&key)
            .cloned()
            .unwrap_or_else(|| type_name.trim().to_string())
    })
}

fn class_type(type_name: &str) -> Type {
    Type::Class(normalize_occt_alias_type_name(type_name))
}

/// Strip C++ type qualifier prefixes (const, volatile, struct, class, typename, enum)
/// from a type spelling. Call sites used to chain these manually; this centralizes the
/// stripping logic and avoids accidental divergence.
```
</details>

> 💭 Now I see the pattern. The thread_local approach is used in parser.rs. For `model.rs`, I should use a similar approach.
> 
> Let me think about the design:
> 
> 1. Add `void_pointer_types: Vec<String>` to `BindingConfig` in `config.rs`
> 2. Add a `set_void_pointer_type_names(names: &HashSet<String>)` function to `model.rs` (using thread_local state)  
> 3. Modify `is_void_type_name()` to also check the thread-local set
> 4. Add `void_pointer_types` to `SelectionInputs` in `main.rs`
> 
> 
> 5. Call `set_void_pointer_type_names()` before `parse_headers()`
> 6. Remove the hardcoded `Aspect_RenderingContext` from multiple files
> 
> Looking at the actual hardcoded references, I need to replace them with dynamic checks using `is_void_type_name()`. In `type_mapping.rs` and `codegen/bindings.rs`, I can swap the literal comparisons for function calls. In `codegen/cpp.rs`, the pattern match needs to become a function call instead. The `parser.rs` and `model.rs` functions need to become dynamic, and I should add the configuration to `bindings.toml`.

**Terminal:**
```sh
sed -n '88,105p' crates/opencascade-binding-generator/src/codegen/cpp.rs
```
<details>
<summary>Output (18 lines)</summary>

```
        }

        match ty {
            Type::Class(name) => {
                // Skip primitive types that don't have headers
                // Also skip Standard_Address which is defined in Standard_TypeDef.hxx, not its own file
                if matches!(name.as_str(), 
                    "bool" | "char" | "int" | "unsigned" | "float" | "double" | 
                    "void" | "size_t" | "Standard_Address" | "Aspect_RenderingContext"
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

**Terminal:**
```sh
sed -n '376,395p' crates/opencascade-binding-generator/src/parser.rs
```
<details>
<summary>Output (20 lines)</summary>

```
    SIMPLE_TYPEDEF_MAP.with(|m| m.borrow().get(name).cloned())
}

fn set_occt_alias_type_overrides(overrides: &HashMap<String, String>) {
    let normalized: HashMap<String, String> = overrides
        .iter()
        .map(|(from, to)| (normalize_template_spelling(from), to.trim().to_string()))
        .collect();
    OCCT_ALIAS_TYPE_OVERRIDES.with(|m| {
        *m.borrow_mut() = normalized;
    });
}



/// Parse a collection of OCCT header files
/// 
/// Uses batch parsing: creates a synthetic source file that includes all headers,
/// parses once, then extracts entities from each target header. This is much faster
/// than parsing each header separately since OCCT headers have deep include chains.
```
</details>

> 💭 Now I have everything I need to implement this properly. Let me think about where exactly to put the `VOID_POINTER_TYPES` thread-local global. Options:
> 
> 1. In `model.rs` — this is where `is_void_type_name()` lives, so it makes sense to put it there
> 2. In `parser.rs` — we'd need to export it from there too, and model.rs doesn't currently import from parser.rs
> 
> I'll put it in `model.rs` since that's where the function is. Then:
> - `model.rs` exports `set_void_pointer_type_names(names: HashSet<String>)` 
> 
> 
> - `main.rs` calls it just before `parse_headers()`
> 
> I need to check how the thread_local pattern is set up in parser.rs so I can replicate it in model.rs. For the `type_mapping.rs` changes, I'm replacing the hardcoded class name checks with a call to `is_void_type_name()`, but I need to verify that function can handle the dynamic type names we're now storing.
> 
> Looking at the current mapping, `void` has its own separate arm that maps to `c_void`, while `Standard_Address` and `Aspect_RenderingContext` both map to `*mut c_void`. Since `is_void_type_name` will include all three, I can't just swap in that function directly — I need to exclude `void` from the `*mut c_void` arm. The cleanest approach is to create a separate predicate like `is_void_ptr_type_name()` that covers `Standard_Address` plus the dynamic set, keeping `void` handled by its own arm. This way the type mapping logic stays clear about which types map to which Rust types.

**Todo list:**
- ✅ Commit current state
- ✅ Read bindings.toml and generator config structures
- 🔄 Design custom_type_mappings in bindings.toml
- ⬜ Implement custom_type_mappings in generator
- ⬜ Regenerate, build, test, commit
- ⬜ Identify and apply refactoring opportunities

> 💭 The design I'll implement:
> 
> - **bindings.toml**: `void_pointer_types = ["Aspect_RenderingContext"]` - a list of C++ type names that should map to `*mut std::ffi::c_void` at the FFI boundary, making methods unsafe. These are platform-dependent pointer typedefs whose canonical form varies by platform.
> 
> - **config.rs**: `void_pointer_types: Vec<String>` field
> 
> - **model.rs**: Thread-local `VOID_POINTER_TYPES` global + `set_void_pointer_type_names()` function; `is_void_type_name()` checks this set too

The design: add `void_pointer_types = ["Aspect_RenderingContext"]` to `crates/opencascade-sys/bindings.toml`. These are platform-dependent C++ pointer typedefs that map to `*mut std::ffi::c_void`. The generator will read them and pass them through a thread-local in `model.rs` so `is_void_type_name()` picks them up everywhere.

Let me read all the specific lines I need to edit simultaneously:

<details>
<summary>Reading lines 1-100 of <a href="../crates/opencascade-binding-generator/src/model.rs#L1-L101">model.rs</a></summary>

```
//! Internal representation (IR) for parsed C++ declarations
//!
//! These types represent the parsed information from OCCT headers
//! in a form suitable for code generation.

#![allow(dead_code)] // Some fields/methods are reserved for future use

use std::collections::HashSet;
use std::path::PathBuf;

/// A parsed header file containing class declarations
#[derive(Debug, Clone)]
pub struct ParsedHeader {
    /// Path to the header file
    pub path: PathBuf,
    /// Classes defined in this header
    pub classes: Vec<ParsedClass>,
    /// Enums defined in this header
    pub enums: Vec<ParsedEnum>,
    /// Free functions (namespace-level) defined in this header
    pub functions: Vec<ParsedFunction>,
}

/// A parsed free function (namespace-level function like TopoDS::Edge)
#[derive(Debug, Clone)]
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

    /// Check if this function has any unsafe raw pointer types that require
    /// the function to be marked `unsafe fn`.
    pub fn has_unsafe_types(&self) -> bool {
        if self.params.iter().any(|p| p.ty.needs_unsafe_fn() && !p.is_nullable_ptr() && p.ty.class_ptr_inner_name().is_none()) {
            return true;
        }
        if let Some(ref ret) = self.return_type {
            if ret.needs_unsafe_fn() {
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
    pub name: String,
    /// Module name extracted from prefix
    pub module: String,
    /// Documentation comment from the header
    pub comment: Option<String>,
    /// Source header file name (e.g., "TopAbs_ShapeEnum.hxx")
    pub source_header: String,
    /// Enum variants
    pub variants: Vec<EnumVariant>,
}

/// A single enum variant
#[derive(Debug, Clone)]
pub struct EnumVariant {
    /// Variant name (e.g., "TopAbs_COMPOUND")
    pub name: String,
    /// Explicit value if specified
    pub value: Option<i64>,
    /// Documentation comment
    pub comment: Option<String>,
}
```
</details>

<details>
<summary>Reading lines 415-432 of <a href="../crates/opencascade-binding-generator/src/model.rs#L415-L433">model.rs</a></summary>

```
}

/// Check if a class name represents a void pointer type.
/// Standard_Address and Aspect_RenderingContext are typedefs for void* or platform pointers,
/// and "void" is the parsed form of literal void* parameters.
pub fn is_void_type_name(name: &str) -> bool {
    name == "Standard_Address" || name == "void" || name == "Aspect_RenderingContext"
}

/// Check if a class name represents a C++ standard library bitmask type.
/// These types are integer-compatible bitmasks that need explicit `static_cast`
/// in C++ wrapper code because on some platforms (e.g., Linux/libstdc++),
/// they are proper enum types that don't implicitly convert from integers.
/// Returns the corresponding FFI integer type if it is a bitmask type.
pub fn std_bitmask_ffi_type(name: &str) -> Option<Type> {
    match name {
        "std::ios_base::openmode" => Some(Type::U32),
        _ => None,
```
</details>

**Terminal:**
```sh
grep -n "use std::collections\|thread_local\|RefCell\|HashMap\|HashSet" crates/opencascade-binding-generator/src/model.rs | head -15
```
```
8:use std::collections::HashSet;
140:    pub all_method_names: std::collections::HashSet<String>,
148:    pub pure_virtual_methods: HashSet<String>,
```

<details>
<summary>Reading lines 1-10 of <a href="../crates/opencascade-binding-generator/src/model.rs#L1-L11">model.rs</a></summary>

```
//! Internal representation (IR) for parsed C++ declarations
//!
//! These types represent the parsed information from OCCT headers
//! in a form suitable for code generation.

#![allow(dead_code)] // Some fields/methods are reserved for future use

use std::collections::HashSet;
use std::path::PathBuf;

```
</details>

Now let me read the relevant section of main.rs and parser.rs for the changes:

**Terminal:**
```sh
sed -n '95,115p' crates/opencascade-binding-generator/src/main.rs
```
<details>
<summary>Output (21 lines)</summary>

```

struct SelectionInputs {
    explicit_headers: Vec<PathBuf>,
    exclude_set: std::collections::HashSet<String>,
    exclude_modules: Vec<String>,
    exclude_methods: HashSet<(String, String)>,
    ambiguous_methods: HashSet<(String, String)>,
    non_allocatable_classes: HashSet<String>,
    manual_type_names: HashSet<String>,
    template_instantiations: HashMap<String, config::TemplateInstantiation>,
    occt_alias_type_overrides: HashMap<String, String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("OCCT Binding Generator");
        println!("======================");
        println!("Include directories: {:?}", args.include_dirs);
        println!("Output directory: {:?}", args.output);
```
</details>

**Terminal:**
```sh
grep -n "occt_alias_type_overrides\|parse_headers\|SelectionInputs" crates/opencascade-binding-generator/src/main.rs
```
<details>
<summary>Output (11 lines)</summary>

```
96:struct SelectionInputs {
105:    occt_alias_type_overrides: HashMap<String, String>,
154:        let occt_alias_overrides = cfg.occt_alias_type_overrides;
155:        SelectionInputs {
164:            occt_alias_type_overrides: occt_alias_overrides,
167:        SelectionInputs {
176:            occt_alias_type_overrides: HashMap::new(),
182:    let SelectionInputs {
191:        occt_alias_type_overrides,
254:    let mut parsed = parser::parse_headers(
257:        &occt_alias_type_overrides,
```
</details>

**Terminal:**
```sh
sed -n '148,200p' crates/opencascade-binding-generator/src/main.rs
```
<details>
<summary>Output (53 lines)</summary>

```

        let excludes: std::collections::HashSet<String> = cfg.exclude_headers.into_iter().collect();
        let exclude_mods: Vec<String> = cfg.exclude_modules;
        let non_alloc_cls: HashSet<String> = cfg.non_allocatable_classes.into_iter().collect();
        let manual_names: HashSet<String> = cfg.manual_types.keys().cloned().collect();
        let tmpl_inst = cfg.template_instantiations;
        let occt_alias_overrides = cfg.occt_alias_type_overrides;
        SelectionInputs {
            explicit_headers: headers,
            exclude_set: excludes,
            exclude_modules: exclude_mods,
            exclude_methods: method_exclusions,
            ambiguous_methods: ambiguous_method_exclusions,
            non_allocatable_classes: non_alloc_cls,
            manual_type_names: manual_names,
            template_instantiations: tmpl_inst,
            occt_alias_type_overrides: occt_alias_overrides,
        }
    } else if !args.headers.is_empty() {
        SelectionInputs {
            explicit_headers: args.headers.clone(),
            exclude_set: std::collections::HashSet::new(),
            exclude_modules: Vec::new(),
            exclude_methods: HashSet::new(),
            ambiguous_methods: HashSet::new(),
            non_allocatable_classes: HashSet::new(),
            manual_type_names: HashSet::new(),
            template_instantiations: HashMap::new(),
            occt_alias_type_overrides: HashMap::new(),
        }
    } else {
        anyhow::bail!("Either --config <file.toml> or positional header arguments are required");
    };

    let SelectionInputs {
        explicit_headers,
        exclude_set,
        exclude_modules,
        exclude_methods,
        ambiguous_methods,
        non_allocatable_classes,
        manual_type_names,
        template_instantiations,
        occt_alias_type_overrides,
    } = inputs;

    // Resolve header dependencies when include directories are available.
    let headers_to_process = if !args.include_dirs.is_empty() {
        // Use first include dir as OCCT include root
        let occt_include_dir = &args.include_dirs[0];

        if args.verbose {
            println!("\nResolving header dependencies...");
```
</details>

**Terminal:**
```sh
sed -n '252,262p' crates/opencascade-binding-generator/src/main.rs
```
<details>
<summary>Output (11 lines)</summary>

```

    println!("Parsing {} headers...", headers_to_process.len());
    let mut parsed = parser::parse_headers(
        &headers_to_process,
        &args.include_dirs,
        &occt_alias_type_overrides,
        args.verbose,
    )?;

    // Rewrite template Handle types to alias names based on template_instantiations config.
    // This transforms e.g. Type::Handle("BVH_Builder<double, 3>") into
```
</details>

Now let me look at the parser.rs `map_standard_type` section to understand exactly what to change:

<details>
<summary>Reading lines 2070-2096 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L2070-L2097">parser.rs</a></summary>

```
        "unsigned char" | "uint8_t" | "Standard_Byte" | "Standard_Utf8UChar" => Some(Type::U8),
        "signed char" | "int8_t" => Some(Type::I8),
        "bool" => Some(Type::Bool),
        // Standard_Address is void* — bound as *mut c_void in unsafe functions.
        // Represented as Type::Class("Standard_Address") so is_void_ptr() can detect it.
        "Standard_Address" => Some(Type::Class("Standard_Address".to_string())),
        // Aspect_RenderingContext is a platform-dependent typedef:
        // void* on Linux, NSOpenGLContext* on macOS, EAGLContext* on iOS.
        // Intercept before canonical resolution to get consistent cross-platform output.
        "Aspect_RenderingContext" => Some(Type::Class("Aspect_RenderingContext".to_string())),
        // Stream types - map both OCCT typedef names and bare C++ names to the
        // same Type::Class so they're recognized as known manual_types.
        "Standard_OStream" | "std::ostream" => Some(Type::Class("Standard_OStream".to_string())),
        "Standard_IStream" | "std::istream" => Some(Type::Class("Standard_IStream".to_string())),
        "Standard_SStream" | "std::stringstream" => Some(Type::Class("Standard_SStream".to_string())),
        _ => None,
    }
}

/// Map C99/C++ fixed-width integer typedef names to our Type enum.
///
/// These typedefs (`uint64_t`, `int64_t`, etc.) resolve to different canonical
/// types on different platforms (e.g. `uint64_t` is `unsigned long` on LP64
/// Linux but `unsigned long long` on macOS). We intercept them before canonical
/// resolution so the generated Rust FFI always uses fixed-width types (`u64`,
/// `i64`) instead of platform-dependent ones (`c_ulong`).
fn map_fixed_width_integer(name: &str) -> Option<Type> {
```
</details>

Now let me look at the type_mapping.rs changes needed:

<details>
<summary>Reading lines 200-490 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L200-L491">type_mapping.rs</a></summary>

```
                needs_unique_ptr: true,
                needs_pin: false,
                source_module,
            }
        }
        Type::Class(class_name) if class_name == "Standard_Address" || class_name == "Aspect_RenderingContext" => {
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
        Type::Class(class_name) if crate::model::std_bitmask_ffi_type(class_name).is_some() => {
            // Standard library bitmask types (e.g., std::ios_base::openmode)
            // mapped to their FFI integer type
            map_type_to_rust(&crate::model::std_bitmask_ffi_type(class_name).unwrap())
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
}

/// Compute the short type name by stripping the module prefix from a C++ type name.
///
/// When the type's name-based prefix differs from its header-based module,
... (truncated, 12503 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code - Insiders/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/ea54976d-0c4a-4387-8833-79fd31749c73/toolu_bdrk_01PV6f6PT8EBvavY3LnSs5op__vscode-1771913781033/content.txt#1-1), lines 1 to 80

Read [content.txt](/Users/sethml/Library/Application Support/Code - Insiders/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/ea54976d-0c4a-4387-8833-79fd31749c73/toolu_bdrk_01PV6f6PT8EBvavY3LnSs5op__vscode-1771913781033/content.txt#100-100), lines 100 to 200

Read [content.txt](/Users/sethml/Library/Application Support/Code - Insiders/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/ea54976d-0c4a-4387-8833-79fd31749c73/toolu_bdrk_01PV6f6PT8EBvavY3LnSs5op__vscode-1771913781033/content.txt#200-200), lines 200 to 290

> 💭 Now I have all the context I need. Let me implement the full design.
> 
> Here's the plan:
> 1. **`config.rs`**: Add `void_pointer_types: Vec<String>` field with descriptive comment
> 2. **`model.rs`**: 
>    - Add `thread_local!` block with `VOID_POINTER_TYPES: RefCell<HashSet<String>>`
>    - Add `pub fn set_void_pointer_type_names(names: HashSet<String>)`
>    - Update `is_void_type_name()` to also check the dynamic set
> 3. **`parser.rs`**: Remove hardcoded `"Aspect_RenderingContext"` arm, add dynamic check at the end of `map_standard_type()` (before `_ => None`)

**Terminal:**
```sh
head -20 crates/opencascade-binding-generator/src/type_mapping.rs
echo "---"
head -15 crates/opencascade-binding-generator/src/codegen/cpp.rs
```
<details>
<summary>Output (20 lines)</summary>

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
```
</details>

> 💭 Now I have everything I need. Let me implement all the changes and then read the exact lines using hashline_read before making edits. Actually, I already have the hash values from previous reads, so let me proceed:
> 
> Changes to make:
> 1. **`config.rs`**: Add `void_pointer_types` field with comment
> 2. **`model.rs`**: 
>    - Add imports for thread_local, RefCell
>    - Add thread_local VOID_POINTER_TYPES
>    - Add set_void_pointer_type_names function
>    - Update is_void_type_name3. **`parser.rs`**: Remove hardcoded Aspect_RenderingContext arm, add dynamic check
> 
> 
> 4. **`type_mapping.rs`**: Both arms updated
> 5. **`codegen/cpp.rs`**: matches! updated
> 6. **`codegen/bindings.rs`**: condition updated
> 7. **`main.rs`**: Add void_pointer_types to SelectionInputs and three places
> 8. **`bindings.toml`**: Add void_pointer_types
> 
> For model.rs, I need to update the is_void_type_name function to check against the thread-local set instead of hardcoding the Aspect_RenderingContext check.

Now I have everything I need for a complete implementation. Let me read the specific main.rs sections:

<details>
<summary>Reading lines 94-200 of <a href="../crates/opencascade-binding-generator/src/main.rs#L94-L201">main.rs</a></summary>

```
}

struct SelectionInputs {
    explicit_headers: Vec<PathBuf>,
    exclude_set: std::collections::HashSet<String>,
    exclude_modules: Vec<String>,
    exclude_methods: HashSet<(String, String)>,
    ambiguous_methods: HashSet<(String, String)>,
    non_allocatable_classes: HashSet<String>,
    manual_type_names: HashSet<String>,
    template_instantiations: HashMap<String, config::TemplateInstantiation>,
    occt_alias_type_overrides: HashMap<String, String>,
}

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
    let inputs = if let Some(ref config_path) = args.config {
        let cfg = config::load_config(config_path)?;

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

        let method_exclusions = parse_class_method_pairs(&cfg.exclude_methods, "exclude_methods");
        let ambiguous_method_exclusions =
            parse_class_method_pairs(&cfg.ambiguous_methods, "ambiguous_methods");

        let excludes: std::collections::HashSet<String> = cfg.exclude_headers.into_iter().collect();
        let exclude_mods: Vec<String> = cfg.exclude_modules;
        let non_alloc_cls: HashSet<String> = cfg.non_allocatable_classes.into_iter().collect();
        let manual_names: HashSet<String> = cfg.manual_types.keys().cloned().collect();
        let tmpl_inst = cfg.template_instantiations;
        let occt_alias_overrides = cfg.occt_alias_type_overrides;
        SelectionInputs {
            explicit_headers: headers,
            exclude_set: excludes,
            exclude_modules: exclude_mods,
            exclude_methods: method_exclusions,
            ambiguous_methods: ambiguous_method_exclusions,
            non_allocatable_classes: non_alloc_cls,
            manual_type_names: manual_names,
            template_instantiations: tmpl_inst,
            occt_alias_type_overrides: occt_alias_overrides,
        }
    } else if !args.headers.is_empty() {
        SelectionInputs {
            explicit_headers: args.headers.clone(),
            exclude_set: std::collections::HashSet::new(),
            exclude_modules: Vec::new(),
            exclude_methods: HashSet::new(),
            ambiguous_methods: HashSet::new(),
            non_allocatable_classes: HashSet::new(),
            manual_type_names: HashSet::new(),
            template_instantiations: HashMap::new(),
            occt_alias_type_overrides: HashMap::new(),
        }
    } else {
        anyhow::bail!("Either --config <file.toml> or positional header arguments are required");
    };

    let SelectionInputs {
        explicit_headers,
        exclude_set,
 
... (truncated, 4554 chars)
```
</details>

<details>
<summary>Reading lines 250-265 of <a href="../crates/opencascade-binding-generator/src/main.rs#L250-L266">main.rs</a></summary>

```
        filtered
    };

    println!("Parsing {} headers...", headers_to_process.len());
    let mut parsed = parser::parse_headers(
        &headers_to_process,
        &args.include_dirs,
        &occt_alias_type_overrides,
        args.verbose,
    )?;

    // Rewrite template Handle types to alias names based on template_instantiations config.
    // This transforms e.g. Type::Handle("BVH_Builder<double, 3>") into
    // Type::Handle("BVH_Builder_double_3") so downstream code sees clean alias names.
    let template_alias_map: HashMap<String, String> = template_instantiations
        .iter()
```
</details>

Now let me look at the cpp.rs section I need to change:

<details>
<summary>Reading lines 88-105 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L88-L106">cpp.rs</a></summary>

```
        }

        match ty {
            Type::Class(name) => {
                // Skip primitive types that don't have headers
                // Also skip Standard_Address which is defined in Standard_TypeDef.hxx, not its own file
                if matches!(name.as_str(), 
                    "bool" | "char" | "int" | "unsigned" | "float" | "double" | 
                    "void" | "size_t" | "Standard_Address" | "Aspect_RenderingContext"
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

Now I have all the hash values. Let me do the implementation in one large batch:

**Todo list:**
- ✅ Commit current state
- ✅ Read bindings.toml and generator config structures
- ✅ Design custom_type_mappings in bindings.toml
- 🔄 Implement custom_type_mappings in generator
- ⬜ Regenerate, build, test, commit
- ⬜ Identify and apply refactoring opportunities

<details>
<summary>**Failing: <a href="../crates/opencascade-sys/bindings.toml#L124-L139">bindings.toml -1+15</a>**  
**Failing: <a href="../crates/opencascade-binding-generator/src/config.rs#L89-L102">config.rs -1+13</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "124:vb",
      "content": "# Opaque types defined in manual/ files. These are not parsed from OCCT headers\n# but referenced by auto-generated bindings. The generator adds them to the known\n# class set so methods using them aren't skipped as \"unknown type\".\n# Format: \"CppTypeName\" = { header = \"Header.hxx\" }\n[manual_types]\nStandard_OStream = { header = \"Standard_OStream.hxx\" }\nStandard_IStream = { header = \"Standard_IStream.hxx\" }\nStandard_SStream = { header = \"Standard_SStream.hxx\" }\n\n# Platform-dependent pointer typedefs that should map to *mut std::ffi::c_void\n# in the FFI layer. Clang canonicalizes these to platform-specific pointer types\n# (e.g. NSOpenGLContext* on macOS, void* on Linux), producing inconsistent output.\n# Intercepting by name before canonical resolution gives uniform cross-platform FFI.\n# Methods using these types are marked unsafe.\nvoid_pointer_types = [\"Aspect_RenderingContext\"]"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/config.rs",
      "lineHashes": "89:jm",
      "content": "    /// Canonical OCCT type spellings that should be rewritten to public aliases during parsing.\n    /// This is used when clang reports internal template spellings that differ from public typedef names.\n    /// Format: \"CanonicalTypeSpelling\" = \"PublicAliasName\"\n    #[serde(default)]\n    pub occt_alias_type_overrides: std::collections::HashMap<String, String>,\n\n    /// C++ typedef names that are platform-dependent pointer types and should map to\n    /// `*mut std::ffi::c_void` at the FFI boundary (making methods unsafe).\n    /// Use for typedefs like `Aspect_RenderingContext` whose canonical form differs\n    /// by platform (void* on Linux, NSOpenGLContext* on macOS, EAGLContext* on iOS).\n    #[serde(default)]\n    pub void_pointer_types: Vec<String>,\n}"
    }
  ]
}
```
**Output:**
```
{
  "applied": 0,
  "failed": 2,
  "errors": [
    {
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "124:vb",
      "error": "hash mismatch at line 124: file has 'wo', edit specified 'vb'"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/config.rs",
      "lineHashes": "89:jm",
      "error": "hash mismatch at line 89: file has 'bj', edit specified 'jm'"
    }
  ]
}
```
</details>

<details>
<summary>Reading lines 118-130 of <a href="../crates/opencascade-sys/bindings.toml#L118-L131">bindings.toml</a></summary>

```
Standard_SStream = { header = "Standard_SStream.hxx" }

# Template instantiation aliases: declare C++ template instantiations as opaque
# Rust types. The generator creates C++ typedefs, Rust opaque types, and Handle
# wrappers for each entry. The TOML key is the exact C++ template spelling as it
# appears in parsed method signatures.
# Format: "Template<Args>" = { header = "Header.hxx", module = "Module", handle = true }
[template_instantiations]
"BVH_Builder<double, 3>" = { header = "BVH_Builder.hxx", module = "BVH", handle = true }
"BVH_Tree<double, 3>" = { header = "BVH_Tree.hxx", module = "BVH", handle = true }
"NCollection_Shared<NCollection_List<gp_Pnt2d>>" = { header = "NCollection_Shared.hxx", module = "NCollection", handle = true }
"NCollection_Shared<TColStd_PackedMapOfInteger>" = { header = "NCollection_Shared.hxx", module = "NCollection", handle = true }
"NCollection_Shared<NCollection_DynamicArray<BRepMesh_Vertex>>" = { header = "NCollection_Shared.hxx", module = "NCollection", handle = true }
```
</details>

<details>
<summary>Reading lines 84-120 of <a href="../crates/opencascade-binding-generator/src/config.rs#L84-L121">config.rs</a></summary>

```

    /// Canonical OCCT type spellings that should be rewritten to public aliases during parsing.
    /// This is used when clang reports internal template spellings that differ from public typedef names.
    /// Format: "CanonicalTypeSpelling" = "PublicAliasName"
    #[serde(default)]
    pub occt_alias_type_overrides: std::collections::HashMap<String, String>,
}

/// A manually-defined opaque type referenced by auto-generated bindings.
#[derive(Debug, Deserialize)]
pub struct ManualType {
    /// The C++ header that defines this type (for wrappers.cpp includes).
    pub header: String,
}

/// A C++ template instantiation declared as an opaque Rust type.
#[derive(Debug, Deserialize)]
pub struct TemplateInstantiation {
    /// The C++ header that defines the template (for wrappers.cpp includes).
    pub header: String,
    /// The OCCT module this type belongs to (for re-export file placement).
    pub module: String,
    /// Whether this instantiation inherits from Standard_Transient and needs Handle support.
    #[serde(default)]
    pub handle: bool,
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
```
</details>

<details>
<summary>Reading lines 96-105 of <a href="../crates/opencascade-binding-generator/src/main.rs#L96-L106">main.rs</a></summary>

```
struct SelectionInputs {
    explicit_headers: Vec<PathBuf>,
    exclude_set: std::collections::HashSet<String>,
    exclude_modules: Vec<String>,
    exclude_methods: HashSet<(String, String)>,
    ambiguous_methods: HashSet<(String, String)>,
    non_allocatable_classes: HashSet<String>,
    manual_type_names: HashSet<String>,
    template_instantiations: HashMap<String, config::TemplateInstantiation>,
    occt_alias_type_overrides: HashMap<String, String>,
```
</details>

Now I have fresh hashes. Let me do the full batch:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/config.rs#L85-L98">config.rs -6+13</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L96-L108">main.rs -11+12</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L155-L158">-2+3</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L165-L169">-3+4</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L178-L182">-3+4</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L194-L198">-3+4</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L258-L261">-2+3</a>  
**Failing: <a href="../crates/opencascade-sys/bindings.toml#L116-L137">bindings.toml -9+21</a>**</summary>

**Input:**
```
{
  "edits": [
    {
      "content": "# Opaque types defined in manual/ files. These are not parsed from OCCT headers\n# but referenced by auto-generated bindings. The generator adds them to the known\n# class set so methods using them aren't skipped as \"unknown type\".\n# Format: \"CppTypeName\" = { header = \"Header.hxx\" }\n[manual_types]\nStandard_OStream = { header = \"Standard_OStream.hxx\" }\nStandard_IStream = { header = \"Standard_IStream.hxx\" }\nStandard_SStream = { header = \"Standard_SStream.hxx\" }\n\n# Platform-dependent pointer typedefs that should map to *mut std::ffi::c_void\n# in the FFI layer. Clang canonicalizes these to platform-specific pointer types\n# (e.g. NSOpenGLContext* on macOS, void* on Linux), producing inconsistent output.\n# Intercepting by name before canonical resolution gives uniform cross-platform FFI.\n# Methods using these types are marked unsafe.\nvoid_pointer_types = [\"Aspect_RenderingContext\"]\n\n# Template instantiation aliases: declare C++ template instantiations as opaque\n# Rust types. The generator creates C++ typedefs, Rust opaque types, and Handle\n# wrappers for each entry. The TOML key is the exact C++ template spelling as it\n# appears in parsed method signatures.\n# Format: \"Template<Args>\" = { header = \"Header.hxx\", module = \"Module\", handle = true }",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "116:wl,117:dk,118:oz,119:dd,120:fa,121:ix,122:yx,123:yc,124:wo"
    },
    {
      "content": "    /// Canonical OCCT type spellings that should be rewritten to public aliases during parsing.\n    /// This is used when clang reports internal template spellings that differ from public typedef names.\n    /// Format: \"CanonicalTypeSpelling\" = \"PublicAliasName\"\n    #[serde(default)]\n    pub occt_alias_type_overrides: std::collections::HashMap<String, String>,\n\n    /// C++ typedef names that are platform-dependent pointer types and should map to\n    /// `*mut std::ffi::c_void` at the FFI boundary (making methods unsafe).\n    /// Use for typedefs like `Aspect_RenderingContext` whose canonical form differs\n    /// by platform (void* on Linux, NSOpenGLContext* on macOS, EAGLContext* on iOS).\n    #[serde(default)]\n    pub void_pointer_types: Vec<String>,\n}",
      "filePath": "crates/opencascade-binding-generator/src/config.rs",
      "lineHashes": "85:er,86:uz,87:cj,88:lb,89:bj,90:io"
    },
    {
      "content": "struct SelectionInputs {\n    explicit_headers: Vec<PathBuf>,\n    exclude_set: std::collections::HashSet<String>,\n    exclude_modules: Vec<String>,\n    exclude_methods: HashSet<(String, String)>,\n    ambiguous_methods: HashSet<(String, String)>,\n    non_allocatable_classes: HashSet<String>,\n    manual_type_names: HashSet<String>,\n    template_instantiations: HashMap<String, config::TemplateInstantiation>,\n    occt_alias_type_overrides: HashMap<String, String>,\n    void_pointer_types: HashSet<String>,\n}",
      "filePath": "crates
... (truncated, 4687 chars)
```
**Output:**
```
{
  "applied": 7,
  "failed": 1,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/config.rs",
      "startLine": 85,
      "endLine": 97
    },
    {
      "file": "crates/opencascade-binding-generator/src/main.rs",
      "startLine": 96,
      "endLine": 260
    }
  ],
  "errors": [
    {
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "116:wl,117:dk,118:oz,119:dd,120:fa,121:ix,122:yx,123:yc,124:wo",
      "error": "hash mismatch at line 116: file has 'ko', edit specified 'wl'"
    }
  ]
}
```
</details>

<details>
<summary>Reading lines 112-130 of <a href="../crates/opencascade-sys/bindings.toml#L112-L131">bindings.toml</a></summary>

```
# but referenced by auto-generated bindings. The generator adds them to the known
# class set so methods using them aren't skipped as "unknown type".
# Format: "CppTypeName" = { header = "Header.hxx" }
[manual_types]
Standard_OStream = { header = "Standard_OStream.hxx" }
Standard_IStream = { header = "Standard_IStream.hxx" }
Standard_SStream = { header = "Standard_SStream.hxx" }

# Template instantiation aliases: declare C++ template instantiations as opaque
# Rust types. The generator creates C++ typedefs, Rust opaque types, and Handle
# wrappers for each entry. The TOML key is the exact C++ template spelling as it
# appears in parsed method signatures.
# Format: "Template<Args>" = { header = "Header.hxx", module = "Module", handle = true }
[template_instantiations]
"BVH_Builder<double, 3>" = { header = "BVH_Builder.hxx", module = "BVH", handle = true }
"BVH_Tree<double, 3>" = { header = "BVH_Tree.hxx", module = "BVH", handle = true }
"NCollection_Shared<NCollection_List<gp_Pnt2d>>" = { header = "NCollection_Shared.hxx", module = "NCollection", handle = true }
"NCollection_Shared<TColStd_PackedMapOfInteger>" = { header = "NCollection_Shared.hxx", module = "NCollection", handle = true }
"NCollection_Shared<NCollection_DynamicArray<BRepMesh_Vertex>>" = { header = "NCollection_Shared.hxx", module = "NCollection", handle = true }
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-sys/bindings.toml#L116-L132">bindings.toml -9+16</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "Standard_OStream = { header = \"Standard_OStream.hxx\" }\nStandard_IStream = { header = \"Standard_IStream.hxx\" }\nStandard_SStream = { header = \"Standard_SStream.hxx\" }\n\n# Platform-dependent pointer typedefs that should map to *mut std::ffi::c_void\n# in the FFI layer. Clang canonicalizes these to platform-specific pointer types\n# (e.g. NSOpenGLContext* on macOS, void* on Linux), producing inconsistent output.\n# Intercepting by name before canonical resolution gives uniform cross-platform FFI.\n# Methods using these types are marked unsafe.\nvoid_pointer_types = [\"Aspect_RenderingContext\"]\n\n# Template instantiation aliases: declare C++ template instantiations as opaque\n# Rust types. The generator creates C++ typedefs, Rust opaque types, and Handle\n# wrappers for each entry. The TOML key is the exact C++ template spelling as it\n# appears in parsed method signatures.\n# Format: \"Template<Args>\" = { header = \"Header.hxx\", module = \"Module\", handle = true }",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "116:ko,117:kh,118:oz,119:dd,120:fa,121:ix,122:yx,123:yc,124:wo"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":116,"endLine":131}
```
</details>

Now update model.rs to add the thread-local and update `is_void_type_name`:

<details>
<summary>Reading lines 413-428 of <a href="../crates/opencascade-binding-generator/src/model.rs#L413-L429">model.rs</a></summary>

```
    /// An OCCT class type (e.g., "gp_Pnt", "TopoDS_Shape")
    Class(String),
}

/// Check if a class name represents a void pointer type.
/// Standard_Address and Aspect_RenderingContext are typedefs for void* or platform pointers,
/// and "void" is the parsed form of literal void* parameters.
pub fn is_void_type_name(name: &str) -> bool {
    name == "Standard_Address" || name == "void" || name == "Aspect_RenderingContext"
}

/// Check if a class name represents a C++ standard library bitmask type.
/// These types are integer-compatible bitmasks that need explicit `static_cast`
/// in C++ wrapper code because on some platforms (e.g., Linux/libstdc++),
/// they are proper enum types that don't implicitly convert from integers.
/// Returns the corresponding FFI integer type if it is a bitmask type.
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L8-L11">model.rs -2+3</a>, <a href="../crates/opencascade-binding-generator/src/model.rs#L415-L439">-9+24</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "use std::cell::RefCell;\nuse std::collections::HashSet;\nuse std::path::PathBuf;",
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "8:xe,9:lw"
    },
    {
      "content": "    Class(String),\n}\n\nthread_local! {\n    /// Configured void-pointer type names loaded from bindings.toml.\n    /// These are C++ typedef names that map to *mut c_void at the FFI boundary.\n    static VOID_POINTER_TYPES: RefCell<HashSet<String>> = RefCell::new(HashSet::new());\n}\n\n/// Call once before parsing/codegen to register void-pointer type names from config.\npub fn set_void_pointer_type_names(names: HashSet<String>) {\n    VOID_POINTER_TYPES.with(|s| {\n        *s.borrow_mut() = names;\n    });\n}\n\n/// Check if a class name represents a void pointer type.\n/// Standard_Address is void*, \"void\" is the pointee of void*, and any\n/// type registered via void_pointer_types in bindings.toml (e.g. Aspect_RenderingContext)\n/// is treated the same as Standard_Address at the FFI boundary.\npub fn is_void_type_name(name: &str) -> bool {\n    name == \"Standard_Address\" || name == \"void\"\n        || VOID_POINTER_TYPES.with(|s| s.borrow().contains(name))\n}",
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "414:ha,415:io,416:dd,417:yc,418:ow,419:fd,420:be,421:ma,422:io"
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
      "startLine": 8,
      "endLine": 438
    }
  ]
}
```
</details>

Now update parser.rs to remove the hardcoded entry and add a dynamic fallthrough check:

<details>
<summary>Reading lines 2072-2100 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L2072-L2101">parser.rs</a></summary>

```
        "bool" => Some(Type::Bool),
        // Standard_Address is void* — bound as *mut c_void in unsafe functions.
        // Represented as Type::Class("Standard_Address") so is_void_ptr() can detect it.
        "Standard_Address" => Some(Type::Class("Standard_Address".to_string())),
        // Aspect_RenderingContext is a platform-dependent typedef:
        // void* on Linux, NSOpenGLContext* on macOS, EAGLContext* on iOS.
        // Intercept before canonical resolution to get consistent cross-platform output.
        "Aspect_RenderingContext" => Some(Type::Class("Aspect_RenderingContext".to_string())),
        // Stream types - map both OCCT typedef names and bare C++ names to the
        // same Type::Class so they're recognized as known manual_types.
        "Standard_OStream" | "std::ostream" => Some(Type::Class("Standard_OStream".to_string())),
        "Standard_IStream" | "std::istream" => Some(Type::Class("Standard_IStream".to_string())),
        "Standard_SStream" | "std::stringstream" => Some(Type::Class("Standard_SStream".to_string())),
        _ => None,
    }
}

/// Map C99/C++ fixed-width integer typedef names to our Type enum.
///
/// These typedefs (`uint64_t`, `int64_t`, etc.) resolve to different canonical
/// types on different platforms (e.g. `uint64_t` is `unsigned long` on LP64
/// Linux but `unsigned long long` on macOS). We intercept them before canonical
/// resolution so the generated Rust FFI always uses fixed-width types (`u64`,
/// `i64`) instead of platform-dependent ones (`c_ulong`).
fn map_fixed_width_integer(name: &str) -> Option<Type> {
    match name {
        "uint64_t" => Some(Type::U64),
        "int64_t" => Some(Type::I64),
        "uint32_t" => Some(Type::U32),
```
</details>

Edited: [parser.rs -15+15](../crates/opencascade-binding-generator/src/parser.rs#L2073-L2088)

Now update type_mapping.rs (both `map_type_to_rust` and `map_type_in_context` arms):

<details>
<summary>Reading lines 203-216 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L203-L217">type_mapping.rs</a></summary>

```
            }
        }
        Type::Class(class_name) if class_name == "Standard_Address" || class_name == "Aspect_RenderingContext" => {
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
```
</details>

<details>
<summary>Reading lines 462-487 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L462-L488">type_mapping.rs</a></summary>

```
                needs_pin: false,
                source_module: None,
            }
        }
        Type::Class(class_name) if class_name == "Standard_Address" || class_name == "Aspect_RenderingContext" => {
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
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L205-L207">type_mapping.rs -2+2</a>, <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L466-L468">-2+2</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        Type::Class(class_name) if crate::model::is_void_type_name(class_name) && class_name != \"void\" => {\n            // void-pointer types (Standard_Address, void_pointer_types from config) — *mut c_void",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "205:jg,206:sv"
    },
    {
      "content": "        Type::Class(class_name) if crate::model::is_void_type_name(class_name) && class_name != \"void\" => {\n            // void-pointer types (Standard_Address, void_pointer_types from config) — *mut c_void",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "466:jg,467:sv"
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
      "startLine": 205,
      "endLine": 467
    }
  ]
}
```
</details>

Now update `codegen/bindings.rs` and `codegen/cpp.rs`:

<details>
<summary>Reading lines 737-746 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L737-L747">bindings.rs</a></summary>

```
        Type::Class(name) => {
            if name == "char" {
                "std::ffi::c_char".to_string()
            } else if name == "Standard_Address" || name == "Aspect_RenderingContext" {
                "*mut std::ffi::c_void".to_string()
            } else if name == "void" {
                "std::ffi::c_void".to_string()
            } else if let Some(ffi_ty) = std_bitmask_ffi_type(name) {
                ffi_ty.to_rust_type_string()
            } else if let Some(ctx) = reexport_ctx {
```
</details>

<details>
<summary>Reading lines 90-100 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L90-L101">cpp.rs</a></summary>

```
        match ty {
            Type::Class(name) => {
                // Skip primitive types that don't have headers
                // Also skip Standard_Address which is defined in Standard_TypeDef.hxx, not its own file
                if matches!(name.as_str(), 
                    "bool" | "char" | "int" | "unsigned" | "float" | "double" | 
                    "void" | "size_t" | "Standard_Address" | "Aspect_RenderingContext"
                ) {
                    return;
                }
                // For nested types (Parent::Nested), include the parent class header
```
</details>

<details>
<summary>Reading lines 735-748 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L735-L749">bindings.rs</a></summary>

```
        Type::U8 => "u8".to_string(),
        Type::I8 => "i8".to_string(),
        Type::Class(name) => {
            if name == "char" {
                "std::ffi::c_char".to_string()
            } else if name == "Standard_Address" || name == "Aspect_RenderingContext" {
                "*mut std::ffi::c_void".to_string()
            } else if name == "void" {
                "std::ffi::c_void".to_string()
            } else if let Some(ffi_ty) = std_bitmask_ffi_type(name) {
                ffi_ty.to_rust_type_string()
            } else if let Some(ctx) = reexport_ctx {
                ctx.resolve_class(name)
            } else {
```
</details>

> 💭 Good, now I have the exact hash values:
> - `codegen/bindings.rs` line 740: hash `vj` 
> - `codegen/cpp.rs` lines 94-97: hashes `mz`, `cg`, `dv`, `zh`
> 
> Let me also check the imports in codegen/bindings.rs to verify `is_void_type_name` is already imported.

Good, I have the hash values. Let me also check the bindings.rs imports and look at the files that were supposed to be modified in previous session to verify their current state.

<details>
<summary>Reading lines 1-15 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1-L16">bindings.rs</a></summary>

```
//! Shared intermediate representation for binding decisions.
//!
//! `ClassBindings` computes all filtering, naming, overload suffixes,
//! and conflict resolution for a class **once**. The emit functions for
//! ffi.rs, wrappers.hxx, and per-module re-exports consume this struct
//! without re-deriving any decisions.

use crate::model::{Constructor, Method, Param, ParsedClass, ParsedField, StaticMethod, Type, is_void_type_name, is_opaque_class_name, std_bitmask_ffi_type};
use crate::module_graph;
use crate::resolver::{self, SymbolTable};
use crate::type_mapping::{self, map_return_type, map_return_type_in_context, map_type_in_context, map_type_to_rust, TypeContext};
use heck::ToSnakeCase;
use std::fmt::Write as _;
use std::collections::{HashMap, HashSet};

```
</details>

<details>
<summary>Reading lines 1-15 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L1-L16">cpp.rs</a></summary>

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

```
</details>

> 💭 `codegen/bindings.rs` already imports `is_void_type_name`. Now I need to add `crate::model::is_void_type_name` to `codegen/cpp.rs` or use it fully qualified.
> 
> Let me now verify the state of all the other files that were supposedly modified in the previous session. Let me check model.rs, type_mapping.rs, parser.rs, config.rs, main.rs, and bindings.toml.

Let me verify the state of the files modified in the previous session:

<details>
<summary>Reading lines 1-50 of <a href="../crates/opencascade-binding-generator/src/model.rs#L1-L51">model.rs</a></summary>

```
//! Internal representation (IR) for parsed C++ declarations
//!
//! These types represent the parsed information from OCCT headers
//! in a form suitable for code generation.

#![allow(dead_code)] // Some fields/methods are reserved for future use

use std::cell::RefCell;
use std::collections::HashSet;
use std::path::PathBuf;

/// A parsed header file containing class declarations
#[derive(Debug, Clone)]
pub struct ParsedHeader {
    /// Path to the header file
    pub path: PathBuf,
    /// Classes defined in this header
    pub classes: Vec<ParsedClass>,
    /// Enums defined in this header
    pub enums: Vec<ParsedEnum>,
    /// Free functions (namespace-level) defined in this header
    pub functions: Vec<ParsedFunction>,
}

/// A parsed free function (namespace-level function like TopoDS::Edge)
#[derive(Debug, Clone)]
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
```
</details>

<details>
<summary>Reading lines 1-35 of <a href="../crates/opencascade-binding-generator/src/config.rs#L1-L36">config.rs</a></summary>

```
//! Configuration file parsing for the binding generator.
//!
//! Reads a TOML configuration file that specifies which OCCT headers to process.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Convert a C++ template spelling to a safe Rust/C++ identifier.
/// e.g. `BVH_Builder<double, 3>` → `BVH_Builder_double_3`
/// e.g. `NCollection_Shared<NCollection_List<gp_Pnt2d>>` → `NCollection_Shared_NCollection_List_gp_Pnt2d`
/// e.g. `NCollection_Sequence<const gp_Pnt2d *>` → `NCollection_Sequence_constgp_Pnt2d_ptr`
pub fn template_alias_name(template_spelling: &str) -> String {
    template_spelling
        .replace("::", "_")
        .replace('<', "_")
        .replace('>', "")
        .replace(", ", "_")
        .replace(',', "_")
        .replace('*', "_ptr")
        .replace('&', "_ref")
        .replace(' ', "")
        .trim_end_matches('_')
        .to_string()
}

/// Top-level configuration structure.
#[derive(Debug, Deserialize)]
pub struct BindingConfig {
    /// Include headers from these OCCT modules.
    /// Supports glob patterns: "*" matches all modules, "Geom*" matches
    /// Geom, GeomAdaptor, GeomAPI, etc.
    #[serde(default)]
    pub modules: Vec<String>,

```
</details>

<details>
<summary>Searched for text <code>void_pointer_types</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 14 results</summary>

- [parser.rs:2081](../crates/opencascade-binding-generator/src/parser.rs#L2081)
- [main.rs:106](../crates/opencascade-binding-generator/src/main.rs#L106)
- [main.rs:156](../crates/opencascade-binding-generator/src/main.rs#L156)
- [main.rs:167](../crates/opencascade-binding-generator/src/main.rs#L167)
- [main.rs:180](../crates/opencascade-binding-generator/src/main.rs#L180)
- [main.rs:196](../crates/opencascade-binding-generator/src/main.rs#L196)
- [main.rs:259](../crates/opencascade-binding-generator/src/main.rs#L259)
- [type_mapping.rs:206](../crates/opencascade-binding-generator/src/type_mapping.rs#L206)
- [type_mapping.rs:467](../crates/opencascade-binding-generator/src/type_mapping.rs#L467)
- [config.rs:96](../crates/opencascade-binding-generator/src/config.rs#L96)
- [model.rs:421](../crates/opencascade-binding-generator/src/model.rs#L421)
- [model.rs:426](../crates/opencascade-binding-generator/src/model.rs#L426)
- [model.rs:433](../crates/opencascade-binding-generator/src/model.rs#L433)
- [model.rs:437](../crates/opencascade-binding-generator/src/model.rs#L437)
</details>

Searched for text `void_pointer_types` (`**/crates/opencascade-sys/bindings.toml`), 1 result
- [bindings.toml:125](../crates/opencascade-sys/bindings.toml#L125)

> 💭 All the previously applied changes are still in place. Now I need to:
> 1. Update `codegen/bindings.rs` line 740 to use `is_void_type_name(name) && name != "void"` instead of the hardcoded check
> 2. Update `codegen/cpp.rs` lines 94-97 to use `crate::model::is_void_type_name(name.as_str())` instead of the matches! with hardcoded names
> 
> Let me now apply these two changes using hashline_edit.

Good, all previous changes are in place. Now I need to complete the two remaining file edits. Let me apply Good, all previous changes are in place. Now I need to complete the two remaining file edits. Let me apply them:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L740-L741">bindings.rs -1+1</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L94-L97">cpp.rs -4+3</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "740:vj",
      "content": "            } else if is_void_type_name(name) && name != \"void\" {"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "94:mz,95:cg,96:dv,97:zh",
      "content": "                if matches!(name.as_str(), \n                    \"bool\" | \"char\" | \"int\" | \"unsigned\" | \"float\" | \"double\" | \"size_t\"\n                ) || crate::model::is_void_type_name(name.as_str()) {"
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
      "startLine": 740,
      "endLine": 740
    },
    {
      "file": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "startLine": 94,
      "endLine": 96
    }
  ]
}
```
</details>

> 💭 Both edits applied. Now let me also check if codegen/rust.rs has any hardcoded reference to Aspect_RenderingContext.

Both edits applied. Now let me check for any remaining hardcoded references:

Searched for text `Aspect_RenderingContext` (`**/crates/opencascade-binding-generator/src/**`), 2 results
- [model.rs:433](../crates/opencascade-binding-generator/src/model.rs#L433)
- [config.rs:93](../crates/opencascade-binding-generator/src/config.rs#L93)

Only doc comments remain — that's correct. Now let's build:

**Todo list:**
- ✅ Edit codegen/bindings.rs and cpp.rs
- 🔄 cargo check binding generator
- ⬜ Regenerate bindings
- ⬜ cargo build + tests
- ⬜ Commit void_pointer_types in bindings.toml
- ⬜ Refactoring opportunities pass

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo check -p opencascade-binding-generator 2>&1
```
```
    Blocking waiting for file lock on build directory
    Checking opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 12.81s
```

**Todo list:**
- ✅ Edit codegen/bindings.rs and cpp.rs
- ✅ cargo check binding generator
- 🔄 Regenerate bindings
- ⬜ cargo build + tests
- ⬜ Commit void_pointer_types in bindings.toml
- ⬜ Refactoring opportunities pass

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
tail -5 "/Users/sethml/Library/Application Support/Code - Insiders/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/ea54976d-0c4a-4387-8833-79fd31749c73/toolu_bdrk_017FBWbHSDtckLrGsPusAFRF__vscode-1771913781070/content.txt"
```
<details>
<summary>Output (5 lines)</summary>

```
Code generation complete!
  321 modules generated

Bindings generated in /Users/sethml/src/opencascade-rs/crates/opencascade-sys/ge
nerated                                                                         real  51.6s
```
</details>

> 💭 Regeneration succeeded. Now let's run cargo build and tests.

Regeneration complete. Now let's build and test:

**Todo list:**
- ✅ Edit codegen/bindings.rs and cpp.rs
- ✅ cargo check binding generator
- ✅ Regenerate bindings
- 🔄 cargo build + tests
- ⬜ Commit void_pointer_types in bindings.toml
- ⬜ Refactoring opportunities pass

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

Build succeeded (only existing deprecation warnings). Now run the tests:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (38 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.75s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-3404803fe4cd90de)

running 17 tests
test header_deps::tests::test_is_likely_occt_header ... ok
test module_graph::tests::test_extract_rust_type_name ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test parser::tests::test_extract_module_from_header ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test parser::tests::test_is_occt_namespace_chain ... ok
test parser::tests::test_extract_template_arg ... ok
test parser::tests::test_map_standard_type ... ok
test resolver::tests::test_safe_method_name ... ok
test resolver::tests::test_safe_param_name ... ok
test type_mapping::tests::test_map_class ... ok
test type_mapping::tests::test_map_const_ref ... ok
test type_mapping::tests::test_map_primitives ... ok
test type_mapping::tests::test_map_return_type ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/occt_bindgen-eda4fc796a0d21f2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests opencascade_binding_generator

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

real  6.7s
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo test --workspace --exclude opencascade-binding-generator 2>&1
```

**Terminal:**
```sh
grep -E "^(test result|FAILED|error)" "/Users/sethml/Library/Application Support/Code - Insiders/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/ea54976d-0c4a-4387-8833-79fd31749c73/toolu_bdrk_01BfpHzw2gBuGj6C67n4wk7K__vscode-1771913781075/content.txt"
```
<details>
<summary>Output (18 lines)</summary>

```
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 19 passed; 0 failed; 4 ignored; 0 measured; 0 filtered out; fin
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

All tests pass. Now commit:

**Todo list:**
- ✅ Edit codegen/bindings.rs and cpp.rs
- ✅ cargo check binding generator
- ✅ Regenerate bindings
- ✅ cargo build + tests
- 🔄 Commit void_pointer_types in bindings.toml
- ⬜ Refactoring opportunities pass

Created [commit2.txt](../tmp/commit2.txt)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git commit -F tmp/commit2.txt -a 2>&1
```
```
[main ec36842] Move void_pointer_types from generator source to bindings.toml
 12 files changed, 77 insertions(+), 80 deletions(-)
```

**Todo list:**
- ✅ Edit codegen/bindings.rs and cpp.rs
- ✅ cargo check binding generator
- ✅ Regenerate bindings
- ✅ cargo build + tests
- ✅ Commit void_pointer_types in bindings.toml
- 🔄 Refactoring opportunities pass

Now let's think about refactoring. The key opportunities mentioned in the session summary:

1. **`is_void_type_name(name) && name != "void"` pattern** — used in both `crates/opencascade-binding-generator/src/type_mapping.rs` and `codegen/bindings.rs`. Could be extracted as `is_void_ptr_type_name()`.
2. **`codegen/cpp.rs`** — the new condition mixes a `matches!` block with an `||` `is_void_type_name()` call; `is_void_type_name` already returns true for `"void"` and `"Standard_Address"`, so those can be removed from the `matches!`.

Let me look at the relevant code:

<details>
<summary>Reading lines 415-445 of <a href="../crates/opencascade-binding-generator/src/model.rs#L415-L446">model.rs</a></summary>

```
    Class(String),
}

thread_local! {
    /// Configured void-pointer type names loaded from bindings.toml.
    /// These are C++ typedef names that map to *mut c_void at the FFI boundary.
    static VOID_POINTER_TYPES: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

/// Call once before parsing/codegen to register void-pointer type names from config.
pub fn set_void_pointer_type_names(names: HashSet<String>) {
    VOID_POINTER_TYPES.with(|s| {
        *s.borrow_mut() = names;
    });
}

/// Check if a class name represents a void pointer type.
/// Standard_Address is void*, "void" is the pointee of void*, and any
/// type registered via void_pointer_types in bindings.toml (e.g. Aspect_RenderingContext)
/// is treated the same as Standard_Address at the FFI boundary.
pub fn is_void_type_name(name: &str) -> bool {
    name == "Standard_Address" || name == "void"
        || VOID_POINTER_TYPES.with(|s| s.borrow().contains(name))
}

/// Check if a class name represents a C++ standard library bitmask type.
/// These types are integer-compatible bitmasks that need explicit `static_cast`
/// in C++ wrapper code because on some platforms (e.g., Linux/libstdc++),
/// they are proper enum types that don't implicitly convert from integers.
/// Returns the corresponding FFI integer type if it is a bitmask type.
pub fn std_bitmask_ffi_type(name: &str) -> Option<Type> {
```
</details>

<details>
<summary>Reading lines 88-105 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L88-L106">cpp.rs</a></summary>

```
        }

        match ty {
            Type::Class(name) => {
                // Skip primitive types that don't have headers
                // Also skip Standard_Address which is defined in Standard_TypeDef.hxx, not its own file
                if matches!(name.as_str(), 
                    "bool" | "char" | "int" | "unsigned" | "float" | "double" | "size_t"
                ) || crate::model::is_void_type_name(name.as_str()) {
                    return;
                }
                // For nested types (Parent::Nested), include the parent class header
                if let Some(parent) = name.split("::").next() {
                    if name.contains("::") {
                        // Nested type — include the parent's header
                        if parent.contains('_') || parent.starts_with("Standard") {
                            let header = format!("{}.hxx", parent);
                            if known_headers.is_empty() || known_headers.contains(&header) {
```
</details>

> 💭 Now I need to think about the refactoring opportunities. Looking at the code:
> 
> 1. **`is_void_type_name(name) && name != "void"` pattern** — used in `type_mapping.rs` and `codegen/bindings.rs`. I could add an `is_void_ptr_type_name()` function that excludes `"void"` (the C++ void type which becomes `c_void`, not `*mut c_void`). This would make the intent clearer.
> 
> 2. **`codegen/cpp.rs` comment** — The comment says "Also skip Standard_Address which is defined in Standard_TypeDef.hxx, not its own file" but the logic now also skips all `void_pointer_types` (which also don't have their own headers since they're typedefs in Standard_TypeDef.hxx). The comment could be updated.

Let me look at the other potentially relevant spots:

<details>
<summary>Searched for text <code>is_void_type_name</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 14 results</summary>

- [parser.rs:2084](../crates/opencascade-binding-generator/src/parser.rs#L2084)
- [model.rs:435](../crates/opencascade-binding-generator/src/model.rs#L435)
- [model.rs:456](../crates/opencascade-binding-generator/src/model.rs#L456)
- [model.rs:607](../crates/opencascade-binding-generator/src/model.rs#L607)
- [type_mapping.rs:205](../crates/opencascade-binding-generator/src/type_mapping.rs#L205)
- [type_mapping.rs:405](../crates/opencascade-binding-generator/src/type_mapping.rs#L405)
- [type_mapping.rs:466](../crates/opencascade-binding-generator/src/type_mapping.rs#L466)
- [bindings.rs:8](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L8)
- [bindings.rs:740](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L740)
- [bindings.rs:793](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L793)
- [bindings.rs:987](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L987)
- [bindings.rs:3632](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3632)
- [cpp.rs:96](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L96)
- [rust.rs:86](../crates/opencascade-binding-generator/src/codegen/rust.rs#L86)
</details>

> 💭 Looking at the uses of `is_void_type_name`:
> 1. `parser.rs:2084` — `_ if crate::model::is_void_type_name(clean)` — catches void types including "void" itself, but "void" would already have matched earlier arms... let me check if this matters
> 2. `model.rs:456` — `!is_void_type_name(name)` — used to determine if a type is a "normal" class that needs an opaque struct
> 3. `model.rs:607` — `is_void_type_name(name)` — checking if a class type is a void type (in `collect_types_from_type` or similar)
> 
> 
> 4. `type_mapping.rs:205` and `type_mapping.rs:466` — checking for void types but excluding the literal "void" string, then mapping to `*mut c_void`
> 5. `type_mapping.rs:405` — another void type check in a different context
> 6. `codegen/bindings.rs:740` — similar pattern of checking for void types excluding "void" itself and mapping to `*mut c_void`
> 7. `codegen/bindings.rs:793` — another void type check that appears to be cut off

Let me look at the `map_standard_type` function and `type_mapping.rs:405` context:

<details>
<summary>Reading lines 2055-2090 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L2055-L2091">parser.rs</a></summary>

```
        "int" => Some(Type::I32),
        "unsigned int" => Some(Type::U32),
        "long" => Some(Type::Long),
        "unsigned long" => Some(Type::ULong),
        "long long" => Some(Type::I64),
        "unsigned long long" => Some(Type::U64),
        "short" => Some(Type::I16),
        "int16_t" => Some(Type::I16),
        "int32_t" => Some(Type::I32),
        "int64_t" => Some(Type::I64),
        "uint32_t" => Some(Type::U32),
        "uint64_t" => Some(Type::U64),
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
        // Check void_pointer_types registered from bindings.toml. These are platform-dependent
        // pointer typedefs that clang would otherwise canonicalize to platform-specific types.
        // Intercepting before canonical resolution ensures consistent cross-platform output.
        _ if crate::model::is_void_type_name(clean) => Some(Type::Class(clean.to_string())),
        _ => None,
    }
}

/// Map C99/C++ fixed-width integer typedef names to our Type enum.
///
```
</details>

<details>
<summary>Reading lines 395-415 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L395-L416">type_mapping.rs</a></summary>

```

/// Check if a type references an unknown class/handle
/// Returns true if the type uses a Handle or Class that is not in all_classes
/// Check if a class name is unknown (not in the known type set).
/// Shared logic for type_uses_unknown_class and type_uses_unknown_handle.
fn is_class_name_unknown(class_name: &str, all_classes: &std::collections::HashSet<String>) -> bool {
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
    // Standard library bitmask types (e.g., std::ios_base::openmode) are known
    if crate::model::std_bitmask_ffi_type(class_name).is_some() {
        return false;
    }
```
</details>

> 💭 Now I have a complete picture. The refactoring opportunities are:
> 
> 1. **Add `is_void_ptr_type_name()` to model.rs** — eliminates the `&& name != "void"` repetition at `type_mapping.rs:205`, `type_mapping.rs:466`, and `codegen/bindings.rs:740`. This is a clear improvement.
> 
> 2. **Comment cleanup in `codegen/cpp.rs`** — The comment on line 93 says "Also skip Standard_Address which is defined in Standard_TypeDef.hxx, not its own file" but now it also skips any void_pointer_types.

The clear refactoring: add `is_void_ptr_type_name()` to eliminate the repeated  pattern in 3 places. Let me get the line hashes:

<details>
<summary>Reading lines 200-215 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L200-L216">type_mapping.rs</a></summary>

```
                needs_unique_ptr: true,
                needs_pin: false,
                source_module,
            }
        }
        Type::Class(class_name) if crate::model::is_void_type_name(class_name) && class_name != "void" => {
            // void-pointer types (Standard_Address, void_pointer_types from config) — *mut c_void
            RustTypeMapping {
                rust_type: "*mut std::ffi::c_void".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: None,
            }
        }
        Type::Class(class_name) if class_name == "void" => {
            // C++ void type (pointee of void*) — just c_void, pointer wrapping handled by outer type
```
</details>

<details>
<summary>Reading lines 460-475 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L460-L476">type_mapping.rs</a></summary>

```
                rust_type: "std::ffi::c_char".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: None,
            }
        }
        Type::Class(class_name) if crate::model::is_void_type_name(class_name) && class_name != "void" => {
            // void-pointer types (Standard_Address, void_pointer_types from config) — *mut c_void
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
<summary>Reading lines 738-745 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L738-L746">bindings.rs</a></summary>

```
            if name == "char" {
                "std::ffi::c_char".to_string()
            } else if is_void_type_name(name) && name != "void" {
                "*mut std::ffi::c_void".to_string()
            } else if name == "void" {
                "std::ffi::c_void".to_string()
            } else if let Some(ffi_ty) = std_bitmask_ffi_type(name) {
                ffi_ty.to_rust_type_string()
```
</details>

> 💭 Now I also need to:
> 1. Add `is_void_ptr_type_name()` to `model.rs` (after `is_void_type_name`)
> 2. Export it from `codegen/bindings.rs` import line
> 3. Update the 3 call sites
> 
> For `type_mapping.rs`, it uses `crate::model::is_void_type_name` so it would become `crate::model::is_void_ptr_type_name`.
> For `codegen/bindings.rs`, it imports `is_void_type_name` from crate::model so I need to add `is_void_ptr_type_name` to the import.

<details>
<summary>Reading lines 1-10 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L1-L11">type_mapping.rs</a></summary>

```
//! Type mapping between C++ and Rust types
//!
//! Maps OCCT C++ types to their Rust equivalents for extern "C" FFI generation.

#![allow(dead_code)] // Some functions are reserved for future use

use crate::model::Type;
use crate::module_graph::module_to_rust_name;

/// Convert a C++ class name to its Rust Handle type name.
```
</details>

<details>
<summary>Reading lines 430-440 of <a href="../crates/opencascade-binding-generator/src/model.rs#L430-L441">model.rs</a></summary>

```

/// Check if a class name represents a void pointer type.
/// Standard_Address is void*, "void" is the pointee of void*, and any
/// type registered via void_pointer_types in bindings.toml (e.g. Aspect_RenderingContext)
/// is treated the same as Standard_Address at the FFI boundary.
pub fn is_void_type_name(name: &str) -> bool {
    name == "Standard_Address" || name == "void"
        || VOID_POINTER_TYPES.with(|s| s.borrow().contains(name))
}

/// Check if a class name represents a C++ standard library bitmask type.
```
</details>

> 💭 Now I have everything I need. Let me plan the edits:
> 
> 1. **model.rs**: Add `is_void_ptr_type_name()` after `is_void_type_name()` (after line 438)
> 2. **codegen/bindings.rs line 8**: Add `is_void_ptr_type_name` to imports 
> 3. **codegen/bindings.rs line 740**: Change to `is_void_ptr_type_name(name)`
> 4. **type_mapping.rs line 205**: Change to `crate::model::is_void_ptr_type_name(class_name)`

Now I'll add `is_void_ptr_type_name()` to `crates/opencascade-binding-generator/src/model.rs` and update all 3 call sites, plus fix the `crates/opencascade-binding-generator/src/codegen/cpp.rs` comment:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/model.rs#L439-L446">model.rs +7</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L8-L9">bindings.rs -1+1</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L740-L741">-1+1</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L205-L206">type_mapping.rs -1+1</a>, <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L466-L467">-1+1</a>  
Edited: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L93-L96">cpp.rs -1+3</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/model.rs",
      "lineHashes": "438:io",
      "insertAfter": true,
      "content": "\n/// Check if a class name represents a void *pointer* type (maps to `*mut c_void`).\n/// Like `is_void_type_name` but excludes the bare `\"void\"` C++ type (which maps to\n/// `c_void`, not `*mut c_void`). Use this when generating pointer-type Rust signatures.\npub fn is_void_ptr_type_name(name: &str) -> bool {\n    name == \"Standard_Address\" || VOID_POINTER_TYPES.with(|s| s.borrow().contains(name))\n}"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "8:pv",
      "content": "use crate::model::{Constructor, Method, Param, ParsedClass, ParsedField, StaticMethod, Type, is_void_type_name, is_void_ptr_type_name, is_opaque_class_name, std_bitmask_ffi_type};"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "740:gs",
      "content": "            } else if is_void_ptr_type_name(name) {"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "205:gu",
      "content": "        Type::Class(class_name) if crate::model::is_void_ptr_type_name(class_name) => {"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "466:gu",
      "content": "        Type::Class(class_name) if crate::model::is_void_ptr_type_name(class_name) => {"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "93:wo",
      "content": "                // Skip primitive types that don't have headers\n                // Also skip void-pointer types (Standard_Address, void_pointer_types from config)\n                // which are typedefs in Standard_TypeDef.hxx, not their own files."
    }
  ]
}
```
**Output:**
```
{
  "applied": 6,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/model.rs",
      "startLine": 438,
      "endLine": 445
    },
    {
      "file": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "startLine": 8,
      "endLine": 740
    },
    {
      "file": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "startLine": 205,
      "endLine": 466
    },
    {
      "file": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "startLine": 93,
      "endLine": 95
    }
  ]
}
```
</details>

All 6 edits applied. Also need to remove the now-redundant  guard on the `crates/opencascade-binding-generator/src/type_mapping.rs` comment lines:

<details>
<summary>Reading lines 203-215 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L203-L216">type_mapping.rs</a></summary>

```
            }
        }
        Type::Class(class_name) if crate::model::is_void_ptr_type_name(class_name) => {
            // void-pointer types (Standard_Address, void_pointer_types from config) — *mut c_void
            RustTypeMapping {
                rust_type: "*mut std::ffi::c_void".to_string(),
                needs_unique_ptr: false,
                needs_pin: false,
                source_module: None,
            }
        }
        Type::Class(class_name) if class_name == "void" => {
            // C++ void type (pointee of void*) — just c_void, pointer wrapping handled by outer type
```
</details>

Good — the  guard is already gone since I replaced the whole match arm. The refactoring is clean. Let me verify the build:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo check -p opencascade-binding-generator 2>&1
```
```
    Checking opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.21s
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/run-binding-generator-tests.sh 2>&1
```
<details>
<summary>Output (38 lines)</summary>

```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.25s
     Running unittests src/lib.rs (target/debug/deps/opencascade_binding_generator-3404803fe4cd90de)

running 17 tests
test header_deps::tests::test_is_likely_occt_header ... ok
test module_graph::tests::test_extract_rust_type_name ... ok
test module_graph::tests::test_module_to_rust_name ... ok
test parser::tests::test_extract_module_from_header ... ok
test parser::tests::test_extract_template_arg ... ok
test codegen::bindings::tests::test_wrapper_method_overload_suffix ... ok
test codegen::bindings::tests::test_static_method_conflict_resolution ... ok
test parser::tests::test_is_occt_namespace_chain ... ok
test codegen::bindings::tests::test_abstract_class_no_constructors ... ok
test codegen::bindings::tests::test_compute_bindings_empty_class ... ok
test parser::tests::test_map_standard_type ... ok
test resolver::tests::test_safe_method_name ... ok
test resolver::tests::test_safe_param_name ... ok
test type_mapping::tests::test_map_class ... ok
test type_mapping::tests::test_map_const_ref ... ok
test type_mapping::tests::test_map_primitives ... ok
test type_mapping::tests::test_map_return_type ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/occt_bindgen-eda4fc796a0d21f2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests opencascade_binding_generator

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

real  5.8s
```
</details>

> 💭 All 17 tests pass. Since the refactoring only changes the generator (not the generated bindings — it produces the same output), I don't need to regenerate bindings. The generator's behavior is unchanged; I'm just refactoring internal function calls. Let me commit this.

All 17 tests pass. The generated bindings are unchanged (same output) since this is a pure internal refactoring. Let me commit:

Created [commit3.txt](../tmp/commit3.txt)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git commit -F tmp/commit3.txt -a 2>&1
```
```
[main 8acb1bd] Refactor: extract is_void_ptr_type_name() from repeated pattern
 4 files changed, 14 insertions(+), 5 deletions(-)
```

**Todo list:**
- ✅ Edit codegen/bindings.rs and cpp.rs
- ✅ cargo check binding generator
- ✅ Regenerate bindings
- ✅ cargo build + tests
- ✅ Commit void_pointer_types in bindings.toml
- ✅ Refactoring opportunities pass

All done. Here's what was completed in this session:

**Commit 1** (`1c50ffc`) — already existed: hardcoded `*mut c_void` approach for `Aspect_RenderingContext`

**Commit 2** (`ec36842`) —  moved to `crates/opencascade-sys/bindings.toml`:
- New  config field
- Thread-local  in `crates/opencascade-binding-generator/src/model.rs` (following `OCCT_ALIAS_TYPE_OVERRIDES` pattern)
- All 6 generator files de-hardcoded: parser.rs, type_mapping.rs, codegen/bindings.rs, codegen/cpp.rs, config.rs, `crates/opencascade-binding-generator/src/main.rs`

**Commit 3** (`8acb1bd`) — refactoring:
- New `crates/opencascade-binding-generator/src/model.rs` eliminates the repeated  pattern from 3 call sites
- Clarified comment in codegen/cpp.rs

*2026-02-24 08:52 · 968s · Words: 6,025 in · 5,063 out · 43,741 ctx · 87 rounds*

---
