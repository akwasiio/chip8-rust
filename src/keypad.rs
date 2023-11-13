use sdl2::keyboard::Keycode;

pub struct Keypad {
    pub keys: [bool; 16] // must be either true or false (1 or 0 respectively)
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            keys: [false; 16]
        }
    }

    pub fn on_key_event(&mut self, key: Keycode, state: bool) {
        match key {
            Keycode::Num1 => self.keys[0x1] = state,
            Keycode::Num2 => self.keys[0x2] = state,
            Keycode::Num3 => self.keys[0x3] = state,
            Keycode::Num4 => self.keys[0x4] = state,
            Keycode::Q => self.keys[0x5] = state,
            Keycode::W => self.keys[0x6] = state,
            Keycode::E => self.keys[0x7] = state,
            Keycode::R => self.keys[0x8] = state,
            Keycode::A => self.keys[0x9] = state,
            Keycode::S => self.keys[0xA] = state,
            Keycode::D => self.keys[0xB] = state,
            Keycode::Z => self.keys[0xC] = state,
            Keycode::X => self.keys[0xD] = state,
            Keycode::C => self.keys[0xE] = state,
            Keycode::V => self.keys[0xF] = state,
            _ => {}
        }
    }

}