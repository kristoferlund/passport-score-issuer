use candid::Principal;
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use std::fmt;
use tiny_keccak::{Hasher, Keccak};

#[derive(Debug)]
pub enum EthError {
    AddressFormatError(String),
    DecodingError(hex::FromHexError),
    SignatureFormatError(String),
    InvalidSignature,
    InvalidRecoveryId,
    PublicKeyRecoveryFailure,
    Eip55Error(String),
}

impl From<hex::FromHexError> for EthError {
    fn from(err: hex::FromHexError) -> Self {
        EthError::DecodingError(err)
    }
}

impl fmt::Display for EthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EthError::AddressFormatError(e) => write!(f, "Address format error: {}", e),
            EthError::DecodingError(e) => write!(f, "Decoding error: {}", e),
            EthError::SignatureFormatError(e) => write!(f, "Signature format error: {}", e),
            EthError::InvalidSignature => write!(f, "Invalid signature"),
            EthError::InvalidRecoveryId => write!(f, "Invalid recovery ID"),
            EthError::PublicKeyRecoveryFailure => {
                write!(f, "Public key recovery failure")
            }
            EthError::Eip55Error(e) => write!(f, "EIP-55 error: {}", e),
        }
    }
}

impl From<EthError> for String {
    fn from(error: EthError) -> Self {
        error.to_string()
    }
}

/// Represents an Ethereum address with validation.
///
/// This struct ensures that the contained Ethereum address string is valid according to Ethereum standards.
/// It checks for correct length, hex encoding, and EIP-55 encoding.
#[derive(Debug)]
pub struct EthAddress(String);

impl EthAddress {
    /// Creates a new `EthAddress` after validating the Ethereum address format and encoding.
    ///
    /// The address must start with '0x', be 42 characters long, and comply with EIP-55 encoding.
    ///
    /// # Arguments
    /// * `address` - A string slice representing the Ethereum address.
    pub fn new(address: &str) -> Result<EthAddress, EthError> {
        if !address.starts_with("0x") || address.len() != 42 {
            return Err(EthError::AddressFormatError(String::from(
                "Must start with '0x' and be 42 characters long",
            )));
        }

        hex::decode(&address[2..]).map_err(EthError::DecodingError)?;

        if address != convert_to_eip55(address).unwrap() {
            return Err(EthError::Eip55Error(String::from("Not EIP-55 encoded")));
        }

        Ok(EthAddress(address.to_owned()))
    }

    /// Returns a string slice of the Ethereum address.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts the Ethereum address into a byte vector.
    pub fn _as_bytes(&self) -> Vec<u8> {
        let address = self.0.strip_prefix("0x").unwrap();
        hex::decode(address).unwrap()
    }

    /// Converts the Ethereum address into a byte array.
    pub fn as_byte_array(&self) -> [u8; 20] {
        let address = self.0.strip_prefix("0x").unwrap();
        let bytes = hex::decode(address).unwrap();
        let mut array = [0; 20];
        array.copy_from_slice(&bytes);
        array
    }

    pub fn as_hash(&self) -> [u8; 32] {
        let bytes = self.as_byte_array();
        let mut keccak256 = [0; 32];
        let mut hasher = Keccak::v256();
        hasher.update(&bytes);
        hasher.finalize(&mut keccak256);
        keccak256
    }
}

/// Represents an Ethereum signature with validation.
///
/// This struct ensures that the contained Ethereum signature string is valid.
/// It checks for correct length and hex encoding.
#[derive(Debug)]
pub struct EthSignature(String);

impl EthSignature {
    /// Creates a new `EthSignature` after validating the Ethereum signature format.
    ///
    /// The signature must start with '0x' and be 132 characters long.
    ///
    /// # Arguments
    /// * `signature` - A string slice representing the Ethereum signature.
    pub fn new(signature: &str) -> Result<EthSignature, EthError> {
        if !signature.starts_with("0x") || signature.len() != 132 {
            return Err(EthError::SignatureFormatError(String::from(
                "Must start with '0x' and be 132 characters long",
            )));
        }

        hex::decode(&signature[2..]).map_err(EthError::DecodingError)?;
        Ok(EthSignature(signature.to_owned()))
    }

    /// Returns a string slice of the Ethereum signature.
    pub fn _as_str(&self) -> &str {
        &self.0
    }

    /// Converts the Ethereum signature into a byte vector.
    pub fn as_bytes(&self) -> Vec<u8> {
        let signature = self.0.strip_prefix("0x").unwrap();
        hex::decode(signature).unwrap()
    }

    /// Converts the Ethereum signature into a byte array.
    pub fn _as_byte_array(&self) -> [u8; 65] {
        let signature = self.0.strip_prefix("0x").unwrap();
        let bytes = hex::decode(signature).unwrap();
        let mut array = [0; 65];
        array.copy_from_slice(&bytes);
        array
    }
}

/// Recovers an Ethereum address from a given message and signature.
///
/// # Parameters
///
/// * `message` - The message that was signed.
/// * `signature` - The hex-encoded signature.
///
/// # Returns
///
/// The recovered Ethereum address if successful, or an error.
pub fn recover_eth_address(message: &str, signature: &EthSignature) -> Result<String, EthError> {
    let message_hash = eip191_hash(message);
    let signature_bytes = signature.as_bytes();

    let recovery_id =
        RecoveryId::try_from(signature_bytes[64] % 27).map_err(|_| EthError::InvalidRecoveryId)?;

    let signature =
        Signature::from_slice(&signature_bytes[..64]).map_err(|_| EthError::InvalidSignature)?;

    let verifying_key = VerifyingKey::recover_from_prehash(&message_hash, &signature, recovery_id)
        .map_err(|_| EthError::PublicKeyRecoveryFailure)?;

    let address = derive_eth_address_from_public_key(&verifying_key)?;

    Ok(address)
}

/// Hashes a message using the EIP-191 standard. See [EIP-191 spec](https://eips.ethereum.org/EIPS/eip-191) for
/// more information.
///
/// # Parameters
///
/// * `message` - The message to hash.
///
/// # Returns
///
/// A 32-byte array containing the hash.
pub fn eip191_hash(message: &str) -> [u8; 32] {
    let mut keccak256 = [0; 32];
    let mut hasher = Keccak::v256();
    hasher.update(&eip191_bytes(message));
    hasher.finalize(&mut keccak256);

    keccak256
}

/// Formats a message according to the EIP-191 standard. See [EIP-191 spec](https://eips.ethereum.org/EIPS/eip-191) for
/// for more information.
///
/// # Parameters
///
/// * `message` - The message to format.
///
/// # Returns
///
/// A vector of bytes containing the formatted message.
pub fn eip191_bytes(message: &str) -> Vec<u8> {
    format!("\x19Ethereum Signed Message:\n{}{}", message.len(), message).into_bytes()
}

/// Derives an Ethereum address from an ECDSA public key.
///
/// # Parameters
///
/// * `key` - The ECDSA public key to derive the address from.
///
/// # Returns
///
/// The derived Ethereum address if successful, or an error.
pub fn derive_eth_address_from_public_key(key: &VerifyingKey) -> Result<String, EthError> {
    let mut keccak256 = [0; 32];
    let mut hasher = Keccak::v256();
    hasher.update(&key.to_encoded_point(false).as_bytes()[1..]);
    hasher.finalize(&mut keccak256);

    let keccak256_hex = hex::encode(keccak256);
    convert_to_eip55(&keccak256_hex[24..])
}

/// Converts an Ethereum address to EIP-55 format. See [EIP-55 spec](https://eips.ethereum.org/EIPS/eip-55) for
/// more information.
///
/// # Parameters
///
/// * `address` - The Ethereum address to convert.
///
/// # Returns
///
/// The EIP-55-compliant Ethereum address if successful, or an error.
pub fn convert_to_eip55(address: &str) -> Result<String, EthError> {
    let address_trimmed = if address.starts_with("0x") {
        address.strip_prefix("0x").unwrap()
    } else {
        address
    };

    let address_lowercase = address_trimmed.to_lowercase();

    // Compute Keccak-256 hash of the lowercase address
    let mut hash = [0; 32];
    let mut hasher = Keccak::v256();
    hasher.update(address_lowercase.as_bytes());
    hasher.finalize(&mut hash);

    // Iterate over each character in the original address
    let checksummed_addr = address_trimmed
        .char_indices()
        .map(|(i, c)| {
            let result = match c {
                '0'..='9' => c.to_string(), // Keep digits as is
                'a'..='f' | 'A'..='F' => {
                    // Extract the corresponding nibble from the hash
                    let hash_nibble = if i % 2 == 0 {
                        (hash[i / 2] >> 4) & 0x0f
                    } else {
                        hash[i / 2] & 0x0f
                    };

                    // Uppercase if the nibble is 8 or more
                    if hash_nibble >= 8 {
                        c.to_ascii_uppercase().to_string()
                    } else {
                        c.to_ascii_lowercase().to_string()
                    }
                }
                _ => {
                    return Err(format!(
                        "Unrecognized hex character '{}' at position {}",
                        c, i
                    ));
                }
            };
            Ok(result)
        })
        .collect::<Result<String, String>>()
        .map_err(EthError::Eip55Error)?; // Convert to error type

    Ok(format!("0x{}", checksummed_addr))
}

///
/// Create a message to sign to link an Ethereum address to an Internet Computer principal.
///
pub fn create_signing_message(address: &EthAddress, principal: &Principal) -> String {
    format!(
        "Sign this message to link your Ethereum address to your Internet Computer identity.\n\nEthereum address: {}\n\nInternet Computer principal: {}",
        address.as_str(),
        principal.to_text())
}
