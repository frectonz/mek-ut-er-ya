mod ethiopian;
mod gregorian;
pub use ethiopian::*;
pub use gregorian::*;

impl From<GregorianYear> for EthiopianYear {
    fn from(gregorian: GregorianYear) -> Self {
        Self::from_jdn(gregorian.to_jdn())
    }
}

impl From<EthiopianYear> for GregorianYear {
    fn from(ethiopian: EthiopianYear) -> Self {
        Self::from_jdn(ethiopian.to_jdn())
    }
}
