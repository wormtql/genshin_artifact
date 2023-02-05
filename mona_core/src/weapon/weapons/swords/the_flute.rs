use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct TheFlute;

impl WeaponTrait for TheFlute {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TheFlute,
        internal_name: "Sword_Troupe",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "普通攻击或重击命中时，会获得一个和音。积攒了5个和音后，释放音律的力量，对周围的敌人造成<span style=\"color: #409EFF;\">100%-125%-150%-175%-200%</span>攻击力的伤害。和音最多存在30秒，每0.5秒至多获得1个和音。",
            en: "Normal or Charged Attacks grant a Harmonic on hits. Gaining 5 Harmonics triggers the power of music and deals <span style=\"color: #409EFF;\">100%-125%-150%-175%-200%</span> ATK DMG to surrounding opponents. Harmonics last up to 30s, and a maximum of 1 can be gained every 0.5s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "笛剑",
            en: "The Flute"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
