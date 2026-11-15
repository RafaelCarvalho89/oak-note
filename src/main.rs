use adw::prelude::*;

const APP_ID: &str = "io.github.rafaelcarvalho89.OakNotePoc";

fn main() {
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_window);
    app.run();
}

fn build_window(app: &adw::Application) {
    let text = gtk::TextView::builder()
        .wrap_mode(gtk::WrapMode::WordChar)
        .top_margin(16)
        .bottom_margin(16)
        .left_margin(16)
        .right_margin(16)
        .build();
    text.buffer()
        .set_text("Oak Note POC\n\nEste é um texto demonstrativo editável.");

    let scroll = gtk::ScrolledWindow::builder()
        .child(&text)
        .vexpand(true)
        .build();
    let header = adw::HeaderBar::builder()
        .title_widget(&adw::WindowTitle::new("Oak Note POC", "Prova de conceito"))
        .build();
    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content.append(&header);
    content.append(&scroll);

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Oak Note POC")
        .default_width(800)
        .default_height(560)
        .content(&content)
        .build();
    window.present();
}
