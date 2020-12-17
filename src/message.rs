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
}