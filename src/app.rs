use std::cell::RefCell;

use windows::Foundation::Uri;
use windows_core::{h, Result, HSTRING};
use winui3::{
    Microsoft::UI::Xaml::{
        Application, Controls::XamlControlsResources, LaunchActivatedEventArgs, Markup::IXamlType,
        ResourceDictionary,
    },
    XamlApp, XamlAppOverrides,
};

use crate::main_page::MainPage;
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
    fn OnLaunched(&self, base: &Application, _: Option<&LaunchActivatedEventArgs>) -> Result<()> {
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

    fn TryResolveXamlType(&self, full_name: &HSTRING) -> Result<IXamlType> {
        if full_name == "MyPage" {
            winui3::XamlCustomType::<MainPage>::new(full_name)
        } else {
            Err(windows_core::Error::empty())
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        log::debug!("App::drop");
    }
}
