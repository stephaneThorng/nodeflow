use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthorizeRequest {
    pub fields: Vec<Field>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthorizeResponse {
    pub fields: Vec<Field>,
}

#[derive(Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub value: FieldValue,
    pub required: bool,
}

#[derive(Serialize, Deserialize)]
pub enum FieldValue {
    Info(String),
    String(String),
    Number(i64),
    Float(f64),
    Boolean(bool),
    // Add other types as needed
}

impl FieldValue {

    pub fn as_string(&self) -> String {
        match self {
            FieldValue::Info(i) => i.clone(),
            FieldValue::String(s) => s.clone(),
            FieldValue::Number(n) => n.to_string(),
            FieldValue::Float(f) => f.to_string(),
            FieldValue::Boolean(b) => b.to_string(),
        }
    }
}

impl AuthorizeResponse {
    pub fn new() -> Self {
        Self {
            fields: vec![],
        }
    }

    pub fn add_field(&mut self, name: String, required: bool, value: FieldValue) {
        self.fields.push(Field {
            name,
            value,
            required,
        });
    }
}