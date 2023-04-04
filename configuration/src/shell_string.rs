use std::{env::VarError, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use shellexpand::LookupError;

// ShellString is a wrapper around String that expands shell variables upon deserialization or from initialization.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ShellString(String);

impl ShellString {
    /// Create a new ShellString. This method does not expand shell variables.
    ///
    /// use [`Self::new()`] to expand shell variables.
    pub fn raw<S: AsRef<str>>(s: S) -> Self {
        Self(s.as_ref().into())
    }

    /// Create a new ShellString. This method will expand shell variables.
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, LookupError<VarError>> {
        s.as_ref().parse()
    }
}

impl<'de> Deserialize<'de> for ShellString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = shellexpand::full(&s).map_err(serde::de::Error::custom)?;
        Ok(Self(s.into()))
    }
}

impl Serialize for ShellString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl FromStr for ShellString {
    type Err = LookupError<VarError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = shellexpand::full(s)?;
        Ok(Self(s.into()))
    }
}

impl AsRef<str> for ShellString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<ShellString> for String {
    fn from(value: ShellString) -> Self {
        value.0
    }
}

impl Display for ShellString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
