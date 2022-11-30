use cid::{Cid, Version};
use std::str::FromStr;
use crate::error::ContractError;
use cosmwasm_std::{
    Response,
};

/*
 * Basic rule /[a-z0-9-]/
 */
pub fn validate_by_basic_rule(
    val: String,
    field_name: String,
) -> Result<Response, ContractError> {

    for byte in val.as_bytes().iter() {
        // - && 0-9 && a-z
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: format!("Incorrect value for field field {}. Allowed expression /[a-z0-9-]/", field_name).to_string()});
        }
    }
    
    Ok(Response::default())
}

/*
 * Basic rule /[0-9]/
 */
pub fn validate_period(
    val: String,
    field_name: String,
) -> Result<Response, ContractError> {

    for byte in val.as_bytes().iter() {
        // 0-9
        if *byte < 48 || *byte > 57 {
            return Err(ContractError::IncorrectInputData {val: format!("Incorrect value for field field {}. Allowed expression /[0-9]/", field_name).to_string()});
        }
    }

    Ok(Response::default())
}

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

pub fn validate_url(
    val: String
) -> Result<Response, ContractError> {

    for byte in val.as_bytes().iter() {
        // : / . - _ 0-9 a-z A-Z
        if  (*byte != 58) && (*byte != 95) && (*byte != 45) && (*byte != 47) && (*byte != 46) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) && (*byte < 65 || *byte > 90)  {
            return Err(ContractError::IncorrectInputData {val: "Incorrect data-type. a-z0-9- allowed".to_string()});
        }
    }
    
    Ok(Response::default())
}


// pub fn validate_particle(
//     particle: Option<String>
// ) -> Result<Response, ContractError> {
//     // let defaultValue="";

//     return validate_ipfs_cid(particle.unwrap())
//     // let particle_value:Cid;
//     // let try_particle = Cid::from_str(&particle.unwrap().clone());
//     // if try_particle.is_ok() {
//     //     particle_value = try_particle.unwrap();

//     //     if particle_value.version() != Version::V0 {

//     //         return Err(ContractError::IncorrectInputData {val: "Incorrect particle".to_string()});

//     //     } 
//     // } else {
//     //     return Err(ContractError::IncorrectInputData {val: "Incorrect particle".to_string()});
//     // }
//     // Ok(Response::default())
// }

pub fn validate_ipfs_cid(
    particle: String
) -> Result<Response, ContractError> {

        let particle_value:Cid;
        let try_particle = Cid::from_str(&particle.clone());
        if try_particle.is_ok() {
            particle_value = try_particle.unwrap();

            if particle_value.version() != Version::V0 {

                return Err(ContractError::IncorrectInputData {val: "Incorrect cid".to_string()});

            } 
        } else {
            return Err(ContractError::IncorrectInputData {val: "Incorrect cid".to_string()});
        }
        Ok(Response::default())

}


