// NOTE: This module is blocked because:
// - BRepOffsetAPI_MakePipeShell::set_law(), add(), set_mode_bool(), build(),
//   make_solid() methods are in FFI but not in module re-exports
// - Also depends on law_function.rs which is blocked
// See TRANSITION_PLAN.md for details.
