import _tags from './secondary_tags.json';

export const secondaryTags = _tags;

export const attributesSet = new Set([
  '攻击力',
  '生命值',
  '防御力',
  '暴击率',
  '暴击伤害',
  '元素精通',
  '元素充能效率',
  '雷元素伤害加成',
  '火元素伤害加成',
  '水元素伤害加成',
  '冰元素伤害加成',
  '风元素伤害加成',
  '岩元素伤害加成',
  '物理伤害加成',
  '治疗加成',
]);

export function getAttributeType(name, value) {
  switch (name) {
    case '攻击力': {
      return value.includes('%') ? 'attackPercentage' : 'attackStatic';
    }
    case '生命值': {
      return value.includes('%') ? 'lifePercentage' : 'lifeStatic';
    }
    case '防御力': {
      return value.includes('%') ? 'defendPercentage' : 'defendStatic';
    }
    case '暴击率': {
      return 'critical';
    }
    case '暴击伤害': {
      return 'criticalDamage';
    }
    case '元素精通': {
      return 'elementalMastery';
    }
    case '元素充能效率': {
      return 'recharge';
    }
    case '雷元素伤害加成': {
      return 'thunderBonus';
    }
    case '火元素伤害加成': {
      return 'fireBonus';
    }
    case '水元素伤害加成': {
      return 'waterBonus';
    }
    case '冰元素伤害加成': {
      return 'iceBonus';
    }
    case '风元素伤害加成': {
      return 'windBonus';
    }
    case '岩元素伤害加成': {
      return 'rockBonus';
    }
    case '物理伤害加成': {
      return 'physicalBonus';
    }
    case '治疗加成': {
      return 'cureEffect';
    }
  }
}
