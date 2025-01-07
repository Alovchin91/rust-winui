#![allow(non_snake_case)]

#[rustfmt::skip]
mod bindings {
    pub mod Interop;
    pub mod Microsoft;
}

pub mod Windows {
    pub mod UI {
        pub mod Xaml {
            pub use crate::bindings::Interop;
        }
    }
}

pub use bindings::Microsoft;

pub mod bootstrap;

pub enum ApartmentType {
    MultiThreaded,
    SingleThreaded,
}

#[inline]
pub fn init_apartment(apartment_type: ApartmentType) -> windows_core::Result<()> {
    use windows::Win32::System::WinRT::{
        RoInitialize, RO_INIT_MULTITHREADED, RO_INIT_SINGLETHREADED,
    };
    let roinit = match apartment_type {
        ApartmentType::MultiThreaded => RO_INIT_MULTITHREADED,
        ApartmentType::SingleThreaded => RO_INIT_SINGLETHREADED,
    };
    unsafe { RoInitialize(roinit) }
}
