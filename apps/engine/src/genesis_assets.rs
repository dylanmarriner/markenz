use blake3;

pub const MARKENZ_GENESIS_SEED: u64 = 0x1337_MARKENZ;

pub const GEM_D_IDENTITY_FINGERPRINT: &[u8] = blake3::hash(b"Gem-D").as_bytes();
pub const GEM_K_IDENTITY_FINGERPRINT: &[u8] = blake3::hash(b"Gem-K").as_bytes();
