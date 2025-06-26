use std::time::Duration;

use gtk4_layer_shell::LayerShell;
use gtk4::prelude::*;
use gtk4::glib;
use crate::{ipc, widgets};

pub fn init (
  app: &gtk4::Application,
  monitor: gdk4::Monitor,
  mut notification_reciever: tokio::sync::broadcast::Receiver::<ipc::dbus::notifications::Notification>
) -> () {
  let window = gtk4::ApplicationWindow::builder()
    .name("notifcation-popup__window")
    .application(app)
    .build();

  window.init_layer_shell();
  window.set_default_height(0);
  window.set_default_width(0);
  window.set_hexpand(false);
  window.set_vexpand(true);
  window.set_monitor(Some(&monitor));
  window.set_anchor(gtk4_layer_shell::Edge::Right, true);
  window.set_anchor(gtk4_layer_shell::Edge::Bottom, true);
  window.set_margin(gtk4_layer_shell::Edge::Right, 12);
  window.set_margin(gtk4_layer_shell::Edge::Bottom, 12);

  let notification_container = gtk4::Box::builder()
    .orientation(gtk4::Orientation::Vertical)
    .spacing(8)
    .valign(gtk4::Align::End)
    .build();

  let notification_container_clone = notification_container.clone();

  glib::spawn_future_local(async move {
    while let Ok(notification) = notification_reciever.recv().await {
      let notification_item = widgets::common::notification::create_notification_item(notification);

      println!("recieved");
      notification_container_clone.append(
        &notification_item
      );

      let notification_container_clone = notification_container_clone.clone();

      glib::spawn_future_local(async move {
        println!("spawned");
        glib::timeout_future(Duration
          ::from_secs(5000)).await;

        println!("removing...");
        notification_container_clone.remove(
          &notification_item
        );
      });
    }
  });

  window.set_child(
    Some(&notification_container)
  );

  window.present();
}

