package io.github.javidaloca;

import java.util.Locale;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public class InvalidLocaleException extends RuntimeException {

  private final Locale locale;

  public InvalidLocaleException(Locale locale) {
    super("Invalid locale: could not convert Java Locale ("
        + locale + ") to Rust LanguageIdentifier");
    this.locale = locale;
  }

  public Locale getLocale() {
    return locale;
  }
}
