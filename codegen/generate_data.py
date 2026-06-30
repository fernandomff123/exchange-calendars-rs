import exchange_calendars as xcals
import pandas as pd
import struct
import os

def export_all_markets():
    all_mics = sorted(xcals.get_calendar_names(include_aliases=False))
    print(f"Foram encontradas {len(all_mics)} bolsas mundiais prontas para exportação!\n")

    os.makedirs("src/data", exist_ok=True)

    for mic_code in all_mics:
        try:
            cal = xcals.get_calendar(mic_code)
        except Exception as e:
            print(f"⚠️ Ignorando {mic_code}: Erro ao inicializar ({e})")
            continue

        start_date = cal.first_session.date()
        end_date = cal.last_session.date()

        dates = pd.date_range(start="2006-06-29", end="2027-06-29", freq='D')
        closes_dict = {session.date(): close_time for session, close_time in cal.closes.items()}

        binary_data = bytearray()

        for d in dates:
            current_date = d.date()
            day_status = 0
            close_time_minutes = 0

            if start_date <= current_date <= end_date and cal.is_session(current_date):
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

        # 🧠 SANITIZATION: Replace '/' with '_' to prevent directory breakdown errors (e.g., 24/5 -> 24_5)
        sanitized_mic = mic_code.lower().replace("/", "_")
        filename = f"src/data/{sanitized_mic}.bin"

        with open(filename, "wb") as f:
            f.write(binary_data)

        print(f"✅ {mic_code} concluído -> {filename} ({len(binary_data) // 1024} KB)")

if __name__ == "__main__":
    export_all_markets()

