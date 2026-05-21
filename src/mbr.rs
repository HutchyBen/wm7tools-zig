#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Partition {
    pub attributes: u8,
    pub start_chs: [u8; 3],
    pub r#type: u8,
    pub end_chs: [u8; 3],
    pub lba_start: u32,
    pub num_sectors: u32,
}

impl From<[u8; 16]> for Partition {
    fn from(bytes: [u8; 16]) -> Self {
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

impl TryFrom<&[u8]> for Partition {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 16 {
            return Err("Partition structure requires exactly 16 bytes");
        }
        let arr: [u8; 16] = bytes[0..16].try_into().unwrap();
        Ok(Self::from(arr))
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

impl From<[u8; 512]> for Mbr {
    fn from(bytes: [u8; 512]) -> Self {
        let mut partitions = [
            Partition::from([0; 16]),
            Partition::from([0; 16]),
            Partition::from([0; 16]),
            Partition::from([0; 16]),
        ];
        for i in 0..4 {
            let offset = 0x1B8 + 4 + 2 + i * 16;
            let part_bytes: [u8; 16] = bytes[offset..offset + 16].try_into().unwrap();
            partitions[i] = Partition::from(part_bytes);
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

impl TryFrom<&[u8]> for Mbr {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 512 {
            return Err("MBR structure requires exactly 512 bytes");
        }
        let arr: [u8; 512] = bytes[0..512].try_into().unwrap();
        Ok(Self::from(arr))
    }
}
