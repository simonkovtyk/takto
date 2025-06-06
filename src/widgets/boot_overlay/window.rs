use std::{cell::RefCell, rc::Rc};

use gdk4::glib::Propagation;
use gtk4::{prelude::{BoxExt, GtkWindowExt}, Align, Box, Button, Window};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

pub fn init() -> Window {
  let window = Window::builder()
    .name("boot-overlay__window")
    .build();

  window.init_layer_shell();
  window.set_margin(Edge::Top, 8);
  window.set_margin(Edge::Left, 8);
  window.set_margin(Edge::Right, 8);
  window.set_margin(Edge::Bottom, 8);

  window.set_layer(Layer::Overlay);

  window.set_anchor(Edge::Left, true);
  window.set_anchor(Edge::Top, true);
  window.set_anchor(Edge::Right, true);
  window.set_anchor(Edge::Bottom, true);

  let content_box = Box::builder()
    .name("boot-overlay-item__container")
    .valign(Align::Center)
    .halign(Align::Center)
    .spacing(8)
    .build();

  content_box.append(
    &create_entry("Exit")
  );
  content_box.append(
    &create_entry("Reboot")
  );
  content_box.append(
    &create_entry("Shutdown")
  );

  window.set_child(
    Some(&content_box)
  );

  window.present();

  return window;
}

fn create_entry (label: &str) -> Button {
  return Button::builder()
    .label(label)
    .valign(Align::Center)
    .name("boot-overlay-item__button")
    .build();
}
