use anyhow::Result;
use cosm_orc::{
    config::cfg::Config,
    orchestrator::{cosm_orc::CosmOrc, Key, SigningKey},
};
use cosmwasm_std::Uint128;
use cw20::{Cw20Coin, Cw20ExecuteMsg, TokenInfoResponse};
use cw20_base::msg::{InstantiateMsg, QueryMsg};
use serde_json::Value;
use std::fs;

fn main() -> Result<()> {
    env_logger::init();

    let cfg = Config::from_yaml("config.yaml")?;
    let mut cosm_orc = CosmOrc::new(cfg.clone(), true)?;

    let key = SigningKey {
        name: "validator".to_string(),
        key: Key::Mnemonic("siren window salt bullet cream letter huge satoshi fade shiver permit offer happy immense wage fitness goose usual aim hammer clap about super trend".to_string()),
    };
    let account = key.to_addr(&cfg.chain_cfg.prefix)?;

    cosm_orc.store_contracts("./artifacts", &key, None)?;

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
        None,
        vec![],
    )?;

    let res = cosm_orc.query("cw20_base", &QueryMsg::TokenInfo {})?;
    let info: TokenInfoResponse = res.data()?;

    println!("{:?}", info);

    cosm_orc.execute(
        "cw20_base",
        "ex_tok_burn",
        &Cw20ExecuteMsg::Burn {
            amount: Uint128::new(50),
        },
        &key,
        vec![],
    )?;

    let res = cosm_orc.query("cw20_base", &QueryMsg::TokenInfo {})?;
    let info: TokenInfoResponse = res.data()?;

    println!("{:?}", info);

    let report = cosm_orc.gas_profiler_report().unwrap();

    let j: Value = serde_json::to_value(report)?;
    fs::write("./gas_report.json", j.to_string())?;

    Ok(())
}
