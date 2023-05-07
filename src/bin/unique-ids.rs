use std::io::StdoutLock;

use anyhow::Context;
use distributed_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum UniquePayload {
    Generate,
    GenerateOk { id: String },
}

struct UniqueNode {
    id: usize,
    node_id: String,
}

impl Node<(), UniquePayload> for UniqueNode {
    fn step(
        &mut self,
        input: Message<UniquePayload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        let mut reply = input.into_reply(Some(&mut self.id));
        let guid = format!("{}{}", self.id, self.node_id);

        match reply.body.payload {
            UniquePayload::Generate => reply.body.payload = UniquePayload::GenerateOk { id: guid },
            UniquePayload::GenerateOk { .. } => {}
        };

        reply.send(output).context("on send reply")?;
        self.id += 1;
        Ok(())
    }
    fn from_init(_state: (), init: &Init) -> anyhow::Result<Self> {
        Ok(Self {
            id: 1,
            node_id: init.node_id.clone(),
        })
    }
}
fn main() -> anyhow::Result<()> {
    run::<_, UniquePayload, UniqueNode>(())
}
