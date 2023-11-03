pub enum Language {
    English,
    Russian,
    German,
}

#[derive(Default)]
pub struct Lang {
    lang: Language,
}

impl Default for Lang {
    fn default() -> Self {
        Lang { lang: Language::English }
    }
}

pub impl Lang {
    fn setLang(&self, curent: Language) {
        self.lang = current
    }

    fn getLang() {
        self.lang
    }
}
