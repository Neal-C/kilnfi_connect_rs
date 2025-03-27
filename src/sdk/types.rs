use serde::{Deserialize, Deserializer, Serialize};
use strum_macros::{AsRefStr, EnumString};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ActivatingStakes {
    pub amount: u64,
    pub amount_usd: u64,
    pub total: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Any {
    #[serde(rename = "camelCase")]
    pub type_url: String,
    // Uint8Array
    pub value: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct BroadcastTxRequest {
    pub tx_serialized: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct BroadcastTxResponse {
    pub tx_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct BitArray {
    pub extra_bits_stored: u64,
    // Uint8Array
    pub elems: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
#[repr(u64)]
pub enum ChainID {
    ArbitrumOne = 42161,
    AvalanceCChain = 43114,
    BNBSmartChainMainnet = 56,
    CoinBase = 8453,
    EthereumMainnet = 1,
    PolygonMainnet = 137,
    Sepolia = 11155111,
    Holesky = 17000,
    OptimismMainnet = 10,
}

#[derive(Serialize, Deserialize, Debug, EnumString)]
pub enum Chain {
    #[strum(serialize = "arb")]
    Arb,
    #[strum(serialize = "eth")]
    Eth,
    #[strum(serialize = "bsc")]
    Bsc,
    #[strum(serialize = "matic")]
    Matic,
    #[strum(serialize = "base")]
    Base,
    #[strum(serialize = "op")]
    Op,
    #[strum(serialize = "cosmoshub-4")]
    CosmosHub4,
}

#[derive(Serialize, Deserialize, AsRefStr, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ChainStakeState {
    Active,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChainStakes {
    pub wallet_addresses: Vec<String>,
    pub stake_address: String,
    pub pool_id: String,
    pub balance: String,
    pub rewards: String,
    pub available_rewards: String,
    pub delegated_epoch: u64,
    pub delegated_at: chrono::DateTime<chrono::Utc>,
    pub activated_epoch: u64,
    pub activated_at: chrono::DateTime<chrono::Utc>,
    pub state: ChainStakeState,
    pub net_apy: f64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CreateStakeRequest {
    #[serde(rename = "camelCase")]
    pub stake_address: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Eigenlayer {
    pub restaked: f64,
    pub total_restaked: u64,
    // likely to be used a bool
    pub total_is_restakable: u64,
    pub total: u64,
    pub eigenpods: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ExitedStakes {
    pub amount: u64,
    pub amount_usd: f64,
    pub total: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ExitingStakes {
    pub amount: u64,
    pub amount_usd: f64,
    pub total: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EventAttribute {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct GetStakesRequest {
    pub validators: Vec<String>,
    pub delegators: Vec<String>,
    pub accounts: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct GetBalanceRequest {
    pub address: String,
    pub denom: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct GetStakesResponse {
    pub validator_address: String,
    pub delegator_address: String,
    pub delegated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub delegated_block: Option<u64>,
    pub undelegated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub undelegated_block: Option<u64>,
    pub rewards: String,
    pub available_rewards: String,
    pub balance: String,
    pub net_apy: f64,
    pub state: StakeState,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub permissions: Vec<StakesResponsePermission>,
    pub unbondings: Vec<StakesResponseUnbonding>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ModeInfo {
    /** single represents a single signer */
    Single { mode: SignMode },
    /** multi represents a nested multisig signer */
    Multi {
        bitarray: Option<BitArray>,
        mode_infos: Vec<ModeInfo>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NetworkStats {
    pub asset: String,
    pub asset_icon: String,
    pub asset_symbol: String,
    pub asset_decimals: String,
    pub assets_price_usd: String,
    pub share_symbol: String,
    pub tvl: String,
    pub protocol: Protocol,
    pub protocol_display_name: String,
    pub protocol_icon: String,
    pub protocol_tvl: String,
    pub protocol_supply_limit: String,
    // Gross Reward Rate
    pub grr: u64,
    // Net Reward Rate
    pub nrr: u64,
    pub vault: String,
    pub chain: String,
    pub chain_id: ChainID,
    pub updated_at_block: u64,
}

#[derive(Deserialize, Serialize, AsRefStr, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Operation {
    Deposit,
    Withdrawal,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Operations {
    // Address
    pub owner: String,
    pub r#type: Operation,
    pub assets: String,
    pub shares: String,
    pub sender: String,
    // Operation timestamp (RFC3339 format)
    pub timestamp: String,
    pub tx_hash: String,
    pub vault: String,
    pub chain: String,
}

#[derive(Serialize, Deserialize, AsRefStr, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum OperationType {
    StakeRegisteration,
    StakeDeregisteration,
    Delegation,
    Reward,
    Withdrawal,
}

#[derive(Serialize, Deserialize, AsRefStr, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum OperationsResponse {
    StakeRegisteration {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        stake_address: String,
        epoch: u64,
        block: u64,
        tx_hash: String,
    },
    StakeDeregisteration {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        stake_address: String,
        epoch: u64,
        block: u64,
        tx_hash: String,
    },
    Delegation {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        stake_address: String,
        epoch: u64,
        block: u64,
        tx_hash: String,
        pool_id: String,
    },
    Reward {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        stake_address: String,
        epoch: u64,
        block: u64,
        pool_id: String,
        amount: String,
    },
    Withdrawal {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        stake_address: String,
        epoch: u64,
        block: u64,
        pool_id: String,
        amount: String,
    },
}

fn deserialize_type<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let the_type = OperationsResponse::deserialize(deserializer)?;

    let the_type = match the_type {
        OperationsResponse::StakeRegisteration { .. } => "stake_registration".into(),
        OperationsResponse::StakeDeregisteration { .. } => "stake_deregistration".into(),
        OperationsResponse::Delegation { .. } => "delegation".into(),
        OperationsResponse::Reward { .. } => "reward".into(),
        OperationsResponse::Withdrawal { .. } => "withdrawal".into(),
    };

    Ok(the_type)
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PortofolioProtocol {
    pub token: String,
    pub name: String,
    pub total_stakes: u64,
    pub total_active_stakes: u64,
    pub total_balance: TotalBalance,
    pub total_rewards: TotalRewards,
    pub balance_share_percent: f64,
    pub rewards_share_percent: f64,
    pub historical_grr: Option<f64>,
    pub activating_stakes: Option<ActivatingStakes>,
    pub exited_stakes: Option<ExitedStakes>,
    pub exiting_stakes: Option<ExitingStakes>,
    pub eigenlayer: Option<Eigenlayer>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Portofolio {
    pub total_balance_usd: f64,
    pub total_rewards_usd: f64,
    pub total_stakes: u64,
    pub total_active_stakes: u64,
    pub protocols: Vec<Protocol>,
    pub error: Option<String>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Serialize, AsRefStr, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Protocol {
    AaveV3,
    Venus,
    CompoundV3,
    Morpho,
    Sdai,
    Ethereum,
    Solana,
    Near,
    Tezos,
    Cardano,
    Cosmos,
    Matic,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PostStakesResponse {
    pub id: Uuid,
    pub tags: Vec<String>,
    // metadata: HashMap<String, String>,
    pub metadata: serde_json::Value,
    pub protocol: Protocol,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PrepareTxRequest {
    // Pubkey
    pub pubkey: String,
    pub tx_body: String,
    pub tx_auth_info: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PrepareTxResponse {
    pub signed_tx_serialized: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PostStakesRequest {
    pub stakes: Vec<StakeRequestStake>,
    pub account_id: Uuid,
}

#[derive(Serialize, Deserialize, AsRefStr, Default, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ResponseFormat {
    #[default]
    Daily,
    Epoch,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Reward {
    pub date: chrono::DateTime<chrono::Utc>,
    pub rewards: String,
    pub active_balance: String,
    pub net_apy: f64,
    pub rewards_usd: String,
    pub stake_balance_usd: String,
    pub rewards_balance_usd: Option<u64>,
    pub active_balance_usd: Option<u64>,
}

#[derive(Serialize, Deserialize, AsRefStr, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RewardRequest {
    Daily {
        stakes_addresses: Vec<String>,
        wallets: Vec<String>,
        pool_ids: Vec<String>,
        accounts: Vec<Uuid>,
        format: ResponseFormat,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        include_usd: bool,
    },
    Epoch {
        stakes_addresses: Vec<String>,
        wallets: Vec<String>,
        pool_ids: Vec<String>,
        accounts: Vec<Uuid>,
        format: ResponseFormat,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        start_epoch: u64,
        end_epoch: u64,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ReportsRequest {
    pub delegators: Vec<String>,
    pub validators: Vec<String>,
    pub accounts: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct RestakeRewardsTxRequest {
    // Pubkey
    pub pubkey: String,
    // Address
    pub validator: String,
    // Address
    pub grantee_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StakeMessageValue {
    pub validator_address: String,
    pub delegator_address: String,
    pub amount: TxStakeCoin,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StakeMessageRestake {
    pub address: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SignerInfoPublicKey {
    pub type_url: String,
    // Uint8Array
    pub value: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponseReceipt {
    pub height: u64,
    pub tx_index: u64,
    pub hash: String,
    pub code: u64,
    pub events: Vec<StatusResponseEvent>,
    pub raw_log: String,
    pub tx: Vec<u8>,
    pub gas_used: u64,
    pub gas_wanted: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StatusResponseEvent {
    pub r#type: String,
    pub attributes: Vec<EventAttribute>,
}

#[derive(Serialize, Deserialize, EnumString, AsRefStr, Debug)]
pub enum StakeState {
    Active,
    Activating,
    Inactive,
    Deactivating,
    Withdrawn,
}

#[derive(Serialize, Deserialize, EnumString, Debug, Default)]
pub enum StakePermissionKind {
    #[default]
    #[strum(serialize = "Staking.MsgDelegate")]
    Delegate,
    #[strum(serialize = "Staking.MsgUndelegate")]
    Undelegate,
    #[strum(serialize = "Staking.Redelegate")]
    Redelegate,
    #[strum(serialize = "Distribution.MsgWithdrawDelegatorReward")]
    WithdrawDelegatorReward,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StakesResponsePermission {
    pub source: String,
    pub creation_height: u64,
    pub permission: StakePermissionKind,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub allow_list: Option<Vec<String>>,
    pub deny_list: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StakesResponseUnbonding {
    pub creation_height: u64,
    pub completion_time: chrono::DateTime<chrono::Utc>,
    pub balance: String,
    pub initial_balance: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StakeRequestStake {
    pub stake_id: String,
}

#[derive(Serialize, Deserialize, EnumString, Debug)]
pub enum StakeStatus {
    #[strum(serialize = "active")]
    Active,
    #[strum(serialize = "pending")]
    Pending,
    #[strum(serialize = "disabled")]
    Disabled,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StakeResponse {
    pub id: Uuid,
    pub tags: Vec<String>,
    pub metadata: serde_json::Value,
    pub protocol: Protocol,
    pub chain_id: ChainID,
    pub chain: Chain,
    pub address: String,
    pub status: StakeStatus,
}

#[derive(Serialize, Deserialize, EnumString, Debug)]
pub enum StakingOperation {
    #[strum(serialize = "Staking.MsgDelegate", serialize = "staking.MsgDelegate")]
    Delegate {
        #[serde(deserialize_with = "deserialize_operation_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        block: u64,
        tx_hash: String,
        tx_gas_used: String,
        message_index: u64,
        validator_address: String,
        delegator_address: String,
        amount: String,
        withdraw_rewards: String,
    },
    #[strum(
        serialize = "Staking.MsgUndelegate",
        serialize = "staking.MsgUndelegate"
    )]
    Undelegate {
        #[serde(deserialize_with = "deserialize_operation_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        block: u64,
        tx_hash: String,
        tx_gas_used: String,
        message_index: u64,
        validator_address: String,
        delegator_address: String,
        amount: String,
        withdraw_rewards: String,
    },
    #[strum(
        serialize = "Staking.MsgBeginRedelegate",
        serialize = "staking.MsgBeginRedelegate"
    )]
    BeginRedelegate {
        #[serde(deserialize_with = "deserialize_operation_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        block: u64,
        tx_hash: String,
        tx_gas_used: String,
        message_index: u64,
        validator_address: String,
        validator_address_source: String,
        delegator_address: String,
        amount: String,
        withdraw_rewards: String,
        withdraw_rewards_source: String,
    },
    #[strum(
        serialize = "Distribution.MsgWithdrawDelegatorReward",
        serialize = "distribution.MsgWithdrawDelegatorReward"
    )]
    WithdrawDelegatorReward {
        #[serde(deserialize_with = "deserialize_operation_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        block: u64,
        tx_hash: String,
        tx_gas_used: String,
        message_index: u64,
        validator_address: String,
        delegator_address: String,
        withdraw_rewards: String,
    },
    Grant {
        #[serde(deserialize_with = "deserialize_operation_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        block: u64,
        tx_hash: String,
        tx_gas_used: String,
        message_index: u64,
        grantee: String,
        granter: String,
        permission: StakePermissionKind,
        allow_list: Option<Vec<String>>,
        denly_list: Option<Vec<String>>,
    },
    Exec {
        #[serde(deserialize_with = "deserialize_operation_type")]
        r#type: String,
        validator_address: String,
        validator_address_source: String,
        delegator_address: String,
        amount: String,
        withdraw_rewards: String,
        withdraw_rewards_source: String,
        executed_operations: Option<Vec<Operation>>,
    },
}

fn deserialize_operation_type<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let the_type = StakingOperation::deserialize(deserializer)?;

    let the_type = match the_type {
        StakingOperation::Delegate { .. } => "staking.MsgDelegate".into(),
        StakingOperation::Undelegate { .. } => "staking.MsgUndelegate".into(),
        StakingOperation::WithdrawDelegatorReward { .. } => {
            "distr.MsgWithdrawDelegatorReward".into()
        }
        StakingOperation::BeginRedelegate { .. } => "staking.MsgBeginRedelegate".into(),
        StakingOperation::Grant { .. } => "auth.grant".into(),
        StakingOperation::Exec { .. } => "auth.exec".into(),
    };

    Ok(the_type)
}

// Not sure about this one, gotta pray
#[derive(Serialize, Deserialize, Debug)]
#[repr(i64)]
pub enum SignMode {
    Unspecified = 0,
    Direct = 1,
    Textual = 2,
    DirectAux = 3,
    LegacyAminoJson = 127,
    Eip191 = 191,
    Unrecognized = -1,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SignerInfo {
    pub public_key: SignerInfoPublicKey,
    pub mode_info: ModeInfo,
    /**
     * sequence is the sequence of the account, which describes the
     * number of committed transactions signed by a given address. It is used to
     * prevent replay attacks.
     */
    // like a nonce
    pub sequence: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Stakes {
    // Address
    pub owner: String,
    pub current_balance: String,
    pub total_rewards: String,
    pub current_rewards: String,
    pub total_deposited_amount: String,
    pub total_withdrawn_amount: String,
    // Address
    pub vault: String,
    pub chain: String,
    pub updated_at_block: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StakeOperationsRequest {
    pub stake_addresses: Vec<String>,
    pub wallets: Vec<String>,
    pub pool_ids: Vec<String>,
    pub accounts: Vec<Uuid>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TxStakeMessage {
    #[serde(rename_all = "snake_case")]
    Value {
        #[serde(rename = "camelCase")]
        type_url: String,
        value: StakeMessageValue,
    },
    Restake {
        #[serde(rename = "camelCase")]
        type_url: String,
        allow_list: StakeMessageRestake,
        #[serde(rename = "PascalCase")]
        authorization_type: u64,
    },
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TotalBalance {
    pub amount_usd: f64,
    pub amount: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TotalRewards {
    pub amount_usd: f64,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TxDecodeAuthinfo {
    pub signer_infos: Vec<SignerInfo>,
    pub tip: Option<Tip>,
    pub fee: Option<TxStakeFee>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TxDecodeBody {
    pub messages: Vec<Any>,
    pub memo: String,
    pub timeout_height: TxDecodeGasLimit,
    pub extension_options: Vec<Any>,
    pub non_critical_extension_options: Vec<Any>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TxDecodingResponse {
    pub auth_info: TxDecodeAuthinfo,
    pub fee: TxStakeFee,
    pub body: TxDecodeBody,
    pub signatures: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TxDecodeGasLimit {
    pub low: u64,
    pub high: u64,
    pub unsigned: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Tip {
    pub amount: Vec<TxStakeCoin>,
    // Address
    pub tipper: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TxStakeCoin {
    pub denom: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TxStakeFee {
    pub amount: Vec<TxStakeCoin>,
    pub gas: String,
    pub granter: Option<String>,
    pub payer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TxResponse {
    pub unsigned_tx_serialized: String,
    pub unsigned_tx_hash: String,
    // hex
    pub tx_body: String,
    pub tx_auth_info: String,
    pub pubkey: String,
    pub fee: TxStakeFee,
    // for some reason, the API can return both a field named "message" and "messages" for the same thing
    pub messages: Vec<TxStakeMessage>,
    pub message: TxStakeMessage,
    pub chain_id: Chain,
    pub account_number: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TxStatusResponse {
    pub status: String,
    pub receipt: StatusResponseReceipt,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ValidatorOperationsRequest {
    pub validators: Vec<String>,
    pub delegators: Vec<String>,
    pub authz: bool,
    pub accounts: Vec<Uuid>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct WithdrawRewardsTxRequest {
    pub pubkey: String,
    pub validator: String,
}
