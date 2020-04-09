use jni::objects::{JString, JClass, JValue, JObject};
use jni::JNIEnv;
use jni::sys::jobject;
use fluent_bundle::FluentValue;

use crate::surrender_rust_pointer;
use fluent_bundle::types::{FluentNumber};

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentString_of(
    env: JNIEnv,
    class: JClass,
    value: JString
) -> jobject {
    let string = env.get_string(value)
        .expect("Could not convert Java string").to_str().unwrap();
    of(env, class, JValue::from(value),
       "Ljava/lang/String;", FluentValue::from(string))
}

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentInt_of(
    env: JNIEnv,
    class: JClass,
    value: JValue
) -> jobject {
    of(env, class, value, "I", FluentValue::from(value.i().unwrap()))
}

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentLong_of(
    env: JNIEnv,
    class: JClass,
    value: JValue
) -> jobject {
    of(env, class, value, "J", FluentValue::from(value.j().unwrap()))
}

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentDouble_of(
    env: JNIEnv,
    class: JClass,
    value: JValue
) -> jobject {
    of(env, class, value, "D", FluentValue::from(value.d().unwrap()))
}

// FIXME FluentValue is not Send
fn of(env: JNIEnv, class: JClass, jvalue: JValue, name: &str, fluent_value: FluentValue) -> jobject {
    let object = env.new_object(
        class, format!("{}V", name), &[jvalue]
    ).unwrap();
    surrender_rust_pointer(&env, &object, fluent_value);
    object.into_inner()
}