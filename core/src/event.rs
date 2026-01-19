use crate::time::Timestamp;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Уникальный идентификатор события
pub type EventId = Uuid;

/// Биржа-источник
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Exchange {
    Binance,
    Okx,
    Bybit,
    Other(String),
}

/// Тип события (строгий контракт)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    // Market data
    BookSnapshot,
    BookDelta,
    Trade,
    TickerBbo,

    // Infra / data quality
    Connectivity,
    GapDetected,
    ResyncStarted,
    ResyncFinished,

    // Execution
    OrderSubmit,
    OrderAck,
    OrderReject,
    Fill,
    CancelAck,

    // Risk
    RiskStateChanged,
    KillSwitch,
}

/// Базовое событие платформы (event-sourcing)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    pub event_type: EventType,

    // Идентификация
    pub exchange: Exchange,
    pub symbol: String,

    // Временные метки
    pub ts_exchange: Option<Timestamp>,
    pub ts_recv: Timestamp,
    pub ts_proc: Timestamp,

    // Последовательность
    pub seq: Option<u64>,
    pub schema_version: u16,

    // Контроль целостности
    pub integrity_flags: Vec<String>,

    // Payload
    pub payload: EventPayload,

    // Произвольные метаданные (trace, debug)
    pub meta: HashMap<String, String>,
}

/// Полезная нагрузка события (строго типизирована)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventPayload {
    BookSnapshot {
        bids: Vec<(f64, f64)>,
        asks: Vec<(f64, f64)>,
    },
    BookDelta {
        bids: Vec<(f64, f64)>,
        asks: Vec<(f64, f64)>,
    },
    Trade {
        price: f64,
        qty: f64,
        is_maker: bool,
    },
    TickerBbo {
        bid: f64,
        ask: f64,
    },
    Connectivity {
        status: String,
    },
    GapDetected {
        from: u64,
        to: u64,
    },
    Order {
        order_id: String,
        price: f64,
        qty: f64,
    },
    Fill {
        order_id: String,
        price: f64,
        qty: f64,
    },
    Risk {
        state: String,
    },
    KillSwitch {
        reason: String,
    },
}
