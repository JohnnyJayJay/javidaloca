package io.github.javidaloca;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public final class FluentLong extends FluentValue {

  private final long value;

  private FluentLong(long value) {
    this.value = value;
    bind(value);
  }

  public static FluentLong of(long value) {
    return new FluentLong(value);
  }

  private native void bind(long value);

}
