use fluent_bundle::{FluentArgs, FluentValue};
use jni::JNIEnv;
use jni::objects::{JObject, JString};
use crate::{surrender_rust_pointer, get_rust_pointer};

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentArgs_bind(
    env: JNIEnv,
    this: JObject
) {
    let args = FluentArgs::new();
    surrender_rust_pointer(&env, &this, args);
}

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentArgs_insert(
    env: JNIEnv,
    this: JObject,
    parameter: JString,
    value: JObject
) {
    let mut args = get_rust_pointer::<FluentArgs>(&env, &this);
    let parameter = env.get_string(parameter)
        .expect("Could not convert Java String to Rust String");
    let parameter = String::from(parameter.to_str().unwrap());
    let value = get_rust_pointer::<FluentValue>(&env, &value).clone();
    // FIXME
    args.insert(&*parameter, value);
}
