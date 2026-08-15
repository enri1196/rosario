use std::{env, path::PathBuf};

use axum::response::Response as AxumResponse;
use axum::routing::get;
use axum::Router;
use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::IntoResponse,
};
use leptos::logging::log;
use leptos::prelude::*;
use tower::ServiceExt;
use tower_http::services::ServeDir;

const BASE_PATH_ENV: &str = "ROSARIO_BASE_PATH";
const GENERATE_SITE_ARG: &str = "--generate-site";

/// Returns a normalized URL prefix for static hosting below a domain root.
fn deployment_base_path() -> String {
    env::var(BASE_PATH_ENV)
        .map(|path| normalize_base_path(&path))
        .unwrap_or_default()
}

/// Normalizes an optional hosting path to either empty or `/path` form.
fn normalize_base_path(path: &str) -> String {
    let path = path.trim().trim_matches('/');
    if path.is_empty() {
        String::new()
    } else {
        format!("/{path}")
    }
}

/// Joins a deployment base path to one application asset path.
fn asset_url(base_path: &str, asset: &str) -> String {
    format!("{base_path}/{}", asset.trim_start_matches('/'))
}

/// Renders the browser shell for either root hosting or a static base path.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    /// Resolves browser-only theme inputs before CSS loads so the
    /// client-mounted application does not flash the wrong theme.
    const THEME_BOOTSTRAP: &str = r#"(() => {
        let theme;
        try {
            const saved = localStorage.getItem("rosary-theme");
            if (saved === "dark" || saved === "light") theme = saved;
        } catch (_) {}
        if (!theme) {
            try {
                theme = matchMedia("(prefers-color-scheme: light)").matches ? "light" : "dark";
            } catch (_) {
                theme = "dark";
            }
        }
        document.documentElement.dataset.theme = theme;
    })();"#;

    let base_path = deployment_base_path();
    let favicon_ico = asset_url(&base_path, "favicon.ico");
    let favicon_png = asset_url(&base_path, "favicon.png");
    let apple_touch_icon = asset_url(&base_path, "icons/rosary-192.png");
    let manifest = asset_url(&base_path, "manifest.webmanifest");
    let stylesheet = asset_url(&base_path, "pkg/rosary.css");

    view! {
        <!DOCTYPE html>
        <html lang="it" data-theme="dark">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="theme-color" content="#0a0d14" media="(prefers-color-scheme: dark)"/>
                <meta name="theme-color" content="#f4f0e7" media="(prefers-color-scheme: light)"/>
                <link rel="icon" href=favicon_ico sizes="any"/>
                <link rel="icon" href=favicon_png type="image/png"/>
                <link rel="apple-touch-icon" href=apple_touch_icon sizes="192x192"/>
                <link rel="manifest" href=manifest/>
                <script>{THEME_BOOTSTRAP}</script>
                <link rel="stylesheet" href=stylesheet/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options root=base_path/>
            </head>
            <body></body>
        </html>
    }
}

/// Generates the site shell, then serves it unless static generation was requested.
#[tokio::main]
async fn main() {
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    let index_path = PathBuf::from(&*leptos_options.site_root).join("index.html");

    tokio::fs::write(index_path, shell(leptos_options.clone()).to_html())
        .await
        .expect("could not write index.html");

    if env::args().any(|argument| argument == GENERATE_SITE_ARG) {
        return;
    }

    let app = Router::new()
        .route("/", get(file_and_error_handler))
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::{asset_url, normalize_base_path};

    /// Confirms Pages paths are normalized without changing root hosting.
    #[test]
    fn normalizes_deployment_base_paths() {
        assert_eq!(normalize_base_path(""), "");
        assert_eq!(normalize_base_path("/"), "");
        assert_eq!(normalize_base_path("rosario"), "/rosario");
        assert_eq!(normalize_base_path(" /rosario/ "), "/rosario");
    }

    /// Confirms assets remain valid at both root and project-site scopes.
    #[test]
    fn joins_assets_to_the_deployment_base_path() {
        assert_eq!(asset_url("", "pkg/rosary.js"), "/pkg/rosary.js");
        assert_eq!(
            asset_url("/rosario", "/pkg/rosary.js"),
            "/rosario/pkg/rosary.js"
        );
    }
}

pub async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
) -> AxumResponse {
    let root = options.site_root.clone();
    match get_static_file(uri.clone(), &root).await {
        Ok(res) => res.into_response(),
        Err(_) => get_static_file(Uri::from_static("/index.html"), &root)
            .await
            .expect("could not find index.html")
            .into_response(),
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.map(Body::new)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}
