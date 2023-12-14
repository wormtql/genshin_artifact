const template = "https://upload-bbs.mihoyo.com/game_record/genshin/equip/#.png"
const newTemplate = "https://act-webstatic.mihoyo.com/hk4e/e20200928calculate/item_icon_uceddf/#.png"
const getIcon = name => template.replace("#", name)
const getHash = md5 => newTemplate.replace("#", md5)

export default {
    
    "adventurer": {
        eng: "adventurer",
        name2: "Adventurer",
        nameLocale: 271,
        minStar: 1,
        maxStar: 3,
    
    effect2: 1293,
    
    effect4: 637,
    

        flower: {
            text: 272,
            url: getIcon("UI_RelicIcon_10010_4")
            },
        feather: {
            text: 274,
            url: getIcon("UI_RelicIcon_10010_2")
            },
        sand: {
            text: 275,
            url: getIcon("UI_RelicIcon_10010_5")
            },
        cup: {
            text: 276,
            url: getIcon("UI_RelicIcon_10010_1")
            },
        head: {
            text: 273,
            url: getIcon("UI_RelicIcon_10010_3")
            },
        config4: [
            
        ],
    },
    
    "archaicPetra": {
        eng: "archaicPetra",
        name2: "ArchaicPetra",
        nameLocale: 663,
        minStar: 4,
        maxStar: 5,
    
    effect2: 1510,
    
    effect4: 1525,
    

        flower: {
            text: 1363,
            url: getIcon("UI_RelicIcon_15014_4")
            },
        feather: {
            text: 612,
            url: getIcon("UI_RelicIcon_15014_2")
            },
        sand: {
            text: 841,
            url: getIcon("UI_RelicIcon_15014_5")
            },
        cup: {
            text: 613,
            url: getIcon("UI_RelicIcon_15014_1")
            },
        head: {
            text: 101,
            url: getIcon("UI_RelicIcon_15014_3")
            },
        config4: [
            
            {"default":"Electro","name":"element","title":201,"type":"element4"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate","title":768,"type":"float"},
            
        ],
    },
    
    "berserker": {
        eng: "berserker",
        name2: "Berserker",
        nameLocale: 670,
        minStar: 3,
        maxStar: 4,
    
    effect2: 1016,
    
    effect4: 1297,
    

        flower: {
            text: 673,
            url: getIcon("UI_RelicIcon_10005_4")
            },
        feather: {
            text: 672,
            url: getIcon("UI_RelicIcon_10005_2")
            },
        sand: {
            text: 671,
            url: getIcon("UI_RelicIcon_10005_5")
            },
        cup: {
            text: 674,
            url: getIcon("UI_RelicIcon_10005_1")
            },
        head: {
            text: 675,
            url: getIcon("UI_RelicIcon_10005_3")
            },
        config4: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate","title":768,"type":"float"},
            
        ],
    },
    
    "blizzardStrayer": {
        eng: "blizzardStrayer",
        name2: "BlizzardStrayer",
        nameLocale: 290,
        minStar: 4,
        maxStar: 5,
    
    effect2: 1509,
    
    effect4: 755,
    

        flower: {
            text: 363,
            url: getIcon("UI_RelicIcon_14001_4")
            },
        feather: {
            text: 735,
            url: getIcon("UI_RelicIcon_14001_2")
            },
        sand: {
            text: 289,
            url: getIcon("UI_RelicIcon_14001_5")
            },
        cup: {
            text: 1698,
            url: getIcon("UI_RelicIcon_14001_1")
            },
        head: {
            text: 1357,
            url: getIcon("UI_RelicIcon_14001_3")
            },
        config4: [
            
            {"default":0.0,"max":0.4,"min":0.0,"name":"critical_bonus","title":1409,"type":"float"},
            
        ],
    },
    
    "bloodstainedChivalry": {
        eng: "bloodstainedChivalry",
        name2: "BloodstainedChivalry",
        nameLocale: 1070,
        minStar: 4,
        maxStar: 5,
    
    effect2: 1695,
    
    effect4: 308,
    

        flower: {
            text: 1068,
            url: getIcon("UI_RelicIcon_15008_4")
            },
        feather: {
            text: 1071,
            url: getIcon("UI_RelicIcon_15008_2")
            },
        sand: {
            text: 1890,
            url: getIcon("UI_RelicIcon_15008_5")
            },
        cup: {
            text: 1072,
            url: getIcon("UI_RelicIcon_15008_1")
            },
        head: {
            text: 1069,
            url: getIcon("UI_RelicIcon_15008_3")
            },
        config4: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate","title":768,"type":"float"},
            
        ],
    },
    
    "braveHeart": {
        eng: "braveHeart",
        name2: "BraveHeart",
        nameLocale: 328,
        minStar: 3,
        maxStar: 4,
    
    effect2: 745,
    
    effect4: 597,
    

        flower: {
            text: 330,
            url: getIcon("UI_RelicIcon_10002_4")
            },
        feather: {
            text: 333,
            url: getIcon("UI_RelicIcon_10002_2")
            },
        sand: {
            text: 331,
            url: getIcon("UI_RelicIcon_10002_5")
            },
        cup: {
            text: 332,
            url: getIcon("UI_RelicIcon_10002_1")
            },
        head: {
            text: 329,
            url: getIcon("UI_RelicIcon_10002_3")
            },
        config4: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate","title":768,"type":"float"},
            
        ],
    },
    
    "crimsonWitch": {
        eng: "crimsonWitch",
        name2: "CrimsonWitchOfFlames",
        nameLocale: 1209,
        minStar: 4,
        maxStar: 5,
    
    effect2: 1512,
    
    effect4: 1643,
    

        flower: {
            text: 1898,
            url: getIcon("UI_RelicIcon_15006_4")
            },
        feather: {
            text: 1896,
            url: getIcon("UI_RelicIcon_15006_2")
            },
        sand: {
            text: 1899,
            url: getIcon("UI_RelicIcon_15006_5")
            },
        cup: {
            text: 1897,
            url: getIcon("UI_RelicIcon_15006_1")
            },
        head: {
            text: 1218,
            url: getIcon("UI_RelicIcon_15006_3")
            },
        config4: [
            
            {"default":0.0,"max":3.0,"min":0.0,"name":"level","title":769,"type":"float"},
            
        ],
    },
    
    "DeepwoodMemories": {
        eng: "DeepwoodMemories",
        name2: "DeepwoodMemories",
        nameLocale: 1156,
        minStar: 4,
        maxStar: 5,
    
    effect2: 1513,
    
    effect4: 224,
    

        flower: {
            text: 1683,
            url: getIcon("UI_RelicIcon_15025_4")
            },
        feather: {
            text: 1456,
            url: getIcon("UI_RelicIcon_15025_2")
            },
        sand: {
            text: 1624,
            url: getIcon("UI_RelicIcon_15025_5")
            },
        cup: {
            text: 1684,
            url: getIcon("UI_RelicIcon_15025_1")
            },
        head: {
            text: 1044,
            url: getIcon("UI_RelicIcon_15025_3")
            },
        config4: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate","title":768,"type":"float"},
            
        ],
    },
    
    "defenderWill": {
        eng: "defenderWill",
        name2: "DefendersWill",
        nameLocale: 559,
        minStar: 3,
        maxStar: 4,
    
    effect2: 1783,
    
    effect4: 1775,
    

        flower: {
            text: 561,
            url: getIcon("UI_RelicIcon_10003_4")
            },
        feather: {
            text: 563,
            url: getIcon("UI_RelicIcon_10003_2")
            },
        sand: {
            text: 562,
            url: getIcon("UI_RelicIcon_10003_5")
            },
        cup: {
            text: 560,
            url: getIcon("UI_RelicIcon_10003_1")
            },
        head: {
            text: 564,
            url: getIcon("UI_RelicIcon_10003_3")
            },
        config4: [
            
        ],
    },
    
    "EchoesOfAnOffering": {
        eng: "EchoesOfAnOffering",
        name2: "EchoesOfAnOffering",
        nameLocale: 1053,
        minStar: 4,
        maxStar: 5,
    
    effect2: 745,
    
    effect4: 954,
    

        flower: {
            text: 1893,
            url: getIcon("UI_RelicIcon_15024_4")
            },
        feather: {
            text: 465,
            url: getIcon("UI_RelicIcon_15024_2")
            },
        sand: {
            text: 1366,
            url: getIcon("UI_RelicIcon_15024_5")
            },
        cup: {
            text: 1155,
            url: getIcon("UI_RelicIcon_15024_1")
            },
        head: {
            text: 1146,
            url: getIcon("UI_RelicIcon_15024_3")
            },
        config4: [
            
            {"default":0.5053283764473575,"max":1.0,"min":0.0,"name":"rate","title":617,"type":"float"},
            
        ],
    },
    
    "emblemOfSeveredFate": {
        eng: "emblemOfSeveredFate",
        name2: "EmblemOfSeveredFate",
        nameLocale: 1434,
        minStar: 4,
        maxStar: 5,
    
    effect2: 204,
    
    effect4: 471,
    

        flower: {
            text: 833,
            url: getIcon("UI_RelicIcon_15020_4")
            },
        feather: {
            text: 315,
            url: getIcon("UI_RelicIcon_15020_2")
            },
        sand: {
            text: 1810,
            url: getIcon("UI_RelicIcon_15020_5")
            },
        cup: {
            text: 1437,
            url: getIcon("UI_RelicIcon_15020_1")
            },
        head: {
            text: 352,
            url: getIcon("UI_RelicIcon_15020_3")
            },
        config4: [
            
        ],
    },
    
    "FlowerOfParadiseLost": {
        eng: "FlowerOfParadiseLost",
        name2: "FlowerOfParadiseLost",
        nameLocale: 118,
        minStar: 4,
        maxStar: 5,
    
    effect2: 245,
    
    effect4: 1571,
    

        flower: {
            text: 1043,
            url: getIcon("UI_RelicIcon_15028_4")
            },
        feather: {
            text: 1621,
            url: getIcon("UI_RelicIcon_15028_2")
            },
        sand: {
            text: 303,
            url: getIcon("UI_RelicIcon_15028_5")
            },
        cup: {
            text: 565,
            url: getIcon("UI_RelicIcon_15028_1")
            },
        head: {
            text: 1419,
            url: getIcon("UI_RelicIcon_15028_3")
            },
        config4: [
            
            {"default":4.0,"max":4.0,"min":0.0,"name":"stack","title":769,"type":"float"},
            
        ],
    },
    
    "gambler": {
        eng: "gambler",
        name2: "Gambler",
        nameLocale: 1628,
        minStar: 3,
        maxStar: 4,
    
    effect2: 230,
    
    effect4: 313,
    

        flower: {
            text: 1632,
            url: getIcon("UI_RelicIcon_10008_4")
            },
        feather: {
            text: 1630,
            url: getIcon("UI_RelicIcon_10008_2")
            },
        sand: {
            text: 1629,
            url: getIcon("UI_RelicIcon_10008_5")
            },
        cup: {
            text: 1633,
            url: getIcon("UI_RelicIcon_10008_1")
            },
        head: {
            text: 1631,
            url: getIcon("UI_RelicIcon_10008_3")
            },
        config4: [
            
        ],
    },
    
    "DesertPavilionChronicle": {
        eng: "DesertPavilionChronicle",
        name2: "DesertPavilionChronicle",
        nameLocale: 1115,
        minStar: 4,
        maxStar: 5,
    
    effect2: 1515,
    
    effect4: 1729,
    

        flower: {
            text: 178,
            url: getIcon("UI_RelicIcon_15027_4")
            },
        feather: {
            text: 1917,
            url: getIcon("UI_RelicIcon_15027_2")
            },
        sand: {
            text: 535,
            url: getIcon("UI_RelicIcon_15027_5")
            },
        cup: {
            text: 1685,
            url: getIcon("UI_RelicIcon_15027_1")
            },
        head: {
            text: 1136,
            url: getIcon("UI_RelicIcon_15027_3")
            },
        config4: [
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":768,"type":"float"},
            
        ],
    },
    
    "GildedDreams": {
        eng: "GildedDreams",
        name2: "GildedDreams",
        nameLocale: 1879,
        minStar: 4,
        maxStar: 5,
    
    effect2: 243,
    
    effect4: 1599,
    

        flower: {
            text: 1079,
            url: getIcon("UI_RelicIcon_15026_4")
            },
        feather: {
            text: 1565,
            url: getIcon("UI_RelicIcon_15026_2")
            },
        sand: {
            text: 1113,
            url: getIcon("UI_RelicIcon_15026_5")
            },
        cup: {
            text: 544,
            url: getIcon("UI_RelicIcon_15026_1")
            },
        head: {
            text: 1116,
            url: getIcon("UI_RelicIcon_15026_3")
            },
        config4: [
            
            {"default":0,"max":3,"min":0,"name":"same_count","title":396,"type":"int"},
            
            {"default":0,"max":3,"min":0,"name":"diff_count","title":103,"type":"int"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate","title":768,"type":"float"},
            
        ],
    },
    
    "gladiatorFinale": {
        eng: "gladiatorFinale",
        name2: "GladiatorsFinale",
        nameLocale: 1586,
        minStar: 4,
        maxStar: 5,
    
    effect2: 745,
    
    effect4: 1572,
    

        flower: {
            text: 1585,
            url: getIcon("UI_RelicIcon_15001_4")
            },
        feather: {
            text: 1584,
            url: getIcon("UI_RelicIcon_15001_2")
            },
        sand: {
            text: 1583,
            url: getIcon("UI_RelicIcon_15001_5")
            },
        cup: {
            text: 1587,
            url: getIcon("UI_RelicIcon_15001_1")
            },
        head: {
            text: 1582,
            url: getIcon("UI_RelicIcon_15001_3")
            },
        config4: [
            
        ],
    },
    
    "GoldenTroupe": {
        eng: "GoldenTroupe",
        name2: "GoldenTroupe",
        nameLocale: 1914,
        minStar: 4,
        maxStar: 5,
    
    effect2: 230,
    
    effect4: 231,
    

        flower: {
            text: 1913,
            url: getHash("fadbbf8dbba05ad1ce0daeb4ebf89413")},
        feather: {
            text: 1918,
            url: getHash("4ca24e57d1adc9f0247c6bffd164d8b7")},
        sand: {
            text: 1916,
            url: getHash("9269d47e4a4edd517042c26fe534060e")},
        cup: {
            text: 1912,
            url: getHash("4d2b22a334d4237ba2cce56aeb5bd023")},
        head: {
            text: 1915,
            url: getHash("2f69d44e2e79a05adbe0c9fe6391ea33")},
        config4: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate","title":1557,"type":"float"},
            
        ],
    },
    
    "heartOfDepth": {
        eng: "heartOfDepth",
        name2: "HeartOfDepth",
        nameLocale: 1111,
        minStar: 4,
        maxStar: 5,
    
    effect2: 1511,
    
    effect4: 791,
    

        flower: {
            text: 1880,
            url: getIcon("UI_RelicIcon_15016_4")
            },
        feather: {
            text: 1687,
            url: getIcon("UI_RelicIcon_15016_2")
            },
        sand: {
            text: 464,
            url: getIcon("UI_RelicIcon_15016_5")
            },
        cup: {
            text: 1112,
            url: getIcon("UI_RelicIcon_15016_1")
            },
        head: {
            text: 1704,
            url: getIcon("UI_RelicIcon_15016_3")
            },
        config4: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate","title":768,"type":"float"},
            
        ],
    },
    
    "huskOfOpulentDreams": {
        eng: "huskOfOpulentDreams",
        name2: "HuskOfOpulentDreams",
        nameLocale: 354,
        minStar: 4,
        maxStar: 5,
    
    effect2: 1783,
    
    effect4: 1566,
    

        flower: {
            text: 1499,
            url: getIcon("UI_RelicIcon_15021_4")
            },
        feather: {
            text: 353,
            url: getIcon("UI_RelicIcon_15021_2")
            },
        sand: {
            text: 179,
            url: getIcon("UI_RelicIcon_15021_5")
            },
        cup: {
            text: 1081,
            url: getIcon("UI_RelicIcon_15021_1")
            },
        head: {
            text: 647,
            url: getIcon("UI_RelicIcon_15021_3")
            },
        config4: [
            
            {"default":0.0,"max":4.0,"min":0.0,"name":"level","title":55,"type":"float"},
            
        ],
    },
    
    "instructor": {
        eng: "instructor",
        name2: "Instructor",
        nameLocale: 773,
        minStar: 3,
        maxStar: 4,
    
    effect2: 244,
    
    effect4: 1600,
    

        flower: {
            text: 778,
            url: getIcon("UI_RelicIcon_10007_4")
            },
        feather: {
            text: 777,
            url: getIcon("UI_RelicIcon_10007_2")
            },
        sand: {
            text: 776,
            url: getIcon("UI_RelicIcon_10007_5")
            },
        cup: {
            text: 779,
            url: getIcon("UI_RelicIcon_10007_1")
            },
        head: {
            text: 775,
            url: getIcon("UI_RelicIcon_10007_3")
            },
        config4: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate","title":768,"type":"float"},
            
        ],
    },
    
    "lavaWalker": {
        eng: "lavaWalker",
        name2: "Lavawalker",
        nameLocale: 1165,
        minStar: 4,
        maxStar: 5,
    
    effect2: 1187,
    
    effect4: 592,
    

        flower: {
            text: 1160,
            url: getIcon("UI_RelicIcon_14003_4")
            },
        feather: {
            text: 1163,
            url: getIcon("UI_RelicIcon_14003_2")
            },
        sand: {
            text: 1162,
            url: getIcon("UI_RelicIcon_14003_5")
            },
        cup: {
            text: 1164,
            url: getIcon("UI_RelicIcon_14003_1")
            },
        head: {
            text: 1161,
            url: getIcon("UI_RelicIcon_14003_3")
            },
        config4: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate","title":770,"type":"float"},
            
        ],
    },
    
    "luckyDog": {
        eng: "luckyDog",
        name2: "LuckyDog",
        nameLocale: 624,
        minStar: 1,
        maxStar: 3,
    
    effect2: 1782,
    
    effect4: 717,
    

        flower: {
            text: 627,
            url: getIcon("UI_RelicIcon_10011_4")
            },
        feather: {
            text: 629,
            url: getIcon("UI_RelicIcon_10011_2")
            },
        sand: {
            text: 626,
            url: getIcon("UI_RelicIcon_10011_5")
            },
        cup: {
            text: 625,
            url: getIcon("UI_RelicIcon_10011_1")
            },
        head: {
            text: 628,
            url: getIcon("UI_RelicIcon_10011_3")
            },
        config4: [
            
        ],
    },
    
    "maidenBeloved": {
        eng: "maidenBeloved",
        name2: "MaidenBeloved",
        nameLocale: 1561,
        minStar: 4,
        maxStar: 5,
    
    effect2: 1593,
    
    effect4: 798,
    

        flower: {
            text: 1671,
            url: getIcon("UI_RelicIcon_14004_4")
            },
        feather: {
            text: 603,
            url: getIcon("UI_RelicIcon_14004_2")
            },
        sand: {
            text: 602,
            url: getIcon("UI_RelicIcon_14004_5")
            },
        cup: {
            text: 601,
            url: getIcon("UI_RelicIcon_14004_1")
            },
        head: {
            text: 600,
            url: getIcon("UI_RelicIcon_14004_3")
            },
        config4: [
            
        ],
    },
    
    "MarechausseeHunter": {
        eng: "MarechausseeHunter",
        name2: "MarechausseeHunter",
        nameLocale: 1691,
        minStar: 4,
        maxStar: 5,
    
    effect2: 947,
    
    effect4: 645,
    

        flower: {
            text: 1244,
            url: getHash("9babba990b561f2b031d5db4145c19a9")},
        feather: {
            text: 1054,
            url: getHash("819b944729f2d5702f46d1403edba587")},
        sand: {
            text: 1562,
            url: getHash("de01dcbf2911968336afbbb61b455831")},
        cup: {
            text: 1699,
            url: getHash("b10902ae43e7f6d6619fe560829f7ba3")},
        head: {
            text: 1459,
            url: getHash("1f3958293c20e8a29f51b9f3ed259e12")},
        config4: [
            
            {"default":0.0,"max":3.0,"min":0.0,"name":"stack","title":616,"type":"float"},
            
        ],
    },
    
    "martialArtist": {
        eng: "martialArtist",
        name2: "MartialArtist",
        nameLocale: 1085,
        minStar: 3,
        maxStar: 4,
    
    effect2: 948,
    
    effect4: 792,
    

        flower: {
            text: 1088,
            url: getIcon("UI_RelicIcon_10006_4")
            },
        feather: {
            text: 1089,
            url: getIcon("UI_RelicIcon_10006_2")
            },
        sand: {
            text: 1087,
            url: getIcon("UI_RelicIcon_10006_5")
            },
        cup: {
            text: 1090,
            url: getIcon("UI_RelicIcon_10006_1")
            },
        head: {
            text: 1086,
            url: getIcon("UI_RelicIcon_10006_3")
            },
        config4: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate","title":768,"type":"float"},
            
        ],
    },
    
    "noblesseOblige": {
        eng: "noblesseOblige",
        name2: "NoblesseOblige",
        nameLocale: 834,
        minStar: 4,
        maxStar: 5,
    
    effect2: 236,
    
    effect4: 808,
    

        flower: {
            text: 570,
            url: getIcon("UI_RelicIcon_15007_4")
            },
        feather: {
            text: 569,
            url: getIcon("UI_RelicIcon_15007_2")
            },
        sand: {
            text: 572,
            url: getIcon("UI_RelicIcon_15007_5")
            },
        cup: {
            text: 575,
            url: getIcon("UI_RelicIcon_15007_1")
            },
        head: {
            text: 578,
            url: getIcon("UI_RelicIcon_15007_3")
            },
        config4: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate","title":768,"type":"float"},
            
        ],
    },
    
    "NymphsDream": {
        eng: "NymphsDream",
        name2: "NymphsDream",
        nameLocale: 1098,
        minStar: 4,
        maxStar: 5,
    
    effect2: 1511,
    
    effect4: 938,
    

        flower: {
            text: 817,
            url: getIcon("UI_RelicIcon_15029_4")
            },
        feather: {
            text: 462,
            url: getIcon("UI_RelicIcon_15029_2")
            },
        sand: {
            text: 1100,
            url: getIcon("UI_RelicIcon_15029_5")
            },
        cup: {
            text: 334,
            url: getIcon("UI_RelicIcon_15029_1")
            },
        head: {
            text: 662,
            url: getIcon("UI_RelicIcon_15029_3")
            },
        config4: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"w1","title":61,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"w2","title":135,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"w3","title":85,"type":"float"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"rate","title":634,"type":"float"},
            
        ],
    },
    
    "oceanHuedClam": {
        eng: "oceanHuedClam",
        name2: "OceanHuedClam",
        nameLocale: 1151,
        minStar: 4,
        maxStar: 5,
    
    effect2: 1120,
    
    effect4: 1567,
    

        flower: {
            text: 1150,
            url: getIcon("UI_RelicIcon_15022_4")
            },
        feather: {
            text: 1159,
            url: getIcon("UI_RelicIcon_15022_2")
            },
        sand: {
            text: 1398,
            url: getIcon("UI_RelicIcon_15022_5")
            },
        cup: {
            text: 1344,
            url: getIcon("UI_RelicIcon_15022_1")
            },
        head: {
            text: 1153,
            url: getIcon("UI_RelicIcon_15022_3")
            },
        config4: [
            
        ],
    },
    
    "paleFlame": {
        eng: "paleFlame",
        name2: "PaleFlame",
        nameLocale: 1484,
        minStar: 4,
        maxStar: 5,
    
    effect2: 1694,
    
    effect4: 219,
    

        flower: {
            text: 824,
            url: getIcon("UI_RelicIcon_15018_4")
            },
        feather: {
            text: 1623,
            url: getIcon("UI_RelicIcon_15018_2")
            },
        sand: {
            text: 200,
            url: getIcon("UI_RelicIcon_15018_5")
            },
        cup: {
            text: 1641,
            url: getIcon("UI_RelicIcon_15018_1")
            },
        head: {
            text: 436,
            url: getIcon("UI_RelicIcon_15018_3")
            },
        config4: [
            
            {"default":0.0,"max":2.0,"min":0.0,"name":"avg_level","title":762,"type":"float"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"full_rate","title":1178,"type":"float"},
            
        ],
    },
    
    "prayersForDestiny": {
        eng: "prayersForDestiny",
        name2: "PrayersForDestiny",
        nameLocale: 1387,
        minStar: 3,
        maxStar: 4,
    effect1: 379,
    
    
    
    

        
        
        
        
        head: {
            text: 1388,
            url: getIcon("UI_RelicIcon_15010_3")
            },
        config4: [
            
        ],
    },
    
    "prayersForIllumination": {
        eng: "prayersForIllumination",
        name2: "PrayersForIllumination",
        nameLocale: 1389,
        minStar: 3,
        maxStar: 4,
    effect1: 380,
    
    
    
    

        
        
        
        
        head: {
            text: 1390,
            url: getIcon("UI_RelicIcon_15009_3")
            },
        config4: [
            
        ],
    },
    
    "prayersForWisdom": {
        eng: "prayersForWisdom",
        name2: "PrayersForWisdom",
        nameLocale: 1395,
        minStar: 3,
        maxStar: 4,
    effect1: 381,
    
    
    
    

        
        
        
        
        head: {
            text: 1396,
            url: getIcon("UI_RelicIcon_15011_3")
            },
        config4: [
            
        ],
    },
    
    "prayersToSpringtime": {
        eng: "prayersToSpringtime",
        name2: "PrayersToSpringtime",
        nameLocale: 1385,
        minStar: 3,
        maxStar: 4,
    effect1: 378,
    
    
    
    

        
        
        
        
        head: {
            text: 1386,
            url: getIcon("UI_RelicIcon_15013_3")
            },
        config4: [
            
        ],
    },
    
    "resolutionOfSojourner": {
        eng: "resolutionOfSojourner",
        name2: "ResolutionOfSojourner",
        nameLocale: 1550,
        minStar: 3,
        maxStar: 4,
    
    effect2: 745,
    
    effect4: 1732,
    

        flower: {
            text: 760,
            url: getIcon("UI_RelicIcon_10001_4")
            },
        feather: {
            text: 644,
            url: getIcon("UI_RelicIcon_10001_2")
            },
        sand: {
            text: 1690,
            url: getIcon("UI_RelicIcon_10001_5")
            },
        cup: {
            text: 640,
            url: getIcon("UI_RelicIcon_10001_1")
            },
        head: {
            text: 668,
            url: getIcon("UI_RelicIcon_10001_3")
            },
        config4: [
            
        ],
    },
    
    "retracingBolide": {
        eng: "retracingBolide",
        name2: "RetracingBolide",
        nameLocale: 1689,
        minStar: 4,
        maxStar: 5,
    
    effect2: 713,
    
    effect4: 481,
    

        flower: {
            text: 487,
            url: getIcon("UI_RelicIcon_15015_4")
            },
        feather: {
            text: 490,
            url: getIcon("UI_RelicIcon_15015_2")
            },
        sand: {
            text: 486,
            url: getIcon("UI_RelicIcon_15015_5")
            },
        cup: {
            text: 489,
            url: getIcon("UI_RelicIcon_15015_1")
            },
        head: {
            text: 488,
            url: getIcon("UI_RelicIcon_15015_3")
            },
        config4: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate","title":715,"type":"float"},
            
        ],
    },
    
    "scholar": {
        eng: "scholar",
        name2: "Scholar",
        nameLocale: 552,
        minStar: 3,
        maxStar: 4,
    
    effect2: 204,
    
    effect4: 1522,
    

        flower: {
            text: 553,
            url: getIcon("UI_RelicIcon_10012_4")
            },
        feather: {
            text: 556,
            url: getIcon("UI_RelicIcon_10012_2")
            },
        sand: {
            text: 555,
            url: getIcon("UI_RelicIcon_10012_5")
            },
        cup: {
            text: 554,
            url: getIcon("UI_RelicIcon_10012_1")
            },
        head: {
            text: 557,
            url: getIcon("UI_RelicIcon_10012_3")
            },
        config4: [
            
        ],
    },
    
    "shimenawaReminiscence": {
        eng: "shimenawaReminiscence",
        name2: "ShimenawasReminiscence",
        nameLocale: 1686,
        minStar: 4,
        maxStar: 5,
    
    effect2: 745,
    
    effect4: 799,
    

        flower: {
            text: 1448,
            url: getIcon("UI_RelicIcon_15019_4")
            },
        feather: {
            text: 657,
            url: getIcon("UI_RelicIcon_15019_2")
            },
        sand: {
            text: 1045,
            url: getIcon("UI_RelicIcon_15019_5")
            },
        cup: {
            text: 1365,
            url: getIcon("UI_RelicIcon_15019_1")
            },
        head: {
            text: 826,
            url: getIcon("UI_RelicIcon_15019_3")
            },
        config4: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate","title":768,"type":"float"},
            
        ],
    },
    
    "tenacityOfTheMillelith": {
        eng: "tenacityOfTheMillelith",
        name2: "TenacityOfTheMillelith",
        nameLocale: 347,
        minStar: 4,
        maxStar: 5,
    
    effect2: 1299,
    
    effect4: 218,
    

        flower: {
            text: 335,
            url: getIcon("UI_RelicIcon_15017_4")
            },
        feather: {
            text: 845,
            url: getIcon("UI_RelicIcon_15017_2")
            },
        sand: {
            text: 1740,
            url: getIcon("UI_RelicIcon_15017_5")
            },
        cup: {
            text: 1341,
            url: getIcon("UI_RelicIcon_15017_1")
            },
        head: {
            text: 599,
            url: getIcon("UI_RelicIcon_15017_3")
            },
        config4: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate","title":768,"type":"float"},
            
        ],
    },
    
    "exile": {
        eng: "exile",
        name2: "TheExile",
        nameLocale: 1129,
        minStar: 3,
        maxStar: 4,
    
    effect2: 204,
    
    effect4: 807,
    

        flower: {
            text: 1132,
            url: getIcon("UI_RelicIcon_10009_4")
            },
        feather: {
            text: 1131,
            url: getIcon("UI_RelicIcon_10009_2")
            },
        sand: {
            text: 1134,
            url: getIcon("UI_RelicIcon_10009_5")
            },
        cup: {
            text: 1130,
            url: getIcon("UI_RelicIcon_10009_1")
            },
        head: {
            text: 1133,
            url: getIcon("UI_RelicIcon_10009_3")
            },
        config4: [
            
        ],
    },
    
    "thunderingFury": {
        eng: "thunderingFury",
        name2: "ThunderingFury",
        nameLocale: 545,
        minStar: 4,
        maxStar: 5,
    
    effect2: 1514,
    
    effect4: 1642,
    

        flower: {
            text: 1831,
            url: getIcon("UI_RelicIcon_15005_4")
            },
        feather: {
            text: 1821,
            url: getIcon("UI_RelicIcon_15005_2")
            },
        sand: {
            text: 1830,
            url: getIcon("UI_RelicIcon_15005_5")
            },
        cup: {
            text: 1803,
            url: getIcon("UI_RelicIcon_15005_1")
            },
        head: {
            text: 432,
            url: getIcon("UI_RelicIcon_15005_3")
            },
        config4: [
            
        ],
    },
    
    "thunderSmoother": {
        eng: "thunderSmoother",
        name2: "Thundersoother",
        nameLocale: 618,
        minStar: 4,
        maxStar: 5,
    
    effect2: 1813,
    
    effect4: 595,
    

        flower: {
            text: 622,
            url: getIcon("UI_RelicIcon_14002_4")
            },
        feather: {
            text: 623,
            url: getIcon("UI_RelicIcon_14002_2")
            },
        sand: {
            text: 620,
            url: getIcon("UI_RelicIcon_14002_5")
            },
        cup: {
            text: 621,
            url: getIcon("UI_RelicIcon_14002_1")
            },
        head: {
            text: 619,
            url: getIcon("UI_RelicIcon_14002_3")
            },
        config4: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate","title":772,"type":"float"},
            
        ],
    },
    
    "tinyMiracle": {
        eng: "tinyMiracle",
        name2: "TinyMiracle",
        nameLocale: 536,
        minStar: 3,
        maxStar: 4,
    
    effect2: 676,
    
    effect4: 372,
    

        flower: {
            text: 540,
            url: getIcon("UI_RelicIcon_10004_4")
            },
        feather: {
            text: 539,
            url: getIcon("UI_RelicIcon_10004_2")
            },
        sand: {
            text: 538,
            url: getIcon("UI_RelicIcon_10004_5")
            },
        cup: {
            text: 537,
            url: getIcon("UI_RelicIcon_10004_1")
            },
        head: {
            text: 541,
            url: getIcon("UI_RelicIcon_10004_3")
            },
        config4: [
            
        ],
    },
    
    "travelingDoctor": {
        eng: "travelingDoctor",
        name2: "TravelingDoctor",
        nameLocale: 1172,
        minStar: 1,
        maxStar: 3,
    
    effect2: 1588,
    
    effect4: 811,
    

        flower: {
            text: 1177,
            url: getIcon("UI_RelicIcon_10013_4")
            },
        feather: {
            text: 1175,
            url: getIcon("UI_RelicIcon_10013_2")
            },
        sand: {
            text: 1173,
            url: getIcon("UI_RelicIcon_10013_5")
            },
        cup: {
            text: 1176,
            url: getIcon("UI_RelicIcon_10013_1")
            },
        head: {
            text: 1174,
            url: getIcon("UI_RelicIcon_10013_3")
            },
        config4: [
            
        ],
    },
    
    "VermillionHereafter": {
        eng: "VermillionHereafter",
        name2: "VermillionHereafter",
        nameLocale: 1668,
        minStar: 4,
        maxStar: 5,
    
    effect2: 745,
    
    effect4: 805,
    

        flower: {
            text: 1312,
            url: getIcon("UI_RelicIcon_15023_4")
            },
        feather: {
            text: 1183,
            url: getIcon("UI_RelicIcon_15023_2")
            },
        sand: {
            text: 1787,
            url: getIcon("UI_RelicIcon_15023_5")
            },
        cup: {
            text: 1431,
            url: getIcon("UI_RelicIcon_15023_1")
            },
        head: {
            text: 1541,
            url: getIcon("UI_RelicIcon_15023_3")
            },
        config4: [
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"rate_q","title":239,"type":"float"},
            
            {"default":0.0,"max":4.0,"min":0.0,"name":"stack","title":616,"type":"float"},
            
        ],
    },
    
    "viridescentVenerer": {
        eng: "viridescentVenerer",
        name2: "ViridescentVenerer",
        nameLocale: 1451,
        minStar: 4,
        maxStar: 5,
    
    effect2: 1515,
    
    effect4: 687,
    

        flower: {
            text: 1738,
            url: getIcon("UI_RelicIcon_15002_4")
            },
        feather: {
            text: 1245,
            url: getIcon("UI_RelicIcon_15002_2")
            },
        sand: {
            text: 1454,
            url: getIcon("UI_RelicIcon_15002_5")
            },
        cup: {
            text: 1453,
            url: getIcon("UI_RelicIcon_15002_1")
            },
        head: {
            text: 1455,
            url: getIcon("UI_RelicIcon_15002_3")
            },
        config4: [
            
        ],
    },
    
    "VourukashasGlow": {
        eng: "VourukashasGlow",
        name2: "VourukashasGlow",
        nameLocale: 1479,
        minStar: 4,
        maxStar: 5,
    
    effect2: 1300,
    
    effect4: 213,
    

        flower: {
            text: 1194,
            url: getIcon("UI_RelicIcon_15030_4")
            },
        feather: {
            text: 1274,
            url: getIcon("UI_RelicIcon_15030_2")
            },
        sand: {
            text: 115,
            url: getIcon("UI_RelicIcon_15030_5")
            },
        cup: {
            text: 828,
            url: getIcon("UI_RelicIcon_15030_1")
            },
        head: {
            text: 1193,
            url: getIcon("UI_RelicIcon_15030_3")
            },
        config4: [
            
            {"default":4.0,"max":5.0,"min":0.0,"name":"stack","title":615,"type":"float"},
            
        ],
    },
    
    "wandererTroupe": {
        eng: "wandererTroupe",
        name2: "WanderersTroupe",
        nameLocale: 1138,
        minStar: 4,
        maxStar: 5,
    
    effect2: 244,
    
    effect4: 1573,
    

        flower: {
            text: 117,
            url: getIcon("UI_RelicIcon_15003_4")
            },
        feather: {
            text: 1280,
            url: getIcon("UI_RelicIcon_15003_2")
            },
        sand: {
            text: 1427,
            url: getIcon("UI_RelicIcon_15003_5")
            },
        cup: {
            text: 398,
            url: getIcon("UI_RelicIcon_15003_1")
            },
        head: {
            text: 726,
            url: getIcon("UI_RelicIcon_15003_3")
            },
        config4: [
            
        ],
    },
    
}