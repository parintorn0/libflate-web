use leptos::prelude::*;
use std::collections::HashMap;
use thaw::*;

use crate::components::algorithm::Algorithm;
use crate::components::compression::Compression;
use crate::components::file_upload::FileUpload;

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
    let uploaded_file_name = RwSignal::new(String::new());
    let uploaded_file: RwSignal<Vec<u8>> = RwSignal::new(Vec::new());
    let has_output = RwSignal::new(false);
    let output_url = RwSignal::new(String::new());
    let output_file_name = RwSignal::new(String::new());
    let is_compress = RwSignal::new(true);
    let algorithm = RwSignal::new(Algorithm::Deflate);
    let is_compressing = RwSignal::new(false);

    view! {
        <AppTheme>
            <div class="app">
                <span>
                    <Link href="https://crates.io/crates/libflate">libflate</Link>" Web"
                </span>
                    <ButtonGroup>
                        <Button
                        appearance=move || { if is_compress.get() {ButtonAppearance::Primary} else {ButtonAppearance::Secondary}}
                        disabled=move || is_compress.get()
                        on_click=move |_| is_compress.set(true)
                        >
                            "Compression"
                        </Button>
                        <Button
                        appearance=move || { if !is_compress.get() {ButtonAppearance::Primary} else {ButtonAppearance::Secondary}}
                        disabled=move || !is_compress.get()
                        on_click=move |_| is_compress.set(false)
                        >
                            "Decompression"
                        </Button>
                    </ButtonGroup>
                    <ButtonGroup>
                        <Button
                            appearance=move || { if algorithm.get()==Algorithm::Deflate {ButtonAppearance::Primary} else {ButtonAppearance::Secondary}}
                            disabled=move || algorithm.get()==Algorithm::Deflate
                            on_click=move |_| algorithm.set(Algorithm::Deflate)
                            >
                            "Deflate"
                        </Button>
                        <Button
                            appearance=move || { if algorithm.get()==Algorithm::Gzip {ButtonAppearance::Primary} else {ButtonAppearance::Secondary}}
                            disabled=move || algorithm.get()==Algorithm::Gzip
                            on_click=move |_| algorithm.set(Algorithm::Gzip)
                            >
                            "Gzip"
                        </Button>
                        <Button
                            appearance=move || { if algorithm.get()==Algorithm::Zlib {ButtonAppearance::Primary} else {ButtonAppearance::Secondary}}
                            disabled=move || algorithm.get()==Algorithm::Zlib
                            on_click=move |_| algorithm.set(Algorithm::Zlib)
                            >
                            "Zlib"
                        </Button>
                    </ButtonGroup>
                    <FileUpload uploaded_file uploaded_file_name />
                    <Compression uploaded_file uploaded_file_name output_file_name output_url has_output is_compress algorithm is_compressing />
                    <Show when=move || is_compressing.get()>
                        <Spinner label="Compressing" />
                    </Show>
                    <Show when=move || has_output.get()>
                        <a href=output_url download=output_file_name>{output_file_name}</a>
                    </Show>
            </div>
            <Link href="https://github.com/parintorn0/libflate-web" class="source">
                <Badge size=BadgeSize::ExtraLarge>
                <Icon icon=icondata::LuGithub class="icon" />
                </Badge>
            </Link>
        </AppTheme>
    }
}
