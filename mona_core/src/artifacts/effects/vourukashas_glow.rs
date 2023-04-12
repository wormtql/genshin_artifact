use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::{ArtifactEffectConfig, ConfigVourukashasGlow};
use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct VourukashasGlowEffect {
    pub stack: f64,
}

impl<A: Attribute> ArtifactEffect<A> for VourukashasGlowEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.add_hp_percentage("花海甘露之光2", 0.2);
    }

    fn effect4(&self, attribute: &mut A) {
        let value = 0.1 + self.stack * 0.08;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "花海甘露之光4", value);
        attribute.set_value_by(AttributeName::BonusElementalBurst, "花海甘露之光4", value);
    }
}

pub struct VourukashasGlow;

impl ArtifactTrait for VourukashasGlow {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(VourukashasGlowEffect {
            stack: config.config_vourukashas_glow.stack
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::VourukashasGlow,
        name_mona: "VourukashasGlow",
        name_locale: locale!(zh_cn: "花海甘露之光", en: "Vourukasha’s Glow"),
        flower: Some(locale!(zh_cn: "灵光源起之蕊", en: "Stamen of Khvarena's Origin")),
        feather: Some(locale!(zh_cn: "琦色灵彩之羽", en: "Vibrant Pinion")),
        sand: Some(locale!(zh_cn: "久远花落之时", en: "Ancient Abscission")),
        goblet: Some(locale!(zh_cn: "无边酣乐之筵", en: "Feast of Boundless Joy")),
        head: Some(locale!(zh_cn: "灵光明烁之心", en: "Heart of Khvarena's Brilliance")),
        star: (4, 5),
        effect1: None,
        effect2: Some(locale!(
            zh_cn: "生命值提升20%。",
            en: "HP +20%"
        )),
        effect3: None,
        effect4: Some(locale!(
            zh_cn: "元素战技与元素爆发造成的伤害提升10%；装备者受到伤害后的5秒内，上述伤害提升效果提高80%，该提高效果至多叠加5层，每层持续时间独立计算，处于队伍后台时依然能触发该效果。",
            en: "	Elemental Skill and Elemental Burst DMG will be increased by 10%. After the equipping character takes DMG, the aforementioned DMG Bonus is increased by 80% for 5s. This effect increase can have 5 stacks. The duration of each stack is counted independently. These effects can be triggered even when the equipping character is not on the field."
        )),
        effect5: None,
        internal_id: 15030
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: locale!(zh_cn: "平均层数", en: "AVG Stack"),
            config: ItemConfigType::Float { min: 0.0, max: 5.0, default: 4.0 }
        }
    ]);
}
