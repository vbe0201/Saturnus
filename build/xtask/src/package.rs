//! Package definitions for the build system.

/// The Kernel package in the Saturnus workspace.
pub const KERNEL: Package = Package {
    name: "kernel",
    cargo_name: "kernel",
};

/// The Kernel Loader package in the Saturnus workspace.
pub const KERNEL_LOADER: Package = Package {
    name: "loader",
    cargo_name: "kernel-loader",
};

/// Definition of a Saturnus package to build.
#[derive(Clone, Copy, Debug)]
pub struct Package {
    /// The name of the package.
    pub name: &'static str,
    /// The package name in the cargo workspace.
    ///
    /// This will be passed as the `-p` argument during build.
    pub cargo_name: &'static str,
}

/// Gets an iterator over all the Saturnus [`Package`]s.
pub fn all_packages() -> impl Iterator<Item = &'static Package> {
    [KERNEL, KERNEL_LOADER].iter()
}
