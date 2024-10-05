use serde::{Serialize, Deserialize};

use fls::prelude::FLSMsg;

#[derive(Serialize, Deserialize, Clone)]
pub struct Float32{
    pub  data:f32,
}

impl FLSMsg for Float32 {}