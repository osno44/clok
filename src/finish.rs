use crate::{project::Project, start::last_session_is_not_finished};
use chrono::Utc;

pub fn finish(project: &mut Project) -> Result<(), Box<dyn std::error::Error>> {
    if !last_session_is_not_finished(project) {
        return Err("no active session to finish, run 'clok start' first".into());
    }
    if let Some(sessions) = &mut project.sessions
        && let Some(last_session) = sessions.last_mut()
    {
        last_session.finished_at = Some(Utc::now());
    }
    project.save()?;
    Ok(())
}
