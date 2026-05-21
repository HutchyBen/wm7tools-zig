use std::fs::File;
use std::io;
use std::ops::Deref;
use std::path::Path;

pub struct MappedFile {
    ptr: *mut std::ffi::c_void,
    len: usize,
    #[cfg(windows)]
    map_handle: *mut std::ffi::c_void,
}

impl Deref for MappedFile {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        if self.len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.ptr as *const u8, self.len) }
        }
    }
}

unsafe impl Send for MappedFile {}
unsafe impl Sync for MappedFile {}

#[cfg(windows)]
impl MappedFile {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        use std::os::windows::io::AsRawHandle;

        let file = File::open(path)?;
        let len = file.metadata()?.len() as usize;
        if len == 0 {
            return Ok(Self {
                ptr: std::ptr::null_mut(),
                len: 0,
                map_handle: std::ptr::null_mut(),
            });
        }

        let file_handle = file.as_raw_handle();
        unsafe {
            type HANDLE = *mut std::ffi::c_void;
            type DWORD = u32;

            #[link(name = "kernel32")]
            extern "system" {
                fn CreateFileMappingW(
                    hFile: HANDLE,
                    lpFileMappingAttributes: *mut std::ffi::c_void,
                    flProtect: DWORD,
                    dwMaximumSizeHigh: DWORD,
                    dwMaximumSizeLow: DWORD,
                    lpName: *const u16,
                ) -> HANDLE;

                fn MapViewOfFile(
                    hFileMappingObject: HANDLE,
                    dwDesiredAccess: DWORD,
                    dwFileOffsetHigh: DWORD,
                    dwFileOffsetLow: DWORD,
                    dwNumberOfBytesToMap: usize,
                ) -> *mut std::ffi::c_void;

                fn CloseHandle(hObject: HANDLE) -> i32;
            }

            // PAGE_READONLY = 0x02
            let map_handle = CreateFileMappingW(
                file_handle as HANDLE,
                std::ptr::null_mut(),
                0x02,
                0,
                0,
                std::ptr::null(),
            );
            if map_handle.is_null() {
                return Err(io::Error::last_os_error());
            }

            // FILE_MAP_READ = 0x0004
            let ptr = MapViewOfFile(map_handle, 0x0004, 0, 0, len);
            if ptr.is_null() {
                let err = io::Error::last_os_error();
                let _ = CloseHandle(map_handle);
                return Err(err);
            }

            Ok(Self { ptr, len, map_handle })
        }
    }
}

#[cfg(windows)]
impl Drop for MappedFile {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                #[link(name = "kernel32")]
                extern "system" {
                    fn UnmapViewOfFile(lpBaseAddress: *const std::ffi::c_void) -> i32;
                    fn CloseHandle(hObject: *mut std::ffi::c_void) -> i32;
                }
                UnmapViewOfFile(self.ptr);
                if !self.map_handle.is_null() {
                    CloseHandle(self.map_handle);
                }
            }
        }
    }
}

#[cfg(unix)]
impl MappedFile {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        use std::os::unix::io::AsRawFd;

        let file = File::open(path)?;
        let len = file.metadata()?.len() as usize;
        if len == 0 {
            return Ok(Self {
                ptr: std::ptr::null_mut(),
                len: 0,
            });
        }

        let fd = file.as_raw_fd();
        unsafe {
            extern "C" {
                fn mmap(
                    addr: *mut std::ffi::c_void,
                    length: usize,
                    prot: std::ffi::c_int,
                    flags: std::ffi::c_int,
                    fd: std::ffi::c_int,
                    offset: i64,
                ) -> *mut std::ffi::c_void;
            }

            // PROT_READ = 1, MAP_PRIVATE = 2
            let ptr = mmap(std::ptr::null_mut(), len, 1, 2, fd, 0);
            if ptr == !0 as *mut std::ffi::c_void {
                return Err(io::Error::last_os_error());
            }

            Ok(Self { ptr, len })
        }
    }
}

#[cfg(unix)]
impl Drop for MappedFile {
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.ptr != !0 as *mut std::ffi::c_void {
            unsafe {
                extern "C" {
                    fn munmap(addr: *mut std::ffi::c_void, length: usize) -> std::ffi::c_int;
                }
                munmap(self.ptr, self.len);
            }
        }
    }
}

pub enum FileData {
    Mmap(MappedFile),
    Vec(Vec<u8>),
}

impl Deref for FileData {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Mmap(m) => m,
            Self::Vec(v) => v,
        }
    }
}

impl FileData {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        match MappedFile::new(path) {
            Ok(mmap) => Ok(Self::Mmap(mmap)),
            Err(_) => {
                let bytes = std::fs::read(path)?;
                Ok(Self::Vec(bytes))
            }
        }
    }
}
