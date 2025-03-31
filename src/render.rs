use std::{
    collections::HashMap,
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
    src: impl AsRef<Path>,
    dest: impl AsRef<Path>,
    overwrite: bool,
    placeholder_values: &HashMap<String, String>,
) -> io::Result<()> {
    let mut stack = vec![PathBuf::new()];

    while let Some(path) = stack.pop() {
        let path_src = src.as_ref().join(&path);

        if path_src == PathBuf::from(TEMPLATE_CONFIG_FILE) {
            continue;
        }

        let path_dest = dest.as_ref().join(&path);

        if path_src.is_dir() {
            if !fs::exists(&path_dest)? {
                fs::create_dir_all(&path_dest)?;
            }

            for entry in fs::read_dir(path_src)? {
                let entry = entry?;
                stack.push(path.join(entry.file_name()));
            }
        } else {
            let src_contents = fs::read_to_string(path_src)?;
            let src_contents_replaced = replace_placeholders(&src_contents, placeholder_values);

            let dest_exists = path_dest.try_exists()?;

            let dest_contents = dest_exists
                .then(|| fs::read_to_string(&path_dest).map(Some))
                .unwrap_or(Ok(None))?;

            let is_identical =
                dest_contents.is_some_and(|dest_contents| src_contents == dest_contents);

            let action = if !dest_exists {
                FileAction::Create
            } else if is_identical {
                FileAction::Identical
            } else if overwrite {
                FileAction::Overwrite
            } else {
                FileAction::Exists
            };

            println!("{} {}", action.stylized(), path_dest.to_string_lossy());

            if action.is_create() || action.is_overwrite() {
                fs::write(&path_dest, &src_contents_replaced)?;
            }
        }
    }

    Ok(())
}
