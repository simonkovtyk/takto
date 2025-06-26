use gtk4::prelude::*;

use crate::utils;

pub fn create_os_image () -> gtk4::Image {
  let image = utils::gtk::image_from_binary_data(include_bytes!("../../../assets/arch.png"));

  image.set_halign(gtk4::Align::Start);

  return image;
}
