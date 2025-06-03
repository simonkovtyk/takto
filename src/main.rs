use std::env;

use gtk::gdk::{Display, Texture};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::{prelude::*, style_context_add_provider_for_display, Box, Image, Widget};
use gtk::{glib, Application, ApplicationWindow};

pub mod widgets;
pub mod utils;
const APP_ID: &str = "dev.simonkov.taskbar";

fn main() -> () {
  let home_env = env::var("HOME").expect("HOME env not found");
  let css_path = format!("{}/.config/gtk-widgets/style.css", home_env);

  gtk::init().expect("GTK could not initialize");

  let css_provider = gtk::CssProvider::new();
  css_provider.load_from_path(&css_path);

  let display = Display::default().expect("No default display");
  style_context_add_provider_for_display(&display, &css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

  let mut app = Application::builder().application_id(APP_ID).build();

  gtk::Window::set_interactive_debugging(true);

  widgets::taskbar::window::connect(&mut app);

  app.run();
}

