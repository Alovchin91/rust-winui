use std::{env, fs, io::Write, str};

fn main() -> Result<(), &'static str> {
    env::var("CARGO")
        .map_err(|_| r"please run this tool with `cargo run -p bindgen` from the workspace root")?;

    if !fs::exists("bindgen/winmd").expect("failed to check if winmd dir exists") {
        return Err("please make sure to put Windows App SDK metadata in the bindgen/winmd dir");
    }

    println!("Generating Windows.UI.Xaml.Interop bindings...");
    let interop_args = [
        "--out",
        "winappsdk/src/bindings/Interop.rs",
        "--config",
        "flatten",
        "--filter",
        "Windows.UI.Xaml.Interop.TypeKind",
        "Windows.UI.Xaml.Interop.TypeName",
    ];
    windows_bindgen::bindgen(interop_args).expect("failed to write Interop.rs");

    println!("Generating Windows App SDK bindings...");
    let microsoft_args = [
        "--in",
        "bindgen/winmd/Microsoft.Foundation.winmd",
        "bindgen/winmd/Microsoft.Graphics.winmd",
        "bindgen/winmd/Microsoft.UI.Text.winmd",
        "bindgen/winmd/Microsoft.UI.winmd",
        "bindgen/winmd/Microsoft.UI.Xaml.winmd",
        "bindgen/winmd/Microsoft.Web.WebView2.Core.winmd",
        "bindgen/winmd/Microsoft.Windows.ApplicationModel.Resources.winmd",
        "--out",
        "winappsdk/src/bindings/Microsoft.rs",
        "--config",
        "implement",
        "--filter",
        "Microsoft.Graphics",
        "Microsoft.UI",
        "Microsoft.Windows.ApplicationModel.Resources",
        // WebView2 is a big part of the API surface
        "!Microsoft.UI.Xaml.Automation.Peers.IWebView2AutomationPeer",
        "!Microsoft.UI.Xaml.Automation.Peers.IWebView2AutomationPeerFactory",
        "!Microsoft.UI.Xaml.Automation.Peers.WebView2AutomationPeer",
        "!Microsoft.UI.Xaml.Controls.IWebView2",
        "!Microsoft.UI.Xaml.Controls.IWebView22",
        "!Microsoft.UI.Xaml.Controls.IWebView2Factory",
        "!Microsoft.UI.Xaml.Controls.IWebView2Statics",
        "!Microsoft.UI.Xaml.Controls.WebView2",
        "!Microsoft.Web.WebView2",
    ];
    windows_bindgen::bindgen(microsoft_args).expect("failed to write Microsoft.rs");

    println!("Patching Windows App SDK bindings...");

    let file = fs::File::options()
        .read(true)
        .write(true)
        .open("winappsdk/src/bindings/Microsoft.rs")
        .expect("failed to open the file");

    let mut mmap = unsafe {
        memmap2::MmapOptions::new()
            .offset(2)
            .map_mut(&file)
            .expect("failed to map the file")
    };

    loop {
        let mstr = str::from_utf8(&mmap).expect("failed to read file to string slice");
        match mstr.find("windows::UI::Xaml::Interop::TypeName") {
            Some(mmatch) => (&mut mmap[mmatch..mmatch + 36])
                .write_all(b" crate::bindings::Interop::TypeName ")
                .expect("failed to replace the string"),
            None => break,
        }
    }

    mmap.flush().expect("failed to flush");
    file.sync_all().expect("failed to sync the file");

    println!("Done.");
    Ok(())
}
