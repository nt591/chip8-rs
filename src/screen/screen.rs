use bitvec::prelude::{bitarr, BitArray};

#[derive(Debug, Clone, Copy)]
struct Screen {
    screen: [BitArray; 32], // 64 pixels wide, 32 pixels tall
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            screen: [bitarr![0; 32]; 32],
        }
    }
}
