package io.github.javidaloca;

/**
 * The base class of all fluent value types.
 *
 * @implNote Currently, this library does not have first class support for custom Java types.
 *           I.e., you may extend this class to implement a custom type, but you still
 *           need to make native bindings manually.
 *
 *           However, most of the required bindings already exist in this library.
 *
 *           Subclasses of this class should follow the pattern of {@link FluentString} and others
 *           by implementing a native {@code bind()} method that creates a corresponding Rust value
 *           and transfers ownership of it to the Java object. This can be done using
 *           {@link https://github.com/jni-rs/jni-rs jni-rs} and its
 *           {@link https://docs.rs/jni/0.16.0/jni/struct.JNIEnv.html#method.set_rust_field set_rust_field method}.
 *           A field to store the pointer (called "pointer") already exists in the superclass of this class
 *           and should be used for this purpose. This superclass also takes care of dropping the Rust value
 *           before the Java object is garbage collected to prevent memory leaks.
 *
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public abstract class FluentValue extends RustObject {

}
