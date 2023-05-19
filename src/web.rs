pub fn web() {
    println!(
        "<html>\n\t<head></head>\n\t<body>\n\t\t{}\n\t</body>\n</html>",
        nav()
    );
}

fn nav() -> String {
    String::from("<div>Home Projects About</div>")
}
