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

    let relative_in_file_path_segments = in_file_buf.iter().skip(1).collect::<PathBuf>();

    let out_segment_path = relative_in_file_path_segments.as_path().to_str().unwrap();
    let mut out_path_buf = out_dir_buf.join(out_segment_path);

    out_path_buf.set_extension("css");

    let out_path = out_path_buf.to_str().unwrap();
    let in_path = in_file_buf.to_str().unwrap();

    let status = Command::new("sass")
      .arg("--style=compressed")
      .arg("--no-source-map")
      .arg(&in_path)
      .arg(&out_path)
      .status()
      .expect("Failed to execute sass command");

    if !status.success() {
      panic!("sass failed on {}", out_path);
    }

    println!("cargo:rerun-if-changed={}", in_path);
  }

  return Ok(());
}
