use jsonwebtoken::{decode, TokenData, errors::Error as JwtError, Validation, Algorithm, DecodingKey};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket::http::Status;
use jsonwebtoken::errors::ErrorKind;

/// JWT

type AuthClaims = ();

pub struct AuthToken(TokenData<AuthClaims>);

#[derive(Debug)]
pub enum AuthError {
    JwtError(JwtError),
    MalformedRequest(String)
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthToken {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let validation = Validation {
            algorithms: vec![Algorithm::HS512],
            ..Validation::default()
        };
        if let Some(auth) = request.headers().get("authorization").nth(0) {
            let secret = std::env::var("HMAC_SECRET").ok();
            if secret.is_none() {
                return Outcome::Forward(())
            }

            match auth.splitn(2, " ").collect::<Vec<&str>>().as_slice() {
                [prefix, token] if prefix.to_lowercase() == "bearer" =>
                    match decode::<AuthClaims>(&token, &DecodingKey::from_secret(secret.unwrap().as_ref()), &validation) {
                        Ok(data) => Outcome::Success(AuthToken(data)),
                        // Err(e) => Outcome::Failure((Status::Unauthorized, AuthError::JwtError(e)))
                        Err(_) => Outcome::Forward(())
                    }

                _ => Outcome::Failure((
                    Status::Unauthorized,
                    AuthError::MalformedRequest("Authorization value must be of the form 'Bearer <TOKEN>'.".into())
                ))
            }
        } else {
            Outcome::Forward(())
            // Outcome::Failure((
            //     Status::BadRequest,
            //     AuthError::MalformedRequest("Expected an 'authorization' field, but none found".into())
            // ))
        }
    }
}