# exchange-calendars-rs

A zero-overhead, high-performance global exchange calendar library for Rust, offering **O(1) lookups** for trading sessions and market close microstructures across 71 global markets.

## Key Features

- **Global Coverage**: 71 stock exchanges mapped natively (NYSE, Bovespa, Madrid, Tokyo, London, Crypto 24/7, and more).
- **Blazing Fast**: Uses compile-time code generation (`build.rs`) to bake highly-compressed binary matrices (3 bytes per day) straight into the compiled library. No runtime disk I/O.
- **Accurate Microstructure**: Resolves weekends, national holidays, and dynamic early closes (half-days like Christmas Eve) instantly.
- **Data Pipeline Friendly**: Designed specifically to feed fast calculations into massive DataFrames like `Polars`.

## Architecture Overview

The library relies on a robust hybrid design:
1. **Python Codegen (`codegen/`)**: Extracts calendar metadata from the industry-standard `exchange_calendars` library and packs daily status and local closing minutes into tiny 3-byte data blocks.
2. **Rust Meta-compilation (`build.rs`)**: Scans all compiled assets at build time and builds static lookups automatically.

## Quick Start

```rust
use chrono::NaiveDate;
use exchange_calendars_rs::get_calendar;

fn main() {
    let test_date = NaiveDate::from_ymd_opt(2026, 6, 30).unwrap();

    // Load London Stock Exchange
    let lse = get_calendar("XLON").unwrap();
    println!("Trading Day: {}", lse.is_trading_day(test_date));
    println!("Local Close Time: {:?}", lse.close_time_on_date(test_date));
}
```
