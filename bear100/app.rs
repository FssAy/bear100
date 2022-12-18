use std::ops::Deref;
use std::ptr::null_mut;
use sysinfo::*;

use crate::config::*;
use crate::winapi::{
    kernel32::{
        WriteProcessMemory as write_process_memory,
        ReadProcessMemory as read_process_memory,
        OpenProcess as open_process,
        GetLastError as get_last_error,
        EnumProcessModules as enum_process_modules,
        CloseHandle as close_handle,
    },
    *,
};


pub struct App<'a> {
    cfg: &'a Config,
    pid: DWORD,
    handle: HANDLE,
    base_addr: ADDR64,
    reward_addr: ADDR64,
}

impl<'a> App<'a> {
    pub fn new(cfg: &'a Config) -> Self {
        Self {
            cfg,
            pid: 0,
            handle: unsafe {
                libc::malloc(
                    std::mem::size_of::<libc::c_void>()
                )
            } as HANDLE,
            base_addr: 0,
            reward_addr: 0,
        }
    }

    // No need to return a specific error in this console application.
    // It will be logged back into the stdout if tracing was initialized.
    pub fn run(&mut self) -> Option<()> {
        let r = self._run();
        if self.show_app_info {
            self.show_app_info();
        }
        r
    }
}

impl<'a> App<'a> {
    fn wait_for_pid(&mut self) {
        let mut sys = System::new_with_specifics(
            RefreshKind::new().with_processes(
                ProcessRefreshKind::new()
            )
        );

        info!("waiting for the process: {}", self.process_name);

        loop {
            sys.refresh_processes();

            for (proc_pid, process) in sys.processes() {
                if process.name() == self.process_name {
                    self.pid = proc_pid.as_u32();
                    return;
                }
            }

            std::thread::sleep(
                std::time::Duration::from_millis(self.refresh_rate_ms)
            );
        }
    }

    fn get_handle(&mut self) -> Option<()> {
        unsafe {
            let handle = open_process(
                PROCESS_VM_OPERATION | PROCESS_VM_WRITE | PROCESS_VM_READ | PROCESS_QUERY_INFORMATION,
                FALSE,
                self.pid,
            );

            if handle.is_null() {
                error!("cannot get the process handle: {:#X}", get_last_error());
                return None;
            }

            self.handle = handle
        };

        Some(())
    }

    fn get_base_addr(&mut self) -> Option<()> {
         let base_addr = unsafe {
            let mut lph_module = [0; 512];
             let mut _lpc_needed: DWORD = 0;

             if enum_process_modules(
                self.handle,
                lph_module.as_mut_ptr(),
                std::mem::size_of::<[HINSTANCE; 512]>() as DWORD,
                &mut _lpc_needed,
             ) == FALSE {
                error!("enum_process_modules failed: {:#X}", get_last_error());
                close_handle(self.handle);
                return None;
            }

            lph_module[0] as ADDR64
        };

        self.base_addr = base_addr;
        Some(())
    }

    fn get_reward_addr(&mut self) -> Option<()> {
        let mut reward_address = self.base_addr;

        let len = self.offsets.len();
        for offset in &self.offsets.as_slice()[..len-1] {
            let offset = *offset;
            let mut new_address = 0;

            unsafe {
                if read_process_memory(
                    self.handle,
                    (reward_address + offset as ADDR64) as LPCVOID,
                    &mut new_address as *mut ADDR64 as LPVOID,
                    std::mem::size_of::<ADDR64>(),
                    null_mut(),
                ) == FALSE {
                    error!("read_process_memory failed: {:#X}", get_last_error());
                    close_handle(self.handle);
                    return None;
                }
            }

            reward_address = new_address;
        }

        if let Some(offset_last) = self.offsets.last() {
            reward_address += *offset_last as ADDR64;
        }

        self.reward_addr = reward_address;
        Some(())
    }

    fn write_reward_val(&self) -> Option<()> {
        unsafe {
            let mut _bytes_written = 0;  // for now unused
            if write_process_memory(
                self.handle,
                self.reward_addr as LPVOID,
                &self.replace_value as *const BearReward as LPCVOID,
                std::mem::size_of::<BearReward>(),
                &mut _bytes_written,
            ) == FALSE {
                error!("write_process_memory failed: {:#X}", get_last_error());
                close_handle(self.handle);
                return None;
            }

            close_handle(self.handle);
        };

        Some(())
    }

    fn show_app_info(&self) {
        info!("App::pid: {}", if self.pid != 0 {
            self.pid.to_string()
        } else {
            "NotFound".into()
        });

        info!("App::handle: {}", if !self.handle.is_null() {
            "Ok"
        } else {
            "NotFound"
        });

        info!("App::base_addr: {}", if self.base_addr != 0 {
            format!("{:#X}", self.base_addr)
        } else {
            "NotFound".into()
        });

        info!("App::reward_addr: {}", if self.reward_addr != 0 {
            format!("{:#X}", self.reward_addr)
        } else {
            "NotFound".into()
        });
    }

    fn _run(&mut self) -> Option<()> {
        self.wait_for_pid();
        self.get_handle()?;
        self.get_base_addr()?;
        self.get_reward_addr()?;
        self.write_reward_val()
    }
}

impl<'a> Deref for App<'a> {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        &self.cfg
    }
}
