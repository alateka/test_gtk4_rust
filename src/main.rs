use gtk4::glib::clone;
use gtk4::{self as gtk, Label};
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};
use std::process::Command;

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("El programa")
            .default_width(500)
            .default_height(500)
            .build();

        let grid = gtk::Grid::builder()
            .margin_start(6)
            .margin_end(6)
            .margin_top(6)
            .margin_bottom(6)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .row_spacing(6)
            .column_spacing(6)
            .build();

        let button = Button::with_label("Clicame muchacho");
        button.connect_clicked(|_| {
            let _output = Command::new("yt-dlp")
                .arg("https://www.youtube.com/watch?v=dARmGC_gKio")
                .output()
                .expect("failed to execute process");
        });
        let label: Label = Label::new(Some("Hola"));
        window.set_child(Some(&grid));

        let quit_button = gtk::Button::with_label("Quit");
        quit_button.connect_clicked(clone!(
            #[weak]
            window,
            move |_| window.destroy()
        ));

        let header_bar = gtk4::HeaderBar::new();
        header_bar.pack_end(&quit_button);
        window.set_titlebar(Some(&header_bar));


        grid.attach(&label, 0, 0, 1, 1);
        grid.attach(&button, 0, 1, 1, 1);
        window.present();
    });

    application.run()
}