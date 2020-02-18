use ::base32::Alphabet::RFC4648;

use crate::models::network::*;
use crate::models::utils::is_hex;
use crate::Result;

const PREFIX_MIJIN: char = 'M';
const PREFIX_MIJIN_TEST: char = 'S';
const PREFIX_PUBLIC: char = 'X';
const PREFIX_PUBLIC_TEST: char = 'V';
const PREFIX_PRIVATE: char = 'Z';
const PREFIX_PRIVATE_TEST: char = 'W';

const EMPTY_STRING: &str = "";
const REGEX_DASH: &str = "-";

/// The `Address` structure describes an address with its network.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// The address in hexadecimal.
    pub address: String,
    pub network_type: NetworkType,
}

impl Address {
    /// Creates an `Address` from a given public_key string for the given `NetworkType`.
    pub fn from_public_key(public_key: &str, network_type: NetworkType) -> Address {
        let _address = super::public_key_to_address(public_key, network_type);

        Address {
            address: _address,
            network_type,
        }
    }

    /// Creates an `Address` from a given `raw_address` string.
    ///
    /// A raw address string looks like:
    /// VAWOEOWTABXR7O3ZAK2XNA5GIBNE6PZIXDAFDWBU or VAWOEO-WTABXR-7O3ZAK-2XNA5G-IBNE6P-ZIXDAF-DWBU
    pub fn from_raw(raw_address: &str) -> Result<Address> {
        ensure!(
            !raw_address.is_empty(),
            "raw address is empty."
         );

        let _address = raw_address.trim().to_uppercase().replace(REGEX_DASH, EMPTY_STRING);
        ensure!(
            _address.len() == 40,
            "Invalid len raw address."
         );

        match _address.chars().next().unwrap() {
            PREFIX_MIJIN => Ok(Address { address: _address, network_type: MIJIN }),
            PREFIX_MIJIN_TEST => Ok(Address { address: _address, network_type: MIJIN_TEST }),
            PREFIX_PUBLIC => Ok(Address { address: _address, network_type: PUBLIC }),
            PREFIX_PUBLIC_TEST => Ok(Address { address: _address, network_type: PUBLIC_TEST }),
            PREFIX_PRIVATE => Ok(Address { address: _address, network_type: PRIVATE }),
            PREFIX_PRIVATE_TEST => Ok(Address { address: _address, network_type: PRIVATE_TEST }),
            _ => Err(format_err!("Wrong address"))
        }
    }

    /// Create an `Address` from the given encoded address.
    pub fn from_encoded(encoded: &str) -> Result<Address> {
        ensure!(
            !encoded.is_empty(),
            "address encoded string is empty."
         );

        ensure!(
            encoded.len() == 50,
            "Invalid len address encoded string."
         );

        ensure!(
            is_hex(encoded),
            "Invalid hex address encoded string."
            );

        let _encoded_to_bytes = hex::decode(encoded)?;

        let _address = base32::encode(RFC4648 { padding: true }, _encoded_to_bytes.as_slice());

        self::Address::from_raw(_address.as_str())
    }

    /// Converts an `Address` String into a more readable/pretty format.
    ///
    /// Before: VAWOEOWTABXR7O3ZAK2XNA5GIBNE6PZIXDAFDWBU
    /// After: VAWOEO-WTABXR-7O3ZAK-2XNA5G-IBNE6P-ZIXDAF-DWBU
    pub fn prettify(&self) -> String {
        let mut res: String = String::new();

        for i in 0..6 {
            res += &self.address[i * 6..i * 6 + 6];
            res.push('-');
        };

        res += &self.address[&self.address.len() - 4..];
        return res;
    }

    pub fn is_empty(&self) -> bool {
        self.address.is_empty() && self.network_type == NOT_SUPPORTED_NET
    }

    }

impl core::fmt::Display for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}
