use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum NotificationImage {
  Binary(zbus::zvariant::OwnedValue)
}

#[derive(Debug, Clone)]
pub struct Notification {
  pub app_name: String,
  pub image: Option<NotificationImage>,
  pub body: String,
  pub summary: String
}

pub struct NotificationServer {
  pub sender: tokio::sync::broadcast::Sender<Notification>
}

#[zbus::interface(name = "org.freedesktop.Notifications")]
impl NotificationServer {
  async fn get_capabilities(&self) -> zbus::fdo::Result<Vec<String>> {
    return Ok(vec![
      "action-icons".to_string(),
      "actions".to_string(),
      "body".to_string(),
      "body-hyperlinks".to_string(),
      "body-images".to_string(),
      "body-markup".to_string(),
      "icon-static".to_string(),
      "image-data".to_string()
    ]);
  }

  async fn get_server_information(&self) ->  zbus::fdo::Result<(String, String, String, String)> {
    return Ok((
      "gtk-widgets".to_string(),
      "simonkov".to_string(),
      "1.0.0".to_string(),
      "1.3".to_string()
    ));
  }
  
  async fn close_notification(&self) -> zbus::fdo::Result<()> {
    return Ok(());
  }

  async fn notify(
    &self,
    app_name: String,
    _replaces_id: u32,
    app_icon: String,
    summary: String,
    body: String,
    actions: Vec<String>,
    hints: HashMap<String, zbus::zvariant::OwnedValue>,
    _expire_timeout: i32
  ) -> zbus::fdo::Result<u32> {
      let image_path = hints.get("image-path");
      let image_data = hints.get("image-data");
      let mut image = None;

      println!("\n--- NEUE BENACHRICHTIGUNG ---");
      println!("app_icon: {}", app_icon.len() != 0);
      println!("image_path: {}", image_path.is_some());
      println!("image_data: {}", image_data.is_some());
      println!("----------------------------\n");

      if let Some(unwrapped_image_data) = image_data {
        image = Some(NotificationImage::Binary(unwrapped_image_data.to_owned()));
      }

      let sanitized_body = if body.len() > 64 {
        let body_substring = body[..64].to_string();

        format!("{}…", body_substring)
      } else {
        body
      };

      let sanitized_summary = if summary.len() > 32 {
        format!("{}…", summary[..32].to_string())
      } else {
        summary
      };

      self.sender.send(Notification {
        image,
        app_name,
        body: sanitized_body,
        summary: sanitized_summary
      }).expect("Could not send notification to reciever");

    return Ok(1);
  }
}
