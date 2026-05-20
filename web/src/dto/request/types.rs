use serde_json::Value;

pub trait DeserializeManual: Sized {
    fn from_json(json: &str) -> Result<Self, String>;
}

pub struct JsonFieldReader {
    value: Value,
}

impl JsonFieldReader {
    pub fn new(json: &str) -> Result<JsonFieldReader, String> {
        let value: Value = serde_json::from_str(json).map_err(|error| error.to_string())?;

        Ok(JsonFieldReader { value })
    }

    pub fn required_string(&self, name: &str) -> Result<String, String> {
        let value: &Value = self
            .value
            .get(name)
            .ok_or_else(|| Self::required_field_error(name))?;

        let text: &str = value
            .as_str()
            .ok_or_else(|| Self::typed_field_error(name, "string"))?;

        Ok(text.to_string())
    }

    pub fn optional_string(&self, name: &str) -> Result<Option<String>, String> {
        let value: Option<&Value> = self.value.get(name);

        match value {
            Some(Value::Null) | None => Ok(None),
            Some(found) => {
                let text: &str = found
                    .as_str()
                    .ok_or_else(|| Self::typed_field_error(name, "string"))?;

                Ok(Some(text.to_string()))
            }
        }
    }

    pub fn required_bool(&self, name: &str) -> Result<bool, String> {
        let value: &Value = self
            .value
            .get(name)
            .ok_or_else(|| Self::required_field_error(name))?;

        let boolean: bool = value
            .as_bool()
            .ok_or_else(|| Self::typed_field_error(name, "boolean"))?;

        Ok(boolean)
    }

    pub fn required_i32(&self, name: &str) -> Result<i32, String> {
        let value: &Value = self
            .value
            .get(name)
            .ok_or_else(|| Self::required_field_error(name))?;

        let integer: i64 = value
            .as_i64()
            .ok_or_else(|| Self::typed_field_error(name, "integer"))?;

        let converted: i32 = i32::try_from(integer)
            .map_err(|error| Self::integer_range_error(name, error.to_string()))?;

        Ok(converted)
    }

    pub fn optional_i32(&self, name: &str) -> Result<Option<i32>, String> {
        let value: Option<&Value> = self.value.get(name);

        match value {
            Some(Value::Null) | None => Ok(None),
            Some(found) => {
                let integer: i64 = found
                    .as_i64()
                    .ok_or_else(|| Self::typed_field_error(name, "integer"))?;

                let converted: i32 = i32::try_from(integer)
                    .map_err(|error| Self::integer_range_error(name, error.to_string()))?;

                Ok(Some(converted))
            }
        }
    }

    fn required_field_error(name: &str) -> String {
        let mut message: String = String::from("Field '");
        message.push_str(name);
        message.push_str("' is required");

        message
    }

    fn typed_field_error(name: &str, expected: &str) -> String {
        let mut message: String = String::from("Field '");
        message.push_str(name);
        message.push_str("' must be a ");
        message.push_str(expected);

        message
    }

    fn integer_range_error(name: &str, error: String) -> String {
        let mut message: String = String::from("Field '");
        message.push_str(name);
        message.push_str("' is outside i32 range: ");
        message.push_str(&error);

        message
    }
}