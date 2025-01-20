use std::sync::Arc;

use iced::widget::text_editor;
use pallas::{
    codec::minicbor,
    ledger::{
        primitives::{alonzo, babbage, byron, conway},
        traverse::Era,
    },
};

#[derive(Debug, Default)]
pub struct State {
    pub tx_cbor: text_editor::Content,
    pub transaction: Option<MultiEraTx>,
}
#[derive(Debug, Clone)]
pub enum MultiEraTx {
    AlonzoCompatible(Arc<alonzo::Tx>, Era),
    Babbage(babbage::Tx),
    Byron(byron::Tx),
    Conway(conway::Tx),
}

#[derive(thiserror::Error, Debug)]
#[error("unknown cbor")]
pub struct DecodeTxError;

impl MultiEraTx {
    pub fn decode(cbor: &[u8]) -> Result<Self, DecodeTxError> {
        if let Ok(tx) = minicbor::decode(cbor) {
            return Ok(MultiEraTx::Conway(tx));
        }

        if let Ok(tx) = minicbor::decode(cbor) {
            return Ok(MultiEraTx::Babbage(tx));
        }

        if let Ok(tx) = minicbor::decode::<alonzo::Tx>(cbor) {
            // Shelley/Allegra/Mary/Alonzo will all decode to Alonzo
            return Ok(MultiEraTx::AlonzoCompatible(tx.into(), Era::Alonzo));
        }

        if let Ok(tx) = minicbor::decode(cbor) {
            Ok(MultiEraTx::Byron(tx))
        } else {
            Err(DecodeTxError)
        }
    }
}
