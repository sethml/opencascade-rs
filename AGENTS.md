My goal is to make crates/opencascade-sys a more complete FFI layer to access
the C++ OCCT library.

In order to do that, there's a work-in-progress tool
crates/opencascade-binding-generator. The PLAN.md and TRANSITION_PLAN.md
documents in that directory describe the progress. Use
scripts/regenerate-bindings.sh in order to run the generator and regenerate the
ffi files in opencascade-sys/generated. Avoid using head or tail when running
the script in order to avoid missing errors.

Avoid using head or tail when doing builds to avoid missing compile errors. Run
builds and tests with `time`. When they take more than 5 minutes, stop and ask
the user if they want to speed them up.

Do not use /tmp/ for temporary files. Use tmp/ in the project directory instead.

