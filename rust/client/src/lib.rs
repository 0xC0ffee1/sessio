
pub mod ipc;
pub mod client;

use android_logger::Config;
use std::os::raw::c_char;
use std::ffi::CStr;
use log::{info, Level};

#[no_mangle]
pub extern "C" fn start_grpc_server(path: *const std::os::raw::c_char) {
    android_logger::init_once(
        Config::default().with_min_level(Level::Info)
    );

    let c_str = unsafe { std::ffi::CStr::from_ptr(path) };
    let path_str = c_str.to_str().unwrap();

    let socket_path = if path_str.starts_with('@') {
        let mut adjusted_path = String::with_capacity(path_str.len());
        adjusted_path.push('\0');  // Abstract namespace: leading null byte
        adjusted_path.push_str(&path_str[1..]);
        adjusted_path
    } else {
        path_str.to_string()
    };    

    info!("Starting gRPC server with path: {}", socket_path);

    ipc::start_grpc_server(&socket_path);
}

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::jstring;

    #[no_mangle]
    pub unsafe extern "C" fn Java_net_c0ffee1_sessio_ui_GrpcServer_start_1grpc_1server(env: JNIEnv, _: JClass, java_pattern: JString) -> jstring {
        // Convert JString to Rust &str
        let c_str = env.get_string(java_pattern).expect("Couldn't get java string!");
        let path_str = c_str.to_str().expect("Couldn't convert java string to Rust string!");

        // Call the Rust function
        start_grpc_server(path_str.as_ptr() as *const c_char);

        // Return a new Java string as needed, here returning the input string for example
        let output = env.new_string("Server started").expect("Couldn't create java string!");
        output.into_inner()
    }
}