//! SigmaOS-emacs-3d-target-theme-directory — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_emacs_3d_target_theme_directory_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-emacs-3d-target-theme-directory: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-emacs-3d-target-theme-directory: ok");
}
