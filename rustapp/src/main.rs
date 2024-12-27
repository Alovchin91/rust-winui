#![windows_subsystem = "console"]

mod app;
mod main_window;
mod winrt;
mod winui;

use app::App;
use simple_logger::SimpleLogger;
use windows::core::Result;

use winappsdk::Microsoft::UI::Xaml::{Application, ApplicationInitializationCallback};
use winui::WinUIDependency;

fn main() -> Result<()> {
    SimpleLogger::new()
        .env()
        .init()
        .expect("failed to initialize the logger");

    let _roinit = winrt::init_apartment()?;

    let winui_dependency = WinUIDependency::initialize_default()?;

    log::debug!(
        "WinUI package full name: {:?}",
        winui_dependency.package_full_name()
    );

    Application::Start(&ApplicationInitializationCallback::new(|_| {
        log::debug!("Application::Start");
        let _app = App::new()?;
        Ok(())
    }))?;

    Ok(())
}
