use cid::{Cid, Version};
use std::str::FromStr;
use crate::error::ContractError;
use cosmwasm_std::{
    Response,
};

pub fn validate_datatype(
    val: String
) -> Result<Response, ContractError> {
    for byte in val.as_bytes().iter() {
        // - && 0-9 && a-z
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect data-type. a-z0-9- allowed".to_string()});
        }
    }
    
    Ok(Response::default())
}


pub fn validate_particle(
    particle: Option<String>
) -> Result<Response, ContractError> {
    if !particle.as_ref().is_none() {
        let particle_value:Cid;
        let try_particle = Cid::from_str(&particle.as_ref().unwrap().clone());
        if try_particle.is_ok() {
            particle_value = try_particle.unwrap();

            if particle_value.version() != Version::V0 {

                return Err(ContractError::IncorrectInputData {val: "Incorrect particle".to_string()});

            } 
        } else {
            return Err(ContractError::IncorrectInputData {val: "Incorrect particle".to_string()});
        }
        Ok(Response::default())
    } else {
        Ok(Response::default())
    }
}


