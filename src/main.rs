use chrono::{TimeZone, Utc};
use chrono_tz::Europe::Madrid;
use chrono_tz::America::Sao_Paulo;
use chrono::NaiveTime;
use exchange_calendars_rs::get_calendar;

fn main() {
    println!("--- VALIDAÇÃO DE INSTANTE DE TRADING EM TEMPO REAL ---");

    // Vamos simular um instante no tempo: Terça-feira, 30 de Junho de 2026 às 15:00:00 UTC
    let instante_utc = Utc.with_ymd_and_hms(2026, 6, 30, 15, 0, 0).unwrap();
    println!("Instante de Análise Global: {} UTC", instante_utc);

    // 1. Testar Madrid (Em Madrid são 17:00h - Mercado Aberto, fecha às 17:30)
    let xmad = get_calendar("XMAD").unwrap();
    let abertura_madrid = NaiveTime::from_hms_opt(9, 0, 0).unwrap(); // Madrid abre às 09:00
    println!(
        "Bolsa: {} (Madrid)  -> Aberta neste segundo UTC? -> {}",
             xmad.mic(),
             xmad.is_open_at(&instante_utc, &Madrid, abertura_madrid)
    );

    // 2. Testar Bovespa (Em São Paulo são 12:00h - Mercado Aberto, abre às 10:00 e fecha às 17:55)
    let bvmf = get_calendar("BVMF").unwrap();
    let abertura_bovespa = NaiveTime::from_hms_opt(10, 0, 0).unwrap(); // B3 abre às 10:00
    println!(
        "Bolsa: {} (Bovespa) -> Aberta neste segundo UTC? -> {}",
             bvmf.mic(),
             bvmf.is_open_at(&instante_utc, &Sao_Paulo, abertura_bovespa)
    );

    // 3. Testar a mesma bolsa de Madrid mas 3 horas mais tarde (18:00 UTC -> 20:00h locais -> FECHADA)
    let instante_noite_utc = Utc.with_ymd_and_hms(2026, 6, 30, 18, 0, 0).unwrap();
    println!(
        "Bolsa: {} às 18:00 UTC -> Aberta neste segundo UTC? -> {}",
        xmad.mic(),
             xmad.is_open_at(&instante_noite_utc, &Madrid, abertura_madrid)
    );
}
