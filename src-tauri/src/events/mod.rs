// // the payload type must implement `Serialize` and `Clone`.
// #[derive(Clone, serde::Serialize)]
// struct Payload {
//   message: String,
// }

use serde_json::{json, Value};
use tauri::{AppHandle, Manager};

pub(crate) fn ws_connected_event(app: &AppHandle) {
  app.emit_all("ws_connected", json!({})).unwrap_or_else(|e| {
    println!("Error: {}", e);
  });
}

pub(crate) fn send_live_feedback(app: &AppHandle, feedback: Value) {
  app.emit_all("live_feedback", feedback).unwrap_or_else(|e| {
    println!("Error: {}", e);
  });
}

pub(crate) fn send_persistent_feedback(app: &AppHandle, feedback: Value) {
  app.emit_all("persistent_feedback", feedback).unwrap_or_else(|e| {
    println!("Error: {}", e);
  });
}