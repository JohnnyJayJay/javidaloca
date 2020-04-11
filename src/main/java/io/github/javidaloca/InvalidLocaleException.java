package io.github.javidaloca;

import java.util.Locale;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public class InvalidLocaleException extends RuntimeException {

  private final Locale locale;

  public InvalidLocaleException(String message, Locale locale) {
    super(message);
    this.locale = locale;
  }

  public Locale getLocale() {
    return locale;
  }
}
