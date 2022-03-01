use std::ops::Add;
use std::str::FromStr;
use cid::{Cid, Version};
use cid::multihash::{Code, MultihashDigest};
use cosmwasm_std::StdError;

pub fn prepare_particle(input: String) -> Result<Cid, StdError> {
    if input.len() == 0 || input.len() > 256 {
        return Err(StdError::generic_err("invalid data"));
    }

    // unixfs/dagnode/proto shortcut
    // wrap input bytes as a dagnode unixfs file
    let length: u8 = input.len() as u8;
    let mut raw: Vec<u8> = vec![10, length.add(6) as u8, 8, 2, 18, length];
    raw.append(&mut input.as_bytes().to_vec());
    raw.append(&mut vec![24, length]);

    let h = Code::Sha2_256.digest(&raw.as_slice());
    let particle = Cid::new_v0(h).unwrap();

    Ok(particle)
}

pub fn check_particle(input: String) -> Result<Cid, StdError> {
    let particle:Cid;
    let try_particle = Cid::from_str(&input.clone());
    if try_particle.is_ok() {
        particle = try_particle.unwrap();
        if particle.version() != Version::V0 {
            return Err(StdError::generic_err("invalid particle version"));
        }
    } else {
        return Err(StdError::generic_err("invalid particle"));
    }

    Ok(particle)
}