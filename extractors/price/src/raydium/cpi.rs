use base64::{engine::general_purpose::STANDARD as base64, Engine as _};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug, Copy, Clone)]
pub struct RaydiumCpiLog {
    amount_in: u64,
}

impl RaydiumCpiLog {
    pub fn from_encoded_log(log: &str) -> Option<Self> {
        if !log.contains("Program data:") {
            return None;
        }

        let base64_data = log.split_whitespace().last()?;
        let decoded = base64.decode(base64_data).ok()?;

        println!(
            "RaydiumCpiLog ------ looking for a swap event {:#?}",
            decoded
        );

        let mut cursor = Cursor::new(decoded);

        cursor.set_position(8);

        Some(Self {
            amount_in: cursor.read_u64::<LittleEndian>().ok()?,
        })
    }
}
