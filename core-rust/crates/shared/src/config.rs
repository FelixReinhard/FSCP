pub fn get_ip() -> &'static str {
    return "127.0.0.1:9123";
}

pub const CURRENT_VERSION: u8 = 1;

// The size of the RSA KEYS to use.
pub const RSA_KEY_SIZE: usize = 2048;

pub const MAX_CLIENTS: u16 = 32;
