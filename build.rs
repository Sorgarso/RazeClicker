fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winres::WindowsResource::new();

        res.set_icon("assets/logo.ico");
        res.set("FileDescription", "RazeClicker");
        res.set("ProductName", "RazeClicker");
        res.set("CompanyName", "RazeClicker");

        res.compile().expect("Failed to compile Windows resources");
    }
}
