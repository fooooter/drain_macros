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

    let attr_str_prepared = attr_str
        .trim_end_matches(|x| x == '/' || x == '\\')
        .replace(|x| x == '/' || x == '\\', "::");

    format!("#[unsafe(export_name = \"{attr_str_prepared}\")]{}(REQUEST_DATA: drain_common::RequestData, \
                REQUEST_HEADERS: &std::collections::HashMap<String, String>, \
                RESPONSE_HEADERS: &mut std::collections::HashMap<String, String>, \
                SET_COOKIE: &mut std::collections::HashMap<String, drain_common::cookies::SetCookie>,\
                HTTP_STATUS_CODE: &mut u16) -> Result<Option<Vec<u8>>, Box<dyn std::any::Any + Send>> {{\
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {{
                        tokio::runtime::Builder::new_multi_thread()\
                            .enable_all()\
                            .build()\
                            .unwrap()\
                            .block_on(async {{\
                                {page_fn_body}\
                            )\
                    }}))
                }}",
            page_fn_split.0)
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn set_header(input: TokenStream) -> TokenStream {
    let in_str = input.to_string();
    let args_split = in_str.split_once(',').unwrap();
    format!("RESPONSE_HEADERS.insert({}.to_lowercase(), String::from({}))",
            args_split.0,
            args_split.1)
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn header(input: TokenStream) -> TokenStream {
    let in_str = input.to_string();
    format!("REQUEST_HEADERS.get({})", in_str)
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn cookies(_input: TokenStream) -> TokenStream {
    "drain_common::cookies::cookies(REQUEST_HEADERS)"
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn start_session(_input: TokenStream) -> TokenStream {
    "drain_common::sessions::start_session(REQUEST_HEADERS, SET_COOKIE)"
        .parse()
        .unwrap()
}

#[proc_macro_derive(SessionValue)]
pub fn derive_session_value(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let impl_decl = input_str
        .trim_start_matches("pub")
        .trim_start_matches("(crate)")
        .trim_start_matches("(super)")
        .split_once(|p| p == '{' || p == ';' || p == '(')
        .unwrap().0
        .trim()
        .split_once(' ')
        .unwrap();

    if !impl_decl.0.eq("struct") && !impl_decl.0.eq("enum") {
        panic!("Implementor of SessionValue must be either a struct or an enum.");
    }

    let impl_name = impl_decl.1;

    format!("\nimpl SessionValue for {impl_name} {{ fn as_any(&self) -> &dyn std::any::Any {{ self }} }}")
        .parse()
        .unwrap()
}