use anyhow::Result;
use cosm_orc::{
    config::cfg::Config,
    orchestrator::cosm_orc::{CosmOrc, WasmMsg},
    profilers::gas_profiler::GasProfiler,
};
use cw20_base::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use serde_json::Value;
use std::fs;

fn main() -> Result<()> {
    env_logger::init();

    let gas_report_out = "./gas_report.json";
    let mut cosm_orc =
        CosmOrc::new(Config::from_yaml("config.yaml")?).add_profiler(Box::new(GasProfiler::new()));

    let msgs: Vec<WasmMsg<InstantiateMsg, ExecuteMsg, QueryMsg>> = vec![
        WasmMsg::InstantiateMsg(InstantiateMsg {
            name: "Meme Token".to_string(),
            symbol: "MEME".to_string(),
            decimals: 6,
            initial_balances: vec![],
            mint: None,
            marketing: None,
        }),
        WasmMsg::QueryMsg(QueryMsg::TokenInfo {}),
    ];

    cosm_orc.process_msgs("cw20_base".to_string(), &msgs)?;

    let reports = cosm_orc.profiler_reports()?;

    let j: Value = serde_json::from_slice(&reports[0].json_data)?;
    fs::write(gas_report_out, j.to_string())?;

    Ok(())
}
