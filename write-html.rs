macro_rules! write_html {
    ($w:expr, ) => (());
    ($w:expr, $e:tt) => (write!($w, "{}", $e));
    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        write!($w, "<{}>", stringify!($tag));
        write_html!($w, $($inner)*);
        write!($w, "</{}>", stringify!($tag));
        write_html!($w, $($rest)*);
    }};
}

fn main() {
    use std::fmt::Write;
    let mut out = String::new();

    write_html!(&mut out,
                html[
                   head[title["My Page"]]
                       body[
                         h1["Willkommen"]
                         p["Wunderbar, oder?"]
                         p["Stimmt!"]]
                ]);

    println!("{}", out);
}
