#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

use log::{debug, info, warn};

use crate::sbi::shutdown;

global_asm!(include_str!("entry.asm"));

mod lang_items;
mod logger;
#[macro_use]
mod console;
mod sbi;
mod timer;
mod init;

#[no_mangle]
pub fn main() -> ! {
    extern "C" {
        fn stext();
        fn etext(); 
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack_lower_bound();
        fn boot_stack_top();
    }
    clear_bss();
    init::InitConfig::init();
    warn!("Hello, world!");
    info!("[kernel] booting...");
    debug!("[stext] 0x{:x}", stext as usize);
    debug!("[etext] 0x{:x}", etext as usize);
    debug!("[srodata] 0x{:x}", srodata as usize);
    debug!("[erodata] 0x{:x}", erodata as usize);
    debug!("[sdata] 0x{:x}", sdata as usize);
    debug!("[edata] 0x{:x}", edata as usize);
    debug!("[sbss] 0x{:x}", sbss as usize);
    debug!("[ebss] 0x{:x}", ebss as usize);
    debug!("[boot_stack_lower_bound] 0x{:x}", boot_stack_lower_bound as usize);
    debug!("[boot_stack_top] 0x{:x}", boot_stack_top as usize);
    shutdown(false);
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}