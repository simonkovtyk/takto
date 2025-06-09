use gtk4::prelude::*;
use crate::utils::gtk;

pub fn build_icon_button (label: &str, icon_path: &str, icon_width: i32, icon_height: i32) -> gtk4::Button {
  let button = gtk4::Button::new();

  let inner_box = gtk4::Box::builder()
    .orientation(gtk4::Orientation::Horizontal)
    .spacing(6)
    .build();

  let icon_image = gtk::image_from_path(icon_path, icon_width, icon_height);

  inner_box.append(
    &icon_image
  );

  let label = gtk4::Label::builder()
    .label(label)
    .build();

  inner_box.append(
    &label
  );

  button.set_child(
    Some(&inner_box)
  );

  return button;
}
