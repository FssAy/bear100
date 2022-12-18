#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use libc::*;

pub type LPVOID = *mut c_void;
pub type LPCVOID = *const c_void;
pub type HANDLE = LPVOID;
pub type DWORD = u32;
pub type BOOL = i32;
pub type SIZE_T = usize;
pub type HINSTANCE = u64;
pub type ADDR64 = u64;

// pub static PROCESS_ALL_ACCESS: DWORD = 0xFFFF;
pub static PROCESS_VM_WRITE: DWORD = 0x0020;
pub static PROCESS_VM_OPERATION: DWORD = 0x0008;
pub static PROCESS_VM_READ: DWORD = 0x0010;
pub static PROCESS_QUERY_INFORMATION : DWORD = 0x0400;
pub static FALSE: BOOL = 0;

pub mod kernel32 {
    use super::*;

    #[link(name = "kernel32")]
    extern "system" {
        pub fn OpenProcess(
            dwDesiredAccess: DWORD,
            bInheritHandle: BOOL,
            dwProcessId: DWORD
        ) -> HANDLE;

        pub fn WriteProcessMemory(
            hProcess: HANDLE,
            lpBaseAddress: LPVOID,
            lpBuffer: LPCVOID,
            nSize: SIZE_T,
            lpNumberOfBytesWritten: &mut SIZE_T,
        ) -> BOOL;

        pub fn ReadProcessMemory(
            hProcess: HANDLE,
            lpBaseAddress: LPCVOID,
            lpBuffer: LPVOID,
            nSize: SIZE_T,
            lpNumberOfBytesRead: *mut SIZE_T
        ) -> BOOL;

        pub fn EnumProcessModules(
            hProcess: HANDLE,
            lphModule: *mut HINSTANCE,
            cb: DWORD,
            lpcbNeeded: *mut DWORD
        ) -> BOOL;

        pub fn CloseHandle(
            hObject: HANDLE
        );

        // todo: implement FormatMessage
        pub fn GetLastError() -> DWORD;
    }
}
