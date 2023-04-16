
mod helper;
use helper::validate_landmarks;
use serde_json::{Map, Value};
use tauri::Manager;
use crate::{LANDMARKS, SERVER_THREAD, server::spawn_tokio};

use pyo3::prelude::*;


#[tauri::command]
pub(crate) fn set_expectation_landmarks(landmarks: String) -> bool {
  if !validate_landmarks(landmarks.as_str()) {
    *LANDMARKS.lock().unwrap() = None;
    return false;
  }
  *LANDMARKS.lock().unwrap() = Some(landmarks);
  true
}

#[tauri::command]
pub(crate) fn set_expectation_video(app_handle: tauri::AppHandle, video: String) -> bool {
  // video is a path to a video file
  if !std::path::Path::new(&video).exists() {
    return false;
  }

  println!("video: {}", video);

  let pose_main = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/pose_extractor/main.py"));

  let from_python: Result<String, PyErr> = Python::with_gil(|py| -> Result<String, PyErr> {
    let app: Py<PyAny> = PyModule::from_code(py, pose_main, "", "")?
        .getattr("run")?
        .into();
    let res = app.call1(py, (video,));
    // check if res is a string
    let res = res?.extract::<String>(py)?;
    Ok(res)
  });

  if let Ok(from_python) = from_python {
    // assume from_python is a serialized json string
    if !validate_landmarks(from_python.as_str()) {
      *LANDMARKS.lock().unwrap() = None;
      return false;
    }
    *LANDMARKS.lock().unwrap() = Some(from_python);

    // TODO: notify frontend of finished landmark extraction
    app_handle.emit_all("landmarks_extracted", "").unwrap();
  } else {
    println!("from python: {:?}", from_python);
    *LANDMARKS.lock().unwrap() = None;
    return false;
  }
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