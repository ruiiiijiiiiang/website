use dioxus::prelude::*;

use crate::components::ProjectCard;
use crate::models::Project;

const PROJECTS: &[Project] = &[
    Project {
        name: "nixos-config",
        language: "nix",
        description: "Fully declarative and reproducible NixOS configuration for a personal homelab. Utilizing Nix Flakes, it manages infrastructure-as-code, including networking, security, and services like VPN and monitoring, ensuring atomic rollbacks and consistent, automated deployments across various hosts.",
        link: "https://github.com/ruiiiijiiiiang/nixos-config",
    },
    Project {
        name: "sdctl",
        language: "rust",
        description: "Security-focused Ratatui TUI for managing Linux systemd services. It prioritizes least privilege, utilizing Polkit for authentication instead of sudo. The application features high-performance fuzzy search, syntax-highlighted log viewing, and integrated unit file editing with intuitive Vim-style navigation.",
        link: "https://github.com/ruiiiijiiiiang/sdctl",
    },
    Project {
        name: "rs-top",
        language: "rust",
        description: "Lightweight, agentless remote system monitor providing a real-time TUI dashboard via SSH. Requiring no remote software installation, it offers read-only monitoring of multiple hosts. Built with Rust, it integrates with existing SSH configurations for efficient performance tracking.",
        link: "https://github.com/ruiiiijiiiiang/rs-top",
    },
    Project {
        name: "lazynmap",
        language: "rust",
        description: "Intuitive terminal user interface designed to simplify nmap command creation. It features interactive options, live command previews, and direct execution. With input validation, it helps users build complex network scans without memorizing cryptic command-line flags.",
        link: "https://github.com/ruiiiijiiiiang/lazynmap",
    },
    Project {
        name: "website",
        language: "rust",
        description: "What you're reading right now. Full-stack rust web application built using Dioxus that runs as a WASM client in the browser with server-side rendering (SSR) and static site generation (SSG) for content like blog posts and project pages.",
        link: "https://github.com/ruiiiijiiiiang/website",
    },
    Project {
        name: "nixos-vm-provisioner",
        language: "nix",
        description: "Provides NixOS modules for automated, host-managed virtual machine installation. It handles storage creation and first-boot provisioning using disko-install, booting guests via libvirt. It installs standard NixOS systems that independently manage their own kernels and upgrades.",
        link: "https://github.com/ruiiiijiiiiang/nixos-vm-provisioner",
    },
    Project {
        name: "file_clipper",
        language: "rust",
        description: "Rust CLI tool providing GUI-style copy, cut, and paste operations for terminal file management. It includes an interactive TUI for clipboard and history tracking, supports glob patterns, and enables symbolic link creation for efficiency.",
        link: "https://github.com/ruiiiijiiiiang/file_clipper",
    },
];

#[component]
pub fn Projects() -> Element {
    rsx! {
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
