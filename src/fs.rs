use alloc::boxed::Box;
use log::debug;
use minifat::{Error, IoBase, IoError, Read, Seek, SeekFrom, Write};
use tom_device::{BlockDevice, DeviceError};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DevError(DeviceError);
impl IoError for DevError {
    fn is_interrupted(&self) -> bool {
        false
    }

    fn new_unexpected_eof_error() -> Self {
        Self(DeviceError::IoError)
    }

    fn new_write_zero_error() -> Self {
        Self(DeviceError::IoError)
    }
}

struct DataBuffer {
    data: Box<[u8]>,
    len: usize,
    blk_cursor: usize,
    data_cursor: usize,
    valid: bool,
}

impl Default for DataBuffer {
    fn default() -> Self {
        Self {
            data: Box::new([0u8; 512]),
            len:512,
            blk_cursor: 0,
            data_cursor: 0,
            valid: true,
        }
    }
}

pub struct Volume {
    blk_dev: &'static mut dyn BlockDevice,
    start_offset: usize,
    buffer: DataBuffer,
    block_num: usize,
}

impl IoBase for Volume {
    type Error = Error<DevError>;
}

impl Volume {
    pub fn new(start_lba: usize, end_lba: usize, blk_dev: &'static mut dyn BlockDevice) -> Self {
        let mut buffer = DataBuffer::default();
        blk_dev.read_block(start_lba, &mut buffer.data[..]).unwrap();
        Self {
            blk_dev,
            start_offset: start_lba,
            block_num: end_lba - start_lba,
            buffer,
        }
    }
}

impl Read for Volume {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        debug!(
            "read operate, block_index {}. buf size: {}",
            self.buffer.blk_cursor,
            buf.len()
        );
        let start = self.byte_offset % 512;
        let end = start + buf.len();
        buf.copy_from_slice(&self.data[start..end]);
        self.byte_offset += buf.len();
        Ok(buf.len())
    }
}

impl Write for Volume {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        debug!(
            "write operate, block_index {}. buf size: {}",
            self.buffer.blk_cursor,
            buf.len()
        );
        let start = self.byte_offset % 512;
        buf.iter().enumerate().for_each(|(index, item)| {
            self.data[start + index] = *item;
        });
        self.byte_offset += buf.len();
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        debug!("flush operate, block_index {}.", self.buffer.blk_cursor);
        self.blk_dev
            .write_block(self.start_offset + self.buffer.blk_cursor, self.data.as_mut())
            .unwrap();
        Ok(())
    }
}

impl Seek for Volume {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error> {
        let new_byte_offset = match pos {
            SeekFrom::Current(x) => {
                debug!(
                    "byte_offset: {},block_index: {}, current seek {}",
                    self.buffer.data_cursor, self.buffer.blk_cursor, x
                );
                self.buffer.blk_cursor as i64 + x
            }
            SeekFrom::Start(x) => {
                debug!(
                    "byte_offset: {},block_index: {}, start seek {}",
                    self.buffer.data_cursor, self.buffer.blk_cursor, x
                );
                x as i64
            }
            SeekFrom::End(x) => {
                debug!(
                    "byte_offset: {},block_index: {}, end seek {}",
                    self.buffer.data_cursor, self.buffer.blk_cursor, x
                );
                self.buffer.data_cursor as i64 + x
            }
        };

        let new_block_index = new_byte_offset / 512;
        if new_block_index < 0 || new_block_index as u64 > self.block_num as u64 {
            Err(Error::InvalidInput)
        } else {
            if self.buffer.blk_cursor != new_block_index as usize {
                self.buffer.blk_cursor = new_block_index as usize;
                self.blk_dev
                    .read_block(
                        self.start_offset + self.buffer.blk_cursor,
                        &mut self.buffer.data[..],
                    )
                    .unwrap();
            }
            self.byte_offset = new_byte_offset as usize;
            Ok(self.byte_offset as u64)
        }
    }
}
