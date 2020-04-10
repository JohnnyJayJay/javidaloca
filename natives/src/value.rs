use fluent_bundle::FluentValue;
use jni::JNIEnv;
use jni::objects::{JObject, JString, JValue};

use crate::surrender_rust_pointer;

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentString_bind(
    env: JNIEnv,
    this: JObject,
    value: JString,
) {
    let string = env.get_string(value)
        .expect("Could not convert Java string");
    let string = string.to_str().unwrap();
    surrender_rust_pointer(&env, &this, FluentValue::from(String::from(string)));
}

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentInt_bind(
    env: JNIEnv,
    this: JObject,
    value: JValue,
) {
    surrender_rust_pointer(&env, &this, FluentValue::from(value.i().unwrap()));
}

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentLong_bind(
    env: JNIEnv,
    this: JObject,
    value: JValue,
) {
    surrender_rust_pointer(&env, &this, FluentValue::from(value.j().unwrap()))
}

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentDouble_bind(
    env: JNIEnv,
    this: JObject,
    value: JValue,
) {
    surrender_rust_pointer(&env, &this, FluentValue::from(value.d().unwrap()))
}