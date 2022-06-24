use super::candlebase_protocol::*;
use actix::Message as ActixMessage;
use bytes::{BufMut, Bytes, BytesMut};
pub use prost::DecodeError;
use prost::Message as ProstMessage;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

pub enum ProtocolMsg {
  None,
  PingReq(PingReq),
  PingRep(PingRep),
  SaveCandlesReq(SaveCandlesReq),
  SaveCandlesRep(SaveCandlesRep),
  DeleteCandlesSinceReq(DeleteCandlesSinceReq),
  DeleteCandlesSinceRep(DeleteCandlesSinceRep),
  GetCandleReq(GetCandleReq),
  GetCandleRep(GetCandleRep),
  GetNthLastCandleReq(GetNthLastCandleReq),
  GetNthLastCandleRep(GetNthLastCandleRep),
  GetCandlesSinceReq(GetCandlesSinceReq),
  GetCandlesSinceRep(GetCandlesSinceRep),
  GetCandlesUntilReq(GetCandlesUntilReq),
  GetCandlesUntilRep(GetCandlesUntilRep),
  GetCandlesUntilLastReq(GetCandlesUntilLastReq),
  GetCandlesUntilLastRep(GetCandlesUntilLastRep),
  GetCandlesBetweenReq(GetCandlesBetweenReq),
  GetCandlesBetweenRep(GetCandlesBetweenRep),
  GetFirstTsReq(GetFirstTsReq),
  GetFirstTsRep(GetFirstTsRep),
  GetLastTsReq(GetLastTsReq),
  GetLastTsRep(GetLastTsRep),
  GetBoundaryTsesReq(GetBoundaryTsesReq),
  GetBoundaryTsesRep(GetBoundaryTsesRep),
  GetExactTsByHighReq(GetExactTsByHighReq),
  GetExactTsByHighRep(GetExactTsByHighRep),
  GetExactTsByLowReq(GetExactTsByLowReq),
  GetExactTsByLowRep(GetExactTsByLowRep),
  DestroyTopicReq(DestroyTopicReq),
  DestroyTopicRep(DestroyTopicRep),
  TruncateTopicReq(TruncateTopicReq),
  TruncateTopicRep(TruncateTopicRep),
  RenameTopicReq(RenameTopicReq),
  RenameTopicRep(RenameTopicRep),
  GetTopicsReq(GetTopicsReq),
  GetTopicsRep(GetTopicsRep),
  OkRep(OkRep),
  ErrorRep(ErrorRep),
}

impl Debug for ProtocolMsg {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    match self {
      ProtocolMsg::None => write!(f, "None"),
      ProtocolMsg::PingReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::PingRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::SaveCandlesReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::SaveCandlesRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::DeleteCandlesSinceReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::DeleteCandlesSinceRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetCandleReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetCandleRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetNthLastCandleReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetNthLastCandleRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetCandlesSinceReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetCandlesSinceRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetCandlesUntilReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetCandlesUntilRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetCandlesUntilLastReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetCandlesUntilLastRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetCandlesBetweenReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetCandlesBetweenRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetFirstTsReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetFirstTsRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetLastTsReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetLastTsRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetBoundaryTsesReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetBoundaryTsesRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetExactTsByHighReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetExactTsByHighRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetExactTsByLowReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetExactTsByLowRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::DestroyTopicReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::DestroyTopicRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::TruncateTopicReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::TruncateTopicRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RenameTopicReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::RenameTopicRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetTopicsReq(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::GetTopicsRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::OkRep(msg) => write!(f, "{:?}", msg),
      ProtocolMsg::ErrorRep(msg) => write!(f, "{:?}", msg),
    }
  }
}

#[derive(Debug)]
pub enum SendError {
  Timeout,
  Closed,
  Any(Box<dyn std::error::Error + Send + Sync>),
}

impl ActixMessage for ProtocolMsg {
  type Result = StdResult<ProtocolMsg, SendError>;
}

impl ActixMessage for PingReq {
  type Result = StdResult<PingRep, SendError>;
}

impl ActixMessage for SaveCandlesReq {
  type Result = StdResult<SaveCandlesRep, SendError>;
}

impl ActixMessage for DeleteCandlesSinceReq {
  type Result = StdResult<DeleteCandlesSinceRep, SendError>;
}

impl ActixMessage for GetCandleReq {
  type Result = StdResult<GetCandleRep, SendError>;
}

impl ActixMessage for GetNthLastCandleReq {
  type Result = StdResult<GetNthLastCandleRep, SendError>;
}

impl ActixMessage for GetCandlesSinceReq {
  type Result = StdResult<GetCandlesSinceRep, SendError>;
}

impl ActixMessage for GetCandlesUntilReq {
  type Result = StdResult<GetCandlesUntilRep, SendError>;
}

impl ActixMessage for GetCandlesUntilLastReq {
  type Result = StdResult<GetCandlesUntilLastRep, SendError>;
}

impl ActixMessage for GetCandlesBetweenReq {
  type Result = StdResult<GetCandlesBetweenRep, SendError>;
}

impl ActixMessage for GetFirstTsReq {
  type Result = StdResult<GetFirstTsRep, SendError>;
}

impl ActixMessage for GetLastTsReq {
  type Result = StdResult<GetLastTsRep, SendError>;
}

impl ActixMessage for GetBoundaryTsesReq {
  type Result = StdResult<GetBoundaryTsesRep, SendError>;
}

impl ActixMessage for GetExactTsByHighReq {
  type Result = StdResult<GetExactTsByHighRep, SendError>;
}

impl ActixMessage for GetExactTsByLowReq {
  type Result = StdResult<GetExactTsByLowRep, SendError>;
}

impl ActixMessage for DestroyTopicReq {
  type Result = StdResult<DestroyTopicRep, SendError>;
}

impl ActixMessage for TruncateTopicReq {
  type Result = StdResult<TruncateTopicRep, SendError>;
}

impl ActixMessage for RenameTopicReq {
  type Result = StdResult<RenameTopicRep, SendError>;
}

impl ActixMessage for GetTopicsReq {
  type Result = StdResult<GetTopicsRep, SendError>;
}

pub trait IntoEnum {
  fn into_enum(self) -> ProtocolMsg;
}

impl IntoEnum for PingReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::PingReq(self)
  }
}

impl IntoEnum for PingRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::PingRep(self)
  }
}

impl IntoEnum for SaveCandlesReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::SaveCandlesReq(self)
  }
}

impl IntoEnum for SaveCandlesRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::SaveCandlesRep(self)
  }
}

impl IntoEnum for DeleteCandlesSinceReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::DeleteCandlesSinceReq(self)
  }
}

impl IntoEnum for DeleteCandlesSinceRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::DeleteCandlesSinceRep(self)
  }
}

impl IntoEnum for GetCandleReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetCandleReq(self)
  }
}

impl IntoEnum for GetCandleRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetCandleRep(self)
  }
}

impl IntoEnum for GetNthLastCandleReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetNthLastCandleReq(self)
  }
}

impl IntoEnum for GetNthLastCandleRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetNthLastCandleRep(self)
  }
}

impl IntoEnum for GetCandlesSinceReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetCandlesSinceReq(self)
  }
}

impl IntoEnum for GetCandlesSinceRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetCandlesSinceRep(self)
  }
}

impl IntoEnum for GetCandlesUntilReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetCandlesUntilReq(self)
  }
}

impl IntoEnum for GetCandlesUntilRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetCandlesUntilRep(self)
  }
}

impl IntoEnum for GetCandlesUntilLastReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetCandlesUntilLastReq(self)
  }
}

impl IntoEnum for GetCandlesUntilLastRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetCandlesUntilLastRep(self)
  }
}

impl IntoEnum for GetCandlesBetweenReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetCandlesBetweenReq(self)
  }
}

impl IntoEnum for GetCandlesBetweenRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetCandlesBetweenRep(self)
  }
}

impl IntoEnum for GetFirstTsReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetFirstTsReq(self)
  }
}

impl IntoEnum for GetFirstTsRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetFirstTsRep(self)
  }
}

impl IntoEnum for GetLastTsReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetLastTsReq(self)
  }
}

impl IntoEnum for GetLastTsRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetLastTsRep(self)
  }
}

impl IntoEnum for GetBoundaryTsesReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetBoundaryTsesReq(self)
  }
}

impl IntoEnum for GetBoundaryTsesRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetBoundaryTsesRep(self)
  }
}

impl IntoEnum for GetExactTsByHighReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetExactTsByHighReq(self)
  }
}

impl IntoEnum for GetExactTsByHighRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetExactTsByHighRep(self)
  }
}

impl IntoEnum for GetExactTsByLowReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetExactTsByLowReq(self)
  }
}

impl IntoEnum for GetExactTsByLowRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetExactTsByLowRep(self)
  }
}

impl IntoEnum for DestroyTopicReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::DestroyTopicReq(self)
  }
}

impl IntoEnum for DestroyTopicRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::DestroyTopicRep(self)
  }
}

impl IntoEnum for TruncateTopicReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::TruncateTopicReq(self)
  }
}

impl IntoEnum for TruncateTopicRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::TruncateTopicRep(self)
  }
}

impl IntoEnum for RenameTopicReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::RenameTopicReq(self)
  }
}

impl IntoEnum for RenameTopicRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::RenameTopicRep(self)
  }
}

impl IntoEnum for GetTopicsReq {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetTopicsReq(self)
  }
}

impl IntoEnum for GetTopicsRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::GetTopicsRep(self)
  }
}

impl IntoEnum for OkRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::OkRep(self)
  }
}

impl IntoEnum for ErrorRep {
  #[inline]
  fn into_enum(self) -> ProtocolMsg {
    ProtocolMsg::ErrorRep(self)
  }
}

pub trait Encode: ProstMessage + Sized {
  fn encode_type(bytes: &mut BytesMut);

  #[inline]
  fn encode_body(&self, bytes: &mut BytesMut) {
    if let Err(err) = self.encode(bytes) {
      panic!("Failed to encode msg: {:?}", err);
    }
  }

  fn encode_msg(&self) -> Bytes {
    let size = self.encoded_len() as usize;
    let mut bytes = BytesMut::with_capacity(size + 1);
    Self::encode_type(&mut bytes);
    self.encode_body(&mut bytes);
    return bytes.freeze();
  }
}

impl Encode for PingReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(1);
  }
}

impl Encode for PingRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(2);
  }
}

impl Encode for SaveCandlesReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(3);
  }
}

impl Encode for SaveCandlesRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(4);
  }
}

impl Encode for DeleteCandlesSinceReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(15);
  }
}

impl Encode for DeleteCandlesSinceRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(16);
  }
}

impl Encode for GetCandleReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(25);
  }
}

impl Encode for GetCandleRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(26);
  }
}

impl Encode for GetNthLastCandleReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(33);
  }
}

impl Encode for GetNthLastCandleRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(34);
  }
}

impl Encode for GetCandlesSinceReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(37);
  }
}

impl Encode for GetCandlesSinceRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(38);
  }
}

impl Encode for GetCandlesUntilReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(39);
  }
}

impl Encode for GetCandlesUntilRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(40);
  }
}

impl Encode for GetCandlesUntilLastReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(43);
  }
}

impl Encode for GetCandlesUntilLastRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(44);
  }
}

impl Encode for GetCandlesBetweenReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(45);
  }
}

impl Encode for GetCandlesBetweenRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(46);
  }
}

impl Encode for GetFirstTsReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(47);
  }
}

impl Encode for GetFirstTsRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(48);
  }
}

impl Encode for GetLastTsReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(49);
  }
}

impl Encode for GetLastTsRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(50);
  }
}

impl Encode for GetBoundaryTsesReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(55);
  }
}

impl Encode for GetBoundaryTsesRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(56);
  }
}

impl Encode for GetExactTsByHighReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(57);
  }
}

impl Encode for GetExactTsByHighRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(58);
  }
}

impl Encode for GetExactTsByLowReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(59);
  }
}

impl Encode for GetExactTsByLowRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(60);
  }
}

impl Encode for DestroyTopicReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(79);
  }
}

impl Encode for DestroyTopicRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(80);
  }
}

impl Encode for TruncateTopicReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(81);
  }
}

impl Encode for TruncateTopicRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(82);
  }
}

impl Encode for RenameTopicReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(83);
  }
}

impl Encode for RenameTopicRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(84);
  }
}

impl Encode for GetTopicsReq {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(85);
  }
}

impl Encode for GetTopicsRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(86);
  }
}

impl Encode for OkRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(99);
  }
}

impl Encode for ErrorRep {
  #[inline]
  fn encode_type(bytes: &mut BytesMut) {
    bytes.put_u8(100);
  }
}

pub fn encode(protocol_msg: &ProtocolMsg) -> Bytes {
  match protocol_msg {
    ProtocolMsg::None => panic!("Failed to encode ProtocolMsg::None"),
    ProtocolMsg::PingReq(msg) => msg.encode_msg(),
    ProtocolMsg::PingRep(msg) => msg.encode_msg(),
    ProtocolMsg::SaveCandlesReq(msg) => msg.encode_msg(),
    ProtocolMsg::SaveCandlesRep(msg) => msg.encode_msg(),
    ProtocolMsg::DeleteCandlesSinceReq(msg) => msg.encode_msg(),
    ProtocolMsg::DeleteCandlesSinceRep(msg) => msg.encode_msg(),
    ProtocolMsg::GetCandleReq(msg) => msg.encode_msg(),
    ProtocolMsg::GetCandleRep(msg) => msg.encode_msg(),
    ProtocolMsg::GetNthLastCandleReq(msg) => msg.encode_msg(),
    ProtocolMsg::GetNthLastCandleRep(msg) => msg.encode_msg(),
    ProtocolMsg::GetCandlesSinceReq(msg) => msg.encode_msg(),
    ProtocolMsg::GetCandlesSinceRep(msg) => msg.encode_msg(),
    ProtocolMsg::GetCandlesUntilReq(msg) => msg.encode_msg(),
    ProtocolMsg::GetCandlesUntilRep(msg) => msg.encode_msg(),
    ProtocolMsg::GetCandlesUntilLastReq(msg) => msg.encode_msg(),
    ProtocolMsg::GetCandlesUntilLastRep(msg) => msg.encode_msg(),
    ProtocolMsg::GetCandlesBetweenReq(msg) => msg.encode_msg(),
    ProtocolMsg::GetCandlesBetweenRep(msg) => msg.encode_msg(),
    ProtocolMsg::GetFirstTsReq(msg) => msg.encode_msg(),
    ProtocolMsg::GetFirstTsRep(msg) => msg.encode_msg(),
    ProtocolMsg::GetLastTsReq(msg) => msg.encode_msg(),
    ProtocolMsg::GetLastTsRep(msg) => msg.encode_msg(),
    ProtocolMsg::GetBoundaryTsesReq(msg) => msg.encode_msg(),
    ProtocolMsg::GetBoundaryTsesRep(msg) => msg.encode_msg(),
    ProtocolMsg::GetExactTsByHighReq(msg) => msg.encode_msg(),
    ProtocolMsg::GetExactTsByHighRep(msg) => msg.encode_msg(),
    ProtocolMsg::GetExactTsByLowReq(msg) => msg.encode_msg(),
    ProtocolMsg::GetExactTsByLowRep(msg) => msg.encode_msg(),
    ProtocolMsg::DestroyTopicReq(msg) => msg.encode_msg(),
    ProtocolMsg::DestroyTopicRep(msg) => msg.encode_msg(),
    ProtocolMsg::TruncateTopicReq(msg) => msg.encode_msg(),
    ProtocolMsg::TruncateTopicRep(msg) => msg.encode_msg(),
    ProtocolMsg::RenameTopicReq(msg) => msg.encode_msg(),
    ProtocolMsg::RenameTopicRep(msg) => msg.encode_msg(),
    ProtocolMsg::GetTopicsReq(msg) => msg.encode_msg(),
    ProtocolMsg::GetTopicsRep(msg) => msg.encode_msg(),
    ProtocolMsg::OkRep(msg) => msg.encode_msg(),
    ProtocolMsg::ErrorRep(msg) => msg.encode_msg(),
  }
}

pub fn decode(bytes: &Bytes) -> Result<ProtocolMsg, DecodeError> {
  let msg_type = bytes[0] as i8;
  let msg_body = bytes.slice(1..);
  if msg_type == 1 {
    let res: Result<PingReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PingReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 2 {
    let res: Result<PingRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::PingRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 3 {
    let res: Result<SaveCandlesReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::SaveCandlesReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 4 {
    let res: Result<SaveCandlesRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::SaveCandlesRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 15 {
    let res: Result<DeleteCandlesSinceReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::DeleteCandlesSinceReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 16 {
    let res: Result<DeleteCandlesSinceRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::DeleteCandlesSinceRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 25 {
    let res: Result<GetCandleReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetCandleReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 26 {
    let res: Result<GetCandleRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetCandleRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 33 {
    let res: Result<GetNthLastCandleReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetNthLastCandleReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 34 {
    let res: Result<GetNthLastCandleRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetNthLastCandleRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 37 {
    let res: Result<GetCandlesSinceReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetCandlesSinceReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 38 {
    let res: Result<GetCandlesSinceRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetCandlesSinceRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 39 {
    let res: Result<GetCandlesUntilReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetCandlesUntilReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 40 {
    let res: Result<GetCandlesUntilRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetCandlesUntilRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 43 {
    let res: Result<GetCandlesUntilLastReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetCandlesUntilLastReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 44 {
    let res: Result<GetCandlesUntilLastRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetCandlesUntilLastRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 45 {
    let res: Result<GetCandlesBetweenReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetCandlesBetweenReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 46 {
    let res: Result<GetCandlesBetweenRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetCandlesBetweenRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 47 {
    let res: Result<GetFirstTsReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetFirstTsReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 48 {
    let res: Result<GetFirstTsRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetFirstTsRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 49 {
    let res: Result<GetLastTsReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetLastTsReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 50 {
    let res: Result<GetLastTsRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetLastTsRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 55 {
    let res: Result<GetBoundaryTsesReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetBoundaryTsesReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 56 {
    let res: Result<GetBoundaryTsesRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetBoundaryTsesRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 57 {
    let res: Result<GetExactTsByHighReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetExactTsByHighReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 58 {
    let res: Result<GetExactTsByHighRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetExactTsByHighRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 59 {
    let res: Result<GetExactTsByLowReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetExactTsByLowReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 60 {
    let res: Result<GetExactTsByLowRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetExactTsByLowRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 79 {
    let res: Result<DestroyTopicReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::DestroyTopicReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 80 {
    let res: Result<DestroyTopicRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::DestroyTopicRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 81 {
    let res: Result<TruncateTopicReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::TruncateTopicReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 82 {
    let res: Result<TruncateTopicRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::TruncateTopicRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 83 {
    let res: Result<RenameTopicReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::RenameTopicReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 84 {
    let res: Result<RenameTopicRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::RenameTopicRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 85 {
    let res: Result<GetTopicsReq, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetTopicsReq(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 86 {
    let res: Result<GetTopicsRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::GetTopicsRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 99 {
    let res: Result<OkRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::OkRep(msg)),
      Err(err) => Err(err),
    }
  } else if msg_type == 100 {
    let res: Result<ErrorRep, DecodeError> = ProstMessage::decode(msg_body);
    match res {
      Ok(msg) => Ok(ProtocolMsg::ErrorRep(msg)),
      Err(err) => Err(err),
    }
  } else {
    Err(DecodeError::new(format!("Invalid msg type: {}", msg_type)))
  }
}

pub fn set_round_ref(protocol_msg: &mut ProtocolMsg, round_ref: u32) -> &ProtocolMsg {
  match protocol_msg {
    ProtocolMsg::None => panic!("Failed to set round ref for ProtocolMsg::None"),
    ProtocolMsg::PingReq(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::PingRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::SaveCandlesReq(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::SaveCandlesRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::DeleteCandlesSinceReq(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::DeleteCandlesSinceRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetCandleReq(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetCandleRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetNthLastCandleReq(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetNthLastCandleRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetCandlesSinceReq(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetCandlesSinceRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetCandlesUntilReq(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetCandlesUntilRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetCandlesUntilLastReq(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetCandlesUntilLastRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetCandlesBetweenReq(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetCandlesBetweenRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetFirstTsReq(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetFirstTsRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetLastTsReq(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetLastTsRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetBoundaryTsesReq(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetBoundaryTsesRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetExactTsByHighReq(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetExactTsByHighRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetExactTsByLowReq(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetExactTsByLowRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::DestroyTopicReq(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::DestroyTopicRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::TruncateTopicReq(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::TruncateTopicRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::RenameTopicReq(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::RenameTopicRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetTopicsReq(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::GetTopicsRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::OkRep(msg) => msg.r#round_ref = round_ref,
    ProtocolMsg::ErrorRep(msg) => msg.r#round_ref = round_ref,
  }
  protocol_msg
}

pub fn get_round_ref(protocol_msg: &ProtocolMsg) -> u32 {
  match protocol_msg {
    ProtocolMsg::None => panic!("Failed to get round ref from ProtocolMsg::None"),
    ProtocolMsg::PingReq(msg) => msg.r#round_ref,
    ProtocolMsg::PingRep(msg) => msg.r#round_ref,
    ProtocolMsg::SaveCandlesReq(msg) => msg.r#round_ref,
    ProtocolMsg::SaveCandlesRep(msg) => msg.r#round_ref,
    ProtocolMsg::DeleteCandlesSinceReq(msg) => msg.r#round_ref,
    ProtocolMsg::DeleteCandlesSinceRep(msg) => msg.r#round_ref,
    ProtocolMsg::GetCandleReq(msg) => msg.r#round_ref,
    ProtocolMsg::GetCandleRep(msg) => msg.r#round_ref,
    ProtocolMsg::GetNthLastCandleReq(msg) => msg.r#round_ref,
    ProtocolMsg::GetNthLastCandleRep(msg) => msg.r#round_ref,
    ProtocolMsg::GetCandlesSinceReq(msg) => msg.r#round_ref,
    ProtocolMsg::GetCandlesSinceRep(msg) => msg.r#round_ref,
    ProtocolMsg::GetCandlesUntilReq(msg) => msg.r#round_ref,
    ProtocolMsg::GetCandlesUntilRep(msg) => msg.r#round_ref,
    ProtocolMsg::GetCandlesUntilLastReq(msg) => msg.r#round_ref,
    ProtocolMsg::GetCandlesUntilLastRep(msg) => msg.r#round_ref,
    ProtocolMsg::GetCandlesBetweenReq(msg) => msg.r#round_ref,
    ProtocolMsg::GetCandlesBetweenRep(msg) => msg.r#round_ref,
    ProtocolMsg::GetFirstTsReq(msg) => msg.r#round_ref,
    ProtocolMsg::GetFirstTsRep(msg) => msg.r#round_ref,
    ProtocolMsg::GetLastTsReq(msg) => msg.r#round_ref,
    ProtocolMsg::GetLastTsRep(msg) => msg.r#round_ref,
    ProtocolMsg::GetBoundaryTsesReq(msg) => msg.r#round_ref,
    ProtocolMsg::GetBoundaryTsesRep(msg) => msg.r#round_ref,
    ProtocolMsg::GetExactTsByHighReq(msg) => msg.r#round_ref,
    ProtocolMsg::GetExactTsByHighRep(msg) => msg.r#round_ref,
    ProtocolMsg::GetExactTsByLowReq(msg) => msg.r#round_ref,
    ProtocolMsg::GetExactTsByLowRep(msg) => msg.r#round_ref,
    ProtocolMsg::DestroyTopicReq(msg) => msg.r#round_ref,
    ProtocolMsg::DestroyTopicRep(msg) => msg.r#round_ref,
    ProtocolMsg::TruncateTopicReq(msg) => msg.r#round_ref,
    ProtocolMsg::TruncateTopicRep(msg) => msg.r#round_ref,
    ProtocolMsg::RenameTopicReq(msg) => msg.r#round_ref,
    ProtocolMsg::RenameTopicRep(msg) => msg.r#round_ref,
    ProtocolMsg::GetTopicsReq(msg) => msg.r#round_ref,
    ProtocolMsg::GetTopicsRep(msg) => msg.r#round_ref,
    ProtocolMsg::OkRep(msg) => msg.r#round_ref,
    ProtocolMsg::ErrorRep(msg) => msg.r#round_ref,
  }
}