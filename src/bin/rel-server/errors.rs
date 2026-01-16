use std::error;
use std::fmt;

use gitlab::GitlabError;

#[derive(Debug)]
pub enum RelError {
    MissingGitlabToken,
    MissingDatabaseUrl,
    GitlabError(GitlabError),
    DbPoolError(r2d2::Error),
}

pub type Result<T> = std::result::Result<T, RelError>;

impl fmt::Display for RelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RelError::MissingGitlabToken => write!(f, "GITLAB_ACCESS_TOKEN not set"),
            RelError::MissingDatabaseUrl => write!(f, "DATABASE_URL not set"),
            RelError::GitlabError(ref e) => write!(f, "An error occured accessing Gitlab: {}", e),
            RelError::DbPoolError(ref e) => write!(f, "Unable to connect to database: {}", e),
        }
    }
}

impl From<GitlabError> for RelError {
    fn from(err: GitlabError) -> RelError {
        RelError::GitlabError(err)
    }
}

impl error::Error for RelError {}
