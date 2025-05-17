/// Build script entry point. Will be compiled AND run before the package is
/// compiled.
fn main() {
    slint_build::compile("ui/main_window.slint").expect("Slint build failed");
}
