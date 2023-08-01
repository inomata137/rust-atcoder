use std::fs::{ self, File };
use serde::Serialize;
use serde_json::Value;

pub fn read(path: &str) -> Value {
    let json = fs::read_to_string(path)
        .expect(format!("failed to read file: {}", path).as_str());

    let json: Value = serde_json::from_str(&json)
        .expect("failed to parse JSON");

    json
}

pub fn write(path: &str, obj: &Value) {
    let file = File::create(path)
        .expect(format!("failed to open file: {}", path).as_str());

    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(file, formatter);

    Serialize::serialize(obj, &mut ser)
        .expect("failed to serialize.");
}
