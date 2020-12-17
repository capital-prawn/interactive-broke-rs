pub enum FieldType {
    IBInteger,
    IBString,
    IBFloat,
}

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
