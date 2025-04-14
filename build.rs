const COMMANDS: &[&str] = &[
    "all_sys_info",
    "total_memory",
    "used_memory",
    "total_swap",
    "used_swap",
    "memory_info",
    "hostname",
    "name",
    "kernel_version",
    "os_version",
    "static_info",
    "components",
    "cpus",
    "cpu_count",
    "cpu_info",
    "disks",
    "networks",
    "processes",
    "refresh_all",
    "refresh_memory",
    "refresh_cpu",
    "refresh_processes",
    "batteries",
    "load_average",
    "uptime",
];

fn main() {
    let result = tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .try_build();

    // when building documentation for Android the plugin build result is always Err() and is irrelevant to the crate documentation build
    if !(cfg!(docsrs) && std::env::var("TARGET").unwrap().contains("android")) {
        result.unwrap();
    }
}
