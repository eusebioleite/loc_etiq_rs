# loc_etiq_rs

**Programa de geração de etiquetas de locais de estoque**
Aplicativo desktop simples e leve feito em **Rust** para imprimir etiquetas de localização de estoque com código de barras.

<img width="519" height="579" alt="image" src="https://github.com/user-attachments/assets/58206876-19ca-40aa-b4af-74d5b53cf36d" />


## ✨ Funcionalidades

- Lista completa de locais de estoque (carregados de forma hardcoded)
- Busca em tempo real (filtro)
- Seleção múltipla com checkboxes
- Seleção de impressora térmica (detecta automaticamente via WMI)
- Controle de quantidade de cópias
- Geração de etiquetas em **ZPL** (linguagem Zebra)
- Suporte a código de barras nas etiquetas
- Interface limpa e moderna com tema Catppuccin Latte
- Janela sem bordas do Windows (design customizado)

## 🛠 Tecnologias utilizadas

| Tecnologia       | Versão    | Uso |
|------------------|-----------|-----|
| **Rust**         | Edition 2024 | Linguagem principal |
| **Iced**         | 0.14      | Interface gráfica (GUI) |
| **windows**      | 0.58      | Impressão direta no Windows (Win32 Printing) |
| **wmi**          | 0.18      | Listagem automática de impressoras |

## 📋 Como usar

1. Abra o programa `loc_etiq_rs.exe`
2. Use a barra **Pesquisar** para filtrar os locais desejados
3. Marque os checkboxes dos locais que deseja imprimir
4. Selecione a **Impressora** no dropdown (apenas impressoras térmicas ZPL são recomendadas)
5. Ajuste o número de **Cópias** com o controle deslizante
6. Clique no botão azul **Gerar Etiquetas**

Pronto! As etiquetas serão enviadas diretamente para a impressora selecionada.

## 📍 Locais de estoque disponíveis

Todos os locais estão definidos no código em `src/core/state.rs`.
**Para adicionar ou remover locais**, edite a constante `STRING_LOCAIS_ESTOQUE` e recompile o projeto.

## 🖨 Formato da etiqueta

- Linguagem: **ZPL** (Zebra Programming Language)
- Tamanho configurado: 831 × 591 dots
- Contém:
  - Código do local em texto grande
  - Código de barras
  - QR Code
  - Layout otimizado para impressoras térmicas de etiqueta

## 🚀 Como compilar / desenvolver

### Pré-requisitos
- Rust (versão recente com edition 2024)
- Windows 10/11 (o programa usa APIs nativas do Windows)

### Comandos

```bash
# Clonar o repositório
git clone https://github.com/eusebioleite/loc_etiq_rs.git
cd loc_etiq_rs

# Rodar em modo desenvolvimento
cargo run

# Compilar versão release (recomendado)
cargo build --release

# Executável final estará em:
target/release/loc_etiq_rs.exe
```

## 📁 Estrutura do projeto
```
loc_etiq_rs/
├── src/
│   ├── core/           # Lógica de negócio
│   │   ├── state.rs    # Estado da aplicação + lista de locais
│   │   ├── printer.rs  # Impressão ZPL + WMI
│   │   ├── message.rs
│   │   └── mod.rs
│   ├── ui/             # Componentes visuais
│   ├── update.rs       # Lógica de atualização da UI
│   └── main.rs         # Entrada principal
├── tests/              # Testes
├── Cargo.toml
├── LICENSE (GPL-3.0)
└── README.md
```
