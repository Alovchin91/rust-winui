use std::cell::RefCell;

use winappsdk::{
    Microsoft::UI::Xaml::{
        Application,
        Controls::XamlControlsResources,
        IApplicationFactory, IApplicationOverrides, IApplicationOverrides_Impl,
        LaunchActivatedEventArgs,
        Markup::{IXamlMetadataProvider, IXamlMetadataProvider_Impl, IXamlType, XmlnsDefinition},
        XamlTypeInfo::XamlControlsXamlMetaDataProvider,
    },
    Windows::UI::Xaml::Interop::TypeName,
};
use windows::core::{
    implement, Array, ComObject, ComObjectInner, IUnknown, Interface, Result, Type, HSTRING,
};

use crate::main_window::MainWindow;

#[implement(IApplicationOverrides, IXamlMetadataProvider)]
pub(crate) struct App {
    application: RefCell<Option<Application>>,
    provider: RefCell<Option<XamlControlsXamlMetaDataProvider>>,
    window: RefCell<Option<MainWindow>>,
}

impl App {
    pub(crate) fn new() -> Result<Self> {
        Ok(App {
            application: RefCell::new(None),
            provider: RefCell::new(None),
            window: RefCell::new(None),
        })
    }

    fn init(&self, app: Application) {
        self.application.borrow_mut().replace(app);
    }

    pub(crate) fn with_app<R, F: FnOnce(&Application) -> Result<R>>(&self, func: F) -> Result<R> {
        let app_ref = self.application.borrow();
        let app = app_ref
            .as_ref()
            .expect("application is not initialised yet");
        func(app)
    }

    fn with_provider<R, F: FnOnce(&XamlControlsXamlMetaDataProvider) -> Result<R>>(
        &self,
        func: F,
    ) -> Result<R> {
        let provider_ref = self.provider.borrow();
        let provider = provider_ref
            .as_ref()
            .expect("provider is not initialised yet");
        func(provider)
    }
}

impl IApplicationOverrides_Impl for App_Impl {
    fn OnLaunched(&self, _: Option<&LaunchActivatedEventArgs>) -> Result<()> {
        log::debug!("App::OnLaunched");

        XamlControlsXamlMetaDataProvider::Initialize()?;
        self.provider
            .borrow_mut()
            .replace(XamlControlsXamlMetaDataProvider::new()?);

        let resources = self.with_app(|app| app.Resources())?;
        let merged_dictionaries = resources.MergedDictionaries()?;
        let xaml_controls_resources = XamlControlsResources::new()?;
        merged_dictionaries.Append(&xaml_controls_resources)?;

        let window = MainWindow::new()?;
        window.InitializeComponent()?;
        window.Activate()?;

        self.window.borrow_mut().replace(window);
        Ok(())
    }
}

impl IXamlMetadataProvider_Impl for App_Impl {
    fn GetXamlType(&self, type_name: &TypeName) -> Result<IXamlType> {
        log::debug!("App::GetXamlType");
        self.with_provider(|p| p.GetXamlType(type_name))
    }
    fn GetXamlTypeByFullName(&self, full_name: &HSTRING) -> Result<IXamlType> {
        log::debug!("App::GetXamlTypeByFullName");
        self.with_provider(|p| p.GetXamlTypeByFullName(full_name))
    }
    fn GetXmlnsDefinitions(&self) -> Result<Array<XmlnsDefinition>> {
        log::debug!("App::GetXmlnsDefinitions");
        self.with_provider(|p| p.GetXmlnsDefinitions())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        log::debug!("App::drop");
    }
}

fn application_factory<R, F: FnOnce(&IApplicationFactory) -> Result<R>>(callback: F) -> Result<R> {
    use windows_core::imp::FactoryCache;
    static SHARED: FactoryCache<Application, IApplicationFactory> = FactoryCache::new();
    SHARED.call(callback)
}

pub(crate) fn create_app() -> Result<ComObject<App>> {
    let app = App::new()?.into_object();
    let app_unknown = app.as_interface::<IUnknown>();
    let application: Application = application_factory(|this| unsafe {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(this).CreateInstance)(
            Interface::as_raw(this),
            Interface::as_raw(&*app_unknown),
            &mut core::ptr::null_mut(),
            &mut result__,
        )
        .and_then(|| Type::from_abi(result__))
    })?;
    app.init(application);
    Ok(app)
}
