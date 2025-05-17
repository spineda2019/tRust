slint::slint! {
    export component MainWindow inherits Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}

fn main() {
    let main_window: MainWindow = match MainWindow::new() {
        Ok(w) => w,
        Err(_) => todo!(),
    };

    if let Err(e) = main_window.run() {
        todo!();
    }
}
