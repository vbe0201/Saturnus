//! Definition of a package inside the Saturnus project (e.g.: kernel, loader)

const PACKAGES: &[Package] = &[
    Package {
        name: "kernel",
        cargo_name: "kernel",
        target: "targets/aarch64-saturnus-none.json",
    },
    Package {
        name: "loader",
        cargo_name: "kernel-loader",
        target: "targets/aarch64-saturnus-none.json",
    },
];

/// Contains everything that is required to build and run a package.
#[derive(Debug, Clone)]
pub struct Package {
    /// The name of this package.
    pub name: &'static str,
    /// The name of the package inside the cargo workspace.
    /// This value will be passed as the `-p` value to cargo
    pub cargo_name: &'static str,
    /// The rust target to use for this package.
    pub target: &'static str,
}

/// Try to find a package with the given name.
pub fn find_package(name: &str) -> Option<Package> {
    PACKAGES
        .iter()
        .find(|p| p.name.eq_ignore_ascii_case(name))
        .cloned()
}
