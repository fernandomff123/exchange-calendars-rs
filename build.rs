// build.rs (Na raiz do projeto)
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let data_dir = Path::new("src/data");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_markets.rs");
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir_clean = manifest_dir.replace("\\", "/");

    let python_path = Path::new("env/bin/python");
    let pip_path = Path::new("env/bin/pip");

    // MECANISMO DE VERIFICAÇÃO INTELIGENTE
    if python_path.exists() && pip_path.exists() {
        // 1. Pergunta ao pip (em microssegundos) se o exchange-calendars está desatualizado na internet
        let check_update = Command::new(pip_path)
            .args(&["list", "--outdated", "--format=json"])
            .output()
            .expect("Falha ao verificar atualizações do Python");

        let output_str = String::from_utf8_lossy(&check_update.stdout);
        
        // 2. SÓ SE O FORMATO CONTIVER O NOME É QUE HÁ NOVA VERSÃO!
        if output_str.contains("exchange-calendars") || output_str.contains("pandas") {
            println!("cargo:warning=🔄 Nova versão detetada no Python! Atualizando e correndo codegen...");
            
            // Atualiza de verdade
            let _ = Command::new(pip_path)
                .args(&["install", "--upgrade", "exchange-calendars", "pandas"])
                .status();

            // Corre o Codegen para atualizar as tuas 71 matrizes binárias
            let _ = Command::new(python_path)
                .arg("codegen/generate_data.py")
                .status();
        } else {
            // Se o projeto Python não mudou, o Rust não faz nada e passa à frente instantaneamente!
            println!("cargo:warning=✅ Base de dados do exchange_calendars atualizada. Ignorando codegen.");
        }
    }

    // --- Miolo do Match Estático (Mantém-se igual) ---
    let mut match_arms = String::new();
    if data_dir.exists() {
        for entry in fs::read_dir(data_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "bin") {
                let file_name = path.file_stem().unwrap().to_str().unwrap().to_uppercase();
                let user_mic = if file_name == "24_5" { "24/5".to_string() } else if file_name == "24_7" { "24/7".to_string() } else { file_name.clone() };
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
