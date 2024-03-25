# Rust API JWT Authentication

This project is a simple demonstration of creating and decoding JSON Web Tokens (JWTs) using the Rust language. The utilized crates are `jsonwebtoken`, `serde`, `chrono`.

## Code Overview

The provided code initiates a new instance of JWT Claims, encodes it to a token and then decodes back, to ensure the validity and functionality of the JWTs.

## Code Structure

- The `Claims` structure is a Rust representation of the JWT Claims object. It includes the subject (sub), issued at time (iat), and expiration time (exp).

- In the main function, RSA keys are loaded from the `/assets/` directory.

- A `Claims` object is initialized with `sub` set to `user123`, `iat` to the current timestamp, and `exp` to one hour from the current time.

- The token is then encoded using the private RSA key with the `RS512` algorithm.

-  The token is then decoded using the public RSA key, to validate its authenticity and the Claims are printed.

## Prerequisites

This application uses `jsonwebtoken`, `serde`, `chrono` libraries of Rust. Please make sure your project has these dependencies in your `Cargo.toml` file. 

```toml
jsonwebtoken = "8.0.1" 
serde = { version = "1.0", features = ["derive"] } 
chrono = "0.4.35"
```

## Running the code

Ensure you have the correct version of Rust installed in your machine. This can be done as follows:

```bash
rustup update
```
Open a terminal, navigate to the project root directory, and run:
```bash
cargo run
```

### Note
Ensure that you replace the `private_key.pem`, and `public_key.pem` paths to your existing RSA keys as required.

## Output

The code prints out the encoded token and decoded claims to the console.

Please note that tokens are sensitive information and should be protected properly in a real-world application.

That's all! You have a simple Rust program to create and decode JWTs. Happy coding!
