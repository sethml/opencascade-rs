My goal is to make crates/opencascade-sys a more complete FFI layer to access the C++ OCCT library.

In order to do that, there's a tool crates/opencascade-binding-generator. The README.md in that directory describes the architecture, CLI usage, and future work. Use scripts/regenerate-bindings.sh to regenerate the ffi files in opencascade-sys/generated after modifying opencascade-binding-generator.


Before comitting, always make sure that everything builds and tests pass with:
```
scripts/regenerate-bindings.sh
cargo build
scripts/run-binding-generator-tests.sh
cargo test --workspace --exclude opencascade-binding-generator
```

You can find opencascade OCCT C++ headers and source in crates/occt-sys/OCCT/src/, and documentation in crates/occt-sys/OCCT/dox/.

Avoid using head or tail when doing builds or regenerating bindings to avoid missing compile errors. Run builds and tests with `time`. When they take more than 5 minutes, stop and ask the user if they want to speed them up.

Do not use /tmp/ for temporary files. Use tmp/ in the project directory instead.

When writing scripts, if they're more than 5 lines write to a temporary file rather than using shell quoting to avoid quoting errors. When doing a git commit, write the commit message to a temporary file rather than using shell quoting.

Never git commit unless explicitly asked to do so.

When resolving git merge conflicts, any code in crates/opencascade-sys/generated is generated code and should be regenerated rather than trying to resolve conflicts.

When converting code to use the new API, refer to crates/opencascade-sys/PORTING.md. Update that file as you discover new porting ideas or changes.
