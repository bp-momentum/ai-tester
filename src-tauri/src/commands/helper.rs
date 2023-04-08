use jsonschema::{JSONSchema};
use serde_json::{json, Value};

pub(crate) fn validate_landmarks(landmarks: &str) -> bool {
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
      },
      "minItems": 33,
      "maxItems": 33
    },
    "minItems": 1
  });
  let Ok(schema) = JSONSchema::compile(&schema) else {
    return false;
  };
  let Ok(data) = serde_json::from_str::<Value>(landmarks) else {
    return false;
  };
  let data = data;
  schema.is_valid(&data)
}
