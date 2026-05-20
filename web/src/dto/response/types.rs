use serde_json::{Map, Number, Value};

pub trait SerializeManual {
    fn serialize_json(&self) -> String;
}

impl<Item> SerializeManual for Vec<Item>
where
    Item: SerializeManual,
{
    fn serialize_json(&self) -> String {
        let mut json: String = String::from("[");
        let mut first_item: bool = true;

        for item in self {
            if first_item {
                first_item = false;
            } else {
                json.push(',');
            }

            json.push_str(&item.serialize_json());
        }

        json.push(']');

        json
    }
}

pub struct JsonObjectBuilder {
    fields: Map<String, Value>,
}

impl JsonObjectBuilder {
    pub fn new() -> JsonObjectBuilder {
        JsonObjectBuilder {
            fields: Map::new(),
        }
    }

    pub fn with_i32(mut self, name: &str, value: i32) -> JsonObjectBuilder {
        self.fields
            .insert(name.to_string(), Value::Number(Number::from(value)));

        self
    }

    pub fn with_bool(mut self, name: &str, value: bool) -> JsonObjectBuilder {
        self.fields.insert(name.to_string(), Value::Bool(value));

        self
    }

    pub fn with_string(mut self, name: &str, value: &str) -> JsonObjectBuilder {
        self.fields
            .insert(name.to_string(), Value::String(value.to_string()));

        self
    }

    pub fn with_optional_i32(mut self, name: &str, value: Option<i32>) -> JsonObjectBuilder {
        match value {
            Some(found) => {
                self.fields
                    .insert(name.to_string(), Value::Number(Number::from(found)));
            }
            None => {
                self.fields.insert(name.to_string(), Value::Null);
            }
        }

        self
    }

    pub fn with_optional_string(mut self, name: &str, value: &Option<String>) -> JsonObjectBuilder {
        match value {
            Some(found) => {
                self.fields
                    .insert(name.to_string(), Value::String(found.clone()));
            }
            None => {
                self.fields.insert(name.to_string(), Value::Null);
            }
        }

        self
    }

    pub fn finish(self) -> String {
        Value::Object(self.fields).to_string()
    }
}