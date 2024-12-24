use windows::{
    core::Result,
    Win32::System::WinRT::{RoInitialize, RoUninitialize, RO_INIT_MULTITHREADED},
};

struct RoInitT;

pub(crate) fn init_apartment() -> Result<impl Drop> {
    unsafe { RoInitialize(RO_INIT_MULTITHREADED) }?;
    Ok(RoInitT)
}

impl Drop for RoInitT {
    fn drop(&mut self) {
        log::debug!("RoUninitialize");
        //unsafe { RoUninitialize() };
    }
}
