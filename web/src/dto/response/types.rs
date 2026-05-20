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
        let mut item_index: usize = 0;

        loop {
            if item_index >= self.len() {
                break;
            }

            let item: &Item = &self[item_index];

            if first_item {
                first_item = false;
            } else {
                (&mut json).push(',');
            }

            (&mut json).push_str(&item.serialize_json());
            item_index += 1;
        }

        (&mut json).push(']');

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
        (&mut self.fields)
            .insert(name.to_string(), Value::Number(Number::from(value)));

        self
    }

    pub fn with_bool(mut self, name: &str, value: bool) -> JsonObjectBuilder {
        (&mut self.fields).insert(name.to_string(), Value::Bool(value));

        self
    }

    pub fn with_string(mut self, name: &str, value: &str) -> JsonObjectBuilder {
        (&mut self.fields)
            .insert(name.to_string(), Value::String(value.to_string()));

        self
    }

    pub fn with_optional_i32(mut self, name: &str, value: Option<i32>) -> JsonObjectBuilder {
        match value {
            Some(found) => {
                (&mut self.fields)
                    .insert(name.to_string(), Value::Number(Number::from(found)));
            }
            None => {
                (&mut self.fields).insert(name.to_string(), Value::Null);
            }
        }

        self
    }

    pub fn with_optional_string(mut self, name: &str, value: &Option<String>) -> JsonObjectBuilder {
        match value {
            Some(found) => {
                (&mut self.fields)
                    .insert(name.to_string(), Value::String(found.clone()));
            }
            None => {
                (&mut self.fields).insert(name.to_string(), Value::Null);
            }
        }

        self
    }

    pub fn finish(self) -> String {
        Value::Object(self.fields).to_string()
    }
}