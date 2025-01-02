use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
}

impl Version {
    pub fn parse(version: &str) -> Result<Version, VersionParseError> {
        if version.is_empty() {
            return Err(VersionParseError::EmptyVersion);
        }
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() < 2 {
            return Err(VersionParseError::InvalidFormat(version.to_string()));
        }

        let major = parts[0]
            .parse::<u32>()
            .map_err(|_| VersionParseError::InvalidFormat(version.to_string()))?;
        let minor = parts[1]
            .parse::<u32>()
            .map_err(|_| VersionParseError::InvalidFormat(version.to_string()))?;

        Ok(Version { major, minor })
    }
}

#[derive(Debug)]
pub enum VersionParseError {
    EmptyVersion,
    InvalidFormat(String),
}

impl fmt::Display for VersionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionParseError::EmptyVersion => write!(f, "Version is empty"),
            VersionParseError::InvalidFormat(version) => {
                write!(
                    f,
                    "Unable to parse version, expected format: x.y[.z], found: {}",
                    version
                )
            }
        }
    }
}

impl Error for VersionParseError {}

pub fn is_compatible(client_version: Option<&str>, server_version: Option<&str>) -> bool {
    if client_version.is_none() || server_version.is_none() {
        println!(
            "Unable to compare versions, client_version: {:?}, server_version: {:?}",
            client_version, server_version
        );
        return false;
    }

    let client_version = client_version.unwrap();
    let server_version = server_version.unwrap();

    if client_version == server_version {
        return true;
    }

    match (
        Version::parse(client_version),
        Version::parse(server_version),
    ) {
        (Ok(client), Ok(server)) => {
            let major_dif = (client.major as i32 - server.major as i32).abs();
            if major_dif >= 1 {
                return false;
            }
            (client.minor as i32 - server.minor as i32).abs() <= 1
        }
        (Err(e), _) | (_, Err(e)) => {
            println!("Unable to compare versions: {}", e);
            false
        }
    }
}
