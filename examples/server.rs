use axum::response::Html;
use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

async fn root() -> Html<&'static str> {
    Html(
        "<html>
    <head>
    <script type=\"text/javascript\">
    const images = [ ];
    setInterval(() => {
    document.querySelector(\"img\").src = images.pop();
    console.log('logging');
    }, 1000);
    </script>
    </head>
    <body>
        <img src=\"\"/>
    </body>
    </html>",
    )
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
