use gtk::Box;

pub fn get_horizontal_box_spacer() -> Box {
  return Box::builder()
    .orientation(gtk::Orientation::Horizontal)
    .hexpand(true)
    .build()
}
