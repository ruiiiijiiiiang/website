use dioxus::prelude::*;

use crate::components::ProjectCard;
use crate::models::Project;

const PROJECTS: &[Project] = &[
    Project {
        name: "nixos-config",
        language: "nix",
        description: "Declarative NixOS configuration for my personal homelab. Powered by Nix Flakes. Manages host infrastructure, networking, security, and application services. Automated git-ops deployment across multiple machines.",
        link: "https://github.com/ruiiiijiiiiang/nixos-config",
        screenshot: Some(
            "https://raw.githubusercontent.com/ruiiiijiiiiang/nixos-config/master/topology/topology.png",
        ),
    },
    Project {
        name: "sdctl",
        language: "rust",
        description: "Security-focused systemd manager TUI. Built with Ratatui. Prioritizes least privilege by utilizing Polkit for authentication instead of sudo. Features fuzzy search, syntax-highlighted log viewing, and Vim-style navigation.",
        link: "https://github.com/ruiiiijiiiiang/sdctl",
        screenshot: Some(
            "https://github.com/user-attachments/assets/16267839-1349-4ea4-a00f-89d763cd8d5a",
        ),
    },
    Project {
        name: "rs-top",
        language: "rust",
        description: "Lightweight, agentless remote system monitor. Provides a real-time terminal dashboard via SSH. Read-only view for multiple hosts with zero remote dependencies. Integrates with existing SSH configurations.",
        link: "https://github.com/ruiiiijiiiiang/rs-top",
        screenshot: Some(
            "https://raw.githubusercontent.com/ruiiiijiiiiang/rs-top/refs/heads/screenshot/screenshot.png",
        ),
    },
    Project {
        name: "nixos-vm-provisioner",
        language: "nix",
        description: "NixOS modules for automated, host-managed VM provisioning. Handles disk layout creation and first-boot installation via disko-install. Boots guests under libvirt/NixVirt. Installs standard NixOS systems that manage their own kernels and upgrades.",
        link: "https://github.com/ruiiiijiiiiang/nixos-vm-provisioner",
        screenshot: None,
    },
    Project {
        name: "lazynmap",
        language: "rust",
        description: "Interactive nmap assistant TUI. Simplifies command creation with live previews, option toggles, and direct execution. Eliminates the need to memorize cryptic network scanning flags.",
        link: "https://github.com/ruiiiijiiiiang/lazynmap",
        screenshot: Some(
            "https://raw.githubusercontent.com/ruiiiijiiiiang/lazynmap/master/assets/lazynmap.png",
        ),
    },
    Project {
        name: "website",
        language: "rust",
        description: "What you're reading right now. Full-stack web application built using Dioxus with server-side rendering and static site generation. WASM front end; 100% JS/TS free. Served from my personal homelab.",
        link: "https://github.com/ruiiiijiiiiang/website",
        screenshot: None,
    },
    Project {
        name: "file_clipper",
        language: "rust",
        description: "GUI-style file operations for terminal file management. Provides copy, cut, and paste commands with an interactive clipboard and history tracking TUI. Supports glob patterns and symlinks.",
        link: "https://github.com/ruiiiijiiiiang/file_clipper",
        screenshot: Some(
            "https://raw.githubusercontent.com/ruiiiijiiiiang/file_clipper/master/assets/tui.png",
        ),
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
            p { "Here are some of the projects I've been working on." }

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
