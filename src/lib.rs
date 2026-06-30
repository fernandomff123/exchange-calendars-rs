// src/lib.rs
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use chrono_tz::Tz;

const START_YEAR: i32 = 2006;
const START_MONTH: u32 = 6;
const START_DAY: u32 = 29;

#[derive(Debug)]
pub enum CalendarError {
    MarketNotFound,
}

pub trait ExchangeCalendar {
    fn mic(&self) -> &'static str;
    fn is_trading_day(&self, date: NaiveDate) -> bool;
    fn close_time_on_date(&self, date: NaiveDate) -> Option<NaiveTime>;

    // NOVO MÉTODO: Verifica se o mercado está aberto ao minuto/segundo exato em UTC
    fn is_open_at(&self, dt: &DateTime<Utc>, tz: &Tz, open_time_padrao: NaiveTime) -> bool;
}

pub struct GenericCalendar {
    mic: &'static str,
    bytes: &'static [u8],
}

impl GenericCalendar {
    pub fn new(mic: &'static str, bytes: &'static [u8]) -> Self {
        Self { mic, bytes }
    }
}

impl ExchangeCalendar for GenericCalendar {
    fn mic(&self) -> &'static str { self.mic }

    fn is_trading_day(&self, date: NaiveDate) -> bool {
        let start_date = NaiveDate::from_ymd_opt(START_YEAR, START_MONTH, START_DAY).unwrap();
        let duration = date.signed_duration_since(start_date);
        let day_index = duration.num_days();

        if day_index < 0 { return false; }
        let byte_offset = (day_index as usize) * 3;
        if byte_offset >= self.bytes.len() { return false; }

        let day_status = self.bytes[byte_offset];
        day_status == 1 || day_status == 2
    }

    fn close_time_on_date(&self, date: NaiveDate) -> Option<NaiveTime> {
        let start_date = NaiveDate::from_ymd_opt(START_YEAR, START_MONTH, START_DAY).unwrap();
        let duration = date.signed_duration_since(start_date);
        let day_index = duration.num_days();

        if day_index < 0 { return None; }
        let byte_offset = (day_index as usize) * 3;
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

    // IMPLEMENTAÇÃO DE OPEN STATUS ULTRA-RÁPIDA
    fn is_open_at(&self, dt: &DateTime<Utc>, tz: &Tz, open_time_padrao: NaiveTime) -> bool {
        // 1. Converter o instante UTC para a hora civil local daquela bolsa
        let dt_local = dt.with_timezone(tz);
        let data_local = dt_local.date_naive();
        let hora_local = dt_local.time();

        // 2. Se não for dia de trading local, está fechado de certeza!
        if !self.is_trading_day(data_local) {
            return false;
        }

        // 3. Buscar a hora de fecho real registada na matriz binária para ESTE dia
        if let Some(hora_fecho_real) = self.close_time_on_date(data_local) {
            // Se o relógio local estiver entre a abertura padrão e o fecho real (trata half-days!) -> ABERTO!
            hora_local >= open_time_padrao && hora_local <= hora_fecho_real
        } else {
            false
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/generated_markets.rs"));
