use rosenpass_to::ops::copy_slice;
use rosenpass_to::To;

use zeroize::Zeroize;

/// The key length is 32 bytes or 256 bits.
pub const KEY_LEN: usize = 32; // Grrrr! Libcrux, please provide me these constants.
/// The  MAC tag length is 16 bytes or 128 bits.
pub const TAG_LEN: usize = 16;
/// The nonce length is 12 bytes or 96 bits.
pub const NONCE_LEN: usize = 12;

/// Encrypts using ChaCha20Poly1305 as implemented in [libcrux](https://github.com/cryspen/libcrux).
/// Key and nonce MUST be chosen (pseudo-)randomly. The `key` slice MUST have a length of
/// [KEY_LEN]. The `nonce` slice MUST have a length of [NONCE_LEN]. The last [TAG_LEN] bytes
/// written in `ciphertext` are the tag guaranteeing integrity. `ciphertext` MUST have a capacity of
/// `plaintext.len()` + [TAG_LEN].
///  
/// # Examples
///```rust
/// # use rosenpass_ciphers::subtle::chacha20poly1305_ietf_libcrux::{encrypt, TAG_LEN, KEY_LEN, NONCE_LEN};
///
/// const PLAINTEXT_LEN: usize = 43;
/// let plaintext = "post-quantum cryptography is very important".as_bytes();
/// assert_eq!(PLAINTEXT_LEN, plaintext.len());
/// let key: &[u8] = &[0u8; KEY_LEN]; // THIS IS NOT A SECURE KEY
/// let nonce: &[u8] = &[0u8; NONCE_LEN]; // THIS IS NOT A SECURE NONCE
/// let additional_data: &[u8] = "the encrypted message is very important".as_bytes();
/// let mut ciphertext_buffer = [0u8; PLAINTEXT_LEN + TAG_LEN];
///
/// let res: anyhow::Result<()> = encrypt(&mut ciphertext_buffer, key, nonce, additional_data, plaintext);
/// assert!(res.is_ok());
/// # let expected_ciphertext: &[u8] = &[239, 104, 148, 202, 120, 32, 77, 27, 246, 206, 226, 17,
/// # 83, 78, 122, 116, 187, 123, 70, 199, 58, 130, 21, 1, 107, 230, 58, 77, 18, 152, 31, 159, 80,
/// # 151, 72, 27, 236, 137, 60, 55, 180, 31, 71, 97, 199, 12, 60, 155, 70, 221, 225, 110, 132, 191,
/// # 8, 114, 85, 4, 25];
/// # assert_eq!(expected_ciphertext, &ciphertext_buffer);
///```
///
#[inline]
pub fn encrypt(
    ciphertext: &mut [u8],
    key: &[u8],
    nonce: &[u8],
    ad: &[u8],
    plaintext: &[u8],
) -> anyhow::Result<()> {
    let (ciphertext, mac) = ciphertext.split_at_mut(ciphertext.len() - TAG_LEN);

    use libcrux::aead as C;
    let crux_key = C::Key::Chacha20Poly1305(C::Chacha20Key(key.try_into().unwrap()));
    let crux_iv = C::Iv(nonce.try_into().unwrap());

    copy_slice(plaintext).to(ciphertext);
    let crux_tag = libcrux::aead::encrypt(&crux_key, ciphertext, crux_iv, ad).unwrap();
    copy_slice(crux_tag.as_ref()).to(mac);

    match crux_key {
        C::Key::Chacha20Poly1305(mut k) => k.0.zeroize(),
        _ => panic!(),
    }

    Ok(())
}

/// Decrypts a `ciphertext` and verifies the integrity of the `ciphertext` and the additional data
/// `ad`. using ChaCha20Poly1305 as implemented in [libcrux](https://github.com/cryspen/libcrux).
///
/// The `key` slice MUST have a length of [KEY_LEN]. The `nonce` slice MUST have a length of
/// [NONCE_LEN]. The plaintext buffer must have a capacity of `ciphertext.len()` - [TAG_LEN].
///
/// # Examples
///```rust
/// # use rosenpass_ciphers::subtle::chacha20poly1305_ietf_libcrux::{decrypt, TAG_LEN, KEY_LEN, NONCE_LEN};
/// let ciphertext: &[u8] = &[239, 104, 148, 202, 120, 32, 77, 27, 246, 206, 226, 17,
/// 83, 78, 122, 116, 187, 123, 70, 199, 58, 130, 21, 1, 107, 230, 58, 77, 18, 152, 31, 159, 80,
/// 151, 72, 27, 236, 137, 60, 55, 180, 31, 71, 97, 199, 12, 60, 155, 70, 221, 225, 110, 132, 191,
/// 8, 114, 85, 4, 25]; // this is the ciphertext generated by the example for the encryption
/// const PLAINTEXT_LEN: usize = 43;
/// assert_eq!(PLAINTEXT_LEN + TAG_LEN, ciphertext.len());
///
/// let key: &[u8] = &[0u8; KEY_LEN]; // THIS IS NOT A SECURE KEY
/// let nonce: &[u8] = &[0u8; NONCE_LEN]; // THIS IS NOT A SECURE NONCE
/// let additional_data: &[u8] = "the encrypted message is very important".as_bytes();
/// let mut plaintext_buffer = [0u8; PLAINTEXT_LEN];
///
/// let res: anyhow::Result<()> = decrypt(&mut plaintext_buffer, key, nonce, additional_data, ciphertext);
/// assert!(res.is_ok());
/// let expected_plaintext = "post-quantum cryptography is very important".as_bytes();
/// assert_eq!(expected_plaintext, plaintext_buffer);
///
///```
#[inline]
pub fn decrypt(
    plaintext: &mut [u8],
    key: &[u8],
    nonce: &[u8],
    ad: &[u8],
    ciphertext: &[u8],
) -> anyhow::Result<()> {
    let (ciphertext, mac) = ciphertext.split_at(ciphertext.len() - TAG_LEN);

    use libcrux::aead as C;
    let crux_key = C::Key::Chacha20Poly1305(C::Chacha20Key(key.try_into().unwrap()));
    let crux_iv = C::Iv(nonce.try_into().unwrap());
    let crux_tag = C::Tag::from_slice(mac).unwrap();

    copy_slice(ciphertext).to(plaintext);
    libcrux::aead::decrypt(&crux_key, plaintext, crux_iv, ad, &crux_tag).unwrap();

    match crux_key {
        C::Key::Chacha20Poly1305(mut k) => k.0.zeroize(),
        _ => panic!(),
    }

    Ok(())
}
