use std::path::PathBuf;

use cargo_metadata::MetadataCommand;

pub fn get_cargo_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let metadata = MetadataCommand::new().no_deps().exec()?;
    Ok(metadata.workspace_root.into())
}

pub fn get_project_name() -> Result<String, Box<dyn std::error::Error>> {
    let metadata = MetadataCommand::new().no_deps().exec()?;
    let root_package = metadata.root_package().ok_or("No root package found")?;
    Ok(root_package.name.to_string())
}
