use anyhow::Result;
use cosm_orc::{
    config::{
        cfg::Config,
        key::{Key, SigningKey},
    },
    orchestrator::cosm_orc::CosmOrc,
    profilers::gas_profiler::GasProfiler,
};
use cosmwasm_std::Uint128;
use cw20::{Cw20Coin, Cw20ExecuteMsg, TokenInfoResponse};
use cw20_base::msg::{InstantiateMsg, QueryMsg};
use serde_json::Value;
use std::fs;

fn main() -> Result<()> {
    env_logger::init();

    let cfg = Config::from_yaml("config.yaml")?;
    let mut cosm_orc = CosmOrc::new(cfg.clone())?.add_profiler(Box::new(GasProfiler::new()));

    let key = SigningKey {
        name: "validator".to_string(),
        key: Key::Mnemonic("siren window salt bullet cream letter huge satoshi fade shiver permit offer happy immense wage fitness goose usual aim hammer clap about super trend".to_string()),
    };
    let account = key.to_account(&cfg.chain_cfg.prefix)?;

    cosm_orc.store_contracts("./artifacts", &key)?;

    cosm_orc.instantiate(
        "cw20_base",
        "ex_tok_info",
        &InstantiateMsg {
            name: "Meme Token".to_string(),
            symbol: "MEME".to_string(),
            decimals: 6,
            initial_balances: vec![Cw20Coin {
                address: account.to_string(),
                amount: Uint128::new(100),
            }],
            mint: None,
            marketing: None,
        },
        &key,
    )?;

    let res = cosm_orc.query("cw20_base", "ex_tok_info", &QueryMsg::TokenInfo {})?;
    let info: TokenInfoResponse = res.data()?;

    println!("{:?}", info);

    cosm_orc.execute(
        "cw20_base",
        "ex_tok_burn",
        &Cw20ExecuteMsg::Burn {
            amount: Uint128::new(50),
        },
        &key,
    )?;

    let res = cosm_orc.query("cw20_base", "ex_tok_info", &QueryMsg::TokenInfo {})?;
    let info: TokenInfoResponse = res.data()?;

    println!("{:?}", info);

    let reports = cosm_orc.profiler_reports().unwrap();

    let j: Value = serde_json::from_slice(&reports[0].json_data)?;
    fs::write("./gas_report.json", j.to_string())?;

    Ok(())
}
