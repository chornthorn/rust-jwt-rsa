use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Utc, TimeDelta};


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}


fn main() {
    // Load RSA keys that located in the assets folder
    let private_key = include_bytes!("../assets/private_key.pem");
    let public_key = include_bytes!("../assets/public_key.pem");

    // Create a claims object
    let my_claims = Claims {
        sub: "user123".to_owned(),
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + TimeDelta::try_minutes(60).unwrap()).timestamp() as usize,
    };

    // Encode the JWT with 1 hour expiration
    let header = Header::new(Algorithm::RS512);
    let encoding_key = EncodingKey::from_rsa_pem(private_key).unwrap();
    let token = encode(&header, &my_claims, &encoding_key).unwrap();
    println!("Token: {}", token);

    println!("-----------------------------");

    // Decode the JWT
    let decoding_key = DecodingKey::from_rsa_pem(public_key).unwrap();
    let token_data = decode::<Claims>(&token, &decoding_key, &Validation::new(Algorithm::RS512)).unwrap();
    println!("Decoded Token: {:?}", token_data.claims);
}
