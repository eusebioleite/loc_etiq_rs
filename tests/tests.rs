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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32Printer {
    name: String,
}
/*
// Função principal que você vai chamar quando o usuário escolher a impressora
pub fn print_raw_to_printer(
    printer_name: &str,
    raw_data: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let mut h_printer: HANDLE = HANDLE::default();

        // Converter nome da impressora para wide string
        let printer_wide: Vec<u16> = printer_name
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let printer_pcwstr = PCWSTR(printer_wide.as_ptr());

        // 1. Abre a impressora
        if !OpenPrinterW(printer_pcwstr, &mut h_printer, None).as_bool() {
            return Err(format!("Falha ao abrir impressora: {}", printer_name).into());
        }

        // 2. Informações do documento (tipo RAW é obrigatório para ZPL)
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

        // 3. Inicia o job de impressão
        let job_id = StartDocPrinterW(h_printer, 1, &mut doc_info as *mut _ as *mut _);
        if job_id == 0 {
            ClosePrinter(h_printer);
            return Err("Falha ao iniciar o documento de impressão".into());
        }

        // 4. Inicia página
        if !StartPagePrinter(h_printer).as_bool() {
            EndDocPrinter(h_printer);
            ClosePrinter(h_printer);
            return Err("Falha ao iniciar página".into());
        }

        // 5. Envia os dados RAW (ZPL)
        let mut bytes_written: u32 = 0;
        let write_result = WritePrinter(
            h_printer,
            raw_data.as_ptr() as *const _,
            raw_data.len() as u32,
            &mut bytes_written,
        );

        if !write_result.as_bool() || bytes_written != (raw_data.len() as u32) {
            EndPagePrinter(h_printer);
            EndDocPrinter(h_printer);
            ClosePrinter(h_printer);
            return Err(format!(
                "Erro ao escrever na impressora. Enviados: {}/{}",
                bytes_written,
                raw_data.len()
            )
            .into());
        }

        // 6. Finaliza
        let _ = EndPagePrinter(h_printer);
        let _ = EndDocPrinter(h_printer);
        ClosePrinter(h_printer);

        println!("✅ Impressão enviada com sucesso! Job ID: {}", job_id);
        Ok(())
    }
}
*/
fn get_system_printers() -> Result<Vec<Win32Printer>, Box<dyn std::error::Error>> {
    let wmi_con = WMIConnection::new()?;
    let results: Vec<Win32Printer> = wmi_con.raw_query("SELECT Name FROM Win32_Printer")?;
    let filtered: Vec<Win32Printer> = results
        .into_iter()
        .filter(|p| {
            !p.name.contains("Microsoft") && !p.name.contains("PDF") && !p.name.contains("Fax")
        })
        .collect();
    Ok(filtered)
}

#[cfg(test)]
mod tests {
    #[test]
    fn testar() {
        //
    }
}
