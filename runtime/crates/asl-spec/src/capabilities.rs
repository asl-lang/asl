use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum FsCapabilitiesHelper {
    Bool(bool),
    List(Vec<String>),
    Struct(FsCapabilitiesStruct),
}

#[derive(Debug, Clone, Default, Deserialize)]
struct FsCapabilitiesStruct {
    #[serde(default, alias = "roots", alias = "read")]
    confined_read_roots: Vec<String>,
    #[serde(default, alias = "write", alias = "write_roots")]
    allow_write: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum NetCapabilitiesHelper {
    Bool(bool),
    List(Vec<String>),
    Struct(NetCapabilitiesStruct),
}

#[derive(Debug, Clone, Default, Deserialize)]
struct NetCapabilitiesStruct {
    #[serde(default, alias = "domains")]
    allow_domains: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum EnvCapabilitiesHelper {
    Bool(bool),
    List(Vec<String>),
    Struct(EnvCapabilitiesStruct),
}

#[derive(Debug, Clone, Default, Deserialize)]
struct EnvCapabilitiesStruct {
    #[serde(default, alias = "keys")]
    allow_keys: Vec<String>,
}

fn deserialize_fs<'de, D>(deserializer: D) -> std::result::Result<FsCapabilities, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let helper = Option::<FsCapabilitiesHelper>::deserialize(deserializer)?;
    match helper {
        Some(FsCapabilitiesHelper::Bool(true)) => Ok(FsCapabilities {
            confined_read_roots: vec![".".to_string()],
            allow_write: vec![".".to_string()],
        }),
        Some(FsCapabilitiesHelper::Bool(false)) => Ok(FsCapabilities::default()),
        Some(FsCapabilitiesHelper::List(roots)) => Ok(FsCapabilities {
            confined_read_roots: roots,
            allow_write: Vec::new(),
        }),
        Some(FsCapabilitiesHelper::Struct(s)) => Ok(FsCapabilities {
            confined_read_roots: s.confined_read_roots,
            allow_write: s.allow_write,
        }),
        None => Ok(FsCapabilities::default()),
    }
}

fn deserialize_net<'de, D>(deserializer: D) -> std::result::Result<NetCapabilities, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let helper = Option::<NetCapabilitiesHelper>::deserialize(deserializer)?;
    match helper {
        Some(NetCapabilitiesHelper::Bool(true)) => Ok(NetCapabilities {
            allow_domains: vec!["*".to_string()],
        }),
        Some(NetCapabilitiesHelper::Bool(false)) => Ok(NetCapabilities::default()),
        Some(NetCapabilitiesHelper::List(domains)) => Ok(NetCapabilities {
            allow_domains: domains,
        }),
        Some(NetCapabilitiesHelper::Struct(s)) => Ok(NetCapabilities {
            allow_domains: s.allow_domains,
        }),
        None => Ok(NetCapabilities::default()),
    }
}

fn deserialize_env<'de, D>(deserializer: D) -> std::result::Result<EnvCapabilities, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let helper = Option::<EnvCapabilitiesHelper>::deserialize(deserializer)?;
    match helper {
        Some(EnvCapabilitiesHelper::Bool(true)) => Ok(EnvCapabilities {
            allow_keys: vec!["*".to_string()],
        }),
        Some(EnvCapabilitiesHelper::Bool(false)) => Ok(EnvCapabilities::default()),
        Some(EnvCapabilitiesHelper::List(keys)) => Ok(EnvCapabilities {
            allow_keys: keys,
        }),
        Some(EnvCapabilitiesHelper::Struct(s)) => Ok(EnvCapabilities {
            allow_keys: s.allow_keys,
        }),
        None => Ok(EnvCapabilities::default()),
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillCapabilities {
    #[serde(default, deserialize_with = "deserialize_fs")]
    pub fs: FsCapabilities,
    #[serde(default, deserialize_with = "deserialize_net")]
    pub net: NetCapabilities,
    #[serde(default, deserialize_with = "deserialize_env")]
    pub env: EnvCapabilities,
    #[serde(default)]
    pub wasi_components: Vec<String>,
    /// Top-level convenience alias: `capabilities: domains: [...]`
    #[serde(default, alias = "allow_domains")]
    pub domains: Vec<String>,
}

impl SkillCapabilities {
    /// Normalizes top-level aliases into canonical capability fields
    pub fn normalize(&mut self) {
        if !self.domains.is_empty() {
            for d in &self.domains {
                if !self.net.allow_domains.contains(d) {
                    self.net.allow_domains.push(d.clone());
                }
            }
            self.domains.clear();
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FsCapabilities {
    #[serde(default, alias = "roots", alias = "read")]
    pub confined_read_roots: Vec<String>,
    #[serde(default, alias = "write", alias = "write_roots")]
    pub allow_write: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetCapabilities {
    #[serde(default, alias = "domains")]
    pub allow_domains: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnvCapabilities {
    #[serde(default, alias = "keys")]
    pub allow_keys: Vec<String>,
}
