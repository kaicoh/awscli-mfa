use crate::Result;

use anyhow::anyhow;
use aws_sdk_sts::{output::GetSessionTokenOutput, Client};
use chrono::prelude::*;

#[derive(Debug, Default)]
pub struct GetSessionToken {
    profile: Option<String>,
    duration_seconds: Option<i32>,
    serial_number: Option<String>,
    token_code: Option<String>,
}

impl GetSessionToken {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_profile(self, profile: Option<String>) -> Self {
        Self { profile, ..self }
    }

    pub fn set_duration_seconds(self, duration_seconds: Option<i32>) -> Self {
        Self {
            duration_seconds,
            ..self
        }
    }

    pub fn set_serial_number(self, serial_number: Option<String>) -> Self {
        Self {
            serial_number,
            ..self
        }
    }

    pub fn set_token_code(self, token_code: Option<String>) -> Self {
        Self { token_code, ..self }
    }

    pub async fn send(self) -> Result<StsCredential> {
        let Self {
            profile,
            duration_seconds,
            serial_number,
            token_code,
        } = self;

        let config = match profile {
            Some(profile) => aws_config::from_env().profile_name(profile).load().await,
            None => aws_config::load_from_env().await,
        };

        let output = Client::new(&config)
            .get_session_token()
            .set_duration_seconds(duration_seconds)
            .set_serial_number(serial_number)
            .set_token_code(token_code)
            .send()
            .await
            .map_err(anyhow::Error::new)?;

        StsCredential::try_from(output)
    }
}

#[derive(Debug, Default)]
pub struct StsCredential {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
    expiration: Option<NaiveDateTime>,
}

impl StsCredential {
    pub fn expiration(&self) -> String {
        self.expiration
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or("Unknown".to_string())
    }
}

impl TryFrom<GetSessionTokenOutput> for StsCredential {
    type Error = anyhow::Error;

    fn try_from(output: GetSessionTokenOutput) -> Result<Self> {
        let cred = output
            .credentials()
            .ok_or(anyhow!("Failed to get credentials from {:#?}", output))?;

        Ok(Self {
            access_key_id: cred.access_key_id().map(String::from).unwrap_or_default(),
            secret_access_key: cred
                .secret_access_key()
                .map(String::from)
                .unwrap_or_default(),
            session_token: cred.session_token().map(String::from).unwrap_or_default(),
            expiration: cred
                .expiration()
                .and_then(|exp| NaiveDateTime::from_timestamp_opt(exp.secs(), exp.subsec_nanos())),
        })
    }
}
