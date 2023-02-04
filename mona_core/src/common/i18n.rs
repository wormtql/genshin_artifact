#[derive(Default, Clone, Debug, Hash, Eq, PartialEq)]
pub struct I18nLocale {
    pub zh_cn: &'static str,
    pub en: &'static str,
    pub ja: &'static str,
}

pub macro locale {
    ($($name:ident : $value:expr,)*) => {
        {
            let mut locale = I18nLocale {
                zh_cn: "",
                en: "",
                ja: "",
            };

            $(
                locale.$name = $value;
            )*

            locale
        }
    },
    ($($name:ident : $value:expr),*) => {
        {
            let mut locale = I18nLocale {
                zh_cn: "",
                en: "",
                ja: "",
            };

            $(
                locale.$name = $value;
            )*

            locale
        }
    }
}

pub macro hit_n_dmg {
    (1) => {
        locale!(zh_cn: "一段伤害", en: "1-Hit DMG")
    },
    (2) => {
        locale!(zh_cn: "二段伤害", en: "2-Hit DMG")
    },
    (3) => {
        locale!(zh_cn: "三段伤害", en: "3-Hit DMG")
    },
    (4) => {
        locale!(zh_cn: "四段伤害", en: "4-Hit DMG")
    },
    (5) => {
        locale!(zh_cn: "五段伤害", en: "5-Hit DMG")
    },
    (6) => {
        locale!(zh_cn: "六段伤害", en: "6-Hit DMG")
    },
    (1, $n:expr) => {
        locale!(zh_cn: format!("一段伤害-{}", $n), en: format!("1-Hit DMG-{}", $n))
    },
    (3, 1) => {
        locale!(zh_cn: "三段伤害-1", en: "3-Hit DMG-1")
    },
    (3, 2) => {
        locale!(zh_cn: "三段伤害-2", en: "3-Hit DMG-2")
    },
    (4, $n:expr) => {
        locale!(zh_cn: format!("四段伤害-{}", $n), en: format!("4-Hit DMG-{}", $n))
    }
}

pub macro plunging_dmg {
    (1) => {
        locale!(zh_cn: "下坠期间伤害", en: "Plunge DMG")
    },
    (2) => {
        locale!(zh_cn: "低空坠地冲击伤害", en: "Low Plunge DMG")
    },
    (3) => {
        locale!(zh_cn: "高空坠地冲击伤害", en: "High Plunge DMG")
    }
}

pub macro charged_dmg {
    () => {
        locale!(zh_cn: "重击伤害", en: "Charged Attack DMG")
    },
    ("shoot1") => {
        locale!(zh_cn: "瞄准射击", en: "Aimed Shot")
    },
    ("shoot2") => {
        locale!(zh_cn: "满蓄力瞄准射击", en: "Fully-Charged Aimed Shot")
    }
}
