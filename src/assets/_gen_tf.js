// generated file, do not edit


import MaxATK_image from "@image/misc/sword"



import MaxDEF_image from "@image/misc/sword"



import MaxHP_image from "@image/misc/sword"



import MaxEM_image from "@image/misc/sword"



import MaxRecharge_image from "@image/misc/sword"



import PyroDamage_image from "@image/misc/fire_slime"



import CryoDamage_image from "@image/misc/ice_slime"



import HydroDamage_image from "@image/misc/water_slime"



import ElectroDamage_image from "@image/misc/thunder_slime"



import AnemoDamage_image from "@image/misc/wind_slime"



import DendroDamage_image from "@image/misc/dendro"



import GeoDamage_image from "@image/misc/sword"



import PhysicalDamage_image from "@image/misc/sword"



import MaxVaporize_image from "@image/misc/sword"



import MaxMelt_image from "@image/misc/sword"



import ExpectVaporize_image from "@image/misc/sword"



import ExpectMelt_image from "@image/misc/sword"













































































































































const template = "https://upload-bbs.mihoyo.com/game_record/genshin/character_icon/#.png"
const newTemplate = "https://act-webstatic.mihoyo.com/hk4e/e20200928calculate/item_icon_uceddf/#.png"
const getImage = name => template.replace("#", name)
const getIcon = md5 => newTemplate.replace("#", md5)

export default {
    
    "MaxATK": {
        name: "MaxATK",
        nameLocale: 1035,
        description: 1031,
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
        nameLocale: 1040,
        description: 1034,
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
        nameLocale: 1037,
        description: 1033,
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
        nameLocale: 1024,
        description: 1029,
        tags: [
            
            "元素精通",
            
        ],
        "for": "common",
        badge: MaxEM_image,
        config: [
            
        ],
    },
    
    "MaxRecharge": {
        name: "MaxRecharge",
        nameLocale: 1025,
        description: 1028,
        tags: [
            
            "",
            
        ],
        "for": "common",
        badge: MaxRecharge_image,
        config: [
            
        ],
    },
    
    "PyroDamage": {
        name: "PyroDamage",
        nameLocale: 1185,
        description: 1186,
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        badge: PyroDamage_image,
        config: [
            
            {"default":0,"name":"t","options":["期望","最大值"],"title":1416,"type":"option"},
            
        ],
    },
    
    "CryoDamage": {
        name: "CryoDamage",
        nameLocale: 278,
        description: 279,
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        badge: CryoDamage_image,
        config: [
            
            {"default":0,"name":"t","options":["期望","最大值"],"title":1416,"type":"option"},
            
        ],
    },
    
    "HydroDamage": {
        name: "HydroDamage",
        nameLocale: 1101,
        description: 1102,
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        badge: HydroDamage_image,
        config: [
            
            {"default":0,"name":"t","options":["期望","最大值"],"title":1416,"type":"option"},
            
        ],
    },
    
    "ElectroDamage": {
        name: "ElectroDamage",
        nameLocale: 1811,
        description: 1812,
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        badge: ElectroDamage_image,
        config: [
            
            {"default":0,"name":"t","options":["期望","最大值"],"title":1416,"type":"option"},
            
        ],
    },
    
    "AnemoDamage": {
        name: "AnemoDamage",
        nameLocale: 1849,
        description: 1851,
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        badge: AnemoDamage_image,
        config: [
            
            {"default":0,"name":"t","options":["期望","最大值"],"title":1416,"type":"option"},
            
        ],
    },
    
    "DendroDamage": {
        name: "DendroDamage",
        nameLocale: 1489,
        description: 1490,
        tags: [
            
            "",
            
        ],
        "for": "common",
        badge: DendroDamage_image,
        config: [
            
            {"default":0,"name":"t","options":["期望","最大值"],"title":1416,"type":"option"},
            
        ],
    },
    
    "GeoDamage": {
        name: "GeoDamage",
        nameLocale: 607,
        description: 608,
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        badge: GeoDamage_image,
        config: [
            
            {"default":0,"name":"t","options":["期望","最大值"],"title":1416,"type":"option"},
            
        ],
    },
    
    "PhysicalDamage": {
        name: "PhysicalDamage",
        nameLocale: 1230,
        description: 1231,
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        badge: PhysicalDamage_image,
        config: [
            
            {"default":0,"name":"t","options":["期望","最大值"],"title":1416,"type":"option"},
            
        ],
    },
    
    "MaxVaporize": {
        name: "MaxVaporize",
        nameLocale: 1038,
        description: 192,
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        badge: MaxVaporize_image,
        config: [
            
            {"default":0,"name":"t","options":["火","水"],"title":1595,"type":"option"},
            
            {"default":"NormalAttack","name":"skill","title":692,"type":"skill4"},
            
        ],
    },
    
    "MaxMelt": {
        name: "MaxMelt",
        nameLocale: 1039,
        description: 194,
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        badge: MaxMelt_image,
        config: [
            
            {"default":0,"name":"t","options":["火","冰"],"title":1595,"type":"option"},
            
            {"default":"NormalAttack","name":"skill","title":692,"type":"skill4"},
            
        ],
    },
    
    "ExpectVaporize": {
        name: "ExpectVaporize",
        nameLocale: 1047,
        description: 193,
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        badge: ExpectVaporize_image,
        config: [
            
            {"default":0,"name":"t","options":["火","水"],"title":1595,"type":"option"},
            
            {"default":"NormalAttack","name":"skill","title":692,"type":"skill4"},
            
        ],
    },
    
    "ExpectMelt": {
        name: "ExpectMelt",
        nameLocale: 1048,
        description: 195,
        tags: [
            
            "输出",
            
        ],
        "for": "common",
        badge: ExpectMelt_image,
        config: [
            
            {"default":0,"name":"t","options":["火","冰"],"title":1595,"type":"option"},
            
            {"default":"NormalAttack","name":"skill","title":692,"type":"skill4"},
            
        ],
    },
    
    "AlbedoDefault": {
        name: "AlbedoDefault",
        nameLocale: 1793,
        description: 859,
        tags: [
            
            "输出",
            
        ],
        "for": "Albedo",
        badge: getImage("UI_AvatarIcon_Albedo"),
        config: [
            
        ],
    },
    
    "AloyDefault": {
        name: "AloyDefault",
        nameLocale: 468,
        description: 982,
        tags: [
            
            "输出",
            
        ],
        "for": "Aloy",
        badge: getImage("UI_AvatarIcon_Aloy"),
        config: [
            
        ],
    },
    
    "AmberDefault": {
        name: "AmberDefault",
        nameLocale: 567,
        description: 985,
        tags: [
            
            "输出",
            
        ],
        "for": "Amber",
        badge: getImage("UI_AvatarIcon_Ambor"),
        config: [
            
        ],
    },
    
    "AratakiIttoDefault": {
        name: "AratakiIttoDefault",
        nameLocale: 1494,
        description: 1496,
        tags: [
            
            "输出",
            
        ],
        "for": "AratakiItto",
        badge: getImage("UI_AvatarIcon_Itto"),
        config: [
            
        ],
    },
    
    "BarbaraDefault": {
        name: "BarbaraDefault",
        nameLocale: 1478,
        description: 191,
        tags: [
            
            "治疗",
            
            "辅助",
            
        ],
        "for": "Barbara",
        badge: getImage("UI_AvatarIcon_Barbara"),
        config: [
            
        ],
    },
    
    "BeidouDefault": {
        name: "BeidouDefault",
        nameLocale: 340,
        description: 860,
        tags: [
            
            "输出",
            
        ],
        "for": "Beidou",
        badge: getImage("UI_AvatarIcon_Beidou"),
        config: [
            
        ],
    },
    
    "BennettDamage": {
        name: "BennettDamage",
        nameLocale: 1266,
        description: 856,
        tags: [
            
            "辅助",
            
            "输出",
            
            "副C",
            
        ],
        "for": "Bennett",
        badge: getImage("UI_AvatarIcon_Bennett"),
        config: [
            
            {"default":1.8,"max":3.0,"min":1.0,"name":"recharge_demand","title":250,"type":"float"},
            
            {"default":0.9,"max":1.0,"min":0.0,"name":"other_dmg_ratio","title":171,"type":"float"},
            
        ],
    },
    
    "BennettDefault": {
        name: "BennettDefault",
        nameLocale: 1267,
        description: 973,
        tags: [
            
            "辅助",
            
        ],
        "for": "Bennett",
        badge: getImage("UI_AvatarIcon_Bennett"),
        config: [
            
            {"default":1.6,"max":3.0,"min":1.0,"name":"recharge_demand","title":250,"type":"float"},
            
        ],
    },
    
    "ChongyunDefault": {
        name: "ChongyunDefault",
        nameLocale: 1707,
        description: 858,
        tags: [
            
            "副C",
            
            "输出",
            
        ],
        "for": "Chongyun",
        badge: getImage("UI_AvatarIcon_Chongyun"),
        config: [
            
        ],
    },
    
    "DilucDefault": {
        name: "DilucDefault",
        nameLocale: 1676,
        description: 999,
        tags: [
            
            "输出",
            
        ],
        "for": "Diluc",
        badge: getImage("UI_AvatarIcon_Diluc"),
        config: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"melt_rate","title":1543,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"vaporize_rate","title":1535,"type":"float"},
            
        ],
    },
    
    "DionaDefault": {
        name: "DionaDefault",
        nameLocale: 1679,
        description: 966,
        tags: [
            
            "治疗",
            
            "护盾",
            
        ],
        "for": "Diona",
        badge: getImage("UI_AvatarIcon_Diona"),
        config: [
            
            {"default":1.0,"max":3.0,"min":1.0,"name":"recharge_demand","title":250,"type":"float"},
            
        ],
    },
    
    "EulaDefault": {
        name: "EulaDefault",
        nameLocale: 182,
        description: 852,
        tags: [
            
            "输出",
            
        ],
        "for": "Eula",
        badge: getImage("UI_AvatarIcon_Eula"),
        config: [
            
        ],
    },
    
    "FischlDefault": {
        name: "FischlDefault",
        nameLocale: 1529,
        description: 853,
        tags: [
            
            "输出",
            
        ],
        "for": "Fischl",
        badge: getImage("UI_AvatarIcon_Fischl"),
        config: [
            
        ],
    },
    
    "GanyuDefault": {
        name: "GanyuDefault",
        nameLocale: 1286,
        description: 994,
        tags: [
            
            "输出",
            
        ],
        "for": "Ganyu",
        badge: getImage("UI_AvatarIcon_Ganyu"),
        config: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"melt_rate","title":1543,"type":"float"},
            
        ],
    },
    
    "GorouDefault": {
        name: "GorouDefault",
        nameLocale: 165,
        description: 862,
        tags: [
            
            "辅助",
            
        ],
        "for": "Gorou",
        badge: getImage("UI_AvatarIcon_Gorou"),
        config: [
            
            {"default":1.7,"max":3.0,"min":1.0,"name":"recharge_demand","title":249,"type":"float"},
            
        ],
    },
    
    "HuTaoDefault": {
        name: "HuTaoDefault",
        nameLocale: 1468,
        description: 978,
        tags: [
            
            "输出",
            
        ],
        "for": "HuTao",
        badge: getImage("UI_AvatarIcon_Hutao"),
        config: [
            
            {"default":0.5,"max":1.0,"min":0.0,"name":"vaporize_rate","title":1535,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"melt_rate","title":1543,"type":"float"},
            
        ],
    },
    
    "JeanDefault": {
        name: "JeanDefault",
        nameLocale: 1278,
        description: 854,
        tags: [
            
            "副C",
            
            "治疗",
            
        ],
        "for": "Jean",
        badge: getImage("UI_AvatarIcon_Qin"),
        config: [
            
            {"default":0.5,"max":1.0,"min":0.0,"name":"damage_weight","title":1117,"type":"float"},
            
            {"default":1.0,"max":3.0,"min":1.0,"name":"recharge_demand","title":249,"type":"float"},
            
        ],
    },
    
    "KaedeharaKazuhaDamage": {
        name: "KaedeharaKazuhaDamage",
        nameLocale: 1065,
        description: 989,
        tags: [
            
            "输出",
            
            "辅助",
            
        ],
        "for": "KaedeharaKazuha",
        badge: getImage("UI_AvatarIcon_Kazuha"),
        config: [
            
            {"default":1.8,"max":3.0,"min":1.0,"name":"recharge_demand","title":250,"type":"float"},
            
            {"default":0.5,"max":1.0,"min":0.0,"name":"swirl_rate","title":689,"type":"float"},
            
            {"default":0.9,"max":1.0,"min":0.0,"name":"other_dmg_ratio","title":171,"type":"float"},
            
        ],
    },
    
    "KaedeharaKazuhaDefault": {
        name: "KaedeharaKazuhaDefault",
        nameLocale: 1064,
        description: 972,
        tags: [
            
            "辅助",
            
        ],
        "for": "KaedeharaKazuha",
        badge: getImage("UI_AvatarIcon_Kazuha"),
        config: [
            
            {"default":1.8,"max":3.0,"min":1.0,"name":"recharge_demand","title":250,"type":"float"},
            
        ],
    },
    
    "KaeyaDefault": {
        name: "KaeyaDefault",
        nameLocale: 306,
        description: 855,
        tags: [
            
            "输出",
            
        ],
        "for": "Kaeya",
        badge: getImage("UI_AvatarIcon_Kaeya"),
        config: [
            
        ],
    },
    
    "KamisatoAyakaDefault": {
        name: "KamisatoAyakaDefault",
        nameLocale: 1384,
        description: 851,
        tags: [
            
            "输出",
            
        ],
        "for": "KamisatoAyaka",
        badge: getImage("UI_AvatarIcon_Ayaka"),
        config: [
            
            {"default":1.0,"max":3.0,"min":1.0,"name":"recharge_demand","title":250,"type":"float"},
            
        ],
    },
    
    "KamisatoAyakaDps": {
        name: "KamisatoAyakaDps",
        nameLocale: 1382,
        description: 1046,
        tags: [
            
            "输出",
            
        ],
        "for": "KamisatoAyaka",
        badge: getImage("UI_AvatarIcon_Ayaka"),
        config: [
            
        ],
    },
    
    "KamisatoAyatoDefault": {
        name: "KamisatoAyatoDefault",
        nameLocale: 1378,
        description: 965,
        tags: [
            
            "输出",
            
        ],
        "for": "KamisatoAyato",
        badge: getImage("UI_AvatarIcon_Ayato"),
        config: [
            
        ],
    },
    
    "KeqingDefault": {
        name: "KeqingDefault",
        nameLocale: 322,
        description: 1003,
        tags: [
            
            "输出",
            
        ],
        "for": "Keqing",
        badge: getImage("UI_AvatarIcon_Keqing"),
        config: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"aggravate_rate","title":1639,"type":"float"},
            
        ],
    },
    
    "KleeDefault": {
        name: "KleeDefault",
        nameLocale: 391,
        description: 394,
        tags: [
            
            "输出",
            
        ],
        "for": "Klee",
        badge: getImage("UI_AvatarIcon_Klee"),
        config: [
            
            {"default":1.0,"max":3.0,"min":1.0,"name":"recharge_demand","title":250,"type":"float"},
            
        ],
    },
    
    "KujouSaraDamage": {
        name: "KujouSaraDamage",
        nameLocale: 121,
        description: 327,
        tags: [
            
            "输出",
            
            "辅助",
            
        ],
        "for": "KujouSara",
        badge: getImage("UI_AvatarIcon_Sara"),
        config: [
            
            {"default":1.6,"max":3.0,"min":1.0,"name":"recharge_demand","title":250,"type":"float"},
            
        ],
    },
    
    "KujouSaraDefault": {
        name: "KujouSaraDefault",
        nameLocale: 122,
        description: 1004,
        tags: [
            
            "辅助",
            
        ],
        "for": "KujouSara",
        badge: getImage("UI_AvatarIcon_Sara"),
        config: [
            
        ],
    },
    
    "LisaDefault": {
        name: "LisaDefault",
        nameLocale: 111,
        description: 979,
        tags: [
            
            "输出",
            
        ],
        "for": "Lisa",
        badge: getImage("UI_AvatarIcon_Lisa"),
        config: [
            
            {"default":1.0,"max":3.0,"min":1.0,"name":"recharge_demand","title":249,"type":"float"},
            
        ],
    },
    
    "MonaDefault": {
        name: "MonaDefault",
        nameLocale: 1503,
        description: 995,
        tags: [
            
            "输出",
            
            "辅助",
            
        ],
        "for": "Mona",
        badge: getImage("UI_AvatarIcon_Mona"),
        config: [
            
            {"default":1.4,"max":3.0,"min":1.0,"name":"recharge_demand","title":249,"type":"float"},
            
        ],
    },
    
    "NingguangDefault": {
        name: "NingguangDefault",
        nameLocale: 300,
        description: 980,
        tags: [
            
            "输出",
            
        ],
        "for": "Ningguang",
        badge: getImage("UI_AvatarIcon_Ningguang"),
        config: [
            
        ],
    },
    
    "NoelleDefault": {
        name: "NoelleDefault",
        nameLocale: 1618,
        description: 997,
        tags: [
            
            "输出",
            
        ],
        "for": "Noelle",
        badge: getImage("UI_AvatarIcon_Noel"),
        config: [
            
        ],
    },
    
    "QiqiDefault": {
        name: "QiqiDefault",
        nameLocale: 77,
        description: 968,
        tags: [
            
            "治疗",
            
        ],
        "for": "Qiqi",
        badge: getImage("UI_AvatarIcon_Qiqi"),
        config: [
            
            {"default":1.6,"max":3.0,"min":1.0,"name":"recharge_demand","title":249,"type":"float"},
            
        ],
    },
    
    "RaidenShogunDefault": {
        name: "RaidenShogunDefault",
        nameLocale: 1826,
        description: 1001,
        tags: [
            
            "输出",
            
        ],
        "for": "RaidenShogun",
        badge: getImage("UI_AvatarIcon_Shougun"),
        config: [
            
            {"default":2.6,"max":4.0,"min":1.0,"name":"recharge_demand","title":250,"type":"float"},
            
        ],
    },
    
    "RazorDefault": {
        name: "RazorDefault",
        nameLocale: 1819,
        description: 993,
        tags: [
            
            "输出",
            
        ],
        "for": "Razor",
        badge: getImage("UI_AvatarIcon_Razor"),
        config: [
            
        ],
    },
    
    "RosariaDefault": {
        name: "RosariaDefault",
        nameLocale: 1444,
        description: 976,
        tags: [
            
            "辅助",
            
            "输出",
            
        ],
        "for": "Rosaria",
        badge: getImage("UI_AvatarIcon_Rosaria"),
        config: [
            
        ],
    },
    
    "SangonomiyaKokomiDefault": {
        name: "SangonomiyaKokomiDefault",
        nameLocale: 1259,
        description: 987,
        tags: [
            
            "输出",
            
            "治疗",
            
        ],
        "for": "SangonomiyaKokomi",
        badge: getImage("UI_AvatarIcon_Kokomi"),
        config: [
            
        ],
    },
    
    "SayuDefault": {
        name: "SayuDefault",
        nameLocale: 832,
        description: 981,
        tags: [
            
            "输出",
            
            "治疗",
            
        ],
        "for": "Sayu",
        badge: getImage("UI_AvatarIcon_Sayu"),
        config: [
            
            {"default":1.4,"max":3.0,"min":1.0,"name":"recharge_demand","title":249,"type":"float"},
            
        ],
    },
    
    "ShenheDefault": {
        name: "ShenheDefault",
        nameLocale: 1320,
        description: 974,
        tags: [
            
            "辅助",
            
        ],
        "for": "Shenhe",
        badge: getImage("UI_AvatarIcon_Shenhe"),
        config: [
            
            {"default":1.6,"max":3.0,"min":1.0,"name":"recharge_demand","title":249,"type":"float"},
            
        ],
    },
    
    "SucroseDefault": {
        name: "SucroseDefault",
        nameLocale: 1352,
        description: 975,
        tags: [
            
            "辅助",
            
        ],
        "for": "Sucrose",
        badge: getImage("UI_AvatarIcon_Sucrose"),
        config: [
            
            {"default":1.4,"max":3.0,"min":1.0,"name":"recharge_demand","title":249,"type":"float"},
            
        ],
    },
    
    "TartagliaDefault": {
        name: "TartagliaDefault",
        nameLocale: 1670,
        description: 998,
        tags: [
            
            "输出",
            
        ],
        "for": "Tartaglia",
        badge: getImage("UI_AvatarIcon_Tartaglia"),
        config: [
            
        ],
    },
    
    "ThomaDefault": {
        name: "ThomaDefault",
        nameLocale: 682,
        description: 971,
        tags: [
            
            "辅助",
            
        ],
        "for": "Thoma",
        badge: getImage("UI_AvatarIcon_Tohma"),
        config: [
            
            {"default":2.0,"max":3.0,"min":1.0,"name":"recharge_demand","title":249,"type":"float"},
            
        ],
    },
    
    "VentiDefault": {
        name: "VentiDefault",
        nameLocale: 1169,
        description: 990,
        tags: [
            
            "输出",
            
        ],
        "for": "Venti",
        badge: getImage("UI_AvatarIcon_Venti"),
        config: [
            
            {"default":0.7,"max":1.0,"min":0.0,"name":"swirl_rate","title":686,"type":"float"},
            
        ],
    },
    
    "XianglingDefault": {
        name: "XianglingDefault",
        nameLocale: 1886,
        description: 991,
        tags: [
            
            "输出",
            
        ],
        "for": "Xiangling",
        badge: getImage("UI_AvatarIcon_Xiangling"),
        config: [
            
            {"default":1.8,"max":3.0,"min":1.0,"name":"recharge_demand","title":249,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"melt_rate","title":1543,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"vaporize_rate","title":1535,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"overload_rate","title":1645,"type":"float"},
            
        ],
    },
    
    "XiaoDefault": {
        name: "XiaoDefault",
        nameLocale: 1895,
        description: 1002,
        tags: [
            
            "输出",
            
        ],
        "for": "Xiao",
        badge: getImage("UI_AvatarIcon_Xiao"),
        config: [
            
        ],
    },
    
    "XingqiuDefault": {
        name: "XingqiuDefault",
        nameLocale: 1548,
        description: 857,
        tags: [
            
            "输出",
            
        ],
        "for": "Xingqiu",
        badge: getImage("UI_AvatarIcon_Xingqiu"),
        config: [
            
            {"default":1.8,"max":3.0,"min":1.0,"name":"recharge_demand","title":249,"type":"float"},
            
        ],
    },
    
    "XinyanDamage": {
        name: "XinyanDamage",
        nameLocale: 1664,
        description: 970,
        tags: [
            
            "输出",
            
        ],
        "for": "Xinyan",
        badge: getImage("UI_AvatarIcon_Xinyan"),
        config: [
            
        ],
    },
    
    "XinyanDefault": {
        name: "XinyanDefault",
        nameLocale: 1663,
        description: 977,
        tags: [
            
            "辅助",
            
        ],
        "for": "Xinyan",
        badge: getImage("UI_AvatarIcon_Xinyan"),
        config: [
            
            {"default":1.4,"max":3.0,"min":1.0,"name":"recharge_demand","title":249,"type":"float"},
            
            {"default":0.5,"max":1.0,"min":0.0,"name":"damage_demand","title":186,"type":"float"},
            
        ],
    },
    
    "YaeMikoDefault": {
        name: "YaeMikoDefault",
        nameLocale: 255,
        description: 727,
        tags: [
            
            "输出",
            
        ],
        "for": "YaeMiko",
        badge: getImage("UI_AvatarIcon_Yae"),
        config: [
            
            {"default":1.0,"max":3.0,"min":1.0,"name":"recharge_requirement","title":249,"type":"float"},
            
            {"default":0,"name":"combo","options":["不站场平A","站场平A"],"title":1672,"type":"option"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"aggravate_rate","title":1639,"type":"float"},
            
            {"default":0.0,"max":4.0,"min":0.0,"name":"hyperbloom_rate","title":1640,"type":"float"},
            
        ],
    },
    
    "YanfeiDefault": {
        name: "YanfeiDefault",
        nameLocale: 1214,
        description: 992,
        tags: [
            
            "输出",
            
        ],
        "for": "Yanfei",
        badge: getImage("UI_AvatarIcon_Feiyan"),
        config: [
            
        ],
    },
    
    "YelanDefault": {
        name: "YelanDefault",
        nameLocale: 497,
        description: 983,
        tags: [
            
            "输出",
            
        ],
        "for": "Yelan",
        badge: getImage("UI_AvatarIcon_Yelan"),
        config: [
            
            {"default":1.0,"max":3.0,"min":1.0,"name":"recharge_demand","title":250,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"vaporize_rate","title":1535,"type":"float"},
            
        ],
    },
    
    "YoimiyaDefault": {
        name: "YoimiyaDefault",
        nameLocale: 583,
        description: 986,
        tags: [
            
            "输出",
            
        ],
        "for": "Yoimiya",
        badge: getImage("UI_AvatarIcon_Yoimiya"),
        config: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"vaporize_rate","title":1535,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"melt_rate","title":1543,"type":"float"},
            
        ],
    },
    
    "YunjinDefault": {
        name: "YunjinDefault",
        nameLocale: 151,
        description: 861,
        tags: [
            
            "辅助",
            
        ],
        "for": "Yunjin",
        badge: getImage("UI_AvatarIcon_Yunjin"),
        config: [
            
            {"default":1.4,"max":3.0,"min":1.0,"name":"recharge_demand","title":249,"type":"float"},
            
        ],
    },
    
    "ZhongliDefault": {
        name: "ZhongliDefault",
        nameLocale: 1745,
        description: 969,
        tags: [
            
            "爆发",
            
        ],
        "for": "Zhongli",
        badge: getImage("UI_AvatarIcon_Zhongli"),
        config: [
            
            {"default":1.4,"max":3.0,"min":1.0,"name":"recharge_demand","title":249,"type":"float"},
            
        ],
    },
    
    "KukiShinobuDefault": {
        name: "KukiShinobuDefault",
        nameLocale: 114,
        description: 1658,
        tags: [
            
            "辅助",
            
        ],
        "for": "KukiShinobu",
        badge: getImage("UI_AvatarIcon_Shinobu"),
        config: [
            
            {"default":0.6,"max":1.0,"min":0.0,"name":"e_ratio","title":13,"type":"float"},
            
        ],
    },
    
    "ShikanoinHeizouDefault": {
        name: "ShikanoinHeizouDefault",
        nameLocale: 1910,
        description: 1659,
        tags: [
            
            "输出",
            
        ],
        "for": "ShikanoinHeizou",
        badge: getImage("UI_AvatarIcon_Heizo"),
        config: [
            
        ],
    },
    
    "TighnariDefault": {
        name: "TighnariDefault",
        nameLocale: 733,
        description: 1032,
        tags: [
            
            "",
            
        ],
        "for": "Tighnari",
        badge: getImage("UI_AvatarIcon_Tighnari"),
        config: [
            
        ],
    },
    
    "CynoDefault": {
        name: "CynoDefault",
        nameLocale: 1635,
        description: 678,
        tags: [
            
            "输出",
            
        ],
        "for": "Cyno",
        badge: getImage("UI_AvatarIcon_Cyno"),
        config: [
            
            {"default":1.3,"max":3.0,"min":1.0,"name":"recharge_requirement","title":249,"type":"float"},
            
            {"default":0,"name":"combo","options":["乱a不取消","取消第五段"],"title":1672,"type":"option"},
            
            {"default":false,"name":"until_expire","title":21,"type":"bool"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"aggravate_rate","title":1639,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"elecharged_rate","title":669,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"overload_rate","title":1644,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"hyperbloom_rate","title":1640,"type":"float"},
            
        ],
    },
    
    "NilouDefault": {
        name: "NilouDefault",
        nameLocale: 549,
        description: 984,
        tags: [
            
            "",
            
        ],
        "for": "Nilou",
        badge: getImage("UI_AvatarIcon_Nilou"),
        config: [
            
            {"default":5.0,"max":10.0,"min":0.0,"name":"e_ratio","title":215,"type":"float"},
            
            {"default":1.0,"max":10.0,"min":0.0,"name":"q_ratio","title":233,"type":"float"},
            
            {"default":3.0,"max":10.0,"min":0.0,"name":"bloom_ratio","title":1439,"type":"float"},
            
            {"default":1000.0,"max":3000.0,"min":0.0,"name":"other_em","title":1776,"type":"float"},
            
            {"default":7.0,"max":10.0,"min":0.0,"name":"other_bloom_ratio","title":1777,"type":"float"},
            
        ],
    },
    
    "NahidaDefault": {
        name: "NahidaDefault",
        nameLocale: 1424,
        description: 196,
        tags: [
            
            "输出",
            
        ],
        "for": "Nahida",
        badge: getImage("UI_AvatarIcon_Nahida"),
        config: [
            
            {"default":0,"max":1500,"min":0,"name":"em_requirement","title":1418,"type":"int"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"spread_rate","title":1537,"type":"float"},
            
            {"default":0.0,"max":4.0,"min":0.0,"name":"bloom_count","title":1439,"type":"float"},
            
            {"default":0.0,"max":3.0,"min":0.0,"name":"burn_duration","title":1223,"type":"float"},
            
            {"default":0,"max":2,"min":0,"name":"pryo_teammate_count","title":1190,"type":"int"},
            
        ],
    },
    
    "WandererDefault": {
        name: "WandererDefault",
        nameLocale: 1141,
        description: 1603,
        tags: [
            
            "输出",
            
        ],
        "for": "Wanderer",
        badge: getImage("UI_AvatarIcon_Wanderer"),
        config: [
            
            {"default":false,"name":"e_hydro","title":30,"type":"bool"},
            
            {"default":false,"name":"e_pyro","title":31,"type":"bool"},
            
            {"default":false,"name":"e_cryo","title":29,"type":"bool"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"spd_extra","title":1848,"type":"float"},
            
            {"default":1.0,"max":1.5,"min":0.5,"name":"spd_comp","title":759,"type":"float"},
            
            {"default":3,"max":12,"min":0,"name":"dash_count","title":33,"type":"int"},
            
            {"default":5,"max":5,"min":0,"name":"q_count","title":16,"type":"int"},
            
            {"default":12,"max":24,"min":0,"name":"swirl_count","title":688,"type":"int"},
            
        ],
    },
    
    "FaruzanDamage": {
        name: "FaruzanDamage",
        nameLocale: 1261,
        description: 190,
        tags: [
            
            "",
            
        ],
        "for": "Faruzan",
        badge: getImage("UI_AvatarIcon_Faruzan"),
        config: [
            
            {"default":2.0,"max":3.0,"min":1.0,"name":"recharge_demand","title":249,"type":"float"},
            
        ],
    },
    
    "AlhaithamDefault": {
        name: "AlhaithamDefault",
        nameLocale: 1474,
        description: 175,
        tags: [
            
            "",
            
        ],
        "for": "Alhaitham",
        badge: getImage("UI_AvatarIcon_Alhatham"),
        config: [
            
            {"default":5.0,"max":10.0,"min":0.0,"name":"charged_ratio","title":1731,"type":"float"},
            
            {"default":5.0,"max":10.0,"min":0.0,"name":"e_ratio","title":226,"type":"float"},
            
            {"default":1.0,"max":10.0,"min":0.0,"name":"q_ratio","title":235,"type":"float"},
            
            {"default":0.3,"max":1.0,"min":0.0,"name":"spread_ratio","title":1537,"type":"float"},
            
        ],
    },
    
    "DehyaDefault": {
        name: "DehyaDefault",
        nameLocale: 1682,
        description: 1000,
        tags: [
            
            "输出",
            
        ],
        "for": "Dehya",
        badge: getImage("UI_AvatarIcon_Dehya"),
        config: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"melt_rate","title":1543,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"vaporize_rate","title":1535,"type":"float"},
            
            {"default":6,"max":20,"min":0,"name":"e_count","title":106,"type":"int"},
            
        ],
    },
    
    "MikaDefault": {
        name: "MikaDefault",
        nameLocale: 1415,
        description: 967,
        tags: [
            
            "治疗",
            
            "护盾",
            
        ],
        "for": "Mika",
        badge: getImage("UI_AvatarIcon_Mika"),
        config: [
            
            {"default":2.0,"max":3.0,"min":1.0,"name":"recharge_demand","title":250,"type":"float"},
            
            {"default":0.6,"max":1.0,"min":0.0,"name":"crit_demand","title":1019,"type":"float"},
            
        ],
    },
    
    "FreminetDefault": {
        name: "FreminetDefault",
        nameLocale: 1527,
        description: 996,
        tags: [
            
            "",
            
        ],
        "for": "Freminet",
        badge: getIcon("7ca0ad25c2cbb36cd55a8a19c1b2a39f"),
        config: [
            
        ],
    },
    
    "LyneyDefault": {
        name: "LyneyDefault",
        nameLocale: 1059,
        description: 988,
        tags: [
            
            "",
            
        ],
        "for": "Lyney",
        badge: getIcon("15c0fae62ec91222148b753e5445c5fe"),
        config: [
            
        ],
    },
    
    "NeuvilletteDefault": {
        name: "NeuvilletteDefault",
        nameLocale: 1702,
        description: 1703,
        tags: [
            
            "",
            
        ],
        "for": "Neuvillette",
        badge: getIcon("965af2f32a5376affcb99afb9915a23d"),
        config: [
            
        ],
    },
    
    "WriothesleyDefault": {
        name: "WriothesleyDefault",
        nameLocale: 1508,
        description: 1030,
        tags: [
            
            "",
            
        ],
        "for": "Wriothesley",
        badge: getIcon("e2ea36ecfdb6f53717b1cadd394fbf49"),
        config: [
            
            {"default":0.5,"max":5.0,"min":0.0,"name":"punch_ratio","title":58,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"melt_rate","title":1543,"type":"float"},
            
        ],
    },
    
    "FurinaDefault": {
        name: "FurinaDefault",
        nameLocale: 1476,
        description: 1027,
        tags: [
            
            "",
            
        ],
        "for": "Furina",
        badge: getIcon("4da8d9d663e2e59f63c19815074074de"),
        config: [
            
        ],
    },
    
}