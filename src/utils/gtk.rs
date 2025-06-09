use gtk4::gdk_pixbuf;

pub fn get_horizontal_box_spacer() -> gtk4::Box {
  return gtk4::Box::builder()
    .orientation(gtk4::Orientation::Horizontal)
    .hexpand(true)
    .build()
}

pub fn image_from_path(path: &str, width: i32, height: i32) -> gtk4::Image {
  let pixbuf = gdk_pixbuf::Pixbuf::from_file(path)
    .expect("Image not found");

  let scaled_pixbuf = pixbuf.scale_simple(width, height, gdk_pixbuf::InterpType::Bilinear).expect("Could not scale pixbuf");
  let image_texture = gdk4::Texture::for_pixbuf(&scaled_pixbuf);

  return gtk4::Image::from_paintable(Some(&image_texture));
}

pub fn image_from_binary_data(data: Vec<u8>, width: i32, height: i32) -> gtk4::Image {
  let pixbuf = gdk_pixbuf::Pixbuf::from_mut_slice(data, gdk_pixbuf::Colorspace::Rgb, false, 8, width, height, width * 3);

  let scaled_pixbuf = pixbuf.scale_simple(width, height, gdk_pixbuf::InterpType::Bilinear).expect("Could not scale pixbuf");

  return gtk4::Image::from_paintable(
    Some(&gdk4::Texture::for_pixbuf(&scaled_pixbuf))
  );
}

pub fn image_from_dbus_value(data: zbus::zvariant::OwnedValue, width: i32, height: i32) -> gtk4::Image {
  let structure = data.downcast_ref::<zbus::zvariant::Structure>().expect("Could not downcast");

  let fields = structure.fields();

  let data_width = fields[0].downcast_ref::<i32>().unwrap().to_owned() as i32;
  let data_height = fields[1].downcast_ref::<i32>().unwrap().to_owned() as i32;
  let rowstride = fields[2].downcast_ref::<i32>().unwrap().to_owned() as i32;
  let has_alpha = fields[3].downcast_ref::<bool>().unwrap().to_owned();
  let bits_per_sample = fields[4].downcast_ref::<i32>().unwrap().to_owned() as i32;
  let _n_channels = fields[5].downcast_ref::<i32>().unwrap().to_owned(); // wird von Pixbuf automatisch erkannt
  let data_array = fields[6].downcast_ref::<zbus::zvariant::Array>().unwrap().to_owned();

  let mut raw_bytes = Vec::with_capacity(data_array.len());

  for data in data_array {
    if let zbus::zvariant::Value::U8(bin) = data {
      raw_bytes.push(bin);
    }
  }

  let bytes = gdk4::glib::Bytes::from(&raw_bytes);

  let pixbuf = gdk_pixbuf::Pixbuf::from_bytes(&bytes, gdk_pixbuf::Colorspace::Rgb, has_alpha, bits_per_sample, data_width, data_height, rowstride);

  let scaled_pixbuf = pixbuf.scale_simple(width, height, gdk_pixbuf::InterpType::Bilinear).unwrap();

  return gtk4::Image::from_paintable(
    Some(&gdk4::Texture::for_pixbuf(&scaled_pixbuf))
  );
}
