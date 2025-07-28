use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ICON_HASHMAP: HashMap<&'static str, &'static str> = {
        let data = vec![
            // 5 Star Characters
            ("Skirk", "6851f37b/b3007bcbb99dd9b47af558ecc1bb65ff"),
            ("Escoffier", "681a947b/6c4f7f722b0226a1171ec265485de61c"),
            ("Varesa", "67e3357c/34596fcb08c5c4b4e91379507e75dba7"),
            ("Yumemizuki Mizuki", "67c7f6c8/1cebf43ac944ad3ff2a0581bebe309d8"),
            ("Citlali", "67c7f6c8/abe2da348abe64c67d9c2763d2d8c4e7"),
            ("Mavuika", "67c7f6c8/c54cf9fe971d6226b78ad795ced9bee8"),
            ("Chasca", "67c7f6c8/c009627a0c0289646a10d41779001b9d"),
            ("Xinon", "67c7f6c8/7c047dce3a1be70baa30f28111222e38"),
            ("Mualani", "67c7f6c8/1fa9d9e4780785e7ab6c2ac10e556527"),
            ("Kinich", "67c7f6c8/ec67809641b4f1d2d2e87bc0e1e88b93"),
            ("Emilie", "67c7f6c8/91922ba4b8882f158eb0d26c30ab74d5"),
            ("Clorinde", "67c7f6c8/67796a8312172b6b2a82d3097452d141"),
            ("Arlecchino", "67c7f6c8/1b277d9a90936da66f13918e3c9b828c"),
            ("Sigewinne", "67c7f6c8/9236f61b549f9dbca318c3a8fd46bcb4"),
            ("Chiori", "67c7f6c8/2eeb4b86d43acc81c6f9b37bdba03e83"),
            ("Xianyun", "67c7f6c8/20c3618fbc7304c6b716e2733b2d4db3"),
            ("Navia", "67c7f6c8/dd80b2174e316e6a9a4f699dba23d6af"),
            ("Furina", "67c7f6c8/0a024f0f2e06e99d67f112cd12eef4af"),
            ("Neuvillette", "67c7f6c8/b30987d539b299dc89004d72274f5e89"),
            ("Wriothesley", "67c7f6c8/7786cf3f9620456f6d14a226dfdcca67"),
            ("Lyney", "67c7f6c8/56d665e0298a876a4393ea385db61da3"),
            // ("Baizhu", "67c7f6c8/afc87fc731e998f8f0f956f03de1cf21"),
            // ("Dehya", "67c7f6c8/e7beb9e36337d7f64139ee0c97d1a93a"),
            // ("Alhaitham", "67c7f6c8/a7eb0a9a0805bdecfaccfd450ca61102"),
            // ("Wanderer", "67c7f6c8/555d1f67bc5ea5ef1dfb39f47287ca11"),
            // ("Nahida", "67c7f6c8/823e31bf43f378bd66c394985a323da8"),
            // ("Cyno", "67c7f6c8/8eb1a5cca01c9e988581067a31e8b1d6"),
            // ("Nilou", "67c7f6c8/2cb9c9350cf6ff26b2ff293cce139cab"),
            // ("Tighnari", "67c7f6c8/2747d92218297b5094d4b8fe6d5f6475"),
            // ("Kamisato Ayato", "67c7f6c8/e350954e7c5f587e8c1e9fd5931ebac2"),
            // ("Shenhe", "67c7f6c8/d7f5dc584b2a9b02b8266084d96ad3cf"),
            // ("Aloy", "67c7f6c8/ac42af1ac5dff78b080c5bdaa1bfbc76"),
            // ("Yelan", "67c7f6c8/e76d6bdb8f4aeea019df7856ef9eca3f"),
            // ("Yae Miko", "67c7f6c8/54f9f46e282f81dc06c45d2783e0206a"),
            // ("Arataki Itto", "67c7f6c8/122eddc88eb224d79ae466ca0c615ce7"),
            // ("Sangonomiya Kokomi", "67c7f6c8/90652aa121930a54372ede5e53b4615e"),
            // ("Raiden Shogun", "67c7f6c8/23aa979c8e86b7006a62039b9dd81c5b"),
            // ("Eula", "67c7f6c8/4ea05e98ca62e2d3c7de83ea6bcf641b"),
            // ("Yoimiya", "67c7f6c8/7d324e9ea99247b744b08f7016649d1d"),
            // ("Kaedehara Kazuha", "67c7f6c8/7d56eb845d4f982757a942a97f88b958"),
            // ("Hu Tao", "67c7f6c8/276d840341c2791df6eefe8dd492ba7d"),
            // ("Keqing", "67c7f6c8/89d47da9c1ed3ade59b3c9f7fd00f1e8"),
            // ("Albedo", "67c7f6c8/e6d4bca80ff5ba2846f4cc7eafcb80a8"),
            // ("Ganyu", "67c7f6c8/2977be100672154bb7f5fc0d1abe0eb2"),
            // ("Qiqi", "67c7f6c8/2825ff5127afc078dd1bd3e1b2e39996"),
            // ("Tartaglia", "67c7f6c8/e23dde6e455a2dc62dcebbf2d41f7f72"),
            // ("Zhongli", "67c7f6c8/9dcda1d9e45a4f9e59266480ee2fc0c8"),
            // ("Klee", "67c7f6c8/54f52fb2be4b82fdbde390db0fce89ea"),
            // ("Xiao", "67c7f6c8/6dca29648877d9140b67d5ccc3db662e"),
            // ("Venti", "67c7f6c8/da04983f26f201db1a287bf922cdcaff"),
            // ("Diluc", "67c7f6c8/929d6645727e400b3a294b559e36a03f"),
            // ("Kamisato Ayaka", "67c7f6c8/f616e8fdc168d5e5314929a492788e93"),

            // 4 Star Characters
            // ("Dahlia", "6851f37b/9e5b1c5805948358c656e702e5eecae2"),
            // ("Ifa", "681a947b/0b3127bb721214a48b20001c64ab727a"),
            ("Iansan", "67e3357c/f44e8b4c7a61a4abb2aa2ccfe9b810cd"),
            ("LanYan", "67c7f6c8/eba829f8ca4b38367c4a2551d0e228ae"),
            ("Ororon", "67c7f6c8/64a90c55b4bb72cf06068f3d82dbc8da"),
            ("Kachina", "67c7f6c8/2c4b6f3e78da44cbe10f9cbc37a50fe1"),
            ("Sethos", "67c7f6c8/62ddf4187c2aea2dda21813b9d90f27f"),
            ("Gaming", "67c7f6c8/1238a7679cc90e138ec793b08aa853c7"),
            ("Chevreuse", "67c7f6c8/817f1d6da2a3ff6e6bfedeb39104ad71"),
            ("Charlotte", "67c7f6c8/f4e68e94225fa95112ae03c0a6d10938"),
            ("Freminet", "67c7f6c8/34ef1ba51a4893d9df4be717e56891ae"),
            ("Lynette", "67c7f6c8/5dad272ce992b32d3aa7a708869d1a4a"),
            ("Kirara", "67c7f6c8/49767b6d7070afddae937e417253bb3f"),
            // ("Kaveh", "67c7f6c8/a34ebd3dba93cda6fba3cc97a995822e"),
            // ("Mika", "67c7f6c8/f239b3032e85ebb64554e69a58eec91b"),
            // ("Yaoyao", "67c7f6c8/d7ec9b285faccd804b54fa026dde96e8"),
            // ("Faruzan", "67c7f6c8/58a03d4cded3b6e93af3cc3efb708031"),
            // ("Layla", "67c7f6c8/523506069469ad5f09b79daf6ed3a285"),
            // ("Candace", "67c7f6c8/9ef120c399afbf7f136034b68d721d44"),
            // ("Dori", "67c7f6c8/d4892796ad70d139c13a6e42d170b877"),
            // ("Collei", "67c7f6c8/eaf61637b76953695940fe1ceedb18a0"),
            // ("Kuki Shinobu", "67c7f6c8/caa2e14875915abee78b186b61fd8f3a"),
            // ("Yun Jin", "67c7f6c8/45fc56fa83f5447d6a61677f514a9131"),
            // ("Shikanoin Heizou", "67c7f6c8/c4d144d578bf42c6f03ac240a3bf2c3c"),
            // ("Kujou Sara", "67c7f6c8/3dc3a368e5c3be42a05afb7a9fdc9c41"),
            // ("Gorou", "67c7f6c8/a3a8da43e8c4d76986237ab784e49070"),
            // ("Sayu", "67c7f6c8/1791a03bc18b12a8030a4e16df2ea5f6"),
            // ("Thoma", "67c7f6c8/57ede6ce84e7030b58af29520198119b"),
            // ("Yanfei", "67c7f6c8/94ef1c503603f0deb0550cdb4a4de40e"),
            // ("Xinyan", "67c7f6c8/a7f600012d57d9302c9d3b7175af244a"),
            // ("Sucrose", "67c7f6c8/27ee1aba588f3e8e42800249fe1047e8"),
            // ("Diona", "67c7f6c8/4fc01b013d363a4805296ba081342870"),
            // ("Chongyun", "67c7f6c8/38b5bd856ea9fff842477a77265da49b"),
            // ("Noelle", "67c7f6c8/1eb5023b8ca517a5fe27316b3771a191"),
            // ("Bennett", "67c7f6c8/41a97aae8bf579ce350a90bbddc59fdf"),
            // ("Fischl", "67c7f6c8/207a0e35e0637a2bcb0a28fdfa61f7a4"),
            // ("Ningguang", "67c7f6c8/d0bd034f8871d647a4a612bf5b15bc21"),
            // ("Xingqiu", "67c7f6c8/7a37a944f52edbd6894bc39da801fb67"),
            // ("Beidou", "67c7f6c8/bc2903d1a46616c871d5108d033a463d"),
            // ("Xiangling", "67c7f6c8/c836c2dc7d71fe8065ae2798d360167b"),
            // ("Razor", "67c7f6c8/0a8a2c7373dcc8f2549b645d0c3374b7"),
            // ("Kaeya", "67c7f6c8/0cb07e45505e78141ac996c89714d73b"),
            // ("Barbara", "67c7f6c8/f55430aba2d84b36d64136958c58c93a"),
            // ("Lisa", "67c7f6c8/415cb4af1a402f97aebfcf8edc2ca235"),

            // 武器
            // ("AstralVulturesCrimsonPlumage", "67c7f6c8/afac47d999995ce3577c7034ff277c44"), // 星鹫赤羽
            ("SilvershowerHeartstrings", "67c7f6c8/49934c2c44b1f643f96bc4b65b534eca"), // 白雨心弦
            ("TheFirstGreatMagic", "67c7f6c8/834006ea86ff338ed823d2ccd09a1169"), // 最初的大魔术
            // ("HuntersPath", "67c7f6c8/dc35942eb26612d7759acededff35fa0"), // 猎人之径
            // ("ThunderingPulse", "67c7f6c8/d3cdbcebbaac5598dcd973424e685b8b"), // 飞雷之弦振
            // ("AquaSimulacra", "67c7f6c8/0986345abd92564279c5a11f491bd6c0"), // 若水
            // ("PolarStar", "67c7f6c8/50ec4928dd095307807b59617a889b62"), // 冬极白星
            // ("ElegyForTheEnd", "67c7f6c8/e9555699ff95fac410422d9dfd7e631a"), // 终末嗟叹之诗
            // ("AmosBow", "67c7f6c8/fe810457e28088dc44b0500d90e417f5"), // 阿莫斯之弓
            // ("SkywardHarp", "67c7f6c8/504e181be5a8ba069b7249a3f7cf978d"), // 天空之翼
            // ("VividNotions", "67e3357c/db3c23247fa556442b7d1fdd5e4a4fb6"), // 溢彩心念
            // ("SunnyMorningSleepIn", "67c7f6c8/fa7fc655e5c1bab724c7893ec3f58d91"), // 寝正月初晴
            // ("StarcallersWatch", "67c7f6c8/dbf7751578357bde53f9b563aee72250"), // 祭星者之望
            ("SurfsUp", "67c7f6c8/ad90ce11f6af2cf1db44380af745e4ec"), // 冲浪时光
            ("CranesEchoingCall", "67c7f6c8/876ff00e5e80e02cc801bc6c37e48b84"), // 鹤鸣余音
            ("TomeOfTheEternalFlow", "67c7f6c8/a9c72ec4bc54b9acf27362095da7af61"), // 万世流涌大典
            ("CashflowSupervision", "67c7f6c8/f540cf493ce73c0b3afa3bc32940bfa0"), // 金流监督
            // ("TulaytullahsRemembrance", "67c7f6c8/f1bd10806edde7470a4e4a00d63692f5"), // 图莱杜拉的回忆
            // ("AThousandFloatingDreams", "67c7f6c8/7cf365df4327805751ed47e8cda853bb"), // 千夜浮梦
            // ("KagurasVerity", "67c7f6c8/5df4b80100a07922792ac85362ee6af8"), // 神乐之真意
            // ("EverlastingMoonglow", "67c7f6c8/c7638821093bb021334a85900466137f"), // 不灭月华
            // ("JadefallsSplendor", "67c7f6c8/ba93fa1f1b51d815a2ee1478f34e84ef"), // 碧落之珑
            // ("MemoryOfDust", "67c7f6c8/7f1e579f9b6925daa69f67e328961033"), // 尘世之锁
            // ("LostPrayerToTheSacredWinds", "67c7f6c8/856b4ea453095a88e8cbb3f438d3fa83"), // 四风原典
            // ("SkywardAtlas", "67c7f6c8/d3f398d9ee417e931b1e273242dd8008"), // 天空之卷
            // ("SymphonistOfScents", "681a947b/6d211f1902adc8a5946a5530489ab77a"), // 香韵奏者
            ("LumidouceElegy", "67c7f6c8/815db248eebc121db24905994b274150"), // 柔灯挽歌
            ("CrimsonMoonsSemblance", "67c7f6c8/30b345a334076865566f0da4e36d8d0d"), // 赤月之形
            // ("StaffOfTheScarletSands", "67c7f6c8/0045e2a1f5eb25cabe20db8c79b3cfd6"), // 赤沙之杖
            // ("EngulfingLightning", "67c7f6c8/481772ac498bbc4a9b61c694144096e5"), // 薙草之稻光
            // ("CalamityQueller", "67c7f6c8/60e9db1560db9a7aa3b8a930cf30ce12"), // 息灾
            // ("PrimordialJadeWingedSpear", "67c7f6c8/4b8624f9f3d335b504d879e9b1e07ae9"), // 和璞鸢
            // ("VortexVanquisher", "67c7f6c8/503b33a86fdecbca9254d3cbf9630e14"), // 贯虹之槊
            // ("SkywardSpine", "67c7f6c8/32df7904999ca5913e0501847a6f57b0"), // 天空之脊
            // ("StaffOfHoma", "67c7f6c8/d9ea5f48b504d48837cace02efdf1b8c"), // 护摩之杖
            // ("AThousandBlazingSuns", "67c7f6c8/6e57c13f34e89eae3339461e4f1afe3f"), // 焚曜千阳
            // ("FangOfTheMountainKing", "67c7f6c8/a9184535c8743e8da52fa2debd54f7d8"), // 山王长牙
            ("Verdict", "67c7f6c8/89c37ece820a0a1eca5f2b65a98a5e7a"), // 裁断
            // ("BeaconOfTheReedSea", "67c7f6c8/e1eefe2e1a8a70ad45823a6af99e7d7a"), // 苇海信标
            // ("RedhornStonethresher", "67c7f6c8/a3d93fd480b8838c79d99aa8f90e7081"), // 赤角石溃杵
            // ("TheUnforged", "67c7f6c8/0ba71f650c9ae79f9f652ae7d6f9df86"), // 无工之剑
            // ("SongOfBrokenPines", "67c7f6c8/13a14bc2183b1a33041042b77ac4b640"), // 松籁响起之时
            // ("WolfsGravestone", "67c7f6c8/45c64eb0f4dfffe24ab9e993616f02c6"), // 狼的末路
            // ("SkywardPride", "67c7f6c8/e53f3ec6e717e1430f991aedf5a33e0a"), // 天空之傲
            // ("Azurelight", "6851f37b/2b47d934eb09f3926908c37462569291"), // 苍耀
            // ("PeakPatrolSong", "67c7f6c8/192f70211e4456786ebb3cb0c9222e43"), // 岩峰巡歌
            ("Absolution", "67c7f6c8/0f4646d85544fa3087186883cd9918c9"), // 赦罪
            ("UrakuMisugiri", "67c7f6c8/c8d3a5b868846ec4819294483de2f90b"), // 有乐御簾切
            ("SplendorOfTranquilWaters", "67c7f6c8/a0c785ee7bcfa4e7dd19032b8c28c7fa"), // 静水流涌之辉
            // ("LightOfFoliarIncision", "67c7f6c8/d4ce6ec60d82eec0908027c16e9c1b68"), // 裁叶萃光
            // ("KeyOfKhajNisut", "67c7f6c8/d51d92c819500a9c2a4fb200ffeb8c08"), // 圣显之钥
            // ("HaranGeppakuFutsu", "67c7f6c8/c0d5a9cc91eec72ee2d77ead379a7331"), // 波乱月白经津
            // ("MistsplitterReforged", "67c7f6c8/3c881bf12a6b24197d9df60b81329e52"), // 雾切之回光
            // ("PrimordialJadeCutter", "67c7f6c8/6a538e31e42088f7ba572556cc5862e0"), // 磐岩结绿
            // ("SummitShaper", "67c7f6c8/4b143147ac265bcf3bf703fbe95c4479"), // 斫峰之刃
            // ("FreedomSworn", "67c7f6c8/fcc21d0d30236aa146f099de4e4bdfa8"), // 苍古自由之誓
            // ("SkywardBlade", "67c7f6c8/ecb922197e45697d9edde75af4b8df93"), // 天空之刃
            // ("AquilaFavonia", "67c7f6c8/2e40de313b0b488549fde96e553e7fcf"), // 风鹰剑
            // ("SequenceOfSolitude", "681a947b/74182d9654a8ed410e405ca87475812f"), // 冷寂迸音
            ("ChainBreaker", "67c7f6c8/db2a58cc67636814146192b0e748e9a6"), // 碎链
            // ("FlowerWreathedFeathers", "67c7f6c8/03b273f10d0633aedb370051950e91fd"), // 缀花之翎
            ("RangeGauge", "67c7f6c8/52be43b8c0cdd1fa06c1a756777a6e93"), // 测距规
            ("Cloudforged", "67c7f6c8/b2c9dcddaa4da988c7ee15045af8d8bc"), // 筑云
            ("SongOfStillness", "67c7f6c8/e8e52713d73416d6bd4860e2ec7eebf0"), // 静谧之曲
            ("ScionOfTheBlazingSun", "67c7f6c8/656ed54983d6f96092bb307e2589f1c1"), // 烈阳之嗣
            ("IbisPiercer", "67c7f6c8/6e37803f70f76be9df083b99a5af8307"), // 鹮穿之喙
            // ("EndOfTheLine", "67c7f6c8/757dd6f75e6f4225088be09435d9d03b"), // 竭泽
            // ("KingsSquire", "67c7f6c8/b14920a1d49af70c345e1663eac49717"), // 王下近侍
            // ("MouunsMoon", "67c7f6c8/25f04eaed84aaf525f6b10e868e7f835"), // 曚云之月
            // ("Predator", "67c7f6c8/95be77123f875f3393bc2a08f5eabe20"), // 掠食者
            // ("Hamayumi", "67c7f6c8/0d09981aa5c965eb05df4d6fa400d7bc"), // 破魔之弓
            // ("WindblumeOde", "67c7f6c8/fc907601decd02b1a63a68ebd62d00cf"), // 风花之颂
            // ("MitternachtsWaltz", "67c7f6c8/425d13a6970fbdb3802304349b4f7de3"), // 幽夜华尔兹
            // ("FadingTwilight", "67c7f6c8/ab3de086b1a1f6a42d1b5c8ae32f3b15"), // 落霞
            // ("AlleyHunter", "67c7f6c8/0bf9e977296b6c9a5e6aff3ef377a3b9"), // 暗巷猎手
            // ("TheViridescentHunt", "67c7f6c8/4cb4be711bf4af4b3ca02a78cc72bd16"), // 苍翠猎弓
            // ("BlackcliffWarbow", "67c7f6c8/a0ff01694b9dfd463fb074c8b4af1063"), // 黑岩战弓
            // ("CompoundBow", "67c7f6c8/3f8226fbfd17d85c094e0796b3a92d8d"), // 钢轮弓
            // ("PrototypeCrescent", "67c7f6c8/80f9f492c45ffb4376aa4e8e496ab5d5"), // 试作澹月
            // ("Rust", "67c7f6c8/6b682afb95373317a4a965b8974b5d08"), // 弓藏
            // ("RoyalBow", "67c7f6c8/893ae1927bea9c1b2b495e46eae472f1"), // 宗室长弓
            // ("SacrificialBow", "67c7f6c8/4a03625fdbc739dca379573871a4c2ca"), // 祭礼弓
            // ("TheStringless", "67c7f6c8/da3d5e7bbc112c4e1378de554a79c6c3"), // 绝弦
            // ("FavoniusWarbow", "67c7f6c8/f0b8edcc330cd9de1dcf97eccb20cbaf"), // 西风猎弓
            ("RingOfYaxche", "67c7f6c8/59c44935ff819c319548daff2b222127"), // 木棉之环
            // ("WaveridingWhirl", "67c7f6c8/bfae38e15e0405f03aa28fdc7b101820"), // 乘浪的回旋
            ("AshGravenDrinkingHorn", "67c7f6c8/92039ba9ea4140e6ffbb61713622847d"), // 苍纹角杯
            ("BalladOfTheBoundlessBlue", "67c7f6c8/e8e43cfd0ba489f79f5fb138e2178554"), // 无垠蔚蓝之歌
            ("FlowingPurity", "67c7f6c8/6533155e829bb778b9ef84563a09e692"), // 纯水流华
            ("SacrificialJade", "67c7f6c8/95e2a7c637afe59f0ec81b2da9ad49c9"), // 遗祀玉珑
            // ("FruitOfFulfillment", "67c7f6c8/c533c74b11161cf1b59f745b727c98cf"), // 盈满之实
            // ("WanderingEvenstar", "67c7f6c8/e8d25e1a09cbca9b5069ad93c00d7b68"), // 流浪的晚星
            // ("OathswornEye", "67c7f6c8/f9308f15888ca90af00ff12c5223c1a9"), // 证誓之明瞳
            // ("HakushinRing", "67c7f6c8/29ef5797e2374e7f3a6996dc02c72d83"), // 白辰之环
            // ("DodocoTales", "67c7f6c8/6ec8e2657f0f24d5ed35ecbde8aa85db"), // 嘟嘟可故事集
            // ("Frostbearer", "67c7f6c8/39a484f89b11fda2c68f29bcca43e26c"), // 忍冬之果
            // ("WineAndSong", "67c7f6c8/32ec1c8d3421d0a88d2e9eb8e357cf29"), // 暗巷的酒与诗
            // ("EyeOfPerception", "67c7f6c8/811ff594eebe6cc05f12e02662803239"), // 昭心
            // ("BlackcliffAgate", "67c7f6c8/5ff68cdb787d782918193236c883ece3"), // 黑岩绯玉
            // ("MappaMare", "67c7f6c8/ad12d566dd0376457b622464e0744fe2"), // 万国诸海图谱
            // ("PrototypeAmber", "67c7f6c8/00d58c6a9f08c487851206576c962551"), // 试作金珀
            // ("SolarPearl", "67c7f6c8/be0408e0e9d8356566c6cbad21b959d7"), // 匣里日月
            // ("RoyalGrimoire", "67c7f6c8/3a69aaa3fe4b3e6d1269790434cfac8a"), // 宗室秘法录
            // ("SacrificialFragments", "67c7f6c8/807ae55311a3a8382194a656e214e7b1"), // 祭礼残章
            // ("TheWidsith", "67c7f6c8/22d2b64c9fd0ab81ef934c8387f72d0c"), // 流浪乐章
            // ("FavoniusCodex", "67c7f6c8/bb4ff947d3649e393b6ca2e117b072d6"), // 西风秘典
            // ("TamayurateiNoOhanashi", "67c7f6c8/7530c502a61c663424986c74899e4b1b"), // 且住亭御咄
            ("FootprintOfTheRainbow", "67c7f6c8/61747b9617129bbe11dcf84a93de3fe1"), // 虹的行迹
            // ("MountainBracingBolt", "67c7f6c8/27f9739cc6d08a99e6d902f1f9895903"), // 镇山之钉
            ("ProspectorsDrill", "67c7f6c8/097c698fe775b593c34bff55f9b6f271"), // 勘探钻机
            ("DialoguesOfTheDesertSages", "67c7f6c8/004899f4640baadfa2be9d9dc0e90419"), // 沙中伟贤的对答
            ("RightfulReward", "67c7f6c8/97828545a1aced0bcab8f3dbd38501b9"), // 公义的酬报
            ("BalladOfTheFjords", "67c7f6c8/78b8e15bfba8cf8070335df8c29213fe"), // 峡湾长歌
            // ("MissiveWindspear", "67c7f6c8/0ea3522df5405f90d90505f5e36c2008"), // 风信之锋
            // ("Moonpiercer", "67c7f6c8/7091bc230ebbe5015f5c8a7418a721c0"), // 贯月矢
            // ("WavebreakersFin", "67c7f6c8/e4f85d4caee3fa61ba93bdad6f0dfdd7"), // 断浪长鳍
            // ("TheCatch", "67c7f6c8/643071ffc7526446b48101f159ee0437"), // 「渔获」
            // ("KitainCrossSpear", "67c7f6c8/8fed2cc01d2f7f9b6c5cac8cf604bf2d"), // 喜多院十文字
            // ("DragonspineSpear", "67c7f6c8/2f8be9e904242c2b40f3d1899ffc0694"), // 龙脊长枪
            // ("RoyalSpear", "67c7f6c8/212da9569426960f5b0dedba9b0d64a2"), // 宗室猎枪
            // ("FavoniusLance", "67c7f6c8/7db45b7a84bb80468aaf0f5f430e9bdc"), // 西风长枪
            // ("LithicSpear", "67c7f6c8/1117bae36b42e7ace47a411cb4b04ca1"), // 千岩长枪
            // ("Deathmatch", "67c7f6c8/e28812a4dbe5207d8d88c68a53212692"), // 决斗之枪
            // ("BlackcliffPole", "67c7f6c8/4bfccf3b0a988ed4c816889be71a7a88"), // 黑岩刺枪
            // ("CrescentPike", "67c7f6c8/b96753e95648a2ddf2ece0cdcd12b758"), // 流月针
            // ("PrototypeStarglitter", "67c7f6c8/e9f8f53de5554bd8ed06e5783aeee3c9"), // 试作星镰
            // ("DragonsBane", "67c7f6c8/cf0f39f3f9fddd9fdcafe921ca8cff1a"), // 匣里灭辰
            ("EarthShaker", "67c7f6c8/5b961a68fc9a53711c1ee38504517401"), // 撼地者
            // ("FruitfulHook", "67c7f6c8/c60cc5afa76926ae412d6c66224f972b"), // 硕果钩
            ("PortablePowerSaw", "67c7f6c8/f849ee063b4aafdf78deae9ef0cbf39f"), // 便携动力锯
            ("UltimateOverlordsMegaMagicSword", "67c7f6c8/2c73799dd587e54c75af09cf87d8576a"), // 「究极霸王超级魔剑」
            ("TidalShadow", "67c7f6c8/95def815e975a9fb89e9697209de8578"), // 浪影阔剑
            ("TalkingStick", "67c7f6c8/f6f37714a0a83e1974c458bdac72df48"), // 聊聊棒
            // ("MailedFlower", "67c7f6c8/f9c8a43e26e2e0cfca8d757a07d4c235"), // 饰铁之花
            // ("ForestRegalia", "67c7f6c8/3ee3cc488b17390afc4aa92d7ee2dad6"), // 森林王器
            // ("Akuoumaru", "67c7f6c8/3b3d34b3ebc2dc1f5bf989d663d42ec6"), // 恶王丸
            // ("MakhairaAquamarine", "67c7f6c8/cc4498636d1de587406d20b7c5f84c60"), // 玛海菈的水色
            // ("KatsuragikiriNagamasa", "67c7f6c8/f1518eb4ebfa7cc60ff113f37e7dfa24"), // 桂木斩长正
            // ("LuxuriousSeaLord", "67c7f6c8/d6a5ab503c052d5c0814a39417cc2fee"), // 衔珠海皇
            // ("SnowTombedStarsilver", "67c7f6c8/15bb71d26fd594136aea9157303ab019"), // 雪葬的星银
            // ("LithicBlade", "67c7f6c8/c6387d2f9dfb648e5ec1ad5a63837761"), // 千岩古剑
            // ("SerpentSpine", "67c7f6c8/8add3ad74e3838695ee23cacbe99e776"), // 螭骨剑
            // ("BlackcliffSlasher", "67c7f6c8/794c87023c6bb414a9a1ba03d52d0297"), // 黑岩斩刀
            // ("Whiteblind", "67c7f6c8/b014e2aa23e856e0cf3f802584b26862"), // 白影剑
            // ("PrototypeArchaic", "67c7f6c8/0365a23ab21b087a788d0d41c37bb049"), // 试作古华
            // ("Rainslasher", "67c7f6c8/de1e307e1104fbf7c8f7a7ef5cb143e1"), // 雨裁
            // ("RoyalGreatsword", "67c7f6c8/6c961ad33f44bd77fcab7c11a2f34f93"), // 宗室大剑
            // ("SacrificialGreatsword", "67c7f6c8/b79312ddc86e05133b8a8d984f9931e1"), // 祭礼大剑
            // ("TheBell", "67c7f6c8/d7f9ca521e414f63dfe24876b91cc139"), // 钟剑
            // ("FavoniusGreatsword", "67c7f6c8/7646388f158d62d56f368631a37427c3"), // 西风大剑
            // ("CalamityOfEshu", "67c7f6c8/6afc33638bed2a9fbc85e2fd2147924e"), // 厄水之祸
            ("FluteOfEzpitzal", "67c7f6c8/4ffc901bcdb3a6c5e6c84d100af0e58d"), // 息燧之笛
            // ("SturdyBone", "67c7f6c8/70c0c7e3eaaad4321d5c5675e1436588"), // 弥坚骨
            ("SwordOfNarzissenkreuz", "67c7f6c8/7ef90872b3553d9c0f4e3452737cec2d"), // 水仙十字之剑
            ("TheDockhandsAssistant", "67c7f6c8/137e6ee9d36a6af76d67718713859bbd"), // 船坞长剑
            ("FleuveCendreFerryman", "67c7f6c8/7ef38be38f19ff27bbba79f48b29e3c1"), // 灰河渡手
            ("FinaleOfTheDeep", "67c7f6c8/3d55aea4fa30254d868ad4fa4720c6b2"), // 海渊终曲
            // ("WolfFang", "67c7f6c8/86a26398db607012d49be9f7e0dd0476"), // 狼牙
            // ("ToukabouShigure", "67c7f6c8/74ef003a497f24a85e4e1e0af0793530"), // 东花坊时雨
            // ("XiphosMoonlight", "67c7f6c8/6e73725d6886d81d5a9b6525a2bdb31f"), // 西福斯的月光
            // ("SapwoodBlade", "67c7f6c8/ade9757e4346ef1402c32bdde6b7a7e4"), // 原木刀
            // ("KagotsurubeIsshin", "67c7f6c8/d0bc1a9d160a598e7cfe669f71402662"), // 笼钓瓶一心
            // ("CinnabarSpindle", "67c7f6c8/8c150d11be0738ee0188f6f5a2a56f86"), // 辰砂之纺锤
            // ("AmenomaKageuchi", "67c7f6c8/1aa93cbdddb0444e0eff47b3f84dc5d9"), // 天目影打刀
            // ("FesteringDesire", "67c7f6c8/685c320aa78fc0c2e29f2e445393949a"), // 腐殖之剑
            // ("SwordOfDescension", "67c7f6c8/58882f57dd30072cbc3f84371d543f06"), // 降临之剑
            // ("TheAlleyFlash", "67c7f6c8/c8c9aeb35a289a39adcd796d421c1b29"), // 暗巷闪光
            // ("TheBlackSword", "67c7f6c8/0d45932d85272f2efe3fb18418f159d0"), // 黑剑
            // ("BlackcliffLongsword", "67c7f6c8/19ed42f8a9b6de92e7aaef93479dae54"), // 黑岩长剑
            // ("IronSting", "67c7f6c8/d6627a35dc10a7c7f9d5db832e271cce"), // 铁蜂刺
            // ("PrototypeRancour", "67c7f6c8/d859d175bb85b6add3dc3df1fb8abc5a"), // 试作斩岩
            // ("LionsRoar", "67c7f6c8/85dbd526e5a6d0f20af4d0253be63ac0"), // 匣里龙吟
            // ("RoyalLongsword", "67c7f6c8/2b92d8da7beff401b00d680b74a7ed56"), // 宗室长剑
            // ("SacrificialSword", "67c7f6c8/8d40dcb0e8e25ad6c5940095fa6984ab"), // 祭礼剑
            // ("TheFlute", "67c7f6c8/e72bdcaaeb4b0a1ea13cfb76c88aa4cb"), // 笛剑
            // ("FavoniusSword", "67c7f6c8/cfda75195bd9c341e314a56e976d532f"), // 西风剑
            // ("Messenger", "67c7f6c8/0c145223319e7035b8bf17a43ad7eba2"), // 信使
            // ("Slingshot", "67c7f6c8/416221f265d0b711d55ad9146bfdb306"), // 弹弓
            // ("RecurveBow", "67c7f6c8/507fcc151bad5fcb3c9d61e436fd8839"), // 反曲弓
            // ("SharpshootersOath", "67c7f6c8/5f810f2e0af41ab46e69f96a44e7aa0e"), // 神射手之誓
            // ("RavenBow", "67c7f6c8/6dd8c709a161594e870fbdbfc5cb6d62"), // 鸦羽弓
            // ("TwinNephrite", "67c7f6c8/7d0e2c53e717e887e0e60b1f56756d4d"), // 甲级宝珏
            // ("EmeraldOrb", "67c7f6c8/a46be1d86331fa7075c0a968505857d7"), // 翡玉法球
            // ("OtherworldlyStory", "67c7f6c8/209a83e1b8e802cfc56442cbe0148ef6"), // 异世界行记
            // ("ThrillingTalesOfDragonSlayers", "67c7f6c8/aff08755a2149ffb6c0fa7d1d189a222"), // 讨龙英杰谭
            // ("MagicGuide", "67c7f6c8/c264b71e825629badbe8da846287c009"), // 魔导绪论
            // ("BlackTassel", "67c7f6c8/badaa063427a6e3f39d9c031b3dddc24"), // 黑缨枪
            // ("Halberd", "67c7f6c8/22ad0b31bd68659f1a160dbca09eb817"), // 钺矛
            // ("WhiteTassel", "67c7f6c8/d9d2b979b41b3a522f622d70787fe2e1"), // 白缨枪
            // ("SkyriderGreatsword", "67c7f6c8/8b59cc85afa53362080a7682e0b2284c"), // 飞天大御剑
            // ("DebateClub", "67c7f6c8/7c806b2d328c89e3f7c6d70b01214af7"), // 以理服人
            // ("WhiteIronGreatsword", "67c7f6c8/14ff7311040e5d563ef75b28e229a7ae"), // 白铁大剑
            // ("BloodtaintedGreatsword", "67c7f6c8/533baca38d78bed9a7334ebd15eb972e"), // 沐浴龙血的剑
            // ("FerrousShadow", "67c7f6c8/f7299d46b25b942d3ff3de4b01d3543a"), // 铁影阔剑
            // ("SkyriderSword", "67c7f6c8/bc3f197fd37c0f92f0f253f578a1d266"), // 飞天御剑
            // ("FilletBlade", "67c7f6c8/f1a49244f7411707529b38815a9d9cb5"), // 吃虎鱼刀
            // ("DarkIronSword", "67c7f6c8/14dbdf6255569327567b6e5e9f113fd4"), // 暗铁剑
            // ("TravelersHandySword", "67c7f6c8/6b735c4a2cc309a44c5c682f81364261"), // 旅行剑
            // ("HarbingerOfDawn", "67c7f6c8/048119e09a837be4c1a735fd046e3261"), // 黎明神剑
            // ("CoolSteel", "67c7f6c8/78feee812f5df4c3a886d93a06092da1"), // 冷刃
            // ("SeasonedHuntersBow", "67c7f6c8/53d5af406088693126f3bd691680c2eb"), // 历练的猎弓
            // ("PocketGrimoire", "67c7f6c8/5edb79d30c591ca6dbb84deeb4fa64d8"), // 口袋魔导书
            // ("IronPoint", "67c7f6c8/d235cf9e396a50d8f673a7caf2f8d718"), // 铁尖枪
            // ("OldMercsPal", "67c7f6c8/165dd007459fa4897a6ddeefe3993e6a"), // 佣兵重剑
            // ("SilverSword", "67c7f6c8/73748953f526f339fe5ed1707c295fd9"), // 银剑
            // ("HuntersBow", "67c7f6c8/c75ade4a5acb5eb9aaefbed318bf75ff"), // 猎弓
            // ("ApprenticesNotes", "67c7f6c8/852d232e46ddaacdfcb86b3f53413405"), // 学徒笔记
            // ("BeginnersProtector", "67c7f6c8/ca0ea04c05d8994f0b7e836012e38563"), // 新手长枪
            // ("WasterGreatsword", "67c7f6c8/b1e284917f1ca8330ba929ef8f116aba"), // 训练大剑
            // ("DullBlade", "67c7f6c8/0b590e80914fdb8e348323fff888be0c"), // 无锋剑

            // 圣遗物
            // ("魂香之花", "67c7f6c8/45b3135178fa8e47165f7b2000479f45"),
            // ("垂玉之叶", "67c7f6c8/386663ece74fde7a8ae6a3b61962bfe7"),
            // ("祝祀之凭", "67c7f6c8/cd08b3bef2d469fdfc255202a3e1d30a"),
            // ("涌泉之盏", "67c7f6c8/4af742760d499055c53ff308536ca5d9"),
            // ("浮溯之珏", "67c7f6c8/8d6e15e10a7f05c4ad0f961d8d74ef2c"),

            // ("生灵之华", "67c7f6c8/c3b0db3fda0f57be83087e8fa47f248d"),
            // ("潜光片羽", "67c7f6c8/0fce49c2072b19b474a29a3ea2300bd8"),
            // ("阳辔之遗", "67c7f6c8/f2b029515fd7e09c1ab913a9aaefad27"),
            // ("结契之刻", "67c7f6c8/801da1ab1db5f074ca151126b0f8aa73"),
            // ("虺雷之姿", "67c7f6c8/e9c3254cf0f4952b3e55c6b3b66f9e11"),

            // ("海染之花", "67c7f6c8/5f1381c508febb2a6e39aa87eaa4c5fc"),
            // ("渊宫之羽", "67c7f6c8/edd5f358c027aff093cbb2ef60e8e3b8"),
            // ("离别之贝", "67c7f6c8/e122b3e8efbbe27bceaac08e17818ce4"),
            // ("真珠之笼", "67c7f6c8/7ebafcb6489bf83e21b7aa53959b7f41"),
            // ("海祇之冠", "67c7f6c8/ae87d46924dfee7cde4379f879c6e4c9"),

            // ("荣花之期", "67c7f6c8/7d2a0cb19ab89de53c88937d6c4648a1"),
            // ("华馆之羽", "67c7f6c8/7db5803961686258dbd643be089789fb"),
            // ("众生之谣", "67c7f6c8/764223d7f9023c5914c9ef6049fc9e6d"),
            // ("梦醒之瓢", "67c7f6c8/544af5812ec326714c12519b4d8cee97"),
            // ("形骸之笠", "67c7f6c8/2f1dcfffae4b7ca51bc8b5a89cec6844"),

            // ("明威之镡", "67c7f6c8/16879d66028f85addecf5a7208f6bffb"),
            // ("切落之羽", "67c7f6c8/35a171a3906cf9789f208628f0f28770"),
            // ("雷云之笼", "67c7f6c8/30ad4db8661bccad21474d67fad9d908"),
            // ("绯花之壶", "67c7f6c8/24b9a122db4e39e9b9e2eb1cbcf5925f"),
            // ("华饰之兜", "67c7f6c8/8af0dc3497b273ccdee34d2cc948aa59"),

            // ("羁缠之花", "67c7f6c8/85f548425496394e4b812302b3de514a"),
            // ("思忆之矢", "67c7f6c8/afcf2a5cf7f130054dac4f31dd7d6c03"),
            // ("朝露之时", "67c7f6c8/f8a6ac0223534c936003a96d018d498b"),
            // ("祈望之心", "67c7f6c8/e6751e19ddf8c4a10632a5999e9395f3"),
            // ("无常之面", "67c7f6c8/71e9ec339457ff7e274469ddb97889f4"),

            // ("无垢之花", "67c7f6c8/038dcfa1e4b0a18dae83a7144448d9d2"),
            // ("贤医之羽", "67c7f6c8/6b4141163c5894064f43c363f0564566"),
            // ("停摆之刻", "67c7f6c8/22be2edca2dccded53c85f524b860631"),
            // ("超越之盏", "67c7f6c8/86f0c7fcf7afe2813a3d463ab1a5848f"),
            // ("嗤笑之面", "67c7f6c8/0fa0f2fe45f925ef624bf054787dba0c"),

            // ("勋绩之花", "67c7f6c8/e52f0d8928f3982864ecc8ea13165ebf"),
            // ("昭武翎羽", "67c7f6c8/84ba5a506941c2e7c67eec4fc91f6e6a"),
            // ("金铜时晷", "67c7f6c8/5e1fe50f98902ba239b743a552c95288"),
            // ("盟誓金爵", "67c7f6c8/5d25615fe6509385b867e634bebe93ed"),
            // ("将帅兜鍪", "67c7f6c8/dcdf1f408c923028f4565bcb1f2d12ae"),

            // ("饰金胸花", "67c7f6c8/e3f3ff85477d75c6b073dcb941865249"),
            // ("追忆之风", "67c7f6c8/c57c81c236d88176a653c2b0be80d71d"),
            // ("坚铜罗盘", "67c7f6c8/6c8ad61b731cc2c0c53ddd4f22e6a50d"),
            // ("沉波之盏", "67c7f6c8/63535d205fedae06e3ed9c403262ccde"),
            // ("酒渍船帽", "67c7f6c8/a6efed58b340e9aea774bd7d53544bc7"),

            // ("夏祭之花", "67c7f6c8/440a8a8ecc8f0f02645d5484949fca6b"),
            // ("夏祭终末", "67c7f6c8/2fda7dfb07d970fc90db910de890f3e8"),
            // ("夏祭之刻", "67c7f6c8/8888163740ced1e480057c56dc3f2600"),
            // ("夏祭水玉", "67c7f6c8/6cb0e3cb2a3b003c9a206e1e89a9bd41"),
            // ("夏祭之面", "67c7f6c8/baa6786cda943cb2800d6ca7d140c5c8"),

            // ("磐陀裂生之花", "67c7f6c8/1da6fb182fbf62736bd20ddbfeb2e485"),
            // ("嵯峨群峰之翼", "67c7f6c8/5ae89b30076c36972b7609b75cf30f9d"),
            // ("星罗圭璧之晷", "67c7f6c8/e15db34d8698aad900ff66e7bdb0ae33"),
            // ("巉岩琢塑之樽", "67c7f6c8/db301a8934290c15dd2dd43102ec8161"),
            // ("不动玄石之相", "67c7f6c8/ab5f9dccce3d0e29617e41b4c21634ad"),

            // ("染血的铁之心", "67c7f6c8/888617693e9828103722622f7c1daca8"),
            // ("染血的黑之羽", "67c7f6c8/059958b3162dcb5e2df47ff97fbc531f"),
            // ("骑士染血之时", "67c7f6c8/f6f09d13b225b09af60bba4c945cb3df"),
            // ("染血骑士之杯", "67c7f6c8/7bd1f02bc563ba3449d9e08161ac978e"),
            // ("染血的铁假面", "67c7f6c8/4ee009e4cc6361ec133a82abaa46dec7"),

            // ("宗室之花", "67c7f6c8/bd80c45d24f484036927eda40339f730"),
            // ("宗室之翎", "67c7f6c8/7a7f049517a93e3b317a5c5df8892e60"),
            // ("宗室时计", "67c7f6c8/a0a8d92d6fe1d956d5e1a97d6ffcd78d"),
            // ("宗室银瓮", "67c7f6c8/4bde30654f95c0666d3318326260ad45"),
            // ("宗室面具", "67c7f6c8/2e5e66807c6c65eea1918ce6465cdc74"),

            // ("魔女的炎之花", "67c7f6c8/a82e8324d448243daaa56674785ee82f"),
            // ("魔女常燃之羽", "67c7f6c8/3c1d9a36485e88013c09f47f3c21278f"),
            // ("魔女破灭之时", "67c7f6c8/c15763dd94d87979e2996cb7d0c1bb86"),
            // ("魔女的心之火", "67c7f6c8/93cad63c87c2579babcc178e56c21880"),
            // ("焦灼的魔女帽", "67c7f6c8/af4efe37b74a0af38cd48784e415ec89"),

            // ("雷鸟的怜悯", "67c7f6c8/40445cb5a9bdd5c8df397d819b8bc958"),
            // ("雷灾的孑遗", "67c7f6c8/531cdc6e8c3a5f8b7798e90d57d8bc60"),
            // ("雷霆的时计", "67c7f6c8/78f59c06ac14bd2ae3e2150d62b84f9f"),
            // ("降雷的凶兆", "67c7f6c8/f58b5b37605550b7ffefe4b4ee2bcf7d"),
            // ("唤雷的头冠", "67c7f6c8/b4634a741ac5cb248dde25c0159be60c"),

            // ("乐团的晨光", "67c7f6c8/5ff2da2140ec40936c57a7e3c00368c8"),
            // ("琴师的箭羽", "67c7f6c8/70e4bef4acbb4d4789923c3e546aeaf1"),
            // ("终幕的时计", "67c7f6c8/00cfc809485c04ff1092838901c8c413"),
            // ("吟游者之壶", "67c7f6c8/00068fbfdac5602f0ebd3d6a2b85d0bc"),
            // ("指挥的礼帽", "67c7f6c8/83903a2b1f0da3e6aeab6328089c9595"),

            // ("野花记忆的绿野", "67c7f6c8/79119f6a441fc37f39b08f225d63034b"),
            // ("猎人青翠的箭羽", "67c7f6c8/37134a2511467762e3a085ab1d043965"),
            // ("翠绿猎人的笃定", "67c7f6c8/892c53d0973b06b805a48c8d5054f53c"),
            // ("翠绿猎人的容器", "67c7f6c8/cfd075bca5834faa858d8a6c5d6dc95c"),
            // ("翠绿的猎人之冠", "67c7f6c8/f568e5d02b0cfa74365cda3159cfacc8"),

            // ("角斗士的留恋", "67c7f6c8/f5a1fb5232b2985617002eac1bfea8e6"),
            // ("角斗士的归宿", "67c7f6c8/5f120cdac0fc7783d0ba6d1452818d5d"),
            // ("角斗士的希冀", "67c7f6c8/34b9935fadcb083f8b4db3c4fd915754"),
            // ("角斗士的酣醉", "67c7f6c8/0c3a5868f5b77bf7bf8ae6c56e016557"),
            // ("角斗士的凯旋", "67c7f6c8/69750e95511b6aed7919f73b1d568ccd"),

            // ("远方的少女之心", "67c7f6c8/4c8a4c6d40add9e2437f556163501b83"),
            // ("少女飘摇的思念", "67c7f6c8/b5ff9725aa638fdc76d9eaea734d481e"),
            // ("少女苦短的良辰", "67c7f6c8/b8dc97d9352fccfe769738e8d9877252"),
            // ("少女片刻的闲暇", "67c7f6c8/979e1957d22f002ffa7d16d1659107d8"),
            // ("少女易逝的芳颜", "67c7f6c8/37fe25ed4ab1c52a54e0c26017b1cf54"),

            // ("渡火者的决绝", "67c7f6c8/daea64738ee81507c05b9752f51f195f"),
            // ("渡火者的解脱", "67c7f6c8/02a81afce7c220d06d7e7519440d7fc0"),
            // ("渡火者的煎熬", "67c7f6c8/5aa06469a8592156350525ba3ce9a557"),
            // ("渡火者的醒悟", "67c7f6c8/f4645ed7349bbc40f77f9f8d35c6cc6e"),
            // ("渡火者的智慧", "67c7f6c8/9467634b7148d06d369f13556f7b4978"),

            // ("平雷之心", "67c7f6c8/e284d88f35046cb166fc34efc95742b5"),
            // ("平雷之羽", "67c7f6c8/76be91fbc19ea78928f6e224d7ba1ead"),
            // ("平雷之刻", "67c7f6c8/dc8feaf903660254d6b525da5bf9e7f3"),
            // ("平雷之器", "67c7f6c8/f78c7caff87e755b60348e5d57833c00"),
            // ("平雷之冠", "67c7f6c8/3d030b87d3d58e64b4d1a7b9cb26b44f"),

            // ("历经风雪的思念", "67c7f6c8/e92dbe60ede8b9f83ea92e32f96c5b7a"),
            // ("摧冰而行的执望", "67c7f6c8/4c8e83f11bd905ad8dd7089b42ad7ad3"),
            // ("冰雪故园的终期", "67c7f6c8/5fcf01c9f5778dfc6b7fee2ae2b42a28"),
            // ("遍结寒霜的傲骨", "67c7f6c8/01cec5c9e1de97af4f4d2a0638fb1aae"),
            // ("破冰踏雪的回音", "67c7f6c8/9ecabf53a38ca54ef43b7f614b5f9375"),

            // 深廊终曲
            ("深廊的回奏之歌", "67e3357c/7254abd4aa1d0d5ebd33f17f4ab0af83"),
            ("深廊的漫远之约", "67e3357c/c7c0db3816392b3fb6096e02f3c88e14"),
            ("深廊的湮落之刻", "67e3357c/bcbf6747008a3af90cf9975832bb12d4"),
            ("深廊的饫赐之宴", "67e3357c/2144c0530b0e2f615c6eb7c21375d485"),
            ("深廊的遂失之冕", "67e3357c/6e08500ffdc3e7d6d2b8fe9012d95a9d"),

            // 长夜之誓
            ("执灯人的誓词", "67e3357c/d3d62fa78b006194cd5733bda38941a0"),
            ("夜鸣莺的尾羽", "67e3357c/9e871606d845598ab89f190eb7031a28"),
            ("不死者的哀铃", "67e3357c/958e2a4797b3f99ae0bbec58d833b790"),
            ("未吹响的号角", "67e3357c/0a8076edeee37ec87809cfe9fb51aa9b"),
            ("被浸染的缨盔", "67e3357c/e965cd4d4e3ecd6635a30c33e10a6f39"),

            // 黑曜秘典
            ("异种的期许", "67c7f6c8/bfc66e52689915c818557a0a58771adc"),
            ("灵髓的根脉", "67c7f6c8/0fa6b8ce47d8f48358175bdf94de904d"),
            ("夜域的迷思", "67c7f6c8/a182ce8b9d9dae40153eefb35f7cafdc"),
            ("纷争的前宴", "67c7f6c8/29bf5094903e121f6d2bd32950407169"),
            ("诸圣的礼冠", "67c7f6c8/a9b8a46d14acb2869ccba955fe62c81c"),

            // // 烬城勇者绘卷
            ("驯兽师的护符", "67c7f6c8/6763f3ef8cd421d4a0578a5551f61331"),
            ("巡山客的信标", "67c7f6c8/662fe59fa0acfe4ed6e03b8c5cac75c8"),
            ("秘术家的金盘", "67c7f6c8/afc0a5445b4d5273cdfb2eace6b0ed84"),
            ("游学者的爪杯", "67c7f6c8/e2b22f6020c58050976a2990d5deb444"),
            ("魔战士的羽面", "67c7f6c8/3963596306673d681691537252730f20"),

            // 未竟的遐思
            ("暗结的明花", "67c7f6c8/21ce5c571c04379238c36e98cb25b976"),
            ("褪光的翠尾", "67c7f6c8/3fdfcac1cec26c96adbc476efc3c564c"),
            ("举业的识刻", "67c7f6c8/8c9be9b689d13583448064ebb9104f55"),
            ("筹谋的共樽", "67c7f6c8/b9606a5809172d5b48839274419fcf30"),
            ("失冕的宝冠", "67c7f6c8/476fc41f8ead619a2ed7eb27f4072bfa"),

            // 谐律异想断章
            ("谐律交响的前奏", "67c7f6c8/e527117e2b63177aeba18790febb4b4d"),
            ("古海玄幽的夜想", "67c7f6c8/de6621d921029a8ebf5d1d6408dc8002"),
            ("命途轮转的谐谑", "67c7f6c8/92974da7e7bd45194f91066779684e74"),
            ("灵露倾洒的狂诗", "67c7f6c8/eeac67eafb162719cae78cde0a0669f6"),
            ("异想零落的圆舞", "67c7f6c8/61962075f2a5fb626f73956e8405cb51"),

            // 回声之林夜话
            ("无私的妆饰花", "67c7f6c8/33cb16d26cc6c808809f15d3dd8b67ee"),
            ("诚恳的蘸水笔", "67c7f6c8/f128c66d0390cf01114eee44156cff48"),
            ("忠实的砂时计", "67c7f6c8/e0c5337e81b3dbce72b942724e2a089c"),
            ("慷慨的墨水瓶", "67c7f6c8/9a1ad5882746a36ad4911f25137e0ddc"),
            ("慈爱的淑女帽", "67c7f6c8/4cd76a546067070256eae420805bfc12"),

            // 昔时之歌
            ("昔时遗落之誓", "67c7f6c8/345fb98aa962b613664b707d8d3f8146"),
            ("昔时浮想之思", "67c7f6c8/63b4fac64b03a2d8db2252d9b43620bb"),
            ("昔时回映之音", "67c7f6c8/3ad8ff36fc68ca77b699594ee354812e"),
            ("昔时应许之梦", "67c7f6c8/73b30580ecaa03fb98a61e003cce0944"),
            ("昔时传奏之诗", "67c7f6c8/c91dde92910944889fde699f200e7438"),

            // 黄金剧团
            ("黄金乐曲的变奏", "67c7f6c8/0387d4668712fff33d58fb63596cc4df"),
            ("黄金飞鸟的落羽", "67c7f6c8/602ca7b9fb450ee7df76b2d2ecbfb18e"),
            ("黄金时代的先声", "67c7f6c8/623e4ec8f219b8ba5c198444a61ca224"),
            ("黄金之夜的喧嚣", "67c7f6c8/aa223ffb8c29f412b724ce5c7979371a"),
            ("黄金剧团的奖赏", "67c7f6c8/0ce935ed4560fe809ef9334c344853f5"),

            // 逐影猎人
            ("猎人的胸花", "67c7f6c8/6951d1c3be4a5a7ed69b0a3509f68b03"),
            ("杰作的序曲", "67c7f6c8/cb025bb6ec2c1a00ee120de2bed4535b"),
            ("裁判的时刻", "67c7f6c8/ae75aea17fa586b710c213b800c4af6f"),
            ("遗忘的容器", "67c7f6c8/9b8f2f78608d37db8a0a9ec7d7d10c6e"),
            ("老兵的容颜", "67c7f6c8/09542a9f1e2f248f9e473776e477f0a8"),

            // ("灵光源起之蕊", "67c7f6c8/c79204097e23b60d19a7b870979ce54f"),
            // ("琦色灵彩之羽", "67c7f6c8/116941fdc0a447966816da2c4cead29d"),
            // ("久远花落之时", "67c7f6c8/3c8b4bfb6150d33c584c1c4fd04152e6"),
            // ("无边酣乐之筵", "67c7f6c8/a773f40f5edfe1e1ee710d929a6a7c45"),
            // ("灵光明烁之心", "67c7f6c8/32a7b0d18a4dec699b89adaf829e7070"),

            // ("旅途中的鲜花", "67c7f6c8/17dfffacc89923b85f3fa406f041208e"),
            // ("坏巫师的羽杖", "67c7f6c8/5d122057db78d8a79131b77b570105fd"),
            // ("水仙的时时刻刻", "67c7f6c8/1536dc944a1a3525b9261c2f1bef268d"),
            // ("勇者们的茶会", "67c7f6c8/94194b8e2eca8f5e64d14453c45233a5"),
            // ("恶龙的单片镜", "67c7f6c8/139417bf17607d0f0b301b203f180f72"),

            // ("月女的华彩", "67c7f6c8/f5f5285555635320b4a308568540591e"),
            // ("谢落的筵席", "67c7f6c8/6a0d53b1912c99a0b3376694f0874369"),
            // ("凝结的时刻", "67c7f6c8/c7710fdd4bd3523284e5dee98baaf8ac"),
            // ("守秘的魔瓶", "67c7f6c8/3aa07867a4cbc7f9634c6a443823e083"),
            // ("紫晶的花冠", "67c7f6c8/98ac777f42f50bd1cb6f9ad2d3fea671"),

            // ("众王之都的开端", "67c7f6c8/c8fb7d152e4ba53305460154a082ede9"),
            // ("黄金邦国的结末", "67c7f6c8/ceab062a7405267b5ff9b85979bc49e9"),
            // ("失落迷途的机芯", "67c7f6c8/1f50efac4732db6a2bc4bc195cfa5c61"),
            // ("迷醉长梦的守护", "67c7f6c8/3ff2eb86a14a72cf25b919449f84c5aa"),
            // ("流沙贵嗣的遗宝", "67c7f6c8/40ed752ca709ba2d84e0c8674eea82e8"),

            // ("梦中的铁花", "67c7f6c8/857d990ee0b5c27e926f5e3fc642f8ca"),
            // ("裁断的翎羽", "67c7f6c8/f6c8890ea69ba66c882603f03ea20322"),
            // ("沉金的岁月", "67c7f6c8/9f1d53d9f4d0fe7ea1c605cc873dd626"),
            // ("如蜜的终宴", "67c7f6c8/b3ad7ec5ac0a0deb2ebee69dc07400fd"),
            // ("沙王的投影", "67c7f6c8/8734dfe7b60bdaecae7da95903db457e"),

            // ("迷宫的游人", "67c7f6c8/6e1a599dabaefe12d191547469d753cd"),
            // ("翠蔓的智者", "67c7f6c8/c38d8f95cdbfb5d70ac50f60e5520c90"),
            // ("贤智的定期", "67c7f6c8/69ac28067027935b79072bd3dc5319ed"),
            // ("迷误者之灯", "67c7f6c8/ed4d28bc20d96536005c1a50fa9a12b2"),
            // ("月桂的宝冠", "67c7f6c8/874608b484a50855196d6b526d844791"),

            // // 4 star artifacts or below
            // ("魂香之花", "67c7f6c8/45b3135178fa8e47165f7b2000479f45"),
            // ("生灵之华", "67c7f6c8/c3b0db3fda0f57be83087e8fa47f248d"),
            // ("海染之花", "67c7f6c8/5f1381c508febb2a6e39aa87eaa4c5fc"),
            // ("荣花之期", "67c7f6c8/7d2a0cb19ab89de53c88937d6c4648a1"),
            // ("明威之镡", "67c7f6c8/16879d66028f85addecf5a7208f6bffb"),
            // ("羁缠之花", "67c7f6c8/85f548425496394e4b812302b3de514a"),
            // ("无垢之花", "67c7f6c8/038dcfa1e4b0a18dae83a7144448d9d2"),
            // ("勋绩之花", "67c7f6c8/e52f0d8928f3982864ecc8ea13165ebf"),
            // ("饰金胸花", "67c7f6c8/e3f3ff85477d75c6b073dcb941865249"),
            // ("夏祭之花", "67c7f6c8/440a8a8ecc8f0f02645d5484949fca6b"),
            // ("磐陀裂生之花", "67c7f6c8/1da6fb182fbf62736bd20ddbfeb2e485"),
            // ("染血的铁之心", "67c7f6c8/888617693e9828103722622f7c1daca8"),
            // ("宗室之花", "67c7f6c8/bd80c45d24f484036927eda40339f730"),
            // ("魔女的炎之花", "67c7f6c8/a82e8324d448243daaa56674785ee82f"),
            // ("雷鸟的怜悯", "67c7f6c8/40445cb5a9bdd5c8df397d819b8bc958"),
            // ("乐团的晨光", "67c7f6c8/5ff2da2140ec40936c57a7e3c00368c8"),
            // ("野花记忆的绿野", "67c7f6c8/79119f6a441fc37f39b08f225d63034b"),
            // ("角斗士的留恋", "67c7f6c8/f5a1fb5232b2985617002eac1bfea8e6"),
            // ("远方的少女之心", "67c7f6c8/4c8a4c6d40add9e2437f556163501b83"),
            // ("渡火者的决绝", "67c7f6c8/daea64738ee81507c05b9752f51f195f"),
            // ("平雷之心", "67c7f6c8/e284d88f35046cb166fc34efc95742b5"),
            // ("历经风雪的思念", "67c7f6c8/e92dbe60ede8b9f83ea92e32f96c5b7a"),
            // ("学士的书签", "67c7f6c8/61dda917310714c09f7c3831c3e73ae9"),
            // ("流放者之花", "67c7f6c8/e8e8c17bd07ea125961b9d2d6f03b1ba"),
            // ("赌徒的胸花", "67c7f6c8/97d5f989c7fb4ffb2be19b1f9d026ec7"),
            // ("教官的胸花", "67c7f6c8/ab213bb1abd2a5467d680d5ea9890417"),
            // ("武人的红花", "67c7f6c8/49679e24870c25904add4951a604af2d"),
            // ("战狂的蔷薇", "67c7f6c8/105fbe95a8b020b587d75387e5d84854"),
            // ("奇迹之花", "67c7f6c8/f656be051fc7e144d51089c816de44c9"),
            // ("守护之花", "67c7f6c8/5de61d71dbba2abcafed9752305ec61a"),
            // ("勇士的勋章", "67c7f6c8/5c9bcb98cf695bac68ef062be00da240"),
            // ("故人之心", "67c7f6c8/72a97946e22ae54e24de5e92b8fdba7b"),
            // ("深廊的回奏之歌", "67e3357c/7254abd4aa1d0d5ebd33f17f4ab0af83"),
            // ("执灯人的誓词", "67e3357c/d3d62fa78b006194cd5733bda38941a0"),
            // ("异种的期许", "67c7f6c8/bfc66e52689915c818557a0a58771adc"),
            // ("驯兽师的护符", "67c7f6c8/6763f3ef8cd421d4a0578a5551f61331"),
            // ("暗结的明花", "67c7f6c8/21ce5c571c04379238c36e98cb25b976"),
            // ("谐律交响的前奏", "67c7f6c8/e527117e2b63177aeba18790febb4b4d"),
            // ("无私的妆饰花", "67c7f6c8/33cb16d26cc6c808809f15d3dd8b67ee"),
            // ("昔时遗落之誓", "67c7f6c8/345fb98aa962b613664b707d8d3f8146"),
            // ("黄金乐曲的变奏", "67c7f6c8/0387d4668712fff33d58fb63596cc4df"),
            // ("猎人的胸花", "67c7f6c8/6951d1c3be4a5a7ed69b0a3509f68b03"),
            // ("灵光源起之蕊", "67c7f6c8/c79204097e23b60d19a7b870979ce54f"),
            // ("旅途中的鲜花", "67c7f6c8/17dfffacc89923b85f3fa406f041208e"),
            // ("月女的华彩", "67c7f6c8/f5f5285555635320b4a308568540591e"),
            // ("众王之都的开端", "67c7f6c8/c8fb7d152e4ba53305460154a082ede9"),
            // ("梦中的铁花", "67c7f6c8/857d990ee0b5c27e926f5e3fc642f8ca"),
            // ("迷宫的游人", "67c7f6c8/6e1a599dabaefe12d191547469d753cd"),
            // ("游医的银莲", "67c7f6c8/fa54d9a90020567e76638dd48427152e"),
            // ("学士的书签", "67c7f6c8/61dda917310714c09f7c3831c3e73ae9"),
            // ("幸运儿绿花", "67c7f6c8/c8a9f671cef1c22a229f83d4466a4072"),
            // ("冒险家之花", "67c7f6c8/2453598cefddaaa4b072185d23940561"),
            // ("流放者之花", "67c7f6c8/e8e8c17bd07ea125961b9d2d6f03b1ba"),
            // ("赌徒的胸花", "67c7f6c8/97d5f989c7fb4ffb2be19b1f9d026ec7"),
            // ("教官的胸花", "67c7f6c8/ab213bb1abd2a5467d680d5ea9890417"),
            // ("武人的红花", "67c7f6c8/49679e24870c25904add4951a604af2d"),
            // ("战狂的蔷薇", "67c7f6c8/105fbe95a8b020b587d75387e5d84854"),
            // ("奇迹之花", "67c7f6c8/f656be051fc7e144d51089c816de44c9"),
            // ("守护之花", "67c7f6c8/5de61d71dbba2abcafed9752305ec61a"),
            // ("勇士的勋章", "67c7f6c8/5c9bcb98cf695bac68ef062be00da240"),
            // ("故人之心", "67c7f6c8/72a97946e22ae54e24de5e92b8fdba7b"),
            // ("游医的银莲", "67c7f6c8/fa54d9a90020567e76638dd48427152e"),
            // ("幸运儿绿花", "67c7f6c8/c8a9f671cef1c22a229f83d4466a4072"),
            // ("冒险家之花", "67c7f6c8/2453598cefddaaa4b072185d23940561"),
            // ("游医的银莲", "67c7f6c8/fa54d9a90020567e76638dd48427152e"),
            // ("幸运儿绿花", "67c7f6c8/c8a9f671cef1c22a229f83d4466a4072"),
            // ("冒险家之花", "67c7f6c8/2453598cefddaaa4b072185d23940561"),
            // ("垂玉之叶", "67c7f6c8/386663ece74fde7a8ae6a3b61962bfe7"),
            // ("潜光片羽", "67c7f6c8/0fce49c2072b19b474a29a3ea2300bd8"),
            // ("渊宫之羽", "67c7f6c8/edd5f358c027aff093cbb2ef60e8e3b8"),
            // ("华馆之羽", "67c7f6c8/7db5803961686258dbd643be089789fb"),
            // ("切落之羽", "67c7f6c8/35a171a3906cf9789f208628f0f28770"),
            // ("思忆之矢", "67c7f6c8/afcf2a5cf7f130054dac4f31dd7d6c03"),
            // ("贤医之羽", "67c7f6c8/6b4141163c5894064f43c363f0564566"),
            // ("昭武翎羽", "67c7f6c8/84ba5a506941c2e7c67eec4fc91f6e6a"),
            // ("追忆之风", "67c7f6c8/c57c81c236d88176a653c2b0be80d71d"),
            // ("夏祭终末", "67c7f6c8/2fda7dfb07d970fc90db910de890f3e8"),
            // ("嵯峨群峰之翼", "67c7f6c8/5ae89b30076c36972b7609b75cf30f9d"),
            // ("染血的黑之羽", "67c7f6c8/059958b3162dcb5e2df47ff97fbc531f"),
            // ("宗室之翎", "67c7f6c8/7a7f049517a93e3b317a5c5df8892e60"),
            // ("魔女常燃之羽", "67c7f6c8/3c1d9a36485e88013c09f47f3c21278f"),
            // ("雷灾的孑遗", "67c7f6c8/531cdc6e8c3a5f8b7798e90d57d8bc60"),
            // ("琴师的箭羽", "67c7f6c8/70e4bef4acbb4d4789923c3e546aeaf1"),
            // ("猎人青翠的箭羽", "67c7f6c8/37134a2511467762e3a085ab1d043965"),
            // ("角斗士的归宿", "67c7f6c8/5f120cdac0fc7783d0ba6d1452818d5d"),
            // ("少女飘摇的思念", "67c7f6c8/b5ff9725aa638fdc76d9eaea734d481e"),
            // ("渡火者的解脱", "67c7f6c8/02a81afce7c220d06d7e7519440d7fc0"),
            // ("平雷之羽", "67c7f6c8/76be91fbc19ea78928f6e224d7ba1ead"),
            // ("摧冰而行的执望", "67c7f6c8/4c8e83f11bd905ad8dd7089b42ad7ad3"),
            // ("学士的羽笔", "67c7f6c8/5836c0d899ea0ced561c8ed348b72389"),
            // ("流放者之羽", "67c7f6c8/1738ca88f6afa4f1d645e8afb214a7ed"),
            // ("赌徒的羽饰", "67c7f6c8/b0a6c05377b31b94936f56054ca44cc6"),
            // ("教官的羽饰", "67c7f6c8/239b10d2e54e17572a34ac34156f91fa"),
            // ("武人的羽饰", "67c7f6c8/82177d441ec3019934c0063e447dec51"),
            // ("战狂的翎羽", "67c7f6c8/92d2a9153a2f03e1249c17f8530a2aa3"),
            // ("奇迹之羽", "67c7f6c8/5ad7571ae5d7bc7efd9d8d40bf23c95d"),
            // ("守护徽印", "67c7f6c8/47550887f22eddca7e2b19c89c55e107"),
            // ("勇士的期许", "67c7f6c8/859da1624357fdadcceb0a99c58ac495"),
            // ("归乡之羽", "67c7f6c8/f6e5274b9ceb03a8bdc15498def6c269"),
            // ("深廊的漫远之约", "67e3357c/c7c0db3816392b3fb6096e02f3c88e14"),
            // ("夜鸣莺的尾羽", "67e3357c/9e871606d845598ab89f190eb7031a28"),
            // ("灵髓的根脉", "67c7f6c8/0fa6b8ce47d8f48358175bdf94de904d"),
            // ("巡山客的信标", "67c7f6c8/662fe59fa0acfe4ed6e03b8c5cac75c8"),
            // ("褪光的翠尾", "67c7f6c8/3fdfcac1cec26c96adbc476efc3c564c"),
            // ("古海玄幽的夜想", "67c7f6c8/de6621d921029a8ebf5d1d6408dc8002"),
            // ("诚恳的蘸水笔", "67c7f6c8/f128c66d0390cf01114eee44156cff48"),
            // ("昔时浮想之思", "67c7f6c8/63b4fac64b03a2d8db2252d9b43620bb"),
            // ("黄金飞鸟的落羽", "67c7f6c8/602ca7b9fb450ee7df76b2d2ecbfb18e"),
            // ("杰作的序曲", "67c7f6c8/cb025bb6ec2c1a00ee120de2bed4535b"),
            // ("琦色灵彩之羽", "67c7f6c8/116941fdc0a447966816da2c4cead29d"),
            // ("坏巫师的羽杖", "67c7f6c8/5d122057db78d8a79131b77b570105fd"),
            // ("谢落的筵席", "67c7f6c8/6a0d53b1912c99a0b3376694f0874369"),
            // ("黄金邦国的结末", "67c7f6c8/ceab062a7405267b5ff9b85979bc49e9"),
            // ("裁断的翎羽", "67c7f6c8/f6c8890ea69ba66c882603f03ea20322"),
            // ("翠蔓的智者", "67c7f6c8/c38d8f95cdbfb5d70ac50f60e5520c90"),
            // ("游医的枭羽", "67c7f6c8/304cb5f4a543893dbac40af467c1ace0"),
            // ("学士的羽笔", "67c7f6c8/5836c0d899ea0ced561c8ed348b72389"),
            // ("幸运儿鹰羽", "67c7f6c8/05f320836724124e3297a63d7d012257"),
            // ("冒险家尾羽", "67c7f6c8/260c4f71afac2b8d0b57e447687f20ab"),
            // ("流放者之羽", "67c7f6c8/1738ca88f6afa4f1d645e8afb214a7ed"),
            // ("赌徒的羽饰", "67c7f6c8/b0a6c05377b31b94936f56054ca44cc6"),
            // ("教官的羽饰", "67c7f6c8/239b10d2e54e17572a34ac34156f91fa"),
            // ("武人的羽饰", "67c7f6c8/82177d441ec3019934c0063e447dec51"),
            // ("战狂的翎羽", "67c7f6c8/92d2a9153a2f03e1249c17f8530a2aa3"),
            // ("奇迹之羽", "67c7f6c8/5ad7571ae5d7bc7efd9d8d40bf23c95d"),
            // ("守护徽印", "67c7f6c8/47550887f22eddca7e2b19c89c55e107"),
            // ("勇士的期许", "67c7f6c8/859da1624357fdadcceb0a99c58ac495"),
            // ("归乡之羽", "67c7f6c8/f6e5274b9ceb03a8bdc15498def6c269"),
            // ("游医的枭羽", "67c7f6c8/304cb5f4a543893dbac40af467c1ace0"),
            // ("幸运儿鹰羽", "67c7f6c8/05f320836724124e3297a63d7d012257"),
            // ("冒险家尾羽", "67c7f6c8/260c4f71afac2b8d0b57e447687f20ab"),
            // ("游医的枭羽", "67c7f6c8/304cb5f4a543893dbac40af467c1ace0"),
            // ("幸运儿鹰羽", "67c7f6c8/05f320836724124e3297a63d7d012257"),
            // ("冒险家尾羽", "67c7f6c8/260c4f71afac2b8d0b57e447687f20ab"),
            // ("初学者之羽", "67c7f6c8/8cae3a02fbc993f1116816d8a44aa207"),
            // ("祝祀之凭", "67c7f6c8/cd08b3bef2d469fdfc255202a3e1d30a"),
            // ("阳辔之遗", "67c7f6c8/f2b029515fd7e09c1ab913a9aaefad27"),
            // ("离别之贝", "67c7f6c8/e122b3e8efbbe27bceaac08e17818ce4"),
            // ("众生之谣", "67c7f6c8/764223d7f9023c5914c9ef6049fc9e6d"),
            // ("雷云之笼", "67c7f6c8/30ad4db8661bccad21474d67fad9d908"),
            // ("朝露之时", "67c7f6c8/f8a6ac0223534c936003a96d018d498b"),
            // ("停摆之刻", "67c7f6c8/22be2edca2dccded53c85f524b860631"),
            // ("金铜时晷", "67c7f6c8/5e1fe50f98902ba239b743a552c95288"),
            // ("坚铜罗盘", "67c7f6c8/6c8ad61b731cc2c0c53ddd4f22e6a50d"),
            // ("夏祭之刻", "67c7f6c8/8888163740ced1e480057c56dc3f2600"),
            // ("星罗圭璧之晷", "67c7f6c8/e15db34d8698aad900ff66e7bdb0ae33"),
            // ("骑士染血之时", "67c7f6c8/f6f09d13b225b09af60bba4c945cb3df"),
            // ("宗室时计", "67c7f6c8/a0a8d92d6fe1d956d5e1a97d6ffcd78d"),
            // ("魔女破灭之时", "67c7f6c8/c15763dd94d87979e2996cb7d0c1bb86"),
            // ("雷霆的时计", "67c7f6c8/78f59c06ac14bd2ae3e2150d62b84f9f"),
            // ("终幕的时计", "67c7f6c8/00cfc809485c04ff1092838901c8c413"),
            // ("翠绿猎人的笃定", "67c7f6c8/892c53d0973b06b805a48c8d5054f53c"),
            // ("角斗士的希冀", "67c7f6c8/34b9935fadcb083f8b4db3c4fd915754"),
            // ("少女苦短的良辰", "67c7f6c8/b8dc97d9352fccfe769738e8d9877252"),
            // ("渡火者的煎熬", "67c7f6c8/5aa06469a8592156350525ba3ce9a557"),
            // ("平雷之刻", "67c7f6c8/dc8feaf903660254d6b525da5bf9e7f3"),
            // ("冰雪故园的终期", "67c7f6c8/5fcf01c9f5778dfc6b7fee2ae2b42a28"),
            // ("学士的时钟", "67c7f6c8/92ccf908dbf8d21d191893a7bdb391e1"),
            // ("流放者怀表", "67c7f6c8/f9dd8d807eaa9ec5997b9c0e6f7cd48c"),
            // ("赌徒的怀表", "67c7f6c8/c1dbf63dbf97babf57d09a5ced654ca3"),
            // ("教官的怀表", "67c7f6c8/76419f505a01e33f9aa12349802e1d1d"),
            // ("武人的水漏", "67c7f6c8/34ce361cc5a4049ac54e07a20845f4cf"),
            // ("战狂的时计", "67c7f6c8/72514ea9341bf0db03079d0c02ef78b2"),
            // ("奇迹之沙", "67c7f6c8/7f60621adab2f6465dd2abe4400d6428"),
            // ("守护座钟", "67c7f6c8/939533640318f8e38f3e575d8a020128"),
            // ("勇士的坚毅", "67c7f6c8/96a053c7e0e67dfedf7aa8f77724dc87"),
            // ("逐光之石", "67c7f6c8/2e497232c519183b75fd4d0324e89405"),
            // ("深廊的湮落之刻", "67e3357c/bcbf6747008a3af90cf9975832bb12d4"),
            // ("不死者的哀铃", "67e3357c/958e2a4797b3f99ae0bbec58d833b790"),
            // ("夜域的迷思", "67c7f6c8/a182ce8b9d9dae40153eefb35f7cafdc"),
            // ("秘术家的金盘", "67c7f6c8/afc0a5445b4d5273cdfb2eace6b0ed84"),
            // ("举业的识刻", "67c7f6c8/8c9be9b689d13583448064ebb9104f55"),
            // ("命途轮转的谐谑", "67c7f6c8/92974da7e7bd45194f91066779684e74"),
            // ("忠实的砂时计", "67c7f6c8/e0c5337e81b3dbce72b942724e2a089c"),
            // ("昔时回映之音", "67c7f6c8/3ad8ff36fc68ca77b699594ee354812e"),
            // ("黄金时代的先声", "67c7f6c8/623e4ec8f219b8ba5c198444a61ca224"),
            // ("裁判的时刻", "67c7f6c8/ae75aea17fa586b710c213b800c4af6f"),
            // ("久远花落之时", "67c7f6c8/3c8b4bfb6150d33c584c1c4fd04152e6"),
            // ("水仙的时时刻刻", "67c7f6c8/1536dc944a1a3525b9261c2f1bef268d"),
            // ("凝结的时刻", "67c7f6c8/c7710fdd4bd3523284e5dee98baaf8ac"),
            // ("失落迷途的机芯", "67c7f6c8/1f50efac4732db6a2bc4bc195cfa5c61"),
            // ("沉金的岁月", "67c7f6c8/9f1d53d9f4d0fe7ea1c605cc873dd626"),
            // ("贤智的定期", "67c7f6c8/69ac28067027935b79072bd3dc5319ed"),
            // ("游医的怀钟", "67c7f6c8/ee4c4cdd2c99003ad571a92a6f5e4faf"),
            // ("学士的时钟", "67c7f6c8/92ccf908dbf8d21d191893a7bdb391e1"),
            // ("幸运儿沙漏", "67c7f6c8/37dc36c2262a6f6572f212d83aac99e5"),
            // ("冒险家怀表", "67c7f6c8/28990ea9a4bbd88bcc0585fd5132a622"),
            // ("流放者怀表", "67c7f6c8/f9dd8d807eaa9ec5997b9c0e6f7cd48c"),
            // ("赌徒的怀表", "67c7f6c8/c1dbf63dbf97babf57d09a5ced654ca3"),
            // ("教官的怀表", "67c7f6c8/76419f505a01e33f9aa12349802e1d1d"),
            // ("武人的水漏", "67c7f6c8/34ce361cc5a4049ac54e07a20845f4cf"),
            // ("战狂的时计", "67c7f6c8/72514ea9341bf0db03079d0c02ef78b2"),
            // ("奇迹之沙", "67c7f6c8/7f60621adab2f6465dd2abe4400d6428"),
            // ("守护座钟", "67c7f6c8/939533640318f8e38f3e575d8a020128"),
            // ("勇士的坚毅", "67c7f6c8/96a053c7e0e67dfedf7aa8f77724dc87"),
            // ("逐光之石", "67c7f6c8/2e497232c519183b75fd4d0324e89405"),
            // ("游医的怀钟", "67c7f6c8/ee4c4cdd2c99003ad571a92a6f5e4faf"),
            // ("幸运儿沙漏", "67c7f6c8/37dc36c2262a6f6572f212d83aac99e5"),
            // ("冒险家怀表", "67c7f6c8/28990ea9a4bbd88bcc0585fd5132a622"),
            // ("游医的怀钟", "67c7f6c8/ee4c4cdd2c99003ad571a92a6f5e4faf"),
            // ("幸运儿沙漏", "67c7f6c8/37dc36c2262a6f6572f212d83aac99e5"),
            // ("冒险家怀表", "67c7f6c8/28990ea9a4bbd88bcc0585fd5132a622"),
            // ("涌泉之盏", "67c7f6c8/4af742760d499055c53ff308536ca5d9"),
            // ("结契之刻", "67c7f6c8/801da1ab1db5f074ca151126b0f8aa73"),
            // ("真珠之笼", "67c7f6c8/7ebafcb6489bf83e21b7aa53959b7f41"),
            // ("梦醒之瓢", "67c7f6c8/544af5812ec326714c12519b4d8cee97"),
            // ("绯花之壶", "67c7f6c8/24b9a122db4e39e9b9e2eb1cbcf5925f"),
            // ("祈望之心", "67c7f6c8/e6751e19ddf8c4a10632a5999e9395f3"),
            // ("超越之盏", "67c7f6c8/86f0c7fcf7afe2813a3d463ab1a5848f"),
            // ("盟誓金爵", "67c7f6c8/5d25615fe6509385b867e634bebe93ed"),
            // ("沉波之盏", "67c7f6c8/63535d205fedae06e3ed9c403262ccde"),
            // ("夏祭水玉", "67c7f6c8/6cb0e3cb2a3b003c9a206e1e89a9bd41"),
            // ("巉岩琢塑之樽", "67c7f6c8/db301a8934290c15dd2dd43102ec8161"),
            // ("染血骑士之杯", "67c7f6c8/7bd1f02bc563ba3449d9e08161ac978e"),
            // ("宗室银瓮", "67c7f6c8/4bde30654f95c0666d3318326260ad45"),
            // ("魔女的心之火", "67c7f6c8/93cad63c87c2579babcc178e56c21880"),
            // ("降雷的凶兆", "67c7f6c8/f58b5b37605550b7ffefe4b4ee2bcf7d"),
            // ("吟游者之壶", "67c7f6c8/00068fbfdac5602f0ebd3d6a2b85d0bc"),
            // ("翠绿猎人的容器", "67c7f6c8/cfd075bca5834faa858d8a6c5d6dc95c"),
            // ("角斗士的酣醉", "67c7f6c8/0c3a5868f5b77bf7bf8ae6c56e016557"),
            // ("少女片刻的闲暇", "67c7f6c8/979e1957d22f002ffa7d16d1659107d8"),
            // ("渡火者的醒悟", "67c7f6c8/f4645ed7349bbc40f77f9f8d35c6cc6e"),
            // ("平雷之器", "67c7f6c8/f78c7caff87e755b60348e5d57833c00"),
            // ("遍结寒霜的傲骨", "67c7f6c8/01cec5c9e1de97af4f4d2a0638fb1aae"),
            // ("学士的墨杯", "67c7f6c8/b362dcad1b3829e74ca90352924d8250"),
            // ("流放者之杯", "67c7f6c8/7847bfdeab60c072ec09b17462505cdc"),
            // ("赌徒的骰盅", "67c7f6c8/ffc623e9ffd410cd1112c13f44921bd0"),
            // ("教官的茶杯", "67c7f6c8/43b58997aabfaf68f64bde7bfee0c957"),
            // ("武人的酒杯", "67c7f6c8/a41bdd4b58cc77d39b8024be945fe39b"),
            // ("战狂的骨杯", "67c7f6c8/a1c51bbf393862969fb15640d6a433bd"),
            // ("奇迹之杯", "67c7f6c8/f5dbfd5d7ac4583b687b54427f3e9e56"),
            // ("守护之皿", "67c7f6c8/3c90348a691123b829dc82bba5252ffe"),
            // ("勇士的壮行", "67c7f6c8/6f7f2dc13d86c7fbd3c98fffcf0073b6"),
            // ("异国之盏", "67c7f6c8/215e8828639a6b6851a6744c6c1828d4"),
            // ("深廊的饫赐之宴", "67e3357c/2144c0530b0e2f615c6eb7c21375d485"),
            // ("未吹响的号角", "67e3357c/0a8076edeee37ec87809cfe9fb51aa9b"),
            // ("纷争的前宴", "67c7f6c8/29bf5094903e121f6d2bd32950407169"),
            // ("游学者的爪杯", "67c7f6c8/e2b22f6020c58050976a2990d5deb444"),
            // ("筹谋的共樽", "67c7f6c8/b9606a5809172d5b48839274419fcf30"),
            // ("灵露倾洒的狂诗", "67c7f6c8/eeac67eafb162719cae78cde0a0669f6"),
            // ("慷慨的墨水瓶", "67c7f6c8/9a1ad5882746a36ad4911f25137e0ddc"),
            // ("昔时应许之梦", "67c7f6c8/73b30580ecaa03fb98a61e003cce0944"),
            // ("黄金之夜的喧嚣", "67c7f6c8/aa223ffb8c29f412b724ce5c7979371a"),
            // ("遗忘的容器", "67c7f6c8/9b8f2f78608d37db8a0a9ec7d7d10c6e"),
            // ("无边酣乐之筵", "67c7f6c8/a773f40f5edfe1e1ee710d929a6a7c45"),
            // ("勇者们的茶会", "67c7f6c8/94194b8e2eca8f5e64d14453c45233a5"),
            // ("守秘的魔瓶", "67c7f6c8/3aa07867a4cbc7f9634c6a443823e083"),
            // ("迷醉长梦的守护", "67c7f6c8/3ff2eb86a14a72cf25b919449f84c5aa"),
            // ("如蜜的终宴", "67c7f6c8/b3ad7ec5ac0a0deb2ebee69dc07400fd"),
            // ("迷误者之灯", "67c7f6c8/ed4d28bc20d96536005c1a50fa9a12b2"),
            // ("游医的药壶", "67c7f6c8/aea9c1751e0e454042c67703102cb0d8"),
            // ("学士的墨杯", "67c7f6c8/b362dcad1b3829e74ca90352924d8250"),
            // ("幸运儿之杯", "67c7f6c8/b42512d4d511944707d69e587465cd6c"),
            // ("冒险家金杯", "67c7f6c8/f10753b76fb044aca89b5673aa550f73"),
            // ("流放者之杯", "67c7f6c8/7847bfdeab60c072ec09b17462505cdc"),
            // ("赌徒的骰盅", "67c7f6c8/ffc623e9ffd410cd1112c13f44921bd0"),
            // ("教官的茶杯", "67c7f6c8/43b58997aabfaf68f64bde7bfee0c957"),
            // ("武人的酒杯", "67c7f6c8/a41bdd4b58cc77d39b8024be945fe39b"),
            // ("战狂的骨杯", "67c7f6c8/a1c51bbf393862969fb15640d6a433bd"),
            // ("奇迹之杯", "67c7f6c8/f5dbfd5d7ac4583b687b54427f3e9e56"),
            // ("守护之皿", "67c7f6c8/3c90348a691123b829dc82bba5252ffe"),
            // ("勇士的壮行", "67c7f6c8/6f7f2dc13d86c7fbd3c98fffcf0073b6"),
            // ("异国之盏", "67c7f6c8/215e8828639a6b6851a6744c6c1828d4"),
            // ("游医的药壶", "67c7f6c8/aea9c1751e0e454042c67703102cb0d8"),
            // ("幸运儿之杯", "67c7f6c8/b42512d4d511944707d69e587465cd6c"),
            // ("冒险家金杯", "67c7f6c8/f10753b76fb044aca89b5673aa550f73"),
            // ("游医的药壶", "67c7f6c8/aea9c1751e0e454042c67703102cb0d8"),
            // ("幸运儿之杯", "67c7f6c8/b42512d4d511944707d69e587465cd6c"),
            // ("冒险家金杯", "67c7f6c8/f10753b76fb044aca89b5673aa550f73"),
            // ("浮溯之珏", "67c7f6c8/8d6e15e10a7f05c4ad0f961d8d74ef2c"),
            // ("虺雷之姿", "67c7f6c8/e9c3254cf0f4952b3e55c6b3b66f9e11"),
            // ("海祇之冠", "67c7f6c8/ae87d46924dfee7cde4379f879c6e4c9"),
            // ("形骸之笠", "67c7f6c8/2f1dcfffae4b7ca51bc8b5a89cec6844"),
            // ("华饰之兜", "67c7f6c8/8af0dc3497b273ccdee34d2cc948aa59"),
            // ("无常之面", "67c7f6c8/71e9ec339457ff7e274469ddb97889f4"),
            // ("嗤笑之面", "67c7f6c8/0fa0f2fe45f925ef624bf054787dba0c"),
            // ("将帅兜鍪", "67c7f6c8/dcdf1f408c923028f4565bcb1f2d12ae"),
            // ("酒渍船帽", "67c7f6c8/a6efed58b340e9aea774bd7d53544bc7"),
            // ("夏祭之面", "67c7f6c8/baa6786cda943cb2800d6ca7d140c5c8"),
            // ("不动玄石之相", "67c7f6c8/ab5f9dccce3d0e29617e41b4c21634ad"),
            // ("祭冰礼冠", "67c7f6c8/d48c6338496d67d5c29e7a900a454294"),
            // ("祭雷礼冠", "67c7f6c8/647f229d2e0eac145c2c44a0705eebaa"),
            // ("祭水礼冠", "67c7f6c8/cc44abdb29d3da19138f52a514e84829"),
            // ("祭火礼冠", "67c7f6c8/731c6674c20e505a2614a5c75986bc9b"),
            // ("染血的铁假面", "67c7f6c8/4ee009e4cc6361ec133a82abaa46dec7"),
            // ("宗室面具", "67c7f6c8/2e5e66807c6c65eea1918ce6465cdc74"),
            // ("焦灼的魔女帽", "67c7f6c8/af4efe37b74a0af38cd48784e415ec89"),
            // ("唤雷的头冠", "67c7f6c8/b4634a741ac5cb248dde25c0159be60c"),
            // ("指挥的礼帽", "67c7f6c8/83903a2b1f0da3e6aeab6328089c9595"),
            // ("翠绿的猎人之冠", "67c7f6c8/f568e5d02b0cfa74365cda3159cfacc8"),
            // ("角斗士的凯旋", "67c7f6c8/69750e95511b6aed7919f73b1d568ccd"),
            // ("少女易逝的芳颜", "67c7f6c8/37fe25ed4ab1c52a54e0c26017b1cf54"),
            // ("渡火者的智慧", "67c7f6c8/9467634b7148d06d369f13556f7b4978"),
            // ("平雷之冠", "67c7f6c8/3d030b87d3d58e64b4d1a7b9cb26b44f"),
            // ("破冰踏雪的回音", "67c7f6c8/9ecabf53a38ca54ef43b7f614b5f9375"),
            // ("学士的镜片", "67c7f6c8/64b60bb7b2b1e35e7ece981c3fcf9134"),
            // ("流放者头冠", "67c7f6c8/22faf0f5c42ac3652ea2c4a1ca38ac05"),
            // ("赌徒的耳环", "67c7f6c8/63a809f9e4a6e051bf3a44f002231e22"),
            // ("教官的帽子", "67c7f6c8/13dc11a5b26e2890847a8f86e07061a7"),
            // ("武人的头巾", "67c7f6c8/a47393ddf72a5af6eb1eedba49f56940"),
            // ("战狂的鬼面", "67c7f6c8/ab750706998dc9d99ed2067a722f9c52"),
            // ("奇迹耳坠", "67c7f6c8/e12b1ed9664165c0413364e00f000021"),
            // ("守护束带", "67c7f6c8/7a64ac846deca47c0c8259d816ead244"),
            // ("勇士的冠冕", "67c7f6c8/7ad38a5c2ce6d3e7e47cb5a830c83cb4"),
            // ("感别之冠", "67c7f6c8/b0823e0c41df019706406112283256fd"),
            // ("深廊的遂失之冕", "67e3357c/6e08500ffdc3e7d6d2b8fe9012d95a9d"),
            // ("被浸染的缨盔", "67e3357c/e965cd4d4e3ecd6635a30c33e10a6f39"),
            // ("诸圣的礼冠", "67c7f6c8/a9b8a46d14acb2869ccba955fe62c81c"),
            // ("魔战士的羽面", "67c7f6c8/3963596306673d681691537252730f20"),
            // ("失冕的宝冠", "67c7f6c8/476fc41f8ead619a2ed7eb27f4072bfa"),
            // ("异想零落的圆舞", "67c7f6c8/61962075f2a5fb626f73956e8405cb51"),
            // ("慈爱的淑女帽", "67c7f6c8/4cd76a546067070256eae420805bfc12"),
            // ("昔时传奏之诗", "67c7f6c8/c91dde92910944889fde699f200e7438"),
            // ("黄金剧团的奖赏", "67c7f6c8/0ce935ed4560fe809ef9334c344853f5"),
            // ("老兵的容颜", "67c7f6c8/09542a9f1e2f248f9e473776e477f0a8"),
            // ("灵光明烁之心", "67c7f6c8/32a7b0d18a4dec699b89adaf829e7070"),
            // ("恶龙的单片镜", "67c7f6c8/139417bf17607d0f0b301b203f180f72"),
            // ("紫晶的花冠", "67c7f6c8/98ac777f42f50bd1cb6f9ad2d3fea671"),
            // ("流沙贵嗣的遗宝", "67c7f6c8/40ed752ca709ba2d84e0c8674eea82e8"),
            // ("沙王的投影", "67c7f6c8/8734dfe7b60bdaecae7da95903db457e"),
            // ("月桂的宝冠", "67c7f6c8/874608b484a50855196d6b526d844791"),
            // ("祭冰礼冠", "67c7f6c8/d48c6338496d67d5c29e7a900a454294"),
            // ("祭雷礼冠", "67c7f6c8/647f229d2e0eac145c2c44a0705eebaa"),
            // ("祭水礼冠", "67c7f6c8/cc44abdb29d3da19138f52a514e84829"),
            // ("祭火礼冠", "67c7f6c8/731c6674c20e505a2614a5c75986bc9b"),
            // ("游医的方巾", "67c7f6c8/0d49358014a33b1cadf4947455373c56"),
            // ("学士的镜片", "67c7f6c8/64b60bb7b2b1e35e7ece981c3fcf9134"),
            // ("幸运儿银冠", "67c7f6c8/b41981a2d335aff25dca11cb416518df"),
            // ("冒险家头带", "67c7f6c8/2e4915a7fccba739312194886d2c0c9a"),
            // ("流放者头冠", "67c7f6c8/22faf0f5c42ac3652ea2c4a1ca38ac05"),
            // ("赌徒的耳环", "67c7f6c8/63a809f9e4a6e051bf3a44f002231e22"),
            // ("教官的帽子", "67c7f6c8/13dc11a5b26e2890847a8f86e07061a7"),
            // ("武人的头巾", "67c7f6c8/a47393ddf72a5af6eb1eedba49f56940"),
            // ("战狂的鬼面", "67c7f6c8/ab750706998dc9d99ed2067a722f9c52"),
            // ("奇迹耳坠", "67c7f6c8/e12b1ed9664165c0413364e00f000021"),
            // ("守护束带", "67c7f6c8/7a64ac846deca47c0c8259d816ead244"),
            // ("勇士的冠冕", "67c7f6c8/7ad38a5c2ce6d3e7e47cb5a830c83cb4"),
            // ("感别之冠", "67c7f6c8/b0823e0c41df019706406112283256fd"),
            // ("游医的方巾", "67c7f6c8/0d49358014a33b1cadf4947455373c56"),
            // ("幸运儿银冠", "67c7f6c8/b41981a2d335aff25dca11cb416518df"),
            // ("冒险家头带", "67c7f6c8/2e4915a7fccba739312194886d2c0c9a"),
            // ("游医的方巾", "67c7f6c8/0d49358014a33b1cadf4947455373c56"),
            // ("幸运儿银冠", "67c7f6c8/b41981a2d335aff25dca11cb416518df"),
            // ("冒险家头带", "67c7f6c8/2e4915a7fccba739312194886d2c0c9a"),

            // OLD DATA!
            // // 5 Star Characters
            // ("Mualani", "43b237af6075b02d0ed6bb0d41e5bf6b"),
            // ("Emilie", "8eca21f28d282fc38578de3a63205a39"),
            // ("Clorinde", "d90efaef8591d45982ca714b27e2faaa"),
            // ("Arlecchino", "5af48ffeeb3cd9e2ef6a5a245ff5a502"),
            // ("Sigewinne", "d6228cee1b60f3e8fc3fd041ec8264ca"),
            // ("Chiori", "0b23eaf3248026bfb9538ccc302dc2e6"),
            // ("Xianyun", "1063a94eb80ebba3683821f393987d93"),
            // ("Navia", "4ecdeaddd3f4de79beaced7c449b9c6f"),
            // ("Furina", "012270473e54fd09aa6ef272f3a050bb"),
            // ("Neuvillette", "1b877ac8988c675cb7b3b905544aad7c"),
            // ("Wriothesley", "a7c3b6949ef42a1cf37cd3be87c34112"),
            // ("Lyney", "16a33017e331f2768f486a5667cd0318"),
            // // 4 Star Characters
            // ("Kachina", "65bda5c712894dedc44a5b7f0e2560ff"),
            // ("Sethos", "73f63b00c1bbe4c061e89a22765c9f93"),
            // ("Gaming", "492ef1d89fbb3519c6b6882e42499ba3"),
            // ("Chevreuse", "f74d07754a17a5025e62cbf0411ba8a0"),
            // ("Charlotte", "9199f0769de996add5c660f4b172ee36"),
            // ("Freminet", "40b78f33038bedf0e470a4a87c6fc968"),
            // ("Lynette", "fd29fb6cb44268355f9a22153341019d"),
            // ("Kirara", "2099863b9d6f7678002a3d7bc1c9450a"),

            // // 5 Star Weapons Bow
            // ("SilvershowerHeartstrings", "18d810e1ac439a81f371504efe7d8afe"),
            // ("TheFirstGreatMagic", "686ed9187ebb59d480917881625f0eb6"),
            // // 5 Star Weapons Catalyst
            // ("SurfsUp", "8d980ddba3723c95bb4795d554bf1640"),
            // ("CranesEchoingCall", "1f38d88e8b34c55143d3d48e902e32dc"),
            // ("TomeOfTheEternalFlow", "2c356588aba222181fda66809a9a425f"),
            // ("CashflowSupervision", "a8002c6d521d8fca48bc1402d7ce5e2b"),
            // // 5 Star Weapons Pole
            // ("LumidouceElegy", "a6fa4ef0c82b6f9c3f824953f7c16acf"),
            // ("CrimsonMoonsSemblance", "94175be57ec8e6c749a6ac22cccde18d"),
            // // 5 Star Weapons Claymore
            // ("Verdict", "f5336c01b0f1e19833a0f9e8bd04c107"),
            // // 5 Star Weapons Sword
            // ("Absolution", "72c9336d54b4653fed5e529aec9b8403"),
            // ("UrakuMisugiri", "a8f87169e43884ef394d23e9857d2bc5"),
            // ("SplendorOfTranquilWaters", "2db4d0468f073acfd6cd9b483d054721"),
            // // 4 Star Weapons Bow
            // ("ChainBreaker", "cc9373e33444d8e6d8a9b1721572f14d"),
            // ("RangeGauge", "3faab532af8b27a55f6450958c9afff0"),
            // ("Cloudforged", "6c9144ab2347a6750252e6938bba99af"),
            // ("SongOfStillness", "b11d52b7a92f661c313fbc3af1974157"),
            // ("ScionOfTheBlazingSun", "8f069e0404cd6d99b4d43e88dce0425f"),
            // ("IbisPiercer", "565c9a5438885c0cb05e560bcd68f474"),
            // // 4 Star Weapons Catalyst
            // ("RingOfYaxche", "087a49cb371e1953d01f1923f32462ba"),
            // ("AshgravenDrinkingHorn", "e9ba031389e067924e2fa035f9790da1"),
            // ("BalladOfTheBoundlessBlue", "705b9cf993f46e1333f24268c0549f3a"),
            // ("FlowingPurity", "20ecbf9b91f08b79d79299ab9663226b"),
            // ("SacrificialJade", "a91df42cd2802cab8477faba2b6e7ca9"),
            // // 4 Star Weapons Pole
            // ("FootprintOfTheRainbow", "a861e8083fcf30d1761e566e22868fd2"),
            // ("ProspectorsDrill", "e61d4d8a61fb5c968761312f18ed7812"),
            // ("DialoguesOfTheDesertSages", "95ed6f690c3e8a747d57a898547cce7a"),
            // ("RightfulReward", "1a5faad17eb91402baff10bd7820fe03"),
            // ("BalladOfTheFjords", "18f2b5997c222617c28b739eb92de1c1"),
            // // 4 Star Weapons Claymore
            // ("EarthShaker", "8157457687dcb9aa1fe3016e6bfb5836"),
            // ("PortablePowerSaw", "f1c68aba7315be5f0c41901590661c9b"),
            // ("UltimateOverlordsMegaMagicSword", "17209097a4d95ec61cb7bf14bd7ab85a"),
            // ("TidalShadow", "0ecc6129519b29b41ad796f92cac9a02"),
            // ("TalkingStick", "35e6503456cd18c40b4a4e856f8f78b4"),
            // // 4 Star Weapons Sword
            // ("FluteOfEzpitzal", "e0a4036946e1e9fa86a95e4ae5d8f937"),
            // ("SwordOfNarzissenkreuz", "6493e5f44ba0bfeee95190c67ce2ce6a"),
            // ("TheDockhandsAssistant", "9ee3b1bd75d9efda391a03aa99dcf1f2"),
            // ("FleuveCendreFerryman", "ce6493b62ee52f371173651b3257a5f9"),
            // ("FinaleOfTheDeep", "613168d81592aa37a10f16bb87a78c85"),
            // // 黑曜秘典
            // ("UI_RelicIcon_15038_4", "e9c42c8132bc360bf465d5f45a5a3dc0"),
            // ("UI_RelicIcon_15038_5", "74684390899b5f05c1648b9404f6d7a2"),
            // ("UI_RelicIcon_15038_3", "29b282b1b9bbe082874fb943aab6a405"),
            // ("UI_RelicIcon_15038_2", "15dfd957828fdfa880424238059b6574"),
            // ("UI_RelicIcon_15038_1", "5f0b9ae4663323fa3a448c406cbbb112"),
            // // 烬城勇者绘卷
            // ("UI_RelicIcon_15037_4", "07398d19576ee60535eb7552c943e738"),
            // ("UI_RelicIcon_15037_5", "dc8f3d29e46ac5c5fca3cc03ea240db1"),
            // ("UI_RelicIcon_15037_3", "daf989272d20ab0411b5408f8504a394"),
            // ("UI_RelicIcon_15037_2", "30c45b49df7b7f2fdeeded5514b10fd9"),
            // ("UI_RelicIcon_15037_1", "8c3542654bd72a5f27030508d7917a6e"),
            // // 未竟的遐思
            // ("UI_RelicIcon_15036_4", "4feb283bbc23e58e9265e18b371c467a"),
            // ("UI_RelicIcon_15036_5", "05300f93fe8cddc4973950f6806e1adb"),
            // ("UI_RelicIcon_15036_3", "6932025ba8b59e2c531441a00c08b572"),
            // ("UI_RelicIcon_15036_2", "33e4aefc97fdff22388249565e0d996e"),
            // ("UI_RelicIcon_15036_1", "9e301f350df51d3360e109e771d44534"),
            // // 谐律异想断章
            // ("UI_RelicIcon_15035_4", "08b16dee4e1e04e13ed33258a34b893a"),
            // ("UI_RelicIcon_15035_5", "074052fa955b62bdaf9435b5fde93fd8"),
            // ("UI_RelicIcon_15035_3", "2a21972e9b89f504c7444fb4a439d9b6"),
            // ("UI_RelicIcon_15035_2", "dfb10d691bffc51c66ac3ccbfb35cce1"),
            // ("UI_RelicIcon_15035_1", "6df7f8173e5869e349eb43d3dd1f72bd"),
            // // 回声之林夜话
            // ("UI_RelicIcon_15034_4", "18b4d9a2f4f3c3c31811879e2e61a11b"),
            // ("UI_RelicIcon_15034_5", "97a19394aa477d8812b6ad3b6f29c650"),
            // ("UI_RelicIcon_15034_3", "ee73f8b13beb3b539593644541dd30ec"),
            // ("UI_RelicIcon_15034_2", "117cb2fbd79f19aa84ec4798eceea2fb"),
            // ("UI_RelicIcon_15034_1", "8697bbf7ae136b67a19797803e3b62ac"),
            // // 昔时之歌
            // ("UI_RelicIcon_15033_4", "02b777d53c28acf5c25efe0c079b0e51"),
            // ("UI_RelicIcon_15033_5", "92822eee6aef16124898cd1a2d796fc8"),
            // ("UI_RelicIcon_15033_3", "8956afdebb6dbd994b881d3a3b4527cc"),
            // ("UI_RelicIcon_15033_2", "b85d969818f94125ec020bb04c697bcf"),
            // ("UI_RelicIcon_15033_1", "d905b2caa4ade21a0312a38b0a565a7c"),
            // // 黄金剧团
            // ("UI_RelicIcon_15032_4", "9c3e75b95befcea2afa110828a2b5679"),
            // ("UI_RelicIcon_15032_5", "be4fa798584c3e6868a228f7e54cbfde"),
            // ("UI_RelicIcon_15032_3", "45d337eaca981b4b3e00d704f6e11c95"),
            // ("UI_RelicIcon_15032_2", "ed99c2a85aca30efdcea7ab2242ac3c1"),
            // ("UI_RelicIcon_15032_1", "ae3867e36dba71d529520d12491c934e"),
            // // 逐影猎人
            // ("UI_RelicIcon_15031_4", "7e4df1daa13237119fd5d789b137b427"),
            // ("UI_RelicIcon_15031_5", "764fda52bb26c4e84510b8a21d4f036b"),
            // ("UI_RelicIcon_15031_3", "45ae02ac98e0a5863ccf35bba707afac"),
            // ("UI_RelicIcon_15031_2", "951e55a31658078648386a4917af39ca"),
            // ("UI_RelicIcon_15031_1", "b613fcca1f28ec0a3f9ee39cffe452cf"),
        ];

        data.into_iter().collect()
    };
}
