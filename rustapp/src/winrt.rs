use windows::{
    core::{Interface, Result, HSTRING},
    Foundation::{IReference, PropertyValue},
    Win32::System::WinRT::{RoInitialize, RO_INIT_MULTITHREADED, RO_INIT_SINGLETHREADED},
};

#[allow(unused)]
pub(crate) enum ApartmentType {
    MultiThreaded,
    SingleThreaded,
}

#[inline]
pub(crate) fn init_apartment(apartment_type: ApartmentType) -> Result<()> {
    let roinit = match apartment_type {
        ApartmentType::MultiThreaded => RO_INIT_MULTITHREADED,
        ApartmentType::SingleThreaded => RO_INIT_SINGLETHREADED,
    };
    unsafe { RoInitialize(roinit) }
}

#[allow(non_snake_case)]
pub(crate) fn HStringReference(text: &HSTRING) -> Result<IReference<HSTRING>> {
    PropertyValue::CreateString(text)?.cast()
}
