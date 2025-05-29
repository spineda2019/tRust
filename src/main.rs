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
//!  main.rs - callback registration and other setup

slint::include_modules!();

mod fatal;
mod ledger;

use fatal::{fatal_panic, SourceLocation, TrustError};
use ledger::Ledger;
use native_dialog::DialogBuilder;
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    /* ******************** Up Front Window Construction ******************** */
    let main_window: MainWindow = MainWindow::new()?;
    let todo_popup: TodoPopup = TodoPopup::new()?;

    /* ******************* Simple Popup Event Registration ****************** */
    let todo_popup_handle = todo_popup.as_weak();
    main_window.on_show_todo_popup(move || {
        if let Some(handle) = todo_popup_handle.upgrade() {
            let _ = handle.run();
        }
    });

    let todo_popup_handle = todo_popup.as_weak();
    todo_popup.on_hide_todo_popup(move || {
        if let Some(handle) = todo_popup_handle.upgrade() {
            let _ = handle.hide();
        }
    });

    /* **************** Real Work + Attempt Loading From Disk *************** */

    let mut ledger: Option<Ledger> = Ledger::new(main_window.as_weak());

    if ledger.is_none() {
        let missing_popup: MissingConfigPopup = MissingConfigPopup::new()?;
        let weak_handle = main_window.as_weak();
        missing_popup.on_make_new_ledger(move || {
            let new_file: Option<PathBuf> = match DialogBuilder::file()
                .add_filter("tRust Ledger", ["ledger", "toml"])
                .save_single_file()
                .show()
            {
                Ok(maybe_path) => maybe_path,
                Err(e) => fatal_panic(Box::new(e)),
            };

            let new_file: PathBuf = match new_file {
                Some(file) => file,
                None => {
                    let message: String =
                        "Tried to save a file, but could not resolve its path".to_string();
                    let source_location: SourceLocation =
                        SourceLocation::new(file!(), line!(), column!());
                    let error: TrustError = TrustError::new(message, source_location);
                    fatal_panic(Box::new(error));
                }
            };

            // ledger = Some(Ledger::with_file(weak_handle, new_file));
        });

        let weak_handle = missing_popup.as_weak();
        missing_popup.on_load_file_picker(move || {
            let new_file: Option<PathBuf> = match DialogBuilder::file()
                .add_filter("tRust Ledger", ["ledger", "toml"])
                .save_single_file()
                .show()
            {
                Ok(maybe_path) => maybe_path,
                Err(e) => fatal_panic(Box::new(e)),
            };

            let new_file: PathBuf = match new_file {
                Some(file) => file,
                None => {
                    let message: String =
                        "Tried to save a file, but could not resolve its path".to_string();
                    let source_location: SourceLocation =
                        SourceLocation::new(file!(), line!(), column!());
                    let error: TrustError = TrustError::new(message, source_location);
                    fatal_panic(Box::new(error));
                }
            };

            // ledger = Some(Ledger::with_file(weak_handle, new_file));
        });

        missing_popup.run()?; // will block until user selects option
    }

    Ok(main_window.run()?)
}
