//! Implementation of the Cyclic Redundancy Check.

const DEFAULT_CRC32_TABLE: [u32; 256] = crc32_table(0x04C11DB7);

/// Perform a CRC32 of the given data.
pub const fn crc32(buf: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFFu32;

    let mut idx = 0;
    while idx < buf.len() {
        let lookup = crc as u8 ^ buf[idx];
        crc = (crc >> 8) ^ DEFAULT_CRC32_TABLE[lookup as usize];

        idx += 1;
    }

    !crc
}

const fn crc32_table(mut poly: u32) -> [u32; 256] {
    let poly = {
        let mut res = 0;
        let mut i = 0;
        while i < 32 {
            res <<= 1;
            res |= poly & 1;
            poly >>= 1;

            i += 1;
        }
        res
    };

    let mut table = [0u32; 256];
    let mut idx = 0;

    while idx < table.len() {
        table[idx] = {
            let mut value = idx as u32;

            let mut i = 0;
            while i < 8 {
                if value & 1 != 0 {
                    value >>= 1;
                    value ^= poly;
                } else {
                    value >>= 1;
                }

                i += 1;
            }

            value
        };
        idx += 1;
    }
    table
}
