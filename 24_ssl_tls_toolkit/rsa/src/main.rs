// The example below generates an RSA public and private key pair, and
// encrypts the keys with a passphrase. The outputs are text strings
// that can be saved into files. Those files are called PEM (Privacy
// Enhanced Mail) files.

extern crate openssl;

use openssl::rsa::{Padding, Rsa};
use openssl::symm::Cipher;

fn main() {
    let passphrase = "rust";

    let rsa = Rsa::generate(1024).unwrap();
    let private_key: Vec<u8> = rsa
        .private_key_to_pem_passphrase(Cipher::aes_128_cbc(), passphrase.as_bytes())
        .unwrap();
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();

    println!("Private key: {}", String::from_utf8(private_key).unwrap());
    println!("Public key: {}", String::from_utf8(public_key).unwrap());

    // Next, we can import public and private keys from the PEM document.
    // In the example, we demonstrate how to encrypt a byte array of data
    // using the public key. Such encrypted data can only be decrypted by
    // the correspding private key.

    let public_key_pem = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQC6HHN9EGn3rumE4JVPqLLIXyFi
oatUktRqK1ytHOcPROIj0kbUNyPNkvJEEz1qV7XDlusyBkCpOTu75jEgqjKibnuk
NALl4nyPD5XjcdvHLZOXwIKLNlue896JC+MOCXYIH8na3SoFE6KHPAFIDBGmQ8lt
dNPlRm5iTnJBgN4l8QIDAQAB
-----END PUBLIC KEY-----";

    let private_key_pem = "-----BEGIN RSA PRIVATE KEY-----
Proc-Type: 4,ENCRYPTED
DEK-Info: AES-128-CBC,27CA6FE65C5E16E1AEE76B90912F1664

07xb9/yxvEckDTZpmXEAdqS/2vXI4z3KsUzeOCP8SSz9ZDxu8sRL/7Ky+sDS4/jn
K7l14Kvn55/54MEG11YCqfacELLukPWBGLi0FoIlwnfK7YjGvfSD6WcyE9ukqOXJ
M4yW/gtpTflIdTo25Y1W6VYXjx7KQxvPrEOrQVTzpu3uWt4TzTjrkz4l7TilGEkn
KnynyJ2sh/PyVxwnazMpgXGIR520jsFRTnPgjBfDSqgq9IluurebQ3IG7ubQwmHg
0OsRUHLVOprx/ceADv12H7f9nue1MjGk7yrOy2BZkdwPv7jWBpyn5+0hsSgD8pwl
UNUZSWwwh556kkN5uBWLdgEHcJH32jwCI8abHAgMMgWcbqKB366NyMfh54bEKaaO
XOW8Ct9bAehqmyPpqX4fPm8WiKWoXaptiUEZs5xxRzSsVvsBA3DX7LHgKRYPkRuh
BQAYi9JoUqdqyyJGHercM/Ti0JaWFqijquXLK/caORkJx154zYqJfXfYSc4aauzE
mJqUPcvh/m2wBptPCw2nDMfyOyk8jibeiYxtQBTv1nZqy7d21wApByQwL1ss1r+n
NZ8YVAekx4EgdfXpHJClimatahGhnN87Wuo04d7cJ4VK5+lcxa1uUnQiaBIlnXDY
gmwQyTl9v0NvfaM10sLuBRjiC4Ff8LCvt1FtydEnUgXMfXCrT16x8fBrJN/rh/Kk
8i+hVWlcxYWgfdG+mD/wM1HHWWI/Fe29+rZWZ/cdPzdFqBD9aBq3X90ekrf5AXaI
SgWZiAuFYud+RLQGSOVYStI0oUHNjseh9NGOa23hD7l17ksPRMqoAdqE3nstu6Al
-----END RSA PRIVATE KEY-----";

    let data = "A quick brown fox jumps over the lazy dog.";

    // Encrypt with public key
    let rsa = Rsa::public_key_from_pem(public_key_pem.as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa
        .public_encrypt(data.as_bytes(), &mut buf, Padding::PKCS1)
        .unwrap();

    let data = buf;

    // Decrypt with private key
    let rsa =
        Rsa::private_key_from_pem_passphrase(private_key_pem.as_bytes(), passphrase.as_bytes())
            .unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa
        .private_decrypt(&data, &mut buf, Padding::PKCS1)
        .unwrap();
    println!("Decrypted: {}", String::from_utf8(buf).unwrap());
}
