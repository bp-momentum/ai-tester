
mod helper;
use helper::validate_landmarks;
use serde_json::{Map, Value};
use crate::{LANDMARKS, SERVER_THREAD, server::spawn_tokio};

#[tauri::command]
pub(crate) fn unset_expectation() {
  *LANDMARKS.lock().unwrap() = None;
}

#[tauri::command]
pub(crate) fn set_expectation_landmarks(landmarks: String) -> bool {
  if !validate_landmarks(landmarks.as_str()) {
    return false;
  }
  *LANDMARKS.lock().unwrap() = Some(landmarks);
  true
}

#[tauri::command]
pub(crate) fn can_run_tests() -> bool {
  LANDMARKS.lock().unwrap().is_some()
}

#[tauri::command]
pub(crate) fn spawn_server(port: u16) {
  let mut res = SERVER_THREAD.lock().unwrap();
  let Some((thread, tx)) = res.take() else {
    return;
  };
  if tx.send(()).is_ok() {
    thread.join().unwrap().unwrap();
  }

  let (tx, rx) = tokio::sync::oneshot::channel::<()>();
  *res = Some((std::thread::spawn(move || {
    spawn_tokio(port, rx)
  }), tx));
}

#[tauri::command]
pub(crate) fn get_landmarks() -> Map<String, Value> {
  let mut map = serde_json::Map::new();
  if let Some(landmarks) = LANDMARKS.lock().unwrap().clone() {
    let lmrks = serde_json::from_str(&landmarks).unwrap();
    map.insert("landmarks".to_string(), lmrks);
  }
  map
}