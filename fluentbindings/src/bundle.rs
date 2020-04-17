use std::collections::HashMap;
use std::ptr::null_mut;

use fluent_bundle::{FluentArgs, FluentError, FluentResource, FluentValue};
use fluent_bundle::concurrent::FluentBundle;
use jni::JNIEnv;
use jni::objects::{JList, JMap, JObject, JString};
use jni::sys::{jboolean, jobject};
use unic_langid::LanguageIdentifier;

use crate::{get_rust_pointer, javastr_to_ruststr, locale_to_langid, surrender_rust_pointer, throw_format_exception, throw_override_exception, throw_parse_exception};

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentBundle_bind(
    env: JNIEnv,
    this: JObject,
    locales: JObject,
) {
    let mut lang_ids: Vec<LanguageIdentifier> = Vec::new();
    let locales = JList::from_env(&env, locales).unwrap();
    for locale in locales.iter().unwrap() {
        if let Some(lang_id) = locale_to_langid(&env, locale) {
            lang_ids.push(lang_id);
        } else {
            return
        }
    }
    let bundle: FluentBundle<FluentResource> = FluentBundle::new(lang_ids.iter());
    surrender_rust_pointer(&env, &this, bundle);
}

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentBundle_addResourceRs(
    env: JNIEnv,
    this: JObject,
    resource: JString,
    do_override: jboolean
) {
    if let Some(resource) = create_resource(&env, resource) {
        let mut bundle = get_rust_pointer::<FluentBundle<FluentResource>>(&env, &this);
        if do_override == 1 {
            bundle.add_resource_overriding(resource);
        } else {
            if let Err(errors) = bundle.add_resource(resource) {
                throw_override_exception(&env, errors);
            }
        }
    }
}

fn create_resource(env: &JNIEnv, source: JString) -> Option<FluentResource> {
    let rust_source = javastr_to_ruststr(env, source);
    let result = FluentResource::try_new(rust_source);
    match result {
        Ok(resource) => Some(resource),
        Err((_, errors)) => {
            throw_parse_exception(env, source, errors);
            None
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentBundle_hasMessageRs(
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
    java_id: JString,
    args: JObject,
) -> jobject {
    let bundle = get_rust_pointer::<FluentBundle<FluentResource>>(&env, &this);
    let id = javastr_to_ruststr(&env, java_id);
    // Everything about this is terrible and I'm sorry you have to witness it.
    let args = JMap::from_env(&env, args).unwrap();
    let mut args_map = HashMap::<String, FluentValue>::new();
    for (key, value) in args.iter().unwrap() {
        let key = javastr_to_ruststr(&env, JString::from(key));
        let fluent_value = get_rust_pointer::<FluentValue>(&env, &value).clone();
        args_map.insert(key, fluent_value);
    }
    let mut ids = Vec::<String>::new();
    for (id, _) in args_map.iter() {
        ids.push(id.to_owned());
    }
    let mut fluent_args = FluentArgs::new();
    for id in ids.iter() {
        fluent_args.insert(id, args_map.remove(id).unwrap());
    }
    match bundle.get_message(&id) {
        Some(message) => {
            let mut errors: Vec<FluentError> = vec![];
            let result = bundle.format_pattern(
                message.value.unwrap(), Some(&fluent_args), &mut errors);
            if !errors.is_empty() {
                throw_format_exception(&env, java_id, errors);
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