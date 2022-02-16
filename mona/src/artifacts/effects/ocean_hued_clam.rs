use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct OceanHuedClamEffect;

impl<T: Attribute> ArtifactEffect<T> for OceanHuedClamEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::HealingBonus, "海染砗磲2", 0.15);
    }

    // todo
    // fn effect4(&self, attribute: &mut AttributeGraph) {
    //     attribute.add_value(AttributeName::CriticalBase, "战狂4", self.rate * 0.24);
    // }
}

pub struct OceanHuedClam;

impl ArtifactTrait for OceanHuedClam {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(OceanHuedClamEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::OceanHuedClam,
        name_mona: "oceanHuedClam",
        chs: "海染砗磲",
        flower: Some("海染之花"),
        feather: Some("渊宫之羽"),
        sand: Some("离别之贝"),
        goblet: Some("真珠之笼"),
        head: Some("海祇之冠"),
        star: (4, 5),
        effect1: None,
        effect2: Some("治疗加成提高15%。"),
        effect3: None,
        effect4: Some("装备此圣遗物套装的角色对队伍中的角色进行治疗时，将产生持续3秒的海染泡沫，记录治疗的生命值回复量（包括溢出值）。持续时间结束时，海染泡沫将会爆炸，对周围的敌人造成90%累计回复量的伤害（该伤害结算方式同感电、超导等元素反应，但不受元素精通、等级或反应伤害加成效果影响）。每3.5秒至多产生一个海染泡沫；海染泡沫至多记录30000点回复量，含溢出部分的治疗量；自己的队伍中同时至多存在一个海染泡沫。装备此圣遗物套装的角色处于队伍后台时，依然能触发该效果。"),
        effect5: None
    };
}
