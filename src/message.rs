//! Contains enums for message types and field types

use anyhow::*;

use crate::traits::FromBytes;

/// Enumerates the three possible data types for message fields
pub enum FieldType {
    IBInteger,
    IBString,
    IBFloat,
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
}

#[derive(Debug, PartialEq)]
pub struct Message {
    fields: Vec<String>,
    raw: Option<String>,
}

impl Message {
    pub fn new() -> Message {
        Message {
            fields: vec![],
            raw: None,
        }
    }

    pub fn add_field<S: Into<String>>(&mut self, v: S) {
        let _v = format!("{}\0", v.into());
        self.fields.push(_v);
    }
}

impl FromBytes for Message {
    fn from_bytes(b: &[u8]) -> Result<Self> {
        match b.len() > 4 {
            true => {
                let size = &b[0..4];
                println!("B is: {:?}", b);
                let text = String::from_utf8(b[4..].into())?;

                let msg = Message {
                    fields: vec![],
                    raw: Some(text),
                };
                Ok(msg)
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
        let test_message = Message::from_bytes(&test_bytes).unwrap();
        assert_eq!(test_message.raw.unwrap().as_bytes(), &[84, 69, 83, 84, 0]);
    }
}
