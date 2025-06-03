use gtk::{Box, Image};
use std::env;

pub fn get_horizontal_box_spacer() -> Box {
  return Box::builder()
    .orientation(gtk::Orientation::Horizontal)
    .hexpand(true)
    .build()
}

pub fn image_from_path(path: &str, width: i32, height: i32) -> Image {
  let home_env = env::var("HOME").expect("HOME env not found");

  println!("{}", home_env);

  let pixbuf = Pixbuf::from_file(format!("{}/.config/gtk-widgets/arch.png", home_env)).expect("Image not found");

  let scaled_pixbuf = pixbuf.scale_simple(40, 40, gtk::gdk_pixbuf::InterpType::Bilinear).expect("Could not scale pixbuf");
  let image_texture = Texture::for_pixbuf(&scaled_pixbuf);

  let image = Image::from_paintable(Some(&image_texture));
}
