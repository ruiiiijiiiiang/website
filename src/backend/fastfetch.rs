use dioxus::prelude::*;

use crate::models::FastfetchData;

#[server]
pub async fn get_fastfetch_data() -> Result<FastfetchData, ServerFnError> {
    use std::fs;
    use std::process::Command;

    let cpu_model = fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|info| {
            info.lines()
                .find(|line| line.starts_with("model name"))
                .and_then(|line| line.split(':').nth(1))
                .map(|name| name.trim().to_string())
        })
        .unwrap_or_else(|| "Unknown CPU".to_string());

    let cores = fs::read_to_string("/proc/cpuinfo")
        .map(|info| {
            info.lines()
                .filter(|line| line.starts_with("processor"))
                .count()
                .max(1)
        })
        .unwrap_or(1);

    let load_1m = fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|s| {
            s.split_whitespace()
                .next()
                .and_then(|val| val.parse::<f64>().ok())
        })
        .unwrap_or(0.0);

    let cpu_load = (((load_1m / cores as f64) * 100.0).round() as u8).min(100);

    let uptime_secs = fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|s| {
            s.split_whitespace()
                .next()
                .and_then(|val| val.parse::<f64>().ok())
        })
        .unwrap_or(0.0) as u64;
    let days = uptime_secs / 86400;
    let hours = (uptime_secs % 86400) / 3600;
    let minutes = (uptime_secs % 3600) / 60;
    let uptime = if days > 0 {
        format!("{}d, {}h, {}m", days, hours, minutes)
    } else {
        format!("{}h, {}m", hours, minutes)
    };

    let mut mem_total = 1;
    let mut mem_available = 0;
    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                mem_total = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(1);
            } else if line.starts_with("MemAvailable:") {
                mem_available = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(0);
            }
        }
    }
    let ram_pct = (((mem_total - mem_available) as f64 / mem_total as f64) * 100.0) as u8;

    let disk_pct = Command::new("df")
        .args(["-h", "/"])
        .output()
        .ok()
        .and_then(|output| {
            String::from_utf8(output.stdout).ok().and_then(|stdout| {
                stdout.lines().nth(1).and_then(|line| {
                    line.split_whitespace()
                        .nth(4)
                        .and_then(|pct| pct.trim_end_matches('%').parse::<u8>().ok())
                })
            })
        })
        .unwrap_or(0);

    let os_name = fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|content| {
            content
                .lines()
                .find(|line| line.starts_with("PRETTY_NAME="))
                .and_then(|line| line.split('=').nth(1))
                .map(|name| name.trim_matches('"').to_string())
        })
        .unwrap_or_else(|| "NixOS".to_string());

    let kernel_release = fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());
    let kernel = format!("Linux {}", kernel_release);

    let os_age_days = fs::metadata("/etc/NIXOS")
        .or_else(|_| fs::metadata("/etc/hostname"))
        .or_else(|_| fs::metadata("/lost+found"))
        .and_then(|m| m.modified().or_else(|_| m.created()))
        .ok()
        .map(|time| {
            std::time::SystemTime::now()
                .duration_since(time)
                .map(|d| d.as_secs() / 86400)
                .unwrap_or(120)
        })
        .unwrap_or(120);
    let os_age = format!("{} days", os_age_days);

    let sys_packages = fs::read_dir("/run/current-system/sw/bin")
        .or_else(|_| fs::read_dir("/usr/bin"))
        .map(|entries| entries.count())
        .ok();

    let packages = match sys_packages {
        Some(count) if count > 0 => count.to_string(),
        _ => "Unknown".to_string(),
    };

    let fetched_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    Ok(FastfetchData {
        cpu_model,
        cpu_load,
        ram_pct,
        disk_pct,
        uptime,
        os_name,
        kernel,
        os_age,
        packages,
        fetched_at,
    })
}
