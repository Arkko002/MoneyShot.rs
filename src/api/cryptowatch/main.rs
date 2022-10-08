use super::lib::{MarketPrice, BASE_URL};

pub async fn get_market_pair_price(market: String, pair: String) -> Result<MarketPrice, ()> {
    // TODO Better url handling
    let url = format!(
        "{base}/markets/{market}/{pair}/price",
        base = BASE_URL,
        market = market,
        pair = pair
    );

    let market_price: MarketPrice = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    Ok(market_price)
}
