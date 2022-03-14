# JWT.rs
A simple Json Web Token crate. The crate let's you create, sign, verify and extract data from
JWT tokens. Data is serialized with `serde` and `serde_json`.

# Examples
## Verification
Extract the payload from a JWT token if the token is valid.
```rust
#[derive(serde::Serialize, serde::Deserialize, PartialEq)]
struct TestPayload {
    is_admin: bool,
    name: String,
    age: u8,
}

const SECRET: &'static str = "This is a very secret secret";

let token_str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc19hZG1pbiI6dHJ1ZSwibmFtZSI6IkpvaG4gRG9lIiwiYWdlIjoxOH0.0mV5XVAmarscyZEwl8PoX4vqVn_JCZSVJRsgnSJTo94";
let token = Token::from_str(token_str)?;
// payload is `Ok(Some(<payload>))`
let payload = token.get_if_valid::<TestPayload>(SECRET);
```

## Signing
It's also possible to create and sign a new token:
```rust
const SECRET: &'static str = "This is a very secret secret";

#[derive(serde::Serialize, serde::Deserialize)]
struct TestPayload {
    is_admin: bool,
    name: String,
    age: u8,
}
let payload = TestPayload {
    is_admin: true,
    name: String::from("John Doe"),
    age: 18
};
let token = Token::try_new(Algorithm::HS256, payload, SECRET).unwrap();
println!("{}", token); // Convert the token to a string.
```
