mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
        }

        links {
            Serialization(::serialization::Error, ::serialization::ErrorKind);
        }

        errors {
            InvalidValueType {
                description("invalid registry value type"),
                display("invalid registry value type"),
            }
        }
    }
}

pub use self::errors::*;

use serialization;
use types;
use winreg::enums::*;
use winreg::{RegKey, RegValue};

const KEY_PATH: &'static str =
    "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings\\Connections";
const VALUE_NAME: &'static str = "DefaultConnectionSettings";

fn open_key() -> Result<RegKey> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey(KEY_PATH)?;
    return Ok(key);
}

fn write_raw(bytes: Vec<u8>) -> Result<()> {
    let value = RegValue {
        vtype: REG_BINARY,
        bytes,
    };
    let key = open_key()?;
    key.set_raw_value(VALUE_NAME, &value)?;
    return Ok(());
}

pub fn write(config: &types::FullConfig) -> Result<()> {
    let mut bytes = Vec::new();
    serialization::serialize(config, &mut bytes)?;
    write_raw(bytes)?;
    return Ok(());
}

fn read_raw() -> Result<Vec<u8>> {
    let key = open_key()?;
    let value = key.get_raw_value(VALUE_NAME)?;

    match value.vtype {
        REG_BINARY => Ok(value.bytes),
        _ => Err(ErrorKind::InvalidValueType.into()),
    }
}

pub fn read() -> Result<types::FullConfig> {
    let bytes = read_raw()?;
    let conf = serialization::deserialize(&bytes[..])?;
    return Ok(conf);
}
