use std::ffi::c_void;
use jni::{JavaVM, JNIEnv, NativeMethod};
use jni::objects::{JClass, JObject};
use jni::signature::{JavaType, TypeSignature};
use jni::sys::{jint, JNI_ERR, JNI_VERSION_1_4, jstring};
use rust_core::get_core_info;
use crate::error::Error;
use crate::logger::init_logger;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");

fn unwrap_or_throw<T>(env: &JNIEnv, result: Result<T, Error>, default: T) -> T {
    if env.exception_check().unwrap() {
        return default;
    }
    match result {
        Ok(data) => data,
        Err(error) => {
            let exception_class = env
                .find_class("com/smalls0098/mylibrary/SmallsException")
                .unwrap();
            env.throw_new(exception_class, format!("{:?}", error))
                .unwrap();
            default
        }
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    let env = vm.get_env().expect("Cannot get reference to the JNIEnv");
    let result = (|| -> Result<jint, Error> {
        register_natives(env)?;
        Ok(JNI_VERSION_1_4)
    })();
    unwrap_or_throw(&env, result, JNI_ERR)
}

pub fn register_natives(env: JNIEnv) -> Result<(), Error> {
    let runtime_methods = vec![
        NativeMethod {
            name: "getVersion".into(),
            sig: TypeSignature {
                args: vec![],
                ret: JavaType::Object("java/lang/String".into()),
            }
                .to_string()
                .into(),
            fn_ptr: get_version as *mut c_void,
        },
    ];
    let runtime_class = env.find_class("com/smalls0098/mylibrary/RustServer")?;
    env.register_native_methods(runtime_class, runtime_methods.as_slice())?;
    Ok(())
}

pub fn get_version(env: JNIEnv, _: JClass) -> jstring {
    init_logger();
    let result = (|| -> Result<jstring, Error> {
        log::info!("get_core_info: {}", get_core_info());
        let v = env.new_string(VERSION)?;
        Ok(v.into_inner())
    })();
    unwrap_or_throw(&env, result, JObject::null().into_inner())
}
