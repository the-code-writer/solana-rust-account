use anchor_lang::prelude::*;
use bs58::decode;

pub fn public_key(input: &str) -> bool {
    // Check if the input is a valid Solana public key (44 characters, base58-encoded)
    if input.len() == 44 && decode(input).is_ok() {
        return true;
    }
    false
}

pub fn address(input: &str) -> bool {
    // Check if the input is a valid Solana wallet address (32 bytes, base58-encoded)
    if input.len() == 32 && decode(input).is_ok() {
        return true;
    }
    false
}
