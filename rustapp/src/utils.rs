use windows::Foundation::{IReference, PropertyValue};
use windows_core::{Interface, Result, HSTRING};

#[allow(non_snake_case)]
pub(crate) fn HStringReference(text: &HSTRING) -> Result<IReference<HSTRING>> {
    PropertyValue::CreateString(text)?.cast()
}
