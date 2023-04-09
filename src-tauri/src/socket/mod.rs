use std::{sync::Mutex, collections::HashMap};

use once_cell::sync::Lazy;
use rust_socketio::{ClientBuilder, client::Client, Payload, RawClient};
use serde_json::json;

use crate::events::{ws_connected_event, send_live_feedback};

static SOCKET_MAP: Lazy<Mutex<HashMap<String,Client>>> = Lazy::new(|| {
  Mutex::new(HashMap::new())
});

#[tauri::command]
pub(crate) fn connect_ws(app_handle: tauri::AppHandle, url: String) -> Result<String, String> {
  for (_, socket) in SOCKET_MAP.lock().unwrap().drain() {
    socket.disconnect().unwrap();
    drop(socket);
  }

  let app_handle1 = app_handle.clone();
  let app_handle2 = app_handle;  // no need to clone for the last one

  // generate a random uuid
  let uuid = uuid::Uuid::new_v4().to_string();
  let uuid_clone = uuid.clone();

  let Ok(socket) = ClientBuilder::new(url)
    .reconnect(false)
    .on("open", move |_: Payload, sock: RawClient| {
      sock.emit("set_exercise_id", json!({
        "exercise": 1
      })).unwrap_or_else(|e| {
        println!("Error: {}", e);
      });

      sock.emit("debug", json!({
        "uuid": uuid_clone
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
    return Err("Could not connect to server".to_string());
  };

  SOCKET_MAP.lock().unwrap().insert(uuid.clone(), socket);
  Ok(uuid)
}

#[tauri::command]
pub(crate) fn send_image(image: String, uuid: String) {
  if let Some(socket) = SOCKET_MAP.lock().unwrap().get(&uuid) {
    socket.emit("send_video", image).unwrap_or_else(|e| {
      println!("Error: {}", e);
    });
  }
}

#[tauri::command]
pub(crate) fn end_repetition(uuid: String) {
  if let Some(socket) = SOCKET_MAP.lock().unwrap().get(&uuid) {
    socket.emit("end_repetition", json!({})).unwrap_or_else(|e| {
      println!("Error: {}", e);
    });
  }
}

#[tauri::command]
pub(crate) fn end_set(uuid: String) {
  if let Some(socket) = SOCKET_MAP.lock().unwrap().remove(&uuid) {
    socket.emit("end_set", json!({
      "set_uuid": uuid
    })).unwrap_or_else(|e| {
      println!("Error: {}", e);
    });
    socket.disconnect().unwrap_or_else(|e| {
      println!("Error: {}", e);
    });

    drop(socket);
  }
}

#[tauri::command]
pub(crate) fn disconnect_ws(uuid: String) {
  if let Some(socket) = SOCKET_MAP.lock().unwrap().remove(&uuid) {
    socket.disconnect().unwrap_or_else(|e| {
      println!("Error: {}", e);
    });
    drop(socket);
  }
}