use fluent_bundle::FluentResource;
use jni::JNIEnv;
use jni::objects::{JString, JObject};
use crate::{surrender_rust_pointer, throw};

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
        Ok(resource) => surrender_rust_pointer(&env, &this, resource),
        Err((_, errors)) => throw(&env, errors),
    };
}