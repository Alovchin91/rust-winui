#![windows_subsystem = "console"]

mod app;
mod main_window;
mod winrt;
mod winui;

use winappsdk::Microsoft::UI::Xaml::{
    Application, ApplicationInitializationCallback, ApplicationInitializationCallbackParams,
    IApplication_Impl, UnhandledExceptionEventHandler,
};
use windows_core::Result;

use app::App;
use simple_logger::SimpleLogger;
use winui::WinUIDependency;

fn main() -> Result<()> {
    SimpleLogger::new()
        .env()
        .init()
        .expect("failed to initialize the logger");

    winrt::init_apartment(winrt::ApartmentType::SingleThreaded)?;

    let winui_dependency = WinUIDependency::initialize_default()?;

    log::debug!(
        "WinUI package full name: {:?}",
        winui_dependency.package_full_name()
    );

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
