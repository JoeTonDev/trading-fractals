use barter::{
  data::historical,
  engine::{trader::Trader, Engine},
  event::{Event, EventTx},
  execution::{
      simulated::{Config as ExecutionConfig, SimulatedExecution},
      Fees,
  },
  portfolio::{
      allocator::DefaultAllocator, portfolio::MetaPortfolio,
      repository::in_memory::InMemoryRepository, risk::DefaultRisk,
  },
  statistic::summary::{
      trading::{Config as StatisticConfig, TradingSummary},
      Initialiser,
  },
  strategy::example::{Config as StrategyConfig, RSIStrategy},
};
use barter_data::{
  event::{DataKind, MarketEvent},
  subscription::candle::Candle,
};
use barter_integration::model::{
  instrument::{kind::InstrumentKind, Instrument},
  Exchange, Market,
};
use chrono::Utc;
use parking_lot::Mutex;
use std::{collections::HashMap, fs, sync::Arc};
use tokio::sync::mpsc;
use uuid::Uuid;