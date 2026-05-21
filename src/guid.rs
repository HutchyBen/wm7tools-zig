use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Guid {
    pub p1: u32,
    pub p2: u16,
    pub p3: u16,
    pub p4: u16,
    pub p5: [u8; 6],
}

impl From<[u8; 16]> for Guid {
    fn from(bytes: [u8; 16]) -> Self {
        Self {
            p1: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            p2: u16::from_le_bytes(bytes[4..6].try_into().unwrap()),
            p3: u16::from_le_bytes(bytes[6..8].try_into().unwrap()),
            p4: u16::from_le_bytes(bytes[8..10].try_into().unwrap()),
            p5: bytes[10..16].try_into().unwrap(),
        }
    }
}

impl TryFrom<&[u8]> for Guid {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 16 {
            return Err("GUID requires exactly 16 bytes");
        }
        let arr: [u8; 16] = bytes[0..16].try_into().unwrap();
        Ok(Self::from(arr))
    }
}

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Formats identically to the original C-style format_guid string
        write!(
            f,
            "{:08x}-{:02x}-{:02x}-{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.p1, self.p2, self.p3, self.p4,
            self.p5[0], self.p5[1], self.p5[2], self.p5[3], self.p5[4], self.p5[5]
        )
    }
}
