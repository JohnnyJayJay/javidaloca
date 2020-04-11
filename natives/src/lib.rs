use jni::JNIEnv;
use jni::objects::{JObject, JString};
use std::sync::MutexGuard;
use unic_langid::LanguageIdentifier;
use std::panic::resume_unwind;
use std::env::var;

mod resource;
mod value;
mod args;
mod bundle;

// TODO check if this works with type parameters (at runtime)
#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_RustObject_surrender<T>(
    env: JNIEnv,
    object: JObject,
) where T : 'static + Send {
    let _value: T = env.take_rust_field(object, "pointer")
        .expect("Could not find pointer in rust object");
}

fn surrender_rust_pointer<'a, T>(env: &'a JNIEnv, object: &'a JObject, value: T)
where T : 'static + Send {
    env.set_rust_field(*object, "pointer", value)
        .expect("Could not find pointer in rust object");
}

fn get_rust_pointer<'a, T>(env: &'a JNIEnv, object: &'a JObject) -> MutexGuard<'a, T>
where T : 'static + Send {
    env.get_rust_field(*object, "pointer")
        .expect("Could not find pointer in rust object")
}

fn javastr_to_ruststr(env: &JNIEnv, string: JString) -> String {
    let result = env.get_string(string)
        .expect("Could not convert Java String to Rust String");
    let result = result.to_str().unwrap();
    String::from(result)
}

fn locale_to_langid(env: &JNIEnv, locale: JObject) -> Option<LanguageIdentifier> {
    let language = call_str_getter(env, &locale, "getLanguage");
    let script = call_str_getter(env, &locale, "getScript");
    let region = call_str_getter(env, &locale, "getCountry");
    let variant = call_str_getter(env, &locale, "getVariant");
    let variants: Vec<String> = match variant {
        Some(v) => vec![v],
        None => vec![]
    };
    if let Ok(id) = LanguageIdentifier::from_parts(language, script, region,
                                                   variants.as_slice()) {
        Some(id)
    } else {
        // TODO better exception type
        env.throw_new(
            "java/lang/RuntimeException",
            format!("Invalid locale: could not convert Java Locale {:?} to Rust LanguageIdentifier", locale));
        None
    }
}

fn call_str_getter(env: &JNIEnv, object: &JObject, name: &str) -> Option<String> {
    let return_value = env.call_method(*object, name, "()Ljava/lang/String;", &[])
        .expect(&format!("Could not call {:?}#{}()", object, name));
    let jstring = JString::from(return_value.l().unwrap());
    let s = javastr_to_ruststr(env, jstring);
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

