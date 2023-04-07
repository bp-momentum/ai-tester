use jsonschema::{JSONSchema};
use serde_json::json;

pub(crate) fn validate_landmarks(landmarks: &str) -> bool {
  if landmarks.trim().is_empty() { return true; }
  let schema = json!({
    "type": "array",
    "items": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "x": {"type": "number"},
          "y": {"type": "number"},
          "z": {"type": "number"},
          "visibility": {"type": "number"}
        },
        "required": ["x", "y", "z", "visibility"]
      }
    }
  });
  let schema = JSONSchema::compile(&schema);
  if schema.is_err() {
    return false;
  }
  let schema = schema.unwrap();
  let data = serde_json::from_str(landmarks);
  if data.is_err() {
    return false;
  }
  let data = data.unwrap();
  schema.is_valid(&data)
}
