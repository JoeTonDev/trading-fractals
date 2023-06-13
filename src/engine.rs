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

const DATA_HISTORIC_CANDLES_1H: &str = "trading-fractals/data/candles_1h.json";

async fn main() {
  let (_command_tx, command_rx) = mpsc::channel(20);
  let (event_tx, event_rx) = mpsc::unbounded_channel();
  let event_tx = EventTx::new(event_tx);
  let engine_id =Uuid::new_v4();
  let market = Market::new("binance", ("btc", "usdt", InstrumentKind::Spot));
  
  let portfolio = Arc::new(Mutex::new(
    MetaPortfolio::builder()
    .engine_id(engine_id)
    .markets(vec![market.clone()])
    .starting_cash(10_000_000.0)
    .repository(InMemoryRepository::new())
    .allocation_manager(DefaultAllocator {
        default_order_value: 100.0,
    })
    .risk_manager(DefaultRisk {})
    .statistic_config(StatisticConfig {
      starting_equity: 10_000.0,
      trading_days_per_year: 365,
      risk_free_return: 0.0,
    })
    .build_and_init()
    .expect("failed to build & initialise MetaPortfolio"),
  ));

  let mut traders = Vec::new();

  let (trader_command_tx, trader_command_rx) = mpsc::channel(10);

  traders.push(
    Trader::builder()
      .engine_id(engine_id)
      .market(market.clone())
      .command_rx(trader_command_rx)
      .event_tx(event_tx.clone())
      .portfolio(Arc::clone(&portfolio))
      .data(historical::MarketFeed::new(
        load_json_market_event_candles().into_iter(),
      ))
      .strategy(Fractals::new(StrategyConfig { 
        simulated_fees_pct: Fees {
          exchange: 0.1,
          slippage: 0.05,
          network: 0.0,
        },
      }))
      .build()
      .expect("failed to build trader"),
  );

  let trader_command_txs = HashMap::from([(market, trader_command_tx)]);

  let engine = Engine::builder()
    .engine_id(engine_id)
    .command_rx(command_rx)
    .portfolio(portfolio)
    .traders(traders)
    .trader_command_txs(TradingSummary::init(StatisticConfig {
      starting_equity: 1000.0,
      trading_days_per_year: 365,
      risk_free_return: 0.0,
    }))
    .build()
    .expect("failed to build engine");

  tokio::spawn(listen_to_engine_events(event_rx));
  engine.run().await;

}

fn load_json_market_event_candles() -> Vec<MarketEvent<DataKind>> {
  let candles = fs::read_to_string(DATA_HISTORIC_CANDLES_1H).expect("failed to read file");
  let candles = serde_json::from_str::<Vec<Candle>>(&candles).expect("failed to parse candles String");

  candles.into_iter()
    .map(|candle| MarketEvent {
      exchange_time: candle.close_time,
      received_time: Utc::now(),
      exchange: Exchange::from("binance"),
      instrument: Instrument::from(("btc", "usdt", InstrumentKind::Spot)),
      kind: DataKind::Candle(candle),
    })
    .collect()

  }
    async fn listen_to_engine_events(mut event_rx: mpsc::UnboundedReceiver<Event>) {
      while let Some(event) = event_rx.recv().await {
          match event {
              Event::Market(_) => {
                  // Market Event occurred in Engine
              }
              Event::Signal(signal) => {
                  // Signal Event occurred in Engine
                  println!("{signal:?}");
              }
              Event::SignalForceExit(_) => {
                  // SignalForceExit Event occurred in Engine
              }
              Event::OrderNew(new_order) => {
                  // OrderNew Event occurred in Engine
                  println!("{new_order:?}");
              }
              Event::OrderUpdate => {
                  // OrderUpdate Event occurred in Engine
              }
              Event::Fill(fill_event) => {
                  // Fill Event occurred in Engine
                  println!("{fill_event:?}");
              }
              Event::PositionNew(new_position) => {
                  // PositionNew Event occurred in Engine
                  println!("{new_position:?}");
              }
              Event::PositionUpdate(updated_position) => {
                  // PositionUpdate Event occurred in Engine
                  println!("{updated_position:?}");
              }
              Event::PositionExit(exited_position) => {
                  // PositionExit Event occurred in Engine
                  println!("{exited_position:?}");
              }
              Event::Balance(balance_update) => {
                  // Balance update Event occurred in Engine
                  println!("{balance_update:?}");
              }
          }
      }
    }

