use iced::{widget::text_editor, Task};

use super::state::{MultiEraTx, State};

#[derive(Debug, Clone)]
pub enum Message {
    TxDecoded(Box<Option<MultiEraTx>>),
    TxCborChanged(text_editor::Action),
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        use Message::*;
        match message {
            TxDecoded(tx) => {
                self.transaction = *tx;

                Task::none()
            }
            TxCborChanged(action) => {
                self.tx_cbor.perform(action);

                let cbor = self.tx_cbor.text();

                Task::perform(tx_decode(cbor), TxDecoded)
            }
        }
    }
}

async fn tx_decode(cbor: String) -> Box<Option<MultiEraTx>> {
    hex::decode(cbor.trim())
        .ok()
        .and_then(|b| MultiEraTx::decode(&b).ok())
        .into()
}
