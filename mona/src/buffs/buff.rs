use crate::attribute::Attribute;
use crate::buffs::buff_meta::BuffMetaData;
use crate::buffs::BuffConfig;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub trait Buff<T: Attribute> {
    fn change_enemy(&self, enemy: &mut Enemy) {}

    fn change_attribute(&self, attribute: &mut T) {}
}

pub trait BuffMeta {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData;

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = None;

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>>;
}
