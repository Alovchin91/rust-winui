#![windows_subsystem = "console"]

mod app;
mod main_page;
mod main_window;
mod utils;

use windows_core::{Ref, Result};
use winui3::Microsoft::UI::Xaml::{
    Application, ApplicationInitializationCallback, ApplicationInitializationCallbackParams,
    DispatcherShutdownMode, UnhandledExceptionEventHandler,
};

use app::App;

fn main() -> Result<()> {
    simple_logger::init_with_env().expect("failed to initialize the logger");

    winui3::init_apartment(winui3::ApartmentType::SingleThreaded)?;

    let winui_dependency = winui3::bootstrap::PackageDependency::initialize()?;

    log::debug!("WinUI initialized: {:?}", winui_dependency);

    Application::Start(&ApplicationInitializationCallback::new(app_start))?;

    Ok(())
}

fn app_start(_: Ref<'_, ApplicationInitializationCallbackParams>) -> Result<()> {
    log::debug!("Application::Start");

    let app = App::create()?;
    app.SetDispatcherShutdownMode(DispatcherShutdownMode::OnLastWindowClose)?;
    app.UnhandledException(Some(&UnhandledExceptionEventHandler::new(
        |_sender, args| {
            match args.as_ref() {
                Some(args) => {
                    log::error!("Unhandled exception: {}", args.Exception()?);
                    log::error!("{}", args.Message()?);
                }
                None => log::error!("Unhandled exception occurred"),
            }
            Ok(())
        },
    )))?;

    Ok(())
}
