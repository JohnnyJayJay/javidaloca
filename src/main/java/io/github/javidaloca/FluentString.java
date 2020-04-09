package io.github.javidaloca;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public final class FluentString extends FluentValue {

  private final String value;

  private FluentString(String value) {
    this.value = value;
  }

  public static native FluentString of(String value);
}
