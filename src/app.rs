use std::cell::RefCell;

use windows::Foundation::Uri;
use windows_core::{h, Ref, Result};
use winui3::{
    Microsoft::UI::Xaml::{
        Application, Controls::XamlControlsResources, LaunchActivatedEventArgs, ResourceDictionary,
    },
    XamlApp, XamlAppOverrides,
};

use crate::main_window::MainWindow;

pub(crate) struct App {
    window: RefCell<Option<MainWindow>>,
}

impl App {
    pub(crate) fn create() -> Result<Application> {
        let app = App {
            window: RefCell::new(None),
        };
        XamlApp::compose(app)
    }
}

impl XamlAppOverrides for App {
    fn OnLaunched(
        &self,
        base: &'_ Application,
        _: Ref<'_, LaunchActivatedEventArgs>,
    ) -> Result<()> {
        log::debug!("App::OnLaunched");

        let resources = base.Resources()?;
        let merged_dictionaries = resources.MergedDictionaries()?;
        let xaml_controls_resources = XamlControlsResources::new()?;
        merged_dictionaries.Append(&xaml_controls_resources)?;

        let compact_resources = ResourceDictionary::new()?;
        compact_resources.SetSource(&Uri::CreateUri(h!(
            "ms-appx:///Microsoft.UI.Xaml/DensityStyles/Compact.xaml"
        ))?)?;
        merged_dictionaries.Append(&compact_resources)?;

        let window = MainWindow::new()?;
        window.InitializeComponent()?;
        window.Activate()?;

        self.window.borrow_mut().replace(window);
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        log::debug!("App::drop");
    }
}
