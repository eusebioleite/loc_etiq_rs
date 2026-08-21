use std::env;
use std::io;

fn main() -> io::Result<()> {
    // Apenas tenta embutir o ícone se o alvo da compilação for o Windows.
    // Isso evita erros se você tentar compilar o mesmo código no Linux ou macOS.
    if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/logo.ico"); // O nome do arquivo salvo no Passo 1
        res.compile()?;
    }
    Ok(())
}