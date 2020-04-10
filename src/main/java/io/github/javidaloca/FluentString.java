package io.github.javidaloca;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public final class FluentString extends FluentValue {

  private final String value;

  private FluentString(String value) {
    this.value = value;
    bind(value);
  }

  public static FluentString of(String value) {
    return new FluentString(value);
  }

  private native void bind(String value);
}
