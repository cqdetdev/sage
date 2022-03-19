use colour::{e_green_ln, e_red_ln};
use std::ffi::{CStr, CString};
use std::mem::{size_of, zeroed};
use std::process::exit;
use std::ptr::null_mut;
use std::thread::sleep;
use std::time::Duration;
use winapi::{
    shared::{ntdef::HANDLE, windef::HWND},
    um::{
        handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
        processthreadsapi::OpenProcess,
        tlhelp32::{
            CreateToolhelp32Snapshot, Module32Next, Process32First, Process32Next, MODULEENTRY32,
            PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPPROCESS,
        },
        winnt::PROCESS_ALL_ACCESS,
        winuser::FindWindowA,
    },
};

/// Struct that holds all of the program
/// data for external
/// memory manipulation
pub struct Program {
    pub hwnd: Option<HWND>,
    pub handle: Option<HANDLE>,
    pub proc_id: Option<u32>,
    pub mod_base_addr: Option<usize>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            hwnd: None,
            handle: None,
            proc_id: None,
            mod_base_addr: None,
        }
    }

    pub unsafe fn hook(&mut self) {
        self.proc_id = Some(Self::get_proc_id());
        let proc_id = self.proc_id.unwrap();
        self.mod_base_addr = Some(Self::get_mod_base_addr(proc_id));
        self.handle = Some(OpenProcess(PROCESS_ALL_ACCESS, 0, proc_id));
        let name = CString::new("Minecraft").unwrap();
        self.hwnd = Some(FindWindowA(null_mut(), name.as_ptr()));
        if self.hwnd.unwrap() == null_mut() {
            e_red_ln!("You must have Minecraft open to use Sage...");
            sleep(Duration::from_millis(5000));
            exit(0);
        }
    }

    unsafe fn get_proc_id() -> u32 {
        let mut proc_id: u32 = 0;
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot != INVALID_HANDLE_VALUE {
            let mut entry: PROCESSENTRY32 = zeroed::<_>();
            entry.dwSize = size_of::<PROCESSENTRY32>() as u32;
            if Process32First(snapshot, &mut entry as *mut PROCESSENTRY32) != 0 {
                loop {
                    if Process32Next(snapshot, &mut entry as *mut PROCESSENTRY32) != 0 {
                        let e = entry.szExeFile.as_ptr();
                        if CStr::from_ptr(e as *const i8)
                            == CStr::from_bytes_with_nul_unchecked(
                                CString::new("Minecraft.Windows.exe")
                                    .unwrap()
                                    .to_bytes_with_nul(),
                            )
                        {
                            proc_id = entry.th32ProcessID;
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        CloseHandle(snapshot);
        proc_id
    }

    unsafe fn get_mod_base_addr(proc_id: u32) -> usize {
        let mut addr: usize = 0;
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, proc_id);
        if snapshot != INVALID_HANDLE_VALUE {
            let mut entry: MODULEENTRY32 = zeroed::<_>();
            entry.dwSize = size_of::<MODULEENTRY32>() as u32;
            while Module32Next(snapshot, &mut entry as *mut MODULEENTRY32) != 0 {
                let e = entry.szModule.as_ptr();
                if CStr::from_ptr(e as *const i8).to_str().unwrap()
                    == CString::new("Minecraft.Windows.exe")
                        .unwrap()
                        .to_str()
                        .unwrap()
                {
                    addr = entry.modBaseAddr as usize;
                }
            }
        }
        CloseHandle(snapshot);
        addr
    }
}
