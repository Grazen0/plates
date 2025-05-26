use std::{fs::File, io, path::PathBuf};

use serde::Deserialize;

use crate::{error::PlatesResult, placeholder::Placeholder};

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

    names.sort();
    Ok(names)
}

pub fn get_template_dir(template_name: &str) -> io::Result<PathBuf> {
    let templates_dir = get_templates_dir()?;
    Ok(templates_dir.join(template_name))
}

pub fn get_template_config_path(template_name: &str) -> io::Result<PathBuf> {
    let template_dir = get_template_dir(template_name)?;
    Ok(template_dir.join("_plates.yml"))
}

pub fn get_template_config(template_name: &str) -> PlatesResult<Option<TemplateConfig>> {
    let config_path = get_template_config_path(template_name)?;

    if config_path.is_file() {
        let reader = File::open(config_path)?;
        Ok(serde_yaml::from_reader(reader)?)
    } else {
        Ok(None)
    }
}
