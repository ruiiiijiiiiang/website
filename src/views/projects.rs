use dioxus::prelude::*;

use crate::components::ProjectCard;
use crate::models::Project;

const PROJECTS: &[Project] = &[
    Project {
        name: "nixos-config",
        language: "nix",
        category: "Platform Engineering",
        tagline: "Declarative infrastructure platform for a multi-host homelab.",
        problem: "Operating a mixed fleet of workstations, a hypervisor, and service VMs needs to be reproducible, reviewable, and recoverable.",
        solution: "A Nix Flake-based platform that defines hosts, network zones, virtualization, application services, observability, and security controls as code.",
        highlights: &[
            "GitOps-style deployments with atomic NixOS activations",
            "Network segmentation, WireGuard access, and declarative libvirt/NixVirt",
            "Centralized metrics, logs, uptime monitoring, and security monitoring",
        ],
        stack: &[
            "NixOS",
            "Nix Flakes",
            "libvirt/NixVirt",
            "WireGuard",
            "VLANs",
            "Kea DHCP",
            "Pi-hole/Unbound",
            "Keepalived",
            "Cloudflare Tunnels",
            "Prometheus/Loki",
            "Wazuh",
        ],
        link: "https://github.com/ruiiiijiiiiang/nixos-config",
        screenshot: Some(
            "https://raw.githubusercontent.com/ruiiiijiiiiang/nixos-config/master/topology/topology.png",
        ),
        screenshot_alt: Some(
            "Network and service topology for the declarative NixOS homelab platform",
        ),
        featured: true,
    },
    Project {
        name: "wazuh-flake",
        language: "nix",
        category: "Security Platform",
        tagline: "Reusable, native NixOS modules for operating a Wazuh SIEM stack.",
        problem: "Operating an SIEM involves coordinated services, credentials, persistent data, backups, and recovery procedures that are difficult to manage consistently.",
        solution: "Native NixOS modules for Wazuh agents, manager, indexer, dashboard, and Filebeat, with operational guidance for health checks, backup, recovery, rotation, and rollback.",
        highlights: &[
            "Automates certificate and internal-credential provisioning",
            "Encodes multi-service SIEM deployment as reusable NixOS modules",
            "Documents production operations, persistent state, recovery, and upgrades",
        ],
        stack: &["NixOS", "Wazuh", "Filebeat", "TLS/PKI", "OpenSearch"],
        link: "https://github.com/ruiiiijiiiiang/wazuh-flake",
        screenshot: None,
        screenshot_alt: None,
        featured: true,
    },
    Project {
        name: "nixos-cis-validator",
        language: "nix",
        category: "Security Automation",
        tagline: "CIS-aligned configuration validation before deployment.",
        problem: "Security-baseline drift is costly to discover after a system has been deployed.",
        solution: "A NixOS module that evaluates merged configurations against versioned CIS-aligned benchmarks and produces machine-readable reports at build time.",
        highlights: &[
            "Policy-as-code with versioned benchmark definitions",
            "Report, warn, and error modes to fit progressive enforcement",
            "Build-time JSON reports make results usable in automation",
        ],
        stack: &[
            "NixOS",
            "CIS Benchmarks",
            "Shift-left Security",
            "Policy as Code",
        ],
        link: "https://github.com/ruiiiijiiiiang/nixos-cis-validator",
        screenshot: None,
        screenshot_alt: None,
        featured: true,
    },
    Project {
        name: "nixos-vm-provisioner",
        language: "nix",
        category: "Platform Engineering",
        tagline: "Declarative, host-managed provisioning for autonomous NixOS VMs.",
        problem: "Traditional libvirt VM setup fragments disk allocation, installation, and lifecycle management into manual steps.",
        solution: "NixOS host and guest modules that allocate storage, run first-boot Disko installation, and then leave each guest in control of its own upgrades and boot lifecycle.",
        highlights: &[
            "Zero-manual-ISO provisioning using disko-install",
            "Supports file, LVM, and raw-block storage backends",
            "Separates host provisioning concerns from guest lifecycle ownership",
        ],
        stack: &["NixOS", "NixVirt", "libvirt", "Disko", "systemd-boot"],
        link: "https://github.com/ruiiiijiiiiang/nixos-vm-provisioner",
        screenshot: None,
        screenshot_alt: None,
        featured: true,
    },
    Project {
        name: "sdctl",
        language: "rust",
        category: "Security Tooling",
        tagline: "Least-privilege systemd management from the terminal.",
        problem: "Routine service administration should not require persistent root shells or unsafe privilege escalation.",
        solution: "A Rust TUI for managing system and user units through Polkit-authenticated actions instead of sudo.",
        highlights: &[
            "Uses Polkit for privileged actions rather than sudo",
            "Combines unit control, logs, configuration inspection, and drop-in overrides",
            "Built for keyboard-first operational workflows",
        ],
        stack: &["Ratatui", "Tokio", "D-Bus", "Polkit"],
        link: "https://github.com/ruiiiijiiiiang/sdctl",
        screenshot: Some(
            "https://github.com/user-attachments/assets/16267839-1349-4ea4-a00f-89d763cd8d5a",
        ),
        screenshot_alt: Some(
            "sdctl terminal interface showing systemd unit status and service controls",
        ),
        featured: false,
    },
    Project {
        name: "rs-top",
        language: "rust",
        category: "Observability Tooling",
        tagline: "Agentless, read-only monitoring for remote Linux hosts.",
        problem: "Operators need quick visibility across hosts without installing agents or granting unnecessary privileges.",
        solution: "An SSH-based terminal dashboard that collects system statistics using standard tools already present on remote Linux hosts.",
        highlights: &[
            "No remote installation or sudo required",
            "Reuses existing SSH configuration, keys, and known-hosts trust",
            "Concurrent remote monitoring with a terminal-native interface",
        ],
        stack: &["SSH", "Ratatui", "Tokio"],
        link: "https://github.com/ruiiiijiiiiang/rs-top",
        screenshot: Some(
            "https://raw.githubusercontent.com/ruiiiijiiiiang/rs-top/refs/heads/screenshot/screenshot.png",
        ),
        screenshot_alt: Some("rs-top terminal dashboard monitoring remote Linux hosts over SSH"),
        featured: false,
    },
    Project {
        name: "website",
        language: "rust",
        category: "Platform Delivery",
        tagline: "A full-stack personal site with a 0 JavaScript / TypeScript frontend.",
        problem: "A personal site should be fast, maintainable, and deployable with the same reproducibility expectations as the rest of the platform.",
        solution: "A Dioxus application with WASM, server-side rendering, static generation, Markdown publishing, and a reproducible Nix build.",
        highlights: &[
            "0 JavaScript / TypeScript frontend: Rust compiles directly to WebAssembly",
            "Full-stack rendering with SSR fallback and static generation",
            "Markdown publishing pipeline with custom Nix syntax highlighting",
            "Reproducible Nix build and homelab deployment",
        ],
        stack: &["Dioxus", "WASM", "Nix", "SSG"],
        link: "https://github.com/ruiiiijiiiiang/website",
        screenshot: None,
        screenshot_alt: None,
        featured: false,
    },
    Project {
        name: "lazynmap",
        language: "rust",
        category: "Security Tooling",
        tagline: "Interactive, validated nmap command construction.",
        problem: "Network scanning options are powerful but error-prone and difficult to compose safely from memory.",
        solution: "A terminal interface that builds nmap commands interactively with live previews, option validation, and direct execution.",
        highlights: &[
            "Live command previews make scan intent inspectable before execution",
            "Validates values for scan flags to reduce common mistakes",
            "Keyboard-first workflow for security testing",
        ],
        stack: &["Nmap", "Terminal UI"],
        link: "https://github.com/ruiiiijiiiiang/lazynmap",
        screenshot: Some(
            "https://raw.githubusercontent.com/ruiiiijiiiiang/lazynmap/master/assets/lazynmap.png",
        ),
        screenshot_alt: Some("lazynmap terminal interface building an nmap scan command"),
        featured: false,
    },
    Project {
        name: "file_clipper",
        language: "rust",
        category: "Developer Tooling",
        tagline: "Terminal-native file operations with an interactive clipboard.",
        problem: "Shell file operations become cumbersome when workflows need persistent selections, history, globbing, and symlink support.",
        solution: "A command-line tool with copy, move, link, paste, and history operations backed by an interactive terminal clipboard.",
        highlights: &[
            "Supports copy, move, and symlink workflows",
            "Interactive clipboard and operation history",
            "Accepts glob patterns and paths piped from other commands",
        ],
        stack: &["CLI", "Terminal UI"],
        link: "https://github.com/ruiiiijiiiiang/file_clipper",
        screenshot: Some(
            "https://raw.githubusercontent.com/ruiiiijiiiiang/file_clipper/master/assets/tui.png",
        ),
        screenshot_alt: Some(
            "file_clipper terminal interface showing clipboard history and file selection",
        ),
        featured: false,
    },
];

const PROJECTS_CSS: Asset = asset!("../../assets/projects.css");

#[component]
pub fn Projects() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: PROJECTS_CSS }
        document::Title { "Rui's Projects" }

        div {
            h1 { "Projects" }
            p { "Selected work in platform engineering, security automation, observability, and developer tooling." }

            div {
                for project in PROJECTS {
                    ProjectCard {
                        project: project.clone()
                    }
                }
            }
        }
    }
}
