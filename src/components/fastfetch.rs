use crate::models::FastfetchData;
use dioxus::prelude::*;

const NIXOS_ASCII: &str = r#"          ▗▄▄▄       ▗▄▄▄▄    ▄▄▄▖
          ▜███▙       ▜███▙  ▟███▛
           ▜███▙       ▜███▙▟███▛
            ▜███▙       ▜██████▛
     ▟█████████████████▙ ▜████▛     ▟▙
    ▟███████████████████▙ ▜███▙    ▟██▙
           ▄▄▄▄▖           ▜███▙  ▟███▛
          ▟███▛             ▜██▛ ▟███▛
         ▟███▛               ▜▛ ▟███▛
▟███████████▛                  ▟██████████▙
▜██████████▛                  ▟███████████▛
      ▟███▛ ▟▙               ▟███▛
     ▟███▛ ▟██▙             ▟███▛
    ▟███▛  ▜███▙           ▝▀▀▀▀
    ▜██▛    ▜███▙ ▜██████████████████▛
     ▜▛     ▟████▙ ▜████████████████▛
           ▟██████▙         ▜███▙
          ▟███▛▜███▙         ▜███▙
         ▟███▛  ▜███▙         ▜███▙
         ▝▀▀▀    ▀▀▀▀▘         ▀▀▀▘"#;

#[component]
pub fn FastfetchCard(data: FastfetchData) -> Element {
    rsx! {
        div {
            class: "fastfetch-layout",

            pre {
                class: "fastfetch-logo",
                "{NIXOS_ASCII}"
            }

            div {
                class: "fastfetch-info",

                div {
                    class: "fastfetch-divider",
                }

                div {
                    class: "fastfetch-info-grid",

                    div { class: "fastfetch-key", "OS" }
                    div { class: "fastfetch-val", "{data.os_name}" }

                    div { class: "fastfetch-key", "Kernel" }
                    div { class: "fastfetch-val", "{data.kernel}" }

                    div { class: "fastfetch-key", "Uptime" }
                    div { class: "fastfetch-val", "{data.uptime}" }

                    div { class: "fastfetch-key", "Packages" }
                    div { class: "fastfetch-val", "{data.packages} (nix)" }

                    div { class: "fastfetch-key", "Age" }
                    div { class: "fastfetch-val", "{data.os_age}" }

                    div { class: "fastfetch-key", "CPU" }
                    div { class: "fastfetch-val", "{data.cpu_model}" }

                    div { class: "fastfetch-key", "CPU Load" }
                    div {
                        class: "fastfetch-val",
                        style: "display: flex; align-items: center; gap: 0.5rem;",
                        progress { value: "{data.cpu_load}", max: "100", style: "margin-bottom: 0; max-width: 150px;" }
                        span { "{data.cpu_load}%" }
                    }

                    div { class: "fastfetch-key", "RAM" }
                    div {
                        class: "fastfetch-val",
                        style: "display: flex; align-items: center; gap: 0.5rem;",
                        progress { value: "{data.ram_pct}", max: "100", style: "margin-bottom: 0; max-width: 150px;" }
                        span { "{data.ram_pct}%" }
                    }

                    div { class: "fastfetch-key", "Disk" }
                    div {
                        class: "fastfetch-val",
                        style: "display: flex; align-items: center; gap: 0.5rem;",
                        progress { value: "{data.disk_pct}", max: "100", style: "margin-bottom: 0; max-width: 150px;" }
                        span { "{data.disk_pct}%" }
                    }

                    div { class: "fastfetch-key", "Fetched At" }
                    div { class: "fastfetch-val", "{data.fetched_at}" }
                }
                div {
                    class: "fastfetch-divider",
                }
            }
        }
    }
}
