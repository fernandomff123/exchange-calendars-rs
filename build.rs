
// build.rs (Na raiz do projeto)
use std::fs;
use std::path::Path;

fn main() {
    let data_dir = Path::new("src/data");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_markets.rs");

    // Captura o caminho absoluto real da raiz do seu projeto no seu computador
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir_clean = manifest_dir.replace("\\", "/"); // Normaliza caminhos no Windows/Linux

    let mut match_arms = String::new();

    if data_dir.exists() {
        for entry in fs::read_dir(data_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "bin") {
                let file_name = path.file_stem().unwrap().to_str().unwrap().to_uppercase();

                // Mapeamento especial para os mercados com nomes sanitizados pelo Python
                let user_mic = if file_name == "24_5" {
                    "24/5".to_string()
                } else if file_name == "24_7" {
                    "24/7".to_string()
                } else {
                    file_name.clone()
                };

                // SOLUÇÃO: Usa o macro concat! + env! para apontar de forma absoluta para src/data/ficheiro.bin
                match_arms.push_str(&format!(
                    "        \"{}\" => Ok(Box::new(GenericCalendar::new(\"{}\", include_bytes!(concat!(\"{}\", \"/src/data/{}.bin\"))))),\n",
                                             user_mic, user_mic, manifest_dir_clean, file_name.to_lowercase()
                ));
            }
        }
    }

    let generated_code = format!(
        "pub fn get_calendar(mic: &str) -> Result<Box<dyn ExchangeCalendar>, CalendarError> {{\n\
match mic.to_uppercase().as_str() {{\n\
{}\
_ => Err(CalendarError::MarketNotFound),\n\
}}\n\
}}",
match_arms
    );

    fs::write(&dest_path, generated_code).unwrap();
    println!("cargo:rerun-if-changed=src/data");
}
