use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default)]
    pub locais: Vec<String>,
    pub zpl: String,
}

const DEFAULT_CONFIG: &str = r#"# Lista de locais de estoque disponíveis
# Exemplo: locais = ["PRAT-01", "PRAT-02", "PALLET-A"]
locais = []

# Template ZPL para impressão das etiquetas.
# A tag [LOCAL_ESTOQUE] será substituída pelo nome do local selecionado.
# Exemplo:
# zpl = '''
# ^XA
# ^FO50,50^ADN,36,20^FD[LOCAL_ESTOQUE]^FS
# ^XZ
# '''
zpl = '''
'''
"#;

impl Config {
    pub fn validate(&self) -> Result<(), String> {
        for (i, local) in self.locais.iter().enumerate() {
            if local.trim().is_empty() {
                return Err(format!(
                    "O item no índice {} em 'locais' não pode estar vazio.",
                    i
                ));
            }
        }
        Ok(())
    }
}

static CONFIG: RwLock<Option<Config>> = RwLock::new(None);

pub fn get() -> Config {
    CONFIG
        .read()
        .unwrap()
        .as_ref()
        .expect("config::init() deve ser chamado antes de config::get()")
        .clone()
}

pub fn reload() -> Result<(), String> {
    let path = std::path::Path::new("etiq.toml");

    let config_file = fs::read_to_string(path)
        .map_err(|e| format!("Erro ao ler arquivo de configuração: {}", e))?;

    let root_config: Config = toml::from_str(&config_file)
        .map_err(|e| format!("Erro ao processar arquivo de configuração: {}", e))?;

    root_config.validate()?;

    *CONFIG.write().unwrap() = Some(root_config);

    Ok(())
}

pub fn add_location(new_loc: String) -> Result<(), String> {
    let mut config = get();
    if !config.locais.contains(&new_loc) {
        config.locais.push(new_loc);
        let config_str = toml::to_string(&config)
            .map_err(|e| format!("Erro ao serializar config: {}", e))?;
        std::fs::write("etiq.toml", config_str)
            .map_err(|e| format!("Erro ao salvar arquivo de configuração: {}", e))?;
        *CONFIG.write().unwrap() = Some(config);
    }
    Ok(())
}

pub fn remove_location(loc_to_remove: &str) -> Result<(), String> {
    let mut config = get();
    if let Some(pos) = config.locais.iter().position(|l| l == loc_to_remove) {
        config.locais.remove(pos);
        let config_str = toml::to_string(&config)
            .map_err(|e| format!("Erro ao serializar config: {}", e))?;
        std::fs::write("etiq.toml", config_str)
            .map_err(|e| format!("Erro ao salvar arquivo de configuração: {}", e))?;
        *CONFIG.write().unwrap() = Some(config);
    }
    Ok(())
}

pub fn init() -> Result<(), String> {
    let path = std::path::Path::new("etiq.toml");

    match fs::metadata(path) {
        Ok(meta) if meta.is_file() => (),
        _ => {
            fs::write(path, DEFAULT_CONFIG)
                .map_err(|e| format!("Erro ao criar arquivo etiq.toml: {}", e))?;
        }
    }

    reload()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_parsing() {
        let config: Config = toml::from_str(DEFAULT_CONFIG).expect("DEFAULT_CONFIG must be valid TOML");
        assert!(config.validate().is_ok());
        assert_eq!(config.locais.len(), 0);
    }

    #[test]
    fn test_validation_empty_location() {
        let config = Config {
            locais: vec!["PRAT-01".to_string(), "".to_string()],
            zpl: "test".to_string(),
        };
        assert!(config.validate().is_err());
    }
}
