use alloy::primitives::FixedBytes;

#[derive(Debug, Clone)]
pub enum StateUpdate {
    DelayedTronSet { key: &'static str, until: u64 },
    DelayedTronClear { key: &'static str },
    TipProofResendRemove { tip: FixedBytes<32> },
}

#[derive(Debug, Clone)]
pub struct Plan<I> {
    pub intent: Option<I>,
    pub updates: Vec<StateUpdate>,
}

impl<I> Plan<I> {
    pub fn none() -> Self {
        Self {
            intent: None,
            updates: Vec::new(),
        }
    }

    pub fn intent(intent: I) -> Self {
        Self {
            intent: Some(intent),
            updates: Vec::new(),
        }
    }

    pub fn update(mut self, update: StateUpdate) -> Self {
        self.updates.push(update);
        self
    }

    pub fn extend_updates(mut self, updates: impl IntoIterator<Item = StateUpdate>) -> Self {
        self.updates.extend(updates);
        self
    }
}
