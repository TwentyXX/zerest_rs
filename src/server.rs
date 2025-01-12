use axum::{
    routing::get,
    Router,
};

/// ルートパスへのGETリクエストに対するハンドラー
async fn root() -> &'static str {
    "Hello, World!"
}

/// サーバーを起動する関数
pub async fn run_server() -> color_eyre::Result<()> {
    // ルーターの設定
    let app = Router::new()
        .route("/", get(root));

    // サーバーのアドレスを設定
    let addr = "127.0.0.1:3000";
    println!("Server running on http://{}", addr);

    // サーバーを起動
    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}