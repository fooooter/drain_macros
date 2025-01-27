use proc_macro::TokenStream;
use regex::Regex;

#[proc_macro_attribute]
pub fn drain_endpoint(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_str = item.to_string();
    let attr_str = attr.to_string();
    let attr_str = attr_str.trim_matches('"');

    let page_fn_split = item_str.split_once('(').unwrap();
    let page_fn_body = page_fn_split.1.split_once('{').unwrap().1;

    let attr_regex = Regex::new(
        r#"^((([A-Za-z0-9\-_]*\.[[:alnum:]]+/?)+)+|([A-Za-z0-9\-_]+/?)+)+$"#
    ).unwrap();

    if !attr_regex.is_match(&attr_str) {
        panic!("Endpoint's name must consist only of alphanumeric characters, hyphens, underscores, slashes and dots.");
    }

    let attr_str_prepared = attr_str.trim_end_matches("/").replace("/", "::");

    format!("#[export_name = \"{attr_str_prepared}\"]{}(request_data: drain_common::RequestData, \
                request_headers: &std::collections::HashMap<String, String>, \
                response_headers: &mut std::collections::HashMap<String, String>, \
                set_cookie: &mut std::collections::HashMap<String, drain_common::cookies::SetCookie>) -> Option<Vec<u8>> {{\
                    tokio::runtime::Builder::new_multi_thread()\
                        .enable_all()\
                        .build()\
                        .unwrap()\
                        .block_on(async {{\
                            {page_fn_body}\
                        )\
                }}",
            page_fn_split.0)
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn set_header(input: TokenStream) -> TokenStream {
    let in_str = input.to_string();
    let args_split = in_str.split_once(',').unwrap();
    format!("response_headers.insert({}.to_lowercase(), String::from({}))",
            args_split.0,
            args_split.1)
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn header(input: TokenStream) -> TokenStream {
    let in_str = input.to_string();
    format!("request_headers.get({})", in_str)
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn cookies(_input: TokenStream) -> TokenStream {
    "drain_common::cookies::cookies(request_headers)"
        .parse()
        .unwrap()
}