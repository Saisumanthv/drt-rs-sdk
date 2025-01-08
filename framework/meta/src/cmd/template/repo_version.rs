use crate::{version::FrameworkVersion, version_history::LAST_TEMPLATE_VERSION};

pub enum RepoVersion {
    Master,
    Tag(String),
}

impl RepoVersion {
    pub fn url(&self) -> String {
        match self {
            RepoVersion::Master => {
                "https://github.com/dharitri/drt-sdk-rs/archive/refs/heads/master.zip".to_string()
            },
            RepoVersion::Tag(tag) => {
                format!("https://github.com/dharitri/drt-sdk-rs/archive/refs/tags/v{tag}.zip")
            },
        }
    }

    pub fn temp_dir_name(&self) -> String {
        match self {
            RepoVersion::Master => "drt-sdk-rs-master".to_string(),
            RepoVersion::Tag(tag) => {
                format!("drt-sdk-rs-{tag}")
            },
        }
    }

    pub fn get_tag(&self) -> FrameworkVersion {
        match self {
            RepoVersion::Master => LAST_TEMPLATE_VERSION,
            RepoVersion::Tag(tag) => FrameworkVersion::from_string_template(tag),
        }
    }
}
