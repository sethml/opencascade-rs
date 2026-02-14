My goal is to make crates/opencascade-sys a more complete FFI layer to access
the C++ OCCT library.

In order to do that, there's a work-in-progress tool
crates/opencascade-binding-generator. The PLAN.md and TRANSITION_PLAN.md
documents in that directory describe the progress. Use
scripts/regenerate-bindings.sh in order to run the generator and regenerate the
ffi files in opencascade-sys/generated. You must regenerate bindings after modifying opencascade-binding-generator. Avoid using head or tail when running
the script in order to avoid missing errors.

Avoid using head or tail when doing builds to avoid missing compile errors. Run
builds and tests with `time`. When they take more than 5 minutes, stop and ask
the user if they want to speed them up.

Do not use /tmp/ for temporary files. Use tmp/ in the project directory instead.

Never git commit unless explicitly asked to do so.

When resolving git merge conflicts, any code in crates/opencascade-sys/generated is generated code and should be regenerated rather than trying to resolve conflicts.

The opencascade crate should build. Making it function correctly is a work in progress. Some dependencies of opencascade may not build, which is OK.

When converting code to use the new API, refer to crates/opencascade-sys/PORTING.md. Update that file as you discover new porting ideas or changes.