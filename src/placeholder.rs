use std::{collections::HashMap, env};

use inquire::{
    Text,
    ui::{Color, RenderConfig, Styled},
};
use serde::Deserialize;
use serde_either::StringOrStruct;

use crate::{PlatesError, error::PlatesResult, render, shell};

pub type PlaceholderValueMap = HashMap<String, String>;

#[derive(Debug, Clone, Copy, Default, Deserialize)]
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
    pub kind: DefaultValueType,
    pub value: String,
}

impl DefaultValue {
    pub fn eval(&self) -> PlatesResult<String> {
        match self.kind {
            DefaultValueType::Str => Ok(self.value.clone()),
            DefaultValueType::Shell => {
                let output = shell::create_shell_command(&self.value).output()?;

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
    pub fn inquire_value(&self, placeholder_values: &PlaceholderValueMap) -> PlatesResult<String> {
        let message = self
            .message
            .clone()
            .unwrap_or_else(|| format!("{}:", self.name));

        let mut prompt = Text::new(&message).with_render_config(
            RenderConfig::default_colored()
                .with_prompt_prefix(Styled::new("%").with_fg(Color::DarkBlue)),
        );
        let default_input: String;

        if let Some(placeholder_default) = self.default.as_ref() {
            let unreplaced_default = match placeholder_default {
                StringOrStruct::String(s) => s.clone(),
                StringOrStruct::Struct(def) => def.eval()?,
            };
            default_input = render::replace_placeholders(&unreplaced_default, placeholder_values);

            if !default_input.is_empty() {
                prompt = prompt.with_default(&default_input);
            }
        }

        Ok(prompt.prompt()?)
    }
}

pub fn inquire_placeholders(
    placeholders: Vec<Placeholder>,
    placeholder_values: &mut PlaceholderValueMap,
) -> PlatesResult<()> {
    for placeholder in placeholders {
        let value = placeholder.inquire_value(placeholder_values)?;
        placeholder_values.insert(placeholder.name, value);
    }
    Ok(())
}
