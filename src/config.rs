use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

use serde::Deserialize;

use crate::{error::PlatesError, placeholder::Placeholder};

pub const TEMPLATE_CONFIG_FILE: &str = "_plates.yml";

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TemplateConfig {
    pub placeholders: Vec<Placeholder>,
}

pub fn get_config_dir() -> Result<PathBuf, xdg::BaseDirectoriesError> {
    let dirs = xdg::BaseDirectories::new()?;
    Ok(dirs.get_config_home().join("plates"))
}

pub fn get_templates_dir() -> Result<PathBuf, xdg::BaseDirectoriesError> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join("templates"))
}

pub fn get_template_names() -> io::Result<Vec<String>> {
    let templates_dir = get_templates_dir()?;
    let entries: Vec<_> = templates_dir.read_dir()?.collect::<Result<_, _>>()?;

    let mut names: Vec<_> = entries
        .into_iter()
        .filter(|entry| entry.file_type().is_ok_and(|ft| ft.is_dir()))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();

    names.sort_unstable();
    Ok(names)
}

pub fn get_template_config(template_name: &str) -> Result<Option<TemplateConfig>, PlatesError> {
    let templates_dir = get_templates_dir()?;
    let config_path = templates_dir.join(template_name).join(TEMPLATE_CONFIG_FILE);

    fs::exists(&config_path)?
        .then(|| {
            File::open(config_path)
                .map_err(PlatesError::from)
                .and_then(|rdr| serde_yaml::from_reader(rdr).map_err(PlatesError::from))
        })
        .unwrap_or(Ok(None))
}
