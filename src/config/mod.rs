use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::OnceLock;

// =====================================================================
// STRUCT RAIZ DO ARQUIVO TOML
// =====================================================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RootConfig {
    #[serde(default)]
    pub locais: Vec<String>,
    pub zpl: String,
}

impl RootConfig {
    pub fn validate(&self) -> Result<(), String> {
        for (i, local) in self.locais.iter().enumerate() {
            if local.trim().is_empty() {
                return Err(format!("O item no índice {} em 'locais' não pode estar vazio.", i));
            }
        }
        if self.zpl.is_empty() {
            return Err("O item 'zpl' não pode estar vazio.".to_string());
        }
        Ok(())
    }
}

// =====================================================================
// GLOBAL SINGLETON
// =====================================================================

/// Process-wide config instance. Populated once by `init()` at startup.
static CONFIG: OnceLock<RootConfig> = OnceLock::new();

/// Returns a reference to the global config.
/// Panics if called before `init()`.
pub fn get() -> &'static RootConfig {
    CONFIG
        .get()
        .expect("config::init() deve ser chamado antes de config::get()")
}

// =====================================================================
// FUNÇÃO DE INICIALIZAÇÃO
// =====================================================================

pub fn init() -> anyhow::Result<()> {
    let path = std::path::Path::new("etiq.toml");

    if !path.is_file() {
        anyhow::bail!(
            "Arquivo de configuração não encontrado em '{}'. Crie o arquivo antes de iniciar.",
            path.display()
        );
    }

    let config_file = fs::read_to_string(path)
        .with_context(|| format!("Erro ao ler o arquivo de configuração em {}", path.display()))?;

    let root_config: RootConfig = toml::from_str(&config_file).with_context(|| {
        format!(
            "Erro de sintaxe no TOML ({})",
            path.display()
        )
    })?;

    if let Err(err_msg) = root_config.validate() {
        anyhow::bail!(
            "Configuração inválida no arquivo TOML ({}): {}",
            path.display(),
            err_msg
        );
    }

    // Armazena no singleton global. Falha apenas se chamado duas vezes.
    CONFIG
        .set(root_config)
        .map_err(|_| anyhow::anyhow!("config::init() foi chamado mais de uma vez"))?;

    Ok(())
}