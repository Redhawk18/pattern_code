use std::{ffi::OsStr, path::Path};

/// All reserved filenames and extensions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    C,
    Dockerfile,
    Python,
    Unknown,
}

/// takes a [`Path`] and returns a [`Language`]
pub fn path_to_language(path: &Path) -> Language {
    let mut language = Language::Unknown;

    match path.file_name() {
        Some(filename) => language = reserved_filename(filename),
        None => return language,
    };

    match path.extension() {
        Some(extension) => language = extension_filename(extension),
        None => return language,
    };

    language
}

fn reserved_filename(filename: &OsStr) -> Language {
    let filename_string = filename.to_str().expect("Failed to convert to unicode.");

    match filename_string {
        "Dockerfile" => Language::Dockerfile,
        _ => Language::Unknown,
    }
}

fn extension_filename(extension: &OsStr) -> Language {
    let extension_string = extension.to_str().expect("Failed to convert to unicode.");

    match extension_string {
        "c" => Language::C,
        "py" => Language::Python,
        _ => Language::Unknown,
    }
}
