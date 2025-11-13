use dirs::config_dir;
use std::{
    io::{Error, ErrorKind, Result},
    path::PathBuf,
};

pub enum AppDirs {
    Pfps,
}

pub fn ensure_app_config_dir() -> Result<PathBuf> {
    let base = config_dir()
        .ok_or(Error::new(
            ErrorKind::NotFound,
            "Could not find config directory",
        ))?
        .join("nomorebeans");

    let dirs_to_create = ["pfps"];

    for dir in dirs_to_create.iter() {
        let dir_path = base.join(dir);
        std::fs::create_dir_all(dir_path)?;
    }

    std::fs::create_dir_all(&base)?;
    Ok(base)
}

pub fn get_app_dir(app_dir: AppDirs) -> Result<PathBuf> {
    let base = ensure_app_config_dir()?;
    let dir_path = match app_dir {
        AppDirs::Pfps => base.join("pfps"),
    };
    Ok(dir_path)
}
