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

mod ledger;

use ledger::Ledger;
use native_dialog::{DialogBuilder, FileDialogBuilder};
use std::error::Error;

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

    let ledger: Option<Ledger> = Ledger::new(main_window.as_weak());

    if ledger.is_none() {
        // TODO: file popup
        let new_ledger = DialogBuilder::file()
            .add_filter("tRust Ledger", ["ledger", "toml"])
            .open_single_file()
            .show()?;
    }

    Ok(main_window.run()?)
}
