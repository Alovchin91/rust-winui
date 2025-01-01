use std::cell::RefCell;

use winappsdk::{
    Microsoft::UI::Xaml::{
        Application, ApplicationHighContrastAdjustment, ApplicationTheme,
        Controls::XamlControlsResources,
        DebugSettings, DispatcherShutdownMode, FocusVisualKind, IApplication2, IApplication2_Impl,
        IApplication3, IApplication3_Impl, IApplicationFactory, IApplicationOverrides,
        IApplicationOverrides_Impl, IApplication_Impl, LaunchActivatedEventArgs,
        Markup::{IXamlMetadataProvider, IXamlMetadataProvider_Impl, IXamlType, XmlnsDefinition},
        ResourceDictionary, ResourceManagerRequestedEventArgs, UnhandledExceptionEventHandler,
        XamlTypeInfo::XamlControlsXamlMetaDataProvider,
    },
    Windows::UI::Xaml::Interop::TypeName,
};
use windows::{
    Foundation::{EventRegistrationToken, TypedEventHandler, Uri},
    Win32::Foundation::{CO_E_NOTCONSTRUCTED, E_ILLEGAL_METHOD_CALL, E_POINTER},
};
use windows_core::{
    h, implement, Array, ComObject, Error, IInspectable, Interface, InterfaceRef, Result, Type,
    HSTRING,
};

use crate::main_window::MainWindow;

#[implement(
    Application,
    IApplication2,
    IApplication3,
    IApplicationOverrides,
    IXamlMetadataProvider
)]
pub(crate) struct App<'a> {
    base: RefCell<Option<InterfaceRef<'a, Application>>>,
    provider: RefCell<Option<XamlControlsXamlMetaDataProvider>>,
    window: RefCell<Option<MainWindow>>,
}

impl<'a> App<'a> {
    pub(crate) fn new() -> Result<Application> {
        let app = ComObject::new(App {
            base: RefCell::new(None),
            provider: RefCell::new(None),
            window: RefCell::new(None),
        });
        let outer = app.as_interface::<IInspectable>();
        let (application, base): (Application, InterfaceRef<'_, IInspectable>) =
            Self::application_factory(|this| unsafe {
            let mut base__ = core::ptr::null_mut();
            let mut result__ = core::mem::zeroed();
            (Interface::vtable(this).CreateInstance)(
                Interface::as_raw(this),
                Interface::as_raw(&*outer),
                &mut base__,
                &mut result__,
            )
            .and_then(|| Type::from_abi(result__))
                .and_then(move |app__| {
                    core::ptr::NonNull::new(base__)
                        .map(|ptr__| InterfaceRef::from_raw(ptr__))
                        .ok_or_else(|| Error::from(E_POINTER))
                        .map(|ref__| (app__, ref__))
            })
        })?;
        app.base
            .borrow_mut()
            .replace(InterfaceRef::from_interface(&base.cast::<Application>()?));
        Ok(application)
    }

    fn application_factory<R, F: FnOnce(&IApplicationFactory) -> Result<R>>(
        callback: F,
    ) -> Result<R> {
        use windows_core::imp::FactoryCache;
        static SHARED: FactoryCache<Application, IApplicationFactory> = FactoryCache::new();
        SHARED.call(callback)
    }

    fn with_base<R, F: FnOnce(&Application) -> Result<R>>(&self, func: F) -> Result<R> {
        self.base
            .borrow()
            .as_ref()
            .ok_or_else(|| Error::from(CO_E_NOTCONSTRUCTED))
            .and_then(|base| func(base))
    }

    fn with_provider<R, F: FnOnce(&XamlControlsXamlMetaDataProvider) -> Result<R>>(
        &self,
        func: F,
    ) -> Result<R> {
        self.provider
            .borrow()
            .as_ref()
            .ok_or_else(|| Error::from(E_ILLEGAL_METHOD_CALL))
            .and_then(|provider| func(provider))
    }
}

impl IApplication_Impl for App_Impl<'_> {
    fn Resources(&self) -> Result<ResourceDictionary> {
        self.with_base(|base| base.Resources())
    }

    fn SetResources(&self, value: Option<&ResourceDictionary>) -> Result<()> {
        self.with_base(|base| base.SetResources(value))
    }

    fn DebugSettings(&self) -> Result<DebugSettings> {
        self.with_base(|base| base.DebugSettings())
    }

    fn RequestedTheme(&self) -> Result<ApplicationTheme> {
        self.with_base(|base| base.RequestedTheme())
    }

    fn SetRequestedTheme(&self, value: ApplicationTheme) -> Result<()> {
        self.with_base(|base| base.SetRequestedTheme(value))
    }

    fn FocusVisualKind(&self) -> Result<FocusVisualKind> {
        self.with_base(|base| base.FocusVisualKind())
    }

    fn SetFocusVisualKind(&self, value: FocusVisualKind) -> Result<()> {
        self.with_base(|base| base.SetFocusVisualKind(value))
    }

    fn HighContrastAdjustment(&self) -> Result<ApplicationHighContrastAdjustment> {
        self.with_base(|base| base.HighContrastAdjustment())
    }

    fn SetHighContrastAdjustment(&self, value: ApplicationHighContrastAdjustment) -> Result<()> {
        self.with_base(|base| base.SetHighContrastAdjustment(value))
    }

    fn UnhandledException(
        &self,
        handler: Option<&UnhandledExceptionEventHandler>,
    ) -> Result<EventRegistrationToken> {
        self.with_base(|base| base.UnhandledException(handler))
    }

    fn RemoveUnhandledException(&self, token: &EventRegistrationToken) -> Result<()> {
        self.with_base(|base| base.RemoveUnhandledException(*token))
    }

    fn Exit(&self) -> Result<()> {
        self.with_base(|base| base.Exit())
    }
}

impl IApplication2_Impl for App_Impl<'_> {
    fn ResourceManagerRequested(
        &self,
        handler: Option<&TypedEventHandler<IInspectable, ResourceManagerRequestedEventArgs>>,
    ) -> Result<EventRegistrationToken> {
        self.with_base(|base| base.ResourceManagerRequested(handler))
    }

    fn RemoveResourceManagerRequested(&self, token: &EventRegistrationToken) -> Result<()> {
        self.with_base(|base| base.RemoveResourceManagerRequested(*token))
    }
}

impl IApplication3_Impl for App_Impl<'_> {
    fn DispatcherShutdownMode(&self) -> Result<DispatcherShutdownMode> {
        self.with_base(|base| base.DispatcherShutdownMode())
    }

    fn SetDispatcherShutdownMode(&self, value: DispatcherShutdownMode) -> Result<()> {
        self.with_base(|base| base.SetDispatcherShutdownMode(value))
    }
}

impl IApplicationOverrides_Impl for App_Impl<'_> {
    fn OnLaunched(&self, _: Option<&LaunchActivatedEventArgs>) -> Result<()> {
        log::debug!("App::OnLaunched");

        XamlControlsXamlMetaDataProvider::Initialize()?;
        self.provider
            .borrow_mut()
            .replace(XamlControlsXamlMetaDataProvider::new()?);

        let resources = self.Resources()?;
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

impl IXamlMetadataProvider_Impl for App_Impl<'_> {
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

impl Drop for App<'_> {
    fn drop(&mut self) {
        log::debug!("App::drop");
    }
}
