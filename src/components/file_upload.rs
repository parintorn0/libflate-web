use leptos::__reexports::wasm_bindgen_futures::JsFuture;
use leptos::web_sys::js_sys::Uint8Array;
use leptos::{leptos_dom::logging::console_log, prelude::*, task::spawn_local};
use thaw::*;

#[component]
pub fn FileUpload(
    uploaded_file: RwSignal<Vec<u8>>,
    uploaded_file_name: RwSignal<String>,
) -> impl IntoView {
    let has_upload = RwSignal::new(false);
    let update_upload_file = move |file_list: FileList| {
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
            uploaded_file.set(bytes.clone());
            uploaded_file_name.set(file.name());
            has_upload.set(true);
        });
    };
    view! {
        <Upload custom_request=update_upload_file>
            <Show when=move || !has_upload.get()>
                <UploadDragger>
                    "Click or drag a file to this area to upload"
                </UploadDragger>
            </Show>
            <Show when=move || has_upload.get()>
                <UploadDragger>
                    {uploaded_file_name}
                </UploadDragger>
            </Show>
        </Upload>
    }
}
