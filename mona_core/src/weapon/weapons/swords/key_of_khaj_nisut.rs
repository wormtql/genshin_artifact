use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct KeyOfKhajNisutEffect {
    pub stack: f64,
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for KeyOfKhajNisutEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        attribute.add_hp_percentage("圣显之钥被动", 0.05 * refine + 0.15);
        let em_bonus = (0.0003 * refine + 0.0009) * self.stack;
        attribute.add_edge1(
            AttributeName::HP,
            AttributeName::ElementalMastery,
            Box::new(move |hp, _| hp * em_bonus),
            Box::new(move |hp, _, grad| (em_bonus, 0.0)),
            "圣显之钥被动等效"
        );

        let em_bonus2 = (0.0005 * refine + 0.0015) * self.rate;
        attribute.add_edge1(
            AttributeName::HP,
            AttributeName::ElementalMasteryExtra,
            Box::new(move |hp, _| hp * em_bonus2),
            Box::new(move |hp, _, grad| (0.0, 0.0)),
            "圣显之钥被动等效"
        );
    }
}

pub struct KeyOfKhajNisut;

impl WeaponTrait for KeyOfKhajNisut {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::KeyOfKhajNisut,
        internal_name: "Sword_Deshret",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP144),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "生命值提升<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>。元素战技命中敌人时，将产生持续20秒的「宏大诗篇」效果：基于装备者生命值上限的<span style=\"color: #409EFF;\">0.12%-0.15%-0.18%-0.21%-0.24%</span>，获得元素精通提升，该效果每0.3秒至多触发一次，至多叠加3层。该效果叠加至3层或3层的持续时间刷新时，将基于装备者生命值上限的<span style=\"color: #409EFF;\">0.2%-0.25%-0.3%-0.35%-0.4%</span>，为队伍中附近所有角色提供元素精通提升，持续20秒。",
            en: "HP increased by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>. When an Elemental Skill hits opponents, you gain the Grand Hymn effect for 20s. This effect increases the equipping character's Elemental Mastery by <span style=\"color: #409EFF;\">0.12%-0.15%-0.18%-0.21%-0.24%</span> of their Max HP. This effect can trigger once every 0.3s. Max 3 stacks. When this effect gains 3 stacks, or when the third stack's duration is refreshed, the Elemental Mastery of all nearby party members will be increased by <span style=\"color: #409EFF;\">0.2%-0.25%-0.3%-0.35%-0.4%</span> of the equipping character's max HP for 20s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "圣显之钥",
            en: "Key of Khaj-Nisut"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK03,
        ItemConfig {
            name: "rate",
            title: locale!(
                zh_cn: "第二个效果比例",
                en: "Second effect's Ratio"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.5 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (stack, rate) = match *config {
            WeaponConfig::KeyOfKhajNisut { stack, rate } => (stack, rate),
            _ => (0.0, 0.0)
        };

        Some(Box::new(KeyOfKhajNisutEffect { stack, rate }))
    }
}
