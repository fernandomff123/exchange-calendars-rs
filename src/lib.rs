// src/lib.rs
use chrono::{DateTime, NaiveDate, NaiveTime, Utc, Duration};
use chrono_tz::Tz;

#[derive(Debug)]
pub enum CalendarError {
    MarketNotFound,
}

pub trait ExchangeCalendar {
    fn mic(&self) -> &'static str;
    fn is_trading_day(&self, date: NaiveDate) -> bool;
    fn close_time_on_date(&self, date: NaiveDate) -> Option<NaiveTime>;
    fn is_open_at(&self, dt: &DateTime<Utc>, tz: &Tz, standard_open: NaiveTime) -> bool;
    fn calendar_bounds(&self) -> (NaiveDate, NaiveDate);
}

pub struct GenericCalendar {
    mic: &'static str,
    bytes: &'static [u8],
}

impl GenericCalendar {
    pub fn new(mic: &'static str, bytes: &'static [u8]) -> Self {
        Self { mic, bytes }
    }

    // EXTRAÇÃO DO CABEÇALHO DINÂMICO: Lê a primeira data diretamente do binário
    fn base_date(&self) -> NaiveDate {
        let mut ano_bytes = [0u8; 2];
        ano_bytes.copy_from_slice(&self.bytes[0..2]);
        let ano = u16::from_le_bytes(ano_bytes) as i32;

        let mes = self.bytes[2] as u32;
        let dia = self.bytes[3] as u32;

        NaiveDate::from_ymd_opt(ano, mes, dia).unwrap()
    }
}

impl ExchangeCalendar for GenericCalendar {
    fn mic(&self) -> &'static str { self.mic }

    fn calendar_bounds(&self) -> (NaiveDate, NaiveDate) {
        let start_date = self.base_date();
        // Descontamos os 4 bytes do cabeçalho inicial
        let dados_dias_len = self.bytes.len() - 4;
        let total_days = (dados_dias_len / 3) as i64;
        
        // A data final está implícita no tamanho do ficheiro!
        let end_date = start_date + Duration::days(total_days - 1);
        (start_date, end_date)
    }

    fn is_trading_day(&self, date: NaiveDate) -> bool {
        let start_date = self.base_date();
        let duration = date.signed_duration_since(start_date);
        let day_index = duration.num_days();

        if day_index < 0 { return false; }
        
        // Salta os 4 bytes do cabeçalho para ler os dados do dia
        let byte_offset = 4 + (day_index as usize) * 3;
        if byte_offset >= self.bytes.len() { return false; }

        let day_status = self.bytes[byte_offset];
        day_status == 1 || day_status == 2
    }

    fn close_time_on_date(&self, date: NaiveDate) -> Option<NaiveTime> {
        let start_date = self.base_date();
        let duration = date.signed_duration_since(start_date);
        let day_index = duration.num_days();

        if day_index < 0 { return None; }
        let byte_offset = 4 + (day_index as usize) * 3;
        if byte_offset >= self.bytes.len() { return None; }

        let day_status = self.bytes[byte_offset];
        if day_status == 0 { return None; }

        let min_b1 = self.bytes[byte_offset + 1] as u16;
        let min_b2 = self.bytes[byte_offset + 2] as u16;
        let minutos_desde_meia_noite = min_b1 | (min_b2 << 8);

        let horas = (minutos_desde_meia_noite / 60) as u32;
        let minutos = (minutos_desde_meia_noite % 60) as u32;

        NaiveTime::from_hms_opt(horas, minutos, 0)
    }

    fn is_open_at(&self, dt: &DateTime<Utc>, tz: &Tz, standard_open: NaiveTime) -> bool {
        let dt_local = dt.with_timezone(tz);
        let date_local = dt_local.date_naive();
        let time_local = dt_local.time();

        if !self.is_trading_day(date_local) { return false; }

        if let Some(actual_close) = self.close_time_on_date(date_local) {
            time_local >= standard_open && time_local <= actual_close
        } else {
            false
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/generated_markets.rs"));

