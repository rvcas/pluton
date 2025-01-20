use iced::{
    font, padding,
    widget::{column, container, row, scrollable, text, text_editor},
    Border, Element,
    Length::Fill,
    Theme,
};
use pallas::ledger::{addresses::Address, primitives::conway};

use super::{Message, MultiEraTx, State};

impl State {
    pub fn view(&self) -> Element<Message> {
        row![
            column![text_editor(&self.tx_cbor)
                .placeholder("tx cbor...")
                .on_action(Message::TxCborChanged)
                .height(Fill)
                .wrapping(text::Wrapping::Glyph)
                .style(|theme: &Theme, status| {
                    let mut defaults = text_editor::default(theme, status);

                    defaults.border.radius = 4.0.into();

                    defaults
                })],
            container(
                scrollable(column![match &self.transaction {
                    Some(tx) => render_tx(tx),
                    None => text("nothing to decode").into(),
                }])
                .width(Fill)
            )
            .width(Fill)
            .height(Fill)
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();

                container::Style {
                    border: Border {
                        width: 1.0,
                        radius: 4.0.into(),
                        color: palette.background.strong.color,
                    },
                    ..Default::default()
                }
            })
            .padding(10)
        ]
        .height(Fill)
        .padding(padding::top(30).left(10).right(10).bottom(10))
        .spacing(10)
        .into()
    }
}

fn render_tx(tx: &MultiEraTx) -> Element<Message> {
    match tx {
        MultiEraTx::AlonzoCompatible(tx, era) => column![
            text("Alonzo Compatible"),
            text(format!("Era: {:?}", era)),
            text(format!("{:?}", tx))
        ]
        .into(),
        MultiEraTx::Babbage(tx) => column![text("Babbage"), text(format!("{:?}", tx))].into(),
        MultiEraTx::Byron(tx) => column![text("Byron"), text(format!("{:?}", tx))].into(),
        MultiEraTx::Conway(tx) => column![
            text("Conway").size(18).font(font::Font {
                weight: font::Weight::Bold,
                ..Default::default()
            }),
            text(format!(
                "total inputs = {}",
                tx.transaction_body.inputs.len()
            )),
            column(tx.transaction_body.inputs.iter().map(|input| {
                column![
                    text(format!("index = {}", input.index)),
                    text(format!("hash  = {}", input.transaction_id)),
                ]
                .into()
            }))
            .spacing(5),
            text(format!(
                "total outputs = {}",
                tx.transaction_body.outputs.len()
            )),
            column(tx.transaction_body.outputs.iter().map(|out| {
                match out {
                    conway::PseudoTransactionOutput::Legacy(output) => {
                        let address = Address::from_bytes(&output.address).unwrap();

                        column![text(format!("address = {}", address.to_bech32().unwrap())),].into()
                    }
                    conway::PseudoTransactionOutput::PostAlonzo(output) => {
                        let address = Address::from_bytes(&output.address).unwrap();

                        column![text(format!("address = {}", address.to_bech32().unwrap())),].into()
                    }
                }
            }))
            .spacing(5),
        ]
        .spacing(10)
        .into(),
    }
}
