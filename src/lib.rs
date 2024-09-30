#![no_std]
#![feature(const_refs_to_static)]
#![feature(const_ptr_as_ref)]
#![feature(const_option)]
#![feature(allocator_api)]
mod fs;
mod logger;
mod mem;
mod sd;
mod timer;
mod uart;
use alloc::string::ToString;
use fs::Volume;
use log::info;
use minifat::{FileSystem, FsOptions, NullTimeProvider, Read};
use gpt::{GptLayout, PRIMARY_HEADER_LBA};
pub use uart::*;
extern crate alloc;
const KERNEL_NAME: &'static str = "TOM.OS";
pub fn init(code_end: usize) {
    uart::init();
    logger::init(log::Level::Debug);
    sd::init();
    mem::init(code_end);
    info!("environment initialized");
}

pub fn load_kernel(load_addr: usize) -> usize {
    let mut buf = [0u8; 512];
    let mut gpt = GptLayout::new();
    let part_index = find_root_partition(&mut gpt, &mut buf);
    let part = gpt.partition(part_index).unwrap();
    let mut fs = init_fat(part.start_lba as usize, part.end_lba as usize);
    load(&mut fs, load_addr)
}

fn find_root_partition(gpt: &mut GptLayout, blk: &mut [u8]) -> usize {
    let root_uuid = "ebd0a0a2-b9e5-4433-87c0-68b6b72699c7";
    info!("find root partition...");
    sd::read_block(PRIMARY_HEADER_LBA, blk);
    gpt.init_primary_header(&blk).unwrap();
    let part_start = gpt.primary_header().part_start as usize;
    sd::read_block(part_start, blk);
    gpt.init_partitions(&blk, 1);
    let root_part = gpt.partition(4).unwrap();
    if root_part.part_type_guid.to_string().eq(root_uuid) {
        info!("find root partition {}", 4);
    }
    4
}

fn init_fat(start_lba: usize, end_lba: usize) -> FileSystem<Volume, NullTimeProvider> {
    info!("init fat file system");
    FileSystem::new(
        Volume::new(start_lba, end_lba, unsafe { sd::blk_dev_mut() }),
        FsOptions::new(),
    )
    .unwrap()
}

fn load(fs: &mut FileSystem<Volume, NullTimeProvider>, load_addr: usize) -> usize {
    let root = fs.root_dir();
    let mut size = 0;
    for item in root.iter() {
        let entry = item.unwrap();
        if entry.is_file() {
            info!("file name: {}", entry.short_file_name());
            if entry.short_file_name().eq(KERNEL_NAME) {
                info!("load kernel {}", entry.short_file_name());
                let mut file = entry.to_file();
                size = file.size().unwrap() as usize;
                let buf = unsafe { core::slice::from_raw_parts_mut(load_addr as *mut u8, size) };
                file.read_exact(buf).unwrap();
                break;
            }
        }
    }
    size
}
