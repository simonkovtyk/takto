use chrono::{Local, Timelike};
use gtk::glib::timeout_add_seconds_local;
use gtk::{Align, Application, ApplicationWindow, Label};
use gtk4_layer_shell::{Edge, LayerShell};
use std::env;
use gtk::gdk::Texture;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::{prelude::*, Box, Image};

use crate::utils::gtk::get_horizontal_box_spacer;

pub fn connect(app: &mut Application) {
  app.connect_activate(init);
}

fn init(app: &Application) {
  let window = ApplicationWindow::builder()
    .application(app)
    .default_height(40)
    .name("taskbar__window")
    .build();

  let box_container = Box::builder()
    .orientation(gtk::Orientation::Horizontal)
    .name("main__container")
    .spacing(0)
    .margin_start(12)
    .margin_end(12)
    .build();

  let mut left_container = Box::builder()
    .orientation(gtk::Orientation::Horizontal)
    .name("left__container")
    .spacing(0)
    .build();

  let mut right_container = Box::builder()
    .orientation(gtk::Orientation::Horizontal)
    .name("right__container")
    .spacing(0)
    .build();

  window.init_layer_shell();

  window.set_anchor(Edge::Left, true);
  window.set_anchor(Edge::Top, true);
  window.set_anchor(Edge::Right, true);
  window.set_anchor(Edge::Bottom, false);
  window.set_exclusive_zone(40);

  init_arch_logo(&mut left_container);
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
  let home_env = env::var("HOME").expect("HOME env not found");

  println!("{}", home_env);

  let pixbuf = Pixbuf::from_file(format!("{}/.config/gtk-widgets/arch.png", home_env)).expect("Image not found");

  let scaled_pixbuf = pixbuf.scale_simple(40, 40, gtk::gdk_pixbuf::InterpType::Bilinear).expect("Could not scale pixbuf");
  let image_texture = Texture::for_pixbuf(&scaled_pixbuf);

  let image = Image::from_paintable(Some(&image_texture));

  image.set_halign(Align::Start);

  box_container.append(
    &image,
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

    gtk::glib::ControlFlow::Continue
  });

  box_container.append(
    &label
  );
}

fn get_formatted_time() -> String {
  let now = Local::now();

  return format!("{:02}:{:02}", now.hour(), now.minute());
}
