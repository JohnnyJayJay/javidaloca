use fluent_bundle::FluentResource;
use jni::JNIEnv;
use jni::objects::{JString, JObject};
use crate::{surrender_rust_pointer, throw_parse_exception};

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentResource_bind(
    env: JNIEnv,
    this: JObject,
    java_source: JString
) {
    let source = env.get_string(java_source)
        .expect("Could not convert Java string to rust string");
    let result = FluentResource::try_new(
        source.to_str().map(|s| String::from(s)).unwrap());
    match result {
        Ok(resource) => surrender_rust_pointer(&env, &this, resource),
        Err((_, errors)) => throw_parse_exception(&env, java_source, errors),
    };
}