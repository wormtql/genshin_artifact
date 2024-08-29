use smallvec::{SmallVec, smallvec};
use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::Element;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ConfigElements8Multi, ItemConfig, ItemConfigType};

pub struct ScrollOfTheHeroOfCinderCityEffect {
    pub elements: ConfigElements8Multi,
    pub rate1: f64,
    pub rate2: f64,
}

impl<A: Attribute> ArtifactEffect<A> for ScrollOfTheHeroOfCinderCityEffect {
    fn effect4(&self, attribute: &mut A) {
        for &element in self.elements.collect_elements().iter() {
            let bonus_attribute_name = AttributeName::bonus_name_by_element(element);
            attribute.set_value_by(bonus_attribute_name, "烬城勇者绘卷4", self.rate1 * 0.12 + self.rate2 * 0.28);
        }
    }
}

pub struct ScrollOfTheHeroOfCinderCity;

impl ArtifactTrait for ScrollOfTheHeroOfCinderCity {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ScrollOfTheHeroOfCinderCityEffect {
            elements: config.config_scroll_of_the_hero_of_cinder_city.elements.clone(),
            rate1: config.config_scroll_of_the_hero_of_cinder_city.rate1,
            rate2: config.config_scroll_of_the_hero_of_cinder_city.rate2
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::ScrollOfTheHeroOfCinderCity,
        name_mona: "ScrollOfTheHeroOfCinderCity",
        name_locale: locale!(
            zh_cn: "烬城勇者绘卷",
            en: "Scroll of the Hero of Cinder City"
        ),
        flower: Some(locale!(
            zh_cn: "驯兽师的护符",
            en: "Beast Tamer's Talisman"
        )),
        feather: Some(locale!(
            zh_cn: "巡山客的信标",
            en: "Mountain Ranger's Marker"
        )),
        sand: Some(locale!(
            zh_cn: "秘术家的金盘",
            en: "Mystic's Gold Dial"
        )),
        goblet: Some(locale!(
            zh_cn: "游学者的爪杯",
            en: "Wandering Scholar's Claw Cup"
        )),
        head: Some(locale!(
            zh_cn: "魔战士的羽面",
            en: "Demon-Warrior's Feather Mask"
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(locale!(
            zh_cn: "队伍中附近的角色触发「夜魂迸发」时，装备者恢复6点元素能量。",
            en: "When a nearby party member triggers a Nightsoul Burst, the equipping character regenerates 6 Elemental Energy."
        )),
        effect3: None,
        effect4: Some(locale!(
            zh_cn: "装备者触发其对应元素类型的相关反应后，队伍中附近的所有角色的该元素反应相关的元素伤害加成提升12%，持续15秒。若触发该效果时，装备者处于夜魂加持状态下，还将使队伍中附近的所有角色的与该元素反应相关的元素伤害加成提升28%，持续20秒。装备者处于后台时也能触发上述效果。同名圣遗物套装产生的伤害加成效果无法叠加。",
            en: "After the equipping character triggers a reaction related to their Elemental Type, all nearby party members gain a 12% Elemental DMG Bonus for the Elemental Types involved in the elemental reaction for 15s. If the equipping character is in the Nightsoul's Blessing state when triggering this effect, all nearby party members gain an additional 28% Elemental DMG Bonus for the Elemental Types involved in the elemental reaction for 20s. The equipping character can trigger this effect while off-field, and the DMG bonus from Artifact Sets with the same name do not stack."
        )),
        effect5: None,
        internal_id: 15037,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "elements",
            title: locale!(
                zh_cn: "元素",
                en: "Elements"
            ),
            config: ItemConfigType::Element8Multi {
                default: ConfigElements8Multi {
                    pyro: false,
                    electro: false,
                    dendro: false,
                    cryo: false,
                    anemo: false,
                    geo: false,
                    hydro: false,
                    physical: false,
                }
            }
        },
        ItemConfig {
            name: "rate1",
            title: locale!(
                zh_cn: "效果1比例",
                en: "Ratio 1"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "rate2",
            title: locale!(
                zh_cn: "效果2比例",
                en: "Ratio 2"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);
}