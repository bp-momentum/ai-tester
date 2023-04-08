use std::sync::Mutex;

use once_cell::sync::Lazy;
use rust_socketio::{ClientBuilder, client::Client, Payload, RawClient};
use serde_json::json;

use crate::events::{ws_connected_event, send_live_feedback};

static SOCKET: Lazy<Mutex<Option<Client>>> = Lazy::new(|| Mutex::new(None));

#[tauri::command]
pub(crate) fn connect_ws(app_handle: tauri::AppHandle, url: String) {
  if let Some(socket) = SOCKET.lock().unwrap().as_mut() {
    socket.disconnect().unwrap_or_else(|e| {
      println!("Error: {}", e);
    });
    return;
  }

  let app_handle1 = app_handle.clone();
  let app_handle2 = app_handle.clone();

  let Ok(socket) = ClientBuilder::new(url)
    .on("open", move |_: Payload, sock: RawClient| {
      sock.emit("set_exercise_id", json!({
        "exercise": 1
      })).unwrap_or_else(|e| {
        println!("Error: {}", e);
      });
      ws_connected_event(&app_handle1);
    })
    .on("live_feedback", move |payload: Payload, _: RawClient| {
      if let Payload::String(payload) = payload {
        if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&payload) {
          send_live_feedback(&app_handle2, payload);
        }
      }
    })
    .connect() else {
    return;
  };

  *SOCKET.lock().unwrap() = Some(socket);
}

#[tauri::command]
pub(crate) fn send_image(image: String) {
  if let Some(socket) = SOCKET.lock().unwrap().as_mut() {
    socket.emit("send_video", image).unwrap_or_else(|e| {
      println!("Error: {}", e);
    });
  }
}

#[tauri::command]
pub(crate) fn end_repetition() {
  if let Some(socket) = SOCKET.lock().unwrap().as_mut() {
    socket.emit("end_repetition", json!({})).unwrap_or_else(|e| {
      println!("Error: {}", e);
    });
  }
}

#[tauri::command]
pub(crate) fn end_set() {
  if let Some(socket) = SOCKET.lock().unwrap().as_mut() {
    socket.emit("end_set", json!({
      "set_uuid": 1
    })).unwrap_or_else(|e| {
      println!("Error: {}", e);
    });
    socket.disconnect().unwrap_or_else(|e| {
      println!("Error: {}", e);
    });
  }
  *SOCKET.lock().unwrap() = None;
}