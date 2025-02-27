use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{Html, IntoResponse};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::{CookieJar};
use axum_macros::debug_handler;
use uuid::Uuid;
use crate::api::dto::{AuthorizeRequest, AuthorizeResponse, FieldValue};
use crate::AppState;
use crate::workflow::arena::{Arena, ArenaState};
use crate::workflow::call_phone::CallPhone;
use crate::workflow::captcha::Captcha;
use crate::workflow::credential::Credential;
use crate::workflow::node::NodeStatus;
use cookie::time::{Duration, OffsetDateTime};

pub async fn index() -> impl IntoResponse {
    Html(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Authorize</title>
        </head>
        <body>
            <form id="authorizeForm">
                <button type="submit">Authorize</button>
                <input type="text" name="fields" placeholder="Field 1">
            </form>

            <script>
                document.getElementById('authorizeForm').addEventListener('submit', async function(event) {
                    event.preventDefault();

                    const formData = new FormData(event.target);
                    const data = {'fields': []};
                    /*formData.forEach((value, key) => {
                        data[key] = value;
                    });*/

                    const response = await fetch('/authorize', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json'
                        },
                        body: JSON.stringify(data)
                    });

                    const result = await response.json();
                    console.log(result);
                });
            </script>
        </body>
        </html>
    "#)
}

#[debug_handler]
pub async fn authorize(State(state) : State<AppState>,  jar: CookieJar, Json(payload): Json<AuthorizeRequest>) -> (CookieJar, Json<AuthorizeResponse>) {
    match jar.get("sth") {
        Some(cookie) => {
            println!("cookie obtained");
            let mut arena: Arena = serde_json::from_str(cookie.value()).unwrap();


            payload.fields.iter().for_each(|field| {
                arena.state.data.insert(field.name.clone(), field.value.as_string());
            });

            // arena.start(arena.state.data.get(crate::workflow::arena::CURRENT_NODE).unwrap().parse().unwrap());
            let serialized = serde_json::to_string(&arena).unwrap();
            let mut response = AuthorizeResponse::new();
            (
                jar.add(Cookie::new("sth", serialized)),
                Json(response),
            )
        }
        None => {
            println!("cookie unset");
            let arena = &create_arena();

            let serialized = serde_json::to_string(arena).unwrap();
            // println!("serialized = {}", serialized);

            let code = arena.state.data.get("captcha_code").unwrap();
            let mut response = AuthorizeResponse::new();
            response.add_field("captcha_code".to_string(), true, FieldValue::Info(code.to_string()));
            response.add_field("captcha_code_value".to_string(), true, FieldValue::String("".to_string()));

            let mut now = OffsetDateTime::now_utc();
            now += Duration::weeks(52);
            let mut cookie1 = Cookie::build(("sth", serialized))
                .secure(true)
                .same_site(SameSite::None)
                .path("/")
                .domain("localhost")
                .http_only(false)
                .expires( now)
                .build();

            (
                jar.add(cookie1),
                Json(response),
                // Html(format!(r#"
                //     <form action="/authorize" method="post">
                //         <label for="captcha_code">Captcha Code: {}</label>
                //         <input type="text" id="captcha_code" name="captcha_code" required>
                //         <button type="submit">Submit</button>
                //     </form>
                // "#, code)),
            )
        }
    }
}

pub async fn decode_cookie(jar: CookieJar) -> Result<String, StatusCode> {
    if let Some(cookie) = jar.get("sth") {
        Ok(cookie.value().to_string())
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

fn create_arena()  -> Arena{
    let mut arena = Arena::new(ArenaState::new(Uuid::new_v4()));

    let credential_id = arena.add_node(
        Box::new(Credential::new("admin".to_string())),
    );
    let captcha_id = arena.add_node(Box::new(Captcha::new(15)));
    let call_id = arena.add_node(
        Box::new(CallPhone::new("+33123456678".to_string())),
    );

    arena.nodes[credential_id].add_child(NodeStatus::Success, captcha_id);
    arena.nodes[captcha_id].add_child(NodeStatus::Success, call_id);
    arena.start(0);
    arena
}