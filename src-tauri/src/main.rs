#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#![forbid(unsafe_code)]

use std::{sync::Mutex, thread::JoinHandle};
use once_cell::sync::Lazy;

use tauri::AppHandle;

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
      get_landmarks,
      unset_expectation,
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
