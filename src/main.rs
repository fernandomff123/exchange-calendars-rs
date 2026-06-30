use exchange_calendars_rs::get_calendar;

fn main() {
    println!("--- AUDITORIA DE LIMITES DINÂMICOS DA BASE DE DADOS ---");

    let mercados = vec!["XMAD", "BVMF", "XNYS", "24/7"];

    for mic in mercados {
        let cal = get_calendar(mic).unwrap();
        // O método calendar_bounds() vai ler o cabeçalho e medir o tamanho do ficheiro!
        let (inicio, fim) = cal.calendar_bounds();
        
        println!(
            "Bolsa: {:<6} | Janela Temporal Calculada: {} até {}",
            cal.mic(),
            inicio,
            fim
        );
    }
}
