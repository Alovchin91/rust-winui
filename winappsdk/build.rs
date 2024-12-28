use std::io::Write;

fn main() {
    // only regenerate on demand
    if !std::fs::exists("src/bindings/Interop.rs").unwrap() {
        let interop_args = [
            "--out",
            "src/bindings/Interop.rs",
            "--config",
            "flatten",
            "--filter",
            "Windows.UI.Xaml.Interop.TypeKind",
            "Windows.UI.Xaml.Interop.TypeName",
        ];
        windows_bindgen::bindgen(interop_args).unwrap();
    }

    // only regenerate on demand
    if !std::fs::exists("src/bindings/Microsoft.rs").unwrap() {
        let microsoft_args = [
            "--in",
            "winmd/Microsoft.Foundation.winmd",
            "winmd/Microsoft.Graphics.winmd",
            "winmd/Microsoft.UI.Text.winmd",
            "winmd/Microsoft.UI.winmd",
            "winmd/Microsoft.UI.Xaml.winmd",
            "winmd/Microsoft.Web.WebView2.Core.winmd",
            "winmd/Microsoft.Windows.ApplicationModel.Resources.winmd",
            "--out",
            "src/bindings/Microsoft.rs",
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
        windows_bindgen::bindgen(microsoft_args).unwrap();

        let file = std::fs::File::options()
            .read(true)
            .write(true)
            .open("src/bindings/Microsoft.rs")
            .expect("failed to open the file");

        let mut mmap = unsafe {
            memmap2::MmapOptions::new()
                .offset(2)
                .map_mut(&file)
                .expect("failed to map the file")
        };

        let mut mstr = std::str::from_utf8(&mmap).expect("failed to read file to string slice");
        while let Some(mmatch) = mstr.find("windows::UI::Xaml::Interop::TypeName") {
            (&mut mmap[mmatch..mmatch + 36])
                .write_all(b" crate::bindings::Interop::TypeName ")
                .expect("failed to replace the string");
            mstr = std::str::from_utf8(&mmap).expect("failed to read file to string slice");
        }

        let mstr = std::str::from_utf8(&mmap).expect("failed to read file to string slice");
        if let Some(mmatch) = mstr.rfind("pub fn ClearBrowsingDataAsync2") {
            (&mut mmap[mmatch..mmatch + 30])
                .write_all(b"pub fn ClearBrowsingDataAsync3")
                .expect("failed to replace the string");
        }

        mmap.flush().expect("failed to flush");
        file.sync_all().expect("failed to sync the file");
    }
}
