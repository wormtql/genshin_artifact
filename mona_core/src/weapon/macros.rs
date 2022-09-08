macro_rules! config_rate01 {
    () => {
        #[cfg(not(target_family = "wasm"))]
        const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
            ItemConfig::RATE01
        ]);
    }
}

pub(crate) use config_rate01;
