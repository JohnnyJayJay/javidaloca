use jni::JNIEnv;
use jni::objects::{JObject, JValue};
use std::sync::MutexGuard;

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_RustObject_00024RustPointer_surrender(
    env: JNIEnv,
    object: JObject,
) {
    let _pointer = env.take_rust_field(object, "pointer")
        .expect("Could not find pointer field");
}

fn surrender_rust_pointer<T>(env: JNIEnv, object: JObject, value: T) {
    let pointer_obj = get_pointer_obj(env, object);
    env.set_rust_field(pointer_obj, "pointer", value);
}

// FIXME lifetimes
fn get_rust_pointer<T>(env: JNIEnv, object: JObject) -> MutexGuard<T> {
    let pointer_obj = get_pointer_obj(env, object);
    env.get_rust_field(pointer_obj, "pointer")
        .expect("Could not find pointer in pointer object")
}

fn get_pointer_obj<'a>(env: JNIEnv, object: JObject) -> JValue<'a> {
    env.get_field(
        object, "pointer", "Lio/github/javidaloca/RustObject00024RustPointer"
    ).expect("Supposed rust object had no pointer object field")
}