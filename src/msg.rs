use serde::{Serialize, Deserialize};

use fls::prelude::FLSMsg;

#[derive(Serialize, Deserialize, Clone)]
pub struct Float32{
    pub  data:f32,
}
impl Float32 {
    pub fn new()->Float32
    {
        Float32 { data: 0.0 }
    }
}
impl FLSMsg for Float32 {}

#[derive(Serialize, Deserialize, Clone)]
pub struct Int32{
    pub  data:i32,
}
impl Int32 {
    pub fn new()->Int32
    {
        Int32 { data: 0 }
    }
}
impl FLSMsg for Int32 {}

#[derive(Serialize, Deserialize, Clone)]
pub struct Float64{
    pub  data:f64,
}
impl Float64 {
    pub fn new()->Float64
    {
        Float64 { data: 0.0 }
    }
}
impl FLSMsg for Float64 {}


#[derive(Serialize, Deserialize, Clone)]
pub struct Int64{
    pub  data:i64,
}
impl Int64 {
    pub fn new()->Int64
    {
        Int64 { data: 0 }
    }
}
impl FLSMsg for Int64 {}


#[derive(Serialize, Deserialize, Clone)]
pub struct Bool{
    pub  data:bool,
}
impl Bool {
    pub fn new()->Bool
    {
        Bool { data: false }
    }
}

impl FLSMsg for Bool {}
