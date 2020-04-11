use fluent_bundle::concurrent::FluentBundle;
use unic_langid::LanguageIdentifier;
use jni::objects::{JObject, JList};
use jni::JNIEnv;
use crate::{locale_to_langid, surrender_rust_pointer};
use fluent_bundle::FluentResource;

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentBundle_bind(
    env: JNIEnv,
    this: JObject,
    locales: JList,
) {
    let mut lang_ids: Vec<LanguageIdentifier> = Vec::new();
    for i in 0..locales.size().unwrap() {
        let locale = locales.get(i).unwrap().unwrap();
        if let Some(lang_id) = locale_to_langid(&env, locale) {
            lang_ids.push(lang_id);
        }
    }
    let bundle: FluentBundle<FluentResource> = FluentBundle::new(lang_ids.iter());
    surrender_rust_pointer(&env, &this, bundle);
}