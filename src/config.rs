use std::{
    collections::HashMap,
    fs::{self, DirEntry, File},
    io,
    path::{Path, PathBuf},
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

pub fn get_template_dir_entries(dir: &Path) -> io::Result<HashMap<String, DirEntry>> {
    let entries: Vec<_> = dir.read_dir()?.collect::<Result<_, _>>()?;

    let entry_map: HashMap<_, _> = entries
        .into_iter()
        .filter(|entry| entry.file_type().is_ok_and(|ft| ft.is_dir()))
        .map(|entry| {
            let template_name = entry.file_name().to_string_lossy().into_owned();
            (template_name, entry)
        })
        .collect();
    Ok(entry_map)
}

pub fn get_template_config(
    template_dir: impl AsRef<Path>,
) -> Result<Option<TemplateConfig>, PlatesError> {
    let config_path = template_dir.as_ref().join(TEMPLATE_CONFIG_FILE);

    fs::exists(&config_path)?
        .then(|| {
            File::open(config_path)
                .map_err(PlatesError::from)
                .and_then(|rdr| serde_yaml::from_reader(rdr).map_err(PlatesError::from))
        })
        .unwrap_or(Ok(None))
}
