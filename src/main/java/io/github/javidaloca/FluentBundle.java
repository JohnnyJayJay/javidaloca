package io.github.javidaloca;

import java.util.*;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
// TODO custom functions
public final class FluentBundle extends RustObject {

  private final List<Locale> locales;

  private FluentBundle(List<Locale> locales) {
    this.locales = Collections.unmodifiableList(new ArrayList<>(locales));
    bind(this.locales);
  }

  private native void bind(List<Locale> locales);

  public native void addResource(FluentResource resource);

  public native boolean hasMessage(String id);

  public Optional<String> formatMessage(String id, Map<String, ? extends FluentValue> arguments) {
    return Optional.ofNullable(formatMessageRs(id, arguments));
  }

  private native String formatMessageRs(String id, Map<String, ? extends FluentValue> arguments);

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
