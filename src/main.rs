slint::include_modules!();

fn main() {
    let main_window: MainWindow = match MainWindow::new() {
        Ok(w) => w,
        Err(_e) => todo!(),
    };

    let todo_popup: TodoPopup = match TodoPopup::new() {
        Ok(tp) => tp,
        Err(_e) => todo!(),
    };

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

    if let Err(_e) = main_window.run() {
        todo!();
    }
}
