use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct FragmentOfHarmonicWhimsyEffect {
    pub stack: f64,
}

impl<A: Attribute> ArtifactEffect<A> for FragmentOfHarmonicWhimsyEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.add_atk_percentage("谐律异想断章2", 0.18);
    }

    fn effect4(&self, attribute: &mut A) {
        let bonus = self.stack * 0.18;
        attribute.set_value_by(AttributeName::BonusBase, "谐律异想断章4", bonus);
    }
}

pub struct FragmentOfHarmonicWhimsy;

impl ArtifactTrait for FragmentOfHarmonicWhimsy {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        let stack = config.config_fragment_of_harmonic_whimsy.level;
        Box::new(FragmentOfHarmonicWhimsyEffect {
            stack
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::FragmentOfHarmonicWhimsy,
        name_mona: "FragmentOfHarmonicWhimsy",
        name_locale: locale!(
            zh_cn: "谐律异想断章",
            en: "Fragment of Harmonic Whimsy"
        ),
        flower: Some(locale!(
            zh_cn: "谐律交响的前奏",
            en: "Harmonious Symphony Prelude"
        )),
        feather: Some(locale!(zh_cn: "古海玄幽的夜想", en: "Ancient Sea's Nocturnal Musing")),
        sand: Some(locale!(zh_cn: "命途轮转的谐谑", en: "The Grand Jape of the Turning of Fate")),
        goblet: Some(locale!(zh_cn: "灵露倾洒的狂诗", en: "Ichor Shower Rhapsody")),
        head: Some(locale!(zh_cn: "异想零落的圆舞", en: "Whimsical Dance of the Withered")),
        star: (4, 5),
        effect1: None,
        effect2: Some(locale!(
            zh_cn: "攻击力提高18%。",
            en: "ATK +18%"
        )),
        effect3: None,
        effect4: Some(locale!(
            zh_cn: "生命之契的数值提升或降低时，角色造成的伤害提升18%，该效果持续6秒，至多叠加3次。",
            en: "When the value of a Bond of Life increases or decreases, this character deals 18% increased DMG for 6s. Max 3 stacks."
        )),
        effect5: None,
        internal_id: 15035, // todo
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "level",
            title: locale!(
                zh_cn: "被动层数",
                en: "Stack"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 }
        }
    ]);
}
