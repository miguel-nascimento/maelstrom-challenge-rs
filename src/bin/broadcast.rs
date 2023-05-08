use std::{collections::HashMap, io::StdoutLock};

use distributed_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum BroadcastPayload {
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<usize>,
    },
    // Node network graph: We'll be ignoring this for now.
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}

struct BroadcastNode {
    id: usize,
    messages: Vec<usize>,
}

impl Node<(), BroadcastPayload> for BroadcastNode {
    fn step(
        &mut self,
        input: Message<BroadcastPayload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        let mut reply = input.into_reply(Some(&mut self.id));
        use BroadcastPayload::*;
        match reply.body.payload {
            Broadcast { message } => {
                self.messages.push(message);
                reply.body.payload = BroadcastOk
            }
            Read => {
                reply.body.payload = ReadOk {
                    messages: self.messages.clone(),
                }
            }
            Topology { .. } => reply.body.payload = TopologyOk,
            BroadcastOk | TopologyOk | ReadOk { .. } => {}
        };
        reply.send(output)
    }
    fn from_init(_state: (), _init: &Init) -> anyhow::Result<Self> {
        Ok(Self {
            id: 1,
            messages: vec![],
        })
    }
}
fn main() -> anyhow::Result<()> {
    run::<_, BroadcastPayload, BroadcastNode>(())
}
