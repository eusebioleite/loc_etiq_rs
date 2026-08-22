use serde::Deserialize;
use windows::core::PCWSTR;
use windows::Win32::Graphics::Printing::*;
use wmi::WMIConnection;

pub fn print(printer_name: &str, location: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = crate::config::reload() {
        return Err(format!("Erro ao recarregar config: {}", e).into());
    }

    unsafe {
        let mut h_printer = PRINTER_HANDLE::default();
        
        let prn = crate::config::get().zpl;
        // Dados da constante convertidos para bytes para a Win32 API
        let prn_file = prn.replace("[LOCAL_ESTOQUE]", location);
        println!("{}", prn_file);
        let raw_data = prn_file.as_bytes();
        let data_len = raw_data.len() as u32;

        // Converter nome da impressora para wide string (UTF-16)
        let printer_wide: Vec<u16> = printer_name
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let printer_pcwstr = PCWSTR(printer_wide.as_ptr());

        // 1. Abre a impressora
        OpenPrinterW(printer_pcwstr, &mut h_printer, None)?;

        // 2. Informações do documento
        let doc_name: Vec<u16> = "Etiqueta_ZPL"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let data_type: Vec<u16> = "RAW".encode_utf16().chain(std::iter::once(0)).collect();

        let mut doc_info = DOC_INFO_1W {
            pDocName: windows::core::PWSTR(doc_name.as_ptr() as *mut u16),
            pOutputFile: windows::core::PWSTR::null(),
            pDatatype: windows::core::PWSTR(data_type.as_ptr() as *mut u16),
        };

        // 3. Inicia o job
        let job_id = StartDocPrinterW(h_printer, 1, &mut doc_info as *mut _ as *mut _);
        if job_id == 0 {
            let _ = ClosePrinter(h_printer);
            return Err("Falha ao iniciar o documento".into());
        }

        // 4. Inicia página
        let _ = StartPagePrinter(h_printer);

        // 5. Envia os dados da constante PRN
        let mut bytes_written: u32 = 0;
        let write_result = WritePrinter(
            h_printer,
            raw_data.as_ptr() as *const _,
            data_len,
            &mut bytes_written,
        );

        if !write_result.as_bool() || bytes_written != data_len {
            let _ = EndPagePrinter(h_printer);
            let _ = EndDocPrinter(h_printer);
            let _ = ClosePrinter(h_printer);
            return Err(format!(
                "Erro Win32 ao escrever. Enviados: {}/{}",
                bytes_written, data_len
            )
            .into());
        }

        // 6. Cleanup
        let _ = EndPagePrinter(h_printer);
        let _ = EndDocPrinter(h_printer);
        let _ = ClosePrinter(h_printer);

        Ok(())
    }
}
pub fn get_printers() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    #[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
    #[serde(rename_all = "PascalCase")]
    struct Printers {
        name: String,
    }

    let wmi_con = WMIConnection::new()?;
    let results: Vec<Printers> = wmi_con.raw_query("SELECT Name FROM Win32_Printer")?;

    let filtered: Vec<String> = results
        .into_iter()
        .map(|p| p.name) // Extraímos a String
        .filter(|name| {
            !name.contains("Microsoft")
                && !name.contains("PDF")
                && !name.contains("Fax")
                && !name.contains("OneNote")
                && !name.contains("AnyDesk")
                && !name.contains("RustDesk")
                && !name.contains("CutePDF")
        })
        .collect();

    Ok(filtered)
}
