use std::ops::Deref;

use winappsdk::Microsoft::UI::Xaml::{
    Controls::StackPanel, HorizontalAlignment, VerticalAlignment, Window,
};
use windows::core::Result;

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
        let stack_panel = StackPanel::new()?;
        stack_panel.SetHorizontalAlignment(HorizontalAlignment::Center)?;
        stack_panel.SetVerticalAlignment(VerticalAlignment::Center)?;
        self.SetContent(&stack_panel)
    }
}

impl Deref for MainWindow {
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}
