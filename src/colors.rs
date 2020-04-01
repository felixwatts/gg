pub type Color = [f32; 4];

pub const RED: Color = [ 1.0, 0.0, 0.0, 1.0];
pub const YELLOW: Color = [ 1.0, 1.0, 0.0, 1.0];
pub const GREEN: Color = [ 0.0, 1.0, 0.0, 1.0];
pub const CYAN: Color = [ 0.0, 1.0, 1.0, 1.0];
pub const BLUE: Color = [ 0.0, 0.0, 1.0, 1.0];
pub const MAGENTA: Color = [ 1.0, 0.0, 1.0, 1.0];
pub const WHITE: Color = [1.0, 1.0, 1.0, 1.0];

const SPECTRUM: [Color; 6] = [
    RED,
    YELLOW,
    GREEN,
    CYAN,
    BLUE,
    MAGENTA
];

pub struct Colors{
    next_index: usize
}

impl Colors{
    pub fn new() -> Colors {
        Colors{next_index: 0}
    }
    pub fn next(&mut self) -> Color {
        let result = SPECTRUM[self.next_index];
        self.next_index = (self.next_index + 1) % SPECTRUM.len();
        result
    }
}