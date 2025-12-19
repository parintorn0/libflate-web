use leptos::prelude::*;
use leptos::web_sys::js_sys::{Array, Uint8Array};
use leptos::web_sys::{Blob, Url};
use leptos::{leptos_dom::logging::console_log, task::spawn_local};
use libflate::{
    deflate,
    gzip,
    // lz77,
    zlib,
};
use thaw::*;

use crate::components::algorithm::Algorithm;

fn compress(algorithm: Algorithm, uploaded_file: Vec<u8>) -> Result<Vec<u8>, bool> {
    if algorithm == Algorithm::Deflate {
        let encoder = deflate::Encoder::new(uploaded_file);
        let compressed = encoder.finish().into_result();
        if let Err(err) = compressed {
            console_log(&format!("{:?}", err));
            return Err(true);
        }
        return Ok(compressed.unwrap());
    } else if algorithm == Algorithm::Gzip {
        let encoder = gzip::Encoder::new(uploaded_file);
        if let Err(err) = encoder {
            console_log(&format!("{:?}", err));
            return Err(true);
        }
        let encoder = encoder.unwrap();
        let compressed = encoder.finish().into_result();
        if let Err(err) = compressed {
            console_log(&format!("{:?}", err));
            return Err(true);
        }
        return Ok(compressed.unwrap());
    } else if algorithm == Algorithm::Zlib {
        let encoder: Result<zlib::Encoder<Vec<u8>>, std::io::Error> =
            zlib::Encoder::new(uploaded_file);
        if let Err(err) = encoder {
            console_log(&format!("{:?}", err));
            return Err(true);
        }
        let encoder = encoder.unwrap();
        let compressed = encoder.finish().into_result();
        if let Err(err) = compressed {
            console_log(&format!("{:?}", err));
            return Err(true);
        }
        return Ok(compressed.unwrap());
    } else {
        return Err(true);
    }
}

fn decompress(algorithm: Algorithm, uploaded_file: Vec<u8>) -> Result<Vec<u8>, ()> {
    if algorithm == Algorithm::Deflate {
        let encoder = deflate::Encoder::new(uploaded_file);
        let compressed = encoder.finish().into_result();
        if let Err(err) = compressed {
            console_log(&format!("{:?}", err));
            return Err(());
        }
        return Ok(compressed.unwrap());
    } else if algorithm == Algorithm::Gzip {
        let encoder = gzip::Encoder::new(uploaded_file);
        if let Err(err) = encoder {
            console_log(&format!("{:?}", err));
            return Err(());
        }
        let encoder = encoder.unwrap();
        let compressed = encoder.finish().into_result();
        if let Err(err) = compressed {
            console_log(&format!("{:?}", err));
            return Err(());
        }
        return Ok(compressed.unwrap());
    } else if algorithm == Algorithm::Zlib {
        let encoder: Result<zlib::Encoder<Vec<u8>>, std::io::Error> =
            zlib::Encoder::new(uploaded_file);
        if let Err(err) = encoder {
            console_log(&format!("{:?}", err));
            return Err(());
        }
        let encoder = encoder.unwrap();
        let compressed = encoder.finish().into_result();
        if let Err(err) = compressed {
            console_log(&format!("{:?}", err));
            return Err(());
        }
        return Ok(compressed.unwrap());
    } else {
        return Err(());
    }
}

fn create_blob_url(
    is_compress: bool,
    algorithm: Algorithm,
    uploaded_file_name: String,
    output_file: Vec<u8>,
    output_url: RwSignal<String>,
    output_file_name: RwSignal<String>,
    has_output: RwSignal<bool>,
) {
    let u8_array = Uint8Array::from(output_file.as_slice());
    let parts = Array::new();
    parts.push(&u8_array);
    let blob = Blob::new_with_u8_array_sequence(&parts);
    if let Err(err) = blob {
        console_log(&format!("{:?}", err));
        return;
    }
    let blob = blob.unwrap();
    let url = &output_url.get();
    let _ = Url::revoke_object_url(url);
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
    output_url.set(blob_url.clone());
    if is_compress {
        output_file_name.set(format!(
            "{}.{}",
            uploaded_file_name,
            match algorithm {
                Algorithm::Deflate => "zz",
                Algorithm::Zlib => ".zlib",
                Algorithm::Gzip => "gz",
            }
        ));
    } else {
        output_file_name.set(format!(
            "{}",
            match algorithm {
                Algorithm::Deflate => uploaded_file_name
                    .split(".zz")
                    .next()
                    .unwrap_or(uploaded_file_name.as_str()),
                Algorithm::Zlib => uploaded_file_name
                    .split(".zlib")
                    .next()
                    .unwrap_or(uploaded_file_name.as_str()),
                Algorithm::Gzip => uploaded_file_name
                    .split(".gz")
                    .next()
                    .unwrap_or(uploaded_file_name.as_str()),
            }
        ));
    }
    has_output.set(true);
}

#[component]
pub fn Compression(
    uploaded_file_name: RwSignal<String>,
    uploaded_file: RwSignal<Vec<u8>>,
    output_url: RwSignal<String>,
    output_file_name: RwSignal<String>,
    has_output: RwSignal<bool>,
    is_compress: RwSignal<bool>,
    algorithm: RwSignal<Algorithm>,
    is_compressing: RwSignal<bool>,
) -> impl IntoView {
    let compress = move |_| {
        is_compressing.set(true);
        spawn_local(async move {
            if is_compress.get() {
                let compressed = compress(algorithm.get(), uploaded_file.get());
                if let Err(_) = compressed {
                    console_log("Failed to compress");
                    is_compressing.set(false);
                    return;
                }
                let output_file = compressed.unwrap();
                create_blob_url(
                    is_compress.get(),
                    algorithm.get(),
                    uploaded_file_name.get(),
                    output_file,
                    output_url,
                    output_file_name,
                    has_output,
                );
            } else {
                let decompressed = decompress(algorithm.get(), uploaded_file.get());
                if let Err(_) = decompressed {
                    console_log("Failed to Decompress");
                    is_compressing.set(false);
                    return;
                }
                let output_file = decompressed.unwrap();
                create_blob_url(
                    is_compress.get(),
                    algorithm.get(),
                    uploaded_file_name.get(),
                    output_file,
                    output_url,
                    output_file_name,
                    has_output,
                );
            }
            is_compressing.set(false);
        })
    };
    view! {
        <Button appearance=ButtonAppearance::Primary class="btn" on_click=compress>
            Start
        </Button>
    }
}
