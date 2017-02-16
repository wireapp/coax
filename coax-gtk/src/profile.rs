use std::fs::DirBuilder;
use std::path::PathBuf;

use coax_actor::config;
use coax_actor::error::Error;
use coax_data::profiles::{Profile, ProfileDb};
use slog::Logger;

pub fn open_profile_db(g: &Logger, cfg: &config::Main) -> Result<ProfileDb, Error> {
    let mut root = PathBuf::from(&cfg.data.root);
    if !root.exists() {
        DirBuilder::new().create(&root)?;
    }
    root.push("profiles.db");
    let ps = root.to_str().ok_or(Error::Message("/data/root contains invalid utf-8"))?;
    let db = ProfileDb::open(&g, ps)?;
    db.setup_schema()?;
    Ok(db)
}

pub fn load_profiles<'a>(db: &ProfileDb) -> Result<Vec<Profile<'a>>, Error> {
    db.select().map_err(From::from)
}

