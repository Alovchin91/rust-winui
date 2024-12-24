use std::sync::OnceLock;
use windows::{
    core::{Result, HSTRING, PWSTR},
    Win32::{
        Foundation::E_UNEXPECTED,
        Storage::Packaging::Appx::{
            AddPackageDependency, AddPackageDependencyOptions_None,
            CreatePackageDependencyOptions_None, PackageDependencyLifetimeKind_Process,
            PackageDependencyProcessorArchitectures_None, RemovePackageDependency,
            TryCreatePackageDependency, PACKAGEDEPENDENCY_CONTEXT, PACKAGE_VERSION,
            PACKAGE_VERSION_0, PACKAGE_VERSION_0_0,
        },
    },
};

#[derive(Debug)]
struct WinUIDependencyID(PWSTR);

unsafe impl Sync for WinUIDependencyID {}
unsafe impl Send for WinUIDependencyID {}

pub(crate) struct WinUIDependency {
    ctx: PACKAGEDEPENDENCY_CONTEXT,
    package_full_name: PWSTR,
}

impl WinUIDependency {
    pub(crate) fn initialize_default() -> Result<Self> {
        Self::initialize(6u16)
    }

    pub(crate) fn initialize(minor_version: u16) -> Result<Self> {
        static WINUI_DEPID: OnceLock<WinUIDependencyID> = OnceLock::new();

        if WINUI_DEPID.get().is_none() {
            let min_version = PACKAGE_VERSION {
                Anonymous: PACKAGE_VERSION_0 {
                    Anonymous: PACKAGE_VERSION_0_0 {
                        Minor: minor_version,
                        ..Default::default()
                    },
                },
            };
            let package_family_name =
                format!("Microsoft.WindowsAppRuntime.1.{minor_version}_8wekyb3d8bbwe");
            let dependency_id = unsafe {
                TryCreatePackageDependency(
                    None,
                    &HSTRING::from(package_family_name),
                    min_version,
                    PackageDependencyProcessorArchitectures_None,
                    PackageDependencyLifetimeKind_Process,
                    None,
                    CreatePackageDependencyOptions_None,
                )
            }?;
            WINUI_DEPID
                .set(WinUIDependencyID(dependency_id))
                .map_err(|_| E_UNEXPECTED)?;
        }

        let mut ctx = PACKAGEDEPENDENCY_CONTEXT::default();
        let mut package_full_name = PWSTR::null();

        unsafe {
            AddPackageDependency(
                WINUI_DEPID.get().expect("it should be set").0,
                0,
                AddPackageDependencyOptions_None,
                &mut ctx,
                Some(&mut package_full_name),
            )
        }?;

        Ok(Self {
            ctx,
            package_full_name,
        })
    }

    pub(crate) fn package_full_name(&self) -> String {
        unsafe { self.package_full_name.to_string() }
            .unwrap_or_else(|_| "unknown package name".to_owned())
    }

    pub(crate) fn uninitialize(&self) -> Result<()> {
        unsafe { RemovePackageDependency(self.ctx) }
    }
}

impl Drop for WinUIDependency {
    fn drop(&mut self) {
        self.uninitialize()
            .expect("failed to remove package dependency")
    }
}
