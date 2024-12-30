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
