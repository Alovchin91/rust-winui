use std::ops::Deref;

use windows_core::Result;
use winui3::{
    xaml_typename,
    Microsoft::UI::Xaml::{Controls::Frame, Media::MicaBackdrop, Window},
};

pub(crate) struct MainWindow {
    window: Window,
}

impl MainWindow {
    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            window: Window::new()?,
        })
    }

    #[allow(non_snake_case)]
    pub(crate) fn InitializeComponent(&self) -> Result<()> {
        self.SetExtendsContentIntoTitleBar(true)?;
        self.SetSystemBackdrop(&MicaBackdrop::new()?)?;

        let frame = Frame::new()?;

        let page_type = xaml_typename("MyPage");

        if let Err(err) = frame.Navigate2(&page_type) {
            log::error!("Failed to navigate to the page: {:?}", err);
        }

        self.SetContent(&frame)
    }
}

impl Deref for MainWindow {
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}
