use jni::JNIEnv;
use jni::objects::{JObject, JValue};
use std::sync::MutexGuard;
use std::any::Any;

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_RustObject_00024RustPointer_surrender(
    env: JNIEnv,
    object: JObject,
) {
    // FIXME annotate type
    let _pointer = env.take_rust_field(object, "pointer")
        .expect("Could not find pointer field");
}

// FIXME Send requirement
fn surrender_rust_pointer<'a, T>(env: &'a JNIEnv, object: JObject<'a>, value: T) {
    let pointer_obj = get_pointer_obj(env, object);
    env.set_rust_field(pointer_obj, "pointer", value);
}

// FIXME Send requirement
fn get_rust_pointer<'a, T>(env: &'a JNIEnv, object: JObject<'a>) -> MutexGuard<'a, T> {
    let pointer_obj = get_pointer_obj(env, object);
    env.get_rust_field(pointer_obj, "pointer")
        .expect("Could not find pointer in pointer object")
}

fn get_pointer_obj<'a>(env: &'a JNIEnv, object: JObject<'a>) -> JObject<'a> {
    env.get_field(
        object, "pointer", "Lio/github/javidaloca/RustObject00024RustPointer"
    ).expect("Supposed rust object had no pointer field")
        .l().expect("Pointer field was no object")
}