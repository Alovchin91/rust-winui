use windows::Foundation::Uri;
use windows_core::{h, IInspectable, Interface, Result};
use winui3::{
    Activatable,
    Microsoft::UI::Xaml::{
        Application,
        Controls::{HyperlinkButton, Page, StackPanel, TextBlock},
        HorizontalAlignment,
        Navigation::{NavigatingCancelEventArgs, NavigationEventArgs},
        Style, VerticalAlignment,
    },
    XamlPage, XamlPageOverrides,
};

use crate::utils::HStringReference;

pub struct MainPage;

impl Activatable for MainPage {
    fn activate() -> Result<IInspectable> {
        let page = XamlPage::compose(MainPage)
            .inspect_err(|err| log::error!("Failed to create a XamlPage: {:?}", err))?;

        Ok(page.into())
    }
}

impl XamlPageOverrides for MainPage {
    fn OnNavigatedFrom(&self, _base: &Page, _args: Option<&NavigationEventArgs>) -> Result<()> {
        log::debug!("OnNavigatedFrom");
        Ok(())
    }

    fn OnNavigatedTo(&self, page: &Page, _args: Option<&NavigationEventArgs>) -> Result<()> {
        log::debug!("OnNavigatedTo");

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

        page.SetContent(&stack_panel)
            .inspect_err(|err| log::error!("Failed to set content of a XamlPage: {:?}", err))?;

        Ok(())
    }

    fn OnNavigatingFrom(
        &self,
        _base: &Page,
        _args: Option<&NavigatingCancelEventArgs>,
    ) -> Result<()> {
        log::debug!("OnNavigatingFrom");
        Ok(())
    }
}

impl Drop for MainPage {
    fn drop(&mut self) {
        log::debug!("MainPage::drop");
    }
}
