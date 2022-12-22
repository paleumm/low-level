//! Rust Zanim Module
use kernel::prelude::*;
use kernel::{
    file::{File, Operations},
    miscdev,
    io_buffer::{IoBufferReader, IoBufferWriter},
    sync::{Arc, ArcBorrow, smutex::Mutex}
};

module! {
    type: Zanim,
    name: "zanim",
    license: "GPL",
}

struct Zanim {
    _dev: Pin<Box<miscdev::Registration<Zanim>>>    
}

struct Device {
    number: usize,
    contents: Mutex<Vec<u8>>
}

impl kernel::Module for Zanim {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("-----------------------------\n");
        pr_info!("initializie zanim! by paleumm\n");
        pr_info!("-----------------------------\n");

        let dev = Arc::try_new(Device {
            number: 0,
            contents: Mutex::new(Vec::new())
        })?;
        let reg = miscdev::Registration::new_pinned(fmt!("zanim"), dev)?;
        Ok(Self {_dev: reg})
    }
}

#[vtable]
impl Operations for Zanim {
    type OpenData = Arc<Device>;
    type Data = Arc<Device>;

    fn open(context: &Arc<Device>, _file: &File) -> Result<Arc<Device>> {
        pr_info!("File for device {} was opened\n", context.number);
        Ok(context.clone())
    }

    fn read(data: ArcBorrow<'_, Device>,_file: &File, writer: &mut impl IoBufferWriter,offset:u64,) -> Result<usize> {
        pr_info!("File for device {} was read\n", data.number);
        let offset = offset.try_into()?;
        let vec = data.contents.lock();
        let len = core::cmp::min(writer.len(), vec.len().saturating_sub(offset));
        writer.write_slice(&vec[offset..][..len])?;
        Ok(len)
    }

    fn write(data: ArcBorrow<'_, Device>, _file: &File,reader: &mut impl IoBufferReader,_offset:u64,) -> Result<usize> {
        pr_info!("File for device {} was written\n", data.number);
        let copy = reader.read_all()?;
        let len = copy.len();
        *data.contents.lock() = copy;
        Ok(len)
    }
}