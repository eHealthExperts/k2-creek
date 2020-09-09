fn main() {
    #[cfg(windows)]
    {
        let res = winres::WindowsResource::new();
        let _ = res.compile().expect("Failed to run resource compiler!");
    }
}
