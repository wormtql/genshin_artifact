use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct SwordOfDescension;

impl WeaponTrait for SwordOfDescension {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SwordOfDescension,
        internal_name: "Sword_Psalmus",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK77),
        weapon_base: WeaponBaseATKFamily::ATK440,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("降世：仅在\"PlayStation Network\"游玩时生效。普通攻击与重击命中敌人后有50%概率在小范围内造成200%攻击力的伤害。该效果每10秒只能触发一次；此外，旅行者装备降临之剑时，攻击力提升66点。该武器技能效果初始即满级，无法精炼。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "降临之剑"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
