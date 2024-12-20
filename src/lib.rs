use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn drain_page(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_str = item.to_string();
    let page_fn_split = item_str.split_once('(').unwrap();
    let page_fn_body = page_fn_split.1.split_once('{').unwrap().1;
    format!("{}(request_data: drain_common::RequestData, \
                request_headers: &HashMap<String, String>, \
                response_headers: &mut HashMap<String, String>, \
                set_cookie: &mut HashMap<String, drain_common::cookies::SetCookie>) -> Option<Vec<u8>> {{\
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