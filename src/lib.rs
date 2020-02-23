extern crate libc;

#[link(name = "intercept")]
extern "cdecl" {
    type intercept {
        fn api_version() -> u32;
        fn on_frame();
        fn post_init();
        fn mission_ended();
    }
}

fn post_init() {
    intercept::api_version();
}
