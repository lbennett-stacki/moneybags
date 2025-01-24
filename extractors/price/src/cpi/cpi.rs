use crate::pump_fun::cpi::PumpFunCpiLog;

#[derive(Debug, Clone)]
pub enum CpiLog {
    PumpFun(PumpFunCpiLog),
}
