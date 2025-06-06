use zbus::Connection;

pub async fn send_shutdown () -> () {
  let connection = Connection::system().await.expect("Could not connect to system dbus");

  connection.call_method(
    Some("org.freedesktop.login1"),
    "/org/freedesktop/login1",
    Some("org.freedesktop.login1.Manager"),
    "PowerOff",
    &false
  ).await.expect("Could not call power off method");
}

pub async fn send_reboot () -> () {
  let connection = Connection::system().await.expect("Could not connect to system dbus");

  connection.call_method(
    Some("org.freedesktop.login1"),
    "/org/freedesktop/login1",
    Some("org.freedesktop.login1.Manager"),
    "Reboot",
    &false
  ).await.expect("Could not call power off method");
}
