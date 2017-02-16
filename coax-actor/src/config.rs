use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use toml;

use app_dirs::{self, AppInfo, AppDataType};

const APP_INFO: AppInfo = AppInfo {
    name:   "coax",
    author: "wire"
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Main {
    pub host: Host,
    pub data: Data
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Host {
    pub url:       String,
    pub websocket: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub root: String
}

impl Main {
    pub fn load_default() -> Result<Main, Box<Error>> {
        let mut path = app_dirs::app_root(AppDataType::UserConfig, &APP_INFO)?;
        path.push("coax.toml");
        if path.exists() {
            Main::load(&path)
        } else {
            let c = Main::default()?;
            let t = toml::to_string(&c)?;
            let mut file = File::create(&path)?;
            file.write_all(t.as_bytes())?;
            Ok(c)
        }
    }

    pub fn load<P: AsRef<Path>>(p: P) -> Result<Main, Box<Error>> {
        let mut file = File::open(p.as_ref())?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        toml::from_str(&buffer).map_err(From::from)
    }

    #[cfg(not(feature = "prod"))]
    pub fn default() -> Result<Main, Box<Error>> {
        let data_root = app_dirs::app_dir(AppDataType::UserData, &APP_INFO, "staging")?;
        let data_root_str = data_root.to_str()
            .ok_or(format!("invalid utf-8 in path: {:?}", data_root))?
            .into();
        Ok(Main {
            host: Host {
                url:       "https://staging-nginz-https.zinfra.io".into(),
                websocket: "https://staging-nginz-ssl.zinfra.io/await".into()
            },
            data: Data { root: data_root_str }
        })
    }

    #[cfg(feature = "prod")]
    pub fn default() -> Result<Main, Box<Error>> {
        let data_root = app_dirs::app_dir(AppDataType::UserData, &APP_INFO, "prod")?;
        let data_root_str = data_root.to_str()
            .ok_or(format!("invalid utf-8 in path: {:?}", data_root))?
            .into();
        Ok(Main {
            host: Host {
                url:       "https://prod-nginz-https.wire.com".into(),
                websocket: "https://prod-nginz-ssl.wire.com/await".into()
            },
            data: Data { root: data_root_str }
        })
    }
}
