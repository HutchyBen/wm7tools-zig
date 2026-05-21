pub mod guid;
pub mod mbr;
pub mod wmnk;
pub mod wmpartitions;

pub use guid::Guid;
pub use mbr::{Mbr, Partition};
pub use wmnk::{
    WmnkHdr, WmnkRomHdr, WmnkTocEntry, WmnkFilesEntry, WmnkCopyEntry, E32Info, E32Rom, O32Rom,
};
pub use wmpartitions::{
    WmstoreHdr, WmpartHdr, WMPART_BOOT, WMPART_DOS3_FAT, WMPART_IMGFS, WMPART_PADDING,
    WMPART_UNK_0X1B, WMPART_UNK_0X29, WMPART_XIP_RAM, WMPART_XIP_ROM,
};

/// Cheap UTF-16 representation to ASCII conversion mimicking the original C behavior.
pub fn cheap_wchar_to_ascii(chars: &[i16]) -> String {
    let mut s = String::new();
    for &c in chars {
        if c == 0 {
            break;
        }
        // Mimic (char)*chars cast in C
        s.push((c as u8) as char);
    }
    s
}

/// Helper to format GUID identically to original C format print_guid.
pub fn format_guid(guid_bytes: [u8; 16]) -> String {
    let guid = Guid::from_bytes(guid_bytes);
    format!(
        "{:08x}-{:02x}-{:02x}-{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        guid.p1, guid.p2, guid.p3, guid.p4,
        guid.p5[0], guid.p5[1], guid.p5[2], guid.p5[3], guid.p5[4], guid.p5[5]
    )
}

/// Translate partition type ID to its string representation.
pub fn partition_type_to_str(t: u32) -> &'static str {
    match t {
        WMPART_DOS3_FAT => "DOS3/FAT",
        WMPART_BOOT => "BOOT",
        WMPART_XIP_ROM => "XIP from ROM",
        WMPART_XIP_RAM => "XIP from RAM",
        WMPART_IMGFS => "IMGFS",
        WMPART_PADDING => "Padding",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cheap_wchar_to_ascii() {
        let name = [
            b'N' as i16, b'K' as i16, 0, 0,
        ];
        assert_eq!(cheap_wchar_to_ascii(&name), "NK");
    }

    #[test]
    fn test_format_guid() {
        let bytes = [
            0x01, 0x02, 0x03, 0x04,
            0x05, 0x06,
            0x07, 0x08,
            0x09, 0x0a,
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
        ];
        let formatted = format_guid(bytes);
        assert_eq!(formatted, "04030201-605-807-a09-0b0c0d0e0f10");
    }

    #[test]
    fn test_partition_type_to_str() {
        assert_eq!(partition_type_to_str(0x04), "DOS3/FAT");
        assert_eq!(partition_type_to_str(0x23), "XIP from RAM");
        assert_eq!(partition_type_to_str(0x99), "Unknown");
    }
}

