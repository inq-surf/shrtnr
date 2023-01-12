use config::{Config, ConfigError};
use harsh::Harsh;

const SHRTNR_CONFIG_PREFIX: &str = "SHRTNR";
const HARSH_CONFIG_PREFIX: &str = "HARSH";

pub trait LoadConfig {
    fn load<T>() -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned;
}

pub struct Globals {
    pub harsh: Harsh,
    pub harsh_config: HarshConfig,
    pub shrtnr_config: ShrtnrConfig,
}

impl Globals {
    pub fn new() -> Self {
        let harsh_config = HarshConfig::load()
            .expect("harsh config failure");

        let shrtnr_config = ShrtnrConfig::load()
            .expect("shrtnr config failure");

        let harsh = get_harsh(&harsh_config);

        Globals {
            harsh,
            harsh_config,
            shrtnr_config,
        }
    }
}

#[derive(Debug, Default, serde_derive::Deserialize, PartialEq, Eq)]
pub struct HarshConfig {
    pub salt: Option<String>,
    pub length: Option<usize>,
    pub alphabet: Option<String>,
}

impl LoadConfig for HarshConfig {
    fn load<T>() -> Result<T, ConfigError>
        where
            T: serde::de::DeserializeOwned {
        get_config(HARSH_CONFIG_PREFIX)
    }
}

#[derive(Debug, Default, serde_derive::Deserialize, PartialEq, Eq)]
pub struct ShrtnrConfig {
    pub host: String,
}

impl LoadConfig for ShrtnrConfig {
    fn load<T>() -> Result<T, ConfigError>
        where
            T: serde::de::DeserializeOwned {
        get_config(SHRTNR_CONFIG_PREFIX)
    }
}

fn get_config<T>(prefix: &str) -> Result<T, ConfigError>
where
    T: serde::de::DeserializeOwned,
{
    let config = Config::builder()
        .add_source(
            config::Environment::with_prefix(prefix)
                .try_parsing(true)
                .separator("__")
                .ignore_empty(true)
        )
        .build();

    let config = config?;

    match config.try_deserialize() {
        Ok(config) => Result::Ok(config),
        Err(e) => {
            return Result::Err(e);
        },
    }
}

fn get_harsh(config: &HarshConfig) -> Harsh {
    let mut builder = Harsh::builder();

    if let Some(salt) = &config.salt {
        builder = builder.salt(salt.clone());
    }

    if let Some(length) = &config.length {
        builder = builder.length(length.clone());
    }

    if let Some(alphabet) = &config.alphabet {
        builder = builder.alphabet(alphabet.clone());
    }

    match builder.build() {
        Ok(harsh) => harsh,
        Err(e) => {
            println!("Error: {}", e);
            Harsh::default()
        },
    }
}
