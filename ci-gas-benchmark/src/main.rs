use anyhow::Result;
use cosm_orc::{
    config::{
        cfg::Config,
        key::{Key, SigningKey},
    },
    orchestrator::cosm_orc::{CosmOrc, WasmMsg},
    profilers::gas_profiler::GasProfiler,
};
use cw20_base::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use serde_json::Value;
use std::fs;

fn main() -> Result<()> {
    env_logger::init();

    let mut cosm_orc =
        CosmOrc::new(Config::from_yaml("config.yaml")?)?.add_profiler(Box::new(GasProfiler::new()));

    let key = SigningKey {
        name: "validator".to_string(),
        key: Key::Mnemonic("siren window salt bullet cream letter huge satoshi fade shiver permit offer happy immense wage fitness goose usual aim hammer clap about super trend".to_string()),
    };

    cosm_orc.store_contracts("./artifacts", &key)?;

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

    cosm_orc.process_msgs("cw20_base", "ex_tok_info", &msgs, &key)?;

    let reports = cosm_orc.profiler_reports().unwrap();

    let j: Value = serde_json::from_slice(&reports[0].json_data)?;
    fs::write("./gas_report.json", j.to_string())?;

    Ok(())
}
