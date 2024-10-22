use human_regex::{capture, one_or_more, text, within_set, zero_or_more};

fn main() {
    // url with some parameters
    let url = r"https://example.com?a=1&b=2&c=3";

    // match on url params
    // see: https://stackoverflow.com/questions/14679113/getting-all-url-parameters-using-regex#comment85109532_14679138
    #[rustfmt::skip]
    let url_params_regex = (
        within_set(&[text("?&;")])
        + capture(
            one_or_more(!text("=")) 
            + text("=") 
            + zero_or_more(!within_set(&[text("&;")]))
        )
    )
    .to_regex();
    for capture in url_params_regex.captures_iter(url) {
        println!("{}", &capture[1]);
    }
}
