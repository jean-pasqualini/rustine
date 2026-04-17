pub fn shift_left(i: u32) -> u32 {
    i << 1
}

pub fn shift_rigtt(i: u32) -> u32 {
    i >> 1
}

pub fn mask(i: u32) -> u32 {
    i & 1
}

pub fn big_endian(i: u16) -> u16 {
    i.to_be()
}

pub fn little_endian(i: u16) -> u16 {
    i.to_le()
}

pub fn low(i: u16) -> u8 {
    (i & 0xFF) as u8
}

pub fn high(i: u16) -> u8 {
    ((i >> 8) & 0xFF) as u8
}