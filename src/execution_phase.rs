use anyhow::Result;
use seda_sdk_rs::{elog, http_fetch, log, Process};
use serde::{Deserialize, Serialize};

// ============================================================================
// DATA STRUCTURES
// ============================================================================

#[derive(Serialize, Deserialize)]
struct KalshiMarket {
    yes_bid: u16,
}

#[derive(Serialize, Deserialize)]
struct KalshiMarketResponse {
    market: KalshiMarket,
}

// ============================================================================
// EXECUTION PHASE - FETCHES LIVE DATA FROM KALSHI
// ============================================================================

/**
 * Executes the data request phase within the SEDA network.
 * This phase fetches bid and ask prices for Kalshi markets based on a series ticker input.
 */
pub fn execution_phase() -> Result<()> {
    // Retrieve the input parameters for the data request (DR).
    // Expected to be a series ticker (e.g., "KXGDP").
    let dr_inputs_raw = String::from_utf8(Process::get_inputs())?;
    let series_ticker = dr_inputs_raw.trim();

    log!("Fetching Kalshi market data for series: {}", series_ticker);

    // Step 1: Get series information
    let series_response = http_fetch(
                "https://api.elections.kalshi.com/trade-api/v2/markets/KXWTAMATCH-25SEP13BARJOV-BAR",
        None,
    );


    // Check if the series request was successful
    if !series_response.is_ok() {
        elog!(
            "Series HTTP Response was rejected: {} - {}",
            series_response.status,
            String::from_utf8(series_response.bytes)?
        );
        Process::error("Error while fetching series information".as_bytes());
        return Ok(());
    }

    // Parse series information
    let series_data = serde_json::from_slice::<KalshiMarketResponse>(&series_response.bytes)?;
    log!(
        "Maket Data: {}",
        serde_json::to_string_pretty(&series_data)?
    );

    Process::success(series_data.market.yes_bid.to_string().as_bytes());
    Ok(())
}
