use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ICON_HASHMAP: HashMap<&'static str, &'static str> = {
        let data = vec![
            // 5 Star Characters
            ("Mualani", "43b237af6075b02d0ed6bb0d41e5bf6b"),
            ("Emilie", "8eca21f28d282fc38578de3a63205a39"),
            ("Clorinde", "d90efaef8591d45982ca714b27e2faaa"),
            ("Arlecchino", "5af48ffeeb3cd9e2ef6a5a245ff5a502"),
            ("Sigewinne", "d6228cee1b60f3e8fc3fd041ec8264ca"),
            ("Chiori", "0b23eaf3248026bfb9538ccc302dc2e6"),
            ("Xianyun", "1063a94eb80ebba3683821f393987d93"),
            ("Navia", "4ecdeaddd3f4de79beaced7c449b9c6f"),
            ("Furina", "012270473e54fd09aa6ef272f3a050bb"),
            ("Neuvillette", "1b877ac8988c675cb7b3b905544aad7c"),
            ("Wriothesley", "a7c3b6949ef42a1cf37cd3be87c34112"),
            ("Lyney", "16a33017e331f2768f486a5667cd0318"),
            // 4 Star Characters
            ("Kachina", "65bda5c712894dedc44a5b7f0e2560ff"),
            ("Sethos", "73f63b00c1bbe4c061e89a22765c9f93"),
            ("Gaming", "492ef1d89fbb3519c6b6882e42499ba3"),
            ("Chevreuse", "f74d07754a17a5025e62cbf0411ba8a0"),
            ("Charlotte", "9199f0769de996add5c660f4b172ee36"),
            ("Freminet", "40b78f33038bedf0e470a4a87c6fc968"),
            ("Lynette", "fd29fb6cb44268355f9a22153341019d"),
            ("Kirara", "2099863b9d6f7678002a3d7bc1c9450a"),
            // 5 Star Weapons Bow
            ("SilvershowerHeartstrings", "18d810e1ac439a81f371504efe7d8afe"),
            ("TheFirstGreatMagic", "686ed9187ebb59d480917881625f0eb6"),
            // 5 Star Weapons Catalyst
            ("SurfsUp", "8d980ddba3723c95bb4795d554bf1640"),
            ("CranesEchoingCall", "1f38d88e8b34c55143d3d48e902e32dc"),
            ("TomeOfTheEternalFlow", "2c356588aba222181fda66809a9a425f"),
            ("CashflowSupervision", "a8002c6d521d8fca48bc1402d7ce5e2b"),
            // 5 Star Weapons Pole
            ("LumidouceElegy", "a6fa4ef0c82b6f9c3f824953f7c16acf"),
            ("CrimsonMoonsSemblance", "94175be57ec8e6c749a6ac22cccde18d"),
            // 5 Star Weapons Claymore
            ("Verdict", "f5336c01b0f1e19833a0f9e8bd04c107"),
            // 5 Star Weapons Sword
            ("Absolution", "72c9336d54b4653fed5e529aec9b8403"),
            ("UrakuMisugiri", "a8f87169e43884ef394d23e9857d2bc5"),
            ("SplendorOfTranquilWaters", "2db4d0468f073acfd6cd9b483d054721"),
            // 4 Star Weapons Bow
            ("ChainBreaker", "cc9373e33444d8e6d8a9b1721572f14d"),
            ("RangeGauge", "3faab532af8b27a55f6450958c9afff0"),
            ("Cloudforged", "6c9144ab2347a6750252e6938bba99af"),
            ("SongOfStillness", "b11d52b7a92f661c313fbc3af1974157"),
            ("ScionOfTheBlazingSun", "8f069e0404cd6d99b4d43e88dce0425f"),
            ("IbisPiercer", "565c9a5438885c0cb05e560bcd68f474"),
            // 4 Star Weapons Catalyst
            ("RingOfYaxche", "087a49cb371e1953d01f1923f32462ba"),
            ("AshgravenDrinkingHorn", "e9ba031389e067924e2fa035f9790da1"),
            ("BalladOfTheBoundlessBlue", "705b9cf993f46e1333f24268c0549f3a"),
            ("FlowingPurity", "20ecbf9b91f08b79d79299ab9663226b"),
            ("SacrificialJade", "a91df42cd2802cab8477faba2b6e7ca9"),
            // 4 Star Weapons Pole
            ("FootprintOfTheRainbow", "a861e8083fcf30d1761e566e22868fd2"),
            ("ProspectorsDrill", "e61d4d8a61fb5c968761312f18ed7812"),
            ("DialoguesOfTheDesertSages", "95ed6f690c3e8a747d57a898547cce7a"),
            ("RightfulReward", "1a5faad17eb91402baff10bd7820fe03"),
            ("BalladOfTheFjords", "18f2b5997c222617c28b739eb92de1c1"),
            // 4 Star Weapons Claymore
            ("EarthShaker", "8157457687dcb9aa1fe3016e6bfb5836"),
            ("PortablePowerSaw", "f1c68aba7315be5f0c41901590661c9b"),
            ("UltimateOverlordsMegaMagicSword", "17209097a4d95ec61cb7bf14bd7ab85a"),
            ("TidalShadow", "0ecc6129519b29b41ad796f92cac9a02"),
            ("TalkingStick", "35e6503456cd18c40b4a4e856f8f78b4"),
            // 4 Star Weapons Sword
            ("FluteOfEzpitzal", "e0a4036946e1e9fa86a95e4ae5d8f937"),
            ("SwordOfNarzissenkreuz", "6493e5f44ba0bfeee95190c67ce2ce6a"),
            ("TheDockhandsAssistant", "9ee3b1bd75d9efda391a03aa99dcf1f2"),
            ("FleuveCendreFerryman", "ce6493b62ee52f371173651b3257a5f9"),
            ("FinaleOfTheDeep", "613168d81592aa37a10f16bb87a78c85"),
            // 黑曜秘典
            ("UI_RelicIcon_15038_4", "e9c42c8132bc360bf465d5f45a5a3dc0"),
            ("UI_RelicIcon_15038_5", "74684390899b5f05c1648b9404f6d7a2"),
            ("UI_RelicIcon_15038_3", "29b282b1b9bbe082874fb943aab6a405"),
            ("UI_RelicIcon_15038_2", "15dfd957828fdfa880424238059b6574"),
            ("UI_RelicIcon_15038_1", "5f0b9ae4663323fa3a448c406cbbb112"),
            // 烬城勇者绘卷
            ("UI_RelicIcon_15037_4", "07398d19576ee60535eb7552c943e738"),
            ("UI_RelicIcon_15037_5", "dc8f3d29e46ac5c5fca3cc03ea240db1"),
            ("UI_RelicIcon_15037_3", "daf989272d20ab0411b5408f8504a394"),
            ("UI_RelicIcon_15037_2", "30c45b49df7b7f2fdeeded5514b10fd9"),
            ("UI_RelicIcon_15037_1", "8c3542654bd72a5f27030508d7917a6e"),
            // 未竟的遐思
            ("UI_RelicIcon_15036_4", "4feb283bbc23e58e9265e18b371c467a"),
            ("UI_RelicIcon_15036_5", "05300f93fe8cddc4973950f6806e1adb"),
            ("UI_RelicIcon_15036_3", "6932025ba8b59e2c531441a00c08b572"),
            ("UI_RelicIcon_15036_2", "33e4aefc97fdff22388249565e0d996e"),
            ("UI_RelicIcon_15036_1", "9e301f350df51d3360e109e771d44534"),
            // 谐律异想断章
            ("UI_RelicIcon_15035_4", "08b16dee4e1e04e13ed33258a34b893a"),
            ("UI_RelicIcon_15035_5", "074052fa955b62bdaf9435b5fde93fd8"),
            ("UI_RelicIcon_15035_3", "2a21972e9b89f504c7444fb4a439d9b6"),
            ("UI_RelicIcon_15035_2", "dfb10d691bffc51c66ac3ccbfb35cce1"),
            ("UI_RelicIcon_15035_1", "6df7f8173e5869e349eb43d3dd1f72bd"),
            // 回声之林夜话
            ("UI_RelicIcon_15034_4", "18b4d9a2f4f3c3c31811879e2e61a11b"),
            ("UI_RelicIcon_15034_5", "97a19394aa477d8812b6ad3b6f29c650"),
            ("UI_RelicIcon_15034_3", "ee73f8b13beb3b539593644541dd30ec"),
            ("UI_RelicIcon_15034_2", "117cb2fbd79f19aa84ec4798eceea2fb"),
            ("UI_RelicIcon_15034_1", "8697bbf7ae136b67a19797803e3b62ac"),
            // 昔时之歌
            ("UI_RelicIcon_15033_4", "02b777d53c28acf5c25efe0c079b0e51"),
            ("UI_RelicIcon_15033_5", "92822eee6aef16124898cd1a2d796fc8"),
            ("UI_RelicIcon_15033_3", "8956afdebb6dbd994b881d3a3b4527cc"),
            ("UI_RelicIcon_15033_2", "b85d969818f94125ec020bb04c697bcf"),
            ("UI_RelicIcon_15033_1", "d905b2caa4ade21a0312a38b0a565a7c"),
            // 黄金剧团
            ("UI_RelicIcon_15032_4", "9c3e75b95befcea2afa110828a2b5679"),
            ("UI_RelicIcon_15032_5", "be4fa798584c3e6868a228f7e54cbfde"),
            ("UI_RelicIcon_15032_3", "45d337eaca981b4b3e00d704f6e11c95"),
            ("UI_RelicIcon_15032_2", "ed99c2a85aca30efdcea7ab2242ac3c1"),
            ("UI_RelicIcon_15032_1", "ae3867e36dba71d529520d12491c934e"),
            // 逐影猎人
            ("UI_RelicIcon_15031_4", "7e4df1daa13237119fd5d789b137b427"),
            ("UI_RelicIcon_15031_5", "764fda52bb26c4e84510b8a21d4f036b"),
            ("UI_RelicIcon_15031_3", "45ae02ac98e0a5863ccf35bba707afac"),
            ("UI_RelicIcon_15031_2", "951e55a31658078648386a4917af39ca"),
            ("UI_RelicIcon_15031_1", "b613fcca1f28ec0a3f9ee39cffe452cf"),
        ];

        data.into_iter().collect()
    };
}
