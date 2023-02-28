extern crate base32;
extern crate totp_lite;

pub fn generate(secret: &str) -> Result<String, &'static str> {
  let secret_bytes = (base32::decode(base32::Alphabet::RFC4648 {padding: false}, &secret.replace(" ", "")).ok_or("invalid base32"))?;
  let code: String = totp_lite::totp::<totp_lite::Sha1>(&secret_bytes, 30);
  Ok(format!("{:0>6}", code))
}
