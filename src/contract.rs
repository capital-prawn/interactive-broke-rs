//! Contains data structures for contracts of various types

use rust_decimal::prelude::*;

/// These are some convenience type wrappers
pub type ContractId = usize;
pub type Symbol = String;
pub type LastTradeDateOrContractMonth = String;
pub type Strike = Decimal;
pub type Right = String;
pub type Multiplier = String;
pub type Exchange = String;
pub type Currency = String;
pub type TradingClass = String;
pub type SecurityId = String;
pub enum SecurityType {
    Stock,
    Bond,
    Future,
    Option,
}

pub enum SecurityIdType {
    CUSIP,
    SEDOL,
    ISIN,
    RIC,
}

/// A combo refers to a complex Options position composed of multiple legs, or individual options strategies
pub struct ComboLeg {}

/// Refers to a contract that tries to keep delta close to 0
pub struct DeltaNeutralContract {}

/// Contract is a general contract, representing a future, a stock, an option, etc. This is the most commonly used contract
pub struct Contract {
    contract_id: ContractId,
    symbol: Symbol,
    security_type: SecurityType,
    last_trade_date_or_contract_month: LastTradeDateOrContractMonth,
    strike: Strike,
    right: Right,
    multiplier: Multiplier,
    exchange: Exchange,
    primary_exchange: Exchange,
    currency: Currency,
    local_symbol: Symbol,
    trading_class: TradingClass,
    include_expired: bool,
    security_id_type: SecurityIdType,
    security_id: SecurityId,
    combo_legs_description: Option<String>,
    combo_legs: Vec<ComboLeg>,
    delta_neutral_contract: Option<DeltaNeutralContract>,
}

pub type Market = String;
pub type Tick = usize;
/// Mixin struct that contains much of the ancillary details of a contract
pub struct ContractDetails {
    contract: Contract,
    market_name: Market,
    min_tick: Tick,
    order_types: Option<String>,
    valid_exchanges: Vec<Exchange>,
    price_magnifier: Option<Decimal>,
}

/// TODO: Find out what this is
pub struct ContractDescription {}
