use chrono::{Local, Timelike};
use gdk4::glib::ControlFlow;
use gdk4::Monitor;
use gtk4::glib::timeout_add_seconds_local;
use gtk4::{Align, Application, ApplicationWindow, Button, Label, Orientation, Popover};
use gtk4_layer_shell::{Edge, LayerShell};
use gtk4::{prelude::*, Box };

use crate::utils::gtk::{get_home_path, get_horizontal_box_spacer, image_from_path};

pub fn init(app: &Application, monitor: Monitor) {
  let window = ApplicationWindow::builder()
    .application(app)
    .default_height(40)
    .name("taskbar__window")
    .build();

  window.init_layer_shell();
  window.set_monitor(Some(&monitor));

  let box_container = Box::builder()
    .orientation(Orientation::Horizontal)
    .name("main__container")
    .spacing(0)
    .margin_start(12)
    .margin_end(12)
    .build();

  let mut left_container = Box::builder()
    .orientation(Orientation::Horizontal)
    .name("left__container")
    .spacing(0)
    .build();

  let mut right_container = Box::builder()
    .orientation(Orientation::Horizontal)
    .name("right__container")
    .spacing(16)
    .build();


  window.set_anchor(Edge::Left, true);
  window.set_anchor(Edge::Top, true);
  window.set_anchor(Edge::Right, true);
  window.set_anchor(Edge::Bottom, false);
  window.set_margin(Edge::Top, 8);
  window.set_margin(Edge::Bottom, 8);
  window.set_exclusive_zone(40);

  init_arch_logo(&mut left_container);
  init_boot_menu(&mut right_container, &app);
  init_clock(&mut right_container);

  box_container.append(
    &left_container
  );
  box_container.append(
    &get_horizontal_box_spacer()
  );
  box_container.append(
    &right_container
  );

  window.set_child(
    Some(&box_container)
  );

  window.present();
}

fn init_arch_logo(box_container: &mut Box) -> () {
  let image = image_from_path(
    &format!("{}/{}", get_home_path(), ".config/gtk-widgets/resources/arch.png"),
    40, 40
  );

  image.set_halign(Align::Start);

  box_container.append(
    &image,
  );
}

fn init_boot_menu(box_container: &mut Box, app: &Application) -> () {
  let button = Button::builder()
    .name("boot__button")
    .build();

  button.connect_clicked(move |_| {
    
  });

  let image = image_from_path(
    &format!("{}/{}", get_home_path(), ".config/gtk-widgets/icons/power.png"),
    24, 24
  );

  button.set_child(
    Some(&image)
  );

  box_container.append(
    &button
  );
}

fn init_clock(box_container: &mut Box) -> () {
  let label = Label::builder()
    .label(&get_formatted_time())
    .halign(Align::End)
    .build();

  let label_clone = label.clone();

  timeout_add_seconds_local(60, move || {
    label_clone.set_label(&get_formatted_time());

    ControlFlow::Continue
  });

  box_container.append(
    &label
  );
}

fn get_formatted_time() -> String {
  let now = Local::now();

  return format!("{:02}:{:02}", now.hour(), now.minute());
}
