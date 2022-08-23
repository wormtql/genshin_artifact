macro_rules! buff_meta_c {
    ($buff_name:ident $character_name:ident) => {
        #[cfg(not(target_family = "wasm"))]
        const META_DATA: BuffMetaData = BuffMetaData {
            name: BuffName::$buff_name,
            chs: "",
            image: BuffImage::Avatar(CharacterName::$character_name),
            genre: BuffGenre::Character,
            description: Some(""),
            from: BuffFrom::Character(CharacterName::$character_name)
        };
    }
}

pub(crate) use buff_meta_c;