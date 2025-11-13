use crate::utils::fs::common::{get_app_dir, AppDirs};
use std::{
    fs,
    io::{Cursor, Error, ErrorKind, Result},
    path::PathBuf,
};

pub fn save_profile_picture(bytes: &[u8]) -> Result<PathBuf> {
    let config_dir = get_app_dir(AppDirs::Pfps)?;
    let img = image::load_from_memory(bytes).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    let file_path = format!("{}.webp", uuid::Uuid::new_v4());
    let full_path = config_dir.join(&file_path);

    let mut buf = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::WebP)
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    fs::write(&full_path, &buf)?;
    Ok(full_path)
}
