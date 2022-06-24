// msg type defs 

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingReq {
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRep {
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveCandlesReq {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub candles: ::prost::alloc::vec::Vec<Candle>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveCandlesRep {
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCandlesSinceReq {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub ts: u32,
    #[prost(uint32, tag="3")]
    pub limit: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCandlesSinceRep {
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandleReq {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub ts: u32,
    #[prost(string, tag="3")]
    pub price_adjusting_type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub vol_adjusting_type: ::prost::alloc::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandleRep {
    #[prost(message, optional, tag="1")]
    pub candle: ::core::option::Option<Candle>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNthLastCandleReq {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub n: u32,
    #[prost(string, tag="3")]
    pub price_adjusting_type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub vol_adjusting_type: ::prost::alloc::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNthLastCandleRep {
    #[prost(message, optional, tag="1")]
    pub candle: ::core::option::Option<Candle>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandlesSinceReq {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub ts: u32,
    #[prost(uint32, tag="3")]
    pub limit: u32,
    #[prost(string, tag="4")]
    pub price_adjusting_type: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub vol_adjusting_type: ::prost::alloc::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandlesSinceRep {
    #[prost(message, repeated, tag="1")]
    pub candles: ::prost::alloc::vec::Vec<Candle>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandlesUntilReq {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub ts: u32,
    #[prost(uint32, tag="3")]
    pub limit: u32,
    #[prost(string, tag="4")]
    pub price_adjusting_type: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub vol_adjusting_type: ::prost::alloc::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandlesUntilRep {
    #[prost(message, repeated, tag="1")]
    pub candles: ::prost::alloc::vec::Vec<Candle>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandlesUntilLastReq {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub limit: u32,
    #[prost(string, tag="3")]
    pub price_adjusting_type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub vol_adjusting_type: ::prost::alloc::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandlesUntilLastRep {
    #[prost(message, repeated, tag="1")]
    pub candles: ::prost::alloc::vec::Vec<Candle>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandlesBetweenReq {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub begin_ts: u32,
    #[prost(uint32, tag="3")]
    pub end_ts: u32,
    #[prost(uint32, tag="4")]
    pub limit: u32,
    #[prost(string, tag="5")]
    pub price_adjusting_type: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub vol_adjusting_type: ::prost::alloc::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandlesBetweenRep {
    #[prost(message, repeated, tag="1")]
    pub candles: ::prost::alloc::vec::Vec<Candle>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFirstTsReq {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFirstTsRep {
    #[prost(uint32, tag="1")]
    pub ts: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastTsReq {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastTsRep {
    #[prost(uint32, tag="1")]
    pub ts: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBoundaryTsesReq {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBoundaryTsesRep {
    #[prost(uint32, tag="1")]
    pub first_ts: u32,
    #[prost(uint32, tag="2")]
    pub last_ts: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExactTsByHighReq {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(double, tag="2")]
    pub price: f64,
    #[prost(uint32, tag="3")]
    pub ts: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExactTsByHighRep {
    #[prost(uint32, tag="1")]
    pub ts: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExactTsByLowReq {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(double, tag="2")]
    pub price: f64,
    #[prost(uint32, tag="3")]
    pub ts: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExactTsByLowRep {
    #[prost(uint32, tag="1")]
    pub ts: u32,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroyTopicReq {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroyTopicRep {
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TruncateTopicReq {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TruncateTopicRep {
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameTopicReq {
    #[prost(string, tag="1")]
    pub old_topic: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub new_topic: ::prost::alloc::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameTopicRep {
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTopicsReq {
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTopicsRep {
    #[prost(string, repeated, tag="1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, repeated, tag="2")]
    pub ids: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OkRep {
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorRep {
    #[prost(int32, tag="1")]
    pub code: i32,
    #[prost(string, tag="2")]
    pub desc: ::prost::alloc::string::String,
    #[prost(uint32, tag="15")]
    pub round_ref: u32,
}
// data type defs 

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Candle {
    #[prost(uint32, tag="1")]
    pub ts: u32,
    #[prost(double, tag="2")]
    pub open: f64,
    #[prost(double, tag="3")]
    pub high: f64,
    #[prost(double, tag="4")]
    pub low: f64,
    #[prost(double, tag="5")]
    pub close: f64,
    #[prost(double, tag="6")]
    pub vol: f64,
    #[prost(double, tag="7")]
    pub amt: f64,
}
// msg type enum 

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MsgType {
    /// unused, placeholder for gpb: 0 
    Unknown = 0,
    PingReq = 1,
    PingRep = 2,
    SaveCandlesReq = 3,
    SaveCandlesRep = 4,
    /// DELETE_CANDLE_REQ =: 5;
    /// DELETE_CANDLE_REP =: 6;
    /// DELETE_FIRST_CANDLE_REQ =: 7;
    /// DELETE_FIRST_CANDLE_REP =: 8;
    /// DELETE_LAST_CANDLE_REQ =: 9;
    /// DELETE_LAST_CANDLE_REP =: 10;
    /// DELETE_NTH_CANDLE_REQ =: 11;
    /// DELETE_NTH_CANDLE_REP =: 12;
    /// DELETE_NTH_LAST_CANDLE_REQ =: 13;
    /// DELETE_NTH_LAST_CANDLE_REP =: 14;
    DeleteCandlesSinceReq = 15,
    /// DELETE_CANDLES_UNTIL_REQ =: 17;
    /// DELETE_CANDLES_UNTIL_REP =: 18;
    /// DELETE_CANDLES_SINCE_FIRST_REQ =: 19;
    /// DELETE_CANDLES_SINCE_FIRST_REP =: 20;
    /// DELETE_CANDLES_UNTIL_LAST_REQ =: 21;
    /// DELETE_CANDLES_UNTIL_LAST_REP =: 22;
    /// DELETE_CANDLES_BETWEEN_REQ =: 23;
    /// DELETE_CANDLES_BETWEEN_REP =: 24;
    DeleteCandlesSinceRep = 16,
    GetCandleReq = 25,
    GetCandleRep = 26,
    /// GET_FIRST_CANDLE_REQ =: 27;
    /// GET_FIRST_CANDLE_REP =: 28;
    /// GET_LAST_CANDLE_REQ =: 29;
    /// GET_LAST_CANDLE_REP =: 30;
    /// GET_NTH_CANDLE_REQ =: 31;
    /// GET_NTH_CANDLE_REP =: 32;
    GetNthLastCandleReq = 33,
    GetNthLastCandleRep = 34,
    /// GET_BOUNDARY_CANDLES_REQ =: 35;
    /// GET_BOUNDARY_CANDLES_REP =: 36;
    GetCandlesSinceReq = 37,
    GetCandlesSinceRep = 38,
    GetCandlesUntilReq = 39,
    GetCandlesUntilRep = 40,
    /// GET_CANDLES_SINCE_FIRST_REQ =: 41;
    /// GET_CANDLES_SINCE_FIRST_REP =: 42;
    GetCandlesUntilLastReq = 43,
    GetCandlesUntilLastRep = 44,
    GetCandlesBetweenReq = 45,
    GetCandlesBetweenRep = 46,
    GetFirstTsReq = 47,
    GetFirstTsRep = 48,
    GetLastTsReq = 49,
    GetLastTsRep = 50,
    /// GET_NTH_TS_REQ =: 51;
    /// GET_NTH_TS_REP =: 52;
    /// GET_NTH_LAST_TS_REQ =: 53;
    /// GET_NTH_LAST_TS_REP =: 54;
    GetBoundaryTsesReq = 55,
    GetBoundaryTsesRep = 56,
    GetExactTsByHighReq = 57,
    GetExactTsByHighRep = 58,
    GetExactTsByLowReq = 59,
    GetExactTsByLowRep = 60,
    // EXIST_REQ =: 77;
    // EXIST_REP =: 78;

    DestroyTopicReq = 79,
    DestroyTopicRep = 80,
    TruncateTopicReq = 81,
    TruncateTopicRep = 82,
    RenameTopicReq = 83,
    RenameTopicRep = 84,
    GetTopicsReq = 85,
    GetTopicsRep = 86,
    // GET_UPDATES_SINCE_REQ =: 90;
    // GET_UPDATES_SINCE_REP =: 91;

    OkRep = 99,
    ErrorRep = 100,
}
