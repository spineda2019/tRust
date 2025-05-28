use crate::MainWindow;
use slint::Weak;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

pub struct Ledger {
    main_window_handle: Weak<MainWindow>,
    backing_file: File,
}

impl Ledger {
    pub fn new(weak_handle: Weak<MainWindow>) -> Option<Ledger> {
        let exe_path: PathBuf = match std::env::current_exe() {
            Ok(ep) => ep,
            _ => return None,
        };
        let exe_dir: &Path = match exe_path.parent() {
            Some(parent) => parent,
            None => return None,
        };
        let conf_path: PathBuf = exe_dir.join("trust.toml");

        let file: File = match File::open(conf_path) {
            Ok(f) => f,
            _ => return None,
        };

        Some(Ledger {
            main_window_handle: weak_handle,
            backing_file: file,
        })
    }
}
