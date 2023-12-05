use rand::random;

use crate::constants::{FONT_ARR_LEN, FONTS, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::keypad::Keypad;

/// Uses 4kb of RAM(4096 bytes) from 0x000 to 0xFFF
/// First 512 bytes(0x000 to 0x1FF) are reserved for the interpreter. Store sprite data here
/// Chip-8 programs usually start at 0x200 and take up the rest of the space
///
///
///
const START_ADDRESS: u16 = 0x200;
const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUM_REGISTERS: usize = 16;

pub struct Chip8 {
    memory: [u8; RAM_SIZE],
    registers: [u8; NUM_REGISTERS],
    index_register: u16,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; STACK_SIZE],
    display_buffer: [[u16; SCREEN_WIDTH]; SCREEN_HEIGHT],
    pub keypad: Keypad,
    delay_timer: u8,
    sound_timer: u8,
    pub update_screen: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut ram = [0_u8; RAM_SIZE];
        ram[..FONT_ARR_LEN].copy_from_slice(&FONTS);

        Self {
            memory: ram,
            stack: [0; STACK_SIZE],
            registers: [0; NUM_REGISTERS],
            index_register: 0,
            program_counter: START_ADDRESS,
            stack_pointer: 0,
            display_buffer: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
            keypad: Keypad::new(),
            delay_timer: 0,
            sound_timer: 0,
            update_screen: false
        }
    }


    pub fn get_display_buffer(&self) -> [[u16; SCREEN_WIDTH]; SCREEN_HEIGHT] {
        self.display_buffer
    }

    fn push_addr_to_stack(&mut self, value: u16) {
        self.stack[self.stack_pointer as usize] = value;
        self.stack_pointer += 1
    }

    fn pop_addr_from_stack(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer as usize]
    }

    pub fn load_rom(&mut self, buffer: Vec<u8>) {
        for (index, byte) in buffer.iter().enumerate() {
            self.memory[START_ADDRESS as usize + index] = *byte
        }
    }

    pub fn run_cpu_cycle(&mut self) {
        let op_code = self.read_opcode();
        self.program_counter += 2; // incrementing twice because opcode is 2 bytes long
        self.process_opcode(op_code);
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 { self.delay_timer -= 1 }
        if self.sound_timer > 0 { self.sound_timer -= 1 }
    }

    /// opcodes are 2 bytes long, but our memory can only have one byte per slot
    /// Assuming we're at location memory[0x200],
    /// we fetch the address at memory[0x200] and shift 8 places to the left.
    /// So if the memory[0x200] is 0xAB, shifting 8 places to the left will give us 0xAB00
    /// which is 1010101100000000. And then we pick the remaining opcode from memory[0x200 + 1]
    /// Assuming that is also 0xCD, which is 11001101 in binary, to get 0xABCD we have to
    /// logically OR (the logical OR operation is perfect for setting bits) 0xAB00 and 0xCD
    /// which will give us 0xABCD
    fn read_opcode(&self) -> u16 {
        let first_byte = (self.memory[self.program_counter as usize] as u16) << 8;
        let second_byte = self.memory[(self.program_counter + 1) as usize] as u16;

        first_byte | second_byte
    }

    // this part was really tricky to understand but I finally get it!
    fn process_opcode(&mut self, op_code: u16) {
        // break down op code??? or just pass it to the function?
        let x: usize = usize::from((op_code & 0x0F00) >> 8);
        let y: usize = usize::from((op_code & 0x00F0) >> 4);

        // no shifting necessary because they are in the LSB positions
        let n = op_code & 0x000F;
        let nn = op_code & 0x00FF;
        let nnn = op_code & 0x0FFF;

        let msb = (op_code & 0xF000) >> 12;

        // grab the first number of the opcode.
        match msb {
            0x0 => self.handle_0xxx_codes(n),
            0x1 => self.handle_1nnn(nnn),
            0x2 => self.handle_2nnn(nnn),
            0x3 => self.handle_3xnn(x, nn),
            0x4 => self.handle_4xnn(x, nn),
            0x5 => self.handle_5xy0(x, y),
            0x6 => self.handle_6x_nn(x, nn),
            0x7 => self.handle_7x_nn(x, nn),
            0x8 => self.handle_8xxx_codes(x, y, n),
            0x9 => self.handle_9xy0(x, y),
            0xA => self.handle_annn(nnn),
            0xB => self.handle_bnnn(nnn),
            0xC => self.handle_cxkk(x, nn),
            0xD => self.dxyn(x, y, n),
            0xE => self.handle_exx_codes(x, nn),
            0xF => self.handle_fxx_codes(x, nn),
            _ => println!("Not handled yet: {}", msb)
        }
    }


    fn handle_0xxx_codes(&mut self, n: u16) {
        match n {
            0 => {
                self.display_buffer = [[0; SCREEN_WIDTH]; SCREEN_HEIGHT]
            }

            0xE => {
                self.stack_pointer -= 1;
                self.program_counter = self.stack[self.stack_pointer as usize];
            }

            _ => { println!("Undefined instruction: {}", n) }
        }
    }

    fn handle_1nnn(&mut self, nnn: u16) {
        self.program_counter = nnn;
    }

    fn handle_2nnn(&mut self, nnn: u16) {
        self.push_addr_to_stack(self.program_counter);
        self.program_counter = nnn
    }

    fn handle_00ee(&mut self) {
        self.program_counter = self.pop_addr_from_stack()
    }

    fn handle_3xnn(&mut self, x: usize, nn: u16) {
        if self.registers[x] == nn as u8 {
            self.program_counter += 2
        }
    }

    fn handle_4xnn(&mut self, x: usize, nn: u16) {
        if self.registers[x] != nn as u8 {
            self.program_counter += 2
        }
    }

    fn handle_5xy0(&mut self, x: usize, y: usize) {
        if self.registers[x] == self.registers[y] {
            self.program_counter += 2
        }
    }

    fn handle_9xy0(&mut self, x: usize, y: usize) {
        if self.registers[x] != self.registers[y] {
            self.program_counter += 2
        }
    }

    fn handle_6x_nn(&mut self, x: usize, nn: u16) {
        // set register vx
        self.registers[x] = nn as u8;
    }

    fn handle_7x_nn(&mut self, x: usize, nn: u16) {
        // add value to register vx
        self.registers[x] = self.registers[x].wrapping_add(nn as u8);
    }

    fn handle_8xxx_codes(&mut self, x: usize, y: usize, n: u16) {
        match n {
            0x0 => { self.handle_8xy0(x, y) }
            0x1 => { self.handle_8xy1(x, y) }
            0x2 => { self.handle_8xy2(x, y) }
            0x3 => { self.handle_8xy3(x, y) }
            0x4 => { self.handle_8xy4(x, y) }
            0x5 => { self.handle_8xy5(x, y) }
            0x6 => { self.handle_8xy6(x) }
            0x7 => { self.handle_8xy7(x, y) }
            0xE => { self.handle_8xye(x) }

            _ => { println!("Not implemented. n: {}", n) }
        }
    }

    fn handle_exx_codes(&mut self, x: usize, nn: u16) {
        match nn {
            0x9E => { self.handle_ex9e(x) }
            0xA1 => { self.handle_exa1(x) }
            _ => { println!("Not implemented. n: {}", nn) }
        }
    }

    fn handle_fxx_codes(&mut self, x: usize, nn: u16) {
        match nn {
            0x07 => self.handle_fx07(x),
            0x15 => self.handle_fx15(x),
            0x18 => self.handle_fx18(x),
            0x1E => self.handle_fx1e(x),
            0x0A => self.handle_fx0a(x),
            0x29 => self.handle_fx29(x),
            0x33 => self.handle_fx33(x),
            0x55 => self.handle_fx55(x),
            0x65 => self.handle_fx65(x),
            _ => {}
        }
    }

    fn handle_8xy0(&mut self, x: usize, y: usize) {
        self.registers[x] = self.registers[y]
    }

    fn handle_8xy1(&mut self, x: usize, y: usize) {
        self.registers[x] |= self.registers[y]
    }

    fn handle_8xy2(&mut self, x: usize, y: usize) {
        self.registers[x] &= self.registers[y]
    }

    fn handle_8xy3(&mut self, x: usize, y: usize) {
        self.registers[x] ^= self.registers[y]
    }

    fn handle_8xy4(&mut self, x: usize, y: usize) {
        let (result, carry) = self.registers[x].overflowing_add(self.registers[y]);

        self.registers[0xF] = if carry { 1 } else { 0 };
        self.registers[x] = result
    }

    fn handle_8xy5(&mut self, x: usize, y: usize) {
        let vx = self.registers[x];
        let vy = self.registers[y];

        let (res, borrow) = vx.overflowing_sub(vy);

        self.registers[x] = res;

        self.registers[0xF] = if borrow { 1 } else { 0 };
    }

    fn handle_8xy6(&mut self, x: usize) {
        // if lsb of vx is 1, then vf = 1 else 0. then divide vx by 2
        let vx = self.registers[x];
        let lsb = vx & 1;

        self.registers[0xF] = lsb;
        self.registers[x] >>= 1 // right shifting by 1 means div by 2
    }

    fn handle_8xy7(&mut self, x: usize, y: usize) {
        let vx = self.registers[x];
        let vy = self.registers[y];

        // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
        self.registers[0xF] = if vy > vx { 1 } else { 0 };
        self.registers[x] = vy - vx
    }

    fn handle_8xye(&mut self, x: usize) {
        let vx = self.registers[x];
        let msb = (vx >> 7) & 1;
        self.registers[0xF] = msb;
        self.registers[x] <<= 1
    }


    fn handle_annn(&mut self, nnn: u16) {
        // set index register I
        self.index_register = nnn
    }

    fn handle_bnnn(&mut self, nnn: u16) {
        // set the program counter to nnn plus v0
        self.program_counter = nnn + self.registers[0x0] as u16
    }

    fn handle_cxkk(&mut self, x: usize, nn: u16) {
        let rnd: u8 = random();
        self.registers[x] = rnd & nn as u8;
    }


    fn dxyn(&mut self, x: usize, y: usize, n: u16) {
        // display
        self.registers[0xF] = 0;
        let vx = self.registers[x];
        let vy = self.registers[y];

        // loop over a sprite
        for sprite_row in 0..n {
            let sprite_addr = self.index_register + sprite_row;
            // grab a sprite starting from I
            let sprite = self.memory[sprite_addr as usize];

            // loop over each bit/pixel in sprite
            for sprite_column in 0..8 {
                // grabs pixel from sprite
                let pixel = sprite & (0x80 >> sprite_column);

                if pixel != 0 {
                    let screen_x = (vx + sprite_column) % (SCREEN_WIDTH as u8);
                    let screen_y = (vy + sprite_row as u8) % (SCREEN_HEIGHT as u8);

                    if self.display_buffer[screen_y as usize][screen_x as usize] == 1 {
                        self.registers[0xF] = 1;
                    }

                    self.display_buffer[screen_y as usize][screen_x as usize] ^= 1;
                }
            }
        }
        self.update_screen = true
    }

    fn handle_ex9e(&mut self, x: usize) {
        let key_x = self.keypad.keys[x];

        if key_x {
            self.program_counter += 2
        }
    }

    fn handle_exa1(&mut self, x: usize) {
        if !self.keypad.keys[x] {
            self.program_counter += 2
        }
    }

    fn handle_fx07(&mut self, x: usize) {
        self.registers[x] = self.delay_timer
    }

    fn handle_fx0a(&mut self, x: usize) {
        let mut pressed = false;
        for i in 0..self.keypad.keys.len() {
            if self.keypad.keys[i] {
                self.registers[x] = i as u8;
                pressed = true;
                break;
            }
        }

        if !pressed {
            // rerun
            self.program_counter -= 2
        }
    }

    fn handle_fx15(&mut self, x: usize) {
        self.delay_timer = self.registers[x]
    }

    fn handle_fx18(&mut self, x: usize) {
        self.sound_timer = self.registers[x]
    }

    fn handle_fx1e(&mut self, x: usize) {
        self.index_register += self.registers[x] as u16
    }

    fn handle_fx29(&mut self, x: usize) {
        self.index_register = self.registers[x] as u16 * 5 // because ram of font takes up 5 bytes
    }

    fn handle_fx33(&mut self, x: usize) {
        let vx = self.registers[x];
        self.memory[self.index_register as usize] = vx / 100;
        self.memory[self.index_register as usize + 1] = (vx / 10) % 10;
        self.memory[self.index_register as usize + 2] = vx % 10;
    }

    fn handle_fx55(&mut self, x: usize) {
        self.memory[self.index_register as usize ..= self.index_register as usize + x].copy_from_slice(&self.registers[0 ..= x])
    }

    fn handle_fx65(&mut self, x: usize) {
        self.registers[0 ..= x].copy_from_slice(&self.memory[self.index_register as usize ..= self.index_register as usize + x])
    }
}