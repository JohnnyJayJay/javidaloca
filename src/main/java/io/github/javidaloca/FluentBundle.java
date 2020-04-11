package io.github.javidaloca;

import java.util.*;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public final class FluentBundle extends RustObject {

  private final List<Locale> locales;

  private FluentBundle(List<Locale> locales) {
    this.locales = Collections.unmodifiableList(new ArrayList<>(locales));
    bind(this.locales);
  }

  private native void bind(List<Locale> locales);

  public static FluentBundle create(Locale... locales) {
    return create(Arrays.asList(locales));
  }

  public static FluentBundle create(List<Locale> locales) {
    if (locales.stream().anyMatch(Objects::isNull)) {
      throw new IllegalArgumentException("Locales must not be null");
    }
    return new FluentBundle(locales);
  }

}
