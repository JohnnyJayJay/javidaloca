use fluent::FluentResource;
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jobject;

use crate::surrender_rust_pointer;
use std::ptr::null_mut;

pub extern "system" fn Java_io_github_javidaloca_FluentResource_tryNew(
    env: JNIEnv,
    class: JClass,
    source: JString
) -> jobject {
    let source = env.get_string(source)
        .map(|s| s.to_str())
        .map(|s| String::from(s))
        .expect("Could not convert Java string to rust string");
    let result = FluentResource::try_new(source);
    match result {
        Ok(resource) => {
            let object = env.new_object(class, "()V", &[])
                .unwrap();
            surrender_rust_pointer(&env, &object, resource);
            object.into_inner()
        }
        Err(result) => {
            // TODO use errors
            let (resource, errors) = result;
            env.throw_new("java/lang/RuntimeException",
                          "Something went wrong while parsing").unwrap();
            null_mut()
        }
    }
}