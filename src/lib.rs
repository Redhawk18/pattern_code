use std::{ffi::OsStr, path::Path};

/// All reserved filenames and extensions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Assembly,
    Bash,
    Batch,
    C,
    Cargo,
    Cargolock,
    CMake,
    CPP,
    CSS,
    DockerCompose,
    Dockerfile,
    DockerIgnore,
    Elixir,
    Elm,
    Erlang,
    GitIgnore,
    Go,
    H,
    Haskell,
    HPP,
    HTML,
    Java,
    JavaScript,
    Json,
    Jupyter,
    Kotlin,
    Lisp,
    Lua,
    Makefile,
    Markdown,
    Nix,
    OCaml,
    Perl,
    PHP,
    PowerShell,
    Python,
    R,
    Racket,
    ReadMe,
    Ruby,
    Rust,
    Shell,
    SQL,
    Svelte,
    SVG,
    Swift,
    Text,
    Toml,
    Typescript,
    Vue,
    XAML,
    XML,
    Yaml,
    Zig,
    Zsh,
    Unknown,
}

/// takes a [`Path`] and returns a [`Language`]
pub fn path_to_language(path: &Path) -> Language {
    let mut language = Language::Unknown;

    match path.file_name() {
        Some(filename) => language = reserved_filename(filename),
        None => return language,
    };

    // we found the file already
    if language != Language::Unknown {
        return language;
    }

    match path.extension() {
        Some(extension) => language = extension_filename(extension),
        None => return language,
    };

    language
}

fn reserved_filename(filename: &OsStr) -> Language {
    let filename_string = filename
        .to_str()
        .expect("Failed to convert to unicode.")
        .to_lowercase();

    // all names have to be lowercase
    match filename_string.as_str() {
        "cargo.toml" => Language::Cargo,
        "cargo.lock" => Language::Cargolock,
        "cmakelists.txt" => Language::CMake,
        "docker-compose.yml" => Language::DockerCompose,
        "dockerfile" => Language::Dockerfile,
        ".dockerignore" => Language::DockerIgnore,
        ".gitignore" => Language::GitIgnore,
        "makefile" => Language::Makefile,
        "README.md" => Language::ReadMe,
        _ => Language::Unknown,
    }
}

fn extension_filename(extension: &OsStr) -> Language {
    let extension_string = extension
        .to_str()
        .expect("Failed to convert to unicode.")
        .to_lowercase();

    match extension_string.as_str() {
        "asm" => Language::Assembly,
        "bash" => Language::Bash,
        "bat" | "cmd" => Language::Batch,
        "c" => Language::C,
        "c++" | "cpp" | "cxx" => Language::CPP,
        "css" => Language::CSS,
        "ex" => Language::Elixir,
        "elm" => Language::Elm,
        "erl" => Language::Erlang,
        "go" => Language::Go,
        "h" => Language::H,
        "hs" => Language::Haskell,
        "hpp" => Language::HPP,
        "html" => Language::HTML,
        "java" => Language::Java,
        "js" => Language::JavaScript,
        "json" => Language::Json,
        "ipynb" => Language::Jupyter,
        "kt" => Language::Kotlin,
        "lisp" => Language::Lisp,
        "lua" => Language::Lua,
        "nix" => Language::Nix,
        "md" => Language::Markdown,
        "ml" => Language::OCaml,
        "perl" => Language::Perl,
        "php" => Language::PHP,
        "ps1" => Language::PowerShell,
        "py" => Language::Python,
        "r" => Language::R,
        "rkt" => Language::Racket,
        "rb" => Language::Ruby,
        "rs" => Language::Rust,
        "sh" => Language::Shell,
        "sql" => Language::SQL,
        "svelte" => Language::Svelte,
        "svg" => Language::SVG,
        "swift" => Language::Swift,
        "txt" => Language::Text,
        "toml" => Language::Toml,
        "ts" => Language::Typescript,
        "vue" => Language::Vue,
        "xaml" => Language::XAML,
        "xml" => Language::XML,
        "yaml" | "yml" => Language::Yaml,
        "zig" => Language::Zig,
        "zsh" => Language::Zsh,
        _ => Language::Unknown,
    }
}
