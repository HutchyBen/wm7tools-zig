use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use wm7tools::{FileData, WmnkFilesEntry, WmnkRomHdr, WmnkTocEntry, WMPART_XIP_RAM};

const OUT_FOLDER: &str = "XIP";

fn write_buf_to_file<P: AsRef<Path>>(filename: P, buffer: &[u8]) {
    let filename = filename.as_ref();
    if let Some(parent) = filename.parent() {
        if !parent.exists() {
            let _ = fs::create_dir_all(parent);
        }
    }
    match File::create(filename) {
        Ok(mut file) => {
            if file.write_all(buffer).is_err() {
                println!("failed to write '{}'", filename.display());
            }
        }
        Err(_) => {
            println!("failed to write '{}'", filename.display());
        }
    }
}

fn get_c_string(nk: &[u8], offset: usize) -> String {
    if offset >= nk.len() {
        return String::new();
    }
    let slice = &nk[offset..];
    let len = slice.iter().position(|&b| b == 0).unwrap_or(slice.len());
    String::from_utf8_lossy(&slice[..len]).into_owned()
}

fn get_nk_start_and_size_in_memory(
    data: &[u8],
    offset: &mut u64,
    size: &mut u64,
) -> Result<(), Box<dyn Error>> {
    if data.len() < 0x1000 {
        return Err("File too small for header scanning".into());
    }
    let tmp_buf = &data[0..0x1000];

    // check if this is a raw NK image
    if &tmp_buf[0x40..0x44] == b"ECEC" {
        *offset = 0;
        *size = data.len() as u64;
        return Ok(());
    }

    // check if this is a WMSTORE structure
    if tmp_buf[510] == 0x55 && tmp_buf[511] == 0xAA {
        if &tmp_buf[512..520] == b"_wmstore" {
            let mut found_nk = None;
            let first_partition_size_sectors = u32::from_le_bytes(
                tmp_buf[1024 + 84..1024 + 88].try_into().unwrap()
            ) as usize;

            for i in 0..6 {
                if i >= first_partition_size_sectors {
                    break;
                }
                let part_offset = 1024 + i * 512;
                if part_offset + 512 > tmp_buf.len() {
                    break;
                }
                let part_magic = &tmp_buf[part_offset..part_offset + 8];
                if part_magic != b"_wmpart_" {
                    break;
                }
                let part_type = u32::from_le_bytes(
                    tmp_buf[part_offset + 100..part_offset + 104].try_into().unwrap()
                );
                let part_name = &tmp_buf[part_offset + 8..part_offset + 14];

                if part_type == WMPART_XIP_RAM && part_name == &[b'N', 0, b'K', 0, 0, 0] {
                    let offset_sector = u32::from_le_bytes(
                        tmp_buf[part_offset + 76..part_offset + 80].try_into().unwrap()
                    );
                    let size_sectors = u32::from_le_bytes(
                        tmp_buf[part_offset + 84..part_offset + 88].try_into().unwrap()
                    );
                    found_nk = Some((offset_sector, size_sectors));
                    break;
                }
            }

            if let Some((offset_sector, size_sectors)) = found_nk {
                *offset = (offset_sector as u64) * 0x200;
                *size = (size_sectors as u64) * 0x200;
                return Ok(());
            } else {
                return Err("Failed to find NK partition in WMPART".into());
            }
        }
    }

    Err("Could not detect image format".into())
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} path/to/image", args[0]);
        return Err("Missing image path argument".into());
    }

    // Load the file using blazing-fast zero-copy memory-mapping!
    let data = FileData::new(&args[1]).map_err(|e| {
        println!("failed to open file");
        e
    })?;

    // Lock and buffer stdout once for massive console print speedup!
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    // get the offset of the NK partition
    let mut nk_offset = 0;
    let mut nk_size = 0;
    get_nk_start_and_size_in_memory(&data, &mut nk_offset, &mut nk_size).map_err(|e| {
        println!("failed to find NK header");
        e
    })?;

    // Slice directly into the loaded data buffer - Zero heap allocation copy!
    let nk_start = nk_offset as usize;
    let nk_end = nk_start + nk_size as usize;
    if nk_end > data.len() {
        writeln!(writer, "failed to read NK data from file")?;
        return Err("NK partition bounds out of file range".into());
    }
    let nk = &data[nk_start..nk_end];

    // guess the base address from this
    let rom_hdr_virt = u32::from_le_bytes(nk[0x44..0x48].try_into().unwrap());
    let rom_hdr_real = u32::from_le_bytes(nk[0x48..0x4C].try_into().unwrap());
    let mut base_address = rom_hdr_virt.wrapping_sub(rom_hdr_real);
    writeln!(writer, "Base: 0x{:08x}", base_address)?;

    let romhdr_offset = rom_hdr_real as usize;
    if romhdr_offset + 84 > nk.len() {
        writeln!(writer, "Actually, 0x00000000? Something's wrong, but trying our best...")?;
        return Err("Invalid ROM header offset".into());
    }

    let romhdr_bytes: [u8; 84] = nk[romhdr_offset..romhdr_offset + 84].try_into().unwrap();
    let romhdr = WmnkRomHdr::try_from(&romhdr_bytes[..])?;

    if romhdr.physfirst != base_address {
        writeln!(
            writer,
            "Actually, 0x{:08x}? Something's wrong, but trying our best...",
            romhdr.physfirst
        )?;
        base_address = romhdr.physfirst;
    }
    writeln!(writer, "DLLs: 0x{:08x}-0x{:08x}", romhdr.dllfirst, romhdr.dlllast)?;
    writeln!(writer, "Phys: 0x{:08x}-0x{:08x}", romhdr.physfirst, romhdr.physlast)?;
    writeln!(
        writer,
        "RAM:  0x{:08x}-0x{:08x} (Free @ 0x{:08x})",
        romhdr.ul_ram_start, romhdr.ul_ram_end, romhdr.ul_ram_free
    )?;
    writeln!(
        writer,
        "Modules: {}, Files: {}, Copyentries: {}",
        romhdr.nummods, romhdr.numfiles, romhdr.ul_copy_entries
    )?;

    // write the rom header to file
    write_buf_to_file(
        format!("{}/romhdr.bin", OUT_FOLDER),
        &nk[romhdr_offset..romhdr_offset + 84],
    );

    let mut entry_offset = romhdr_offset + 84;

    // iterate through all the modules
    for _ in 0..romhdr.nummods {
        if entry_offset + 32 > nk.len() {
            break;
        }
        let entry_bytes: [u8; 32] = nk[entry_offset..entry_offset + 32].try_into().unwrap();
        let toc = WmnkTocEntry::try_from(&entry_bytes[..])?;

        let name_offset = toc.lpsz_file_name.wrapping_sub(base_address) as usize;
        let name = get_c_string(nk, name_offset);

        writeln!(
            writer,
            "0x{:08x} = Module: {} (size: 0x{:x}, attr: 0x{:x})",
            toc.ul_load_offset, name, toc.n_file_size, toc.dw_file_attributes
        )?;

        entry_offset += 32;
    }

    // iterate through all the files
    for _ in 0..romhdr.numfiles {
        if entry_offset + 28 > nk.len() {
            break;
        }
        let entry_bytes: [u8; 28] = nk[entry_offset..entry_offset + 28].try_into().unwrap();
        let file_entry = WmnkFilesEntry::try_from(&entry_bytes[..])?;

        let name_offset = file_entry.lpsz_file_name.wrapping_sub(base_address) as usize;
        let name = get_c_string(nk, name_offset);

        writeln!(
            writer,
            "0x{:08x} = File: {} (size: 0x{:x}, comp: 0x{:x}, attr: 0x{:x})",
            file_entry.ul_load_offset,
            name,
            file_entry.n_real_file_size,
            file_entry.n_comp_file_size,
            file_entry.dw_file_attributes
        )?;

        if file_entry.n_real_file_size == file_entry.n_comp_file_size {
            let file_data_offset = file_entry.ul_load_offset.wrapping_sub(base_address) as usize;
            let file_data_len = file_entry.n_real_file_size as usize;
            if file_data_offset + file_data_len <= nk.len() {
                let file_data = &nk[file_data_offset..file_data_offset + file_data_len];
                let filepath = format!("{}/{}", OUT_FOLDER, name);
                write_buf_to_file(&filepath, file_data);
            }
        }

        entry_offset += 28;
    }

    // Explicitly flush the BufWriter to ensure all outputs are written before exit
    writer.flush()?;

    Ok(())
}

fn main() {
    if run().is_err() {
        std::process::exit(-1);
    }
}
