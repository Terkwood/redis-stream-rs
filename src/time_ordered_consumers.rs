use crate::consumer::*;
use anyhow::{Context, Result};

pub struct TimeOrderedConsumers {
  consumers: Vec<Consumer>,
}

impl TimeOrderedConsumers {
  fn consume(&self) -> Result<()> {
    todo!("sort by xid");
    todo!("copy paste a bunch from consumer.consume");
    todo!("call something like consumer.process_message... BUT");
    todo!("xack a bunch of things at once to reduce calls? maybe?")
  }
}
