use std::{env, process};

use inquire::Text;
use serde::Deserialize;
use serde_either::StringOrStruct;

use crate::PlatesError;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DefaultValueType {
    #[default]
    Str,
    Shell,
    Env,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DefaultValue {
    #[serde(rename = "type")]
    kind: DefaultValueType,
    value: String,
}

impl DefaultValue {
    pub fn eval(&self) -> Result<String, PlatesError> {
        match self.kind {
            DefaultValueType::Str => Ok(self.value.clone()),
            DefaultValueType::Shell => {
                let output = process::Command::new("/bin/sh")
                    .arg("-c")
                    .arg(&self.value)
                    .output()?;

                if !output.status.success() {
                    Err(PlatesError::Shell(output.status))
                } else {
                    Ok(String::from_utf8_lossy(output.stdout.trim_ascii()).into_owned())
                }
            }
            DefaultValueType::Env => {
                env::var(&self.value).map_err(|e| PlatesError::EnvVar(self.value.clone(), e))
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Placeholder {
    pub name: String,
    pub message: Option<String>,
    pub default: Option<StringOrStruct<DefaultValue>>,
}

impl Placeholder {
    pub fn inquire_value(&self) -> Result<String, PlatesError> {
        let message = self
            .message
            .clone()
            .unwrap_or_else(|| format!("{}:", self.name));

        let mut prompt = Text::new(&message);
        let default_input: String;

        if let Some(placeholder_default) = self.default.as_ref() {
            default_input = match placeholder_default {
                StringOrStruct::String(s) => s.clone(),
                StringOrStruct::Struct(def) => def.eval()?,
            };
            prompt = prompt.with_default(&default_input);
        }

        Ok(prompt.prompt()?)
    }
}
