use axum::Json;
use serde::Serialize;
use std::{io::BufReader, collections::BTreeMap};
use bytes::{Bytes};

use xml::reader::{EventReader, XmlEvent};

use chrono::{DateTime, Utc, Months};

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
                                return parse_treasury_xml(bytes);
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
            let curr_time = Utc::now();
            match curr_time.checked_sub_months(Months::new(1)){
                Some(one_month_ago) =>{ 
                    return make_request(one_month_ago).await;
                }
                None => { return Err(TreasuryBillDataError::NoEntries) }
            };
        }
        Err(e) => {
            return Err(e);
        }

    }
}






const FIELD_VALUES : [&str; 7] = ["ROUND_B1_YIELD_4WK_2","ROUND_B1_YIELD_8WK_2","ROUND_B1_YIELD_13WK_2","ROUND_B1_YIELD_17WK_2","ROUND_B1_YIELD_26WK_2", "ROUND_B1_YIELD_52WK_2","INDEX_DATE"];

#[derive(Debug,Serialize)]
pub struct TreasuryBillRates{
    ROUND_B1_YIELD_4WK_2 : String,
    ROUND_B1_YIELD_8WK_2 : String,
    ROUND_B1_YIELD_13WK_2 : String,
    ROUND_B1_YIELD_17WK_2 : String,
    ROUND_B1_YIELD_26WK_2 : String,
    ROUND_B1_YIELD_52WK_2 : String,
    INDEX_DATE : String
}


// From https://home.treasury.gov/resource-center/data-chart-center/interest-rates/TextView
pub fn parse_treasury_xml(input : Bytes) -> Result<Json<TreasuryBillRates> , TreasuryBillDataError >{
    let stream = BufReader::new(input.as_ref());
    let parser = EventReader::new(stream);


    let mut found_entry = false;
    let mut current_catagory = String::new();
    let mut rates = BTreeMap::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                current_catagory = name.local_name;
                if !found_entry && current_catagory.as_str() == "entry" {
                    found_entry = true;
                }
            }
            Ok(XmlEvent::Characters(s) ) => {
                if FIELD_VALUES.contains( &current_catagory.as_str()   ) {
                    rates.insert(current_catagory.clone(), s);
                }
            }
            Err(e) => {
                return Err(TreasuryBillDataError::XMLParseError(e));
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }
    if !found_entry{
        return Err(TreasuryBillDataError::NoEntries);
    }
    let mut rates_struct = TreasuryBillRates{
        ROUND_B1_YIELD_4WK_2 : String::new(),
        ROUND_B1_YIELD_8WK_2 : String::new(),
        ROUND_B1_YIELD_13WK_2 : String::new(),
        ROUND_B1_YIELD_17WK_2 : String::new(),
        ROUND_B1_YIELD_26WK_2 : String::new(),
        ROUND_B1_YIELD_52WK_2 : String::new(),
        INDEX_DATE : String::new()
    };
    if let Some(v) = rates.get("ROUND_B1_YIELD_4WK_2"){
        rates_struct.ROUND_B1_YIELD_4WK_2 = v.to_string();
    }
    if let Some(v) = rates.get("ROUND_B1_YIELD_8WK_2"){
        rates_struct.ROUND_B1_YIELD_8WK_2 = v.to_string();
    }
    if let Some(v) = rates.get("ROUND_B1_YIELD_13WK_2"){
        rates_struct.ROUND_B1_YIELD_13WK_2 = v.to_string();
    }
    if let Some(v) = rates.get("ROUND_B1_YIELD_17WK_2"){
        rates_struct.ROUND_B1_YIELD_17WK_2 = v.to_string();
    }
    if let Some(v) = rates.get("ROUND_B1_YIELD_26WK_2"){
        rates_struct.ROUND_B1_YIELD_26WK_2 = v.to_string();
    }
    if let Some(v) = rates.get("ROUND_B1_YIELD_52WK_2"){
        rates_struct.ROUND_B1_YIELD_52WK_2 = v.to_string();
    }
    if let Some(v) = rates.get("INDEX_DATE"){
        rates_struct.INDEX_DATE = v.to_string();
    }
    return Ok( Json(rates_struct) );
}

