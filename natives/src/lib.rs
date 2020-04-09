use jni::JNIEnv;
use jni::objects::JObject;
use std::sync::MutexGuard;
use fluent::FluentResource;

mod resource;

// TODO check if this works with type parameters (at runtime)
#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_RustObject_surrender<T>(
    env: JNIEnv,
    object: JObject,
) where T : 'static + Send {
    let _value: T = env.take_rust_field(object, "pointer")
        .expect("Could not find pointer in rust object");
}

fn surrender_rust_pointer<'a, T>(env: &'a JNIEnv, object: JObject<'a>, value: T)
where T : 'static + Send {
    env.set_rust_field(object, "pointer", value)
        .expect("Could not find pointer in pointer object");
}

fn get_rust_pointer<'a, T>(env: &'a JNIEnv, object: JObject<'a>) -> MutexGuard<'a, T>
where T : 'static + Send
{
    env.get_rust_field(object, "pointer")
        .expect("Could not find pointer in pointer object")
}