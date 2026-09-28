//! `cargo xtask layers`: the crate table of the Phase 6 plan's §2.1, made mechanical.

mod check_layers;
mod members;
mod read_manifest;
mod run;

pub(crate) use run::layers;

/// One workspace member's `Cargo.toml`, read textually.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Manifest {
    /// `[package] name`.
    pub(crate) name: String,
    /// Every name in the `[dependencies]` table, in file order.
    pub(crate) deps: Vec<String>,
    /// Whether `[lints]` says `workspace = true`.
    pub(crate) lints_workspace: bool,
}

/// Crates above the renderer boundary: exact, no float, no wgpu.
pub(crate) const ABOVE: &[&str] = &[
    "joinn-frame",
    "joinn-dna",
    "joinn-gate",
    "joinn-prim",
    "joinn-live",
    "joinn-assay",
    "joinn-link",
    "joinn-host",
    "joinn-test-host",
    "joinn-cli",
    "joinn-visual",
];

/// Crates below the boundary. Only these hold their own `[lints]` table.
pub(crate) const BELOW: &[&str] = &["joinn-gpu", "joinn-shell-desktop"];

/// Workspace edges §2.1 allows for each new crate.
pub(crate) const EDGES: &[(&str, &[&str])] = &[
    (
        "joinn-visual",
        &[
            "joinn-frame",
            "joinn-dna",
            "joinn-link",
            "joinn-host",
            "joinn-live",
        ],
    ),
    ("joinn-gpu", &["joinn-frame", "joinn-visual"]),
    (
        "joinn-shell-desktop",
        &[
            "joinn-frame",
            "joinn-dna",
            "joinn-gate",
            "joinn-prim",
            "joinn-live",
            "joinn-link",
            "joinn-host",
            "joinn-visual",
            "joinn-gpu",
        ],
    ),
];

/// External crates §2.2 admits, each with the crates that may use it.
pub(crate) const EXTERNAL: &[(&str, &[&str])] = &[
    ("wgpu", &["joinn-gpu", "joinn-shell-desktop"]),
    ("winit", &["joinn-shell-desktop"]),
    ("pollster", &["joinn-gpu", "joinn-shell-desktop"]),
    ("bytemuck", &["joinn-gpu"]),
];
