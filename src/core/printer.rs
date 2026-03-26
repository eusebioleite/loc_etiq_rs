use crate::core::state::Printers;
use wmi::WMIConnection;

pub fn get_printers() -> Result<Vec<Printers>, Box<dyn std::error::Error>> {
    let wmi_con = WMIConnection::new()?;
    let results: Vec<Printers> = wmi_con.raw_query("SELECT Name FROM Win32_Printer")?;
    let filtered: Vec<Printers> = results
        .into_iter()
        .filter(|p| {
            !p.name.contains("Microsoft") && !p.name.contains("PDF") && !p.name.contains("Fax")
        })
        .collect();
    Ok(filtered)
}
