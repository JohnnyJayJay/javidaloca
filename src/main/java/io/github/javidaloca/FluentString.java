package io.github.javidaloca;

import javax.annotation.Nonnull;

import static io.github.javidaloca.Checks.notNull;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public final class FluentString extends FluentValue {

  private final String value;

  private FluentString(String value) {
    this.value = value;
    bind(value);
  }

  @Nonnull
  public static FluentString of(@Nonnull String value) {
    return new FluentString(notNull(value, "Value"));
  }

  private native void bind(String value);

  @Nonnull
  public String getValue() {
    return value;
  }
}
