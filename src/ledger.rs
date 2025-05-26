use crate::MainWindow;
use slint::Weak;
use std::{
    error::Error,
    fs::File,
    path::{Path, PathBuf},
};

pub struct Ledger {
    main_window_handle: Weak<MainWindow>,
}

impl Ledger {
    pub fn new(weak_handle: Weak<MainWindow>) -> Result<Ledger, Box<dyn Error>> {
        let exe_path: PathBuf = std::env::current_exe()?;
        let exe_dir: &Path = match exe_path.parent() {
            Some(parent) => parent,
            None => todo!(),
        };
        let conf_path: PathBuf = exe_dir.join("trust.toml");

        // let file: File = File::open(conf_path)?;

        Ok(Ledger {
            main_window_handle: weak_handle,
        })
    }
}
