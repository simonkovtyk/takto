use gtk4::prelude::*;

use crate::utils;

pub fn create_os_image () -> gtk4::Image {
  let image = utils::gtk::image_from_path(
    &format!("{}/{}", utils::env::get_home_env(), ".config/gtk-widgets/resources/arch.png"),
    40, 40
  );

  image.set_halign(gtk4::Align::Start);

  return image;
}
