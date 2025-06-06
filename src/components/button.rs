use gtk4::{prelude::{BoxExt, ButtonExt, WidgetExt}, Align, Box, Button, Label, Orientation};

use crate::utils::gtk::image_from_path;

pub fn build_icon_button (label: &str, icon_path: &str, icon_width: i32, icon_height: i32) -> Button {
  let button = Button::new();

  let inner_box = Box::builder()
    .orientation(Orientation::Horizontal)
    .spacing(6)
    .build();

  let icon_image = image_from_path(icon_path, icon_width, icon_height);

  inner_box.append(
    &icon_image
  );

  let label = Label::builder()
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
