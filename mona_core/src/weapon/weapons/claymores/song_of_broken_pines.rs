use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct SongOfBrokenPinesEffect {
    rate: f64,
}

impl SongOfBrokenPinesEffect {
    pub fn new(config: &WeaponConfig) -> SongOfBrokenPinesEffect {
        match *config {
            WeaponConfig::SongOfBrokenPines { rate } => SongOfBrokenPinesEffect {
                rate,
            },
            _ => SongOfBrokenPinesEffect {
                rate: 0.0,
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for SongOfBrokenPinesEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let value = refine * 0.04 + 0.12 + (refine * 0.05 + 0.15) * self.rate;
        attribute.add_atk_percentage("松籁响起之时被动等效", value);
    }
}

pub struct SongOfBrokenPines;

impl WeaponTrait for SongOfBrokenPines {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SongOfBrokenPines,
        internal_name: "Claymore_Widsith",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::PhysicalBonus45),
        weapon_base: WeaponBaseATKFamily::ATK741,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "飘游风中的「千年的大乐章」的一部分。攻击力提高<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>；普通攻击或重击命中敌人时，角色获得一枚低语之符，每0.3秒内至多触发一次。拥有4枚低语之符时，将消耗所有低语之符，使附近的队伍中所有角色获得持续12秒的「千年的大乐章·揭旗之歌」效果：普通攻击速度提高<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>，攻击力提升<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>。触发后20秒内，无法再次获得低语之符。「千年的大乐章」触发的多种数值效果中，同类数值效果不可叠加。",
            en: "A part of the \"Millennial Movement\" that wanders amidst the winds. Increases ATK by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>, and when Normal or Charged Attacks hit opponents, the character gains a Sigil of Whispers. This effect can be triggered once every 0.3s. When you possess 4 Sigils of Whispers, all of them will be consumed and all nearby party members will obtain the \"Millennial Movement: Banner-Hymn\" effect for 12s. \"Millennial Movement: Banner-Hymn\" increases Normal ATK SPD by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> and increases ATK by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>. Once this effect is triggered, you will not gain Sigils of Whispers for 20s. Of the many effects of the \"Millennial Movement,\" buffs of the same type will not stack."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "松籁响起之时",
            en: "Song of Broken Pines"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: ItemConfig::DEFAULT_RATE_TITLE,
            config: ItemConfigType::Float {
                min: 0.0,
                max: 1.0,
                default: 0.0
            }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(SongOfBrokenPinesEffect::new(config)))
    }
}
