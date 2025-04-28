// build.rs -----------------------------------------------------------
#[cfg(feature = "extension-module")]
fn main() {
    // For macOS .dylib / Linux .so builds
    pyo3_build_config::add_extension_module_link_args();
}

#[cfg(not(feature = "extension-module"))]
fn main() {
    // For binaries & tests: let pyo3-build-config emit the normal
    //  `cargo:rustc-link-lib=python3.x` lines so the linker can
    //  find libpython and all symbols resolve.
}
