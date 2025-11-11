fn main() {
    // Only embed manifest on Windows
    #[cfg(windows)]
    {
        embed_resource::compile("p4-fast-reconcile.rc", embed_resource::NONE);
    }
}
