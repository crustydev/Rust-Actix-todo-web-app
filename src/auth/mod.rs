use actix_web::dev::ServiceRequest;
pub mod jwt;
mod processes;


pub fn process_token(request: &ServiceRequest) -> 
                    Result<String, &'static str> {
    match processes::extract_header_token(request) {
        Ok(token) => {
            match processes::check_password(token) {
                Ok(token) => Ok(token),
                Err(message) => Err(message)
            }
        },
        Err(message) => Err(message)
    }
}


#[cfg(test)]
mod processes_test {
    use super::process_token;
    use actix_web::test::TestRequest;
    use super::jwt::JwtToken;
    #[test]
    fn no_token_process_token() {
        let mock_request = TestRequest::default().to_srv_request();

        match process_token(&mock_request) {
            Err(message) => assert_eq!(message, "there is no token"),
            _ => panic!("token should not be present in header")
        }
    }

    #[test]
    fn incorrect_token() {
        let mock_request = TestRequest::with_header(
                            "user-token", "test").to_srv_request();

        match process_token(&mock_request) {
            Err(message) => assert_eq!(message, "Could not decode"),
            _ => panic!("Incorrect token shoud not be decodeable")
        }
    }

    #[test]
    fn correct_token() {
        let encoded_token: String = JwtToken::encode(32);
        let mock_request = TestRequest::with_header(
                            "user-token", encoded_token).to_srv_request();

        match process_token(&mock_request) {
            Ok(token) => assert_eq!(token, "passed"),
            _ => panic!("Correct token present in header should return passed")
        }
    }

}