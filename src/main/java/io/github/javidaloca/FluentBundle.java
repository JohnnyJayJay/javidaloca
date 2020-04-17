package io.github.javidaloca;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
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

  public static FluentBundle create(Locale... locales) {
    return create(locales == null ? null : Arrays.asList(locales));
  }

  public static FluentBundle create(List<Locale> locales) {
    if (locales == null || locales.stream().anyMatch(Objects::isNull)) {
      throw new IllegalArgumentException("Locales must not be null");
    }
    return new FluentBundle(locales);
  }

  private native void bind(List<Locale> locales);

  public void addResource(InputStream resource, boolean override) throws IOException {
    ByteArrayOutputStream bytes = new ByteArrayOutputStream();
    int next;
    while ((next = resource.read()) != -1) {
      bytes.write(next);
    }
    addResource(new String(bytes.toByteArray()), override);
  }

  public void addResource(Path resource, boolean override) throws IOException {
    addResource(new String(Files.readAllBytes(resource)), override);
  }

  public native void addResource(String resource, boolean override);

  public native boolean hasMessage(String id);

  public Optional<String> formatMessage(String id, Map<String, ? extends FluentValue> arguments) {
    return Optional.ofNullable(formatMessageRs(id, arguments));
  }

  private native String formatMessageRs(String id, Map<String, ? extends FluentValue> arguments);

  public List<Locale> getLocales() {
    return locales;
  }

}
