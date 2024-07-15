use std::ops::Deref;

use probe_rs::probe::DebugProbeSelector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(into = "String", try_from = "String")]
pub struct DebugProbeSelectorShim(pub DebugProbeSelector);

impl Deref for DebugProbeSelectorShim {
    type Target = DebugProbeSelector;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<String> for DebugProbeSelectorShim {
    type Error = <DebugProbeSelector as TryFrom<String>>::Error;
    
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(value.parse()?))
    }
}

impl From<DebugProbeSelectorShim> for String {
    fn from(value: DebugProbeSelectorShim) -> Self {
        value.0.to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub name: String,
    pub chip: String,
    pub probe: DebugProbeSelectorShim,
    pub connect_under_reset: bool,
    pub speed: Option<u32>,
    pub up: bool,
    pub power_reset: bool,
    pub cycle_delay_seconds: f64,
    pub max_settle_time_millis: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetList {
    pub targets: Vec<Target>,
}
