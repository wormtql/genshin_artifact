use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct CranesEchoingCallEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for CranesEchoingCallEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let bonus = 0.13 * refine + 0.15;
        attribute.set_value_by(AttributeName::BonusPlungingAttack, "鹤鸣余音被动", bonus * self.rate);
    }
}

pub struct CranesEchoingCall;

impl WeaponTrait for CranesEchoingCall {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::CranesEchoingCall,
        internal_name: "Catalyst_MountainGale",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK36),
        weapon_base: WeaponBaseATKFamily::ATK741,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "装备者下落攻击命中敌人后，队伍中附近的所有角色下落攻击造成的伤害提高<span style=\"color: #409EFF;\">28%-41%-54%-67%-80%</span>，持续20秒；队伍中附近的角色的下落攻击命中敌人时，为装备者恢复<span style=\"color: #409EFF;\">2.5-2.75-3-3.25-3.5</span>点元素能量，每0.7秒至多通过这种方式恢复一次元素能量，装备者处于队伍后台时依然能通过这种方式恢复元素能量。",
            en: "After the equipping character hits an opponent with a Plunging Attack, all nearby party members' Plunging Attacks deal <span style=\"color: #409EFF;\">28%-41%-54%-67%-80%</span> increased DMG for 20s. When nearby party members hit opponents with Plunging Attacks, they will restore <span style=\"color: #409EFF;\">2.5-2.75-3-3.25-3.5</span> Energy to the equipping character. Energy can be restored this way every 0.7s. This energy regain effect can be triggered even if the equipping character is not on the field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "鹤鸣余音",
            en: "Crane's Echoing Call"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::CranesEchoingCall { rate } => {
                Some(Box::new(CranesEchoingCallEffect { rate }))
            },
            _ => None
        }
    }
}
