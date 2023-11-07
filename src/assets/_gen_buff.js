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
        nameLocale: 735,
        
        description: null,
        
        
        badge: ATKPercentage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":775,"type":"floatPercentageInput"},
            
        ],
    },
    
    "DEFPercentage": {
        name: "DEFPercentage",
        nameLocale: 1770,
        
        description: null,
        
        
        badge: DEFPercentage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":775,"type":"floatPercentageInput"},
            
        ],
    },
    
    "HPPercentage": {
        name: "HPPercentage",
        nameLocale: 1284,
        
        description: null,
        
        
        badge: HPPercentage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":775,"type":"floatPercentageInput"},
            
        ],
    },
    
    "ATKFixed": {
        name: "ATKFixed",
        nameLocale: 734,
        
        description: null,
        
        
        badge: ATKFixed_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":775,"type":"floatInput"},
            
        ],
    },
    
    "DEFFixed": {
        name: "DEFFixed",
        nameLocale: 1769,
        
        description: null,
        
        
        badge: DEFFixed_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":775,"type":"floatInput"},
            
        ],
    },
    
    "HPFixed": {
        name: "HPFixed",
        nameLocale: 1283,
        
        description: null,
        
        
        badge: HPFixed_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":775,"type":"floatInput"},
            
        ],
    },
    
    "Critical": {
        name: "Critical",
        nameLocale: 1008,
        
        description: null,
        
        
        badge: Critical_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":775,"type":"floatPercentageInput"},
            
        ],
    },
    
    "CriticalDamage": {
        name: "CriticalDamage",
        nameLocale: 1004,
        
        description: null,
        
        
        badge: CriticalDamage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":775,"type":"floatPercentageInput"},
            
        ],
    },
    
    "CustomBonus": {
        name: "CustomBonus",
        nameLocale: 182,
        
        description: null,
        
        
        badge: CustomBonus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":775,"type":"floatPercentageInput"},
            
        ],
    },
    
    "ElementalMastery": {
        name: "ElementalMastery",
        nameLocale: 239,
        
        description: null,
        
        
        badge: ElementalMastery_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":775,"type":"floatInput"},
            
        ],
    },
    
    "Recharge": {
        name: "Recharge",
        nameLocale: 201,
        
        description: null,
        
        
        badge: Recharge_image,
        
        genre: "Common",
        config: [
            
            {"default":20.0,"name":"p","title":775,"type":"floatPercentageInput"},
            
        ],
    },
    
    "DEFMinus": {
        name: "DEFMinus",
        nameLocale: 293,
        
        description: 0,
        
        
        badge: DEFMinus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":775,"type":"floatPercentageInput"},
            
        ],
    },
    
    "ResMinus": {
        name: "ResMinus",
        nameLocale: 292,
        
        description: null,
        
        
        badge: ResMinus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":775,"type":"floatPercentageInput"},
            
        ],
    },
    
    "HealingBonus": {
        name: "HealingBonus",
        nameLocale: 1112,
        
        description: null,
        
        
        badge: HealingBonus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":775,"type":"floatPercentageInput"},
            
        ],
    },
    
    "BaseDmg": {
        name: "BaseDmg",
        nameLocale: 471,
        
        description: 1641,
        
        
        badge: BaseDmg_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":775,"type":"floatInput"},
            
        ],
    },
    
    "AlbedoTalent2": {
        name: "AlbedoTalent2",
        nameLocale: 1782,
        
        description: 1787,
        
        
        badge: getImage("Albedo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AlbedoC4": {
        name: "AlbedoC4",
        nameLocale: 1783,
        
        description: 1785,
        
        
        badge: getImage("Albedo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AlbedoC6": {
        name: "AlbedoC6",
        nameLocale: 1781,
        
        description: 1786,
        
        
        badge: getImage("Albedo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AloyTalent1": {
        name: "AloyTalent1",
        nameLocale: 466,
        
        description: 467,
        
        
        badge: getImage("Aloy"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AratakiIttoC4": {
        name: "AratakiIttoC4",
        nameLocale: 1486,
        
        description: 1488,
        
        
        badge: getImage("Itto"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "BeidouC6": {
        name: "BeidouC6",
        nameLocale: 336,
        
        description: 338,
        
        
        badge: getImage("Beidou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "BennettQ": {
        name: "BennettQ",
        nameLocale: 1259,
        
        description: 1262,
        
        
        badge: getImage("Bennett"),
        
        genre: "Character",
        config: [
            
            {"default":800.0,"name":"base_atk","title":1264,"type":"floatInput"},
            
            {"default":true,"name":"c1","title":841,"type":"bool"},
            
            {"default":10,"max":15,"min":1,"name":"skill3","title":703,"type":"int"},
            
        ],
    },
    
    "BennettC6": {
        name: "BennettC6",
        nameLocale: 1258,
        
        description: 1263,
        
        
        badge: getImage("Bennett"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "ChongyunTalent2": {
        name: "ChongyunTalent2",
        nameLocale: 1697,
        
        description: 1699,
        
        
        badge: getImage("Chongyun"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "DionaC6G50": {
        name: "DionaC6G50",
        nameLocale: 1669,
        
        description: 1671,
        
        
        badge: getImage("Diona"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "EulaE": {
        name: "EulaE",
        nameLocale: 178,
        
        description: 180,
        
        
        badge: getImage("Eula"),
        
        genre: "Character",
        config: [
            
            {"default":9,"max":15,"min":1,"name":"skill2","title":703,"type":"int"},
            
        ],
    },
    
    "GanyuTalent2": {
        name: "GanyuTalent2",
        nameLocale: 1278,
        
        description: 1282,
        
        
        badge: getImage("Ganyu"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GanyuC1": {
        name: "GanyuC1",
        nameLocale: 1279,
        
        description: 1281,
        
        
        badge: getImage("Ganyu"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GorouE1": {
        name: "GorouE1",
        nameLocale: 159,
        
        description: 164,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
            {"default":10,"max":15,"min":1,"name":"skill2","title":163,"type":"int"},
            
        ],
    },
    
    "GorouE3": {
        name: "GorouE3",
        nameLocale: 160,
        
        description: 165,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GorouTalent1": {
        name: "GorouTalent1",
        nameLocale: 158,
        
        description: 167,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GorouC6": {
        name: "GorouC6",
        nameLocale: 161,
        
        description: 166,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
            {"default":1,"max":3,"min":1,"name":"level","title":1838,"type":"int"},
            
        ],
    },
    
    "HuTaoTalent1": {
        name: "HuTaoTalent1",
        nameLocale: 1460,
        
        description: 1462,
        
        
        badge: getImage("Hutao"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "JeanC4": {
        name: "JeanC4",
        nameLocale: 1271,
        
        description: 1273,
        
        
        badge: getImage("Qin"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KaedeharaKazuhaTalent2": {
        name: "KaedeharaKazuhaTalent2",
        nameLocale: 1057,
        
        description: 1061,
        
        
        badge: getImage("Kazuha"),
        
        genre: "Character",
        config: [
            
            {"default":"Electro","name":"element","title":682,"type":"element4"},
            
            {"default":800.0,"name":"em","title":79,"type":"floatInput"},
            
        ],
    },
    
    "KaedeharaKazuhaC2": {
        name: "KaedeharaKazuhaC2",
        nameLocale: 1056,
        
        description: 1060,
        
        
        badge: getImage("Kazuha"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KamisatoAyakaC4": {
        name: "KamisatoAyakaC4",
        nameLocale: 1377,
        
        description: 1428,
        
        
        badge: getImage("Ayaka"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KleeC2": {
        name: "KleeC2",
        nameLocale: 387,
        
        description: 389,
        
        
        badge: getImage("Klee"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KleeC6": {
        name: "KleeC6",
        nameLocale: 386,
        
        description: 390,
        
        
        badge: getImage("Klee"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KujouSaraEOrQ": {
        name: "KujouSaraEOrQ",
        nameLocale: 117,
        
        description: 120,
        
        
        badge: getImage("Sara"),
        
        genre: "Character",
        config: [
            
            {"default":700.0,"name":"base_atk","title":121,"type":"floatInput"},
            
            {"default":false,"name":"c6","title":844,"type":"bool"},
            
            {"default":10,"max":15,"min":1,"name":"skill2","title":14,"type":"int"},
            
        ],
    },
    
    "LisaTalent2": {
        name: "LisaTalent2",
        nameLocale: 107,
        
        description: 109,
        
        
        badge: getImage("Lisa"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "MonaQ": {
        name: "MonaQ",
        nameLocale: 1494,
        
        description: 1497,
        
        
        badge: getImage("Mona"),
        
        genre: "Character",
        config: [
            
            {"default":9,"max":15,"min":1,"name":"skill3","title":20,"type":"int"},
            
            {"default":false,"name":"c4","title":843,"type":"bool"},
            
        ],
    },
    
    "MonaC1": {
        name: "MonaC1",
        nameLocale: 1495,
        
        description: 1498,
        
        
        badge: getImage("Mona"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "NingguangTalent2": {
        name: "NingguangTalent2",
        nameLocale: 296,
        
        description: 298,
        
        
        badge: getImage("Ningguang"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "RaidenShogunE": {
        name: "RaidenShogunE",
        nameLocale: 1816,
        
        description: 1819,
        
        
        badge: getImage("Shougun"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"skill2","title":1818,"type":"int"},
            
            {"default":80,"max":100,"min":20,"name":"energy","title":366,"type":"int"},
            
        ],
    },
    
    "RaidenShogunC4": {
        name: "RaidenShogunC4",
        nameLocale: 1815,
        
        description: 1820,
        
        
        badge: getImage("Shougun"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "RazorC4": {
        name: "RazorC4",
        nameLocale: 1809,
        
        description: 1811,
        
        
        badge: getImage("Razor"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "RosariaTalent2": {
        name: "RosariaTalent2",
        nameLocale: 1436,
        
        description: 1439,
        
        
        badge: getImage("Rosaria"),
        
        genre: "Character",
        config: [
            
            {"default":70.0,"name":"crit","title":1440,"type":"floatPercentageInput"},
            
        ],
    },
    
    "RosariaC6": {
        name: "RosariaC6",
        nameLocale: 1435,
        
        description: 1438,
        
        
        badge: getImage("Rosaria"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "ShenheE": {
        name: "ShenheE",
        nameLocale: 1310,
        
        description: 1316,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":3000.0,"name":"atk","title":1321,"type":"floatInput"},
            
            {"default":8,"max":15,"min":1,"name":"skill2","title":1315,"type":"int"},
            
        ],
    },
    
    "ShenheQ": {
        name: "ShenheQ",
        nameLocale: 1312,
        
        description: 1318,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"skill3","title":1317,"type":"int"},
            
        ],
    },
    
    "ShenheTalent1": {
        name: "ShenheTalent1",
        nameLocale: 1311,
        
        description: 1319,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"c2","title":842,"type":"bool"},
            
        ],
    },
    
    "ShenheTalent2": {
        name: "ShenheTalent2",
        nameLocale: 1313,
        
        description: 1320,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":0,"name":"t","options":["点按","长按"],"title":704,"type":"option"},
            
        ],
    },
    
    "SucroseTalent1": {
        name: "SucroseTalent1",
        nameLocale: 1345,
        
        description: 1348,
        
        
        badge: getImage("Sucrose"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "SucroseTalent2": {
        name: "SucroseTalent2",
        nameLocale: 1343,
        
        description: 1349,
        
        
        badge: getImage("Sucrose"),
        
        genre: "Character",
        config: [
            
            {"default":200.0,"name":"em","title":1350,"type":"floatInput"},
            
        ],
    },
    
    "SucroseC6": {
        name: "SucroseC6",
        nameLocale: 1344,
        
        description: 1347,
        
        
        badge: getImage("Sucrose"),
        
        genre: "Character",
        config: [
            
            {"default":"Electro","name":"element","title":687,"type":"element4"},
            
        ],
    },
    
    "ThomaTalent1": {
        name: "ThomaTalent1",
        nameLocale: 678,
        
        description: 681,
        
        
        badge: getImage("Tohma"),
        
        genre: "Character",
        config: [
            
            {"default":2.0,"max":5.0,"min":0.0,"name":"stack","title":380,"type":"float"},
            
        ],
    },
    
    "ThomaC6": {
        name: "ThomaC6",
        nameLocale: 677,
        
        description: 680,
        
        
        badge: getImage("Tohma"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "VentiC2": {
        name: "VentiC2",
        nameLocale: 1162,
        
        description: 1164,
        
        
        badge: getImage("Venti"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"levitating","title":1524,"type":"bool"},
            
        ],
    },
    
    "VentiC6": {
        name: "VentiC6",
        nameLocale: 1161,
        
        description: 1165,
        
        
        badge: getImage("Venti"),
        
        genre: "Character",
        config: [
            
            {"default":true,"name":"is_convert","title":364,"type":"bool"},
            
            {"default":"Electro","name":"element","title":1646,"type":"element4"},
            
        ],
    },
    
    "XianglingTalent2": {
        name: "XianglingTalent2",
        nameLocale: 1876,
        
        description: 1880,
        
        
        badge: getImage("Xiangling"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XianglingC1": {
        name: "XianglingC1",
        nameLocale: 1874,
        
        description: 1878,
        
        
        badge: getImage("Xiangling"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XianglingC6": {
        name: "XianglingC6",
        nameLocale: 1875,
        
        description: 1879,
        
        
        badge: getImage("Xiangling"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XingqiuC2": {
        name: "XingqiuC2",
        nameLocale: 1540,
        
        description: 1542,
        
        
        badge: getImage("Xingqiu"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XinyanC4": {
        name: "XinyanC4",
        nameLocale: 1652,
        
        description: 1656,
        
        
        badge: getImage("Xinyan"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XinyanTalent2": {
        name: "XinyanTalent2",
        nameLocale: 1653,
        
        description: 1657,
        
        
        badge: getImage("Xinyan"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "YaeMikoC4": {
        name: "YaeMikoC4",
        nameLocale: 252,
        
        description: 254,
        
        
        badge: getImage("Yae"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "YoimiyaTalent2": {
        name: "YoimiyaTalent2",
        nameLocale: 579,
        
        description: 581,
        
        
        badge: getImage("Yoimiya"),
        
        genre: "Character",
        config: [
            
            {"default":0,"max":10,"min":0,"name":"talent1_stack","title":49,"type":"int"},
            
        ],
    },
    
    "YunjinQ": {
        name: "YunjinQ",
        nameLocale: 147,
        
        description: 149,
        
        
        badge: getImage("Yunjin"),
        
        genre: "Character",
        config: [
            
            {"default":10,"max":15,"min":1,"name":"skill3","title":20,"type":"int"},
            
            {"default":2000.0,"name":"def","title":151,"type":"floatInput"},
            
            {"default":true,"name":"talent2","title":7,"type":"bool"},
            
            {"default":4,"max":4,"min":1,"name":"ele_count","title":1754,"type":"int"},
            
        ],
    },
    
    "YunjinC2": {
        name: "YunjinC2",
        nameLocale: 146,
        
        description: 150,
        
        
        badge: getImage("Yunjin"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "ZhongliShield": {
        name: "ZhongliShield",
        nameLocale: 1735,
        
        description: 1737,
        
        
        badge: getImage("Zhongli"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "YelanTalent2": {
        name: "YelanTalent2",
        nameLocale: 492,
        
        description: 496,
        
        
        badge: getImage("Yelan"),
        
        genre: "Character",
        config: [
            
            {"default":14,"max":14,"min":0,"name":"secs","title":1423,"type":"int"},
            
        ],
    },
    
    "YelanC4": {
        name: "YelanC4",
        nameLocale: 493,
        
        description: 495,
        
        
        badge: getImage("Yelan"),
        
        genre: "Character",
        config: [
            
            {"default":4,"max":4,"min":1,"name":"count","title":1068,"type":"int"},
            
        ],
    },
    
    "KamisatoAyatoQ": {
        name: "KamisatoAyatoQ",
        nameLocale: 1371,
        
        description: 1374,
        
        
        badge: getImage("Ayato"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"skill_level","title":1373,"type":"int"},
            
        ],
    },
    
    "ShikanoinHeizouTalent2": {
        name: "ShikanoinHeizouTalent2",
        nameLocale: 1900,
        
        description: 1902,
        
        
        badge: getImage("Heizo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "TighnariC4": {
        name: "TighnariC4",
        nameLocale: 729,
        
        description: 731,
        
        
        badge: getImage("Tighnari"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"after_reaction","title":1688,"type":"bool"},
            
        ],
    },
    
    "DoriC4": {
        name: "DoriC4",
        nameLocale: 489,
        
        description: 490,
        
        
        badge: getImage("Dori"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"hp_below50","title":1289,"type":"bool"},
            
            {"default":true,"name":"energy_below50","title":246,"type":"bool"},
            
        ],
    },
    
    "NilouTalent1": {
        name: "NilouTalent1",
        nameLocale: 544,
        
        description: 479,
        
        
        badge: getImage("Nilou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "NilouTalent2": {
        name: "NilouTalent2",
        nameLocale: 545,
        
        description: 469,
        
        
        badge: getImage("Nilou"),
        
        genre: "Character",
        config: [
            
            {"default":60000.0,"name":"hp","title":547,"type":"floatInput"},
            
        ],
    },
    
    "CandaceQ": {
        name: "CandaceQ",
        nameLocale: 457,
        
        description: 1582,
        
        
        badge: getImage("Candace"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "CandaceTalent2": {
        name: "CandaceTalent2",
        nameLocale: 456,
        
        description: 477,
        
        
        badge: getImage("Candace"),
        
        genre: "Character",
        config: [
            
            {"default":30000.0,"name":"hp","title":458,"type":"floatInput"},
            
        ],
    },
    
    "NahidaTalent1": {
        name: "NahidaTalent1",
        nameLocale: 1416,
        
        description: 808,
        
        
        badge: getImage("Nahida"),
        
        genre: "Character",
        config: [
            
            {"default":1000.0,"max":3000.0,"min":0.0,"name":"max_em","title":1763,"type":"float"},
            
        ],
    },
    
    "FaruzanQ": {
        name: "FaruzanQ",
        nameLocale: 1117,
        
        description: 1256,
        
        
        badge: getImage("Faruzan"),
        
        genre: "Character",
        config: [
            
            {"default":650,"max":1000,"min":0,"name":"base_atk","title":1118,"type":"int"},
            
            {"default":10,"max":15,"min":1,"name":"q_level","title":20,"type":"int"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate_q1","title":50,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate_q2","title":46,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate_talent2","title":22,"type":"float"},
            
            {"default":false,"name":"enable_c6","title":256,"type":"bool"},
            
        ],
    },
    
    "Mika": {
        name: "Mika",
        nameLocale: 1407,
        
        description: 838,
        
        
        badge: getImage("Mika"),
        
        genre: "Character",
        config: [
            
            {"default":3.0,"max":5.0,"min":0.0,"name":"stack_talent2","title":195,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate_c6","title":258,"type":"float"},
            
        ],
    },
    
    "KavehQ": {
        name: "KavehQ",
        nameLocale: 356,
        
        description: 1760,
        
        
        badge: getImage("Kaveh"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"q_level","title":357,"type":"int"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":1090,"type":"float"},
            
        ],
    },
    
    "BaizhuTalent2": {
        name: "BaizhuTalent2",
        nameLocale: 1325,
        
        description: 368,
        
        
        badge: getImage("Baizhuer"),
        
        genre: "Character",
        config: [
            
            {"default":50000.0,"max":50000.0,"min":0.0,"name":"hp","title":1327,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":1090,"type":"float"},
            
        ],
    },
    
    "BaizhuC4": {
        name: "BaizhuC4",
        nameLocale: 1326,
        
        description: 809,
        
        
        badge: getImage("Baizhuer"),
        
        genre: "Character",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":1090,"type":"float"},
            
        ],
    },
    
    "FreedomSworn": {
        name: "FreedomSworn",
        nameLocale: 1476,
        
        description: 25,
        
        
        badge: getImageW("Sword_Widsith"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1410,"type":"intInput"},
            
        ],
    },
    
    "SongOfBrokenPines": {
        name: "SongOfBrokenPines",
        nameLocale: 1050,
        
        description: 26,
        
        
        badge: getImageW("Claymore_Widsith"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1410,"type":"intInput"},
            
        ],
    },
    
    "WolfsGravestone": {
        name: "WolfsGravestone",
        nameLocale: 1232,
        
        description: 749,
        
        
        badge: getImageW("Claymore_Wolfmound"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1410,"type":"intInput"},
            
        ],
    },
    
    "ThrillingTalesOfDragonSlayers": {
        name: "ThrillingTalesOfDragonSlayers",
        nameLocale: 1596,
        
        description: 181,
        
        
        badge: getImageW("Catalyst_Pulpfic"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1410,"type":"intInput"},
            
        ],
    },
    
    "ElegyOfTheEnd": {
        name: "ElegyOfTheEnd",
        nameLocale: 1422,
        
        description: 347,
        
        
        badge: getImageW("Bow_Widsith"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1410,"type":"intInput"},
            
        ],
    },
    
    "HakushinRing": {
        name: "HakushinRing",
        nameLocale: 1332,
        
        description: 1077,
        
        
        badge: getImageW("Catalyst_Bakufu"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1410,"type":"intInput"},
            
            {"default":"Electro","name":"element","title":199,"type":"element8"},
            
        ],
    },
    
    "SapwoodBlade": {
        name: "SapwoodBlade",
        nameLocale: 362,
        
        description: 715,
        
        
        badge: getImageW("Sword_Arakalari"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1410,"type":"intInput"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":632,"type":"float"},
            
        ],
    },
    
    "Moonpiercer": {
        name: "Moonpiercer",
        nameLocale: 1617,
        
        description: 716,
        
        
        badge: getImageW("Pole_Arakalari"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1410,"type":"intInput"},
            
        ],
    },
    
    "XiphosMoonlight": {
        name: "XiphosMoonlight",
        nameLocale: 1567,
        
        description: 1561,
        
        
        badge: getImageW("Sword_Pleroma"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1410,"type":"intInput"},
            
            {"default":900.0,"name":"em","title":238,"type":"floatInput"},
            
        ],
    },
    
    "MakhairaAquamarine": {
        name: "MakhairaAquamarine",
        nameLocale: 1251,
        
        description: 36,
        
        
        badge: getImageW("Claymore_Pleroma"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1410,"type":"intInput"},
            
            {"default":900.0,"name":"em","title":238,"type":"floatInput"},
            
        ],
    },
    
    "KeyOfKhajNisut": {
        name: "KeyOfKhajNisut",
        nameLocale: 452,
        
        description: 595,
        
        
        badge: getImageW("Sword_Deshret"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1410,"type":"intInput"},
            
            {"default":20000.0,"name":"hp","title":1283,"type":"floatInput"},
            
        ],
    },
    
    "ResonancePyro2": {
        name: "ResonancePyro2",
        nameLocale: 207,
        
        description: 738,
        
        
        badge: ResonancePyro2_image,
        
        genre: "Resonance",
        config: [
            
        ],
    },
    
    "ResonanceCryo2": {
        name: "ResonanceCryo2",
        nameLocale: 208,
        
        description: 733,
        
        
        badge: ResonanceCryo2_image,
        
        genre: "Resonance",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":632,"type":"float"},
            
        ],
    },
    
    "ResonanceGeo2": {
        name: "ResonanceGeo2",
        nameLocale: 205,
        
        description: 708,
        
        
        badge: ResonanceGeo2_image,
        
        genre: "Resonance",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate1","title":760,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate2","title":761,"type":"float"},
            
        ],
    },
    
    "ResonanceHydro2": {
        name: "ResonanceHydro2",
        nameLocale: 206,
        
        description: 1183,
        
        
        badge: ResonanceHydro2_image,
        
        genre: "Resonance",
        config: [
            
        ],
    },
    
    "ResonanceDendro2": {
        name: "ResonanceDendro2",
        nameLocale: 209,
        
        description: 240,
        
        
        badge: ResonanceDendro2_image,
        
        genre: "Resonance",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate1","title":760,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate2","title":761,"type":"float"},
            
        ],
    },
    
    "Instructor4": {
        name: "Instructor4",
        nameLocale: 769,
        
        description: 1587,
        
        
        badge: getImageA("UI_RelicIcon_10007_4"),
        
        genre: "Artifact",
        config: [
            
        ],
    },
    
    "NoblesseOblige4": {
        name: "NoblesseOblige4",
        nameLocale: 830,
        
        description: 804,
        
        
        badge: getImageA("UI_RelicIcon_15007_4"),
        
        genre: "Artifact",
        config: [
            
        ],
    },
    
    "ArchaicPetra4": {
        name: "ArchaicPetra4",
        nameLocale: 661,
        
        description: 1517,
        
        
        badge: getImageA("UI_RelicIcon_15014_4"),
        
        genre: "Artifact",
        config: [
            
            {"default":"Electro","name":"element","title":1425,"type":"element4"},
            
        ],
    },
    
    "ViridescentVenerer4": {
        name: "ViridescentVenerer4",
        nameLocale: 1445,
        
        description: 1069,
        
        
        badge: getImageA("UI_RelicIcon_15002_4"),
        
        genre: "Artifact",
        config: [
            
            {"default":"Electro","name":"element","title":682,"type":"element4"},
            
        ],
    },
    
    "TenacityOfTheMillelith4": {
        name: "TenacityOfTheMillelith4",
        nameLocale: 345,
        
        description: 215,
        
        
        badge: getImageA("UI_RelicIcon_15017_4"),
        
        genre: "Artifact",
        config: [
            
        ],
    },
    
    "DeepwoodMemories4": {
        name: "DeepwoodMemories4",
        nameLocale: 1151,
        
        description: 221,
        
        
        badge: getImageA("UI_RelicIcon_15025_4"),
        
        genre: "Artifact",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":632,"type":"float"},
            
        ],
    },
    
}