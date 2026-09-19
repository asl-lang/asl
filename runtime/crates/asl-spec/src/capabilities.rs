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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostSecurityPolicy {
    #[serde(default)]
    pub allowed_fs_read_roots: Vec<String>,
    #[serde(default)]
    pub allowed_fs_write_roots: Vec<String>,
    #[serde(default)]
    pub allowed_domains: Vec<String>,
    #[serde(default)]
    pub allowed_env_keys: Vec<String>,
    #[serde(default = "default_policy_fuel")]
    pub max_fuel_opcodes: u64,
    #[serde(default = "default_policy_timeout")]
    pub max_timeout_ms: u64,
}

fn default_policy_fuel() -> u64 {
    1_000_000
}

fn default_policy_timeout() -> u64 {
    15_000
}

impl Default for HostSecurityPolicy {
    fn default() -> Self {
        Self {
            allowed_fs_read_roots: Vec::new(),
            allowed_fs_write_roots: Vec::new(),
            allowed_domains: Vec::new(),
            allowed_env_keys: Vec::new(),
            max_fuel_opcodes: default_policy_fuel(),
            max_timeout_ms: default_policy_timeout(),
        }
    }
}

impl HostSecurityPolicy {
    pub fn permissive() -> Self {
        Self {
            allowed_fs_read_roots: vec!["*".to_string()],
            allowed_fs_write_roots: vec!["*".to_string()],
            allowed_domains: vec!["*".to_string()],
            allowed_env_keys: vec!["*".to_string()],
            max_fuel_opcodes: 10_000_000,
            max_timeout_ms: 30_000,
        }
    }

    pub fn intersect(&self, requested: &SkillCapabilities) -> std::result::Result<SkillCapabilities, crate::AslError> {
        let mut effective = SkillCapabilities::default();

        // 1. FS Read
        for root in &requested.fs.confined_read_roots {
            let is_allowed = self.allowed_fs_read_roots.iter().any(|a| {
                a == "*" || root == a || root.starts_with(a)
            });
            if is_allowed {
                effective.fs.confined_read_roots.push(root.clone());
            } else {
                return Err(crate::AslError::CapabilityViolation(format!(
                    "Host policy denied FS read root: '{}' (authorized roots: {:?})",
                    root, self.allowed_fs_read_roots
                )));
            }
        }

        // 2. FS Write
        for root in &requested.fs.allow_write {
            let is_allowed = self.allowed_fs_write_roots.iter().any(|a| {
                a == "*" || root == a || root.starts_with(a)
            });
            if is_allowed {
                effective.fs.allow_write.push(root.clone());
            } else {
                return Err(crate::AslError::CapabilityViolation(format!(
                    "Host policy denied FS write root: '{}' (authorized write roots: {:?})",
                    root, self.allowed_fs_write_roots
                )));
            }
        }

        // 3. Net domains
        for domain in &requested.net.allow_domains {
            let d_clean = domain.trim().trim_end_matches('.').to_lowercase();
            let is_allowed = self.allowed_domains.iter().any(|allowed| {
                let a = allowed.trim().trim_end_matches('.').to_lowercase();
                a == "*" || d_clean == a || d_clean.ends_with(&format!(".{}", a))
            });
            if is_allowed {
                effective.net.allow_domains.push(domain.clone());
            } else {
                return Err(crate::AslError::CapabilityViolation(format!(
                    "Host policy denied network domain: '{}' (authorized domains: {:?})",
                    domain, self.allowed_domains
                )));
            }
        }

        // 4. Env keys
        for key in &requested.env.allow_keys {
            let is_allowed = self.allowed_env_keys.iter().any(|k| k == "*" || k == key);
            if is_allowed {
                effective.env.allow_keys.push(key.clone());
            } else {
                return Err(crate::AslError::CapabilityViolation(format!(
                    "Host policy denied environment key: '{}' (authorized keys: {:?})",
                    key, self.allowed_env_keys
                )));
            }
        }

        effective.wasi_components = requested.wasi_components.clone();
        Ok(effective)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_host_security_policy_intersection() {
        let policy = HostSecurityPolicy {
            allowed_fs_read_roots: vec!["/safe/dir".to_string()],
            allowed_domains: vec!["api.github.com".to_string()],
            allowed_env_keys: vec!["API_KEY".to_string()],
            ..Default::default()
        };

        let mut requested = SkillCapabilities::default();
        requested.fs.confined_read_roots = vec!["/safe/dir/file.txt".to_string()];
        requested.net.allow_domains = vec!["api.github.com".to_string()];
        requested.env.allow_keys = vec!["API_KEY".to_string()];

        let effective = policy.intersect(&requested).expect("Should succeed");
        assert_eq!(effective.fs.confined_read_roots, vec!["/safe/dir/file.txt"]);
        assert_eq!(effective.net.allow_domains, vec!["api.github.com"]);
        assert_eq!(effective.env.allow_keys, vec!["API_KEY"]);

        // Denied domain
        requested.net.allow_domains = vec!["evil.com".to_string()];
        let denied = policy.intersect(&requested);
        assert!(denied.is_err());

        // Denied read root
        requested.net.allow_domains = vec!["api.github.com".to_string()];
        requested.fs.confined_read_roots = vec!["/etc/passwd".to_string()];
        let denied_fs = policy.intersect(&requested);
        assert!(denied_fs.is_err());
    }
}
