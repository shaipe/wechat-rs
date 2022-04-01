pub fn get_url_encode(c:&str)->String{
    use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
    const FRAGMENT: &AsciiSet = &CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'<')
        .add(b'>')
        .add(b'`')
        .add(b'+')
        .add(b'=')
        .add(b'/');
        utf8_percent_encode(c, FRAGMENT).to_string()
}