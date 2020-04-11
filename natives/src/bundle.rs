use std::ptr::null_mut;

use fluent_bundle::{FluentArgs, FluentError, FluentResource};
use fluent_bundle::concurrent::FluentBundle;
use jni::JNIEnv;
use jni::objects::{JList, JObject, JString};
use jni::sys::{jboolean, jobject};
use unic_langid::LanguageIdentifier;

use crate::{get_rust_pointer, javastr_to_ruststr, locale_to_langid, surrender_rust_pointer, throw};

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentBundle_bind(
    env: JNIEnv,
    this: JObject,
    locales: JList,
) {
    let mut lang_ids: Vec<LanguageIdentifier> = Vec::new();
    for locale in locales.iter().unwrap() {
        if let Some(lang_id) = locale_to_langid(&env, locale) {
            lang_ids.push(lang_id);
        }
    }
    let bundle: FluentBundle<FluentResource> = FluentBundle::new(lang_ids.iter());
    surrender_rust_pointer(&env, &this, bundle);
}

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentBundle_addResource(
    env: JNIEnv,
    this: JObject,
    resource: JObject,
) {
    let mut bundle = get_rust_pointer::<FluentBundle<FluentResource>>(&env, &this);
    let resource = get_rust_pointer::<FluentResource>(&env, &resource);
    // FIXME illegal move out of resource (copy/clone it somehow to fix)
    if let Err(errors) = bundle.add_resource(*resource) {
        throw(&env, errors);
    }
}

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentBundle_hasMessage(
    env: JNIEnv,
    this: JObject,
    id: JString,
) -> jboolean {
    let bundle = get_rust_pointer::<FluentBundle<FluentResource>>(&env, &this);
    let id = javastr_to_ruststr(&env, id);
    bundle.has_message(&id).into()
}

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentBundle_formatMessageRs(
    env: JNIEnv,
    this: JObject,
    id: JString,
    args: JObject,
) -> jobject {
    let bundle = get_rust_pointer::<FluentBundle<FluentResource>>(&env, &this);
    let id = javastr_to_ruststr(&env, id);
    let args = get_rust_pointer::<FluentArgs>(&env, &args);
    match bundle.get_message(&id) {
        Some(message) => {
            let mut errors: Vec<FluentError> = vec![];
            let result = bundle.format_pattern(
                message.value.unwrap(), Some(&*args), &mut errors);
            if !errors.is_empty() {
                throw(&env, errors);
                return null_mut()
            }
            let failure = format!("Failed to create Java String from {}", result);
            let result = env.new_string(result)
                .expect(&failure);
            result.into_inner()
        },
        None => null_mut()
    }
}