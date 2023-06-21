use crate::data::MarketMeta;
use barter_data::event::{DataKind, MarketEvent};
use barter_integration::model::{instrument::Instrument, Exchange, Market};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod engine;

pub trait SignalGenerator {
  fn generate_signal(&mut self, market: &MarketEvent<DataKind>) -> Option<Signal>;
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Signal {
  pub time: DateTime<Utc>,
    pub exchange: Exchange,
    pub instrument: Instrument,
    pub signals: HashMap<Decision, SignalStrength>,
    pub market_data: MarketMeta,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub enum Decision {
  Long,
  CloseLong,
  Short,
  CloseShort,
}

impl Default for Decision {
  fn Default() -> Self {
    Self::Long
  }
}

impl Decision {
  pub fn is_long(&self) -> bool {
    matches!(self, Decision::Long)
  }

  pub fn is_short(&self) -> bool {
    matches!(self, Decision::Short)
  }

  pub fn is_entry(&self) -> bool {
    matches!(self, Decision::Short | Decision::Long)
  }

  pub fn is_exit(&self) -> bool {
    matches!(self, Decision::CloseLong | Decision::CloseShort)
  }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct SignalStrength(pub f64);

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct SignalForceExit {
  pub time: DateTime<Utc>,
    pub exchange: Exchange,
    pub instrument: Instrument,
}

impl <M> From<M> for SignalForceExit
where 
M: Into<Market>,
{
  fn from(market: M) -> Self {
    let market = market.into();
    Self::new(market.exchange, market.instrument)
  }
}

impl SignalForceExit {
  pub const FORCED_EXIT_SIGNAL: &'static str = "SignalForcedExit";

  pub fn new<E, I>(exchange: E, instrument: I) -> Self
  where
    E: Into<Exchange>,
    I: Into<Instrument>,
    {
      Self {
        time: Utc::now(),
            exchange: exchange.into(),
            instrument: instrument.into(),
      }
    }
}