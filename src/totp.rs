extern crate base32;
extern crate totp_lite;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate(secret: &str) -> Result<String, &'static str> {
  let secret_bytes = (base32::decode(base32::Alphabet::RFC4648 {padding: false}, &secret.replace(" ", "")).ok_or("invalid base32"))?;
  let seconds: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
  let code: String = totp_lite::totp_custom::<totp_lite::Sha1>(totp_lite::DEFAULT_STEP, 6, &secret_bytes, seconds);
  Ok(format!("{:0>6}", code))
}
