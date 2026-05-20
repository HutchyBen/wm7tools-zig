#ifndef _WMNK_H_
#define _WMNK_H_

#include <stdint.h>

typedef struct __attribute__((packed)) _wmnk_hdr {
    uint8_t boot_code[0x40]; // just an ARM jump instruction to the entrypoint, rest is NULL
    char magic[4]; // 'ECEC' ('CECE' byteswapped)
    uint32_t rom_hdr_virt; // address of the ROM header in virtual memory space
    uint32_t rom_hdr_real; // offset of ROM header from start of NK partition
    uint8_t padding[0xFB4]; // pad to 0x1000
} wmnk_hdr_t;

// ROMHDR in WCE
typedef struct __attribute__((packed)) _wmnk_rom_hdr {
    uint32_t dllfirst;
    uint32_t dlllast;
    uint32_t physfirst;
    uint32_t physlast;
    uint32_t nummods;
    uint32_t ulRAMStart;
    uint32_t ulRAMFree;
    uint32_t ulRAMEnd;
    uint32_t ulCopyEntries;
    uint32_t ulCopyOffset;
    uint32_t ulProfileLen;
    uint32_t ulProfileOffset;
    uint32_t numfiles;
    uint32_t ulKernelFlags;
    uint32_t ulFSRamPercent;
    uint32_t ulDrivglobStart;
    uint32_t ulDrivglobLen;
    uint16_t usCPUType;
    uint16_t usMiscFlags;
    uint32_t pExtensions;
    uint32_t ulTrackingStart;
    uint32_t ulTrackingLen;
} wmnk_rom_hdr_t;

// TOCentry in WCE
typedef struct __attribute__((packed)) _wmnk_toc_entry {
    uint32_t dwFileAttributes;
    uint64_t ftTime;
    uint32_t nFileSize;
    uint32_t lpszFileName;
    uint32_t ulE32Offset;
    uint32_t ulO32Offset;
    uint32_t ulLoadOffset;
} wmnk_toc_entry_t;

// FILESentry in WCE
typedef struct __attribute__((packed)) _wmnk_files_entry {
    uint32_t dwFileAttributes;
    uint64_t ftTime;
    uint32_t nRealFileSize;
    uint32_t nCompFileSize;
    uint32_t lpszFileName;
    uint32_t ulLoadOffset;
} wmnk_files_entry_t;

// COPYentry in WCE
typedef struct __attribute__((packed)) _wmnk_copy_entry {
    uint32_t ulSource;
    uint32_t ulDest;
    uint32_t ulCopyLen;
    uint32_t ulDestLen;
} wmnk_copy_entry_t;

// info in WCE
typedef struct __attribute__((packed)) _e32_info {
    uint32_t rva;
    uint32_t size;
} e32_info_t;

// e32_rom in WCE
typedef struct __attribute__((packed)) _e32_rom {
    uint16_t objcnt;
    uint16_t imageflags;
    uint32_t entryrva;
    uint32_t vbase;
    uint16_t subsysmajor;
    uint16_t subsysminor;
    uint32_t stackmax;
    uint32_t vsize;
    uint32_t sect14rva;
    uint32_t sect14size;
    e32_info_t unit[9];
    uint16_t subsys;
    uint16_t unknown; //wtf?
} e32_rom_t;

// o32_rom in WCE
typedef struct __attribute__((packed)) _o32_rom {
    uint32_t vsize;
    uint32_t rva;
    uint32_t psize;
    uint32_t dataptr;
    uint32_t realaddr;
    uint32_t flags;
} o32_rom_t;

#endif // _WMNK_H_
