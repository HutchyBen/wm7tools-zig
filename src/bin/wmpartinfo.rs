use std::env;
use std::error::Error;
use std::io::{self, Write};
use wm7tools::{
    cheap_wchar_to_ascii, partition_type_to_str, FileData, Guid, Mbr, WmpartHdr, WmstoreHdr,
};

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} path/to/image", args[0]);
        return Err("Missing image path argument".into());
    }

    // Load file via blazing-fast zero-copy memory mapping!
    let data = FileData::new(&args[1]).map_err(|e| {
        println!("failed to open file");
        e
    })?;

    if data.len() < 512 {
        println!("failed to read header");
        return Err("File too small for MBR header".into());
    }

    // Lock and buffer stdout once for massive console print speedup!
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    // Zero-copy parse MBR from the in-memory slice
    let mbr_hdr = Mbr::try_from(&data[0..512])?;

    // verify values
    if mbr_hdr.partitions[0].r#type != 0xEF {
        writeln!(
            writer,
            "partition header has type 0x{:02X} instead of 0xEF",
            mbr_hdr.partitions[0].r#type
        )?;
        return Err("Invalid partition header type".into());
    }
    if mbr_hdr.signature[0] != 0x55 || mbr_hdr.signature[1] != 0xAA {
        writeln!(
            writer,
            "file header has signature 0x{:02X},0x{:02X} instead of 0x55,0xAA",
            mbr_hdr.signature[0], mbr_hdr.signature[1]
        )?;
        return Err("Invalid file signature".into());
    }

    if data.len() < 1024 {
        writeln!(writer, "failed to read wmstore header")?;
        return Err("File too small for store header".into());
    }

    // Zero-copy parse store header
    let store_hdr = WmstoreHdr::try_from(&data[512..1024])?;

    // verify magic
    if store_hdr.magic != *b"_wmstore" {
        writeln!(writer, "invalid store magic (expected _wmstore)")?;
        return Err("Invalid store magic".into());
    }

    // print info about the store
    writeln!(writer, "WMSTORE:")?;
    writeln!(writer, "  Name: {}", cheap_wchar_to_ascii(&store_hdr.name))?;
    writeln!(writer, "  GUID: {}", Guid::from(store_hdr.guid))?;
    writeln!(writer, "  Max Partition Count: 0x{:x}", store_hdr.num_sectors)?;
    writeln!(writer, "  Unk2: 0x{:x}", store_hdr.unk2)?;
    writeln!(writer, "  Unk3: 0x{:x}", store_hdr.unk3)?;
    writeln!(writer, "  Timestamp: TODO")?; // store_hdr.timestamp
    writeln!(writer, "  Unk5: 0x{:x}", store_hdr.unk5)?;

    // Parse all partitions directly from the in-memory array with zero seeking overhead!
    let mut offset = 1024;
    let mut parts = 0;
    for _ in 0..store_hdr.num_sectors {
        if offset + 512 > data.len() {
            writeln!(writer, "failed to read wmpart header")?;
            return Err("Unexpected EOF reading partition header".into());
        }
        let part_hdr = WmpartHdr::try_from(&data[offset..offset + 512])?;
        if part_hdr.magic != *b"_wmpart_" {
            break;
        }

        // print info about the partition
        writeln!(writer, "WMPART {}:", parts)?;
        writeln!(writer, "  Name: {}", cheap_wchar_to_ascii(&part_hdr.name))?;
        writeln!(writer, "  Unk1: 0x{:x}", part_hdr.unk1)?;
        writeln!(
            writer,
            "  Offset: 0x{:x} (@ 0x{:x})",
            part_hdr.offset_sector,
            (part_hdr.offset_sector as u64) * 512
        )?;
        writeln!(writer, "  Unk2: 0x{:x}", part_hdr.unk2)?;
        writeln!(
            writer,
            "  Size: 0x{:x} (0x{:x})",
            part_hdr.size_sectors,
            (part_hdr.size_sectors as u64) * 512
        )?;
        writeln!(writer, "  Unk3: 0x{:x}", part_hdr.unk3)?;
        writeln!(writer, "  Timestamp: TODO")?; // part_hdr.timestamp
        writeln!(
            writer,
            "  Partition Type: {} (0x{:x})",
            partition_type_to_str(part_hdr.partition_type),
            part_hdr.partition_type
        )?;
        writeln!(writer, "  Unk5: 0x{:x}", part_hdr.unk5)?;
        writeln!(writer, "  Unk6: 0x{:x}", part_hdr.unk6)?;
        parts += 1;
        offset += 512;
    }

    writer.flush()?;
    Ok(())
}

fn main() {
    if run().is_err() {
        std::process::exit(-1);
    }
}
