pub fn init_logger() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            console_log::init_with_level(log::Level::Debug).expect("Failed to init console_log");
        } else {
            env_logger::builder()
                .filter_level(log::LevelFilter::Info)
                .filter_module("wgpu_core", log::LevelFilter::Warn)
                .filter_module("wgpu_hal", log::LevelFilter::Warn)
                .parse_default_env()
                .init();
        }
    }
}
