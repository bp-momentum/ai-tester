#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::io::Read;
use base64::{Engine as _, engine::general_purpose};

use tauri::http::ResponseBuilder;

mod commands;
use commands::*;

static mut LANDMARKS: String = String::new();

fn main() {
  tauri::Builder::default()
    .register_uri_scheme_protocol("video", move |_, request| {
      let res_not_img = ResponseBuilder::new()
      .status(404)
      .body(Vec::new());
      if request.method() != "GET" { return res_not_img; }
      let uri = request.uri();
      // uri is formated like this: "https://video.localhost/base64_encoded_path"
      let path = uri.split("/").last().unwrap();
      let path = general_purpose::STANDARD.decode(path.as_bytes()).unwrap();
      let path = String::from_utf8(path).unwrap();
      let path = std::path::Path::new(&path);
      if !path.exists() { return res_not_img; }
      let ext = path.extension().unwrap().to_str().unwrap();
      let mime = match ext {
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "ogg" => "video/ogg",
        "avi" => "video/x-msvideo",
        "mov" => "video/quicktime",
        "mkv" => "video/x-matroska",
        "flv" => "video/x-flv",
        "wmv" => "video/x-ms-wmv",
        "mpg" => "video/mpeg",
        "mpeg" => "video/mpeg",
        "m4v" => "video/x-m4v",
        _ => return res_not_img,
      };
      let mut file = std::fs::File::open(path).unwrap();
      let mut buf = Vec::new();
      if file.read_to_end(&mut buf).is_err() { return res_not_img; }
      let res = ResponseBuilder::new()
      .status(200)
      .header("Content-Type", mime)
      .body(buf);
      res
    })
    .invoke_handler(tauri::generate_handler![
      set_expectation_landmarks, 
      set_expectation_video
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
