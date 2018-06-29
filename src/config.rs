use serde_yaml as yaml;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::io::{Error as IoError, Read};
use std::path::Path;


#[derive(Debug)]
pub enum ConfigError {
    Io(IoError),
    Parse(yaml::Error),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            ConfigError::Io(ref err) => write!(f, "IO Error: {}", err),
            ConfigError::Parse(ref err) => write!(f, "Parse Error: {}", err),
        }
    }
}

impl StdError for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::Io(ref err) => err.description(),
            ConfigError::Parse(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            ConfigError::Io(ref err) => Some(err),
            ConfigError::Parse(ref err) => Some(err),
        }
    }
}

impl From<IoError> for ConfigError {
    fn from(err: IoError) -> Self {
        ConfigError::Io(err)
    }
}

impl From<yaml::Error> for ConfigError {
    fn from(err: yaml::Error) -> Self {
        ConfigError::Parse(err)
    }
}


#[derive(Clone, Deserialize, Debug)]
pub struct Pin {
    pub name: String,

    #[serde(default)]
    pub inverted: bool,
    pub debounce: u64,

    pub script: String,
}


#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub pins: HashMap<u64, Pin>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        return Ok(yaml::from_str(&data)?);
    }
}
