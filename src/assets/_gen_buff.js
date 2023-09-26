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
        nameLocale: 721,
        
        description: null,
        
        
        badge: ATKPercentage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":761,"type":"floatPercentageInput"},
            
        ],
    },
    
    "DEFPercentage": {
        name: "DEFPercentage",
        nameLocale: 1735,
        
        description: null,
        
        
        badge: DEFPercentage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":761,"type":"floatPercentageInput"},
            
        ],
    },
    
    "HPPercentage": {
        name: "HPPercentage",
        nameLocale: 1261,
        
        description: null,
        
        
        badge: HPPercentage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":761,"type":"floatPercentageInput"},
            
        ],
    },
    
    "ATKFixed": {
        name: "ATKFixed",
        nameLocale: 720,
        
        description: null,
        
        
        badge: ATKFixed_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":761,"type":"floatInput"},
            
        ],
    },
    
    "DEFFixed": {
        name: "DEFFixed",
        nameLocale: 1734,
        
        description: null,
        
        
        badge: DEFFixed_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":761,"type":"floatInput"},
            
        ],
    },
    
    "HPFixed": {
        name: "HPFixed",
        nameLocale: 1260,
        
        description: null,
        
        
        badge: HPFixed_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":761,"type":"floatInput"},
            
        ],
    },
    
    "Critical": {
        name: "Critical",
        nameLocale: 991,
        
        description: null,
        
        
        badge: Critical_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":761,"type":"floatPercentageInput"},
            
        ],
    },
    
    "CriticalDamage": {
        name: "CriticalDamage",
        nameLocale: 987,
        
        description: null,
        
        
        badge: CriticalDamage_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":761,"type":"floatPercentageInput"},
            
        ],
    },
    
    "CustomBonus": {
        name: "CustomBonus",
        nameLocale: 172,
        
        description: null,
        
        
        badge: CustomBonus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":761,"type":"floatPercentageInput"},
            
        ],
    },
    
    "ElementalMastery": {
        name: "ElementalMastery",
        nameLocale: 229,
        
        description: null,
        
        
        badge: ElementalMastery_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":761,"type":"floatInput"},
            
        ],
    },
    
    "Recharge": {
        name: "Recharge",
        nameLocale: 191,
        
        description: null,
        
        
        badge: Recharge_image,
        
        genre: "Common",
        config: [
            
            {"default":20.0,"name":"p","title":761,"type":"floatPercentageInput"},
            
        ],
    },
    
    "DEFMinus": {
        name: "DEFMinus",
        nameLocale: 283,
        
        description: 0,
        
        
        badge: DEFMinus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":761,"type":"floatPercentageInput"},
            
        ],
    },
    
    "ResMinus": {
        name: "ResMinus",
        nameLocale: 282,
        
        description: null,
        
        
        badge: ResMinus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":761,"type":"floatPercentageInput"},
            
        ],
    },
    
    "HealingBonus": {
        name: "HealingBonus",
        nameLocale: 1092,
        
        description: null,
        
        
        badge: HealingBonus_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"p","title":761,"type":"floatPercentageInput"},
            
        ],
    },
    
    "BaseDmg": {
        name: "BaseDmg",
        nameLocale: 461,
        
        description: 1607,
        
        
        badge: BaseDmg_image,
        
        genre: "Common",
        config: [
            
            {"default":0.0,"name":"value","title":761,"type":"floatInput"},
            
        ],
    },
    
    "AlbedoTalent2": {
        name: "AlbedoTalent2",
        nameLocale: 1747,
        
        description: 1752,
        
        
        badge: getImage("Albedo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AlbedoC4": {
        name: "AlbedoC4",
        nameLocale: 1748,
        
        description: 1750,
        
        
        badge: getImage("Albedo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AlbedoC6": {
        name: "AlbedoC6",
        nameLocale: 1746,
        
        description: 1751,
        
        
        badge: getImage("Albedo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AloyTalent1": {
        name: "AloyTalent1",
        nameLocale: 455,
        
        description: 456,
        
        
        badge: getImage("Aloy"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "AratakiIttoC4": {
        name: "AratakiIttoC4",
        nameLocale: 1457,
        
        description: 1459,
        
        
        badge: getImage("Itto"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "BeidouC6": {
        name: "BeidouC6",
        nameLocale: 326,
        
        description: 328,
        
        
        badge: getImage("Beidou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "BennettQ": {
        name: "BennettQ",
        nameLocale: 1236,
        
        description: 1239,
        
        
        badge: getImage("Bennett"),
        
        genre: "Character",
        config: [
            
            {"default":800.0,"name":"base_atk","title":1241,"type":"floatInput"},
            
            {"default":true,"name":"c1","title":826,"type":"bool"},
            
            {"default":10,"max":15,"min":1,"name":"skill3","title":689,"type":"int"},
            
        ],
    },
    
    "BennettC6": {
        name: "BennettC6",
        nameLocale: 1235,
        
        description: 1240,
        
        
        badge: getImage("Bennett"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "ChongyunTalent2": {
        name: "ChongyunTalent2",
        nameLocale: 1663,
        
        description: 1665,
        
        
        badge: getImage("Chongyun"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "DionaC6G50": {
        name: "DionaC6G50",
        nameLocale: 1635,
        
        description: 1637,
        
        
        badge: getImage("Diona"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "EulaE": {
        name: "EulaE",
        nameLocale: 168,
        
        description: 170,
        
        
        badge: getImage("Eula"),
        
        genre: "Character",
        config: [
            
            {"default":9,"max":15,"min":1,"name":"skill2","title":689,"type":"int"},
            
        ],
    },
    
    "GanyuTalent2": {
        name: "GanyuTalent2",
        nameLocale: 1255,
        
        description: 1259,
        
        
        badge: getImage("Ganyu"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GanyuC1": {
        name: "GanyuC1",
        nameLocale: 1256,
        
        description: 1258,
        
        
        badge: getImage("Ganyu"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GorouE1": {
        name: "GorouE1",
        nameLocale: 150,
        
        description: 155,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
            {"default":10,"max":15,"min":1,"name":"skill2","title":154,"type":"int"},
            
        ],
    },
    
    "GorouE3": {
        name: "GorouE3",
        nameLocale: 151,
        
        description: 156,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GorouTalent1": {
        name: "GorouTalent1",
        nameLocale: 149,
        
        description: 158,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "GorouC6": {
        name: "GorouC6",
        nameLocale: 152,
        
        description: 157,
        
        
        badge: getImage("Gorou"),
        
        genre: "Character",
        config: [
            
            {"default":1,"max":3,"min":1,"name":"level","title":1801,"type":"int"},
            
        ],
    },
    
    "HuTaoTalent1": {
        name: "HuTaoTalent1",
        nameLocale: 1434,
        
        description: 1436,
        
        
        badge: getImage("Hutao"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "JeanC4": {
        name: "JeanC4",
        nameLocale: 1248,
        
        description: 1250,
        
        
        badge: getImage("Qin"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KaedeharaKazuhaTalent2": {
        name: "KaedeharaKazuhaTalent2",
        nameLocale: 1038,
        
        description: 1042,
        
        
        badge: getImage("Kazuha"),
        
        genre: "Character",
        config: [
            
            {"default":"Electro","name":"element","title":668,"type":"element4"},
            
            {"default":800.0,"name":"em","title":71,"type":"floatInput"},
            
        ],
    },
    
    "KaedeharaKazuhaC2": {
        name: "KaedeharaKazuhaC2",
        nameLocale: 1037,
        
        description: 1041,
        
        
        badge: getImage("Kazuha"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KamisatoAyakaC4": {
        name: "KamisatoAyakaC4",
        nameLocale: 1351,
        
        description: 1402,
        
        
        badge: getImage("Ayaka"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KleeC2": {
        name: "KleeC2",
        nameLocale: 376,
        
        description: 378,
        
        
        badge: getImage("Klee"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KleeC6": {
        name: "KleeC6",
        nameLocale: 375,
        
        description: 379,
        
        
        badge: getImage("Klee"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "KujouSaraEOrQ": {
        name: "KujouSaraEOrQ",
        nameLocale: 108,
        
        description: 111,
        
        
        badge: getImage("Sara"),
        
        genre: "Character",
        config: [
            
            {"default":700.0,"name":"base_atk","title":112,"type":"floatInput"},
            
            {"default":false,"name":"c6","title":829,"type":"bool"},
            
            {"default":10,"max":15,"min":1,"name":"skill2","title":12,"type":"int"},
            
        ],
    },
    
    "LisaTalent2": {
        name: "LisaTalent2",
        nameLocale: 99,
        
        description: 101,
        
        
        badge: getImage("Lisa"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "MonaQ": {
        name: "MonaQ",
        nameLocale: 1465,
        
        description: 1468,
        
        
        badge: getImage("Mona"),
        
        genre: "Character",
        config: [
            
            {"default":9,"max":15,"min":1,"name":"skill3","title":18,"type":"int"},
            
            {"default":false,"name":"c4","title":828,"type":"bool"},
            
        ],
    },
    
    "MonaC1": {
        name: "MonaC1",
        nameLocale: 1466,
        
        description: 1469,
        
        
        badge: getImage("Mona"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "NingguangTalent2": {
        name: "NingguangTalent2",
        nameLocale: 286,
        
        description: 288,
        
        
        badge: getImage("Ningguang"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "RaidenShogunE": {
        name: "RaidenShogunE",
        nameLocale: 1781,
        
        description: 1784,
        
        
        badge: getImage("Shougun"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"skill2","title":1783,"type":"int"},
            
            {"default":80,"max":100,"min":20,"name":"energy","title":355,"type":"int"},
            
        ],
    },
    
    "RaidenShogunC4": {
        name: "RaidenShogunC4",
        nameLocale: 1780,
        
        description: 1785,
        
        
        badge: getImage("Shougun"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "RazorC4": {
        name: "RazorC4",
        nameLocale: 1774,
        
        description: 1776,
        
        
        badge: getImage("Razor"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "RosariaTalent2": {
        name: "RosariaTalent2",
        nameLocale: 1410,
        
        description: 1413,
        
        
        badge: getImage("Rosaria"),
        
        genre: "Character",
        config: [
            
            {"default":70.0,"name":"crit","title":1414,"type":"floatPercentageInput"},
            
        ],
    },
    
    "RosariaC6": {
        name: "RosariaC6",
        nameLocale: 1409,
        
        description: 1412,
        
        
        badge: getImage("Rosaria"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "ShenheE": {
        name: "ShenheE",
        nameLocale: 1286,
        
        description: 1292,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":3000.0,"name":"atk","title":1297,"type":"floatInput"},
            
            {"default":8,"max":15,"min":1,"name":"skill2","title":1291,"type":"int"},
            
        ],
    },
    
    "ShenheQ": {
        name: "ShenheQ",
        nameLocale: 1288,
        
        description: 1294,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"skill3","title":1293,"type":"int"},
            
        ],
    },
    
    "ShenheTalent1": {
        name: "ShenheTalent1",
        nameLocale: 1287,
        
        description: 1295,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"c2","title":827,"type":"bool"},
            
        ],
    },
    
    "ShenheTalent2": {
        name: "ShenheTalent2",
        nameLocale: 1289,
        
        description: 1296,
        
        
        badge: getImage("Shenhe"),
        
        genre: "Character",
        config: [
            
            {"default":0,"name":"t","options":["点按","长按"],"title":690,"type":"option"},
            
        ],
    },
    
    "SucroseTalent1": {
        name: "SucroseTalent1",
        nameLocale: 1319,
        
        description: 1322,
        
        
        badge: getImage("Sucrose"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "SucroseTalent2": {
        name: "SucroseTalent2",
        nameLocale: 1317,
        
        description: 1323,
        
        
        badge: getImage("Sucrose"),
        
        genre: "Character",
        config: [
            
            {"default":200.0,"name":"em","title":1324,"type":"floatInput"},
            
        ],
    },
    
    "SucroseC6": {
        name: "SucroseC6",
        nameLocale: 1318,
        
        description: 1321,
        
        
        badge: getImage("Sucrose"),
        
        genre: "Character",
        config: [
            
            {"default":"Electro","name":"element","title":673,"type":"element4"},
            
        ],
    },
    
    "ThomaTalent1": {
        name: "ThomaTalent1",
        nameLocale: 664,
        
        description: 667,
        
        
        badge: getImage("Tohma"),
        
        genre: "Character",
        config: [
            
            {"default":2.0,"max":5.0,"min":0.0,"name":"stack","title":369,"type":"float"},
            
        ],
    },
    
    "ThomaC6": {
        name: "ThomaC6",
        nameLocale: 663,
        
        description: 666,
        
        
        badge: getImage("Tohma"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "VentiC2": {
        name: "VentiC2",
        nameLocale: 1141,
        
        description: 1143,
        
        
        badge: getImage("Venti"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"levitating","title":1495,"type":"bool"},
            
        ],
    },
    
    "VentiC6": {
        name: "VentiC6",
        nameLocale: 1140,
        
        description: 1144,
        
        
        badge: getImage("Venti"),
        
        genre: "Character",
        config: [
            
            {"default":true,"name":"is_convert","title":354,"type":"bool"},
            
            {"default":"Electro","name":"element","title":1612,"type":"element4"},
            
        ],
    },
    
    "XianglingTalent2": {
        name: "XianglingTalent2",
        nameLocale: 1839,
        
        description: 1843,
        
        
        badge: getImage("Xiangling"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XianglingC1": {
        name: "XianglingC1",
        nameLocale: 1837,
        
        description: 1841,
        
        
        badge: getImage("Xiangling"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XianglingC6": {
        name: "XianglingC6",
        nameLocale: 1838,
        
        description: 1842,
        
        
        badge: getImage("Xiangling"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XingqiuC2": {
        name: "XingqiuC2",
        nameLocale: 1511,
        
        description: 1513,
        
        
        badge: getImage("Xingqiu"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XinyanC4": {
        name: "XinyanC4",
        nameLocale: 1618,
        
        description: 1622,
        
        
        badge: getImage("Xinyan"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "XinyanTalent2": {
        name: "XinyanTalent2",
        nameLocale: 1619,
        
        description: 1623,
        
        
        badge: getImage("Xinyan"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "YaeMikoC4": {
        name: "YaeMikoC4",
        nameLocale: 242,
        
        description: 244,
        
        
        badge: getImage("Yae"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "YoimiyaTalent2": {
        name: "YoimiyaTalent2",
        nameLocale: 565,
        
        description: 567,
        
        
        badge: getImage("Yoimiya"),
        
        genre: "Character",
        config: [
            
            {"default":0,"max":10,"min":0,"name":"talent1_stack","title":42,"type":"int"},
            
        ],
    },
    
    "YunjinQ": {
        name: "YunjinQ",
        nameLocale: 138,
        
        description: 140,
        
        
        badge: getImage("Yunjin"),
        
        genre: "Character",
        config: [
            
            {"default":10,"max":15,"min":1,"name":"skill3","title":18,"type":"int"},
            
            {"default":2000.0,"name":"def","title":142,"type":"floatInput"},
            
            {"default":true,"name":"talent2","title":7,"type":"bool"},
            
            {"default":4,"max":4,"min":1,"name":"ele_count","title":1719,"type":"int"},
            
        ],
    },
    
    "YunjinC2": {
        name: "YunjinC2",
        nameLocale: 137,
        
        description: 141,
        
        
        badge: getImage("Yunjin"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "ZhongliShield": {
        name: "ZhongliShield",
        nameLocale: 1701,
        
        description: 1703,
        
        
        badge: getImage("Zhongli"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "YelanTalent2": {
        name: "YelanTalent2",
        nameLocale: 480,
        
        description: 484,
        
        
        badge: getImage("Yelan"),
        
        genre: "Character",
        config: [
            
            {"default":14,"max":14,"min":0,"name":"secs","title":1397,"type":"int"},
            
        ],
    },
    
    "YelanC4": {
        name: "YelanC4",
        nameLocale: 481,
        
        description: 483,
        
        
        badge: getImage("Yelan"),
        
        genre: "Character",
        config: [
            
            {"default":4,"max":4,"min":1,"name":"count","title":1049,"type":"int"},
            
        ],
    },
    
    "KamisatoAyatoQ": {
        name: "KamisatoAyatoQ",
        nameLocale: 1345,
        
        description: 1348,
        
        
        badge: getImage("Ayato"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"skill_level","title":1347,"type":"int"},
            
        ],
    },
    
    "ShikanoinHeizouTalent2": {
        name: "ShikanoinHeizouTalent2",
        nameLocale: 1863,
        
        description: 1865,
        
        
        badge: getImage("Heizo"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "TighnariC4": {
        name: "TighnariC4",
        nameLocale: 715,
        
        description: 717,
        
        
        badge: getImage("Tighnari"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"after_reaction","title":1654,"type":"bool"},
            
        ],
    },
    
    "DoriC4": {
        name: "DoriC4",
        nameLocale: 477,
        
        description: 478,
        
        
        badge: getImage("Dori"),
        
        genre: "Character",
        config: [
            
            {"default":false,"name":"hp_below50","title":1266,"type":"bool"},
            
            {"default":true,"name":"energy_below50","title":236,"type":"bool"},
            
        ],
    },
    
    "NilouTalent1": {
        name: "NilouTalent1",
        nameLocale: 532,
        
        description: 468,
        
        
        badge: getImage("Nilou"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "NilouTalent2": {
        name: "NilouTalent2",
        nameLocale: 533,
        
        description: 458,
        
        
        badge: getImage("Nilou"),
        
        genre: "Character",
        config: [
            
            {"default":60000.0,"name":"hp","title":535,"type":"floatInput"},
            
        ],
    },
    
    "CandaceQ": {
        name: "CandaceQ",
        nameLocale: 446,
        
        description: 1549,
        
        
        badge: getImage("Candace"),
        
        genre: "Character",
        config: [
            
        ],
    },
    
    "CandaceTalent2": {
        name: "CandaceTalent2",
        nameLocale: 445,
        
        description: 466,
        
        
        badge: getImage("Candace"),
        
        genre: "Character",
        config: [
            
            {"default":30000.0,"name":"hp","title":447,"type":"floatInput"},
            
        ],
    },
    
    "NahidaTalent1": {
        name: "NahidaTalent1",
        nameLocale: 1390,
        
        description: 794,
        
        
        badge: getImage("Nahida"),
        
        genre: "Character",
        config: [
            
            {"default":1000.0,"max":3000.0,"min":0.0,"name":"max_em","title":1728,"type":"float"},
            
        ],
    },
    
    "FaruzanQ": {
        name: "FaruzanQ",
        nameLocale: 1097,
        
        description: 1233,
        
        
        badge: getImage("Faruzan"),
        
        genre: "Character",
        config: [
            
            {"default":650,"max":1000,"min":0,"name":"base_atk","title":1098,"type":"int"},
            
            {"default":10,"max":15,"min":1,"name":"q_level","title":18,"type":"int"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate_q1","title":43,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate_q2","title":39,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate_talent2","title":20,"type":"float"},
            
            {"default":false,"name":"enable_c6","title":246,"type":"bool"},
            
        ],
    },
    
    "Mika": {
        name: "Mika",
        nameLocale: 1381,
        
        description: 823,
        
        
        badge: getImage("Mika"),
        
        genre: "Character",
        config: [
            
            {"default":3.0,"max":5.0,"min":0.0,"name":"stack_talent2","title":185,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate_c6","title":248,"type":"float"},
            
        ],
    },
    
    "KavehQ": {
        name: "KavehQ",
        nameLocale: 346,
        
        description: 1725,
        
        
        badge: getImage("Kaveh"),
        
        genre: "Character",
        config: [
            
            {"default":8,"max":15,"min":1,"name":"q_level","title":347,"type":"int"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":1071,"type":"float"},
            
        ],
    },
    
    "BaizhuTalent2": {
        name: "BaizhuTalent2",
        nameLocale: 1301,
        
        description: 357,
        
        
        badge: getImage("Baizhuer"),
        
        genre: "Character",
        config: [
            
            {"default":50000.0,"max":50000.0,"min":0.0,"name":"hp","title":1303,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":1071,"type":"float"},
            
        ],
    },
    
    "BaizhuC4": {
        name: "BaizhuC4",
        nameLocale: 1302,
        
        description: 795,
        
        
        badge: getImage("Baizhuer"),
        
        genre: "Character",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":1071,"type":"float"},
            
        ],
    },
    
    "FreedomSworn": {
        name: "FreedomSworn",
        nameLocale: 1448,
        
        description: 22,
        
        
        badge: getImageW("Sword_Widsith"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1384,"type":"intInput"},
            
        ],
    },
    
    "SongOfBrokenPines": {
        name: "SongOfBrokenPines",
        nameLocale: 1032,
        
        description: 23,
        
        
        badge: getImageW("Claymore_Widsith"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1384,"type":"intInput"},
            
        ],
    },
    
    "WolfsGravestone": {
        name: "WolfsGravestone",
        nameLocale: 1209,
        
        description: 735,
        
        
        badge: getImageW("Claymore_Wolfmound"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1384,"type":"intInput"},
            
        ],
    },
    
    "ThrillingTalesOfDragonSlayers": {
        name: "ThrillingTalesOfDragonSlayers",
        nameLocale: 1563,
        
        description: 171,
        
        
        badge: getImageW("Catalyst_Pulpfic"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1384,"type":"intInput"},
            
        ],
    },
    
    "ElegyOfTheEnd": {
        name: "ElegyOfTheEnd",
        nameLocale: 1396,
        
        description: 337,
        
        
        badge: getImageW("Bow_Widsith"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1384,"type":"intInput"},
            
        ],
    },
    
    "HakushinRing": {
        name: "HakushinRing",
        nameLocale: 1308,
        
        description: 1058,
        
        
        badge: getImageW("Catalyst_Bakufu"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1384,"type":"intInput"},
            
            {"default":"Electro","name":"element","title":189,"type":"element8"},
            
        ],
    },
    
    "SapwoodBlade": {
        name: "SapwoodBlade",
        nameLocale: 352,
        
        description: 701,
        
        
        badge: getImageW("Sword_Arakalari"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1384,"type":"intInput"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":617,"type":"float"},
            
        ],
    },
    
    "Moonpiercer": {
        name: "Moonpiercer",
        nameLocale: 1583,
        
        description: 702,
        
        
        badge: getImageW("Pole_Arakalari"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1384,"type":"intInput"},
            
        ],
    },
    
    "XiphosMoonlight": {
        name: "XiphosMoonlight",
        nameLocale: 1534,
        
        description: 1528,
        
        
        badge: getImageW("Sword_Pleroma"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1384,"type":"intInput"},
            
            {"default":900.0,"name":"em","title":228,"type":"floatInput"},
            
        ],
    },
    
    "MakhairaAquamarine": {
        name: "MakhairaAquamarine",
        nameLocale: 1228,
        
        description: 32,
        
        
        badge: getImageW("Claymore_Pleroma"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1384,"type":"intInput"},
            
            {"default":900.0,"name":"em","title":228,"type":"floatInput"},
            
        ],
    },
    
    "KeyOfKhajNisut": {
        name: "KeyOfKhajNisut",
        nameLocale: 441,
        
        description: 581,
        
        
        badge: getImageW("Sword_Deshret"),
        
        genre: "Weapon",
        config: [
            
            {"default":1,"max":5,"min":1,"name":"refine","title":1384,"type":"intInput"},
            
            {"default":20000.0,"name":"hp","title":1260,"type":"floatInput"},
            
        ],
    },
    
    "ResonancePyro2": {
        name: "ResonancePyro2",
        nameLocale: 197,
        
        description: 724,
        
        
        badge: ResonancePyro2_image,
        
        genre: "Resonance",
        config: [
            
        ],
    },
    
    "ResonanceCryo2": {
        name: "ResonanceCryo2",
        nameLocale: 198,
        
        description: 719,
        
        
        badge: ResonanceCryo2_image,
        
        genre: "Resonance",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":617,"type":"float"},
            
        ],
    },
    
    "ResonanceGeo2": {
        name: "ResonanceGeo2",
        nameLocale: 195,
        
        description: 694,
        
        
        badge: ResonanceGeo2_image,
        
        genre: "Resonance",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate1","title":745,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate2","title":747,"type":"float"},
            
        ],
    },
    
    "ResonanceHydro2": {
        name: "ResonanceHydro2",
        nameLocale: 196,
        
        description: 1162,
        
        
        badge: ResonanceHydro2_image,
        
        genre: "Resonance",
        config: [
            
        ],
    },
    
    "ResonanceDendro2": {
        name: "ResonanceDendro2",
        nameLocale: 199,
        
        description: 230,
        
        
        badge: ResonanceDendro2_image,
        
        genre: "Resonance",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate1","title":745,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate2","title":747,"type":"float"},
            
        ],
    },
    
    "Instructor4": {
        name: "Instructor4",
        nameLocale: 755,
        
        description: 1554,
        
        
        badge: getImageA("UI_RelicIcon_10007_4"),
        
        genre: "Artifact",
        config: [
            
        ],
    },
    
    "NoblesseOblige4": {
        name: "NoblesseOblige4",
        nameLocale: 815,
        
        description: 790,
        
        
        badge: getImageA("UI_RelicIcon_15007_4"),
        
        genre: "Artifact",
        config: [
            
        ],
    },
    
    "ArchaicPetra4": {
        name: "ArchaicPetra4",
        nameLocale: 647,
        
        description: 1488,
        
        
        badge: getImageA("UI_RelicIcon_15014_4"),
        
        genre: "Artifact",
        config: [
            
            {"default":"Electro","name":"element","title":1399,"type":"element4"},
            
        ],
    },
    
    "ViridescentVenerer4": {
        name: "ViridescentVenerer4",
        nameLocale: 1419,
        
        description: 1050,
        
        
        badge: getImageA("UI_RelicIcon_15002_4"),
        
        genre: "Artifact",
        config: [
            
            {"default":"Electro","name":"element","title":668,"type":"element4"},
            
        ],
    },
    
    "TenacityOfTheMillelith4": {
        name: "TenacityOfTheMillelith4",
        nameLocale: 335,
        
        description: 205,
        
        
        badge: getImageA("UI_RelicIcon_15017_4"),
        
        genre: "Artifact",
        config: [
            
        ],
    },
    
    "DeepwoodMemories4": {
        name: "DeepwoodMemories4",
        nameLocale: 1130,
        
        description: 212,
        
        
        badge: getImageA("UI_RelicIcon_15025_4"),
        
        genre: "Artifact",
        config: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":617,"type":"float"},
            
        ],
    },
    
}