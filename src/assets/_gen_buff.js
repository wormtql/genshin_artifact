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
        nameLocale: 737,
        
        description: null,
        
        
        badge: ATKPercentage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":777,"type":"floatPercentageInput"},
            
        ],
    },
    
    "DEFPercentage": {
        name: "DEFPercentage",
        nameLocale: 1773,
        
        description: null,
        
        
        badge: DEFPercentage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":777,"type":"floatPercentageInput"},
            
        ],
    },
    
    "HPPercentage": {
        name: "HPPercentage",
        nameLocale: 1287,
        
        description: null,
        
        
        badge: HPPercentage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":777,"type":"floatPercentageInput"},
            
        ],
    },
    
    "ATKFixed": {
        name: "ATKFixed",
        nameLocale: 736,
        
        description: null,
        
        
        badge: ATKFixed_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":777,"type":"floatInput"},
            
        ],
    },
    
    "DEFFixed": {
        name: "DEFFixed",
        nameLocale: 1772,
        
        description: null,
        
        
        badge: DEFFixed_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":777,"type":"floatInput"},
            
        ],
    },
    
    "HPFixed": {
        name: "HPFixed",
        nameLocale: 1286,
        
        description: null,
        
        
        badge: HPFixed_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":777,"type":"floatInput"},
            
        ],
    },
    
    "Critical": {
        name: "Critical",
        nameLocale: 1010,
        
        description: null,
        
        
        badge: Critical_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":777,"type":"floatPercentageInput"},
            
        ],
    },
    
    "CriticalDamage": {
        name: "CriticalDamage",
        nameLocale: 1006,
        
        description: null,
        
        
        badge: CriticalDamage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":777,"type":"floatPercentageInput"},
            
        ],
    },
    
    "CustomBonus": {
        name: "CustomBonus",
        nameLocale: 184,
        
        description: null,
        
        
        badge: CustomBonus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":777,"type":"floatPercentageInput"},
            
        ],
    },
    
    "ElementalMastery": {
        name: "ElementalMastery",
        nameLocale: 240,
        
        description: null,
        
        
        badge: ElementalMastery_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":777,"type":"floatInput"},
            
        ],
    },
    
    "Recharge": {
        name: "Recharge",
        nameLocale: 202,
        
        description: null,
        
        
        badge: Recharge_image,
        
        genre: "Common",
        config: [
            
            {"default":20.0,"name":"p","title":777,"type":"floatPercentageInput"},
            
        ],
    },
    
    "DEFMinus": {
        name: "DEFMinus",
        nameLocale: 295,
        
        description: 0,
        
        
        badge: DEFMinus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":777,"type":"floatPercentageInput"},
            
        ],
    },
    
    "ResMinus": {
        name: "ResMinus",
        nameLocale: 294,
        
        description: null,
        
        
        badge: ResMinus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":777,"type":"floatPercentageInput"},
            
        ],
    },
    
    "HealingBonus": {
        name: "HealingBonus",
        nameLocale: 1115,
        
        description: null,
        
        
        badge: HealingBonus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":777,"type":"floatPercentageInput"},
            
        ],
    },
    
    "BaseDmg": {
        name: "BaseDmg",
        nameLocale: 474,
        
        description: 1644,
        
        
        badge: BaseDmg_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":777,"type":"floatInput"},
            
        ],
    },
    
    "AlbedoTalent2": {
        name: "AlbedoTalent2",
        nameLocale: 1785,
        
        description: 1790,
        
        
        badge: getImage("Albedo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AlbedoC4": {
        name: "AlbedoC4",
        nameLocale: 1786,
        
        description: 1788,
        
        
        badge: getImage("Albedo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AlbedoC6": {
        name: "AlbedoC6",
        nameLocale: 1784,
        
        description: 1789,
        
        
        badge: getImage("Albedo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AloyTalent1": {
        name: "AloyTalent1",
        nameLocale: 468,
        
        description: 469,
        
        
        badge: getImage("Aloy"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AratakiIttoC4": {
        name: "AratakiIttoC4",
        nameLocale: 1489,
        
        description: 1491,
        
        
        badge: getImage("Itto"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "BeidouC6": {
        name: "BeidouC6",
        nameLocale: 338,
        
        description: 340,
        
        
        badge: getImage("Beidou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "BennettQ": {
        name: "BennettQ",
        nameLocale: 1262,
        
        description: 1265,
        
        
        badge: getImage("Bennett"),
        
        genre: "Character",
        config: [
            
            {"default":800.0,"name":"base_atk","title":1267,"type":"floatInput"},
            
            {"default":true,"name":"c1","title":843,"type":"bool"},
            
            {"default":10,"max":15,"min":1,"name":"skill3","title":705,"type":"int"},
            
        ],
    },
    
    "BennettC6": {
        name: "BennettC6",
        nameLocale: 1261,
        
        description: 1266,
        
        
        badge: getImage("Bennett"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "ChongyunTalent2": {
        name: "ChongyunTalent2",
        nameLocale: 1700,
        
        description: 1702,
        
        
        badge: getImage("Chongyun"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "DionaC6G50": {
        name: "DionaC6G50",
        nameLocale: 1672,
        
        description: 1674,
        
        
        badge: getImage("Diona"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "EulaE": {
        name: "EulaE",
        nameLocale: 180,
        
        description: 182,
        
        
        badge: getImage("Eula"),
        
        genre: "Character",
        config: [
            
            {"default":9,"max":15,"min":1,"name":"skill2","title":705,"type":"int"},
            
        ],
    },
    
    "GanyuTalent2": {
        name: "GanyuTalent2",
        nameLocale: 1281,
        
        description: 1285,
        
        
        badge: getImage("Ganyu"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GanyuC1": {
        name: "GanyuC1",
        nameLocale: 1282,
        
        description: 1284,
        
        
        badge: getImage("Ganyu"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GorouE1": {
        name: "GorouE1",
        nameLocale: 161,
        
        description: 166,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
            {"default":10,"max":15,"min":1,"name":"skill2","title":165,"type":"int"},
            
        ],
    },
    
    "GorouE3": {
        name: "GorouE3",
        nameLocale: 162,
        
        description: 167,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GorouTalent1": {
        name: "GorouTalent1",
        nameLocale: 160,
        
        description: 169,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GorouC6": {
        name: "GorouC6",
        nameLocale: 163,
        
        description: 168,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
            {"default":1,"max":3,"min":1,"name":"level","title":1841,"type":"int"},
            
        ],
    },
    
    "HuTaoTalent1": {
        name: "HuTaoTalent1",
        nameLocale: 1463,
        
        description: 1465,
        
        
        badge: getImage("Hutao"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "JeanC4": {
        name: "JeanC4",
        nameLocale: 1274,
        
        description: 1276,
        
        
        badge: getImage("Qin"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KaedeharaKazuhaTalent2": {
        name: "KaedeharaKazuhaTalent2",
        nameLocale: 1060,
        
        description: 1064,
        
        
        badge: getImage("Kazuha"),
        
        genre: "Character",
        config: [
            
            {"default":"Electro","name":"element","title":684,"type":"element4"},
            
            {"default":800.0,"name":"em","title":81,"type":"floatInput"},
            
        ],
    },
    
    "KaedeharaKazuhaC2": {
        name: "KaedeharaKazuhaC2",
        nameLocale: 1059,
        
        description: 1063,
        
        
        badge: getImage("Kazuha"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KamisatoAyakaC4": {
        name: "KamisatoAyakaC4",
        nameLocale: 1380,
        
        description: 1431,
        
        
        badge: getImage("Ayaka"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KleeC2": {
        name: "KleeC2",
        nameLocale: 389,
        
        description: 391,
        
        
        badge: getImage("Klee"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KleeC6": {
        name: "KleeC6",
        nameLocale: 388,
        
        description: 392,
        
        
        badge: getImage("Klee"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KujouSaraEOrQ": {
        name: "KujouSaraEOrQ",
        nameLocale: 119,
        
        description: 122,
        
        
        badge: getImage("Sara"),
        
        genre: "Character",
        config: [
            
            {"default":700.0,"name":"base_atk","title":123,"type":"floatInput"},
            
            {"default":false,"name":"c6","title":846,"type":"bool"},
            
            {"default":10,"max":15,"min":1,"name":"skill2","title":14,"type":"int"},
            
        ],
    },
    
    "LisaTalent2": {
        name: "LisaTalent2",
        nameLocale: 109,
        
        description: 111,
        
        
        badge: getImage("Lisa"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "MonaQ": {
        name: "MonaQ",
        nameLocale: 1497,
        
        description: 1500,
        
        
        badge: getImage("Mona"),
        
        genre: "Character",
        config: [
            
            {"default":9,"max":15,"min":1,"name":"skill3","title":20,"type":"int"},
            
            {"default":false,"name":"c4","title":845,"type":"bool"},
            
        ],
    },
    
    "MonaC1": {
        name: "MonaC1",
        nameLocale: 1498,
        
        description: 1501,
        
        
        badge: getImage("Mona"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "NingguangTalent2": {
        name: "NingguangTalent2",
        nameLocale: 298,
        
        description: 300,
        
        
        badge: getImage("Ningguang"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "RaidenShogunE": {
        name: "RaidenShogunE",
        nameLocale: 1819,
        
        description: 1822,
        
        
        badge: getImage("Shougun"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"skill2","title":1821,"type":"int"},
            
            {"default":80,"max":100,"min":20,"name":"energy","title":368,"type":"int"},
            
        ],
    },
    
    "RaidenShogunC4": {
        name: "RaidenShogunC4",
        nameLocale: 1818,
        
        description: 1823,
        
        
        badge: getImage("Shougun"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "RazorC4": {
        name: "RazorC4",
        nameLocale: 1812,
        
        description: 1814,
        
        
        badge: getImage("Razor"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "RosariaTalent2": {
        name: "RosariaTalent2",
        nameLocale: 1439,
        
        description: 1442,
        
        
        badge: getImage("Rosaria"),
        
        genre: "Character",
        config: [
            
            {"default":70.0,"name":"crit","title":1443,"type":"floatPercentageInput"},
            
        ],
    },
    
    "RosariaC6": {
        name: "RosariaC6",
        nameLocale: 1438,
        
        description: 1441,
        
        
        badge: getImage("Rosaria"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "ShenheE": {
        name: "ShenheE",
        nameLocale: 1313,
        
        description: 1319,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":3000.0,"name":"atk","title":1324,"type":"floatInput"},
            
            {"default":8,"max":15,"min":1,"name":"skill2","title":1318,"type":"int"},
            
        ],
    },
    
    "ShenheQ": {
        name: "ShenheQ",
        nameLocale: 1315,
        
        description: 1321,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"skill3","title":1320,"type":"int"},
            
        ],
    },
    
    "ShenheTalent1": {
        name: "ShenheTalent1",
        nameLocale: 1314,
        
        description: 1322,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"c2","title":844,"type":"bool"},
            
        ],
    },
    
    "ShenheTalent2": {
        name: "ShenheTalent2",
        nameLocale: 1316,
        
        description: 1323,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":0,"name":"t","options":["点按","长按"],"title":706,"type":"option"},
            
        ],
    },
    
    "SucroseTalent1": {
        name: "SucroseTalent1",
        nameLocale: 1348,
        
        description: 1351,
        
        
        badge: getImage("Sucrose"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "SucroseTalent2": {
        name: "SucroseTalent2",
        nameLocale: 1346,
        
        description: 1352,
        
        
        badge: getImage("Sucrose"),
        
        genre: "Character",
        config: [
            
            {"default":200.0,"name":"em","title":1353,"type":"floatInput"},
            
        ],
    },
    
    "SucroseC6": {
        name: "SucroseC6",
        nameLocale: 1347,
        
        description: 1350,
        
        
        badge: getImage("Sucrose"),
        
        genre: "Character",
        config: [
            
            {"default":"Electro","name":"element","title":689,"type":"element4"},
            
        ],
    },
    
    "ThomaTalent1": {
        name: "ThomaTalent1",
        nameLocale: 680,
        
        description: 683,
        
        
        badge: getImage("Tohma"),
        
        genre: "Character",
        config: [
            
            {"default":2.0,"max":5.0,"min":0.0,"name":"stack","title":382,"type":"float"},
            
        ],
    },
    
    "ThomaC6": {
        name: "ThomaC6",
        nameLocale: 679,
        
        description: 682,
        
        
        badge: getImage("Tohma"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "VentiC2": {
        name: "VentiC2",
        nameLocale: 1165,
        
        description: 1167,
        
        
        badge: getImage("Venti"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"levitating","title":1527,"type":"bool"},
            
        ],
    },
    
    "VentiC6": {
        name: "VentiC6",
        nameLocale: 1164,
        
        description: 1168,
        
        
        badge: getImage("Venti"),
        
        genre: "Character",
        config: [
            
            {"default":true,"name":"is_convert","title":366,"type":"bool"},
            
            {"default":"Electro","name":"element","title":1649,"type":"element4"},
            
        ],
    },
    
    "XianglingTalent2": {
        name: "XianglingTalent2",
        nameLocale: 1879,
        
        description: 1883,
        
        
        badge: getImage("Xiangling"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XianglingC1": {
        name: "XianglingC1",
        nameLocale: 1877,
        
        description: 1881,
        
        
        badge: getImage("Xiangling"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XianglingC6": {
        name: "XianglingC6",
        nameLocale: 1878,
        
        description: 1882,
        
        
        badge: getImage("Xiangling"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XingqiuC2": {
        name: "XingqiuC2",
        nameLocale: 1543,
        
        description: 1545,
        
        
        badge: getImage("Xingqiu"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XinyanC4": {
        name: "XinyanC4",
        nameLocale: 1655,
        
        description: 1659,
        
        
        badge: getImage("Xinyan"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XinyanTalent2": {
        name: "XinyanTalent2",
        nameLocale: 1656,
        
        description: 1660,
        
        
        badge: getImage("Xinyan"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "YaeMikoC4": {
        name: "YaeMikoC4",
        nameLocale: 253,
        
        description: 255,
        
        
        badge: getImage("Yae"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "YoimiyaTalent2": {
        name: "YoimiyaTalent2",
        nameLocale: 581,
        
        description: 583,
        
        
        badge: getImage("Yoimiya"),
        
        genre: "Character",
        config: [
            
            {"default":0,"max":10,"min":0,"name":"talent1_stack","title":50,"type":"int"},
            
        ],
    },
    
    "YunjinQ": {
        name: "YunjinQ",
        nameLocale: 149,
        
        description: 151,
        
        
        badge: getImage("Yunjin"),
        
        genre: "Character",
        config: [
            
            {"default":10,"max":15,"min":1,"name":"skill3","title":20,"type":"int"},
            
            {"default":2000.0,"name":"def","title":153,"type":"floatInput"},
            
            {"default":true,"name":"talent2","title":7,"type":"bool"},
            
            {"default":4,"max":4,"min":1,"name":"ele_count","title":1757,"type":"int"},
            
        ],
    },
    
    "YunjinC2": {
        name: "YunjinC2",
        nameLocale: 148,
        
        description: 152,
        
        
        badge: getImage("Yunjin"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "ZhongliShield": {
        name: "ZhongliShield",
        nameLocale: 1738,
        
        description: 1740,
        
        
        badge: getImage("Zhongli"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "YelanTalent2": {
        name: "YelanTalent2",
        nameLocale: 494,
        
        description: 498,
        
        
        badge: getImage("Yelan"),
        
        genre: "Character",
        config: [
            
            {"default":14,"max":14,"min":0,"name":"secs","title":1426,"type":"int"},
            
        ],
    },
    
    "YelanC4": {
        name: "YelanC4",
        nameLocale: 495,
        
        description: 497,
        
        
        badge: getImage("Yelan"),
        
        genre: "Character",
        config: [
            
            {"default":4,"max":4,"min":1,"name":"count","title":1071,"type":"int"},
            
        ],
    },
    
    "KamisatoAyatoQ": {
        name: "KamisatoAyatoQ",
        nameLocale: 1374,
        
        description: 1377,
        
        
        badge: getImage("Ayato"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"skill_level","title":1376,"type":"int"},
            
        ],
    },
    
    "ShikanoinHeizouTalent2": {
        name: "ShikanoinHeizouTalent2",
        nameLocale: 1903,
        
        description: 1905,
        
        
        badge: getImage("Heizo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "TighnariC4": {
        name: "TighnariC4",
        nameLocale: 731,
        
        description: 733,
        
        
        badge: getImage("Tighnari"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"after_reaction","title":1691,"type":"bool"},
            
        ],
    },
    
    "DoriC4": {
        name: "DoriC4",
        nameLocale: 491,
        
        description: 492,
        
        
        badge: getImage("Dori"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"hp_below50","title":1292,"type":"bool"},
            
            {"default":true,"name":"energy_below50","title":247,"type":"bool"},
            
        ],
    },
    
    "NilouTalent1": {
        name: "NilouTalent1",
        nameLocale: 546,
        
        description: 481,
        
        
        badge: getImage("Nilou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "NilouTalent2": {
        name: "NilouTalent2",
        nameLocale: 547,
        
        description: 471,
        
        
        badge: getImage("Nilou"),
        
        genre: "Character",
        config: [
            
            {"default":60000.0,"name":"hp","title":549,"type":"floatInput"},
            
        ],
    },
    
    "CandaceQ": {
        name: "CandaceQ",
        nameLocale: 459,
        
        description: 1585,
        
        
        badge: getImage("Candace"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "CandaceTalent2": {
        name: "CandaceTalent2",
        nameLocale: 458,
        
        description: 479,
        
        
        badge: getImage("Candace"),
        
        genre: "Character",
        config: [
            
            {"default":30000.0,"name":"hp","title":460,"type":"floatInput"},
            
        ],
    },
    
    "NahidaTalent1": {
        name: "NahidaTalent1",
        nameLocale: 1419,
        
        description: 810,
        
        
        badge: getImage("Nahida"),
        
        genre: "Character",
        config: [
            
            {"default":1000.0,"max":3000.0,"min":0.0,"name":"max_em","title":1766,"type":"float"},
            
        ],
    },
    
    "FaruzanQ": {
        name: "FaruzanQ",
        nameLocale: 1120,
        
        description: 1259,
        
        
        badge: getImage("Faruzan"),
        
        genre: "Character",
        config: [
            
            {"default":650,"max":1000,"min":0,"name":"base_atk","title":1121,"type":"int"},
            
            {"default":10,"max":15,"min":1,"name":"q_level","title":20,"type":"int"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate_q1","title":51,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate_q2","title":46,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate_talent2","title":22,"type":"float"},
            
            {"default":false,"name":"enable_c6","title":257,"type":"bool"},
            
        ],
    },
    
    "Mika": {
        name: "Mika",
        nameLocale: 1410,
        
        description: 840,
        
        
        badge: getImage("Mika"),
        
        genre: "Character",
        config: [
            
            {"default":3.0,"max":5.0,"min":0.0,"name":"stack_talent2","title":196,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate_c6","title":259,"type":"float"},
            
        ],
    },
    
    "KavehQ": {
        name: "KavehQ",
        nameLocale: 358,
        
        description: 1763,
        
        
        badge: getImage("Kaveh"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"q_level","title":359,"type":"int"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":1093,"type":"float"},
            
        ],
    },
    
    "BaizhuTalent2": {
        name: "BaizhuTalent2",
        nameLocale: 1328,
        
        description: 370,
        
        
        badge: getImage("Baizhuer"),
        
        genre: "Character",
        config: [
            
            {"default":50000.0,"max":50000.0,"min":0.0,"name":"hp","title":1330,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":1093,"type":"float"},
            
        ],
    },
    
    "BaizhuC4": {
        name: "BaizhuC4",
        nameLocale: 1329,
        
        description: 811,
        
        
        badge: getImage("Baizhuer"),
        
        genre: "Character",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":1093,"type":"float"},
            
        ],
    },
    
    "FreedomSworn": {
        name: "FreedomSworn",
        nameLocale: 1479,
        
        description: 25,
        
        
        badge: getImageW("Sword_Widsith"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1413,"type":"intInput"},
            
        ],
    },
    
    "SongOfBrokenPines": {
        name: "SongOfBrokenPines",
        nameLocale: 1053,
        
        description: 26,
        
        
        badge: getImageW("Claymore_Widsith"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1413,"type":"intInput"},
            
        ],
    },
    
    "WolfsGravestone": {
        name: "WolfsGravestone",
        nameLocale: 1235,
        
        description: 751,
        
        
        badge: getImageW("Claymore_Wolfmound"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1413,"type":"intInput"},
            
        ],
    },
    
    "ThrillingTalesOfDragonSlayers": {
        name: "ThrillingTalesOfDragonSlayers",
        nameLocale: 1599,
        
        description: 183,
        
        
        badge: getImageW("Catalyst_Pulpfic"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1413,"type":"intInput"},
            
        ],
    },
    
    "ElegyOfTheEnd": {
        name: "ElegyOfTheEnd",
        nameLocale: 1425,
        
        description: 349,
        
        
        badge: getImageW("Bow_Widsith"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1413,"type":"intInput"},
            
        ],
    },
    
    "HakushinRing": {
        name: "HakushinRing",
        nameLocale: 1335,
        
        description: 1080,
        
        
        badge: getImageW("Catalyst_Bakufu"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1413,"type":"intInput"},
            
            {"default":"Electro","name":"element","title":200,"type":"element8"},
            
        ],
    },
    
    "SapwoodBlade": {
        name: "SapwoodBlade",
        nameLocale: 364,
        
        description: 717,
        
        
        badge: getImageW("Sword_Arakalari"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1413,"type":"intInput"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":634,"type":"float"},
            
        ],
    },
    
    "Moonpiercer": {
        name: "Moonpiercer",
        nameLocale: 1620,
        
        description: 718,
        
        
        badge: getImageW("Pole_Arakalari"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1413,"type":"intInput"},
            
        ],
    },
    
    "XiphosMoonlight": {
        name: "XiphosMoonlight",
        nameLocale: 1570,
        
        description: 1564,
        
        
        badge: getImageW("Sword_Pleroma"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1413,"type":"intInput"},
            
            {"default":900.0,"name":"em","title":239,"type":"floatInput"},
            
        ],
    },
    
    "MakhairaAquamarine": {
        name: "MakhairaAquamarine",
        nameLocale: 1254,
        
        description: 37,
        
        
        badge: getImageW("Claymore_Pleroma"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1413,"type":"intInput"},
            
            {"default":900.0,"name":"em","title":239,"type":"floatInput"},
            
        ],
    },
    
    "KeyOfKhajNisut": {
        name: "KeyOfKhajNisut",
        nameLocale: 454,
        
        description: 597,
        
        
        badge: getImageW("Sword_Deshret"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1413,"type":"intInput"},
            
            {"default":20000.0,"name":"hp","title":1286,"type":"floatInput"},
            
        ],
    },
    
    "ResonancePyro2": {
        name: "ResonancePyro2",
        nameLocale: 208,
        
        description: 740,
        
        
        badge: ResonancePyro2_image,
        
        genre: "Resonance",
        config: [
            
        ],
    },
    
    "ResonanceCryo2": {
        name: "ResonanceCryo2",
        nameLocale: 209,
        
        description: 735,
        
        
        badge: ResonanceCryo2_image,
        
        genre: "Resonance",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":634,"type":"float"},
            
        ],
    },
    
    "ResonanceGeo2": {
        name: "ResonanceGeo2",
        nameLocale: 206,
        
        description: 710,
        
        
        badge: ResonanceGeo2_image,
        
        genre: "Resonance",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate1","title":762,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate2","title":764,"type":"float"},
            
        ],
    },
    
    "ResonanceHydro2": {
        name: "ResonanceHydro2",
        nameLocale: 207,
        
        description: 1186,
        
        
        badge: ResonanceHydro2_image,
        
        genre: "Resonance",
        config: [
            
        ],
    },
    
    "ResonanceDendro2": {
        name: "ResonanceDendro2",
        nameLocale: 210,
        
        description: 241,
        
        
        badge: ResonanceDendro2_image,
        
        genre: "Resonance",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate1","title":762,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate2","title":764,"type":"float"},
            
        ],
    },
    
    "Instructor4": {
        name: "Instructor4",
        nameLocale: 771,
        
        description: 1590,
        
        
        badge: getImageA("UI_RelicIcon_10007_4"),
        
        genre: "Artifact",
        config: [
            
        ],
    },
    
    "NoblesseOblige4": {
        name: "NoblesseOblige4",
        nameLocale: 832,
        
        description: 806,
        
        
        badge: getImageA("UI_RelicIcon_15007_4"),
        
        genre: "Artifact",
        config: [
            
        ],
    },
    
    "ArchaicPetra4": {
        name: "ArchaicPetra4",
        nameLocale: 663,
        
        description: 1520,
        
        
        badge: getImageA("UI_RelicIcon_15014_4"),
        
        genre: "Artifact",
        config: [
            
            {"default":"Electro","name":"element","title":1428,"type":"element4"},
            
        ],
    },
    
    "ViridescentVenerer4": {
        name: "ViridescentVenerer4",
        nameLocale: 1448,
        
        description: 1072,
        
        
        badge: getImageA("UI_RelicIcon_15002_4"),
        
        genre: "Artifact",
        config: [
            
            {"default":"Electro","name":"element","title":684,"type":"element4"},
            
        ],
    },
    
    "TenacityOfTheMillelith4": {
        name: "TenacityOfTheMillelith4",
        nameLocale: 347,
        
        description: 216,
        
        
        badge: getImageA("UI_RelicIcon_15017_4"),
        
        genre: "Artifact",
        config: [
            
        ],
    },
    
    "DeepwoodMemories4": {
        name: "DeepwoodMemories4",
        nameLocale: 1154,
        
        description: 222,
        
        
        badge: getImageA("UI_RelicIcon_15025_4"),
        
        genre: "Artifact",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":634,"type":"float"},
            
        ],
    },
    
}