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



import BaseDmg_image from "@image/misc/sword"









































































































































































import ResonancePyro2_image from "@image/misc/pyro"



import ResonanceCryo2_image from "@image/misc/cryo"



import ResonanceGeo2_image from "@image/misc/geo"



import ResonanceHydro2_image from "@image/misc/hydro"



import ResonanceDendro2_image from "@image/misc/dendro"















const template = "https://upload-bbs.mihoyo.com/game_record/genshin/character_icon/UI_AvatarIcon_#.png"
const getImage = name => template.replace("#", name)
const templateWeapon = "https://upload-bbs.mihoyo.com/game_record/genshin/equip/UI_EquipIcon_#.png"
const getImageW = name => templateWeapon.replace("#", name)
const templateArtifact = "https://upload-bbs.mihoyo.com/game_record/genshin/equip/#.png"
const getImageA = name => templateArtifact.replace("#", name)

export default {
    
    "ATKPercentage": {
        name: "ATKPercentage",
        nameLocale: 728,
        
        description: null,
        
        
        badge: ATKPercentage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":768,"type":"floatPercentageInput"},
            
        ],
    },
    
    "DEFPercentage": {
        name: "DEFPercentage",
        nameLocale: 1754,
        
        description: null,
        
        
        badge: DEFPercentage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":768,"type":"floatPercentageInput"},
            
        ],
    },
    
    "HPPercentage": {
        name: "HPPercentage",
        nameLocale: 1273,
        
        description: null,
        
        
        badge: HPPercentage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":768,"type":"floatPercentageInput"},
            
        ],
    },
    
    "ATKFixed": {
        name: "ATKFixed",
        nameLocale: 727,
        
        description: null,
        
        
        badge: ATKFixed_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":768,"type":"floatInput"},
            
        ],
    },
    
    "DEFFixed": {
        name: "DEFFixed",
        nameLocale: 1753,
        
        description: null,
        
        
        badge: DEFFixed_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":768,"type":"floatInput"},
            
        ],
    },
    
    "HPFixed": {
        name: "HPFixed",
        nameLocale: 1272,
        
        description: null,
        
        
        badge: HPFixed_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":768,"type":"floatInput"},
            
        ],
    },
    
    "Critical": {
        name: "Critical",
        nameLocale: 1000,
        
        description: null,
        
        
        badge: Critical_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":768,"type":"floatPercentageInput"},
            
        ],
    },
    
    "CriticalDamage": {
        name: "CriticalDamage",
        nameLocale: 996,
        
        description: null,
        
        
        badge: CriticalDamage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":768,"type":"floatPercentageInput"},
            
        ],
    },
    
    "CustomBonus": {
        name: "CustomBonus",
        nameLocale: 176,
        
        description: null,
        
        
        badge: CustomBonus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":768,"type":"floatPercentageInput"},
            
        ],
    },
    
    "ElementalMastery": {
        name: "ElementalMastery",
        nameLocale: 232,
        
        description: null,
        
        
        badge: ElementalMastery_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":768,"type":"floatInput"},
            
        ],
    },
    
    "Recharge": {
        name: "Recharge",
        nameLocale: 195,
        
        description: null,
        
        
        badge: Recharge_image,
        
        genre: "Common",
        config: [
            
            {"default":20.0,"name":"p","title":768,"type":"floatPercentageInput"},
            
        ],
    },
    
    "DEFMinus": {
        name: "DEFMinus",
        nameLocale: 287,
        
        description: 0,
        
        
        badge: DEFMinus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":768,"type":"floatPercentageInput"},
            
        ],
    },
    
    "ResMinus": {
        name: "ResMinus",
        nameLocale: 286,
        
        description: null,
        
        
        badge: ResMinus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":768,"type":"floatPercentageInput"},
            
        ],
    },
    
    "HealingBonus": {
        name: "HealingBonus",
        nameLocale: 1103,
        
        description: null,
        
        
        badge: HealingBonus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":768,"type":"floatPercentageInput"},
            
        ],
    },
    
    "BaseDmg": {
        name: "BaseDmg",
        nameLocale: 465,
        
        description: 1625,
        
        
        badge: BaseDmg_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":768,"type":"floatInput"},
            
        ],
    },
    
    "AlbedoTalent2": {
        name: "AlbedoTalent2",
        nameLocale: 1766,
        
        description: 1771,
        
        
        badge: getImage("Albedo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AlbedoC4": {
        name: "AlbedoC4",
        nameLocale: 1767,
        
        description: 1769,
        
        
        badge: getImage("Albedo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AlbedoC6": {
        name: "AlbedoC6",
        nameLocale: 1765,
        
        description: 1770,
        
        
        badge: getImage("Albedo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AloyTalent1": {
        name: "AloyTalent1",
        nameLocale: 460,
        
        description: 461,
        
        
        badge: getImage("Aloy"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AratakiIttoC4": {
        name: "AratakiIttoC4",
        nameLocale: 1471,
        
        description: 1473,
        
        
        badge: getImage("Itto"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "BeidouC6": {
        name: "BeidouC6",
        nameLocale: 330,
        
        description: 332,
        
        
        badge: getImage("Beidou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "BennettQ": {
        name: "BennettQ",
        nameLocale: 1248,
        
        description: 1251,
        
        
        badge: getImage("Bennett"),
        
        genre: "Character",
        config: [
            
            {"default":800.0,"name":"base_atk","title":1253,"type":"floatInput"},
            
            {"default":true,"name":"c1","title":834,"type":"bool"},
            
            {"default":10,"max":15,"min":1,"name":"skill3","title":696,"type":"int"},
            
        ],
    },
    
    "BennettC6": {
        name: "BennettC6",
        nameLocale: 1247,
        
        description: 1252,
        
        
        badge: getImage("Bennett"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "ChongyunTalent2": {
        name: "ChongyunTalent2",
        nameLocale: 1681,
        
        description: 1683,
        
        
        badge: getImage("Chongyun"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "DionaC6G50": {
        name: "DionaC6G50",
        nameLocale: 1653,
        
        description: 1655,
        
        
        badge: getImage("Diona"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "EulaE": {
        name: "EulaE",
        nameLocale: 172,
        
        description: 174,
        
        
        badge: getImage("Eula"),
        
        genre: "Character",
        config: [
            
            {"default":9,"max":15,"min":1,"name":"skill2","title":696,"type":"int"},
            
        ],
    },
    
    "GanyuTalent2": {
        name: "GanyuTalent2",
        nameLocale: 1267,
        
        description: 1271,
        
        
        badge: getImage("Ganyu"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GanyuC1": {
        name: "GanyuC1",
        nameLocale: 1268,
        
        description: 1270,
        
        
        badge: getImage("Ganyu"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GorouE1": {
        name: "GorouE1",
        nameLocale: 154,
        
        description: 159,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
            {"default":10,"max":15,"min":1,"name":"skill2","title":158,"type":"int"},
            
        ],
    },
    
    "GorouE3": {
        name: "GorouE3",
        nameLocale: 155,
        
        description: 160,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GorouTalent1": {
        name: "GorouTalent1",
        nameLocale: 153,
        
        description: 162,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GorouC6": {
        name: "GorouC6",
        nameLocale: 156,
        
        description: 161,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
            {"default":1,"max":3,"min":1,"name":"level","title":1822,"type":"int"},
            
        ],
    },
    
    "HuTaoTalent1": {
        name: "HuTaoTalent1",
        nameLocale: 1448,
        
        description: 1450,
        
        
        badge: getImage("Hutao"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "JeanC4": {
        name: "JeanC4",
        nameLocale: 1260,
        
        description: 1262,
        
        
        badge: getImage("Qin"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KaedeharaKazuhaTalent2": {
        name: "KaedeharaKazuhaTalent2",
        nameLocale: 1048,
        
        description: 1052,
        
        
        badge: getImage("Kazuha"),
        
        genre: "Character",
        config: [
            
            {"default":"Electro","name":"element","title":675,"type":"element4"},
            
            {"default":800.0,"name":"em","title":75,"type":"floatInput"},
            
        ],
    },
    
    "KaedeharaKazuhaC2": {
        name: "KaedeharaKazuhaC2",
        nameLocale: 1047,
        
        description: 1051,
        
        
        badge: getImage("Kazuha"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KamisatoAyakaC4": {
        name: "KamisatoAyakaC4",
        nameLocale: 1365,
        
        description: 1416,
        
        
        badge: getImage("Ayaka"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KleeC2": {
        name: "KleeC2",
        nameLocale: 381,
        
        description: 383,
        
        
        badge: getImage("Klee"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KleeC6": {
        name: "KleeC6",
        nameLocale: 380,
        
        description: 384,
        
        
        badge: getImage("Klee"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KujouSaraEOrQ": {
        name: "KujouSaraEOrQ",
        nameLocale: 112,
        
        description: 115,
        
        
        badge: getImage("Sara"),
        
        genre: "Character",
        config: [
            
            {"default":700.0,"name":"base_atk","title":116,"type":"floatInput"},
            
            {"default":false,"name":"c6","title":837,"type":"bool"},
            
            {"default":10,"max":15,"min":1,"name":"skill2","title":13,"type":"int"},
            
        ],
    },
    
    "LisaTalent2": {
        name: "LisaTalent2",
        nameLocale: 103,
        
        description: 105,
        
        
        badge: getImage("Lisa"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "MonaQ": {
        name: "MonaQ",
        nameLocale: 1479,
        
        description: 1482,
        
        
        badge: getImage("Mona"),
        
        genre: "Character",
        config: [
            
            {"default":9,"max":15,"min":1,"name":"skill3","title":19,"type":"int"},
            
            {"default":false,"name":"c4","title":836,"type":"bool"},
            
        ],
    },
    
    "MonaC1": {
        name: "MonaC1",
        nameLocale: 1480,
        
        description: 1483,
        
        
        badge: getImage("Mona"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "NingguangTalent2": {
        name: "NingguangTalent2",
        nameLocale: 290,
        
        description: 292,
        
        
        badge: getImage("Ningguang"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "RaidenShogunE": {
        name: "RaidenShogunE",
        nameLocale: 1800,
        
        description: 1803,
        
        
        badge: getImage("Shougun"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"skill2","title":1802,"type":"int"},
            
            {"default":80,"max":100,"min":20,"name":"energy","title":360,"type":"int"},
            
        ],
    },
    
    "RaidenShogunC4": {
        name: "RaidenShogunC4",
        nameLocale: 1799,
        
        description: 1804,
        
        
        badge: getImage("Shougun"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "RazorC4": {
        name: "RazorC4",
        nameLocale: 1793,
        
        description: 1795,
        
        
        badge: getImage("Razor"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "RosariaTalent2": {
        name: "RosariaTalent2",
        nameLocale: 1424,
        
        description: 1427,
        
        
        badge: getImage("Rosaria"),
        
        genre: "Character",
        config: [
            
            {"default":70.0,"name":"crit","title":1428,"type":"floatPercentageInput"},
            
        ],
    },
    
    "RosariaC6": {
        name: "RosariaC6",
        nameLocale: 1423,
        
        description: 1426,
        
        
        badge: getImage("Rosaria"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "ShenheE": {
        name: "ShenheE",
        nameLocale: 1298,
        
        description: 1304,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":3000.0,"name":"atk","title":1309,"type":"floatInput"},
            
            {"default":8,"max":15,"min":1,"name":"skill2","title":1303,"type":"int"},
            
        ],
    },
    
    "ShenheQ": {
        name: "ShenheQ",
        nameLocale: 1300,
        
        description: 1306,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"skill3","title":1305,"type":"int"},
            
        ],
    },
    
    "ShenheTalent1": {
        name: "ShenheTalent1",
        nameLocale: 1299,
        
        description: 1307,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"c2","title":835,"type":"bool"},
            
        ],
    },
    
    "ShenheTalent2": {
        name: "ShenheTalent2",
        nameLocale: 1301,
        
        description: 1308,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":0,"name":"t","options":["点按","长按"],"title":697,"type":"option"},
            
        ],
    },
    
    "SucroseTalent1": {
        name: "SucroseTalent1",
        nameLocale: 1333,
        
        description: 1336,
        
        
        badge: getImage("Sucrose"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "SucroseTalent2": {
        name: "SucroseTalent2",
        nameLocale: 1331,
        
        description: 1337,
        
        
        badge: getImage("Sucrose"),
        
        genre: "Character",
        config: [
            
            {"default":200.0,"name":"em","title":1338,"type":"floatInput"},
            
        ],
    },
    
    "SucroseC6": {
        name: "SucroseC6",
        nameLocale: 1332,
        
        description: 1335,
        
        
        badge: getImage("Sucrose"),
        
        genre: "Character",
        config: [
            
            {"default":"Electro","name":"element","title":680,"type":"element4"},
            
        ],
    },
    
    "ThomaTalent1": {
        name: "ThomaTalent1",
        nameLocale: 671,
        
        description: 674,
        
        
        badge: getImage("Tohma"),
        
        genre: "Character",
        config: [
            
            {"default":2.0,"max":5.0,"min":0.0,"name":"stack","title":374,"type":"float"},
            
        ],
    },
    
    "ThomaC6": {
        name: "ThomaC6",
        nameLocale: 670,
        
        description: 673,
        
        
        badge: getImage("Tohma"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "VentiC2": {
        name: "VentiC2",
        nameLocale: 1152,
        
        description: 1154,
        
        
        badge: getImage("Venti"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"levitating","title":1509,"type":"bool"},
            
        ],
    },
    
    "VentiC6": {
        name: "VentiC6",
        nameLocale: 1151,
        
        description: 1155,
        
        
        badge: getImage("Venti"),
        
        genre: "Character",
        config: [
            
            {"default":true,"name":"is_convert","title":358,"type":"bool"},
            
            {"default":"Electro","name":"element","title":1630,"type":"element4"},
            
        ],
    },
    
    "XianglingTalent2": {
        name: "XianglingTalent2",
        nameLocale: 1860,
        
        description: 1864,
        
        
        badge: getImage("Xiangling"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XianglingC1": {
        name: "XianglingC1",
        nameLocale: 1858,
        
        description: 1862,
        
        
        badge: getImage("Xiangling"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XianglingC6": {
        name: "XianglingC6",
        nameLocale: 1859,
        
        description: 1863,
        
        
        badge: getImage("Xiangling"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XingqiuC2": {
        name: "XingqiuC2",
        nameLocale: 1525,
        
        description: 1527,
        
        
        badge: getImage("Xingqiu"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XinyanC4": {
        name: "XinyanC4",
        nameLocale: 1636,
        
        description: 1640,
        
        
        badge: getImage("Xinyan"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XinyanTalent2": {
        name: "XinyanTalent2",
        nameLocale: 1637,
        
        description: 1641,
        
        
        badge: getImage("Xinyan"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "YaeMikoC4": {
        name: "YaeMikoC4",
        nameLocale: 246,
        
        description: 248,
        
        
        badge: getImage("Yae"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "YoimiyaTalent2": {
        name: "YoimiyaTalent2",
        nameLocale: 572,
        
        description: 574,
        
        
        badge: getImage("Yoimiya"),
        
        genre: "Character",
        config: [
            
            {"default":0,"max":10,"min":0,"name":"talent1_stack","title":46,"type":"int"},
            
        ],
    },
    
    "YunjinQ": {
        name: "YunjinQ",
        nameLocale: 142,
        
        description: 144,
        
        
        badge: getImage("Yunjin"),
        
        genre: "Character",
        config: [
            
            {"default":10,"max":15,"min":1,"name":"skill3","title":19,"type":"int"},
            
            {"default":2000.0,"name":"def","title":146,"type":"floatInput"},
            
            {"default":true,"name":"talent2","title":7,"type":"bool"},
            
            {"default":4,"max":4,"min":1,"name":"ele_count","title":1738,"type":"int"},
            
        ],
    },
    
    "YunjinC2": {
        name: "YunjinC2",
        nameLocale: 141,
        
        description: 145,
        
        
        badge: getImage("Yunjin"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "ZhongliShield": {
        name: "ZhongliShield",
        nameLocale: 1719,
        
        description: 1721,
        
        
        badge: getImage("Zhongli"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "YelanTalent2": {
        name: "YelanTalent2",
        nameLocale: 486,
        
        description: 490,
        
        
        badge: getImage("Yelan"),
        
        genre: "Character",
        config: [
            
            {"default":14,"max":14,"min":0,"name":"secs","title":1411,"type":"int"},
            
        ],
    },
    
    "YelanC4": {
        name: "YelanC4",
        nameLocale: 487,
        
        description: 489,
        
        
        badge: getImage("Yelan"),
        
        genre: "Character",
        config: [
            
            {"default":4,"max":4,"min":1,"name":"count","title":1059,"type":"int"},
            
        ],
    },
    
    "KamisatoAyatoQ": {
        name: "KamisatoAyatoQ",
        nameLocale: 1359,
        
        description: 1362,
        
        
        badge: getImage("Ayato"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"skill_level","title":1361,"type":"int"},
            
        ],
    },
    
    "ShikanoinHeizouTalent2": {
        name: "ShikanoinHeizouTalent2",
        nameLocale: 1884,
        
        description: 1886,
        
        
        badge: getImage("Heizo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "TighnariC4": {
        name: "TighnariC4",
        nameLocale: 722,
        
        description: 724,
        
        
        badge: getImage("Tighnari"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"after_reaction","title":1672,"type":"bool"},
            
        ],
    },
    
    "DoriC4": {
        name: "DoriC4",
        nameLocale: 483,
        
        description: 484,
        
        
        badge: getImage("Dori"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"hp_below50","title":1278,"type":"bool"},
            
            {"default":true,"name":"energy_below50","title":240,"type":"bool"},
            
        ],
    },
    
    "NilouTalent1": {
        name: "NilouTalent1",
        nameLocale: 538,
        
        description: 473,
        
        
        badge: getImage("Nilou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "NilouTalent2": {
        name: "NilouTalent2",
        nameLocale: 539,
        
        description: 463,
        
        
        badge: getImage("Nilou"),
        
        genre: "Character",
        config: [
            
            {"default":60000.0,"name":"hp","title":541,"type":"floatInput"},
            
        ],
    },
    
    "CandaceQ": {
        name: "CandaceQ",
        nameLocale: 451,
        
        description: 1567,
        
        
        badge: getImage("Candace"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "CandaceTalent2": {
        name: "CandaceTalent2",
        nameLocale: 450,
        
        description: 471,
        
        
        badge: getImage("Candace"),
        
        genre: "Character",
        config: [
            
            {"default":30000.0,"name":"hp","title":452,"type":"floatInput"},
            
        ],
    },
    
    "NahidaTalent1": {
        name: "NahidaTalent1",
        nameLocale: 1404,
        
        description: 801,
        
        
        badge: getImage("Nahida"),
        
        genre: "Character",
        config: [
            
            {"default":1000.0,"max":3000.0,"min":0.0,"name":"max_em","title":1747,"type":"float"},
            
        ],
    },
    
    "FaruzanQ": {
        name: "FaruzanQ",
        nameLocale: 1108,
        
        description: 1245,
        
        
        badge: getImage("Faruzan"),
        
        genre: "Character",
        config: [
            
            {"default":650,"max":1000,"min":0,"name":"base_atk","title":1109,"type":"int"},
            
            {"default":10,"max":15,"min":1,"name":"q_level","title":19,"type":"int"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate_q1","title":47,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate_q2","title":42,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate_talent2","title":21,"type":"float"},
            
            {"default":false,"name":"enable_c6","title":250,"type":"bool"},
            
        ],
    },
    
    "Mika": {
        name: "Mika",
        nameLocale: 1395,
        
        description: 831,
        
        
        badge: getImage("Mika"),
        
        genre: "Character",
        config: [
            
            {"default":3.0,"max":5.0,"min":0.0,"name":"stack_talent2","title":189,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate_c6","title":252,"type":"float"},
            
        ],
    },
    
    "KavehQ": {
        name: "KavehQ",
        nameLocale: 350,
        
        description: 1744,
        
        
        badge: getImage("Kaveh"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"q_level","title":351,"type":"int"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":1081,"type":"float"},
            
        ],
    },
    
    "BaizhuTalent2": {
        name: "BaizhuTalent2",
        nameLocale: 1313,
        
        description: 362,
        
        
        badge: getImage("Baizhuer"),
        
        genre: "Character",
        config: [
            
            {"default":50000.0,"max":50000.0,"min":0.0,"name":"hp","title":1315,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":1081,"type":"float"},
            
        ],
    },
    
    "BaizhuC4": {
        name: "BaizhuC4",
        nameLocale: 1314,
        
        description: 802,
        
        
        badge: getImage("Baizhuer"),
        
        genre: "Character",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":1081,"type":"float"},
            
        ],
    },
    
    "FreedomSworn": {
        name: "FreedomSworn",
        nameLocale: 1462,
        
        description: 23,
        
        
        badge: getImageW("Sword_Widsith"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1398,"type":"intInput"},
            
        ],
    },
    
    "SongOfBrokenPines": {
        name: "SongOfBrokenPines",
        nameLocale: 1041,
        
        description: 24,
        
        
        badge: getImageW("Claymore_Widsith"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1398,"type":"intInput"},
            
        ],
    },
    
    "WolfsGravestone": {
        name: "WolfsGravestone",
        nameLocale: 1221,
        
        description: 742,
        
        
        badge: getImageW("Claymore_Wolfmound"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1398,"type":"intInput"},
            
        ],
    },
    
    "ThrillingTalesOfDragonSlayers": {
        name: "ThrillingTalesOfDragonSlayers",
        nameLocale: 1581,
        
        description: 175,
        
        
        badge: getImageW("Catalyst_Pulpfic"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1398,"type":"intInput"},
            
        ],
    },
    
    "ElegyOfTheEnd": {
        name: "ElegyOfTheEnd",
        nameLocale: 1410,
        
        description: 341,
        
        
        badge: getImageW("Bow_Widsith"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1398,"type":"intInput"},
            
        ],
    },
    
    "HakushinRing": {
        name: "HakushinRing",
        nameLocale: 1320,
        
        description: 1068,
        
        
        badge: getImageW("Catalyst_Bakufu"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1398,"type":"intInput"},
            
            {"default":"Electro","name":"element","title":193,"type":"element8"},
            
        ],
    },
    
    "SapwoodBlade": {
        name: "SapwoodBlade",
        nameLocale: 356,
        
        description: 708,
        
        
        badge: getImageW("Sword_Arakalari"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1398,"type":"intInput"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":624,"type":"float"},
            
        ],
    },
    
    "Moonpiercer": {
        name: "Moonpiercer",
        nameLocale: 1601,
        
        description: 709,
        
        
        badge: getImageW("Pole_Arakalari"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1398,"type":"intInput"},
            
        ],
    },
    
    "XiphosMoonlight": {
        name: "XiphosMoonlight",
        nameLocale: 1552,
        
        description: 1546,
        
        
        badge: getImageW("Sword_Pleroma"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1398,"type":"intInput"},
            
            {"default":900.0,"name":"em","title":233,"type":"floatInput"},
            
        ],
    },
    
    "MakhairaAquamarine": {
        name: "MakhairaAquamarine",
        nameLocale: 1240,
        
        description: 33,
        
        
        badge: getImageW("Claymore_Pleroma"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1398,"type":"intInput"},
            
            {"default":900.0,"name":"em","title":233,"type":"floatInput"},
            
        ],
    },
    
    "KeyOfKhajNisut": {
        name: "KeyOfKhajNisut",
        nameLocale: 446,
        
        description: 588,
        
        
        badge: getImageW("Sword_Deshret"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1398,"type":"intInput"},
            
            {"default":20000.0,"name":"hp","title":1272,"type":"floatInput"},
            
        ],
    },
    
    "ResonancePyro2": {
        name: "ResonancePyro2",
        nameLocale: 201,
        
        description: 731,
        
        
        badge: ResonancePyro2_image,
        
        genre: "Resonance",
        config: [
            
        ],
    },
    
    "ResonanceCryo2": {
        name: "ResonanceCryo2",
        nameLocale: 202,
        
        description: 726,
        
        
        badge: ResonanceCryo2_image,
        
        genre: "Resonance",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":624,"type":"float"},
            
        ],
    },
    
    "ResonanceGeo2": {
        name: "ResonanceGeo2",
        nameLocale: 199,
        
        description: 701,
        
        
        badge: ResonanceGeo2_image,
        
        genre: "Resonance",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate1","title":753,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate2","title":755,"type":"float"},
            
        ],
    },
    
    "ResonanceHydro2": {
        name: "ResonanceHydro2",
        nameLocale: 200,
        
        description: 1173,
        
        
        badge: ResonanceHydro2_image,
        
        genre: "Resonance",
        config: [
            
        ],
    },
    
    "ResonanceDendro2": {
        name: "ResonanceDendro2",
        nameLocale: 203,
        
        description: 234,
        
        
        badge: ResonanceDendro2_image,
        
        genre: "Resonance",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate1","title":753,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate2","title":755,"type":"float"},
            
        ],
    },
    
    "Instructor4": {
        name: "Instructor4",
        nameLocale: 762,
        
        description: 1572,
        
        
        badge: getImageA("UI_RelicIcon_10007_4"),
        
        genre: "Artifact",
        config: [
            
        ],
    },
    
    "NoblesseOblige4": {
        name: "NoblesseOblige4",
        nameLocale: 823,
        
        description: 797,
        
        
        badge: getImageA("UI_RelicIcon_15007_4"),
        
        genre: "Artifact",
        config: [
            
        ],
    },
    
    "ArchaicPetra4": {
        name: "ArchaicPetra4",
        nameLocale: 654,
        
        description: 1502,
        
        
        badge: getImageA("UI_RelicIcon_15014_4"),
        
        genre: "Artifact",
        config: [
            
            {"default":"Electro","name":"element","title":1413,"type":"element4"},
            
        ],
    },
    
    "ViridescentVenerer4": {
        name: "ViridescentVenerer4",
        nameLocale: 1433,
        
        description: 1060,
        
        
        badge: getImageA("UI_RelicIcon_15002_4"),
        
        genre: "Artifact",
        config: [
            
            {"default":"Electro","name":"element","title":675,"type":"element4"},
            
        ],
    },
    
    "TenacityOfTheMillelith4": {
        name: "TenacityOfTheMillelith4",
        nameLocale: 339,
        
        description: 209,
        
        
        badge: getImageA("UI_RelicIcon_15017_4"),
        
        genre: "Artifact",
        config: [
            
        ],
    },
    
    "DeepwoodMemories4": {
        name: "DeepwoodMemories4",
        nameLocale: 1141,
        
        description: 215,
        
        
        badge: getImageA("UI_RelicIcon_15025_4"),
        
        genre: "Artifact",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":624,"type":"float"},
            
        ],
    },
    
}