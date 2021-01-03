//! Contains enums for message types and field types

use anyhow::*;
use std::convert::Into;

use crate::traits::FromBytes;

/// Enumerates the three possible data types for message fields
#[derive(Debug, PartialEq)]
pub enum IBField {
    IBInteger(u32),
    IBString(String),
    IBFloat(f32),
}

#[derive(Debug, PartialEq)]
pub enum Message {
    Inbound(InboundMessage),
    Outbound(OutboundMessage),
}

impl Message {
    pub fn new_inbound() -> Message {
        Message::Inbound(InboundMessage::new())
    }

    pub fn new_outbound() -> Message {
        Message::Outbound(OutboundMessage::new())
    }

    pub fn add_field(&mut self, field: IBField) {
        match self {
            Message::Inbound(msg) => {
                msg.add_field(field);
            }
            Message::Outbound(msg) => msg.add_field(field),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Message::Inbound(msg) => msg.to_bytes(),
            Message::Outbound(msg) => msg.to_bytes(),
        }
    }
}
/// These are the possible inbound messages we can receive from the server
pub enum InboundMessages {
    TickPrice,
    TickSize,
    OrderStatus,
    ErrMsg,
    OpenOrder,
    AccountValue,
    PortfolioValue,
    AccountUpdateTime,
    NextValidID,
    ContractData,
    ExecutionData,
    MarketDepth,
    MarketDepthL2,
    NewsBulletins,
    ManagedAccounts,
    ReceiveFa,
    HistoricalData,
    BondContractData,
    ScannerParameters,
    ScannerData,
    TickOptionComputation,
    TickGeneric,
    TickString,
    TickEfp,
    CurrentTime,
    RealTimeBars,
    FundamentalData,
    ContractDataEnd,
    OpenOrderEnd,
    AccountDownloadEnd,
    ExecutionDataEnd,
    DeltaNeutralValidation,
    TickSnapshotEnd,
    MarketDataType,
    CommissionReport,
    PositionData,
    PositionEnd,
    AccountSummary,
    AccountSummaryEnd,
    VerifyMessageApi,
    VerifyCompleted,
    DisplayGroupList,
    DisplayGroupUpdated,
    VerifyAndAuthMessageApi,
    VerifyAndAuthCompleted,
    PositionMulti,
    PositionMultiEnd,
    AccountUpdateMulti,
    AccountUpdateMultiEnd,
    SecurityDefinitionOptionParameter,
    SecurityDefinitionOptionParameterEnd,
    SoftDollarTiers,
    FamilyCodes,
    SymbolSamples,
    MarketDepthExchanges,
    TickReqParams,
    SmartComponents,
    NewsArticle,
    TickNews,
    NewsProviders,
    HistoricalNews,
    HistoricalNewsEnd,
    HeadTimestamp,
    HistogramData,
    HistoricalDataUpdate,
    RerouteMarketDataReq,
    RerouteMarketDepthReq,
    MarketRule,
    ProfitAndLoss,
    ProfitAndLossSingle,
    HistoricalTicks,
    HistoricalTicksBidAsk,
    HistoricalTicksLast,
    TickByTick,
    OrderBound,
    CompletedOrder,
    CompletedOrdersEnd,
    ReplaceFaEnd,
}

/// These are the possible outbound message types we can send to the server
pub enum OutboundMessages {
    ReqMarketData,
    CancelMarketData,
    PlacerOrder,
    CancelOrder,
    ReqOpenOrders,
    ReqAccountData,
    ReqExecutions,
    ReqIds,
    ReqContractData,
    ReqMarketDepth,
    CancelMarketDepth,
    ReqNewsBulletins,
    CancelNewsBulletins,
    SetServerLogLevel,
    ReqAutoOpenOrders,
    ReqAllOpenOrders,
    ReqManagedAccounts,
    ReqFa,
    ReplaceFa,
    ReqHistoricalData,
    ExerciseOptions,
    ReqScannerSubscription,
    CancelScannerSubscription,
    ReqScannerParameters,
    CancelHistoricalData,
    ReqCurrentTime,
    ReqRealTimeBars,
    CancelRealTimeBars,
    ReqFundamentalData,
    CancelFundamentalData,
    ReqCalcImpliedVolat,
    CancelCalcImpliedVolat,
    CancelCalcOptionPrice,
    ReqGlobalCancel,
    ReqMarketDataType,
    ReqPositions,
    ReqAccountSummary,
    CancelAccountSummary,
    CancelPositions,
    VerifyRequest,
    VerifyMessage,
    QueryDisplayGroups,
    SubscribeToGroupEvents,
    UpdateDisplayGroup,
    UnsubscribeFromGroupEvents,
    StartApi,
    VerifyAndAuthRequest,
    VerifyAndAuthMessage,
    ReqPositionsMulti,
    CancelPositionsMulti,
    ReqAccountUpdatesMulti,
    CancelAccountUpdatesMulti,
    ReqSecDefOptParams,
    ReqSoftDollarTiers,
    ReqFamilyCodes,
    ReqMatchingSymbols,
    ReqMarketDepthExchanges,
    ReqSmartComponents,
    ReqNewsArticle,
    ReqNewsProviders,
    ReqHistoricalNews,
    ReqHeadTimestamp,
    ReqHistogramData,
    CancelHistogramData,
    CancelHeadTimestamp,
    ReqMarketRule,
    ReqPnl,
    CancelPnl,
    ReqPnlSingle,
    CancelPnlSingle,
    ReqHistoricalTicks,
    ReqTickByTickData,
    CancelTickByTickData,
    ReqCompletedOrders,
    Invalid,
}

/// Allows for converting an OutboundMessage to a u32 to be serialized for transmission
impl Into<u32> for OutboundMessages {
    fn into(self) -> u32 {
        match self {
            OutboundMessages::StartApi => 71,
            _ => 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OutboundMessage {
    fields: Vec<IBField>,
}

impl OutboundMessage {
    pub fn new() -> OutboundMessage {
        OutboundMessage { fields: vec![] }
    }

    pub fn add_field(&mut self, v: IBField) {
        self.fields.push(v);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        for field in &self.fields {
            match field {
                IBField::IBFloat(v) => {
                    bytes = [bytes, v.to_be_bytes().to_vec()].concat();
                }
                IBField::IBInteger(v) => {
                    bytes = [bytes, v.to_be_bytes().to_vec()].concat();
                }
                IBField::IBString(v) => {
                    bytes = [bytes, v.as_bytes().to_vec()].concat();
                }
            }
        }
        bytes
    }
}

#[derive(Debug, PartialEq)]
pub struct InboundMessage {
    fields: Vec<IBField>,
    raw: Option<String>,
}

impl InboundMessage {
    pub fn new() -> InboundMessage {
        InboundMessage {
            fields: vec![],
            raw: None,
        }
    }

    pub fn add_field(&mut self, v: IBField) {
        self.fields.push(v);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        for field in &self.fields {
            match field {
                IBField::IBFloat(v) => {
                    bytes = [bytes, v.to_be_bytes().to_vec()].concat();
                }
                IBField::IBInteger(v) => {
                    bytes = [bytes, v.to_be_bytes().to_vec()].concat();
                }
                IBField::IBString(v) => {
                    bytes = [bytes, v.as_bytes().to_vec()].concat();
                }
            }
        }
        bytes
    }

    pub fn from_bytes(b: &[u8]) -> Result<InboundMessage, Error> {
        match b.len() > 4 {
            true => {
                let size = &b[0..4];
                let msg = InboundMessage {
                    raw: Some(String::from_utf8(b[4..].into())?),
                    fields: vec![],
                };

                Ok(msg)
            }
            false => Err(anyhow!("Not enough bytes in message")),
        }
    }
}

impl FromBytes for InboundMessage {
    fn from_bytes(b: &[u8]) -> Result<Message> {
        match b.len() > 4 {
            true => {
                let size = &b[0..4];
                let text = String::from_utf8(b[4..].into())?;

                let msg = InboundMessage {
                    fields: vec![],
                    raw: Some(text),
                };
                Ok(Message::Inbound(msg))
            }
            false => Err(anyhow!("Not enough bytes in message")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_message_from_bytes() {
        let _length: u32 = 0x000004;
        let _length = _length.to_be_bytes();
        let test_string = String::from("TEST\0");
        let test_bytes = test_string.as_bytes();
        let test_bytes = [&_length, test_bytes].concat();
        let test_message = InboundMessage::from_bytes(&test_bytes).unwrap();
    }
}
