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
    return Url::parse_with_params(BASEURL, &[("data", "daily_treasury_yield_curve"), ("field_tdr_date_value_month", date_string.as_str())] );
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






const FIELD_VALUES : [&str; 14] = ["BC_1MONTH","BC_2MONTH","BC_3MONTH","BC_4MONTH","BC_6MONTH", "BC_1YEAR","BC_2YEAR","BC_3YEAR","BC_5YEAR","BC_7YEAR","BC_10YEAR","BC_20YEAR","BC_30YEAR","updated"];

#[derive(Debug,Serialize)]
pub struct TreasuryBillRates{
    BC_1MONTH : String,
    BC_2MONTH : String,
    BC_3MONTH : String,
    BC_4MONTH : String,
    BC_6MONTH : String, 
    BC_1YEAR : String,
    BC_2YEAR : String,
    BC_3YEAR : String,
    BC_5YEAR : String,
    BC_7YEAR : String,
    BC_10YEAR : String,
    BC_20YEAR : String,
    BC_30YEAR : String,
    updated : String
}


// From https://home.treasury.gov/resource-center/data-chart-center/interest-rates/TextView?type=daily_treasury_yield_curve&field_tdr_date_value_month=202306
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
        BC_1MONTH : String::new(),
        BC_2MONTH : String::new(),
        BC_3MONTH : String::new(),
        BC_4MONTH : String::new(),
        BC_6MONTH : String::new(),
        BC_1YEAR : String::new(),
        BC_2YEAR : String::new(),
        BC_3YEAR : String::new(),
        BC_5YEAR : String::new(),
        BC_7YEAR : String::new(),
        BC_10YEAR : String::new(),
        BC_20YEAR : String::new(),
        BC_30YEAR : String::new(),
        updated : String::new()
    };
    if let Some(v) = rates.get("BC_1MONTH"){
        rates_struct.BC_1MONTH = v.to_string();
    }
    if let Some(v) = rates.get("BC_2MONTH"){
        rates_struct.BC_2MONTH = v.to_string();
    }
    if let Some(v) = rates.get("BC_3MONTH"){
        rates_struct.BC_3MONTH = v.to_string();
    }
    if let Some(v) = rates.get("BC_4MONTH"){
        rates_struct.BC_4MONTH = v.to_string();
    }
    if let Some(v) = rates.get("BC_6MONTH"){
        rates_struct.BC_6MONTH = v.to_string();
    }
    if let Some(v) = rates.get("BC_1YEAR"){
        rates_struct.BC_1YEAR = v.to_string();
    }
    if let Some(v) = rates.get("BC_2YEAR"){
        rates_struct.BC_2YEAR = v.to_string();
    }
    if let Some(v) = rates.get("BC_3YEAR"){
        rates_struct.BC_3YEAR = v.to_string();
    }
    if let Some(v) = rates.get("BC_5YEAR"){
        rates_struct.BC_5YEAR = v.to_string();
    }
    if let Some(v) = rates.get("BC_7YEAR"){
        rates_struct.BC_7YEAR = v.to_string();
    }
    if let Some(v) = rates.get("BC_10YEAR"){
        rates_struct.BC_10YEAR = v.to_string();
    }
    if let Some(v) = rates.get("BC_20YEAR"){
        rates_struct.BC_20YEAR = v.to_string();
    }
    if let Some(v) = rates.get("BC_30YEAR"){
        rates_struct.BC_30YEAR = v.to_string();
    }
    if let Some(v) = rates.get("updated"){
        rates_struct.updated = v.to_string();
    }
    return Ok( Json(rates_struct) );
}

