#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <wchar.h>

#include "../common/mbr.h"
#include "../common/wmpartitions.h"
#include "../common/wmnk.h"

#define SECTOR_SIZE 0x200

// TODO: make this defineable
#define OUT_FOLDER "XIP/"

uint32_t base_address = 0x80500000;

void write_buf_to_file(const char *filename, void *buffer, size_t length)
{
    FILE *fp = fopen(filename, "wb");
    if (fp == NULL) {
        printf("failed to write '%s'\n", filename);
        return;
    }
    fwrite(buffer, 1, length, fp);
    fclose(fp);
}

// gets the start offset and length of the NK image
// this detects whether it's a WMPART partition structure or 
int get_nk_start_and_size(FILE *fp, long *offset, long *size)
{
    uint8_t tmp_buf[0x1000]; // enough to fit the NK header itself or the first 6 wmparts
    int r = 0;

    r = fread(tmp_buf, 1, sizeof(tmp_buf), fp);
    if (r != sizeof(tmp_buf))
        return -2; // can't read the entire thing

    // check if this is a raw NK image
    wmnk_hdr_t *nk_tmp = (wmnk_hdr_t *)tmp_buf;
    if (memcmp(nk_tmp->magic, "ECEC", 4) == 0)
    {
        fseek(fp, 0, SEEK_END);
        long filesize = ftell(fp);
        fseek(fp, 0, SEEK_SET);
        if (offset != NULL)
            *offset = 0;
        if (size != NULL)
            *size = filesize;
        return 0;
    }

    // check if this is a WMSTORE structure
    mbr_t *mbr_tmp = (mbr_t *)tmp_buf;
    if (mbr_tmp->signature[0] == 0x55 && mbr_tmp->signature[1] == 0xAA) {
        // well it's an MBR let's check for it being a _wmstore
        wmstore_hdr_t *wmstore_tmp = (wmstore_hdr_t *)(tmp_buf + sizeof(mbr_t));
        if (memcmp(wmstore_tmp->magic, "_wmstore", 8) == 0) {
            // ok iterate through the partitions we would have
            wmpart_hdr_t *partitions = (wmpart_hdr_t *)(tmp_buf + sizeof(mbr_t) + sizeof(wmstore_hdr_t));
            wmpart_hdr_t *found_nk = NULL;
            for (int i = 0; i < 6 && i < partitions->size_sectors; i++) {
                if (memcmp(partitions[i].magic, "_wmpart_", 8) != 0)
                    break;
                static const char widenk[6] = { 'N', 0, 'K', 0, 0, 0 };
                if (partitions[i].partition_type == wmpart_xip_ram &&
                    memcmp(partitions[i].name, widenk, sizeof(widenk)) == 0) {
                    found_nk = &partitions[i];
                    break;
                }
            }
            if (found_nk == NULL)
                return -3; // couldn't find nk in wmpart
            if (offset != NULL)
                *offset = found_nk->offset_sector * SECTOR_SIZE;
            if (size != NULL)
                *size = found_nk->size_sectors * SECTOR_SIZE;
            return 0;
        }
    }

    return -1; // couldn't detect format
}

int main(int argc, const char **argv)
{   
    if (argc < 2) {
        printf("usage: %s path/to/image\n", argv[0]);
        return -1;
    }

    // try to open the file
    FILE *fp = fopen(argv[1], "rb");
    if (fp == NULL) {
        printf("failed to open file\n");
        return -1;
    }

    // get the offset of the NK partition
    long nk_offset = 0;
    long nk_size = 0;
    int fnk = get_nk_start_and_size(fp, &nk_offset, &nk_size);
    if (fnk < 0) {
        printf("failed to find NK header\n");
        fclose(fp);
        return -1;
    }
    
    // load the entire nk partition
    void *nk_buf = malloc(nk_size);
    if (nk_buf == NULL) {
        printf("failed to allocate buffer for NK data\n");
        fclose(fp);
        return -1;
    }
    fseek(fp, nk_offset, SEEK_SET);
    if (fread(nk_buf, 1, nk_size, fp) != nk_size) {
        printf("failed to read NK data from file\n");
        fclose(fp);
        free(nk_buf);
        return -1;
    }
    fclose(fp);
    uint8_t *nk = nk_buf;

    wmnk_hdr_t *nkhdr = (wmnk_hdr_t *)(nk + 0x0);
    //guess the base address from this
    base_address = nkhdr->rom_hdr_virt - nkhdr->rom_hdr_real;
    printf("Base: 0x%08x\n", base_address);

    wmnk_rom_hdr_t *romhdr = (wmnk_rom_hdr_t *)(nk + nkhdr->rom_hdr_real);
    if (romhdr->physfirst != base_address) {
        printf("Actually, 0x%08x? Something's wrong, but trying our best...\n", romhdr->physfirst);
        base_address = romhdr->physfirst;
    }
    printf("DLLs: 0x%08x-0x%08x\n", romhdr->dllfirst, romhdr->dlllast);
    printf("Phys: 0x%08x-0x%08x\n", romhdr->physfirst, romhdr->physlast);
    printf("RAM:  0x%08x-0x%08x (Free @ 0x%08x)\n", romhdr->ulRAMStart, romhdr->ulRAMEnd, romhdr->ulRAMFree);
    printf("Modules: %i, Files: %i, Copyentries: %i\n", romhdr->nummods, romhdr->numfiles, romhdr->ulCopyEntries);

    // write the rom header to file
    write_buf_to_file(OUT_FOLDER "/romhdr.bin", romhdr, sizeof(wmnk_rom_hdr_t));
    
#define REALPTR(x) (x - base_address)

    // iterate through all the modules
    uint8_t *entryptr = nk + nkhdr->rom_hdr_real + sizeof(wmnk_rom_hdr_t);
    wmnk_toc_entry_t *tocentries = (wmnk_toc_entry_t *)entryptr;
    for (int i = 0; i < romhdr->nummods; i++) {
        printf("0x%08x = Module: %s (size: 0x%x, attr: 0x%x)\n", tocentries[i].ulLoadOffset, (char*)(nk + REALPTR(tocentries[i].lpszFileName)),
            tocentries[i].nFileSize, tocentries[i].dwFileAttributes);
        entryptr += sizeof(wmnk_toc_entry_t);
    }
    // iterate through all the files
    wmnk_files_entry_t *filesentries = (wmnk_files_entry_t *)entryptr;
    for (int i = 0; i < romhdr->numfiles; i++) {
        printf("0x%08x = File: %s (size: 0x%x, comp: 0x%x, attr: 0x%x)\n", filesentries[i].ulLoadOffset, (char*)(nk + REALPTR(filesentries[i].lpszFileName)),
            filesentries[i].nRealFileSize, filesentries[i].nCompFileSize, filesentries[i].dwFileAttributes);
        entryptr += sizeof(wmnk_files_entry_t);

        if (filesentries[i].nRealFileSize == filesentries[i].nCompFileSize) {
            char filenamebuf[1024];
            sprintf(filenamebuf, "%s/%s", OUT_FOLDER, (char*)(nk + REALPTR(filesentries[i].lpszFileName)));
            write_buf_to_file(filenamebuf, nk + REALPTR(filesentries[i].ulLoadOffset), filesentries[i].nRealFileSize);
        }
    }

#undef REALPTR

    return 0;
}
