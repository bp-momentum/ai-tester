#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#![forbid(unsafe_code)]

use std::{io::Read, sync::Mutex, thread::JoinHandle};
use once_cell::sync::Lazy;
use base64::{Engine as _, engine::general_purpose};

use tauri::{http::ResponseBuilder, AppHandle};

mod commands;
use commands::*;

mod server;
use server::*;

mod socket;
use socket::*;
use tokio::sync::oneshot::Sender;

mod events;

type ServerThreadResult = std::result::Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
type ServerThreadTuple = (JoinHandle<ServerThreadResult>,Sender<()>);

static LANDMARKS: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
static APP_HANDLE: Lazy<Mutex<Option<AppHandle>>> = Lazy::new(|| Mutex::new(None));
static SERVER_THREAD: Lazy<Mutex<Option<ServerThreadTuple>>> = Lazy::new(|| Mutex::new(None));

fn main() {
  tauri::Builder::default()
    .register_uri_scheme_protocol("video", move |_, request| {
      let res_not_img = ResponseBuilder::new()
      .status(404)
      .body(Vec::new());
      if request.method() != "GET" { return res_not_img; }
      let uri = request.uri();
      // uri is formatted like this: "https://video.localhost/base64_encoded_path"
      let Some(path) = uri.split('/').last() else {
        return res_not_img;
      };
      let Ok(path) = general_purpose::STANDARD.decode(path.as_bytes()) else {
        return res_not_img;
      };
      let Ok(path) = String::from_utf8(path) else {
        return res_not_img;
      };
      let path = std::path::Path::new(&path);
      if !path.exists() { return res_not_img; }
      let Some(ext) = path.extension() else {
        return res_not_img;
      };
      let Some(ext) = ext.to_str() else {
        return res_not_img;
      };
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
      let Ok(mut file) = std::fs::File::open(path) else {
        return res_not_img;
      };
      let mut buf = Vec::new();
      if file.read_to_end(&mut buf).is_err() { return res_not_img; }
      let res = ResponseBuilder::new()
      .status(200)
      .header("Content-Type", mime)
      .header("Access-Control-Allow-Origin", "*")
      .header("Access-Control-Allow-Headers", "referer, range, accept-encoding, x-requested-with")
      .body(buf);
      res
    })
    .invoke_handler(tauri::generate_handler![
      set_expectation_landmarks, 
      set_expectation_video,
      can_run_tests,
      connect_ws,
      send_image,
      end_repetition,
      end_set,
      spawn_server,
      disconnect_ws,
    ])
    .setup(|app| {
      let handle = app.handle();
      *APP_HANDLE.lock().unwrap() = Some(handle);
      
      let (tx, rx) = tokio::sync::oneshot::channel::<()>();
      *SERVER_THREAD.lock().unwrap() = Some((std::thread::spawn(|| {
        spawn_tokio(3000, rx)
      }), tx));

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("Error while running tauri application");
}
