use crate::pump_fun::cpi::PumpFunCpiLog;
use crate::raydium::cpi::RaydiumCpiLog;

#[derive(Debug, Clone)]
pub enum CpiLog {
    PumpFun(PumpFunCpiLog),
    Raydium(RaydiumCpiLog),
}
