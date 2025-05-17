slint::include_modules!();

fn main() {
    let main_window: MainWindow = match MainWindow::new() {
        Ok(w) => w,
        Err(_) => todo!(),
    };

    if let Err(_e) = main_window.run() {
        todo!();
    }
}
