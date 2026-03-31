use std::{fmt::Debug, str::FromStr};

use serde::{Deserialize, de::Visitor};

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum H256Error {
    #[error("Invlaid length: expected 32 bytes")]
    InvalidLength,
    #[error("Invalid hex")]
    InvalidHex,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct H256([u8; 32]);

impl H256 {
    pub const ZERO:Self = Self([0u8; 32]);

    pub fn zero() -> Self {
        Self::ZERO
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, H256Error> {
        if slice.len() != 32 {
            return Err(H256Error::InvalidLength);
        }
        let mut bytes_slice = [0u8; 32];
        bytes_slice.copy_from_slice(slice);
        Ok(Self(bytes_slice))
    }
}

impl From<[u8; 32]> for H256 {
    fn from(value: [u8; 32]) -> Self {
        H256(value)
    }
}

impl AsRef<[u8]> for H256 {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl std::fmt::Display for H256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self))
    }
}

impl Debug for H256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "H256({self})")
    }
}

impl FromStr for H256 {
    type Err = H256Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        let value = hex::decode(s).map_err(|_| H256Error::InvalidHex)?;
        H256::from_slice(&value)
    }
}

struct H256Visitor;

impl<'de> Visitor<'de> for H256Visitor {
    type Value = H256;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A hex value")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.visit_str(v.as_str())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        let hash_string = H256::from_str(v);
        match hash_string {
            Ok(v) => Ok(v),
            Err(e) => match e {
                H256Error::InvalidHex => Err(E::custom(format!("Invlaid length: expected 32 bytes"))),
                H256Error::InvalidLength => Err(E::custom(format!("Invalid hex")))
            }
        }
    }
}

impl<'de> Deserialize<'de> for H256 {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        deserializer.deserialize_string(H256Visitor)
    }
}
