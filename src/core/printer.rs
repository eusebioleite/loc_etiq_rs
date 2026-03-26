use serde::Deserialize;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Graphics::Printing::*;
use wmi::WMIConnection;
const PRN: &str = r#"CT~~CD,~CC^~CT~
^XA~TA000~JSN^LT0^MNW^MTT^PON^PMN^LH0,0^JMA^PR4,4~SD10^JUS^LRN^CI0^XZ
^XA
^MMT
^PW831
^LL0591
^LS0
^FO0,0^GFA,59904,59904,00104,:Z64:
eJzs1DFug0AQBdBFFFv6AlG4SGRfy4WlcDSOwhFcurBCEtg1HQ0apETvFwzFiCcxs5uSiOxKOwXmsTpdpDOtTigzjZVpYp17dfJX5PBPz9d47lt9e9O8BnQZIp302deXUCZ1w1Kb52bb7uQylvax3bc39fs5dA3W/5WHWCfdlnLqg53zUrpgpgKXcKefyznaKYO5RTtl0a7RTrs4wcf0xxl/n03wMa3CYc4Y7aTZaeOdeQPKNkTmeqQTfl2XmyD34c4Hh/MPnbeDnHcOh8PhcDgcDofD4XA4HA6Hw+FwOBwOh8PhcDgcDofD4XA4HA6Hw+FwOBwOh8PhcDgcDofD4XA4HA6Hw+FwOBwOh8PhcDgcDofD4XA4HA6Hw+FwOBwOh8PhcDgcDofD4fw15xsAAP//7dQxjsIwEAVQWSkoc4RcZAXXokCCo+UoHIFyC0RWJE7YigZ9S7t6v/C4iPJke2wOh8PhcDgcDofD4XA4HA6Hw+FwOBwOh8PhcDgcDofD4XA4HA6Hw+FwOBwOh8PhcDgcDofD4XA4HA6Hw+FwOBwOh8PhcDgcDofD4XA4nD/h7Bs5XxxOc2eMO6fn0OWdY1PnGne+n0PJO7fZuaWZ0tRZdi+Z2gHHuDPO5ZR26kuwTzv9ZS6HtDMszhB3llKXlUs9mPiDXRttF76o5b7ULnxR1/+vXirbfp2zzjDWyWF889XnOV9WMNoIZVpnu0fS6bfjL1OwE/rptVtTNNfNGaLOa21dkvn9CpQ2jMg/yw9i3ngF:D49E
^BY4,3,241^FT757,244^BCI,,N,N
^FD>:[LOCAL_ESTOQUE]^FS
^FT757,119^A0I,99,98^FH\^FD[LOCAL_ESTOQUE]^FS
^PQ1,0,1,Y^XZ"#;

pub fn print(printer_name: &str, location: &str) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let mut h_printer: HANDLE = HANDLE::default();

        // Dados da constante convertidos para bytes para a Win32 API
        let prn_file = PRN.replace("[LOCAL_ESTOQUE]", location);
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
