package io.github.javidaloca;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public final class FluentInt extends FluentValue {

  private final int value;

  private FluentInt(int value) {
    this.value = value;
    bind(value);
  }

  public static FluentInt of(int value) {
    return new FluentInt(value);
  }

  private native void bind(int value);
}
