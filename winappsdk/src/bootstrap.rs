use std::{fmt, sync::OnceLock};
use windows::{
    core::{h, Result, HSTRING, PWSTR},
    Win32::Storage::Packaging::Appx::{
        AddPackageDependency, AddPackageDependencyOptions_None,
        CreatePackageDependencyOptions_None, PackageDependencyLifetimeKind_Process,
        PackageDependencyProcessorArchitectures_None, RemovePackageDependency,
        TryCreatePackageDependency, PACKAGEDEPENDENCY_CONTEXT, PACKAGE_VERSION, PACKAGE_VERSION_0,
    },
};

const WINDOWSAPPSDK_RUNTIME_VERSION_UINT64: u64 = 0x1770013E09000000u64;
const WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME: &HSTRING =
    h!("Microsoft.WindowsAppRuntime.1.6_8wekyb3d8bbwe");

#[derive(Debug)]
struct PackageDependencyID(PWSTR);

unsafe impl Sync for PackageDependencyID {}
unsafe impl Send for PackageDependencyID {}

pub struct PackageDependency {
    ctx: PACKAGEDEPENDENCY_CONTEXT,
    package_full_name: HSTRING,
}

impl PackageDependency {
    pub fn initialize() -> Result<Self> {
        static RUNTIME_PACKAGE_FRAMEWORK_DEPENDENCY_ID: OnceLock<PackageDependencyID> =
            OnceLock::new();

        let dependency_id = match RUNTIME_PACKAGE_FRAMEWORK_DEPENDENCY_ID.get() {
            Some(dependency_id) => dependency_id,
            None => {
                let min_version = PACKAGE_VERSION {
                    Anonymous: PACKAGE_VERSION_0 {
                        Version: WINDOWSAPPSDK_RUNTIME_VERSION_UINT64,
                    },
                };
                let dependency_id = unsafe {
                    TryCreatePackageDependency(
                        None,
                        WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME,
                        min_version,
                        PackageDependencyProcessorArchitectures_None,
                        PackageDependencyLifetimeKind_Process,
                        None,
                        CreatePackageDependencyOptions_None,
                    )
                }?;
                RUNTIME_PACKAGE_FRAMEWORK_DEPENDENCY_ID
                    .get_or_init(|| PackageDependencyID(dependency_id))
            }
        };

        let mut ctx = PACKAGEDEPENDENCY_CONTEXT::default();
        let mut package_full_name = PWSTR::null();

        unsafe {
            AddPackageDependency(
                dependency_id.0,
                0,
                AddPackageDependencyOptions_None,
                &mut ctx,
                Some(&mut package_full_name),
            )
        }?;

        Ok(Self {
            ctx,
            package_full_name: unsafe { package_full_name.to_hstring() }?,
        })
    }

    fn uninitialize(&self) -> Result<()> {
        unsafe { RemovePackageDependency(self.ctx) }
    }
}

impl fmt::Debug for PackageDependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PackageDependency")
            .field("package_full_name", &self.package_full_name)
            .finish_non_exhaustive()
    }
}

impl Drop for PackageDependency {
    fn drop(&mut self) {
        self.uninitialize()
            .expect("failed to remove package dependency")
    }
}
