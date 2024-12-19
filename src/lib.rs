use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn drain_page(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_str = item.to_string();
    let page_fn_split = item_str.split_once('(').unwrap();
    format!("{}(request_data: RequestData, response_headers: &mut HashMap<String, String>{}",
            page_fn_split.0,
            page_fn_split.1)
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn header(input: TokenStream) -> TokenStream {
    let in_str = input.to_string();
    let args_split = in_str.split_once(',').unwrap();
    format!("response_headers.insert({}.to_lowercase(), String::from({}))",
            args_split.0,
            args_split.1)
        .parse()
        .unwrap()
}