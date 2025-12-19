use std::path::PathBuf;

#[must_use]
pub fn project_root_path() -> PathBuf {
    let dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());

    PathBuf::from(dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned()
}

#[must_use]
pub fn tombi_schema_path() -> PathBuf {
    project_root_path()
        .join(tombi_uri::schemastore_hostname!())
        .join("tombi.json")
}

#[must_use]
pub fn cargo_schema_path() -> PathBuf {
    project_root_path()
        .join(tombi_uri::schemastore_hostname!())
        .join("cargo.json")
}

#[must_use]
pub fn pyproject_schema_path() -> PathBuf {
    project_root_path()
        .join(tombi_uri::schemastore_hostname!())
        .join("pyproject.json")
}

#[must_use]
pub fn type_test_schema_path() -> PathBuf {
    project_root_path()
        .join("schemas")
        .join("type-test.schema.json")
}

#[must_use]
pub fn untagged_union_schema_path() -> PathBuf {
    project_root_path()
        .join("schemas")
        .join("untagged-union.schema.json")
}
