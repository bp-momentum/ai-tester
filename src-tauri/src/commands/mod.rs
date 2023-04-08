
mod helper;
use helper::validate_landmarks;
use crate::{LANDMARKS, SERVER_THREAD, server::spawn_tokio};

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
pub(crate) fn set_expectation_video(video: String) -> bool {
  // video is a path to a video file
  if !std::path::Path::new(&video).exists() {
    return false;
  }
  // TODO: process video using pose-extractor
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
  tx.send(()).unwrap();
  thread.join().unwrap().unwrap();

  let (tx, rx) = tokio::sync::oneshot::channel::<()>();
  *res = Some((std::thread::spawn(move || {
    spawn_tokio(port, rx)
  }), tx));
}