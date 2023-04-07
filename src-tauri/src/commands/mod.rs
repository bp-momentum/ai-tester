
mod helper;
use helper::validate_landmarks;
use crate::LANDMARKS;

#[tauri::command]
pub(crate) fn set_expectation_landmarks(landmarks: String) -> bool {
  if !validate_landmarks(landmarks.as_str()) {
    return false;
  }
  unsafe {
    LANDMARKS = landmarks;
  }
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