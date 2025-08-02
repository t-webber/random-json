use crate::{
    data::generate::generate_data,
    errors::{Error, Res},
};
use rand::{Rng as _, rngs::ThreadRng};
use serde_json::{Map, Value};
use std::fs;

fn generate_json(json: &Value, rng: &mut ThreadRng) -> Option<Value> {
    Some(match json {
        Value::Null | Value::Bool(_) | Value::Number(_) => {
            panic!("not-supported: {json}")
        }
        Value::String(data_type) => {
            let parsed_data_type = if data_type.ends_with("?") {
                if rng.random_bool(0.3) {
                    return None;
                } else {
                    &data_type[..&data_type.len() - 1]
                }
            } else {
                data_type
            };
            Value::String(generate_data(parsed_data_type, rng))
        }
        Value::Array(values) => {
            let new_values = values
                .iter()
                .filter_map(|son| generate_json(son, rng))
                .collect();
            Value::Array(new_values)
        }
        Value::Object(map) => {
            let mut new_map = Map::with_capacity(map.len());
            for (k, v) in map {
                if let Some(generated_value) = generate_json(v, rng) {
                    new_map.insert(k.to_string(), generated_value);
                }
            }
            Value::Object(new_map)
        }
    })
}

pub struct JsonArgs<'rng> {
    before: String,
    after: String,
    count: u32,
    file: String,
    rng: &'rng mut ThreadRng,
}

impl<'rng> JsonArgs<'rng> {
    pub fn new(
        before: String,
        after: String,
        count: u32,
        file: String,
        rng: &'rng mut ThreadRng,
    ) -> Self {
        Self {
            before,
            after,
            count,
            file,
            rng,
        }
    }

    pub fn generate(self) -> Res {
        let json_file_content = match fs::read_to_string(&self.file) {
            Err(error) => {
                return Err(Error::FileNotFound {
                    file: self.file,
                    error,
                });
            }
            Ok(content) => content,
        };
        let json: Value =
            serde_json::from_str(&json_file_content).map_err(Error::invalid_file(self.file))?;

        for _ in 0..self.count {
            let generate_json = generate_json(&json, self.rng).unwrap_or_default();
            let generate_json_str = serde_json::to_string_pretty(&generate_json).unwrap();
            println!("{}\n{generate_json_str}\n{}", self.before, self.after);
        }

        Ok(())
    }
}
