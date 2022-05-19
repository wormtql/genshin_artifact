// generated file, do not edit


import ATKPercentage_image from "@image/misc/sword"



import DEFPercentage_image from "@image/misc/sword"



import HPPercentage_image from "@image/misc/sword"



import ATKFixed_image from "@image/misc/sword"



import DEFFixed_image from "@image/misc/sword"



import HPFixed_image from "@image/misc/sword"



import Critical_image from "@image/misc/sword"



import CriticalDamage_image from "@image/misc/sword"



import CustomBonus_image from "@image/misc/sword"



import ElementalMastery_image from "@image/misc/sword"



import Recharge_image from "@image/misc/sword"



import DEFMinus_image from "@image/misc/sword"



import ResMinus_image from "@image/misc/sword"



import HealingBonus_image from "@image/misc/sword"













































































































import FreedomSworn_image from "@image/weapons/FreedomSworn_tn"



import SongOfBrokenPines_image from "@image/weapons/SongOfBrokenPines_tn"



import WolfsGravestone_image from "@image/weapons/WolfsGravestone_tn"



import ThrillingTalesOfDragonSlayers_image from "@image/weapons/ThrillingTalesOfDragonSlayers_tn"



import ElegyOfTheEnd_image from "@image/weapons/ElegyOfTheEnd_tn"



import HakushinRing_image from "@image/weapons/HakushinRing_tn"



import ResonancePyro2_image from "@image/misc/pyro"



import ResonanceCryo2_image from "@image/misc/cryo"



import ResonanceGeo2_image from "@image/misc/geo"



import Instructor4_image from "@image/artifacts/Instructor_flower"



import NoblesseOblige4_image from "@image/artifacts/NoblesseOblige_flower"



import ArchaicPetra4_image from "@image/artifacts/ArchaicPetra_flower"



import ViridescentVenerer4_image from "@image/artifacts/ViridescentVenerer_flower"



import TenacityOfTheMillelith4_image from "@image/artifacts/TenacityOfTheMillelith_flower"



const template = "https://upload-bbs.mihoyo.com/game_record/genshin/character_icon/UI_AvatarIcon_#.png"
const getImage = name => template.replace("#", name)

export default {
    
    "ATKPercentage": {
        name: "ATKPercentage",
        chs: "攻击力%",
        
        badge: ATKPercentage_image,
        
        genre: "Common",
        description: "",
        config: [
            
            {"default":0.0,"name":"p","title":"数值","type":"floatPercentageInput"},
            
        ],
    },
    
    "DEFPercentage": {
        name: "DEFPercentage",
        chs: "防御力%",
        
        badge: DEFPercentage_image,
        
        genre: "Common",
        description: "",
        config: [
            
            {"default":0.0,"name":"p","title":"数值","type":"floatPercentageInput"},
            
        ],
    },
    
    "HPPercentage": {
        name: "HPPercentage",
        chs: "生命值%",
        
        badge: HPPercentage_image,
        
        genre: "Common",
        description: "",
        config: [
            
            {"default":0.0,"name":"p","title":"数值","type":"floatPercentageInput"},
            
        ],
    },
    
    "ATKFixed": {
        name: "ATKFixed",
        chs: "攻击力",
        
        badge: ATKFixed_image,
        
        genre: "Common",
        description: "",
        config: [
            
            {"default":0.0,"name":"value","title":"数值","type":"floatInput"},
            
        ],
    },
    
    "DEFFixed": {
        name: "DEFFixed",
        chs: "防御力",
        
        badge: DEFFixed_image,
        
        genre: "Common",
        description: "",
        config: [
            
            {"default":0.0,"name":"value","title":"数值","type":"floatInput"},
            
        ],
    },
    
    "HPFixed": {
        name: "HPFixed",
        chs: "生命值",
        
        badge: HPFixed_image,
        
        genre: "Common",
        description: "",
        config: [
            
            {"default":0.0,"name":"value","title":"数值","type":"floatInput"},
            
        ],
    },
    
    "Critical": {
        name: "Critical",
        chs: "暴击率",
        
        badge: Critical_image,
        
        genre: "Common",
        description: "",
        config: [
            
            {"default":0.0,"name":"p","title":"数值","type":"floatPercentageInput"},
            
        ],
    },
    
    "CriticalDamage": {
        name: "CriticalDamage",
        chs: "暴击伤害",
        
        badge: CriticalDamage_image,
        
        genre: "Common",
        description: "",
        config: [
            
            {"default":0.0,"name":"p","title":"数值","type":"floatPercentageInput"},
            
        ],
    },
    
    "CustomBonus": {
        name: "CustomBonus",
        chs: "伤害加成",
        
        badge: CustomBonus_image,
        
        genre: "Common",
        description: "",
        config: [
            
            {"default":0.0,"name":"p","title":"数值","type":"floatPercentageInput"},
            
        ],
    },
    
    "ElementalMastery": {
        name: "ElementalMastery",
        chs: "元素精通",
        
        badge: ElementalMastery_image,
        
        genre: "Common",
        description: "",
        config: [
            
            {"default":0.0,"name":"value","title":"数值","type":"floatInput"},
            
        ],
    },
    
    "Recharge": {
        name: "Recharge",
        chs: "元素充能效率",
        
        badge: Recharge_image,
        
        genre: "Common",
        description: "",
        config: [
            
            {"default":20.0,"name":"p","title":"值","type":"floatPercentageInput"},
            
        ],
    },
    
    "DEFMinus": {
        name: "DEFMinus",
        chs: "减防",
        
        badge: DEFMinus_image,
        
        genre: "Common",
        description: "百分比减防",
        config: [
            
            {"default":0.0,"name":"p","title":"数值","type":"floatPercentageInput"},
            
        ],
    },
    
    "ResMinus": {
        name: "ResMinus",
        chs: "减抗",
        
        badge: ResMinus_image,
        
        genre: "Common",
        description: "",
        config: [
            
            {"default":0.0,"name":"p","title":"数值","type":"floatPercentageInput"},
            
        ],
    },
    
    "HealingBonus": {
        name: "HealingBonus",
        chs: "治疗加成",
        
        badge: HealingBonus_image,
        
        genre: "Common",
        description: "",
        config: [
            
            {"default":0.0,"name":"p","title":"数值","type":"floatPercentageInput"},
            
        ],
    },
    
    "AlbedoTalent2": {
        name: "AlbedoTalent2",
        chs: "阿贝多-「瓶中人的天慧」",
        
        badge: getImage("Albedo"),
        
        genre: "Character",
        description: "阿贝多天赋2：释放诞生式·大地之潮时,使附近的队伍中角色的元素精通提高125点，持续10秒",
        config: [
            
        ],
    },
    
    "AloyTalent1": {
        name: "AloyTalent1",
        chs: "埃洛伊-「战斗覆盖」",
        
        badge: getImage("Aloy"),
        
        genre: "Character",
        description: "埃洛伊天赋1：埃洛伊获得冰尘雪野的线圈效果时，队伍中附近的其他角色的攻击力提升8%，持续10秒。",
        config: [
            
        ],
    },
    
    "AratakiIttoC4": {
        name: "AratakiIttoC4",
        chs: "荒泷一斗-「奉行牢狱，茶饭之所」",
        
        badge: getImage("Itto"),
        
        genre: "Character",
        description: "荒泷一斗命座4：最恶鬼王•一斗轰临！！施加的「怒目鬼王」状态结束后，附近的队伍中所有角色的防御力提升20%，攻击力提升20%，持续10秒。",
        config: [
            
        ],
    },
    
    "BeidouC6": {
        name: "BeidouC6",
        chs: "北斗-「北斗祓幽孽」",
        
        badge: getImage("Beidou"),
        
        genre: "Character",
        description: "北斗命座6：斫雷持续期间，周围敌人的雷元素抗性降低15%。",
        config: [
            
        ],
    },
    
    "BennettQ": {
        name: "BennettQ",
        chs: "班尼特-「美妙旅程」",
        
        badge: getImage("Bennett"),
        
        genre: "Character",
        description: "班尼特Q技能：基于班尼特的基础攻击力，以一定比例获得攻击力加成<br>一命：美妙旅程的攻击力提升效果不再有血量限制，数值上追加班尼特基础攻击力的20%。",
        config: [
            
            {"default":800.0,"name":"base_atk","title":"班尼特的基础攻击力","type":"floatInput"},
            
            {"default":true,"name":"c1","title":"是否1命","type":"bool"},
            
            {"default":10,"max":15,"min":1,"name":"skill3","title":"技能等级","type":"int"},
            
        ],
    },
    
    "BennettC6": {
        name: "BennettC6",
        chs: "班尼特-「烈火与勇气」",
        
        badge: getImage("Bennett"),
        
        genre: "Character",
        description: "班尼特命座6：处在美妙旅程领域内的队伍中当前场上单手剑、双手剑、长柄武器角色获得15%火元素伤害加成<br>注：此处不管当前角色的武器类型",
        config: [
            
        ],
    },
    
    "ChongyunTalent2": {
        name: "ChongyunTalent2",
        chs: "重云-「追冰剑诀」",
        
        badge: getImage("Chongyun"),
        
        genre: "Character",
        description: "重云天赋2：灵刃·重华叠霜领域消失时，会唤出一柄灵刃自动攻击附近的敌人，造成相当于灵刃·重华叠霜技能伤害100%的冰元素范围伤害。被击中的敌人冰元素抗性降低10%，持续8秒。",
        config: [
            
        ],
    },
    
    "DionaC6G50": {
        name: "DionaC6G50",
        chs: "迪奥娜-「猫尾打烊之时」",
        
        badge: getImage("Diona"),
        
        genre: "Character",
        description: "迪奥娜命座6：生命值高于50%时，元素精通提升200。",
        config: [
            
        ],
    },
    
    "GanyuTalent2": {
        name: "GanyuTalent2",
        chs: "甘雨-「天地交泰」",
        
        badge: getImage("Ganyu"),
        
        genre: "Character",
        description: "甘雨天赋2：降众天华领域内的队伍中当前场上角色获得20%冰元素伤害加成。",
        config: [
            
        ],
    },
    
    "GanyuC1": {
        name: "GanyuC1",
        chs: "甘雨-「饮露」",
        
        badge: getImage("Ganyu"),
        
        genre: "Character",
        description: "甘雨命座1：二段蓄力重击的霜华矢或霜华绽发命中敌人时，会使敌人的冰元素抗性降低15%，持续6秒。",
        config: [
            
        ],
    },
    
    "GorouE1": {
        name: "GorouE1",
        chs: "五郎-「大将旗指物」-1",
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        description: "五郎E技能：一名角色时：「坚牢」：防御力提升。",
        config: [
            
            {"default":10,"max":15,"min":1,"name":"skill2","title":"五郎E技能等级","type":"int"},
            
        ],
    },
    
    "GorouE3": {
        name: "GorouE3",
        chs: "五郎-「大将旗指物」-3",
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        description: "五郎E技能：三名角色时：「摧碎」：除上述效果外，获得岩元素伤害加成。",
        config: [
            
        ],
    },
    
    "GorouTalent1": {
        name: "GorouTalent1",
        chs: "五郎-「不畏风雨」",
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        description: "五郎天赋1：施放兽牙逐突形胜战法后的12秒内，附近的队伍中所有角色的防御力提升25%。",
        config: [
            
        ],
    },
    
    "GorouC6": {
        name: "GorouC6",
        chs: "五郎-「犬勇•忠如山」",
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        description: "五郎命座6：施放犬坂吠吠方圆阵或兽牙逐突形胜战法后的12秒内，依据施放时的领域等级，提高附近的队伍中所有角色岩元素伤害的暴击伤害。",
        config: [
            
            {"default":1,"max":3,"min":1,"name":"level","title":"领域等级","type":"int"},
            
        ],
    },
    
    "HuTaoTalent1": {
        name: "HuTaoTalent1",
        chs: "胡桃-「蝶隐之时」",
        
        badge: getImage("Hutao"),
        
        genre: "Character",
        description: "胡桃天赋1：蝶引来生施加的彼岸蝶舞状态结束后，队伍中所有角色（不包括胡桃自己）的暴击率提高12%，持续8秒。",
        config: [
            
        ],
    },
    
    "JeanC4": {
        name: "JeanC4",
        chs: "琴-「蒲公英的国土」",
        
        badge: getImage("Qin"),
        
        genre: "Character",
        description: "琴命座4：在蒲公英之风的领域内，所有敌人的风元素抗性下降40％。",
        config: [
            
        ],
    },
    
    "KaedeharaKazuhaTalent2": {
        name: "KaedeharaKazuhaTalent2",
        chs: "枫原万叶-「风物之诗咏」",
        
        badge: getImage("Kazuha"),
        
        genre: "Character",
        description: "枫原万叶天赋2：枫原万叶触发扩散反应后，枫原万叶的每点元素精通，会为队伍中所有角色提供0.04%对应元素伤害加成，持续8秒。",
        config: [
            
            {"default":"Electro","name":"element","title":"扩散元素","type":"element4"},
            
            {"default":800.0,"name":"em","title":"万叶的元素精通","type":"floatInput"},
            
        ],
    },
    
    "KaedeharaKazuhaC2": {
        name: "KaedeharaKazuhaC2",
        chs: "枫原万叶-「山岚残芯」",
        
        badge: getImage("Kazuha"),
        
        genre: "Character",
        description: "枫原万叶命座2：场上角色的元素精通提升200点。",
        config: [
            
        ],
    },
    
    "KamisatoAyakaC4": {
        name: "KamisatoAyakaC4",
        chs: "神里绫华-「盈缺流返」",
        
        badge: getImage("Ayaka"),
        
        genre: "Character",
        description: "绫华命座4：敌人受到神里流•霜灭的霜见雪关扉造成的伤害后，防御力降低30%，持续6秒。",
        config: [
            
        ],
    },
    
    "KleeC2": {
        name: "KleeC2",
        chs: "可莉-「破破弹片」",
        
        badge: getImage("Klee"),
        
        genre: "Character",
        description: "可莉命座2：蹦蹦炸弹的诡雷会使敌人防御力降低23％，持续10秒。",
        config: [
            
        ],
    },
    
    "KleeC6": {
        name: "KleeC6",
        chs: "可莉-「火力全开」",
        
        badge: getImage("Klee"),
        
        genre: "Character",
        description: "可莉命座6：施放轰轰火花后的25秒内，队伍中所有角色获得10％火元素伤害加成。",
        config: [
            
        ],
    },
    
    "KujouSaraEOrQ": {
        name: "KujouSaraEOrQ",
        chs: "九条裟罗-「天狗咒雷」",
        
        badge: getImage("Sara"),
        
        genre: "Character",
        description: "九条裟罗E/Q技能：基于九条裟罗的基础攻击力，以一定比例获得攻击力加成<br>六命：处于天狗咒雷带来的攻击力提升效果状态下的角色，其雷元素伤害的暴击伤害提高60%。",
        config: [
            
            {"default":700.0,"name":"base_atk","title":"九条裟罗的基础攻击力","type":"floatInput"},
            
            {"default":false,"name":"c6","title":"是否六命","type":"bool"},
            
            {"default":10,"max":15,"min":1,"name":"skill2","title":"E技能等级","type":"int"},
            
        ],
    },
    
    "LisaTalent2": {
        name: "LisaTalent2",
        chs: "丽莎-「静电场力」",
        
        badge: getImage("Lisa"),
        
        genre: "Character",
        description: "丽莎天赋2：敌人受到蔷薇的雷光攻击后，降低15%防御力，持续10秒。",
        config: [
            
        ],
    },
    
    "MonaQ": {
        name: "MonaQ",
        chs: "莫娜-「星异」",
        
        badge: getImage("Mona"),
        
        genre: "Character",
        description: "莫娜Q技能：对敌人施加星异的伤害加成效果，并以此提高这一次造成的伤害。四命：队伍中所有角色攻击处于星异状态下的敌人时，暴击率提升15%",
        config: [
            
            {"default":9,"max":15,"min":1,"name":"skill3","title":"Q技能等级","type":"int"},
            
            {"default":false,"name":"c4","title":"是否4命","type":"bool"},
            
        ],
    },
    
    "MonaC1": {
        name: "MonaC1",
        chs: "莫娜-「沉没的预言」",
        
        badge: getImage("Mona"),
        
        genre: "Character",
        description: "莫娜命座1：队伍中自己的角色攻击命中处于星异状态下的敌人后的8秒内，水元素相关反应的效果提升：<br>•感电反应造成的伤害提升15%，蒸发反应造成的伤害提升15%，水元素扩散反应造成的伤害提升15%",
        config: [
            
        ],
    },
    
    "NingguangTalent2": {
        name: "NingguangTalent2",
        chs: "凝光-「储之千日，用之一刻」",
        
        badge: getImage("Ningguang"),
        
        genre: "Character",
        description: "凝光天赋2：穿过璇玑屏的角色会获得12%岩元素伤害加成，持续10秒。",
        config: [
            
        ],
    },
    
    "RaidenShogunE": {
        name: "RaidenShogunE",
        chs: "雷电将军-「雷罚恶曜之眼」",
        
        badge: getImage("Shougun"),
        
        genre: "Character",
        description: "雷电将军E技能：雷罚恶曜之眼的角色在持续期间内，元素爆发造成的伤害获得提升，提升程度基于元素爆发的元素能量。",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"skill2","title":"雷电将军E技能等级","type":"int"},
            
            {"default":80,"max":100,"min":20,"name":"energy","title":"受BUFF角色最大元素能量","type":"int"},
            
        ],
    },
    
    "RaidenShogunC4": {
        name: "RaidenShogunC4",
        chs: "雷电将军-「誓奉常道」",
        
        badge: getImage("Shougun"),
        
        genre: "Character",
        description: "雷电将军命座4：奥义•梦想真说施加的梦想一心状态结束后，附近的队伍中所有角色（不包括雷电将军自己）的攻击力提升30%，持续10秒。",
        config: [
            
        ],
    },
    
    "RazorC4": {
        name: "RazorC4",
        chs: "雷泽-「撕咬」",
        
        badge: getImage("Razor"),
        
        genre: "Character",
        description: "雷泽命座4：利爪与苍雷点按时，会使命中的敌人防御力降低15％，持续7秒。",
        config: [
            
        ],
    },
    
    "RosariaTalent2": {
        name: "RosariaTalent2",
        chs: "罗莎莉亚-「暗中支援的黯色」",
        
        badge: getImage("Rosaria"),
        
        genre: "Character",
        description: "罗莎莉亚天赋2：施放终命的圣礼时，基于自身暴击率的15%，提高附近的队伍中所有角色(不包括罗莎莉亚自己)的暴击率，持续10秒。通过这种方式获得的暴击率提升，无法超过15%。",
        config: [
            
            {"default":70.0,"name":"crit","title":"罗莎莉亚的暴击率","type":"floatPercentageInput"},
            
        ],
    },
    
    "RosariaC6": {
        name: "RosariaC6",
        chs: "罗莎莉亚-「代行裁判」",
        
        badge: getImage("Rosaria"),
        
        genre: "Character",
        description: "罗莎莉亚命座6：终命的圣礼的攻击会使敌人的物理抗性降低20%，持续10秒。",
        config: [
            
        ],
    },
    
    "ShenheE": {
        name: "ShenheE",
        chs: "申鹤-「冰翎」",
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        description: "申鹤E技能：基于申鹤自己当前的攻击力，提高造成的伤害。",
        config: [
            
            {"default":3000.0,"name":"atk","title":"申鹤的攻击力","type":"floatInput"},
            
            {"default":8,"max":15,"min":1,"name":"skill2","title":"申鹤E技能等级","type":"int"},
            
        ],
    },
    
    "ShenheQ": {
        name: "ShenheQ",
        chs: "申鹤-「神女遣灵真诀」减抗",
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        description: "申鹤Q技能：「箓灵」将结成领域，使其中敌人的冰元素抗性与物理抗性降低。",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"skill3","title":"申鹤Q技能等级","type":"int"},
            
        ],
    },
    
    "ShenheTalent1": {
        name: "ShenheTalent1",
        chs: "申鹤-「大洞弥罗尊法」",
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        description: "申鹤天赋1：处于神女遣灵真诀的领域中的当前场上角色，冰元素伤害加成提高15%。二命：领域中的当前场上角色，冰元素伤害的暴击伤害提高15%。",
        config: [
            
            {"default":false,"name":"c2","title":"是否二命","type":"bool"},
            
        ],
    },
    
    "ShenheTalent2": {
        name: "ShenheTalent2",
        chs: "申鹤-「缚灵通真法印」",
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        description: "申鹤天赋2：申鹤施放仰灵威召将役咒后，将使附近的队伍中所有角色获得如下效果：<br>•点按：元素战技和元素爆发造成的伤害提高15%，持续10秒；<br>•长按：普通攻击、重击和下落攻击造成的伤害提高15%，持续15秒。",
        config: [
            
            {"default":0,"name":"t","options":["点按","长按"],"title":"技能释放方式","type":"option"},
            
        ],
    },
    
    "SucroseTalent1": {
        name: "SucroseTalent1",
        chs: "砂糖-「触媒置换术」",
        
        badge: getImage("Sucrose"),
        
        genre: "Character",
        description: "砂糖天赋1：砂糖触发扩散反应时，使队伍中所有对应元素类型的角色（不包括砂糖自己）元素精通提升50，持续8秒。",
        config: [
            
        ],
    },
    
    "SucroseTalent2": {
        name: "SucroseTalent2",
        chs: "砂糖-「小小的慧风」",
        
        badge: getImage("Sucrose"),
        
        genre: "Character",
        description: "砂糖天赋2：风灵作成·陆叁零捌或禁·风灵作成·染伍同构贰型命中敌人时，基于砂糖元素精通的20%,为队伍中所有角色（不包括砂糖自己）提供元素精通加成，持续8秒。",
        config: [
            
            {"default":200.0,"name":"em","title":"砂糖的元素精通","type":"floatInput"},
            
        ],
    },
    
    "SucroseC6": {
        name: "SucroseC6",
        chs: "砂糖-「混元熵增论」",
        
        badge: getImage("Sucrose"),
        
        genre: "Character",
        description: "砂糖命座6：禁·风灵作成·柒伍同构贰型如果发生了元素转化，则使队伍中所有角色在技能持续时间内获得20%的对应元素伤害加成。",
        config: [
            
            {"default":"Electro","name":"element","title":"扩散类型","type":"element4"},
            
        ],
    },
    
    "ThomaTalent1": {
        name: "ThomaTalent1",
        chs: "托马-「甲衣交叠」",
        
        badge: getImage("Tohma"),
        
        genre: "Character",
        description: "托马天赋1：当前场上自己的角色获取或刷新烈烧佑命护盾时，护盾强效提升5%，持续时间6秒。此效果每0.3秒至多触发一次，至多叠加5次。",
        config: [
            
            {"default":2.0,"max":5.0,"min":0.0,"name":"stack","title":"叠加层数","type":"float"},
            
        ],
    },
    
    "ThomaC6": {
        name: "ThomaC6",
        chs: "托马-「炽烧的至心」",
        
        badge: getImage("Tohma"),
        
        genre: "Character",
        description: "托马命座6：获取或刷新烈烧佑命护盾时，队伍中所有角色的普通攻击，重击与下落攻击造成的伤害提升15%，持续6秒。",
        config: [
            
        ],
    },
    
    "VentiC2": {
        name: "VentiC2",
        chs: "温迪-「眷恋的泠风」",
        
        badge: getImage("Venti"),
        
        genre: "Character",
        description: "温迪命座2：高天之歌会使敌人的风元素抗性与物理抗性降低12％，持续10秒。被高天之歌击飞的敌人在落地前，风元素抗性与物理抗性额外降低12％。",
        config: [
            
            {"default":false,"name":"levitating","title":"落地前","type":"bool"},
            
        ],
    },
    
    "VentiC6": {
        name: "VentiC6",
        chs: "温迪-「抗争的暴风」",
        
        badge: getImage("Venti"),
        
        genre: "Character",
        description: "温迪命座6：温迪六命BUFF。受风神之诗伤害的敌人，风元素抗性降低20％。若产生了元素转化，则使转换的元素抗性也降低20％。",
        config: [
            
            {"default":true,"name":"is_convert","title":"发生了元素转化","type":"bool"},
            
            {"default":"Electro","name":"element","title":"转化类型","type":"element4"},
            
        ],
    },
    
    "XianglingTalent2": {
        name: "XianglingTalent2",
        chs: "香菱-「绝云朝天椒」",
        
        badge: getImage("Xiangling"),
        
        genre: "Character",
        description: "香菱天赋2：锅巴出击效果结束时，锅巴会在消失的位置留下辣椒。拾取辣椒会提高10%攻击力，持续10秒。",
        config: [
            
        ],
    },
    
    "XianglingC1": {
        name: "XianglingC1",
        chs: "香菱-「外酥里嫩」",
        
        badge: getImage("Xiangling"),
        
        genre: "Character",
        description: "香菱命座1：受到锅巴攻击的敌人，火元素抗性降低15％，持续6秒。",
        config: [
            
        ],
    },
    
    "XianglingC6": {
        name: "XianglingC6",
        chs: "香菱-「大龙卷旋火轮」",
        
        badge: getImage("Xiangling"),
        
        genre: "Character",
        description: "香菱命座6；旋火轮持续期间，队伍中所有角色获得15％火元素伤害加成。",
        config: [
            
        ],
    },
    
    "XingqiuC2": {
        name: "XingqiuC2",
        chs: "行秋-「天青现虹」",
        
        badge: getImage("Xingqiu"),
        
        genre: "Character",
        description: "行秋命座2：受到剑雨攻击的敌人，水元素抗性降低15％，持续4秒。",
        config: [
            
        ],
    },
    
    "XinyanC4": {
        name: "XinyanC4",
        chs: "辛焱-「节奏的传染」",
        
        badge: getImage("Xinyan"),
        
        genre: "Character",
        description: "辛焱命座4：热情拂扫的伤害，会使敌人的物理抗性降低15%，持续12秒。",
        config: [
            
        ],
    },
    
    "XinyanTalent2": {
        name: "XinyanTalent2",
        chs: "辛焱-「这才是摇滚!」",
        
        badge: getImage("Xinyan"),
        
        genre: "Character",
        description: "辛焱天赋2：处于热情拂扫的护盾保护下的角色造成的物理伤害提高15%。",
        config: [
            
        ],
    },
    
    "YaeMikoC4": {
        name: "YaeMikoC4",
        chs: "八重神子-「绯樱引雷章」",
        
        badge: getImage("Yae"),
        
        genre: "Character",
        description: "八重神子命座4：杀生樱的落雷命中敌人后，队伍中附近的所有角色获得20%雷元素伤害加成，持续5秒。",
        config: [
            
        ],
    },
    
    "YoimiyaTalent2": {
        name: "YoimiyaTalent2",
        chs: "宵宫-「炎昼风物诗」",
        
        badge: getImage("Yoimiya"),
        
        genre: "Character",
        description: "宵宫天赋2：释放琉金云间草后的15秒内，附近的队伍中所有其它角色（不包括宵宫自己）攻击力提高10%。此外，依据宵宫自己释放琉金云间草时固有天赋「袖火百景图」的叠加层数，将额外提升上述的攻击力效果，每层提升1%攻击力。",
        config: [
            
            {"default":0,"max":10,"min":0,"name":"talent1_stack","title":"「袖火百景图」叠加层数","type":"int"},
            
        ],
    },
    
    "YunjinQ": {
        name: "YunjinQ",
        chs: "云堇-「飞云旗阵」",
        
        badge: getImage("Yunjin"),
        
        genre: "Character",
        description: "云堇Q技能：对敌人造成普通攻击伤害时，基于云堇自己当前的防御力，提高造成的伤害。<br>天赋「莫从恒蹊」：「飞云旗阵」提供的普通攻击伤害提高，当队伍中存在1/2/3/4种元素类型的角色时，数值上进一步追加云堇防御力的2.5%/5.0%/7.5%/11.5%。",
        config: [
            
            {"default":10,"max":15,"min":1,"name":"skill3","title":"Q技能等级","type":"int"},
            
            {"default":2000.0,"name":"def","title":"云堇的防御力","type":"floatInput"},
            
            {"default":true,"name":"talent2","title":"60级突破","type":"bool"},
            
            {"default":4,"max":4,"min":1,"name":"ele_count","title":"队伍不同属性数量","type":"int"},
            
        ],
    },
    
    "YunjinC2": {
        name: "YunjinC2",
        chs: "云堇-「诸般切末」",
        
        badge: getImage("Yunjin"),
        
        genre: "Character",
        description: "云堇命座2：施放破嶂见旌仪后，附近队伍中所有角色普通攻击造成的伤害提高15%，持续12秒。",
        config: [
            
        ],
    },
    
    "ZhongliShield": {
        name: "ZhongliShield",
        chs: "钟离-「玉璋护盾」",
        
        badge: getImage("Zhongli"),
        
        genre: "Character",
        description: "钟离盾：使附近小范围内敌人的所有元素抗性与物理抗性降低20%",
        config: [
            
        ],
    },
    
    "FreedomSworn": {
        name: "FreedomSworn",
        chs: "苍古自由之誓-「千年的大乐章·抗争之歌」",
        
        badge: FreedomSworn_image,
        
        genre: "Weapon",
        description: "「千年的大乐章·抗争之歌」效果：普通攻击、重击、下落攻击造成的伤害提升16%/20%/24%/28%/32%，攻击力提升20%/25%/30%/35%/40%。",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":"精炼","type":"intInput"},
            
        ],
    },
    
    "SongOfBrokenPines": {
        name: "SongOfBrokenPines",
        chs: "松籁响起之时-「千年的大乐章·揭旗之歌」",
        
        badge: SongOfBrokenPines_image,
        
        genre: "Weapon",
        description: "「千年的大乐章·揭旗之歌」效果：普通攻击速度提升12%/15%/18%/21%/24%，攻击力提升20%/25%/30%/35%/40%。",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":"精炼","type":"intInput"},
            
        ],
    },
    
    "WolfsGravestone": {
        name: "WolfsGravestone",
        chs: "狼的末路-「如狼般狩猎者」",
        
        badge: WolfsGravestone_image,
        
        genre: "Weapon",
        description: "攻击命中生命值低于30%的敌人时，队伍中所有成员的攻击力提高40%/50%/60%/70%/80%，持续12秒。该效果30秒只能触发一次。",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":"精炼","type":"intInput"},
            
        ],
    },
    
    "ThrillingTalesOfDragonSlayers": {
        name: "ThrillingTalesOfDragonSlayers",
        chs: "讨龙英杰谭-「传承」",
        
        badge: ThrillingTalesOfDragonSlayers_image,
        
        genre: "Weapon",
        description: "传承：主动切换角色时，新登场的角色攻击力提升24%/30%/36%/42%/48%，持续10秒。该效果每20秒只能触发一次。",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":"精炼","type":"intInput"},
            
        ],
    },
    
    "ElegyOfTheEnd": {
        name: "ElegyOfTheEnd",
        chs: "终末嗟叹之诗-「千年的大乐章·别离之歌」",
        
        badge: ElegyOfTheEnd_image,
        
        genre: "Weapon",
        description: "千年的大乐章·别离之歌」效果：元素精通提高100/125/150/175/200点，攻击力提升20%/25%/30%/35%/40%。",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":"精炼","type":"intInput"},
            
        ],
    },
    
    "HakushinRing": {
        name: "HakushinRing",
        chs: "白辰之环-「樱之斋宫」",
        
        badge: HakushinRing_image,
        
        genre: "Weapon",
        description: "樱之斋宫：装备该武器的角色触发雷元素相关反应后，队伍中附近的与该元素反应相关的元素类型的角色，获得10%/12.5%/15%/17.5%/20%对应元素的元素伤害加成，持续6秒。通过这种方式，角色获得的元素伤害加成无法叠加。",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":"精炼","type":"intInput"},
            
            {"default":"Electro","name":"element","title":"元素","type":"element8"},
            
        ],
    },
    
    "ResonancePyro2": {
        name: "ResonancePyro2",
        chs: "元素共鸣-热诚之火",
        
        badge: ResonancePyro2_image,
        
        genre: "Resonance",
        description: "攻击力提升25%。",
        config: [
            
        ],
    },
    
    "ResonanceCryo2": {
        name: "ResonanceCryo2",
        chs: "元素共鸣-粉碎之冰",
        
        badge: ResonanceCryo2_image,
        
        genre: "Resonance",
        description: "攻击冰元素附着或冻结状态下的敌人时，暴击率提高15%。",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":"应用比例","type":"float"},
            
        ],
    },
    
    "ResonanceGeo2": {
        name: "ResonanceGeo2",
        chs: "元素共鸣-坚定之岩",
        
        badge: ResonanceGeo2_image,
        
        genre: "Resonance",
        description: "护盾强效提升15%。角色处于护盾保护状态时，①造成的伤害提升15%，对敌人造成伤害时会使敌人的的②岩元素抗性降低20%，持续15秒。",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate1","title":"效果①比例","type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate2","title":"效果②比例","type":"float"},
            
        ],
    },
    
    "Instructor4": {
        name: "Instructor4",
        chs: "教官4",
        
        badge: Instructor4_image,
        
        genre: "Artifact",
        description: "触发元素反应后。队伍中所有角色元素精通提高120点，持续8秒。",
        config: [
            
        ],
    },
    
    "NoblesseOblige4": {
        name: "NoblesseOblige4",
        chs: "昔日宗室之仪4",
        
        badge: NoblesseOblige4_image,
        
        genre: "Artifact",
        description: "施放元素爆发后，队伍中所有角色攻击力提升20％，持续12秒。该效果不可叠加。",
        config: [
            
        ],
    },
    
    "ArchaicPetra4": {
        name: "ArchaicPetra4",
        chs: "悠古的磐岩4",
        
        badge: ArchaicPetra4_image,
        
        genre: "Artifact",
        description: "获得结晶反应形成的晶片时，队伍中所有角色获得35%对应元素伤害加成，持续10秒。",
        config: [
            
            {"default":"Electro","name":"element","title":"结晶元素","type":"element4"},
            
        ],
    },
    
    "ViridescentVenerer4": {
        name: "ViridescentVenerer4",
        chs: "翠绿之影4",
        
        badge: ViridescentVenerer4_image,
        
        genre: "Artifact",
        description: "根据扩散的元素类型，降低受到影响的敌人40%的对应元素抗性，持续10秒。",
        config: [
            
            {"default":"Electro","name":"element","title":"扩散元素","type":"element4"},
            
        ],
    },
    
    "TenacityOfTheMillelith4": {
        name: "TenacityOfTheMillelith4",
        chs: "千岩牢固4",
        
        badge: TenacityOfTheMillelith4_image,
        
        genre: "Artifact",
        description: "元素战技命中敌人后，使队伍中附近的所有角色攻击力提升20%，护盾强效提升30%，持续3秒。",
        config: [
            
        ],
    },
    
}