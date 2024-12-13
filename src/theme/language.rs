use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
    utils::{ConditionalSendFuture, HashMap},
};
use std::future::Future;

pub(super) fn plugin(app: &mut App) {
    app.register_asset_loader(TranslationsAssetLoader)
        .init_asset::<Translation>()
        .add_systems(Update, update_text);
    app.insert_resource(Localize::from_asset_path("configs/duolingo.csv"));
}

/// You can use this resource in two ways:
/// 1. Load from file
///    This way makes sure that the resource will be completely initialized and ready to translate.
/// 2. Insert it empty, then load from asset handle
///    Using it this way will result in a slight delay until it gets initialized.
#[derive(Resource)]
pub struct Localize {
    is_initialized: bool,
    set_language_after_init: Option<String>,
    current_language_id: usize,
    languages: HashMap<String, usize>,
    words: HashMap<String, Vec<String>>,
    asset_handle_path: Option<String>,
    asset_handle: Option<Handle<Translation>>,
}
impl Localize {
    /// Initializes an empty resource
    pub fn empty() -> Self {
        Self {
            is_initialized: false,
            set_language_after_init: None,
            current_language_id: 0,
            languages: HashMap::new(),
            words: HashMap::new(),
            asset_handle_path: None,
            asset_handle: None,
        }
    }
    /// Creates a new resource from specified data (in a .csv format)
    pub fn from_data(translations: &str) -> Self {
        let mut localize = Self::empty();
        localize.set_data(translations);
        localize
    }
    /// Creates a new resource from specified asset path.
    pub fn from_asset_path(path: &str) -> Self {
        let mut localize = Self::empty();
        localize.asset_handle_path = Some(path.to_string());
        localize
    }
    /// Creates a new resource from `self` with a given default language.
    pub fn with_default_language(mut self, language: impl ToString) -> Self {
        self.set_language(language);
        self
    }
    /// Sets data for the resource
    pub fn set_data(&mut self, translations: &str) {
        let mut languages = HashMap::new();
        let mut words = HashMap::new();

        let mut data = csv::Reader::from_reader(translations.as_bytes());
        let mut records: Vec<Vec<_>> = Vec::new();
        if let Ok(headers) = data.headers() {
            records.push(headers.iter().map(|field| field.to_string()).collect());
        }
        for result in data.records().flatten() {
            records.push(result.iter().map(|field| field.to_string()).collect());
        }
        for (language_id, language) in records[0][2..].iter().enumerate() {
            languages.insert(language.to_string(), language_id);
        }
        for record in &records[1..] {
            let keyword = &record[0];
            let translations = record[2..].iter().map(|x| x.to_string()).collect();
            words.insert(keyword.to_string(), translations);
        }
        self.languages = languages;
        self.words = words;
        self.initialized();
    }
    /// Get a translation for a specified keyword.
    /// If there is no translation for the keyword, it will return an empty string.
    pub fn get(&self, keyword: &str) -> &str {
        match self.words.get(keyword) {
            Some(k) => {
                if self.current_language_id < k.len() {
                    &k[self.current_language_id]
                } else {
                    ""
                }
            }
            None => "",
        }
    }
    /// Sets the language for the resource.
    pub fn set_language(&mut self, language: impl ToString) {
        let language = language.to_string();
        if self.is_initialized {
            if let Some(language_id) = self.languages.get(&language) {
                self.current_language_id = *language_id;
            } else {
                error!("Language not found! ({})", language);
            }
        } else {
            self.set_language_after_init = Some(language);
        }
    }
    fn initialized(&mut self) {
        self.is_initialized = true;
        if let Some(language) = self.set_language_after_init.clone() {
            self.set_language(language);
        }
    }
}
/// Translates text.
/// Use it with the `Text` component.
#[derive(Component)]
pub struct LocalizeText {
    sections: Vec<String>,
    translated_language: Option<usize>,
}
impl LocalizeText {
    /// The first section of the text will be translated using the specified keyword
    pub fn from_section(keyword: impl Into<String>) -> Self {
        Self {
            sections: vec![keyword.into()],
            translated_language: None,
        }
    }
    /// All sections of the text will be translated using the specified keywords
    pub fn from_sections(keywords: impl IntoIterator<Item = String>) -> Self {
        Self {
            sections: keywords.into_iter().collect(),
            translated_language: None,
        }
    }
}

fn update_text(
    localize: Option<ResMut<Localize>>,
    translation_assets: ResMut<Assets<Translation>>,
    mut ev_asset: EventReader<AssetEvent<Translation>>,
    asset_server: Res<AssetServer>,
    mut text: Query<(Entity, &mut LocalizeText)>,
    mut writer: TextUiWriter,
) {
    if let Some(mut localize) = localize {
        if let Some(asset_handle_path) = localize.asset_handle_path.clone() {
            localize.asset_handle_path = None;
            localize.asset_handle = Some(asset_server.load(asset_handle_path));
        }
        if let Some(asset_handle) = localize.asset_handle.clone() {
            for ev in ev_asset.read() {
                match ev {
                    AssetEvent::Added { id } | AssetEvent::Modified { id } => {
                        if id == &asset_handle.id() {
                            let translation = translation_assets.get(&asset_handle).unwrap();
                            localize.set_data(&translation.0);
                        }
                    }
                    _ => {}
                }
            }
        }
        if localize.is_initialized {
            // info!("is_initialized!");
            for (entity, mut localize_text) in &mut text {
                if localize_text.translated_language.is_none()
                    || localize_text.translated_language.unwrap_or(0)
                        != localize.current_language_id
                {
                    localize_text.translated_language = Some(localize.current_language_id);
                    for (id, keyword) in localize_text.sections.iter().enumerate() {
                        // info!("keyword:{},id:{}", keyword,id);
                        if keyword != "" {
                            *writer.text(entity, id) = localize.get(keyword).to_string();
                        }
                    }
                }
            }
        }
    }
}

#[derive(Asset, TypePath, Debug)]
pub struct Translation(pub String);
#[derive(Default)]
struct TranslationsAssetLoader;
impl AssetLoader for TranslationsAssetLoader {
    type Asset = Translation;
    type Settings = ();
    type Error = std::io::Error;
    fn load(
        &self,
        reader: &mut dyn Reader,
        _: &Self::Settings,
        _: &mut LoadContext,
    ) -> impl ConditionalSendFuture + Future<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes: Vec<u8> = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let translation_asset = Translation(std::str::from_utf8(&bytes).unwrap().to_string());
            Ok(translation_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["csv"]
    }
}
