use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{CharacterConfig, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::common::ChangeAttribute;
use crate::common::item_config_type::ItemConfig;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::common::element::Element;
use crate::common::i18n::I18nLocale;

#[derive(Clone)]
pub struct CharacterSkillMapItem {
    pub index: usize,
    pub text: I18nLocale,
}

pub struct CharacterSkillMap {
    pub skill1: Option<&'static [CharacterSkillMapItem]>,
    pub skill2: Option<&'static [CharacterSkillMapItem]>,
    pub skill3: Option<&'static [CharacterSkillMapItem]>,
}

pub trait CharacterTrait {
    // 角色元数据
    const STATIC_DATA: CharacterStaticData;
    // 角色技能类型
    type SkillType;
    // 角色技能数值常量
    const SKILL: Self::SkillType;
    // 角色伤害Key
    type DamageEnumType: Copy + Clone + Into<usize>;
    // 角色定位枚举
    type RoleEnum;

    // 角色技能映射
    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap;

    // 角色参数
    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = None;
    // 角色技能参数
    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = None;

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result;

    fn damage<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: Self::DamageEnumType, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        Self::damage_internal::<D>(context, s.into(), config, fumo)
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>>;

    fn get_target_function_by_role(
        role_index: usize,
        team: &TeamQuantization,
        c: &CharacterCommonData,
        w: &WeaponCommonData
    ) -> Box<dyn TargetFunction>;
}
