use fluent_bundle::FluentValue;
use fluent_bundle::types::{FluentNumber, FluentNumberStyle, FluentNumberCurrencyDisplayStyle, FluentNumberOptions};
use jni::JNIEnv;
use jni::sys::jdouble;
use jni::objects::{JObject, JString};

use crate::{javastr_to_ruststr, surrender_rust_pointer, call_str_getter};

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentString_bind(
    env: JNIEnv,
    this: JObject,
    value: JString,
) {
    let string = javastr_to_ruststr(&env, value);
    surrender_rust_pointer(&env, &this, FluentValue::from(string));
}

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentNumber_bind(
    env: JNIEnv,
    this: JObject,
    value: jdouble,
    options: JObject
) {
    surrender_rust_pointer(&env, &this, FluentValue::Number(FluentNumber {
        value,
        options: java_options_to_rust(&env, options)
    }));
}

fn java_options_to_rust(env: &JNIEnv, options: JObject) -> FluentNumberOptions {
    let style = env.get_field(
        options, "style", "Lio/github/javidaloca/FluentNumber$Style;"
    ).unwrap().l().unwrap();
    let name = call_str_getter(env, &style, "name").unwrap().to_lowercase();
    let style = FluentNumberStyle::from(&*name);
    let currency = env.get_field(options, "currency", "Ljava/lang/String;")
        .unwrap().l().unwrap();
    let currency = if currency.is_null() {
        None
    } else {
        Some(javastr_to_ruststr(env, JString::from(currency)))
    };
    let currency_display = env.get_field(
        options, "currencyDisplay",
        "Lio/github/javidaloca/FluentNumber$CurrencyDisplayStyle;"
    ).unwrap().l().unwrap();
    let name = call_str_getter(env, &currency_display, "name").unwrap().to_lowercase();
    let currency_display = FluentNumberCurrencyDisplayStyle::from(&*name);
    let get_usize_field = |name: &str| {
        let value = env.get_field(options, name, "J").unwrap().j().unwrap();
        if value < 0 {
            None
        } else {
            Some(value as usize)
        }
    };
    let use_grouping = env.get_field(options, "useGrouping", "Z").unwrap().z().unwrap();
    let minimum_integer_digits = get_usize_field("minimumIntegerDigits");
    let minimum_fraction_digits = get_usize_field("minimumFractionDigits");
    let maximum_fraction_digits = get_usize_field("maximumFractionDigits");
    let minimum_significant_digits = get_usize_field("minimumSignificantDigits");
    let maximum_significant_digits = get_usize_field("maximumSignificantDigits");
    FluentNumberOptions {
        style, currency, currency_display,
        use_grouping, minimum_integer_digits,
        minimum_fraction_digits, maximum_fraction_digits,
        minimum_significant_digits, maximum_significant_digits
    }
}