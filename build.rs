use std::{fs, path::{PathBuf}, process::Command};
use anyhow::Result;

use glob::glob;

fn main () -> Result<(), anyhow::Error> {
  let in_file_paths = glob("src_css/**/*.scss")?;
  let out_dir_buf = PathBuf::from("target_css");

  fs::create_dir_all(&out_dir_buf).unwrap();

  for in_file_path in in_file_paths {
    let in_file_buf = in_file_path?;
    let in_file_name = in_file_buf.file_name().unwrap().to_str().unwrap();

    if in_file_name.starts_with("_") {
      continue;
    }

    let in_file = in_file_buf.to_str().unwrap();

    let mut out_file_buf = in_file_buf.clone();
    out_file_buf.set_extension("css");

    let out_file = format!("{}/{}",
      out_dir_buf.to_str().unwrap(),
      out_file_buf.file_name().unwrap().to_str().unwrap()
    );

    let status = Command::new("sass")
      .arg("--style=compressed")
      .arg("--no-source-map")
      .arg(&in_file)
      .arg(&out_file)
      .status()
      .expect("Failed to execute sass command");

    if !status.success() {
      panic!("sass failed on {}", out_file);
    }

    println!("cargo:rerun-if-changed={}", out_file);
  }

  return Ok(());
}
