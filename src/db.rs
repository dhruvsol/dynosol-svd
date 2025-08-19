use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ValidatorCategory {
    Base,
    SocialImpact,
    ValidatorTooling,
    Growth,
}

impl ValidatorCategory {
    fn stake(&self) -> u64 {
        match self {
            ValidatorCategory::Base => 2500u64.saturating_mul(1000_000_000u64), // 2500 SOL
            ValidatorCategory::SocialImpact => 1250u64.saturating_mul(1000_000_000u64), // 1250 SOL
            ValidatorCategory::ValidatorTooling => 1250u64.saturating_mul(1000_000_000u64), // 1250 SOL
            ValidatorCategory::Growth => 2500u64.saturating_mul(1000_000_000u64), // 2500 SOL
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ValidatorList {
    pub name: String,
    pub logo: String,
    pub vote_account: String,
    pub identity: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ValidatorInfo {
    pub vote_account: String,
    pub category: Vec<ValidatorCategory>,
    pub pool_stake: u64,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ValidatorEpochData {
    pub vote_account: String,
    pub total_stake: u64,
    pub epoch: u64,
}
