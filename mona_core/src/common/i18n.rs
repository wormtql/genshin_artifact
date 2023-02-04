#[derive(Default, Clone, Debug, Hash, Eq, PartialEq)]
pub struct I18nLocale {
    pub zh_cn: &'static str,
    pub en: &'static str,
    pub ja: &'static str,
}

pub macro locale($($name:ident : $value:expr,)*) {
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
