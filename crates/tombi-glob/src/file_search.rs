use std::path::PathBuf;
use tombi_config::{Config, ConfigLevel, FilesOptions};

use crate::WalkDir;

/// Input source for TOML files.
///
/// Standard input or file paths. Contains a list of files that match the glob pattern.
#[derive(Debug)]
pub enum FileSearch {
    Stdin,
    Files(Vec<Result<PathBuf, crate::Error>>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileInputType {
    Stdin,
    Project,
    Files,
}

impl<T: AsRef<str>> From<&[T]> for FileInputType {
    fn from(files: &[T]) -> Self {
        match files.len() {
            0 => Self::Project,
            1 if files[0].as_ref() == "-" => Self::Stdin,
            _ => Self::Files,
        }
    }
}

impl FileSearch {
    pub async fn new<T: AsRef<str>>(
        files: &[T],
        config: &Config,
        config_path: Option<&std::path::Path>,
        config_level: ConfigLevel,
    ) -> Self {
        let root = match config_level {
            ConfigLevel::Project => config_path.and_then(|p| p.parent()).unwrap_or(".".as_ref()),
            _ => ".".as_ref(),
        };
        let files_options = config.files.clone().unwrap_or_default();

        match FileInputType::from(files) {
            FileInputType::Stdin => Self::Stdin,
            FileInputType::Project => {
                tracing::debug!("Searching for TOML files using configured patterns...");

                Self::Files(search_pattern_matched_paths(root, files_options).await)
            }
            FileInputType::Files => {
                tracing::debug!("Searching for TOML files using user input patterns...");

                let mut matched_paths = Vec::with_capacity(100);

                for file_input in files {
                    let file_path = file_input.as_ref();

                    if is_glob_pattern(file_path) {
                        matched_paths.extend(
                            search_pattern_matched_paths(
                                root,
                                FilesOptions {
                                    include: Some(vec![file_path.to_string()]),
                                    exclude: None,
                                },
                            )
                            .await,
                        );
                    } else {
                        let path = PathBuf::from(file_path);
                        if path.is_file() {
                            matched_paths.push(Ok(path));
                        } else if path.is_dir() {
                            matched_paths.extend(
                                search_pattern_matched_paths(path, files_options.clone()).await,
                            );
                        } else {
                            matched_paths.push(Err(crate::Error::FileNotFound(path)));
                        }
                    }
                }

                Self::Files(matched_paths)
            }
        }
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        match self {
            Self::Stdin => 1,
            Self::Files(files) => files.len(),
        }
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        match self {
            Self::Stdin => false,
            Self::Files(files) => files.is_empty(),
        }
    }
}

pub async fn search_pattern_matched_paths<P: AsRef<std::path::Path>>(
    root: P,
    files_options: FilesOptions,
) -> Vec<Result<PathBuf, crate::Error>> {
    tracing::debug!("Include patterns: {:?}", files_options.include);
    tracing::debug!("Exclude patterns: {:?}", files_options.exclude);

    match WalkDir::new_with_options(root, files_options).walk().await {
        Ok(results) => {
            let matched_paths: Vec<Result<PathBuf, crate::Error>> =
                results.into_iter().map(Ok).collect();
            matched_paths
        }
        Err(err) => {
            vec![Err(err)]
        }
    }
}

fn is_glob_pattern(path_str: &str) -> bool {
    for c in path_str.chars() {
        if matches!(c, '*' | '?' | '[' | ']') {
            return true;
        }
    }
    false
}
