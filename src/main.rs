slint::include_modules!();

fn main() {
    let main_window: MainWindow = match MainWindow::new() {
        Ok(w) => w,
        Err(_) => todo!(),
    };

    let todo_popup: TodoPopup = match TodoPopup::new() {
        Ok(tp) => tp,
        Err(_e) => todo!(),
    };

    main_window.on_show_todo_popup(move || {
        todo_popup.run();
        todo_popup.set_popup_visible(true);
    });

    if let Err(_e) = main_window.run() {
        todo!();
    }
}
