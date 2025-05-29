//!  Copyright (C) 2024  Sebastian Pineda (spineda.wpi.alum@gmail.com)
//!
//!  This program is free software; you can redistribute it and/or modify
//!  it under the terms of the GNU General Public License as published by
//!  the Free Software Foundation; either version 3 of the License, or
//!  (at your option) any later version.
//!
//!  This program is distributed in the hope that it will be useful,
//!  but WITHOUT ANY WARRANTY; without even the implied warranty of
//!  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//!  GNU General Public License for more details.
//!
//!  You should have received a copy of the GNU General Public License along
//!  with this program. If not, see <https://www.gnu.org/licenses/>
//!
//!  ledger.rs - internal loading and bookkeeping of a user's ledger

use crate::{fatal::fatal_panic, MainWindow};
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

        let exe_dir: &Path = exe_path.parent()?;
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

    // Fallback
    pub fn with_file(weak_handle: Weak<MainWindow>, conf_path: PathBuf) -> Ledger {
        let file: File = match File::open(conf_path) {
            Ok(f) => f,
            Err(e) => fatal_panic(Box::new(e)),
        };

        Ledger {
            main_window_handle: weak_handle,
            backing_file: file,
        }
    }
}
