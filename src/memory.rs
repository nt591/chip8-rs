use crate::screen::font::FONT;

const MEMORY_SIZE: usize = 2 << 11; //4096
const ROM_ADDR: usize = 2 << 8; // 512
const FONT_ADDR: usize = 0x050; // by convention

#[derive(Debug, Clone)]
struct Memory {
    memory: [u8; MEMORY_SIZE],
}

impl Memory {
    fn new() -> Self {
        let mut memory = [0u8; MEMORY_SIZE];
        // load fonts
        FONT.iter()
            .enumerate()
            .for_each(|(idx, font)| memory[FONT_ADDR + idx] = *font);
        Self { memory }
    }
}
