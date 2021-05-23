#[cfg(feature = "tocs")]
use crate::consumer::*;

pub struct TimeOrderedConsumers {
  consumers: Vec<Consumer>,
}

impl TimeOrderedConsumers {
  fn consume(&self) {}
}
