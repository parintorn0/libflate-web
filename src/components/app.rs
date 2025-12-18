use leptos::__reexports::wasm_bindgen_futures::JsFuture;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys::js_sys::{Array, Uint8Array};
use leptos::web_sys::{Blob, Url};
use libflate::gzip;
use std::collections::HashMap;
use std::io::Write;
use thaw::*;

#[component]
fn AppTheme(children: Children) -> impl IntoView {
    let brand_colors = HashMap::from([
        (10, "#040203"),
        (20, "#1E141B"),
        (30, "#331F2D"),
        (40, "#45273C"),
        (50, "#57304C"),
        (60, "#69395C"),
        (70, "#7D436C"),
        (80, "#904C7E"),
        (90, "#A5568F"),
        (100, "#B95FA1"),
        (110, "#CF69B3"),
        (120, "#E473C6"),
        (130, "#EE87D1"),
        (140, "#F39EDA"),
        (150, "#F7B4E2"),
        (160, "#FAC9EA"),
    ]);
    let theme = RwSignal::new(Theme::custom_dark(&brand_colors));
    Effect::new(move |_| theme.set(Theme::custom_dark(&brand_colors)));

    view! {
        <ConfigProvider theme class="body">
            {children()}
        </ConfigProvider>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let had_compressed = RwSignal::new(false);
    let compressed_url = RwSignal::new(String::new());
    let compressed_file_name = RwSignal::new(String::new());

    let compress = move |file_list: FileList| {
        let file = file_list.get(0).unwrap();
        let blob = file.slice();
        if let Err(err) = blob {
            console_log(&format!("{:?}", err));
            return;
        }
        let blob = file.slice().unwrap();
        spawn_local(async move {
            let bytes = JsFuture::from(blob.bytes()).await;
            if let Err(err) = bytes {
                console_log(&format!("{:?}", err));
                return;
            }
            let bytes = Uint8Array::new(&bytes.unwrap()).to_vec();
            let encoder = gzip::Encoder::new(Vec::new());
            if let Err(err) = encoder {
                console_log(&format!("{:?}", err));
                return;
            }
            let mut encoder = encoder.unwrap();
            encoder.write_all(&bytes[..]).unwrap();
            let compressed = encoder.finish().into_result();
            if let Err(err) = compressed {
                console_log(&format!("{:?}", err));
                return;
            }
            let compressed: Vec<u8> = compressed.unwrap();
            let u8_array = Uint8Array::from(compressed.as_slice());
            let parts = Array::new();
            parts.push(&u8_array);
            let blob = Blob::new_with_u8_array_sequence(&parts);
            if let Err(err) = blob {
                console_log(&format!("{:?}", err));
                return;
            }
            let blob = blob.unwrap();
            let blob_url = Url::create_object_url_with_blob(&blob);
            if let Err(err) = blob_url {
                console_log(&format!("{:?}", err));
                return;
            }
            let blob_url = blob_url.unwrap();

            let link = document().create_element("a");
            if let Err(err) = link {
                console_log(&format!("{:?}", err));
                return;
            }
            compressed_url.set(blob_url.clone());
            compressed_file_name.set(format!("{}.gz", file.name()));
            had_compressed.set(true);
        })
    };
    view! {
        <AppTheme>
            <div class="app">
                <Upload custom_request=compress>
                    <UploadDragger>"Click or drag a file to this area to upload"</UploadDragger>
                </Upload>
                <Show when=move || had_compressed.get()>
                    <a href=compressed_url download=compressed_file_name>{compressed_file_name}</a>
                </Show>
            </div>
        </AppTheme>
    }
}
