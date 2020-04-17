package io.github.javidaloca;

import javax.annotation.Nonnull;

import static io.github.javidaloca.Checks.notNull;

/**
 * A String value. Corresponds to FluentValue::String in fluent-rs.
 *
 * @see #of(String)
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public final class FluentString extends FluentValue {

  private final String value;

  private FluentString(String value) {
    this.value = value;
    bind(value);
  }

  /**
   * Creates a new FluentString by wrapping the given value.
   *
   * @param value The String value.
   * @return The new FluentString instance
   * @throws IllegalArgumentException If the provided value is {@code null}.
   */
  @Nonnull
  public static FluentString of(@Nonnull String value) {
    return new FluentString(notNull(value, "Value"));
  }

  private native void bind(String value);

  /**
   * Returns the value wrapped by this FluentValue instance.
   *
   * @return The String value.
   */
  @Nonnull
  public String getValue() {
    return value;
  }
}
