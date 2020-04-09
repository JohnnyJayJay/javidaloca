package io.github.javidaloca;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public final class FluentDouble extends FluentValue {

  private final double value;

  private FluentDouble(double value) {
    this.value = value;
  }

  public static native FluentDouble of(double value);
}
