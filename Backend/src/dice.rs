use crate::game::Game;
use crate::hasher::new_hash_from_bytes;

pub(crate) struct Dice {
    base_hash: [u8; 16],
}

impl Game for Dice {
    const KENO: [u8; 40] = todo!();
    fn new() -> Self {
        Dice {
            base_hash: crate::hasher::random_hash(),
        }
    }

    fn roll(self, input: [u8; 16]) -> f32 {
        let hash = new_hash_from_bytes(
            input
                .iter()
                .cloned()
                .chain(self.base_hash.iter().cloned())
                .collect::<Vec<u8>>()
                .as_slice(),
        );

        let seed = u64::from_be_bytes(hash[0..8].try_into().unwrap());

        ((seed % 10001) as f64 / 100.0f64) as f32
    }

    fn shuff(self, input: [u8; 16]) -> [u8; 40] {
        let mut output = [0u8; 40];
        output.copy_from_slice(&input);
        return output;
    }
}
