#![windows_subsystem = "console"]

mod app;
mod main_window;
mod winrt;

use winappsdk::Microsoft::UI::Xaml::{
    Application, ApplicationInitializationCallback, ApplicationInitializationCallbackParams,
    UnhandledExceptionEventHandler,
};
use windows_core::Result;

use app::App;
use simple_logger::SimpleLogger;

fn main() -> Result<()> {
    SimpleLogger::new()
        .env()
        .init()
        .expect("failed to initialize the logger");

    winrt::init_apartment(winrt::ApartmentType::SingleThreaded)?;

    let winui_dependency = winappsdk::bootstrap::PackageDependency::initialize()?;

    log::debug!("WinUI initialized: {:?}", winui_dependency);

    Application::Start(&ApplicationInitializationCallback::new(app_start))?;

    Ok(())
}

fn app_start(_: Option<&ApplicationInitializationCallbackParams>) -> Result<()> {
    log::debug!("Application::Start");

    let app = App::new()?;
    app.UnhandledException(Some(&UnhandledExceptionEventHandler::new(
        |_sender, args| {
            match args {
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
