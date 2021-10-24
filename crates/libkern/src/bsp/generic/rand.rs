use core::mem::size_of;

/// The default seed for a [`MtRand`].
pub const DEFAULT_SEED: u32 = 5489_u32;

const STATE_SIZE: usize = 624;
const MIDDLE: usize = 397;
const INIT_FACT: u32 = 1_812_433_253;
const INIT_SHIFT: u32 = 30;
const LOWER_MASK: u32 = 0x7FFFFFFF;
const UPPER_MASK: u32 = !LOWER_MASK;
const TWIST_MASK: u32 = 0x9908B0DF;

const SHIFT1: u32 = 11;
const SHIFT2: u32 = 7;
const SHIFT3: u32 = 15;
const SHIFT4: u32 = 18;

const MASK1: u32 = 0xFFFFFFFF;
const MASK2: u32 = 0x9D2C5680;
const MASK3: u32 = 0xEFC60000;

/// The 32-bit flavor of the Mersenne Twister pseudorandom number
/// generator.
pub struct MtRand {
    idx: usize,
    state: [u32; STATE_SIZE],
}

impl MtRand {
    /// Create a new `MtRand` instance with the given seed.
    pub const fn new(seed: u32) -> Self {
        let mut mt = Self {
            idx: 0,
            state: [0; STATE_SIZE],
        };
        mt.reseed(seed);
        mt
    }

    const fn reseed(&mut self, seed: u32) {
        self.idx = STATE_SIZE;
        self.state[0] = seed;

        let mut i = 1;
        while i < STATE_SIZE {
            // state[i] = (INIT_FACT * (state[i - 1] ^ (state[i - 1] >> INIT_SHIFT))) + i;
            self.state[i] = INIT_FACT
                .wrapping_mul(self.state[i - 1] ^ (self.state[i - 1].wrapping_shr(INIT_SHIFT)))
                .wrapping_add(i as u32);

            i += 1;
        }
    }

    /// Generate a new random `u32` number.
    pub const fn next_u32(&mut self) -> u32 {
        debug_assert!(self.idx != 0);

        if self.idx >= STATE_SIZE {
            self.twist();
        }

        let mut x = self.state[self.idx];
        self.idx += 1;

        // y ^= (y >> SHIFT1) & MASK1;
        x ^= x.wrapping_shr(SHIFT1) & MASK1;
        x ^= x.wrapping_shl(SHIFT2) & MASK2;
        x ^= x.wrapping_shl(SHIFT3) & MASK3;
        x ^= x.wrapping_shr(SHIFT4);
        x
    }

    /// Fill a buffer with bytes generated from the RNG.
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        const CHUNK: usize = size_of::<u32>();
        let mut left = dest;
        while left.len() >= CHUNK {
            let (next, remainder) = left.split_at_mut(CHUNK);
            left = remainder;
            let chunk: [u8; CHUNK] = self.next_u32().to_le_bytes();
            next.copy_from_slice(&chunk);
        }

        let n = left.len();
        if n > 0 {
            let chunk: [u8; CHUNK] = self.next_u32().to_le_bytes();
            left.copy_from_slice(&chunk[..n]);
        }
    }

    const fn twist(&mut self) {
        let mut i = 0;
        while i < STATE_SIZE {
            let x = (self.state[i] & UPPER_MASK) | (self.state[(i + 1) % STATE_SIZE] & LOWER_MASK);
            let y = if x & 1 != 0 { TWIST_MASK } else { 0 };
            let x = x.wrapping_shr(1) ^ y;
            self.state[i] = self.state[(i + MIDDLE) % STATE_SIZE] ^ x;

            i += 1;
        }
    }
}
