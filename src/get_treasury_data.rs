#[path = "parse_treasury_data.rs"] mod parse_treasury_data;


use axum::Json;
use chrono::{DateTime, Utc, Months};

use parse_treasury_data::{parse_treasury_xml, DoubleError, TreasuryBillRates};
use url::{Url, ParseError};

const BASEURL : &str = "https://home.treasury.gov/resource-center/data-chart-center/interest-rates/pages/xml";


pub enum TreasuryBillDataError{
    XMLParseError(xml::reader::Error),
    RequestError(reqwest::Error),
    UrlParseError(ParseError),
    NoEntries
}

fn get_url(date : DateTime<Utc> ) -> Result<Url, ParseError> {
    let date_string = date.format("%Y%m").to_string();
    return Url::parse_with_params(BASEURL, &[("data", "daily_treasury_bill_rates"), ("field_tdr_date_value_month", date_string.as_str())] );
}


async fn make_request(date : DateTime<Utc>) -> Result<Json<TreasuryBillRates> , TreasuryBillDataError >{
        match get_url(date){
            Ok(url) =>{
                let resp =  reqwest::get(url).await;

                match resp{
                    Ok(resp)=> {
                        match resp.bytes().await {
                            Ok(bytes) => {
                                match parse_treasury_xml(bytes){
                                    Ok(json) =>{ return Ok(json); }
                                    Err(DoubleError::NoEntries) =>{ return Err(TreasuryBillDataError::NoEntries); }
                                    Err(DoubleError::XMLParseError(e)) => {return Err(TreasuryBillDataError::XMLParseError(e)); }
                                }
                            }
                            Err(e) => {  return Err(TreasuryBillDataError::RequestError(e));  }
                        }                
                    },

                    Err(e) => {
                        return Err(TreasuryBillDataError::RequestError(e));
                    }
                }
            }
            Err(e) =>{
                return Err( TreasuryBillDataError::UrlParseError(e) );
            }
        }

}

pub async fn get_data() -> Result<Json<TreasuryBillRates> , TreasuryBillDataError > {
    match make_request(Utc::now()).await {
        Ok(json) => {
            return Ok(json);
        }
        Err(TreasuryBillDataError::NoEntries) =>{
            let x = Utc::now();
            x.checked_sub_months(Months::new(1));
            return make_request(x).await;
        }
        Err(e) => {
            return Err(e);
        }

    }
}
