use axum::Json;
use std::{io::BufReader, collections::BTreeMap};
use bytes::{Bytes};

use xml::reader::{EventReader, XmlEvent, Error};

#[derive(Debug)]
pub enum DoubleError{
    NoEntries,
    XMLParseError(Error) ,
}

const FIELD_VALUES : [&str; 7] = ["ROUND_B1_YIELD_4WK_2","ROUND_B1_YIELD_8WK_2","ROUND_B1_YIELD_13WK_2","ROUND_B1_YIELD_17WK_2","ROUND_B1_YIELD_26WK_2", "ROUND_B1_YIELD_52WK_2","INDEX_DATE"];

#[derive(Debug)]
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
pub fn parse_treasury_xml(input : Bytes) -> Result<Json<TreasuryBillRates> , DoubleError >{
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
                return Err(DoubleError::XMLParseError(e));
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }
    if !found_entry{
        return Err(DoubleError::NoEntries);
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

