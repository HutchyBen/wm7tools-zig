#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WmnkHdr {
    pub boot_code: [u8; 0x40],
    pub magic: [u8; 4], // 'ECEC'
    pub rom_hdr_virt: u32,
    pub rom_hdr_real: u32,
    pub padding: [u8; 0xFB4],
}

impl From<[u8; 0x1000]> for WmnkHdr {
    fn from(bytes: [u8; 0x1000]) -> Self {
        Self {
            boot_code: bytes[0..0x40].try_into().unwrap(),
            magic: bytes[0x40..0x44].try_into().unwrap(),
            rom_hdr_virt: u32::from_le_bytes(bytes[0x44..0x48].try_into().unwrap()),
            rom_hdr_real: u32::from_le_bytes(bytes[0x48..0x4C].try_into().unwrap()),
            padding: bytes[0x4C..0x1000].try_into().unwrap(),
        }
    }
}

impl TryFrom<&[u8]> for WmnkHdr {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 0x1000 {
            return Err("WmnkHdr requires exactly 0x1000 bytes");
        }
        let arr: [u8; 0x1000] = bytes[0..0x1000].try_into().unwrap();
        Ok(Self::from(arr))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WmnkRomHdr {
    pub dllfirst: u32,
    pub dlllast: u32,
    pub physfirst: u32,
    pub physlast: u32,
    pub nummods: u32,
    pub ul_ram_start: u32,
    pub ul_ram_free: u32,
    pub ul_ram_end: u32,
    pub ul_copy_entries: u32,
    pub ul_copy_offset: u32,
    pub ul_profile_len: u32,
    pub ul_profile_offset: u32,
    pub numfiles: u32,
    pub ul_kernel_flags: u32,
    pub ul_fs_ram_percent: u32,
    pub ul_drivglob_start: u32,
    pub ul_drivglob_len: u32,
    pub us_cpu_type: u16,
    pub us_misc_flags: u16,
    pub p_extensions: u32,
    pub ul_tracking_start: u32,
    pub ul_tracking_len: u32,
}

impl From<[u8; 84]> for WmnkRomHdr {
    fn from(bytes: [u8; 84]) -> Self {
        Self {
            dllfirst: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            dlllast: u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
            physfirst: u32::from_le_bytes(bytes[8..12].try_into().unwrap()),
            physlast: u32::from_le_bytes(bytes[12..16].try_into().unwrap()),
            nummods: u32::from_le_bytes(bytes[16..20].try_into().unwrap()),
            ul_ram_start: u32::from_le_bytes(bytes[20..24].try_into().unwrap()),
            ul_ram_free: u32::from_le_bytes(bytes[24..28].try_into().unwrap()),
            ul_ram_end: u32::from_le_bytes(bytes[28..32].try_into().unwrap()),
            ul_copy_entries: u32::from_le_bytes(bytes[32..36].try_into().unwrap()),
            ul_copy_offset: u32::from_le_bytes(bytes[36..40].try_into().unwrap()),
            ul_profile_len: u32::from_le_bytes(bytes[40..44].try_into().unwrap()),
            ul_profile_offset: u32::from_le_bytes(bytes[44..48].try_into().unwrap()),
            numfiles: u32::from_le_bytes(bytes[48..52].try_into().unwrap()),
            ul_kernel_flags: u32::from_le_bytes(bytes[52..56].try_into().unwrap()),
            ul_fs_ram_percent: u32::from_le_bytes(bytes[56..60].try_into().unwrap()),
            ul_drivglob_start: u32::from_le_bytes(bytes[60..64].try_into().unwrap()),
            ul_drivglob_len: u32::from_le_bytes(bytes[64..68].try_into().unwrap()),
            us_cpu_type: u16::from_le_bytes(bytes[68..70].try_into().unwrap()),
            us_misc_flags: u16::from_le_bytes(bytes[70..72].try_into().unwrap()),
            p_extensions: u32::from_le_bytes(bytes[72..76].try_into().unwrap()),
            ul_tracking_start: u32::from_le_bytes(bytes[76..80].try_into().unwrap()),
            ul_tracking_len: u32::from_le_bytes(bytes[80..84].try_into().unwrap()),
        }
    }
}

impl TryFrom<&[u8]> for WmnkRomHdr {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 84 {
            return Err("WmnkRomHdr requires exactly 84 bytes");
        }
        let arr: [u8; 84] = bytes[0..84].try_into().unwrap();
        Ok(Self::from(arr))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WmnkTocEntry {
    pub dw_file_attributes: u32,
    pub ft_time: u64,
    pub n_file_size: u32,
    pub lpsz_file_name: u32,
    pub ul_e32_offset: u32,
    pub ul_o32_offset: u32,
    pub ul_load_offset: u32,
}

impl From<[u8; 32]> for WmnkTocEntry {
    fn from(bytes: [u8; 32]) -> Self {
        Self {
            dw_file_attributes: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            ft_time: u64::from_le_bytes(bytes[4..12].try_into().unwrap()),
            n_file_size: u32::from_le_bytes(bytes[12..16].try_into().unwrap()),
            lpsz_file_name: u32::from_le_bytes(bytes[16..20].try_into().unwrap()),
            ul_e32_offset: u32::from_le_bytes(bytes[20..24].try_into().unwrap()),
            ul_o32_offset: u32::from_le_bytes(bytes[24..28].try_into().unwrap()),
            ul_load_offset: u32::from_le_bytes(bytes[28..32].try_into().unwrap()),
        }
    }
}

impl TryFrom<&[u8]> for WmnkTocEntry {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 32 {
            return Err("WmnkTocEntry requires exactly 32 bytes");
        }
        let arr: [u8; 32] = bytes[0..32].try_into().unwrap();
        Ok(Self::from(arr))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WmnkFilesEntry {
    pub dw_file_attributes: u32,
    pub ft_time: u64,
    pub n_real_file_size: u32,
    pub n_comp_file_size: u32,
    pub lpsz_file_name: u32,
    pub ul_load_offset: u32,
}

impl From<[u8; 28]> for WmnkFilesEntry {
    fn from(bytes: [u8; 28]) -> Self {
        Self {
            dw_file_attributes: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            ft_time: u64::from_le_bytes(bytes[4..12].try_into().unwrap()),
            n_real_file_size: u32::from_le_bytes(bytes[12..16].try_into().unwrap()),
            n_comp_file_size: u32::from_le_bytes(bytes[16..20].try_into().unwrap()),
            lpsz_file_name: u32::from_le_bytes(bytes[20..24].try_into().unwrap()),
            ul_load_offset: u32::from_le_bytes(bytes[24..28].try_into().unwrap()),
        }
    }
}

impl TryFrom<&[u8]> for WmnkFilesEntry {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 28 {
            return Err("WmnkFilesEntry requires exactly 28 bytes");
        }
        let arr: [u8; 28] = bytes[0..28].try_into().unwrap();
        Ok(Self::from(arr))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WmnkCopyEntry {
    pub ul_source: u32,
    pub ul_dest: u32,
    pub ul_copy_len: u32,
    pub ul_dest_len: u32,
}

impl From<[u8; 16]> for WmnkCopyEntry {
    fn from(bytes: [u8; 16]) -> Self {
        Self {
            ul_source: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            ul_dest: u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
            ul_copy_len: u32::from_le_bytes(bytes[8..12].try_into().unwrap()),
            ul_dest_len: u32::from_le_bytes(bytes[12..16].try_into().unwrap()),
        }
    }
}

impl TryFrom<&[u8]> for WmnkCopyEntry {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 16 {
            return Err("WmnkCopyEntry requires exactly 16 bytes");
        }
        let arr: [u8; 16] = bytes[0..16].try_into().unwrap();
        Ok(Self::from(arr))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct E32Info {
    pub rva: u32,
    pub size: u32,
}

impl From<[u8; 8]> for E32Info {
    fn from(bytes: [u8; 8]) -> Self {
        Self {
            rva: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            size: u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
        }
    }
}

impl TryFrom<&[u8]> for E32Info {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 8 {
            return Err("E32Info requires exactly 8 bytes");
        }
        let arr: [u8; 8] = bytes[0..8].try_into().unwrap();
        Ok(Self::from(arr))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct E32Rom {
    pub objcnt: u16,
    pub imageflags: u16,
    pub entryrva: u32,
    pub vbase: u32,
    pub subsysmajor: u16,
    pub subsysminor: u16,
    pub stackmax: u32,
    pub vsize: u32,
    pub sect14rva: u32,
    pub sect14size: u32,
    pub unit: [E32Info; 9],
    pub subsys: u16,
    pub unknown: u16,
}

impl From<[u8; 108]> for E32Rom {
    fn from(bytes: [u8; 108]) -> Self {
        let mut unit = [E32Info { rva: 0, size: 0 }; 9];
        for i in 0..9 {
            let offset = 28 + i * 8;
            let unit_bytes: [u8; 8] = bytes[offset..offset + 8].try_into().unwrap();
            unit[i] = E32Info::from(unit_bytes);
        }
        Self {
            objcnt: u16::from_le_bytes(bytes[0..2].try_into().unwrap()),
            imageflags: u16::from_le_bytes(bytes[2..4].try_into().unwrap()),
            entryrva: u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
            vbase: u32::from_le_bytes(bytes[8..12].try_into().unwrap()),
            subsysmajor: u16::from_le_bytes(bytes[12..14].try_into().unwrap()),
            subsysminor: u16::from_le_bytes(bytes[14..16].try_into().unwrap()),
            stackmax: u32::from_le_bytes(bytes[16..20].try_into().unwrap()),
            vsize: u32::from_le_bytes(bytes[20..24].try_into().unwrap()),
            sect14rva: u32::from_le_bytes(bytes[24..28].try_into().unwrap()),
            sect14size: u32::from_le_bytes(bytes[28..32].try_into().unwrap()),
            unit,
            subsys: u16::from_le_bytes(bytes[104..106].try_into().unwrap()),
            unknown: u16::from_le_bytes(bytes[106..108].try_into().unwrap()),
        }
    }
}

impl TryFrom<&[u8]> for E32Rom {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 108 {
            return Err("E32Rom requires exactly 108 bytes");
        }
        let arr: [u8; 108] = bytes[0..108].try_into().unwrap();
        Ok(Self::from(arr))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct O32Rom {
    pub vsize: u32,
    pub rva: u32,
    pub psize: u32,
    pub dataptr: u32,
    pub realaddr: u32,
    pub flags: u32,
}

impl From<[u8; 24]> for O32Rom {
    fn from(bytes: [u8; 24]) -> Self {
        Self {
            vsize: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            rva: u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
            psize: u32::from_le_bytes(bytes[8..12].try_into().unwrap()),
            dataptr: u32::from_le_bytes(bytes[12..16].try_into().unwrap()),
            realaddr: u32::from_le_bytes(bytes[16..20].try_into().unwrap()),
            flags: u32::from_le_bytes(bytes[20..24].try_into().unwrap()),
        }
    }
}

impl TryFrom<&[u8]> for O32Rom {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 24 {
            return Err("O32Rom requires exactly 24 bytes");
        }
        let arr: [u8; 24] = bytes[0..24].try_into().unwrap();
        Ok(Self::from(arr))
    }
}
