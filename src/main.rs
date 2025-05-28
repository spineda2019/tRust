slint::include_modules!();

mod ledger;

use ledger::Ledger;
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
    }

    Ok(main_window.run()?)
}
