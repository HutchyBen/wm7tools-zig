#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Partition {
    pub attributes: u8,
    pub start_chs: [u8; 3],
    pub r#type: u8,
    pub end_chs: [u8; 3],
    pub lba_start: u32,
    pub num_sectors: u32,
}

impl Partition {
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self {
            attributes: bytes[0],
            start_chs: bytes[1..4].try_into().unwrap(),
            r#type: bytes[4],
            end_chs: bytes[5..8].try_into().unwrap(),
            lba_start: u32::from_le_bytes(bytes[8..12].try_into().unwrap()),
            num_sectors: u32::from_le_bytes(bytes[12..16].try_into().unwrap()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mbr {
    pub bootstrap: [u8; 0x1B8],
    pub disk_id: [u8; 4],
    pub reserved: [u8; 2],
    pub partitions: [Partition; 4],
    pub signature: [u8; 2],
}

impl Mbr {
    pub fn from_bytes(bytes: [u8; 512]) -> Self {
        let mut partitions = [
            Partition::from_bytes([0; 16]),
            Partition::from_bytes([0; 16]),
            Partition::from_bytes([0; 16]),
            Partition::from_bytes([0; 16]),
        ];
        for i in 0..4 {
            let offset = 0x1B8 + 4 + 2 + i * 16;
            partitions[i] = Partition::from_bytes(bytes[offset..offset + 16].try_into().unwrap());
        }
        Self {
            bootstrap: bytes[0..0x1B8].try_into().unwrap(),
            disk_id: bytes[0x1B8..0x1B8 + 4].try_into().unwrap(),
            reserved: bytes[0x1B8 + 4..0x1B8 + 6].try_into().unwrap(),
            partitions,
            signature: bytes[510..512].try_into().unwrap(),
        }
    }
}
