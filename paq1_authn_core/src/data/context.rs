use chrono::{Local, NaiveDateTime};

#[derive(Debug, Clone)]
pub struct ContextCore {
    pub subject: String,
    pub at: NaiveDateTime,
    pub token: Option<String>,
}

impl ContextCore {
    pub fn new(subject: String, at: NaiveDateTime, token: Option<String>) -> Self {
        Self { subject, at, token }
    }

    pub fn of(subject: String, at: NaiveDateTime) -> Self {
        Self {
            subject,
            at,
            token: None,
        }
    }

    pub fn from_now(subject: &str, token: Option<String>) -> Self {
        Self {
            subject: subject.to_string(),
            at: Local::now().naive_utc(),
            token,
        }
    }

    pub fn get_subject(&self) -> &str {
        self.subject.as_str()
    }

    pub fn get_time(&self) -> &NaiveDateTime {
        &self.at
    }
}
