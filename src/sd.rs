use crate::timer::TICKER_REF;
use sdio::DwMmcHost;
use tom_device::BlockDevice;
const SDIO_BASE: usize = 0x16020000;
static mut MMC: DwMmcHost = DwMmcHost::new(SDIO_BASE, TICKER_REF);
pub fn init() {
    let dw_mmc = unsafe { blk_dev_mut() };
    dw_mmc.init().unwrap();
}

pub fn read_block(lba: usize, blk: &mut [u8]) {
    let mmc = unsafe { blk_dev_mut() };
    mmc.read_block(lba, blk).unwrap();
}

#[inline]
pub unsafe fn blk_dev_mut() -> &'static mut dyn BlockDevice {
    (&raw mut MMC as *mut dyn BlockDevice).as_mut().unwrap()
}
