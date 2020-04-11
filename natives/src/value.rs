use fluent::FluentValue;
use jni::JNIEnv;
use jni::objects::{JObject, JString, JValue};

use crate::{surrender_rust_pointer, javastr_to_ruststr};

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentString_bind(
    env: JNIEnv,
    this: JObject,
    value: JString,
) {
    let string = javastr_to_ruststr(&env, value);
    surrender_rust_pointer(&env, &this, FluentValue::from(string));
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