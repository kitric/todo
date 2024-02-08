mod ui;

fn main() {
    MainWindow::new().unwrap().run().unwrap();
}


slint::slint! {
    import {MainWindow} from "src/ui/main_window.slint";
}
