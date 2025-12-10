#![allow(unused)]

use crate::error;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Metadata {}

impl Metadata {
    pub fn from_string(data: &String) -> Result<Self, error::Error> {
        todo!()
    }

    pub fn to_string(&self) -> &String {
        todo!()
    }
}
