use crate::project::{Project, Session};

pub fn start(project: &mut Project) -> Result<(), Box<dyn std::error::Error>> {
    if last_session_is_not_finished(project) {
        return Err(
            "there is already an active session, finish it before starting a new one using 'clok stop'".into(),
        );
    }
    let session = Session::new();
    match &mut project.sessions {
        Some(sessions) => {
            sessions.push(session);
        }
        None => {
            project.sessions = Some(vec![session]);
        }
    }
    project.save()?;
    Ok(())
}

pub fn last_session_is_not_finished(project: &Project) -> bool {
    match &project.sessions {
        Some(sessions) => match sessions.last() {
            Some(session) => session.finished_at.is_none(),
            None => false,
        },
        None => false,
    }
}
