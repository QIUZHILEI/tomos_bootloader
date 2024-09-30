#![no_std]
#![no_main]
use core::panic::PanicInfo;
use core::sync::atomic::Ordering;
use core::{
    arch::global_asm,
    sync::atomic::{AtomicBool, AtomicUsize},
};
use log::info;
use tom_arch::riscv::mhartid;

use tomos_bootloader::{init, load_kernel, println};
global_asm!(include_str!("./entry.S"));
global_asm!(include_str!("./jump.S"));
extern "C" {
    static _bss_start: usize;
    static _bss_end: usize;
    fn jump_kernel(size: usize, addr: usize) -> !;
}
const LOAD_ADDRESS: usize = 0x40000000;
static BLOCK: AtomicBool = AtomicBool::new(true);
static KERNEL_SIZE: AtomicUsize = AtomicUsize::new(0);
#[no_mangle]
pub extern "C" fn rust_entry(code_end: usize) -> ! {
    if mhartid::read() == 1 {
        clear_bss();
        init(code_end);
        let size = load_kernel(LOAD_ADDRESS);
        KERNEL_SIZE.store(size, Ordering::Relaxed);
        BLOCK.store(false, Ordering::Relaxed);
        info!("prepare into kernel.");
    } else {
        while BLOCK.load(Ordering::Relaxed) {}
    }
    unsafe { jump_kernel(KERNEL_SIZE.load(Ordering::Relaxed), LOAD_ADDRESS) }
}

fn clear_bss() {
    let mut bss = unsafe { _bss_start as *mut usize };
    let bss_end = unsafe { _bss_end as *mut usize };
    unsafe {
        while bss.lt(&bss_end) {
            (*bss) = 0;
            bss = bss.add(1);
        }
    }
}

#[panic_handler]
pub fn panic(println: &PanicInfo) -> ! {
    if let Some(location) = println.location() {
        println!(
            "panic occurred in file '{}' at line {}",
            location.file(),
            location.line(),
        );
    } else {
        println!("panic occurred but can't get location printlnrmation...");
    }

    println!("panic message: {:?}", println.message());
    loop {}
}
