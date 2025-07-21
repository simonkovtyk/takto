pub fn init_notification () -> () {
  /*
  let (notification_sender, _notification_reciever) = tokio::sync::broadcast::channel(1);
  let notification_sender_clone = notification_sender.clone();

  tokio::task::spawn(async move {
    let _connt = zbus::connection::Builder::session().unwrap()
      .name("org.freedesktop.Notifications").unwrap()
      .serve_at("/org/freedesktop/Notifications", ipc::dbus::notifications::NotificationServer {
        sender: notification_sender_clone
      }).unwrap()
      .build()
      .await.unwrap();

    pending::<()>().await;
  });
  */
}
