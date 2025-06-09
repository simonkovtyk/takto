use gtk4::glib;
use crate::utils;

pub fn create_clock_label () -> gtk4::Label {
  let label = gtk4::Label::builder()
    .label(&utils::time::get_time_str())
    .name("taskbar-clock__label")
    .halign(gtk4::Align::End)
    .build();

  let label_clone = label.clone();

  glib::timeout_add_seconds_local(20, move || {
    label_clone.set_label(&utils::time::get_time_str());

    glib::ControlFlow::Continue
  });

  return label;
}
