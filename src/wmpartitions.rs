pub const WMPART_DOS3_FAT: u32 = 0x04;
pub const WMPART_UNK_0X1B: u32 = 0x1B;
pub const WMPART_BOOT: u32 = 0x20;
pub const WMPART_XIP_ROM: u32 = 0x22;
pub const WMPART_XIP_RAM: u32 = 0x23;
pub const WMPART_IMGFS: u32 = 0x25;
pub const WMPART_UNK_0X29: u32 = 0x29;
pub const WMPART_PADDING: u32 = 0x2A;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WmstoreHdr {
    pub magic: [u8; 8],
    pub guid: [u8; 16],
    pub name: [i16; 0x20], // wchar_t (0x40 bytes)
    pub num_sectors: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub timestamp: u64,
    pub unk5: u32,
    pub padding: [u8; 0x190],
}

impl WmstoreHdr {
    pub fn from_bytes(bytes: [u8; 512]) -> Self {
        let mut name = [0i16; 0x20];
        for i in 0..0x20 {
            let offset = 8 + 16 + i * 2;
            name[i] = i16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap());
        }
        Self {
            magic: bytes[0..8].try_into().unwrap(),
            guid: bytes[8..24].try_into().unwrap(),
            name,
            num_sectors: u32::from_le_bytes(bytes[88..92].try_into().unwrap()),
            unk2: u32::from_le_bytes(bytes[92..96].try_into().unwrap()),
            unk3: u32::from_le_bytes(bytes[96..100].try_into().unwrap()),
            timestamp: u64::from_le_bytes(bytes[100..108].try_into().unwrap()),
            unk5: u32::from_le_bytes(bytes[108..112].try_into().unwrap()),
            padding: bytes[112..512].try_into().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WmpartHdr {
    pub magic: [u8; 8],
    pub name: [i16; 0x20], // 0x40 bytes
    pub unk1: u32,
    pub offset_sector: u32,
    pub unk2: u32,
    pub size_sectors: u32,
    pub unk3: u32,
    pub timestamp: u64,
    pub partition_type: u32,
    pub unk5: u32,
    pub unk6: u32,
    pub padding: [u8; 0x190],
}

impl WmpartHdr {
    pub fn from_bytes(bytes: [u8; 512]) -> Self {
        let mut name = [0i16; 0x20];
        for i in 0..0x20 {
            let offset = 8 + i * 2;
            name[i] = i16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap());
        }
        Self {
            magic: bytes[0..8].try_into().unwrap(),
            name,
            unk1: u32::from_le_bytes(bytes[72..76].try_into().unwrap()),
            offset_sector: u32::from_le_bytes(bytes[76..80].try_into().unwrap()),
            unk2: u32::from_le_bytes(bytes[80..84].try_into().unwrap()),
            size_sectors: u32::from_le_bytes(bytes[84..88].try_into().unwrap()),
            unk3: u32::from_le_bytes(bytes[88..92].try_into().unwrap()),
            timestamp: u64::from_le_bytes(bytes[92..100].try_into().unwrap()),
            partition_type: u32::from_le_bytes(bytes[100..104].try_into().unwrap()),
            unk5: u32::from_le_bytes(bytes[104..108].try_into().unwrap()),
            unk6: u32::from_le_bytes(bytes[108..112].try_into().unwrap()),
            padding: bytes[112..512].try_into().unwrap(),
        }
    }
}
