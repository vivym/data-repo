use std::fmt::Display;
use std::error::Error;

use deadpool_diesel::{InteractError, PoolError};

#[derive(Debug)]
pub enum RepoError {
    Pool(PoolError),
    Interact(String),
    Diesel(diesel::result::Error),
}

pub type RepoResult<T> = Result<T, RepoError>;

impl Display for RepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pool(err) => write!(f, "Database pool error: {}", err),
            Self::Interact(err) => write!(f, "Database interact error: {}", err),
            Self::Diesel(err) => write!(f, "Diesel error: {}", err),
        }
    }
}

impl Error for RepoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Pool(err) => Some(err),
            Self::Interact(_) => None,
            Self::Diesel(err) => Some(err),
        }
    }
}

pub fn map_interact_error(err: InteractError) -> RepoError {
    match err {
        InteractError::Panic(err) => RepoError::Interact(format!("Panic: {:?}", err)),
        InteractError::Aborted => RepoError::Interact("Aborted".to_string()),
    }
}
