fn main() {
    #[cfg(target_os = "linux")]
    {
        use std::path::PathBuf;
        use std::process::Command;
        let path: PathBuf = String::from_utf8(
            Command::new("g++")
                .args(["--print-file-name=libstdc++.a"])
                .output()
                .expect("failed to find libstdc++")
                .stdout,
        )
        .unwrap()
        .into();
        println!("cargo:rustc-link-search=native={}", path.parent().unwrap().to_string_lossy());
        println!("cargo:rustc-link-lib=static=stdc++");
    }
    let files = &[
        "libwebm/mkvmuxer/mkvmuxer.cc",
        "libwebm/mkvmuxer/mkvwriter.cc",
        "libwebm/mkvmuxer/mkvmuxerutil.cc",
        "libwebm/mkvparser/mkvparser.cc",
        "libwebm/mkvparser/mkvreader.cc",
        "ffi.cpp",
    ];
    let mut c = cc::Build::new();
    c.cpp(true);
    c.warnings(false);
    c.flag("-fno-rtti");
    c.flag("-std=gnu++11");
    c.flag("-fno-exceptions");
    c.include("libwebm");
    for &f in files.iter() {
        c.file(f);
    }
    c.compile("libwebmadapter.a");
}
