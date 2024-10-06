use serde::{Serialize, Deserialize};

use fls::prelude::FLSMsg;

#[derive(Serialize, Deserialize, Clone)]
pub struct Float32{
    pub  data:f32,
}

impl FLSMsg for Float32 {}
#[derive(Serialize, Deserialize, Clone)]
pub struct Int32{
    pub  data:i32,
}

impl FLSMsg for Int32 {}
#[derive(Serialize, Deserialize, Clone)]
pub struct Float64{
    pub  data:f64,
}

impl FLSMsg for Float64 {}
#[derive(Serialize, Deserialize, Clone)]
pub struct Int64{
    pub  data:i64,
}

impl FLSMsg for Int64 {}
#[derive(Serialize, Deserialize, Clone)]
pub struct Bool{
    pub  data:bool,
}

impl FLSMsg for Bool {}