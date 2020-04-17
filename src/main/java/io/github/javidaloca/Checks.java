package io.github.javidaloca;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
final class Checks {

  private Checks() {}

  static <T> T notNull(T arg, String name) {
    check(arg != null, name + " must not be null");
    return arg;
  }

  static void check(boolean expr, String message) {
    if (!expr) {
      throw new IllegalArgumentException(message);
    }
  }

}
