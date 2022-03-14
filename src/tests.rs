#[cfg(test)]
mod tests {
    use crate::*;
    const SECRET: &'static str = "This is a very secret secret";
    const BAD_SECRET: &'static str = "A bad secret";

    #[derive(serde::Serialize, serde::Deserialize)]
    struct TestPayload {
        is_admin: bool,
        name: String,
        age: u8,
    }

    fn token(alg: Algorithm) -> Token {
        let payload = TestPayload {
            is_admin: true,
            name: String::from("John Doe"),
            age: 18,
        };
        Token::try_new(alg, payload, SECRET).expect("Should not panic")
    }

    #[test]
    fn from_str() {
        let token_str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc19hZG1pbiI6dHJ1ZSwibmFtZSI6IkpvaG4gRG9lIiwiYWdlIjoxOH0.0mV5XVAmarscyZEwl8PoX4vqVn_JCZSVJRsgnSJTo94";
        let _token = Token::from_str(token_str);
    }
    #[test]
    fn new_hs512() {
        let token = token(Algorithm::HS512);
        assert_eq!("eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJpc19hZG1pbiI6dHJ1ZSwibmFtZSI6IkpvaG4gRG9lIiwiYWdlIjoxOH0.5d-eNSPgx25mrONPXf8w2IWEPu1nkqNVtLtDI-p8Gd6kkmNFEiwzZlo2DcUZWasQSDefWLRpjiOC0rGqC1-v0Q", token.to_string());
        assert!(token.is_valid(SECRET).expect("Should not panic"));
        assert!(!token.is_valid(BAD_SECRET).expect("Should not panic"));
    }

    #[test]
    fn new_hs256() {
        let token = token(Algorithm::HS256);
        assert_eq!("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc19hZG1pbiI6dHJ1ZSwibmFtZSI6IkpvaG4gRG9lIiwiYWdlIjoxOH0.0mV5XVAmarscyZEwl8PoX4vqVn_JCZSVJRsgnSJTo94", token.to_string());
        assert!(token.is_valid(SECRET).expect("Should not panic"));
    }

    #[test]
    fn bad_secert() {
        let token = token(Algorithm::HS256);
        assert!(!token.is_valid(BAD_SECRET).expect("Should not panic"));
    }

    #[test]
    fn get_content() {
        let token = token(Algorithm::HS256);
        let payload = token
            .get_if_valid::<TestPayload>(SECRET)
            .expect("Should not panic")
            .expect("Should have data");
        assert_eq!(String::from("John Doe"), payload.name);
    }

    #[test]
    fn get_content_error() {
        let token = token(Algorithm::HS256);
        let payload = token
            .get_if_valid::<TestPayload>(BAD_SECRET)
            .expect("Should not panic");
        assert!(payload.is_none());
    }
}
