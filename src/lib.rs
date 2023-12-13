use worker::*;

mod image_upload;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        .post_async("/upload", |req, ctx| async move {
            image_upload::handle(req, ctx).await
        })
        .run(req, env)
        .await
}
