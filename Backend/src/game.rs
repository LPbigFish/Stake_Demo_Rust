pub trait Game {

    const KENO: [u8;40];
    fn new() -> Self;
    fn roll(self, input: [u8; 16]) -> f32;

    fn shuff(self, input: [u8; 16]) -> [u8; 40];
}