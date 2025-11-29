use crate::database::{ProxyConfig, ProxyMode};
use reqwest::{Client, ClientBuilder, Proxy};
use std::sync::{OnceLock, RwLock};
use std::env;

const PROXY_ENV_KEYS: [&str; 6] = [
    "HTTP_PROXY",
    "http_proxy",
    "HTTPS_PROXY",
    "https_proxy",
    "ALL_PROXY",
    "all_proxy",
];

static ORIGINAL_PROXY_ENV: OnceLock<Vec<(String, Option<String>)>> = OnceLock::new();

static HTTP_CLIENT: OnceLock<RwLock<Client>> = OnceLock::new();

fn client_lock() -> &'static RwLock<Client> {
    HTTP_CLIENT.get_or_init(|| {
        let client = build_client(None).expect("Failed to create initial HTTP client");
        RwLock::new(client)
    })
}

fn build_client(proxy: Option<&ProxyConfig>) -> Result<Client, reqwest::Error> {
    let mut builder = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(10));

    if let Some(config) = proxy {
        if config.enabled {
            builder = apply_proxy(builder, config)?;
        }
    }

    builder.build()
}

fn apply_proxy(
    builder: ClientBuilder,
    config: &ProxyConfig,
) -> Result<ClientBuilder, reqwest::Error> {
    match config.mode {
        ProxyMode::System => configure_system_proxy(builder),
        ProxyMode::Https | ProxyMode::Http | ProxyMode::Socks5 => {
            let server = config.server.trim();
            if server.is_empty() {
                return Ok(builder);
            }
            let proxy = match config.mode {
                ProxyMode::Https => Proxy::https(server)?,
                ProxyMode::Http => Proxy::http(server)?,
                ProxyMode::Socks5 => Proxy::all(server)?,
                ProxyMode::System => unreachable!(),
            };
            apply_single_proxy(builder, proxy)
        }
    }
}

fn apply_single_proxy(
    builder: ClientBuilder,
    proxy: Proxy,
) -> Result<ClientBuilder, reqwest::Error> {
    Ok(builder.proxy(proxy))
}

fn configure_system_proxy(builder: ClientBuilder) -> Result<ClientBuilder, reqwest::Error> {
    let mut builder = builder;

    if let Some(url) = env_proxy_value(&["ALL_PROXY", "all_proxy"]) {
        builder = builder.proxy(Proxy::all(url)?);
    }

    if let Some(url) = env_proxy_value(&["HTTPS_PROXY", "https_proxy"]) {
        builder = builder.proxy(Proxy::https(url)?);
    }

    if let Some(url) = env_proxy_value(&["HTTP_PROXY", "http_proxy"]) {
        builder = builder.proxy(Proxy::http(url)?);
    }

    Ok(builder)
}

fn env_proxy_value(keys: &[&str]) -> Option<String> {
    for key in keys {
        if let Ok(value) = std::env::var(key) {
            let trimmed = value.trim().to_string();
            if !trimmed.is_empty() {
                return Some(trimmed);
            }
        }
    }
    None
}

fn capture_original_proxy_env() -> &'static Vec<(String, Option<String>)> {
    ORIGINAL_PROXY_ENV.get_or_init(|| {
        PROXY_ENV_KEYS
            .iter()
            .map(|key| (key.to_string(), env::var(key).ok()))
            .collect()
    })
}

fn clear_runtime_proxy_env() {
    for key in PROXY_ENV_KEYS {
        env::remove_var(key);
    }
}

fn restore_proxy_environment() {
    let original = capture_original_proxy_env();
    for (key, value) in original {
        match value {
            Some(val) => env::set_var(key, val),
            None => env::remove_var(key),
        }
    }
}

fn set_proxy_env_var(key: &str, value: &str) {
    if value.is_empty() {
        env::remove_var(key);
    } else {
        env::set_var(key, value);
    }
}

fn apply_proxy_environment(proxy: Option<&ProxyConfig>) {
    capture_original_proxy_env();

    let Some(config) = proxy else {
        clear_runtime_proxy_env();
        return;
    };

    if !config.enabled {
        clear_runtime_proxy_env();
        return;
    }

    if matches!(config.mode, ProxyMode::System) {
        restore_proxy_environment();
        return;
    }

    let server = config.server.trim();
    if server.is_empty() {
        clear_runtime_proxy_env();
        return;
    }

    clear_runtime_proxy_env();
    set_proxy_env_var("HTTP_PROXY", server);
    set_proxy_env_var("http_proxy", server);
    set_proxy_env_var("HTTPS_PROXY", server);
    set_proxy_env_var("https_proxy", server);

    if matches!(config.mode, ProxyMode::Socks5) {
        set_proxy_env_var("ALL_PROXY", server);
        set_proxy_env_var("all_proxy", server);
    } else {
        env::remove_var("ALL_PROXY");
        env::remove_var("all_proxy");
    }
}

pub fn http_client() -> Client {
    client_lock()
        .read()
        .expect("Failed to read HTTP client")
        .clone()
}

pub fn configure_http_client(proxy: Option<&ProxyConfig>) -> Result<(), String> {
    let client = build_client(proxy).map_err(|e| e.to_string())?;
    apply_proxy_environment(proxy);
    if let Ok(mut guard) = client_lock().write() {
        *guard = client;
        Ok(())
    } else {
        Err("无法更新HTTP客户端".to_string())
    }
}

pub fn validate_http_client(proxy: Option<&ProxyConfig>) -> Result<(), String> {
    build_client(proxy).map(|_| ()).map_err(|e| e.to_string())
}
