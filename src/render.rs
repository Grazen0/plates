use std::{
    collections::HashMap,
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
};

use crossterm::style::{StyledContent, Stylize};
use derive_more::{Display, IsVariant};
use lazy_regex::regex_replace_all;

use crate::config::TEMPLATE_CONFIG_FILE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, IsVariant)]
pub enum FileAction {
    #[display("create")]
    Create,
    #[display("exists")]
    Exists,
    #[display("overwrite")]
    Overwrite,
    #[display("identical")]
    Identical,
}

impl FileAction {
    pub fn stylized(&self) -> StyledContent<String> {
        let display_str = format!("{:>9}", self.to_string());
        match self {
            Self::Identical => display_str.dark_grey().bold(),
            Self::Overwrite => display_str.dark_magenta().bold(),
            Self::Create => display_str.dark_green().bold(),
            Self::Exists => display_str.dark_yellow().bold(),
        }
    }
}

fn replace_placeholders(text: &str, placeholder_values: &HashMap<String, String>) -> String {
    regex_replace_all!(r#"\{\{\s*(\w+)\s*}}"#, text, |original: &str, name| {
        placeholder_values
            .get(name)
            .map(String::as_ref)
            .unwrap_or(original)
            .to_owned()
    })
    .into_owned()
}

pub fn render_template(
    root_src: impl AsRef<Path>,
    root_dest: impl AsRef<Path>,
    overwrite: bool,
    placeholder_values: &HashMap<String, String>,
) -> io::Result<()> {
    let mut stack = vec![(root_src.as_ref().to_owned(), root_dest.as_ref().to_owned())];

    while let Some((src, dest)) = stack.pop() {
        if src == PathBuf::from(TEMPLATE_CONFIG_FILE) {
            continue;
        }

        if src.is_dir() {
            if !fs::exists(&dest)? {
                fs::create_dir_all(&dest)?;
            }

            for entry in fs::read_dir(&src)? {
                let file_name = entry?.file_name();
                let file_name_replaced = file_name
                    .clone()
                    .into_string()
                    .map(|s| OsString::from(replace_placeholders(&s, placeholder_values)))
                    .unwrap_or_else(|s| s);

                stack.push((src.join(file_name), dest.join(file_name_replaced)));
            }
        } else {
            let src_contents = fs::read_to_string(src)?;
            let src_contents_replaced = replace_placeholders(&src_contents, placeholder_values);

            let dest_exists = dest.try_exists()?;

            let dest_contents = dest_exists
                .then(|| fs::read_to_string(&dest).map(Some))
                .unwrap_or(Ok(None))?;

            let is_identical =
                dest_contents.is_some_and(|dest_contents| src_contents_replaced == dest_contents);

            let action = if !dest_exists {
                FileAction::Create
            } else if is_identical {
                FileAction::Identical
            } else if overwrite {
                FileAction::Overwrite
            } else {
                FileAction::Exists
            };

            println!("{} {}", action.stylized(), dest.to_string_lossy());

            if action.is_create() || action.is_overwrite() {
                fs::write(&dest, &src_contents_replaced)?;
            }
        }
    }

    Ok(())
}
