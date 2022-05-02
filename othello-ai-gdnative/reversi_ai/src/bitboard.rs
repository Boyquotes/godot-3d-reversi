pub const SIZE: i32 = 8;

const MASKS: [u64; 8] = [
    0x7F7F7F7F7F7F7F7F,
    0x007F7F7F7F7F7F7F,
    0xFFFFFFFFFFFFFFFF,
    0x00FEFEFEFEFEFEFE,
    0xFEFEFEFEFEFEFEFE,
    0xFEFEFEFEFEFEFE00,
    0xFFFFFFFFFFFFFFFF,
    0x7F7F7F7F7F7F7F00,
];

const LSHIFT: [u64; 8] = [0, 0, 0, 0, 1, 9, 8, 7];
const RSHIFT: [u64; 8] = [1, 9, 8, 7, 0, 0, 0, 0];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Black,
    White,
    Null,
}

impl Color {
    pub fn reverse(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
            Color::Null => Color::Null,
        }
    }
}

fn shift(disk: u64, dir: usize) -> u64 {
    if dir < 8 / 2 {
        (disk >> RSHIFT[dir]) & MASKS[dir]
    } else {
        (disk << LSHIFT[dir]) & MASKS[dir]
    }
}

// This uses fewer arithmetic operations than any other known
// implementation on machines with slow multiplication.
// This algorithm uses 17 arithmetic operations.
fn pop_cnt(mut x: u64) -> i32 {
    x -= (x >> 1) & 0x5555555555555555; //put count of each 2 bits into those 2 bits
    x = (x & 0x3333333333333333) + ((x >> 2) & 0x3333333333333333); //put count of each 4 bits into those 4 bits
    x = (x + (x >> 4)) & 0x0f0f0f0f0f0f0f0f; //put count of each 8 bits into those 8 bits
    x += x >> 8; //put count of each 16 bits into their lowest 8 bits
    x += x >> 16; //put count of each 32 bits into their lowest 8 bits
    x += x >> 32; //put count of each 64 bits into their lowest 8 bits
    (x & 0x7f) as i32
}

#[derive(Debug)]
pub struct BitBoard {
    black: u64,
    white: u64,
}

impl BitBoard {
    pub fn new(input: String) -> BitBoard {
        let mut bd = BitBoard { black: 0, white: 0 };
        let bytes = input.as_bytes();
        for i in 0..((SIZE * SIZE) as usize) {
            match bytes[i] {
                b'X' => bd.assign(i as i32, Color::Black),
                b'O' => bd.assign(i as i32, Color::White),
                b'+' => bd.assign(i as i32, Color::Null),
                _ => panic!("unknown identifier {}", bytes[i].to_string()),
            }
        }
        bd
    }

    pub fn copy(&self) -> BitBoard {
        BitBoard {
            black: self.black,
            white: self.white,
        }
    }

    pub fn at(&self, loc: i32) -> Color {
        let sh = 1u64 << loc;
        if self.black & sh != 0 {
            Color::Black
        } else if self.white & sh != 0 {
            Color::White
        } else {
            Color::Null
        }
    }

    pub fn assign(&mut self, loc: i32, cl: Color) {
        let sh = 1u64 << loc;
        match cl {
            Color::Black => self.black |= sh,
            Color::White => self.white |= sh,
            _ => (),
        }
    }

    pub fn put(&mut self, loc: i32, cl: Color) {
        self.assign(loc, cl);
        self.flip(loc, cl);
    }

    pub fn put_and_check(&mut self, loc: i32, cl: Color) -> bool {
        if loc < 0 || loc >= SIZE * SIZE {
            return false;
        }
        if self.at(loc) != Color::Null || !self.is_valid_loc(loc, cl) {
            return false;
        }
        self.put(loc, cl);
        true
    }

    pub fn flip(&mut self, loc: i32, cl: Color) {
        let (mut x, mut bounding_disk): (u64, u64);
        let new_disk = 1u64 << loc;
        let mut captured_disk: u64 = 0;

        if cl == Color::Black {
            self.black |= new_disk;
            for dir in 0..8 {
                x = shift(new_disk, dir) & self.white;
                x |= shift(x, dir) & self.white;
                x |= shift(x, dir) & self.white;
                x |= shift(x, dir) & self.white;
                x |= shift(x, dir) & self.white;
                x |= shift(x, dir) & self.white;
                bounding_disk = shift(x, dir) & self.black;

                if bounding_disk != 0 {
                    captured_disk |= x;
                }
            }
            self.black ^= captured_disk;
            self.white ^= captured_disk;
        } else {
            self.white |= new_disk;
            for dir in 0..8 {
                x = shift(new_disk, dir) & self.black;
                x |= shift(x, dir) & self.black;
                x |= shift(x, dir) & self.black;
                x |= shift(x, dir) & self.black;
                x |= shift(x, dir) & self.black;
                x |= shift(x, dir) & self.black;
                bounding_disk = shift(x, dir) & self.white;

                if bounding_disk != 0 {
                    captured_disk |= x;
                }
            }
            self.black ^= captured_disk;
            self.white ^= captured_disk;
        }
    }

    pub fn all_valid_loc(&self, cl: Color) -> u64 {
        let (mine, opp): (u64, u64);
        let mut legal: u64 = 0;

        if cl == Color::Black {
            mine = self.black;
            opp = self.white;
        } else {
            mine = self.white;
            opp = self.black;
        }
        let empty = !(mine | opp);

        for dir in 0..8 {
            let mut x = shift(mine, dir) & opp;
            x |= shift(x, dir) & opp;
            x |= shift(x, dir) & opp;
            x |= shift(x, dir) & opp;
            x |= shift(x, dir) & opp;
            x |= shift(x, dir) & opp;
            legal |= shift(x, dir) & empty;
        }

        legal
    }

    pub fn has_valid_move(&self, cl: Color) -> bool {
        self.all_valid_loc(cl) != 0
    }

    pub fn is_valid_loc(&self, loc: i32, cl: Color) -> bool {
        let mask = 1u64 << loc;
        self.all_valid_loc(cl) & mask != 0
    }

    pub fn count(&self, cl: Color) -> i32 {
        match cl {
            Color::Black => pop_cnt(self.black),
            Color::White => pop_cnt(self.white),
            _ => panic!("wrong color {:#?}", cl),
        }
    }

    pub fn empty_count(&self) -> i32 {
        SIZE * SIZE - pop_cnt(self.black | self.white)
    }

    pub fn is_over(&self) -> bool {
        !(self.has_valid_move(Color::Black) || self.has_valid_move(Color::White))
    }

    pub fn eval(&self, cl: Color) -> i32 {
        let mut bv: i32 = 0;
        let mut wv: i32 = 0;
        let mut cnt: i32;

        cnt = pop_cnt(self.black & 0x8100000000000081);
        bv += cnt * 800;
        cnt = pop_cnt(self.black & 0x0042000000004200);
        bv += cnt * -552;
        cnt = pop_cnt(self.black & 0x0000240000240000);
        bv += cnt * 62;
        cnt = pop_cnt(self.black & 0x0000001818000000);
        bv += cnt * -18;
        cnt = pop_cnt(self.black & 0x4281000000008142);
        bv += cnt * -286;
        cnt = pop_cnt(self.black & 0x2400810000810024);
        bv += cnt * 426;
        cnt = pop_cnt(self.black & 0x1800008181000018);
        bv += cnt * -24;
        cnt = pop_cnt(self.black & 0x0024420000422400);
        bv += cnt * -177;
        cnt = pop_cnt(self.black & 0x0018004242001800);
        bv += cnt * -82;
        cnt = pop_cnt(self.black & 0x0000182424180000);
        bv += cnt * 8;

        cnt = pop_cnt(self.white & 0x8100000000000081);
        wv += cnt * 800;
        cnt = pop_cnt(self.white & 0x0042000000004200);
        wv += cnt * -552;
        cnt = pop_cnt(self.white & 0x0000240000240000);
        wv += cnt * 62;
        cnt = pop_cnt(self.white & 0x0000001818000000);
        wv += cnt * -18;
        cnt = pop_cnt(self.white & 0x4281000000008142);
        wv += cnt * -286;
        cnt = pop_cnt(self.white & 0x2400810000810024);
        wv += cnt * 426;
        cnt = pop_cnt(self.white & 0x1800008181000018);
        wv += cnt * -24;
        cnt = pop_cnt(self.white & 0x0024420000422400);
        wv += cnt * -177;
        cnt = pop_cnt(self.white & 0x0018004242001800);
        wv += cnt * -82;
        cnt = pop_cnt(self.white & 0x0000182424180000);
        wv += cnt * 8;

        match cl {
            Color::Black => bv - wv,
            Color::White => wv - bv,
            _ => panic!("unknown color {:#?}", cl),
        }
    }

    pub fn mobility(&self, cl: Color) -> i32 {
        let allv = self.all_valid_loc(cl);
        pop_cnt(allv)
    }
}
