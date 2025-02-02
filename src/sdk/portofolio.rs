use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TotalBalance {
    amount_usd: f64,
    amount: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TotalRewards {
    amount_usd: f64,
    amount: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ActivatingStakes {
    amount: u64,
    amount_usd: u64,
    total: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ExitedStakes {
    amount: u64,
    amount_usd: f64,
    total: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ExitingStakes {
    amount: u64,
    amount_usd: f64,
    total: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Eigenlayer {
    restaked: f64,
    total_restaked: u64,
    // likely to be used a bool
    total_is_restakable: u64,
    total: u64,
    eigenpods: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Protocol {
    token: String,
    name: String,
    total_stakes: u64,
    total_active_stakes: u64,
    total_balance: TotalBalance,
    total_rewards: TotalRewards,
    balance_share_percent: f64,
    rewards_share_percent: f64,
    historical_grr: Option<f64>,
    activating_stakes: Option<ActivatingStakes>,
    exited_stakes: Option<ExitedStakes>,
    exiting_stakes: Option<ExitingStakes>,
    eigenlayer: Option<Eigenlayer>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Portofolio {
    total_balance_usd: f64,
    total_rewards_usd: f64,
    total_stakes: u64,
    total_active_stakes: u64,
    protocols: Vec<Protocol>,
    error: Option<String>,
    updated_at: chrono::DateTime<chrono::Utc>,
}
