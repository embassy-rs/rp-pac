use super::*;
pub trait Access {}
impl Access for R {}
impl Access for W {}
impl Access for RW {}
