pub fn get_home_env () -> String {
  return std::env::var("HOME").expect("HOME env not found");
}
