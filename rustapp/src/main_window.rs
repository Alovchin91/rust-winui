use std::ops::Deref;

use winappsdk::Microsoft::UI::Xaml::{
    Application,
    Controls::{HyperlinkButton, StackPanel, TextBlock},
    HorizontalAlignment, Style, VerticalAlignment, Window,
};
use windows::{
    core::{h, Interface, Result},
    Foundation::Uri,
};

use crate::winrt::HStringReference;

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

        let current_app = Application::Current()?;

        let title = TextBlock::new()?;
        let style: Style = current_app
            .Resources()?
            .Lookup(&HStringReference(h!("TitleTextBlockStyle"))?)?
            .cast()?;
        title.SetStyle(&style)?;
        title.SetText(h!("WinUI 3 in Rust! (Without XAML of course)"))?;
        title.SetHorizontalAlignment(HorizontalAlignment::Center)?;

        let hyperlink = HyperlinkButton::new()?;
        hyperlink.SetContent(&HStringReference(h!("GitHub Project Repository"))?)?;
        hyperlink.SetNavigateUri(&Uri::CreateUri(h!(
            "https://github.com/Alovchin91/rust-winui"
        ))?)?;
        hyperlink.SetHorizontalAlignment(HorizontalAlignment::Center)?;

        stack_panel.Children()?.Append(&title)?;
        stack_panel.Children()?.Append(&hyperlink)?;

        self.SetContent(&stack_panel)
    }
}

impl Deref for MainWindow {
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}
