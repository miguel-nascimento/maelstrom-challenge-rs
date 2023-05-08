use std::io::StdoutLock;

use distributed_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum EchoPayload {
    Echo { echo: String },
    EchoOk { echo: String },
}

struct EchoNode {
    id: usize,
}

impl Node<(), EchoPayload> for EchoNode {
    fn step(&mut self, input: Message<EchoPayload>, output: &mut StdoutLock) -> anyhow::Result<()> {
        let mut reply = input.into_reply(Some(&mut self.id));
        use EchoPayload::*;
        match reply.body.payload {
            Echo { echo } => reply.body.payload = EchoOk { echo },
            EchoOk { .. } => {}
        };
        reply.send(output)
    }
    fn from_init(_state: (), _init: &Init) -> anyhow::Result<Self> {
        Ok(Self { id: 1 })
    }
}
fn main() -> anyhow::Result<()> {
    run::<_, EchoPayload, EchoNode>(())
}
