use gtk4::prelude::*;
use crate::utils::config::parse_config;

pub mod widgets;
pub mod utils;
pub mod components;
pub mod ipc;

const APP_ID: &str = "takto.rs.layer";
const CSS: &str = include_str!("../assets/styles.css");

#[tokio::main]
async fn main() -> () {
  /*
  let (notification_sender, _notification_reciever) = tokio::sync::broadcast::channel(1);
  let notification_sender_clone = notification_sender.clone();

  tokio::task::spawn(async move {
    let _connt = zbus::connection::Builder::session().unwrap()
      .name("org.freedesktop.Notifications").unwrap()
      .serve_at("/org/freedesktop/Notifications", ipc::dbus::notifications::NotificationServer {
        sender: notification_sender_clone
      }).unwrap()
      .build()
      .await.unwrap();

    pending::<()>().await;
  });
  */

  ipc::dbus::tray::listen();

  gtk4::init().expect("GTK could not initialize");

  let display = gdk4::Display::default().expect("No default display");
  let css_provider = gtk4::CssProvider::new();

  css_provider.load_from_string(CSS);
  gtk4::style_context_add_provider_for_display(&display, &css_provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

  let app = gtk4::Application::builder()
    .application_id(APP_ID)
    .build();
  let monitors = display.monitors();
  let config = parse_config();
  let mut visible_monitors = Vec::<gdk4::Monitor>::new();

  for index in 0..monitors.n_items() {
    let current_monitor = monitors.item(index);
    let downcasted_monitor = current_monitor.unwrap().downcast::<gdk4::Monitor>().unwrap();
    let monitor_connector = downcasted_monitor.connector();

    if config.all_monitors.is_some_and(|x| x) {
      visible_monitors.push(downcasted_monitor);

      continue;
    }

    if monitor_connector.is_none() || config.monitors.as_ref().is_some() && !config.monitors.as_ref().unwrap().contains(&monitor_connector.unwrap().to_string()) {
      continue;
    }

    visible_monitors.push(downcasted_monitor);
  }
  
  gtk4::Window::set_interactive_debugging(true);

  app.connect_activate(move |application| {
    for monitor in visible_monitors.clone() {
      /*
      let notification_reciever_clone = notification_sender.subscribe();

      widgets::notification_popup::window::init(application, monitor.clone(), notification_reciever_clone);

      let notification_reciever_clone = notification_sender.subscribe();

      widgets::notification_popup::window::init(
        application,
        monitor.clone(),
        notification_reciever_clone
      );

      let notification_reciever_clone = notification_sender.subscribe();
      */

      widgets::taskbar::window::init(
        application,
        monitor.clone()
      );
    }
  });

  app.run();
}

