use std::io::{StdoutLock, Write};

use anyhow::{Context, Ok};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message<Payload> {
    pub src: String,
    pub dest: String,
    pub body: Body<Payload>,
}

impl<Payload> Message<Payload>
where
    Payload: Serialize,
{
    pub fn send(&self, output: &mut StdoutLock) -> anyhow::Result<()> {
        serde_json::to_writer(&mut *output, &self).context("on write reply")?;
        output.write_all(b"\n").context("while writing newline")?;
        Ok(())
    }

    pub fn into_reply(self, id: Option<&mut usize>) -> Self {
        Self {
            src: self.dest,
            dest: self.src,
            body: Body {
                id: id.map(|id| {
                    let mut ptr_id = *id;
                    ptr_id += 1;
                    ptr_id
                }),
                in_reply_to: self.body.id,
                payload: self.body.payload,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Body<Payload> {
    #[serde(rename = "msg_id")]
    pub id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum InitPayload {
    Init(Init),
    InitOk,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

pub trait Node<S, Payload> {
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()>;
    fn from_init(state: S, init: &Init) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub fn run<S, P, N>(initial: S) -> anyhow::Result<()>
where
    P: DeserializeOwned,
    N: Node<S, P>,
{
    let mut stdin = std::io::stdin().lines();
    let mut stdout = std::io::stdout().lock();

    let init_msg: Message<InitPayload> =
        serde_json::from_str(&stdin.next().expect("no init msg").context("no init msg")?)
            .context("init could be serialized")?;

    let InitPayload::Init(init) = &init_msg.body.payload else {
        panic!("expected init payload")
    };

    let mut node = N::from_init(initial, init).context("failed to initialize node")?;

    let mut initial_id = 0;
    let mut reply = init_msg.into_reply(Some(&mut initial_id));
    match reply.body.payload {
        InitPayload::Init { .. } => reply.body.payload = InitPayload::InitOk,
        InitPayload::InitOk => {}
    };
    reply.send(&mut stdout).context("on write init msg reply")?;

    for line in stdin {
        let line = line.context("fail to read from stdin")?;
        let data: Message<P> =
            serde_json::from_str(&line).context("while maelstrom serialization")?;
        node.step(data, &mut stdout)?;
    }

    Ok(())
}
