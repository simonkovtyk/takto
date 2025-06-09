use gtk4::prelude::*;
use gtk4::pango;
use gio::glib;
use crate::{ipc, utils};

pub fn create_notification_menu (parent_button: &gtk4::Button, mut notification_reciever: tokio::sync::broadcast::Receiver<ipc::dbus::notifications::Notification>) {
  let image = utils::gtk::image_from_path(
    &format!("{}/{}", utils::env::get_home_env(), ".config/gtk-widgets/icons/bell.png"),
    24, 24
  );

  let badge_image = utils::gtk::image_from_path(
    &format!("{}/{}", utils::env::get_home_env(), ".config/gtk-widgets/icons/badge.png"),
    32, 32);

  badge_image.set_pixel_size(12);
  badge_image.set_margin_top(8);

  badge_image.set_halign(gtk4::Align::End);
  badge_image.set_valign(gtk4::Align::Start);
  badge_image.set_margin_start(2);
  badge_image.set_visible(false);

  let notification_box = gtk4::Box::builder()
    .orientation(gtk4::Orientation::Vertical)
    .spacing(16)
    .build();

  let empty_label = gtk4::Label::builder()
    .label("Nothing here")
    .build();
  
  notification_box.append(
    &empty_label
  );

  let notification_box_clone = notification_box.clone();
  let badge_image_clone = badge_image.clone();

  glib::spawn_future_local(async move {
    while let Ok(notification) = notification_reciever.recv().await {
      notification_box_clone.remove(&empty_label);
      notification_box_clone.set_width_request(384);

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

      badge_image_clone.set_visible(true);

      notification_box_clone.prepend(
        &notification_item_box
      );
    }
  });

  let popover = gtk4::Popover::builder()
    .autohide(true)
    .build();

  let revealer = gtk4::Revealer::builder()
    .transition_type(gtk4::RevealerTransitionType::SlideDown)
    .transition_duration(200)
    .reveal_child(false)
    .build();

  let popover_scroll_container = gtk4::ScrolledWindow::builder()
    .child(&notification_box)
    .width_request(386)
    .height_request(480)
    .build();

  revealer.set_child(
    Some(&popover_scroll_container)
  );

  popover.set_child(
    Some(&revealer)
  );

  popover.set_parent(
    parent_button
  );

  let revealer_clone = revealer.clone();
  let parent_button_clone = parent_button.clone();

  popover.connect_show(move |_| {
    revealer_clone.set_reveal_child(true);
    parent_button_clone.add_css_class("taskbar-side-item__button--active");
  });

  let revealer_clone = revealer.clone();
  let parent_button_clone = parent_button.clone();

  popover.connect_hide(move |_| {
    revealer_clone.set_reveal_child(false);
    parent_button_clone.remove_css_class("taskbar-side-item__button--active");
  });

  parent_button.connect_clicked(move |_| {
    popover.popup();
  });


  let button_overlay = gtk4::Overlay::builder()
    .build();

  image.set_margin_end(4);
  image.set_margin_start(4);

  button_overlay.set_child(
    Some(&image)
  );
  button_overlay.add_overlay(
    &badge_image
  );

  parent_button.set_child(
    Some(&button_overlay)
  );
}
