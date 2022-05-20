// generated file, do not edit


import MaxATK_image from "@image/misc/sword"



import MaxDEF_image from "@image/misc/sword"



import MaxHP_image from "@image/misc/sword"



import MaxEM_image from "@image/misc/sword"



import PyroDamage_image from "@image/misc/fire_slime"



import CryoDamage_image from "@image/misc/ice_slime"



import HydroDamage_image from "@image/misc/water_slime"



import ElectroDamage_image from "@image/misc/thunder_slime"



import AnemoDamage_image from "@image/misc/wind_slime"



import GeoDamage_image from "@image/misc/sword"



import PhysicalDamage_image from "@image/misc/sword"



import MaxVaporize_image from "@image/misc/sword"



import MaxMelt_image from "@image/misc/sword"



import ExpectVaporize_image from "@image/misc/sword"



import ExpectMelt_image from "@image/misc/sword"









































































































const template = "https://upload-bbs.mihoyo.com/game_record/genshin/character_icon/UI_AvatarIcon_#.png"
const getImage = name => template.replace("#", name)

export default {
    
    "MaxATK": {
        name: "MaxATK",
        chs: "最大攻击力",
        description: "最大化攻击力",
        tags: [
            
            "攻击",
            
        ],
        "for": "common",
        
        badge: MaxATK_image,
        
        config: [
            
        ],
    },
    
    "MaxDEF": {
        name: "MaxDEF",
        chs: "最大防御力",
        description: "最大化防御力",
        tags: [
            
            "防御",
            
        ],
        "for": "common",
        
        badge: MaxDEF_image,
        
        config: [
            
        ],
    },
    
    "MaxHP": {
        name: "MaxHP",
        chs: "最大生命值",
        description: "最大化生命值",
        tags: [
            
            "生命",
            
        ],
        "for": "common",
        
        badge: MaxHP_image,
        
        config: [
            
        ],
    },
    
    "MaxEM": {
        name: "MaxEM",
        chs: "最大元素精通",
        description: "最大化元素精通",
        tags: [
            
            "元素精通",
            
        ],
        "for": "common",
        
        badge: MaxEM_image,
        
        config: [
            
        ],
    },
    
    "PyroDamage": {
        name: "PyroDamage",
        chs: "火伤",
        description: "火元素伤害最大化或最大化期望",
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        
        badge: PyroDamage_image,
        
        config: [
            
            {"default":0,"name":"t","options":["期望","最大值"],"title":"类型","type":"option"},
            
        ],
    },
    
    "CryoDamage": {
        name: "CryoDamage",
        chs: "冰伤",
        description: "冰元素伤害最大化或最大化期望",
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        
        badge: CryoDamage_image,
        
        config: [
            
            {"default":0,"name":"t","options":["期望","最大值"],"title":"类型","type":"option"},
            
        ],
    },
    
    "HydroDamage": {
        name: "HydroDamage",
        chs: "水伤",
        description: "水元素伤害最大化或最大化期望",
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        
        badge: HydroDamage_image,
        
        config: [
            
            {"default":0,"name":"t","options":["期望","最大值"],"title":"类型","type":"option"},
            
        ],
    },
    
    "ElectroDamage": {
        name: "ElectroDamage",
        chs: "雷伤",
        description: "雷元素伤害最大化或最大化期望",
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        
        badge: ElectroDamage_image,
        
        config: [
            
            {"default":0,"name":"t","options":["期望","最大值"],"title":"类型","type":"option"},
            
        ],
    },
    
    "AnemoDamage": {
        name: "AnemoDamage",
        chs: "风伤",
        description: "风元素伤害最大化或最大化期望",
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        
        badge: AnemoDamage_image,
        
        config: [
            
            {"default":0,"name":"t","options":["期望","最大值"],"title":"类型","type":"option"},
            
        ],
    },
    
    "GeoDamage": {
        name: "GeoDamage",
        chs: "岩伤",
        description: "岩元素伤害最大化或最大化期望",
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        
        badge: GeoDamage_image,
        
        config: [
            
            {"default":0,"name":"t","options":["期望","最大值"],"title":"类型","type":"option"},
            
        ],
    },
    
    "PhysicalDamage": {
        name: "PhysicalDamage",
        chs: "物伤",
        description: "物理伤害最大化或最大化期望",
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        
        badge: PhysicalDamage_image,
        
        config: [
            
            {"default":0,"name":"t","options":["期望","最大值"],"title":"类型","type":"option"},
            
        ],
    },
    
    "MaxVaporize": {
        name: "MaxVaporize",
        chs: "最大蒸发伤害",
        description: "使得蒸发反应的伤害最高。<br><b>注意：</b>仅考虑最简单的情况，特殊机制不考虑（例如某些技能的属性转化等）",
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        
        badge: MaxVaporize_image,
        
        config: [
            
            {"default":0,"name":"t","options":["火","水"],"title":"触发元素","type":"option"},
            
            {"default":"NormalAttack","name":"skill","title":"技能","type":"skill4"},
            
        ],
    },
    
    "MaxMelt": {
        name: "MaxMelt",
        chs: "最大融化伤害",
        description: "使得融化反应的伤害最高。<br><b>注意：</b>仅考虑最简单的情况，特殊机制不考虑（例如某些技能的属性转化等）",
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        
        badge: MaxMelt_image,
        
        config: [
            
            {"default":0,"name":"t","options":["火","冰"],"title":"触发元素","type":"option"},
            
            {"default":"NormalAttack","name":"skill","title":"技能","type":"skill4"},
            
        ],
    },
    
    "ExpectVaporize": {
        name: "ExpectVaporize",
        chs: "期望蒸发伤害",
        description: "使得蒸发反应的期望伤害最高。<br><b>注意：</b>仅考虑最简单的情况，特殊机制不考虑（例如某些技能的属性转化等）",
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        
        badge: ExpectVaporize_image,
        
        config: [
            
            {"default":0,"name":"t","options":["火","水"],"title":"触发元素","type":"option"},
            
            {"default":"NormalAttack","name":"skill","title":"技能","type":"skill4"},
            
        ],
    },
    
    "ExpectMelt": {
        name: "ExpectMelt",
        chs: "期望融化伤害",
        description: "使得融化反应的期望伤害最高。<br><b>注意：</b>仅考虑最简单的情况，特殊机制不考虑（例如某些技能的属性转化等）",
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        
        badge: ExpectMelt_image,
        
        config: [
            
            {"default":0,"name":"t","options":["火","冰"],"title":"触发元素","type":"option"},
            
            {"default":"NormalAttack","name":"skill","title":"技能","type":"skill4"},
            
        ],
    },
    
    "AlbedoDefault": {
        name: "AlbedoDefault",
        chs: "阿贝多-白垩之子",
        description: "普通副C阿贝多",
        tags: [
            
            "输出",
            
        ],
        "for": "Albedo",
        
        badge: getImage("Albedo"),
        
        config: [
            
        ],
    },
    
    "AloyDefault": {
        name: "AloyDefault",
        chs: "埃洛伊-「异界的救世主」",
        description: "普通输出埃洛伊",
        tags: [
            
            "输出",
            
        ],
        "for": "Aloy",
        
        badge: getImage("Aloy"),
        
        config: [
            
        ],
    },
    
    "AmberDefault": {
        name: "AmberDefault",
        chs: "安柏-侦察骑士",
        description: "普通输出安柏",
        tags: [
            
            "输出",
            
        ],
        "for": "Amber",
        
        badge: getImage("Ambor"),
        
        config: [
            
        ],
    },
    
    "AratakiIttoDefault": {
        name: "AratakiIttoDefault",
        chs: "荒泷一斗-最恶鬼王！",
        description: "荒泷一斗输出",
        tags: [
            
            "输出",
            
        ],
        "for": "AratakiItto",
        
        badge: getImage("Itto"),
        
        config: [
            
        ],
    },
    
    "BarbaraDefault": {
        name: "BarbaraDefault",
        chs: "芭芭拉-闪耀偶像",
        description: "使得芭芭拉Q技能治疗效果最好",
        tags: [
            
            "治疗",
            
            "辅助",
            
        ],
        "for": "Barbara",
        
        badge: getImage("Barbara"),
        
        config: [
            
        ],
    },
    
    "BeidouDefault": {
        name: "BeidouDefault",
        chs: "北斗-无冕的龙王",
        description: "普通北斗弹反流",
        tags: [
            
            "输出",
            
        ],
        "for": "Beidou",
        
        badge: getImage("Beidou"),
        
        config: [
            
        ],
    },
    
    "BennettDamage": {
        name: "BennettDamage",
        chs: "班尼特-副C",
        description: "普通副C班尼特",
        tags: [
            
            "辅助",
            
            "输出",
            
            "副C",
            
        ],
        "for": "Bennett",
        
        badge: getImage("Bennett"),
        
        config: [
            
            {"default":1.8,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
            {"default":0.9,"max":1.0,"min":0.0,"name":"other_dmg_ratio","title":"他人伤害占比","type":"float"},
            
        ],
    },
    
    "BennettDefault": {
        name: "BennettDefault",
        chs: "班尼特-命运试金石",
        description: "普通辅助班尼特",
        tags: [
            
            "辅助",
            
        ],
        "for": "Bennett",
        
        badge: getImage("Bennett"),
        
        config: [
            
            {"default":1.6,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
    "ChongyunDefault": {
        name: "ChongyunDefault",
        chs: "重云-雪融有踪",
        description: "普通副C重云",
        tags: [
            
            "副C",
            
            "输出",
            
        ],
        "for": "Chongyun",
        
        badge: getImage("Chongyun"),
        
        config: [
            
        ],
    },
    
    "DilucDefault": {
        name: "DilucDefault",
        chs: "迪卢克-晨曦酒庄的贵公子",
        description: "普通输出迪卢克",
        tags: [
            
            "输出",
            
        ],
        "for": "Diluc",
        
        badge: getImage("Diluc"),
        
        config: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"melt_rate","title":"融化比例","type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"vaporize_rate","title":"蒸发比例","type":"float"},
            
        ],
    },
    
    "DionaDefault": {
        name: "DionaDefault",
        chs: "迪奥娜-猫尾特调",
        description: "普通治疗、护盾辅助",
        tags: [
            
            "治疗",
            
            "护盾",
            
        ],
        "for": "Diona",
        
        badge: getImage("Diona"),
        
        config: [
            
        ],
    },
    
    "EulaDefault": {
        name: "EulaDefault",
        chs: "优菈-浪花骑士",
        description: "普通优菈输出",
        tags: [
            
            "输出",
            
        ],
        "for": "Eula",
        
        badge: getImage("Eula"),
        
        config: [
            
        ],
    },
    
    "FischlDefault": {
        name: "FischlDefault",
        chs: "菲谢尔-断罪皇女！！",
        description: "普通元素输出菲谢尔",
        tags: [
            
            "输出",
            
        ],
        "for": "Fischl",
        
        badge: getImage("Fischl"),
        
        config: [
            
        ],
    },
    
    "GanyuDefault": {
        name: "GanyuDefault",
        chs: "甘雨-循循守月",
        description: "普通输出甘雨",
        tags: [
            
            "输出",
            
        ],
        "for": "Ganyu",
        
        badge: getImage("Ganyu"),
        
        config: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"melt_rate","title":"融化占比","type":"float"},
            
        ],
    },
    
    "GorouDefault": {
        name: "GorouDefault",
        chs: "五郎-戎犬锵锵",
        description: "普通岩辅五郎",
        tags: [
            
            "辅助",
            
        ],
        "for": "Gorou",
        
        badge: getImage("Gorou"),
        
        config: [
            
            {"default":1.7,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
    "HuTaoDefault": {
        name: "HuTaoDefault",
        chs: "胡桃-雪霁梅香",
        description: "普通输出主C胡桃",
        tags: [
            
            "输出",
            
        ],
        "for": "HuTao",
        
        badge: getImage("Hutao"),
        
        config: [
            
            {"default":0.5,"max":1.0,"min":0.0,"name":"vaporize_rate","title":"蒸发占比","type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"melt_rate","title":"融化占比","type":"float"},
            
        ],
    },
    
    "JeanDefault": {
        name: "JeanDefault",
        chs: "琴-蒲公英骑士",
        description: "普通六边形琴",
        tags: [
            
            "副C",
            
            "治疗",
            
        ],
        "for": "Jean",
        
        badge: getImage("Qin"),
        
        config: [
            
            {"default":0.5,"max":1.0,"min":0.0,"name":"damage_weight","title":"治疗-伤害比重（0：纯治疗，1：纯伤害）","type":"float"},
            
            {"default":1.0,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
    "KaedeharaKazuhaDamage": {
        name: "KaedeharaKazuhaDamage",
        chs: "枫原万叶-输出",
        description: "普通输出枫原万叶（兼辅助）",
        tags: [
            
            "输出",
            
            "辅助",
            
        ],
        "for": "KaedeharaKazuha",
        
        badge: getImage("Kazuha"),
        
        config: [
            
            {"default":1.8,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
            {"default":0.5,"max":1.0,"min":0.0,"name":"swirl_rate","title":"扩散相对频率","type":"float"},
            
            {"default":0.9,"max":1.0,"min":0.0,"name":"other_dmg_ratio","title":"他人伤害比例","type":"float"},
            
        ],
    },
    
    "KaedeharaKazuhaDefault": {
        name: "KaedeharaKazuhaDefault",
        chs: "枫原万叶-红叶逐荒波",
        description: "普通辅助万叶",
        tags: [
            
            "辅助",
            
        ],
        "for": "KaedeharaKazuha",
        
        badge: getImage("Kazuha"),
        
        config: [
            
            {"default":1.8,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
    "KaeyaDefault": {
        name: "KaeyaDefault",
        chs: "凯亚-寒风剑士",
        description: "普通冰伤凯亚",
        tags: [
            
            "输出",
            
        ],
        "for": "Kaeya",
        
        badge: getImage("Kaeya"),
        
        config: [
            
        ],
    },
    
    "KamisatoAyakaDefault": {
        name: "KamisatoAyakaDefault",
        chs: "神里绫华-白鹭霜华",
        description: "普通主C绫华",
        tags: [
            
            "输出",
            
        ],
        "for": "KamisatoAyaka",
        
        badge: getImage("Ayaka"),
        
        config: [
            
            {"default":1.0,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
    "KamisatoAyakaDps": {
        name: "KamisatoAyakaDps",
        chs: "神里绫华-DPS",
        description: "期望DPS输出，输出手法模拟如下循环：4s 左右辅助铺场，平a4段接重击，有e放e，有大开大",
        tags: [
            
            "输出",
            
        ],
        "for": "KamisatoAyaka",
        
        badge: getImage("Ayaka"),
        
        config: [
            
        ],
    },
    
    "KamisatoAyatoDefault": {
        name: "KamisatoAyatoDefault",
        chs: "神里绫人-磐祭叶守",
        description: "普通水系输出绫人",
        tags: [
            
            "输出",
            
        ],
        "for": "KamisatoAyato",
        
        badge: getImage("Ayato"),
        
        config: [
            
        ],
    },
    
    "KeqingDefault": {
        name: "KeqingDefault",
        chs: "刻晴-霆霓快雨",
        description: "普通雷伤刻晴",
        tags: [
            
            "输出",
            
        ],
        "for": "Keqing",
        
        badge: getImage("Keqing"),
        
        config: [
            
        ],
    },
    
    "KleeDefault": {
        name: "KleeDefault",
        chs: "可莉-逃跑的太阳",
        description: "可莉火伤输出",
        tags: [
            
            "输出",
            
        ],
        "for": "Klee",
        
        badge: getImage("Klee"),
        
        config: [
            
            {"default":1.0,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
    "KujouSaraDamage": {
        name: "KujouSaraDamage",
        chs: "九条裟罗-副C",
        description: "副C九条裟罗",
        tags: [
            
            "输出",
            
            "辅助",
            
        ],
        "for": "KujouSara",
        
        badge: getImage("Sara"),
        
        config: [
            
            {"default":1.6,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
    "KujouSaraDefault": {
        name: "KujouSaraDefault",
        chs: "九条裟罗-黑羽鸣镝",
        description: "普通雷系辅助九条",
        tags: [
            
            "辅助",
            
        ],
        "for": "KujouSara",
        
        badge: getImage("Sara"),
        
        config: [
            
        ],
    },
    
    "LisaDefault": {
        name: "LisaDefault",
        chs: "丽莎-蔷薇魔女",
        description: "普通输出丽莎",
        tags: [
            
            "输出",
            
        ],
        "for": "Lisa",
        
        badge: getImage("Lisa"),
        
        config: [
            
            {"default":1.0,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
    "MonaDefault": {
        name: "MonaDefault",
        chs: "莫娜-星天水镜",
        description: "普通输出莫娜",
        tags: [
            
            "输出",
            
            "辅助",
            
        ],
        "for": "Mona",
        
        badge: getImage("Mona"),
        
        config: [
            
            {"default":1.4,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
    "NingguangDefault": {
        name: "NingguangDefault",
        chs: "凝光-掩月天权",
        description: "普通输出凝光",
        tags: [
            
            "输出",
            
        ],
        "for": "Ningguang",
        
        badge: getImage("Ningguang"),
        
        config: [
            
        ],
    },
    
    "NoelleDefault": {
        name: "NoelleDefault",
        chs: "诺艾尔-未授勋之花",
        description: "普通输出诺艾尔",
        tags: [
            
            "输出",
            
        ],
        "for": "Noelle",
        
        badge: getImage("Noel"),
        
        config: [
            
        ],
    },
    
    "QiqiDefault": {
        name: "QiqiDefault",
        chs: "七七-冻冻回魂夜",
        description: "普通治疗辅助七七",
        tags: [
            
            "治疗",
            
        ],
        "for": "Qiqi",
        
        badge: getImage("Qiqi"),
        
        config: [
            
            {"default":1.6,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
    "RaidenShogunDefault": {
        name: "RaidenShogunDefault",
        chs: "雷电将军-一心净土",
        description: "普通输出雷军",
        tags: [
            
            "输出",
            
        ],
        "for": "RaidenShogun",
        
        badge: getImage("Shougun"),
        
        config: [
            
            {"default":2.6,"max":4.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
    "RazorDefault": {
        name: "RazorDefault",
        chs: "雷泽-奔狼领的传说",
        description: "普通输出物理雷泽",
        tags: [
            
            "输出",
            
        ],
        "for": "Razor",
        
        badge: getImage("Razor"),
        
        config: [
            
        ],
    },
    
    "RosariaDefault": {
        name: "RosariaDefault",
        chs: "罗莎莉亚-棘冠恩典",
        description: "普通辅助罗莎莉亚兼一定的输出",
        tags: [
            
            "辅助",
            
            "输出",
            
        ],
        "for": "Rosaria",
        
        badge: getImage("Rosaria"),
        
        config: [
            
        ],
    },
    
    "SangonomiyaKokomiDefault": {
        name: "SangonomiyaKokomiDefault",
        chs: "珊瑚宫心海-真珠之智",
        description: "普通输出心海",
        tags: [
            
            "输出",
            
            "治疗",
            
        ],
        "for": "SangonomiyaKokomi",
        
        badge: getImage("Kokomi"),
        
        config: [
            
        ],
    },
    
    "SayuDefault": {
        name: "SayuDefault",
        chs: "早柚-忍里之貉",
        description: "普通输出型早柚",
        tags: [
            
            "输出",
            
            "治疗",
            
        ],
        "for": "Sayu",
        
        badge: getImage("Sayu"),
        
        config: [
            
            {"default":1.4,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
    "ShenheDefault": {
        name: "ShenheDefault",
        chs: "申鹤-孤辰茕怀",
        description: "普通辅助申鹤",
        tags: [
            
            "辅助",
            
        ],
        "for": "Shenhe",
        
        badge: getImage("Shenhe"),
        
        config: [
            
            {"default":1.6,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
    "SucroseDefault": {
        name: "SucroseDefault",
        chs: "砂糖-无害甜度",
        description: "普通辅助砂糖",
        tags: [
            
            "辅助",
            
        ],
        "for": "Sucrose",
        
        badge: getImage("Sucrose"),
        
        config: [
            
            {"default":1.4,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
    "TartagliaDefault": {
        name: "TartagliaDefault",
        chs: "达达利亚-「公子」",
        description: "普通输出达达利鸭",
        tags: [
            
            "输出",
            
        ],
        "for": "Tartaglia",
        
        badge: getImage("Tartaglia"),
        
        config: [
            
        ],
    },
    
    "ThomaDefault": {
        name: "ThomaDefault",
        chs: "托马-渡来介者",
        description: "普通盾辅托马",
        tags: [
            
            "辅助",
            
        ],
        "for": "Thoma",
        
        badge: getImage("Tohma"),
        
        config: [
            
            {"default":2.0,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
    "VentiDefault": {
        name: "VentiDefault",
        chs: "温迪-风色诗人",
        description: "普通输出温迪",
        tags: [
            
            "输出",
            
        ],
        "for": "Venti",
        
        badge: getImage("Venti"),
        
        config: [
            
            {"default":0.7,"max":1.0,"min":0.0,"name":"swirl_rate","title":"扩散占比","type":"float"},
            
        ],
    },
    
    "XianglingDefault": {
        name: "XianglingDefault",
        chs: "香菱-万民百味",
        description: "普通输出火伤香菱",
        tags: [
            
            "输出",
            
        ],
        "for": "Xiangling",
        
        badge: getImage("Xiangling"),
        
        config: [
            
            {"default":1.8,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"melt_rate","title":"融化频率","type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"vaporize_rate","title":"蒸发频率","type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"overload_rate","title":"超载频率","type":"float"},
            
        ],
    },
    
    "XiaoDefault": {
        name: "XiaoDefault",
        chs: "魈-护法夜叉",
        description: "普通输出魈",
        tags: [
            
            "输出",
            
        ],
        "for": "Xiao",
        
        badge: getImage("Xiao"),
        
        config: [
            
        ],
    },
    
    "XingqiuDefault": {
        name: "XingqiuDefault",
        chs: "行秋-少年春衫薄",
        description: "普通副C行秋",
        tags: [
            
            "输出",
            
        ],
        "for": "Xingqiu",
        
        badge: getImage("Xingqiu"),
        
        config: [
            
            {"default":1.8,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
    "XinyanDefault": {
        name: "XinyanDefault",
        chs: "辛焱-燥热旋律",
        description: "普通辅助辛焱",
        tags: [
            
            "辅助",
            
        ],
        "for": "Xinyan",
        
        badge: getImage("Xinyan"),
        
        config: [
            
            {"default":1.4,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
            {"default":0.5,"max":1.0,"min":0.0,"name":"damage_demand","title":"伤害需求","type":"float"},
            
        ],
    },
    
    "YaeMikoDefault": {
        name: "YaeMikoDefault",
        chs: "八重神子-浮世笑百姿",
        description: "普通输出八重神子",
        tags: [
            
            "输出",
            
        ],
        "for": "YaeMiko",
        
        badge: getImage("Yae"),
        
        config: [
            
            {"default":0.0,"max":3.0,"min":0.0,"name":"electro_charged_times","title":"感电相对频率","type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"overload_times","title":"超载相对频率","type":"float"},
            
            {"default":1.0,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
    "YanfeiDefault": {
        name: "YanfeiDefault",
        chs: "烟绯-智明无邪",
        description: "普通输出烟绯",
        tags: [
            
            "输出",
            
        ],
        "for": "Yanfei",
        
        badge: getImage("Feiyan"),
        
        config: [
            
        ],
    },
    
    "YoimiyaDefault": {
        name: "YoimiyaDefault",
        chs: "宵宫-琉焰华舞",
        description: "普通输出宵宫",
        tags: [
            
            "输出",
            
        ],
        "for": "Yoimiya",
        
        badge: getImage("Yoimiya"),
        
        config: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"vaporize_rate","title":"蒸发占比","type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"melt_rate","title":"融化占比","type":"float"},
            
        ],
    },
    
    "YunjinDefault": {
        name: "YunjinDefault",
        chs: "云堇-红毹婵娟",
        description: "普通增伤辅助云堇",
        tags: [
            
            "辅助",
            
        ],
        "for": "Yunjin",
        
        badge: getImage("Yunjin"),
        
        config: [
            
            {"default":1.4,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
    "ZhongliDefault": {
        name: "ZhongliDefault",
        chs: "钟离-尘世闲游",
        description: "普通爆发钟离",
        tags: [
            
            "爆发",
            
        ],
        "for": "Zhongli",
        
        badge: getImage("Zhongli"),
        
        config: [
            
            {"default":1.4,"max":3.0,"min":1.0,"name":"recharge_demand","title":"充能需求","type":"float"},
            
        ],
    },
    
}