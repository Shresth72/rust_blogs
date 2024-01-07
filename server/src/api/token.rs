use actix_web::{web::Form, post, web::Json};
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl,
    PkceCodeVerifier,
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use serde::{Serialize, Deserialize};
use log::{info, error};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenBody {
    code: String,
    grant_type: String,
    code_verifier: String,
    redirect_uri: String,
}

#[post("/token")]
pub async fn token(
    body: Form<TokenBody>,
) -> Json<oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>> {
    let req = body.into_inner();
    println!("req: {:?}", req);

    let client = BasicClient::new(
        ClientId::new("604tk757p8f5b61m4n7od2fj48".to_string()),
        Some(ClientSecret::new("iilb4afcncfg4a9fc18kmaoicn5hpm3t6j7e7eamjmfi06rhgv3".to_string())),
        AuthUrl::new("http://auth".to_string()).expect("Invalid auth url"),
        Some(TokenUrl::new("http://auth/token".to_string()).expect("Invalid token url"))
    ).set_redirect_uri(RedirectUrl::new(req.redirect_uri)).expect("Invalid redirect url");

    let pkce_verifier = PkceCodeVerifier::new(req.code_verifier);
    let token_result = client.exchange_code(AuthorizationCode::new(req.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await;

    match token_result {
        Ok(token) => {
            info!("Token: {:?}", token);
            Json(token)
        },
        Err(err) => {
            error!("Error: {:?}", err);
            Json(err)
        },
    }
}