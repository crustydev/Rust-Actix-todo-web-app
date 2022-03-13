extern crate hmac;
extern crate jwt;
extern crate sha2;

use hmac::{Hmac, NewMac};
use jwt::{Header, Token, VerifyWithKey};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

//represents encoded user details
pub struct JwtToken {
    pub user_id: i32,
    pub body: String
}

impl JwtToken{
    //lets us encode the user_id into a token
    pub fn encode(user_id: i32) -> String {
        //create signature using secret key
        let key: Hmac<Sha256> =
        Hmac::new_varkey(b"secret").unwrap();
        let mut claims = BTreeMap::new();
        //insert user_id into claims
        claims.insert("user_id", user_id);
        //encode claims into a string and return
        let token_str: String = claims.sign_with_key(&key).unwrap();
        return token_str;
    }

    //gets the data it needs passes the encoded string as a struct
    pub fn decode(encoded_token: String) -> 
                        Result<JwtToken, &'static str> {
        //create signature
        let key: Hmac<Sha256> = Hmac::new_varkey(
                                        b"secret").unwrap();
        let token_str: &str = encoded_token.as_str();
        
        let token: Result<Token<Header, BTreeMap<String,
            i32>, _>, _> = VerifyWithKey::verify_with_key(
            token_str, &key);

        match token {
            Ok(token) => {
                let _header = token.header();
                let claims = token.claims();
                return Ok(JwtToken {
                    user_id: claims["user_id"],
                    body: encoded_token
                })
            }
            Err(_) =>  return Err("Could not decode")
        }
    } 
}

