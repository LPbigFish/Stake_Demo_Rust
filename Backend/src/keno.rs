use crate::game::Game;
use crate::hasher::{join_arrays, new_hash_from_bytes, random_hash};

#[derive(Clone, Copy)]
pub(crate) struct Keno {
    base_hash: [u8; 16],
}

impl Game for Keno {
    const KENO: [u8; 40] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
    ];
    fn new() -> Self {
        Keno {
            base_hash: random_hash(),
        }
    }

    fn new_with_params(base_hash: [u8; 16]) -> Self {
        Keno { base_hash }
    }

    fn roll(self, input: [u8; 16]) -> f32 {
        input.iter().sum::<u8>() as f32
    }

    fn shuff(self, input: [u8; 16]) -> [u8; 40] {
        let mut pad = Self::KENO.clone();

        let seed = new_hash_from_bytes(join_arrays(&input, &self.base_hash).as_slice());

        let mut j = 0;

        for i in 0..pad.len() {
            // Calculate a deterministic index based on the seed and the current index.
            j = (&j + seed[i % 16] as usize) % pad.len();

            // Swap the elements at the current index and the calculated index.
            pad.swap(i, j);
        }

        let mut result = [0u8; 40];
        result.copy_from_slice(pad.as_slice());
        result
    }
}
