use crate::project::Project;
use chrono::Duration;

pub fn info(project: &Project) -> Result<String, Box<dyn std::error::Error>> {
    let total_duration = calculate_total_duration(project);

    let hours = total_duration.num_hours();
    let minutes = total_duration.num_minutes() % 60;
    let seconds = total_duration.num_seconds() % 60;

    let mut result = String::from("you spent ");

    if hours > 0 {
        result.push_str(&format!(
            "{} hour{} ",
            hours,
            if hours == 1 { "" } else { "s" }
        ));
    }
    if minutes > 0 || hours > 0 {
        result.push_str(&format!(
            "{} minute{} ",
            minutes,
            if minutes == 1 { "" } else { "s" }
        ));
    }
    result.push_str(&format!(
        "{} second{}",
        seconds,
        if seconds == 1 { "" } else { "s" }
    ));

    result.push_str(&format!(" on project '{}'", project.title()));

    Ok(result)
}

fn calculate_total_duration(project: &Project) -> Duration {
    let mut total = Duration::zero();

    if let Some(sessions) = &project.sessions {
        for session in sessions {
            if let Some(finished_at) = session.finished_at {
                let duration = finished_at - session.started_at;
                total += duration;
            }
        }
    }

    total
}
