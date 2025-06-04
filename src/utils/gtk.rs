use std::env;

use gdk4::{gdk_pixbuf::{InterpType, Pixbuf}, Texture};
use gtk4::{Box, Image, Orientation};

pub fn get_horizontal_box_spacer() -> Box {
  return Box::builder()
    .orientation(Orientation::Horizontal)
    .hexpand(true)
    .build()
}

pub fn get_home_path () -> String {
  return env::var("HOME").expect("HOME env not found");
}

pub fn image_from_path(path: &str, width: i32, height: i32) -> Image {
  let pixbuf = Pixbuf::from_file(path)
    .expect("Image not found");

  let scaled_pixbuf = pixbuf.scale_simple(width, height, InterpType::Bilinear).expect("Could not scale pixbuf");
  let image_texture = Texture::for_pixbuf(&scaled_pixbuf);

  return Image::from_paintable(Some(&image_texture));
}
