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
        nameLocale: 706,
        
        description: null,
        
        
        badge: ATKPercentage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":745,"type":"floatPercentageInput"},
            
        ],
    },
    
    "DEFPercentage": {
        name: "DEFPercentage",
        nameLocale: 1689,
        
        description: null,
        
        
        badge: DEFPercentage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":745,"type":"floatPercentageInput"},
            
        ],
    },
    
    "HPPercentage": {
        name: "HPPercentage",
        nameLocale: 1231,
        
        description: null,
        
        
        badge: HPPercentage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":745,"type":"floatPercentageInput"},
            
        ],
    },
    
    "ATKFixed": {
        name: "ATKFixed",
        nameLocale: 705,
        
        description: null,
        
        
        badge: ATKFixed_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":745,"type":"floatInput"},
            
        ],
    },
    
    "DEFFixed": {
        name: "DEFFixed",
        nameLocale: 1688,
        
        description: null,
        
        
        badge: DEFFixed_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":745,"type":"floatInput"},
            
        ],
    },
    
    "HPFixed": {
        name: "HPFixed",
        nameLocale: 1230,
        
        description: null,
        
        
        badge: HPFixed_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":745,"type":"floatInput"},
            
        ],
    },
    
    "Critical": {
        name: "Critical",
        nameLocale: 969,
        
        description: null,
        
        
        badge: Critical_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":745,"type":"floatPercentageInput"},
            
        ],
    },
    
    "CriticalDamage": {
        name: "CriticalDamage",
        nameLocale: 965,
        
        description: null,
        
        
        badge: CriticalDamage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":745,"type":"floatPercentageInput"},
            
        ],
    },
    
    "CustomBonus": {
        name: "CustomBonus",
        nameLocale: 169,
        
        description: null,
        
        
        badge: CustomBonus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":745,"type":"floatPercentageInput"},
            
        ],
    },
    
    "ElementalMastery": {
        name: "ElementalMastery",
        nameLocale: 223,
        
        description: null,
        
        
        badge: ElementalMastery_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":745,"type":"floatInput"},
            
        ],
    },
    
    "Recharge": {
        name: "Recharge",
        nameLocale: 187,
        
        description: null,
        
        
        badge: Recharge_image,
        
        genre: "Common",
        config: [
            
            {"default":20.0,"name":"p","title":745,"type":"floatPercentageInput"},
            
        ],
    },
    
    "DEFMinus": {
        name: "DEFMinus",
        nameLocale: 277,
        
        description: 0,
        
        
        badge: DEFMinus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":745,"type":"floatPercentageInput"},
            
        ],
    },
    
    "ResMinus": {
        name: "ResMinus",
        nameLocale: 276,
        
        description: null,
        
        
        badge: ResMinus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":745,"type":"floatPercentageInput"},
            
        ],
    },
    
    "HealingBonus": {
        name: "HealingBonus",
        nameLocale: 1067,
        
        description: null,
        
        
        badge: HealingBonus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":745,"type":"floatPercentageInput"},
            
        ],
    },
    
    "BaseDmg": {
        name: "BaseDmg",
        nameLocale: 448,
        
        description: 1569,
        
        
        badge: BaseDmg_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":745,"type":"floatInput"},
            
        ],
    },
    
    "AlbedoTalent2": {
        name: "AlbedoTalent2",
        nameLocale: 1701,
        
        description: 1706,
        
        
        badge: getImage("Albedo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AlbedoC4": {
        name: "AlbedoC4",
        nameLocale: 1702,
        
        description: 1704,
        
        
        badge: getImage("Albedo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AlbedoC6": {
        name: "AlbedoC6",
        nameLocale: 1700,
        
        description: 1705,
        
        
        badge: getImage("Albedo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AloyTalent1": {
        name: "AloyTalent1",
        nameLocale: 443,
        
        description: 444,
        
        
        badge: getImage("Aloy"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AratakiIttoC4": {
        name: "AratakiIttoC4",
        nameLocale: 1423,
        
        description: 1425,
        
        
        badge: getImage("Itto"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "BeidouC6": {
        name: "BeidouC6",
        nameLocale: 319,
        
        description: 321,
        
        
        badge: getImage("Beidou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "BennettQ": {
        name: "BennettQ",
        nameLocale: 1206,
        
        description: 1209,
        
        
        badge: getImage("Bennett"),
        
        genre: "Character",
        config: [
            
            {"default":800.0,"name":"base_atk","title":1211,"type":"floatInput"},
            
            {"default":true,"name":"c1","title":808,"type":"bool"},
            
            {"default":10,"max":15,"min":1,"name":"skill3","title":674,"type":"int"},
            
        ],
    },
    
    "BennettC6": {
        name: "BennettC6",
        nameLocale: 1205,
        
        description: 1210,
        
        
        badge: getImage("Bennett"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "ChongyunTalent2": {
        name: "ChongyunTalent2",
        nameLocale: 1620,
        
        description: 1622,
        
        
        badge: getImage("Chongyun"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "DionaC6G50": {
        name: "DionaC6G50",
        nameLocale: 1597,
        
        description: 1599,
        
        
        badge: getImage("Diona"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "EulaE": {
        name: "EulaE",
        nameLocale: 165,
        
        description: 167,
        
        
        badge: getImage("Eula"),
        
        genre: "Character",
        config: [
            
            {"default":9,"max":15,"min":1,"name":"skill2","title":674,"type":"int"},
            
        ],
    },
    
    "GanyuTalent2": {
        name: "GanyuTalent2",
        nameLocale: 1225,
        
        description: 1229,
        
        
        badge: getImage("Ganyu"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GanyuC1": {
        name: "GanyuC1",
        nameLocale: 1226,
        
        description: 1228,
        
        
        badge: getImage("Ganyu"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GorouE1": {
        name: "GorouE1",
        nameLocale: 147,
        
        description: 152,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
            {"default":10,"max":15,"min":1,"name":"skill2","title":151,"type":"int"},
            
        ],
    },
    
    "GorouE3": {
        name: "GorouE3",
        nameLocale: 148,
        
        description: 153,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GorouTalent1": {
        name: "GorouTalent1",
        nameLocale: 146,
        
        description: 155,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GorouC6": {
        name: "GorouC6",
        nameLocale: 149,
        
        description: 154,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
            {"default":1,"max":3,"min":1,"name":"level","title":1754,"type":"int"},
            
        ],
    },
    
    "HuTaoTalent1": {
        name: "HuTaoTalent1",
        nameLocale: 1401,
        
        description: 1403,
        
        
        badge: getImage("Hutao"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "JeanC4": {
        name: "JeanC4",
        nameLocale: 1218,
        
        description: 1220,
        
        
        badge: getImage("Qin"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KaedeharaKazuhaTalent2": {
        name: "KaedeharaKazuhaTalent2",
        nameLocale: 1014,
        
        description: 1018,
        
        
        badge: getImage("Kazuha"),
        
        genre: "Character",
        config: [
            
            {"default":"Electro","name":"element","title":653,"type":"element4"},
            
            {"default":800.0,"name":"em","title":68,"type":"floatInput"},
            
        ],
    },
    
    "KaedeharaKazuhaC2": {
        name: "KaedeharaKazuhaC2",
        nameLocale: 1013,
        
        description: 1017,
        
        
        badge: getImage("Kazuha"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KamisatoAyakaC4": {
        name: "KamisatoAyakaC4",
        nameLocale: 1320,
        
        description: 1370,
        
        
        badge: getImage("Ayaka"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KleeC2": {
        name: "KleeC2",
        nameLocale: 366,
        
        description: 368,
        
        
        badge: getImage("Klee"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KleeC6": {
        name: "KleeC6",
        nameLocale: 365,
        
        description: 369,
        
        
        badge: getImage("Klee"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KujouSaraEOrQ": {
        name: "KujouSaraEOrQ",
        nameLocale: 105,
        
        description: 108,
        
        
        badge: getImage("Sara"),
        
        genre: "Character",
        config: [
            
            {"default":700.0,"name":"base_atk","title":109,"type":"floatInput"},
            
            {"default":false,"name":"c6","title":811,"type":"bool"},
            
            {"default":10,"max":15,"min":1,"name":"skill2","title":12,"type":"int"},
            
        ],
    },
    
    "LisaTalent2": {
        name: "LisaTalent2",
        nameLocale: 96,
        
        description: 98,
        
        
        badge: getImage("Lisa"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "MonaQ": {
        name: "MonaQ",
        nameLocale: 1431,
        
        description: 1434,
        
        
        badge: getImage("Mona"),
        
        genre: "Character",
        config: [
            
            {"default":9,"max":15,"min":1,"name":"skill3","title":18,"type":"int"},
            
            {"default":false,"name":"c4","title":810,"type":"bool"},
            
        ],
    },
    
    "MonaC1": {
        name: "MonaC1",
        nameLocale: 1432,
        
        description: 1435,
        
        
        badge: getImage("Mona"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "NingguangTalent2": {
        name: "NingguangTalent2",
        nameLocale: 280,
        
        description: 282,
        
        
        badge: getImage("Ningguang"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "RaidenShogunE": {
        name: "RaidenShogunE",
        nameLocale: 1735,
        
        description: 1738,
        
        
        badge: getImage("Shougun"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"skill2","title":1737,"type":"int"},
            
            {"default":80,"max":100,"min":20,"name":"energy","title":348,"type":"int"},
            
        ],
    },
    
    "RaidenShogunC4": {
        name: "RaidenShogunC4",
        nameLocale: 1734,
        
        description: 1739,
        
        
        badge: getImage("Shougun"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "RazorC4": {
        name: "RazorC4",
        nameLocale: 1728,
        
        description: 1730,
        
        
        badge: getImage("Razor"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "RosariaTalent2": {
        name: "RosariaTalent2",
        nameLocale: 1378,
        
        description: 1381,
        
        
        badge: getImage("Rosaria"),
        
        genre: "Character",
        config: [
            
            {"default":70.0,"name":"crit","title":1382,"type":"floatPercentageInput"},
            
        ],
    },
    
    "RosariaC6": {
        name: "RosariaC6",
        nameLocale: 1377,
        
        description: 1380,
        
        
        badge: getImage("Rosaria"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "ShenheE": {
        name: "ShenheE",
        nameLocale: 1255,
        
        description: 1261,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":3000.0,"name":"atk","title":1266,"type":"floatInput"},
            
            {"default":8,"max":15,"min":1,"name":"skill2","title":1260,"type":"int"},
            
        ],
    },
    
    "ShenheQ": {
        name: "ShenheQ",
        nameLocale: 1257,
        
        description: 1263,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"skill3","title":1262,"type":"int"},
            
        ],
    },
    
    "ShenheTalent1": {
        name: "ShenheTalent1",
        nameLocale: 1256,
        
        description: 1264,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"c2","title":809,"type":"bool"},
            
        ],
    },
    
    "ShenheTalent2": {
        name: "ShenheTalent2",
        nameLocale: 1258,
        
        description: 1265,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":0,"name":"t","options":["点按","长按"],"title":675,"type":"option"},
            
        ],
    },
    
    "SucroseTalent1": {
        name: "SucroseTalent1",
        nameLocale: 1288,
        
        description: 1291,
        
        
        badge: getImage("Sucrose"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "SucroseTalent2": {
        name: "SucroseTalent2",
        nameLocale: 1286,
        
        description: 1292,
        
        
        badge: getImage("Sucrose"),
        
        genre: "Character",
        config: [
            
            {"default":200.0,"name":"em","title":1293,"type":"floatInput"},
            
        ],
    },
    
    "SucroseC6": {
        name: "SucroseC6",
        nameLocale: 1287,
        
        description: 1290,
        
        
        badge: getImage("Sucrose"),
        
        genre: "Character",
        config: [
            
            {"default":"Electro","name":"element","title":658,"type":"element4"},
            
        ],
    },
    
    "ThomaTalent1": {
        name: "ThomaTalent1",
        nameLocale: 649,
        
        description: 652,
        
        
        badge: getImage("Tohma"),
        
        genre: "Character",
        config: [
            
            {"default":2.0,"max":5.0,"min":0.0,"name":"stack","title":359,"type":"float"},
            
        ],
    },
    
    "ThomaC6": {
        name: "ThomaC6",
        nameLocale: 648,
        
        description: 651,
        
        
        badge: getImage("Tohma"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "VentiC2": {
        name: "VentiC2",
        nameLocale: 1114,
        
        description: 1116,
        
        
        badge: getImage("Venti"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"levitating","title":1459,"type":"bool"},
            
        ],
    },
    
    "VentiC6": {
        name: "VentiC6",
        nameLocale: 1113,
        
        description: 1117,
        
        
        badge: getImage("Venti"),
        
        genre: "Character",
        config: [
            
            {"default":true,"name":"is_convert","title":347,"type":"bool"},
            
            {"default":"Electro","name":"element","title":1574,"type":"element4"},
            
        ],
    },
    
    "XianglingTalent2": {
        name: "XianglingTalent2",
        nameLocale: 1792,
        
        description: 1796,
        
        
        badge: getImage("Xiangling"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XianglingC1": {
        name: "XianglingC1",
        nameLocale: 1790,
        
        description: 1794,
        
        
        badge: getImage("Xiangling"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XianglingC6": {
        name: "XianglingC6",
        nameLocale: 1791,
        
        description: 1795,
        
        
        badge: getImage("Xiangling"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XingqiuC2": {
        name: "XingqiuC2",
        nameLocale: 1475,
        
        description: 1477,
        
        
        badge: getImage("Xingqiu"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XinyanC4": {
        name: "XinyanC4",
        nameLocale: 1580,
        
        description: 1584,
        
        
        badge: getImage("Xinyan"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XinyanTalent2": {
        name: "XinyanTalent2",
        nameLocale: 1581,
        
        description: 1585,
        
        
        badge: getImage("Xinyan"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "YaeMikoC4": {
        name: "YaeMikoC4",
        nameLocale: 237,
        
        description: 239,
        
        
        badge: getImage("Yae"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "YoimiyaTalent2": {
        name: "YoimiyaTalent2",
        nameLocale: 553,
        
        description: 555,
        
        
        badge: getImage("Yoimiya"),
        
        genre: "Character",
        config: [
            
            {"default":0,"max":10,"min":0,"name":"talent1_stack","title":41,"type":"int"},
            
        ],
    },
    
    "YunjinQ": {
        name: "YunjinQ",
        nameLocale: 135,
        
        description: 137,
        
        
        badge: getImage("Yunjin"),
        
        genre: "Character",
        config: [
            
            {"default":10,"max":15,"min":1,"name":"skill3","title":18,"type":"int"},
            
            {"default":2000.0,"name":"def","title":139,"type":"floatInput"},
            
            {"default":true,"name":"talent2","title":7,"type":"bool"},
            
            {"default":4,"max":4,"min":1,"name":"ele_count","title":1673,"type":"int"},
            
        ],
    },
    
    "YunjinC2": {
        name: "YunjinC2",
        nameLocale: 134,
        
        description: 138,
        
        
        badge: getImage("Yunjin"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "ZhongliShield": {
        name: "ZhongliShield",
        nameLocale: 1655,
        
        description: 1657,
        
        
        badge: getImage("Zhongli"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "YelanTalent2": {
        name: "YelanTalent2",
        nameLocale: 468,
        
        description: 472,
        
        
        badge: getImage("Yelan"),
        
        genre: "Character",
        config: [
            
            {"default":14,"max":14,"min":0,"name":"secs","title":1365,"type":"int"},
            
        ],
    },
    
    "YelanC4": {
        name: "YelanC4",
        nameLocale: 469,
        
        description: 471,
        
        
        badge: getImage("Yelan"),
        
        genre: "Character",
        config: [
            
            {"default":4,"max":4,"min":1,"name":"count","title":1025,"type":"int"},
            
        ],
    },
    
    "KamisatoAyatoQ": {
        name: "KamisatoAyatoQ",
        nameLocale: 1314,
        
        description: 1317,
        
        
        badge: getImage("Ayato"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"skill_level","title":1316,"type":"int"},
            
        ],
    },
    
    "ShikanoinHeizouTalent2": {
        name: "ShikanoinHeizouTalent2",
        nameLocale: 1816,
        
        description: 1818,
        
        
        badge: getImage("Heizo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "TighnariC4": {
        name: "TighnariC4",
        nameLocale: 700,
        
        description: 702,
        
        
        badge: getImage("Tighnari"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"after_reaction","title":1615,"type":"bool"},
            
        ],
    },
    
    "DoriC4": {
        name: "DoriC4",
        nameLocale: 465,
        
        description: 466,
        
        
        badge: getImage("Dori"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"hp_below50","title":1236,"type":"bool"},
            
            {"default":true,"name":"energy_below50","title":231,"type":"bool"},
            
        ],
    },
    
    "NilouTalent1": {
        name: "NilouTalent1",
        nameLocale: 520,
        
        description: 456,
        
        
        badge: getImage("Nilou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "NilouTalent2": {
        name: "NilouTalent2",
        nameLocale: 521,
        
        description: 446,
        
        
        badge: getImage("Nilou"),
        
        genre: "Character",
        config: [
            
            {"default":60000.0,"name":"hp","title":523,"type":"floatInput"},
            
        ],
    },
    
    "CandaceQ": {
        name: "CandaceQ",
        nameLocale: 435,
        
        description: 1511,
        
        
        badge: getImage("Candace"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "CandaceTalent2": {
        name: "CandaceTalent2",
        nameLocale: 434,
        
        description: 454,
        
        
        badge: getImage("Candace"),
        
        genre: "Character",
        config: [
            
            {"default":30000.0,"name":"hp","title":436,"type":"floatInput"},
            
        ],
    },
    
    "NahidaTalent1": {
        name: "NahidaTalent1",
        nameLocale: 1358,
        
        description: 777,
        
        
        badge: getImage("Nahida"),
        
        genre: "Character",
        config: [
            
            {"default":1000.0,"max":3000.0,"min":0.0,"name":"max_em","title":1682,"type":"float"},
            
        ],
    },
    
    "FaruzanQ": {
        name: "FaruzanQ",
        nameLocale: 1072,
        
        description: 1203,
        
        
        badge: getImage("Faruzan"),
        
        genre: "Character",
        config: [
            
            {"default":650,"max":1000,"min":0,"name":"base_atk","title":1073,"type":"int"},
            
            {"default":10,"max":15,"min":1,"name":"q_level","title":18,"type":"int"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate_q1","title":42,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate_q2","title":39,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate_talent2","title":20,"type":"float"},
            
            {"default":false,"name":"enable_c6","title":241,"type":"bool"},
            
        ],
    },
    
    "Mika": {
        name: "Mika",
        nameLocale: 1350,
        
        description: 805,
        
        
        badge: getImage("Mika"),
        
        genre: "Character",
        config: [
            
            {"default":3.0,"max":5.0,"min":0.0,"name":"stack_talent2","title":182,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate_c6","title":243,"type":"float"},
            
        ],
    },
    
    "KavehQ": {
        name: "KavehQ",
        nameLocale: 339,
        
        description: 1679,
        
        
        badge: getImage("Kaveh"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"q_level","title":340,"type":"int"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":1047,"type":"float"},
            
        ],
    },
    
    "BaizhuTalent2": {
        name: "BaizhuTalent2",
        nameLocale: 1270,
        
        description: 350,
        
        
        badge: getImage("Baizhuer"),
        
        genre: "Character",
        config: [
            
            {"default":50000.0,"max":50000.0,"min":0.0,"name":"hp","title":1272,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":1047,"type":"float"},
            
        ],
    },
    
    "BaizhuC4": {
        name: "BaizhuC4",
        nameLocale: 1271,
        
        description: 778,
        
        
        badge: getImage("Baizhuer"),
        
        genre: "Character",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":1047,"type":"float"},
            
        ],
    },
    
    "FreedomSworn": {
        name: "FreedomSworn",
        nameLocale: 1414,
        
        description: 22,
        
        
        badge: getImageW("Sword_Widsith"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1353,"type":"intInput"},
            
        ],
    },
    
    "SongOfBrokenPines": {
        name: "SongOfBrokenPines",
        nameLocale: 1008,
        
        description: 23,
        
        
        badge: getImageW("Claymore_Widsith"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1353,"type":"intInput"},
            
        ],
    },
    
    "WolfsGravestone": {
        name: "WolfsGravestone",
        nameLocale: 1180,
        
        description: 719,
        
        
        badge: getImageW("Claymore_Wolfmound"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1353,"type":"intInput"},
            
        ],
    },
    
    "ThrillingTalesOfDragonSlayers": {
        name: "ThrillingTalesOfDragonSlayers",
        nameLocale: 1525,
        
        description: 168,
        
        
        badge: getImageW("Catalyst_Pulpfic"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1353,"type":"intInput"},
            
        ],
    },
    
    "ElegyOfTheEnd": {
        name: "ElegyOfTheEnd",
        nameLocale: 1364,
        
        description: 330,
        
        
        badge: getImageW("Bow_Widsith"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1353,"type":"intInput"},
            
        ],
    },
    
    "HakushinRing": {
        name: "HakushinRing",
        nameLocale: 1277,
        
        description: 1034,
        
        
        badge: getImageW("Catalyst_Bakufu"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1353,"type":"intInput"},
            
            {"default":"Electro","name":"element","title":185,"type":"element8"},
            
        ],
    },
    
    "SapwoodBlade": {
        name: "SapwoodBlade",
        nameLocale: 345,
        
        description: 686,
        
        
        badge: getImageW("Sword_Arakalari"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1353,"type":"intInput"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":604,"type":"float"},
            
        ],
    },
    
    "Moonpiercer": {
        name: "Moonpiercer",
        nameLocale: 1545,
        
        description: 687,
        
        
        badge: getImageW("Pole_Arakalari"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1353,"type":"intInput"},
            
        ],
    },
    
    "XiphosMoonlight": {
        name: "XiphosMoonlight",
        nameLocale: 1496,
        
        description: 1490,
        
        
        badge: getImageW("Sword_Pleroma"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1353,"type":"intInput"},
            
            {"default":900.0,"name":"em","title":224,"type":"floatInput"},
            
        ],
    },
    
    "MakhairaAquamarine": {
        name: "MakhairaAquamarine",
        nameLocale: 1198,
        
        description: 31,
        
        
        badge: getImageW("Claymore_Pleroma"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1353,"type":"intInput"},
            
            {"default":900.0,"name":"em","title":224,"type":"floatInput"},
            
        ],
    },
    
    "KeyOfKhajNisut": {
        name: "KeyOfKhajNisut",
        nameLocale: 430,
        
        description: 569,
        
        
        badge: getImageW("Sword_Deshret"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1353,"type":"intInput"},
            
            {"default":20000.0,"name":"hp","title":1230,"type":"floatInput"},
            
        ],
    },
    
    "ResonancePyro2": {
        name: "ResonancePyro2",
        nameLocale: 193,
        
        description: 709,
        
        
        badge: ResonancePyro2_image,
        
        genre: "Resonance",
        config: [
            
        ],
    },
    
    "ResonanceCryo2": {
        name: "ResonanceCryo2",
        nameLocale: 194,
        
        description: 704,
        
        
        badge: ResonanceCryo2_image,
        
        genre: "Resonance",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":604,"type":"float"},
            
        ],
    },
    
    "ResonanceGeo2": {
        name: "ResonanceGeo2",
        nameLocale: 191,
        
        description: 679,
        
        
        badge: ResonanceGeo2_image,
        
        genre: "Resonance",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate1","title":730,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate2","title":732,"type":"float"},
            
        ],
    },
    
    "ResonanceHydro2": {
        name: "ResonanceHydro2",
        nameLocale: 192,
        
        description: 1134,
        
        
        badge: ResonanceHydro2_image,
        
        genre: "Resonance",
        config: [
            
        ],
    },
    
    "ResonanceDendro2": {
        name: "ResonanceDendro2",
        nameLocale: 195,
        
        description: 225,
        
        
        badge: ResonanceDendro2_image,
        
        genre: "Resonance",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate1","title":730,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate2","title":732,"type":"float"},
            
        ],
    },
    
    "Instructor4": {
        name: "Instructor4",
        nameLocale: 739,
        
        description: 1516,
        
        
        badge: getImageA("UI_RelicIcon_10007_4"),
        
        genre: "Artifact",
        config: [
            
        ],
    },
    
    "NoblesseOblige4": {
        name: "NoblesseOblige4",
        nameLocale: 797,
        
        description: 773,
        
        
        badge: getImageA("UI_RelicIcon_15007_4"),
        
        genre: "Artifact",
        config: [
            
        ],
    },
    
    "ArchaicPetra4": {
        name: "ArchaicPetra4",
        nameLocale: 633,
        
        description: 1452,
        
        
        badge: getImageA("UI_RelicIcon_15014_4"),
        
        genre: "Artifact",
        config: [
            
            {"default":"Electro","name":"element","title":1367,"type":"element4"},
            
        ],
    },
    
    "ViridescentVenerer4": {
        name: "ViridescentVenerer4",
        nameLocale: 1387,
        
        description: 1026,
        
        
        badge: getImageA("UI_RelicIcon_15002_4"),
        
        genre: "Artifact",
        config: [
            
            {"default":"Electro","name":"element","title":653,"type":"element4"},
            
        ],
    },
    
    "TenacityOfTheMillelith4": {
        name: "TenacityOfTheMillelith4",
        nameLocale: 328,
        
        description: 201,
        
        
        badge: getImageA("UI_RelicIcon_15017_4"),
        
        genre: "Artifact",
        config: [
            
        ],
    },
    
    "DeepwoodMemories4": {
        name: "DeepwoodMemories4",
        nameLocale: 1103,
        
        description: 208,
        
        
        badge: getImageA("UI_RelicIcon_15025_4"),
        
        genre: "Artifact",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":604,"type":"float"},
            
        ],
    },
    
}