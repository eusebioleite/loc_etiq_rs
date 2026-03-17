use serde::Deserialize;
use wmi::{WMIConnection};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32Printer {
    name: String,
    port_name: String,
    printer_status: u32,
}

fn get_system_printers() -> Result<Vec<Win32Printer>, Box<dyn std::error::Error>> {
    //let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new()?;

    let results: Vec<Win32Printer> = wmi_con.raw_query("SELECT Name, PortName, PrinterStatus FROM Win32_Printer")?;
    
    // Filtragem de lixo (PDF, OneNote, etc)
    let filtered: Vec<Win32Printer> = results.into_iter()
        .filter(|p| 
            !p.name.contains("Microsoft") && 
            !p.name.contains("PDF") && 
            !p.name.contains("Fax") &&
            !p.name.contains("OneNote"))
        .collect();

    Ok(filtered)
}

fn main() {
    let printers: Vec<Win32Printer> = match get_system_printers() {
        Ok(printers) => printers,
        Err(e) => {
            eprintln!("Error: {}", e);
            let empty_printer: Win32Printer = Win32Printer {
                name: "Empty".to_string(),
                port_name: "Empty".to_string(),
                printer_status: 0
            };
            let mut no_printers: Vec<Win32Printer> = Vec::new();
            no_printers.push(empty_printer);
            no_printers
        }
    };

    for printer in printers {
        println!("Printer: {}", printer.name);
        println!("Porta: {}", printer.port_name);
        println!("Status: {}", printer.printer_status);
    }
}
