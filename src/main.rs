use gdk4::{Display, DisplayManager, Monitor, Texture};
use gtk4::{prelude::*, style_context_add_provider_for_display, Box, CssProvider, Image, Widget, Window, STYLE_PROVIDER_PRIORITY_APPLICATION};
use gtk4::Application;
use gdk4::prelude::*;
use gtk4::prelude::*;

use crate::utils::config::parse_config;
use crate::utils::gtk::get_home_path;

pub mod widgets;
pub mod utils;
const APP_ID: &str = "dev.simonkov.taskbar";

fn main() -> () {
  gtk4::init().expect("GTK could not initialize");

  let css_provider = CssProvider::new();
  let css_path = format!("{}/.config/gtk-widgets/style.css", get_home_path());
  css_provider.load_from_path(&css_path);

  let display = Display::default().expect("No default display");

  style_context_add_provider_for_display(&display, &css_provider, STYLE_PROVIDER_PRIORITY_APPLICATION);

  let app = Application::builder()
    .application_id(APP_ID)
    .build();

  let monitors = display.monitors();

  let config = parse_config();

  let mut visible_monitors = Vec::<Monitor>::new();

  for index in 0..monitors.n_items() {
    let current_monitor = monitors.item(index);
    let downcasted_monitor = current_monitor.unwrap().downcast::<Monitor>().unwrap();
    let monitor_connector = downcasted_monitor.connector();

    if config.all_monitors.is_some_and(|x| x) {
      visible_monitors.push(downcasted_monitor);

      continue;
    }

    if monitor_connector.is_none() || config.monitors.as_ref().is_some() && config.monitors.as_ref().unwrap().contains(&monitor_connector.unwrap().to_string()) {
      continue;
    }

    visible_monitors.push(downcasted_monitor);
  }

  Window::set_interactive_debugging(true);

  app.connect_activate(move |application| {
    for monitor in visible_monitors.clone() {
      widgets::taskbar::window::init(application, monitor);
    }
  });

  app.run();
}

