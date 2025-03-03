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
    Env,
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

impl Language {
    pub fn extension_as_str(&self) -> &str {
        match self {
            Language::Assembly => "asm",
            Language::Shell | Language::Zsh | Language::Bash => "bash",
            Language::Batch => "bat",
            Language::H | Language::C => "c",
            Language::Cargolock => "lock",
            Language::CMake => "cmake",
            Language::HPP | Language::CPP => "cpp",
            Language::CSS => "css",
            Language::DockerCompose => "yaml",
            Language::Dockerfile | Language::DockerIgnore => "dockerfile",
            Language::Elixir => "elixir",
            Language::Elm => "elm",
            Language::Env => "dotenv",
            Language::Erlang => "erlang",
            Language::GitIgnore => "git",
            Language::Go => "go",
            Language::Haskell => "haskell",
            Language::HTML => "html",
            Language::Java => "java",
            Language::JavaScript => "javascript",
            Language::Json => "json",
            Language::Jupyter => "jupyter",
            Language::Kotlin => "kotlin",
            Language::Lisp => "lisp",
            Language::Lua => "lua",
            Language::Makefile => "makefile",
            Language::Markdown | Language::ReadMe => "markdown",
            Language::Nix => "nix",
            Language::OCaml => "ocaml",
            Language::Perl => "perl",
            Language::PHP => "php",
            Language::PowerShell => "powershell",
            Language::Python => "python",
            Language::R => "r",
            Language::Racket => "racket",
            Language::Ruby => "ruby",
            Language::Rust => "rust",
            Language::SQL => "sql",
            Language::Svelte => "svelte",
            Language::SVG => "svg",
            Language::Swift => "swift",
            Language::Text => "txt",
            Language::Cargo | Language::Toml => "toml",
            Language::Typescript => "typescript",
            Language::Vue => "vue",
            Language::XAML | Language::XML => "xml",
            Language::Yaml => "yaml",
            Language::Zig => "zig",
            Language::Unknown => "unknown",
        }
    }
}

impl From<&Path> for Language {
    fn from(value: &Path) -> Self {
        let mut language = Language::Unknown;

        match value.file_name() {
            Some(filename) => language = reserved_filename(filename),
            None => return language,
        };

        // we found the file already
        if language != Language::Unknown {
            return language;
        }

        match value.extension() {
            Some(extension) => language = extension_filename(extension),
            None => return language,
        };

        language
    }
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
        "env" => Language::Env,
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
