use std::{collections::HashSet, time::Duration};

use crate::api::models::ExchangeDetails;

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        duration: Duration,
        counter_sleep: u32,
        counter_tick: u64,
        exchange_details: HashSet<ExchangeDetails>,
    },
    UpdateExchangeDetais {
        details: ExchangeDetails,
    },
}

impl AppState {
    pub fn initialized() -> Self {
        let duration = Duration::from_secs(1);
        let counter_sleep = 0;
        let counter_tick = 0;
        let exchange_details: HashSet<ExchangeDetails> = HashSet::new();
        Self::Initialized {
            duration,
            counter_sleep,
            counter_tick,
            exchange_details,
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn incr_sleep(&mut self) {
        if let Self::Initialized { counter_sleep, .. } = self {
            *counter_sleep += 1;
        }
    }

    pub fn incr_tick(&mut self) {
        if let Self::Initialized { counter_tick, .. } = self {
            *counter_tick += 1;
        }
    }

    pub fn count_sleep(&self) -> Option<u32> {
        if let Self::Initialized { counter_sleep, .. } = self {
            Some(*counter_sleep)
        } else {
            None
        }
    }

    pub fn count_tick(&self) -> Option<u64> {
        if let Self::Initialized { counter_tick, .. } = self {
            Some(*counter_tick)
        } else {
            None
        }
    }

    pub fn duration(&self) -> Option<&Duration> {
        if let Self::Initialized { duration, .. } = self {
            Some(duration)
        } else {
            None
        }
    }

    pub fn increment_delay(&mut self) {
        if let Self::Initialized { duration, .. } = self {
            // Set the duration, note that the duration is in 1s..10s
            let secs = (duration.as_secs() + 1).clamp(1, 10);
            *duration = Duration::from_secs(secs);
        }
    }

    pub fn decrement_delay(&mut self) {
        if let Self::Initialized { duration, .. } = self {
            // Set the duration, note that the duration is in 1s..10s
            let secs = (duration.as_secs() - 1).clamp(1, 10);
            *duration = Duration::from_secs(secs);
        }
    }

    pub fn upsert_exchange_details(&mut self, insert_details: ExchangeDetails) {
        if let Self::Initialized {
            exchange_details, ..
        } = self
        {
            exchange_details.insert(insert_details);
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
