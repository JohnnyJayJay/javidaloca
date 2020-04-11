use jni::JNIEnv;
use jni::objects::{JObject, JString, JValue, JThrowable};
use std::sync::MutexGuard;
use unic_langid::LanguageIdentifier;
use fluent_bundle::FluentError;

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
        let message = format!("Invalid locale: could not convert Java Locale ({:?}) to Rust LanguageIdentifier", locale);
        let failure = &format!("Could not create Java String from {}", message);
        let message = env.new_string(message)
            .expect(failure);
        let exception = env.new_object(
            "io/github/javidaloca/InvalidLocaleException",
            "(Ljava/lang/String;Ljava/util/Locale;)V",
            &[JValue::from(message), JValue::from(locale)]
        ).unwrap();
        env.throw(JThrowable::from(exception));
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

// TODO exception model
fn throw<E>(env: &JNIEnv, mut errors: Vec<E>)
where E : Into<FluentError> {
    while !errors.is_empty() {
        match errors.pop().unwrap().into() {
            FluentError::ResolverError(re) => {

            },
            FluentError::ParserError(pe) => {

            },
            FluentError::Overriding { kind, id} => {

            }
        }
    }
}

