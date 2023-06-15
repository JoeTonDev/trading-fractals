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

pub struct FractalIndicator {
 
  period: usize,
}

impl FractalIndicator {
  pub fn new(period: usize) -> Self {
      FractalIndicator { period }
  }

  pub fn calculate(&self, highs: &[f64], lows: &[f64]) -> Result<Vec<bool>, &'static str> {
      if highs.len() < self.period + 2 || lows.len() < self.period + 2 {
          return Err("Insufficient price data");
      }

      let mut signals: Vec<bool> = vec![false; highs.len()];

      for i in (self.period..highs.len() - self.period).step_by(1) {
          let n = highs[i];
          let n_minus_2 = highs[i - 2];
          let n_minus_1 = highs[i - 1];
          let n_plus_1 = highs[i + 1];
          let n_plus_2 = highs[i + 2];

          let is_bearish_fractal = n > n_minus_2 && n > n_minus_1 && n > n_plus_1 && n > n_plus_2;

          signals[i] = is_bearish_fractal;
      }

      for i in (self.period..lows.len() - self.period).step_by(1) {
          let n = lows[i];
          let n_minus_2 = lows[i - 2];
          let n_minus_1 = lows[i - 1];
          let n_plus_1 = lows[i + 1];
          let n_plus_2 = lows[i + 2];

          let is_bullish_fractal = n < n_minus_2 && n < n_minus_1 && n < n_plus_1 && n < n_plus_2;

          signals[i] = signals[i] || is_bullish_fractal;
      }

      Ok(signals)
  }
}


// use fractal_indicator::FractalIndicator;

// fn main() {
//     let highs = vec![30.0, 20.0, 40.0, 50.0, 25.0, 60.0];
//     let lows = vec![10.0, 5.0, 15.0, 20.0, 10.0, 25.0];
//     let period = 2;

//     let indicator = FractalIndicator::new(period);
//     let signals = indicator.calculate(&highs, &lows).unwrap();

//     for (i, signal) in signals.iter().enumerate() {
//         if *signal {
//             println!("Fractal pattern signal found at index {}", i);
//         }
//     }
// }
