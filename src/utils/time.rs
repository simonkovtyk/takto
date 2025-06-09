use chrono::Timelike;

pub fn get_time_str() -> String {
  let now = chrono::Local::now();

  return format!("{:02}:{:02}", now.hour(), now.minute());
}
