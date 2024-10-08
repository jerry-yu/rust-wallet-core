use super::ordinal::OrdinalsInscription;
use std::ops::Deref;
use tw_coin_entry::error::prelude::*;
use tw_hash::H264;

#[derive(Debug, Clone)]
pub struct Brc20Ticker(String);

impl Brc20Ticker {
    pub fn new(string: String) -> SigningResult<Self> {
        // Brc20Ticker must be a 4,5-letter identifier.
        let len = string.len();
        if len != 4 && len != 5 {
            return SigningError::err(SigningErrorType::Error_invalid_params)
                .context("BRC20 ticker must be exactly 4 or 5 bytes length");
        }

        Ok(Brc20Ticker(string))
    }
}

const BRC20_PROTOCOL_ID: &str = "brc-20";
const BRC20_MIME: &[u8] = b"text/plain;charset=utf-8";

pub struct BRC20TransferInscription(OrdinalsInscription);

impl Deref for BRC20TransferInscription {
    type Target = OrdinalsInscription;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BRC20TransferInscription {
    pub fn new(
        recipient: &H264,
        ticker: &Brc20Ticker,
        value: &str,
    ) -> SigningResult<BRC20TransferInscription> {
        let payload = format!(
            "{{\"p\":\"{protocol}\",\"op\":\"transfer\",\"tick\":\"{ticker}\",\"amt\":\"{amt}\"}}",
            protocol = BRC20_PROTOCOL_ID,
            ticker = ticker.0,
            amt = value
        );

        let inscription = OrdinalsInscription::new(BRC20_MIME, payload.as_bytes(), recipient)?;
        Ok(BRC20TransferInscription(inscription))
    }
}

pub struct BRC20MintInscription(OrdinalsInscription);

impl Deref for BRC20MintInscription {
    type Target = OrdinalsInscription;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BRC20MintInscription {
    pub fn new(
        recipient: &H264,
        ticker: &Brc20Ticker,
        value: &str,
    ) -> SigningResult<BRC20MintInscription> {
        let payload = format!(
            "{{\"p\":\"{protocol}\",\"op\":\"mint\",\"tick\":\"{ticker}\",\"amt\":\"{amt}\"}}",
            protocol = BRC20_PROTOCOL_ID,
            ticker = ticker.0,
            amt = value
        );

        let inscription = OrdinalsInscription::new(BRC20_MIME, payload.as_bytes(), recipient)?;
        Ok(BRC20MintInscription(inscription))
    }
}

pub struct BRC20DeployInscription(OrdinalsInscription);

impl Deref for BRC20DeployInscription {
    type Target = OrdinalsInscription;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BRC20DeployInscription {
    pub fn new(
        recipient: &H264,
        ticker: &Brc20Ticker,
        max: &str,
        lim: &str,
    ) -> SigningResult<BRC20DeployInscription> {
        let payload = format!(
            "{{\"p\":\"{protocol}\",\"op\":\"deploy\",\"tick\":\"{ticker}\",\"max\":\"{max}\",\"lim\":\"{lim}\"}}",
            protocol = BRC20_PROTOCOL_ID,
            ticker = ticker.0,
            max = max,
            lim = lim,
        );

        let inscription = OrdinalsInscription::new(BRC20_MIME, payload.as_bytes(), recipient)?;
        Ok(BRC20DeployInscription(inscription))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brc20_ticker_validity() {
        // Must be four characters.
        let ticker = Brc20Ticker::new("invalid".to_string());
        assert!(ticker.is_err());

        let ticker = Brc20Ticker::new("asdf".to_string());
        assert!(ticker.is_ok());

        // Cover clone implemenation.
        let ticker = ticker.unwrap();

        let _cloned = ticker.clone();
        let _ticker = ticker;
    }
}
