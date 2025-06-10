use crate::{ipc, utils};
use gtk4::prelude::*;
use gdk4::pango;

pub fn create_notification_item (notification: ipc::dbus::notifications::Notification) -> gtk4::Box {
  let notification_item_box = gtk4::Box::builder()
    .orientation(gtk4::Orientation::Horizontal)
    .halign(gtk4::Align::Start)
    .valign(gtk4::Align::Start)
    .spacing(8)
    .build();

  let notification_item_content_box = gtk4::Box::builder()
    .orientation(gtk4::Orientation::Vertical)
    .spacing(0)
    .build();

  if let Some(notification_image) = notification.image {
    match notification_image {
      ipc::dbus::notifications::NotificationImage::Binary(image) => {
        let image = utils::gtk::image_from_dbus_value(image, 48, 48);

        image.set_pixel_size(48);

        notification_item_box.append(
          &image
        );
      }
    }
  }

  let notification_app_name = gtk4::Label::builder()
    .label(notification.app_name)
    .halign(gtk4::Align::Start)
    .build();

  let notification_summary = gtk4::Label::builder()
    .label(notification.summary)
    .halign(gtk4::Align::Start)
    .wrap(true)
    .wrap_mode(pango::WrapMode::Word)
    .build();

  let notification_body = gtk4::Label::builder()
    .label(notification.body)
    .halign(gtk4::Align::Start)
    .wrap(true)
    .wrap_mode(pango::WrapMode::Word)
    .build();

  let notification_footer_separator = gtk4::Label::builder()
    .label(" — ")
    .build();

  let notification_footer = gtk4::Box::builder()
    .orientation(gtk4::Orientation::Horizontal)
    .build();

  let notification_time = gtk4::Label::builder()
    .label(utils::time::get_time_str())
    .build();

  notification_footer.append(
    &notification_time
  );

  notification_footer.append(
    &notification_footer_separator
  );

  notification_footer.append(
    &notification_app_name
  );

  notification_item_content_box.append(
    &notification_summary
  );

  notification_item_content_box.append(
    &notification_body
  );

  notification_item_content_box.append(
    &notification_footer
  );

  notification_item_box.append(
    &notification_item_content_box
  );

  return notification_item_box;
}
