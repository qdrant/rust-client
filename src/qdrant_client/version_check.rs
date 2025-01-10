use std::error::Error;
use std::fmt;
use semver::Version;

pub fn parse(version: &str) -> Result<Version, VersionParseError> {
    if version.is_empty() {
        return Err(VersionParseError::EmptyVersion);
    }
    match Version::parse(version) {
        Ok(v) => Ok(v),
        Err(_) => Err(VersionParseError::InvalidFormat(version.to_string())),
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
        parse(client_version),
        parse(server_version),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_compatible() {
        let test_cases = vec![
            (Some("1.9.3.dev0"), Some("2.8.1-dev12"), false),
            (Some("1.9"), Some("2.8"), false),
            (Some("1"), Some("2"), false),
            (Some("1.9.0"), Some("2.9.0"), false),
            (Some("1.1.0"), Some("1.2.9"), true),
            (Some("1.2.7"), Some("1.1.8-dev0"), true),
            (Some("1.2.1"), Some("1.2.29"), true),
            (Some("1.2.0"), Some("1.2.0"), true),
            (Some("1.2.0"), Some("1.4.0"), false),
            (Some("1.4.0"), Some("1.2.0"), false),
            (Some("1.9.0"), Some("3.7.0"), false),
            (Some("3.0.0"), Some("1.0.0"), false),
            (None, Some("1.0.0"), false),
            (Some("1.0.0"), None, false),
            (None, None, false),
        ];

        for (client_version, server_version, expected_result) in test_cases {
            let result = is_compatible(client_version, server_version);
            assert_eq!(
                result, expected_result,
                "Failed for client: {:?}, server: {:?}",
                client_version, server_version
            );
        }
    }

    #[test]
    fn test_version_parse_errors() {
        let test_cases = vec![
            ("1", VersionParseError::InvalidFormat("1".to_string())),
            ("1.", VersionParseError::InvalidFormat("1.".to_string())),
            (".1", VersionParseError::InvalidFormat(".1".to_string())),
            (".1.", VersionParseError::InvalidFormat(".1.".to_string())),
            (
                "1.a.1",
                VersionParseError::InvalidFormat("1.a.1".to_string()),
            ),
            (
                "a.1.1",
                VersionParseError::InvalidFormat("a.1.1".to_string()),
            ),
            ("", VersionParseError::EmptyVersion),
        ];

        for (input, expected_error) in test_cases {
            let result = parse(input);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
        }
    }
}
