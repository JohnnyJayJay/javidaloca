use std::sync::MutexGuard;

use fluent_bundle::FluentError;
use fluent_bundle::resolve::ResolverError;
use fluent_syntax::parser::errors::{ErrorKind, ParserError};
use jni::JNIEnv;
use jni::objects::{JList, JObject, JString, JThrowable, JValue};
use jni::signature::JavaType;
use jni::signature::Primitive;
use unic_langid::LanguageIdentifier;

mod resource;
mod value;
mod bundle;

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_RustObject_surrender(
    env: JNIEnv,
    object: JObject,
) {
    // String is just a placeholder. In reality, this is some other type,
    // but it doesn't matter because we only want to drop it here
    let _value: String = env.take_rust_field(object, "pointer")
        .expect("Could not find pointer in rust object");
}

fn surrender_rust_pointer<'a, T>(env: &'a JNIEnv, object: &'a JObject, value: T)
    where T: 'static + Send {
    env.set_rust_field(*object, "pointer", value)
        .expect("Could not find pointer in rust object");
}

fn get_rust_pointer<'a, T>(env: &'a JNIEnv, object: &'a JObject) -> MutexGuard<'a, T>
    where T: 'static + Send {
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
            &[JValue::from(message), JValue::from(locale)],
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

fn new_list<'a, 'b>(env: &'b JNIEnv<'a>) -> JList<'a, 'b> {
    JList::from_env(
        env, env.new_object("java/util/ArrayList", "()V", &[]).unwrap(),
    ).unwrap()
}

fn throw_override_exception(env: &JNIEnv, mut errors: Vec<FluentError>) {
    let exception_cls_name = "io/github/javidaloca/OverrideException";
    let override_cls =
        env.find_class(format!("{}$Override", exception_cls_name)).unwrap();
    let override_constr =
        env.get_method_id(override_cls, "<init>",
                          "(Ljava/lang/String;Ljava/lang/String;)V").unwrap();
    let list = new_list(env);
    while !errors.is_empty() {
        let error = errors.pop().unwrap();
        if let FluentError::Overriding { kind, id } = error {
            let kind = JValue::from(*env.new_string(kind).unwrap());
            let id = JValue::from(*env.new_string(id).unwrap());
            let object = env.alloc_object(override_cls).unwrap();
            env.call_method_unchecked(
                object, override_constr,
                JavaType::Object("V".to_owned()), &[kind, id],
            );
            list.add(object);
        }
    }
    let exception = env.new_object(
        exception_cls_name, "(Ljava/util/List;)V", &[JValue::from(*list)],
    ).unwrap();
    env.throw(JThrowable::from(exception)).unwrap();
}

fn throw_format_exception(env: &JNIEnv, message_id: JString, mut errors: Vec<FluentError>) {
    let list = new_list(env);
    while !errors.is_empty() {
        if let FluentError::ResolverError(resolver_error) = errors.pop().unwrap() {
            let msg = match resolver_error {
                ResolverError::Cyclic => "Cyclic reference".into(),
                ResolverError::MissingDefault => "Missing default value".into(),
                ResolverError::TooManyPlaceables => "Too many Placeables".into(),
                ResolverError::Reference(reference) => format!("Unresolved reference: {}", reference)
            };
            list.add(*env.new_string(msg).unwrap());
        }
    }
    let exception =
        env.new_object(
            "io/github/javidaloca/MessageFormatException",
            "(Ljava/lang/String;Ljava/util/List;)V",
            &[JValue::from(*message_id), JValue::from(*list)],
        ).unwrap();
    env.throw(JThrowable::from(exception)).unwrap();
}

fn throw_parse_exception(env: &JNIEnv, source: JString, mut errors: Vec<ParserError>) {
    let exception_cls_name = "io/github/javidaloca/ParseException";
    let error_cls = env.find_class(format!("{}$Error", exception_cls_name)).unwrap();
    let error_constr = env.get_method_id(
        error_cls, "<init>", "(IILjava/lang/String;)V",
    ).unwrap();
    let list = new_list(env);
    while !errors.is_empty() {
        let parser_error = errors.pop().unwrap();
        let (from, to) = parser_error.pos;
        let msg = match parser_error.kind {
            ErrorKind::DuplicatedNamedArgument(arg) => format!("Duplicate argument name: {}", arg),
            ErrorKind::Generic => "Generic error (unknown)".into(),
            ErrorKind::ExpectedEntry => "Expected an entry".into(),
            ErrorKind::ExpectedToken(token) => format!("Expected token: {}", token),
            ErrorKind::ExpectedCharRange { range } => format!("Expected character range: {}", range),
            ErrorKind::ExpectedMessageField { entry_id } => format!("Expected message field for {}", entry_id),
            ErrorKind::ExpectedTermField { entry_id } => format!("Expected term field for {}", entry_id),
            ErrorKind::ForbiddenWhitespace => "Forbidden whitespace".into(),
            ErrorKind::ForbiddenCallee => "Forbidden callee".into(),
            ErrorKind::ForbiddenKey => "Forbidden key".into(),
            ErrorKind::MissingDefaultVariant => "Missing default variant".into(),
            ErrorKind::MissingVariants => "Missing variants".into(),
            ErrorKind::MissingValue => "Missing value".into(),
            ErrorKind::MissingVariantKey => "Missing variant key".into(),
            ErrorKind::MissingLiteral => "Missing literal".into(),
            ErrorKind::MultipleDefaultVariants => "Multiple default variants".into(),
            ErrorKind::MessageReferenceAsSelector => "Message reference as selector".into(),
            ErrorKind::TermReferenceAsSelector => "Term reference as selector".into(),
            ErrorKind::MessageAttributeAsSelector => "Message attribute as selector".into(),
            ErrorKind::TermAttributeAsPlaceable => "Term attribute as Placeable".into(),
            ErrorKind::UnterminatedStringExpression => "Unterminated string expression".into(),
            ErrorKind::PositionalArgumentFollowsNamed => "Positional argument follows named argument".into(),
            ErrorKind::ForbiddenVariantAccessor => "Forbidden variant accessor".into(),
            ErrorKind::UnknownEscapeSequence(seq) => format!("Unknown escape sequence: {}", seq),
            ErrorKind::InvalidUnicodeEscapeSequence(seq) => format!("Invalid unicode escape sequence: {}", seq),
            ErrorKind::UnbalancedClosingBrace => "Unbalanced closing brace".into(),
            ErrorKind::ExpectedInlineExpression => "Expected inline expression".into(),
            ErrorKind::ExpectedSimpleExpressionAsSelector => "Expected simple expression as selector".into()
        };
        let msg = *env.new_string(msg).unwrap();
        let error = env.alloc_object(error_cls).unwrap();
        env.call_method_unchecked(
            error, error_constr,
            JavaType::Primitive(Primitive::Void),
            &[JValue::from(from as i32), JValue::from(to as i32), JValue::from(msg)],
        ).unwrap();
        list.add(error);
    }
    let exception = env.new_object(
        exception_cls_name, "(Ljava/lang/String;Ljava/util/List;)V",
        &[JValue::from(*source), JValue::from(*list)],
    ).unwrap();
    env.throw(JThrowable::from(exception)).unwrap();
}

