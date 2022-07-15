use std::mem;

use byteorder::{ByteOrder, LE};

/// The maximum number of KIPs a INI1 record can store.
pub const MAX_KIP_COUNT: u8 = 0x50;

/// The header magic of a KIP binary.
pub const KIP_MAGIC: &[u8] = b"KIP1";

const INI1_MAGIC: u32 = u32::from_le_bytes(*b"INI1");

/// The header of an INI1 record.
pub type Ini1Header = [u8; 16];

/// Builds the INI1 header for a record of KIPs, if necessary.
pub fn build_ini1_header(kip_bytes: usize, kip_count: u8) -> Option<Ini1Header> {
    if kip_count == 0 {
        return None;
    }

    let mut header = [0; 16];

    LE::write_u32(&mut header, INI1_MAGIC);
    LE::write_u32(
        &mut header[4..],
        (kip_bytes + mem::size_of::<Ini1Header>()) as u32,
    );
    LE::write_u32(&mut header[8..], kip_count as u32);

    Some(header)
}
