use fluent_bundle::FluentResource;
use jni::JNIEnv;
use jni::objects::{JClass, JString, JObject};
use jni::sys::jobject;

use crate::surrender_rust_pointer;
use std::ptr::null_mut;

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentResource_bind(
    env: JNIEnv,
    this: JObject,
    source: JString
) {
    let source = env.get_string(source)
        .expect("Could not convert Java string to rust string");
    let result = FluentResource::try_new(
        source.to_str().map(|s| String::from(s)).unwrap());
    match result {
        Ok(resource) =>
            surrender_rust_pointer(&env, &this, resource),
        Err(result) => {
            // TODO use errors properly
            let (resource, errors) = result;
            env.throw_new("java/lang/RuntimeException",
                          "Something went wrong while parsing").unwrap();
        }
    }
}