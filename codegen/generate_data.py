import exchange_calendars as xcals
import pandas as pd
import struct
import os

def export_all_markets():
    all_mics = sorted(xcals.get_calendar_names(include_aliases=False))
    print(f"Foram encontradas {len(all_mics)} bolsas mundiais prontas para exportação!\n")

    os.makedirs("src/data", exist_ok=True)

    # Janela temporal controlada pelo Python (Pode mudar estes anos quando quiser!)
    START_STR = "2000-01-01"
    END_STR = "2028-12-31"

    start_date_obj = pd.Timestamp(START_STR).date()

    for mic_code in all_mics:
        try:
            cal = xcals.get_calendar(mic_code)
        except Exception as e:
            print(f"⚠️ Ignorando {mic_code}: Erro ao inicializar ({e})")
            continue

        cal_start = cal.first_session.date()
        cal_end = cal.last_session.date()
        
        dates = pd.date_range(start=START_STR, end=END_STR, freq='D')
        closes_dict = {session.date(): close_time for session, close_time in cal.closes.items()}
        
        # CABEÇALHO DINÂMICO (4 Bytes): Escreve Ano (u16), Mês (u8), Dia (u8) no início do binário
        binary_data = bytearray(struct.pack("<HBB", start_date_obj.year, start_date_obj.month, start_date_obj.day))

        for d in dates:
            current_date = d.date()
            day_status = 0 
            close_time_minutes = 0

            if cal_start <= current_date <= cal_end and cal.is_session(current_date):
                day_status = 1
                close_datetime_utc = closes_dict.get(current_date)
                
                if close_datetime_utc:
                    close_datetime_local = close_datetime_utc.tz_convert(cal.tz)
                    close_time = close_datetime_local.time()
                    close_time_minutes = close_time.hour * 60 + close_time.minute
                
                if d.floor('D') in cal.early_closes:
                    day_status = 2

            day_bytes = struct.pack("<BH", day_status, close_time_minutes)
            binary_data.extend(day_bytes)

        sanitized_mic = mic_code.lower().replace("/", "_")
        filename = f"src/data/{sanitized_mic}.bin"
        
        with open(filename, "wb") as f:
            f.write(binary_data)
            
        print(f"✅ {mic_code} concluído -> {filename} ({len(binary_data) // 1024} KB)")

if __name__ == "__main__":
    export_all_markets()
