use rand::{distributions::Alphanumeric, Rng};
use std::str::FromStr;
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .post_async("/create", |mut req, ctx| async move {
            // get request payload
            let request_payload = req.json::<LinkCreateRequestPayload>().await?;

            // random link_key
            let link_key = rand::thread_rng()
                .sample_iter(Alphanumeric)
                .take(7)
                .map(char::from)
                .collect::<String>();

            // get kv store
            let kv = ctx.kv("tp")?;

            // put link_key to kv
            kv.put(&link_key, &request_payload.url)?.execute().await?;

            // build response
            let tp_url = format!("/goto/{}", link_key);
            Response::ok(tp_url)
        })
        .get_async("/goto/:link_key", |_req, ctx| async move {
            // get link_key
            match ctx.param("link_key") {
                None => Response::error("Bad Request. No link_key.", 400),
                Some(link_key) => {
                    // get kv store
                    let kv = ctx.kv("tp")?;
                    // get real url
                    match kv.get(link_key).text().await? {
                        None => Response::error("Bad Request. Invalid link_key.", 400),
                        Some(real_url) => {
                            // redirect
                            let real_url = Url::from_str(real_url.as_str())?;
                            Response::redirect(real_url)
                        }
                    }
                }
            }
        })
        .run(req, env)
        .await
}

#[derive(serde::Deserialize)]
struct LinkCreateRequestPayload {
    url: String,
}
