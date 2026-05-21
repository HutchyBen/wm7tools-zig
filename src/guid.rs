#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Guid {
    pub p1: u32,
    pub p2: u16,
    pub p3: u16,
    pub p4: u16,
    pub p5: [u8; 6],
}

impl Guid {
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self {
            p1: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            p2: u16::from_le_bytes(bytes[4..6].try_into().unwrap()),
            p3: u16::from_le_bytes(bytes[6..8].try_into().unwrap()),
            p4: u16::from_le_bytes(bytes[8..10].try_into().unwrap()),
            p5: bytes[10..16].try_into().unwrap(),
        }
    }
}
