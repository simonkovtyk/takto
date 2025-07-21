use async_channel::Receiver;
use gtk4::glib;

use crate::utils::time::get_time_str;

pub fn init_clock_listener () -> Receiver<String> {
  let (sender, reciever) = async_channel::bounded::<String>(1);

  glib::spawn_future_local(async move {
    loop {
      sender.send(get_time_str()).await
        .expect("Could not send clock information");

      glib::timeout_future_seconds(60).await;
    }
  });

  return reciever;
}
