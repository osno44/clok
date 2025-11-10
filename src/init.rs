use crate::{
    metadata::get_project_name,
    project::{Project, is_already_initialized},
};

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let project_title = get_project_name()?;
    if is_already_initialized()? {
        return Err(format!(
            "project '{}' is already initialized for clok",
            project_title
        )
        .into());
    }
    let project = Project::new(project_title);
    project.save()?;
    Ok(())
}
