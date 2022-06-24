use crate::connection_mgr::ConnectionMgr;
use crate::protocol::{Candle, GetCandlesSinceReq, GetCandlesUntilLastReq, IntoEnum, ProtocolMsg};

pub struct Client {
    endpoint: String,
    connection_mgr: ConnectionMgr,
}

impl Client {
    pub fn new(endpoint: String) -> Self {
        Client {
            endpoint,
            connection_mgr: ConnectionMgr::default(),
        }
    }

    pub async fn get_candles_until_last(&self, topic: String, limit: u32) -> Option<Vec<Candle>> {
        let conn = self.connection_mgr.fetch_connection(&self.endpoint);
        let result = conn
            .send(
                GetCandlesUntilLastReq {
                    topic,
                    limit,
                    price_adjusting_type: String::from("pre"),
                    vol_adjusting_type: String::from("none"),
                    round_ref: 0,
                }
                .into_enum(),
            )
            .await;
        if let Err(_) = result {
            return None;
        };
        let result = result.unwrap();
        if let Err(_) = result {
            return None;
        };
        let result = result.unwrap();
        match result {
            ProtocolMsg::GetCandlesUntilLastRep(a) => Some(a.candles),
            _ => None,
        }
    }

    pub async fn get_candles_since(
        &self,
        topic: String,
        ts: u32,
        limit: u32,
    ) -> Option<Vec<Candle>> {
        let conn = self.connection_mgr.fetch_connection(&self.endpoint);
        let result = conn
            .send(
                GetCandlesSinceReq {
                    topic,
                    ts,
                    limit,
                    price_adjusting_type: "pre".to_string(),
                    vol_adjusting_type: "none".to_string(),
                    round_ref: 0,
                }
                .into_enum(),
            )
            .await;
        if let Err(_) = result {
            return None;
        };
        let result = result.unwrap();
        if let Err(_) = result {
            return None;
        };
        let result = result.unwrap();
        match result {
            ProtocolMsg::GetCandlesSinceRep(a) => Some(a.candles),
            _ => None,
        }
    }
}
