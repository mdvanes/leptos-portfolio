use leptos::prelude::*;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CloseEvent, ErrorEvent, MessageEvent, WebSocket};

#[derive(Deserialize, Clone)]
struct TimeMessage {
    server_time: String,
}

#[derive(Clone, PartialEq)]
enum ConnectionStatus {
    Connecting,
    Connected,
    Disconnected,
    Error,
}

#[component]
pub fn ServerTime() -> impl IntoView {
    let (server_time, set_server_time) = signal(String::from("Connecting..."));
    let (connection_status, set_connection_status) = signal(ConnectionStatus::Connecting);

    // Effect to handle WebSocket connection
    Effect::new(move |_| {
        // Only run on client side
        if !cfg!(feature = "hydrate") && !cfg!(feature = "csr") {
            return;
        }

        let protocol = if web_sys::window().unwrap().location().protocol().unwrap() == "https:" {
            "wss:"
        } else {
            "ws:"
        };

        let host = web_sys::window().unwrap().location().host().unwrap();

        let ws_url = format!("{p}//{h}/ws", p = protocol, h = host);

        // Debug: log the URL being used
        web_sys::console::log_1(&format!("Attempting WebSocket connection to: {}", ws_url).into());

        let ws = match WebSocket::new(&ws_url) {
            Ok(ws) => ws,
            Err(_) => {
                set_connection_status.set(ConnectionStatus::Error);
                return;
            }
        };

        // Handle WebSocket message
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                let text_str = text.as_string().unwrap_or_default();
                if let Ok(time_msg) = serde_json::from_str::<TimeMessage>(&text_str) {
                    set_server_time.clone().set(time_msg.server_time);
                    set_connection_status
                        .clone()
                        .set(ConnectionStatus::Connected);
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        // Handle WebSocket close
        let onclose_callback = Closure::wrap(Box::new(move |_: CloseEvent| {
            set_connection_status
                .clone()
                .set(ConnectionStatus::Disconnected);
        }) as Box<dyn FnMut(CloseEvent)>);
        ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();

        // Handle WebSocket error
        let onerror_callback = Closure::wrap(Box::new(move |_: ErrorEvent| {
            set_connection_status.clone().set(ConnectionStatus::Error);
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();
    });

    view! {
        <div class="server-time">
            <div class="server-time-content">
                <h3>
                    "Server Time"
                    <span class={move || format!("connection-dot {}",
                        match connection_status.get() {
                            ConnectionStatus::Connected => "connected",
                            ConnectionStatus::Connecting => "connecting",
                            ConnectionStatus::Disconnected | ConnectionStatus::Error => "disconnected"
                        }
                    )}></span>
                </h3>
                <p>{move || server_time.get()}</p>
            </div>
        </div>
    }
}
