use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ICON_HASHMAP: HashMap<&'static str, &'static str> = {
        let data = vec![
            ("Furina", "4da8d9d663e2e59f63c19815074074de"),
            ("Neuvillette", "965af2f32a5376affcb99afb9915a23d"),
            ("Wriothesley", "e2ea36ecfdb6f53717b1cadd394fbf49"),
            ("Lyney", "15c0fae62ec91222148b753e5445c5fe"),
            ("Charlotte", "99218c303fd1ca9cc53e052ebdd5cbf3"),
            ("Freminet", "7ca0ad25c2cbb36cd55a8a19c1b2a39f"),
            ("Lynette", "3163e147dc45dec9944d06355d778879"),
            ("Momoka", "8e8c8c18039f5441d4f2f43757781ae0"),
            ("Bow_Pledge", "2b3f4a4eef528e1150893d58c68a5de3"),
            ("Bow_Ibis", "aef17fddf164e3ea98c4fd073a5f02a3"),
            ("Bow_Mechanic", "98e0ebb9a9857e2b21deda9d8c574b89"),
            ("Bow_Gurabad", "1cd241d13f320f3a0fbdfe378f423364"),
            ("Bow_Vorpal", "d70cf7fcc37cfbce1a07f5f3e214f278"),
            ("Catalyst_Iudex", "8f34b571d7e2d26e6c2ef2886defa86b"),
            ("Catalyst_Wheatley", "74712841e6a4cc25e0a2a36874ac71d7"),
            ("Catalyst_DandelionPoem", "b90f7edc1c4a6c0c132e00da99918d84"),
            ("Catalyst_Vorpal", "d4f997c8b399ef27d027653ba73645a5"),
            ("Catalyst_Yue", "93b6d05a8bc54d8eb69e4ed450c981c8"),
            ("Pole_Shanty", "2dc8e2cc9ab49c1d0ff9f67b3c98a94d"),
            ("Pole_Mechanic", "3333f73a195e3da1ed9a125658dfcf16"),
            ("Pole_Vorpal", "6514981aeab437590ebbfe15f5aef671"),
            ("Sword_Regalis", "1d603d4764cf292a12e268b8d8012688"),
            ("Sword_Vorpal", "acde24e9fa3f3e8c14f40aa4cd1a9007"),
            ("Sword_Machination", "b6259f0f93f824b169b50b6bc593dd08"),
            ("Sword_Purewill", "9c06477c167aadef25d03b4ceb1320ba"),
            ("Sword_Mechanic", "da80c69eada7e7fb6053b6301a6011ba"),
            ("Claymore_Mechanic", "6dc1e808543a09ddc1daf1e76721cd3d"),
            ("Claymore_Vorpal", "a76dd2c45f3bcfaf3f547d5c90bd20e2"),
            ("Claymore_BeastTamer", "4add4bfe23684d374398998eb5eb247d"),
            ("UI_RelicIcon_15032_4", "fadbbf8dbba05ad1ce0daeb4ebf89413"),
            ("UI_RelicIcon_15032_2", "4ca24e57d1adc9f0247c6bffd164d8b7"),
            ("UI_RelicIcon_15032_5", "9269d47e4a4edd517042c26fe534060e"),
            ("UI_RelicIcon_15032_1", "4d2b22a334d4237ba2cce56aeb5bd023"),
            ("UI_RelicIcon_15032_3", "2f69d44e2e79a05adbe0c9fe6391ea33"),
            ("UI_RelicIcon_15031_4", "9babba990b561f2b031d5db4145c19a9"),
            ("UI_RelicIcon_15031_2", "819b944729f2d5702f46d1403edba587"),
            ("UI_RelicIcon_15031_5", "de01dcbf2911968336afbbb61b455831"),
            ("UI_RelicIcon_15031_1", "b10902ae43e7f6d6619fe560829f7ba3"),
            ("UI_RelicIcon_15031_3", "1f3958293c20e8a29f51b9f3ed259e12"),
        ];

        data.into_iter().collect()
    };
}
