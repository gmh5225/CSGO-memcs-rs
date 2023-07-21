use std::collections::HashMap;

use toml::{map::Map, Value};

use super::error::Error;

#[derive(Clone, Default)]
pub struct Offsets {
    sigs: HashMap<String, i64>,
    vars: HashMap<String, i64>,
}

impl Offsets {
    pub fn new(signatures: Map<String, Value>, netvars: Map<String, Value>) -> Self {
        let mut sigs = HashMap::new();
        signatures.into_iter().for_each(|f| {
            sigs.insert(f.0, f.1.as_integer().unwrap());
        });

        let mut vars = HashMap::new();
        netvars.into_iter().for_each(|f| {
            vars.insert(f.0, f.1.as_integer().unwrap());
        });

        Self { sigs, vars }
    }

    pub fn get_sig(&self, sig_name: &str) -> Result<i64, super::error::Error> {
        self.sigs
            .get(sig_name)
            .copied()
            .ok_or(Error::SignatureNotFoundError)
    }

    pub fn get_var(&self, var_name: &str) -> Result<i64, super::error::Error> {
        self.vars
            .get(var_name)
            .copied()
            .ok_or(Error::NetvarNotFoundError)
    }
}

pub fn get_offsets() -> Result<Offsets, Error> {
    let offsets = read_offsets_from_file()?;

    let netvars = offsets["netvars"].as_table().unwrap().clone();
    let signatures = offsets["signatures"].as_table().unwrap().clone();

    let offsets = Offsets::new(signatures, netvars);

    Ok(offsets)
}

fn read_offsets_from_file() -> Result<Value, Error> {
    if std::path::Path::new("./offsets.toml").exists() {
        let file_content = std::fs::read("./offsets.toml")?;

        let file_content = String::from_utf8(file_content)?;
        let file_content = file_content.parse::<Value>()?;

        Ok(file_content)
    } else {
        let res = reqwest::blocking::get(
            "https://raw.githubusercontent.com/frk1/hazedumper/master/csgo.toml",
        )?;
        let content = res.text()?;

        std::fs::write("./offsets.toml", content)?;

        read_offsets_from_file()
    }
}
