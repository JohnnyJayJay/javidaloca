package io.github.javidaloca;

import javax.annotation.Nonnull;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;

import static io.github.javidaloca.Checks.check;
import static io.github.javidaloca.Checks.notNull;

/**
 * The central class of this library. Each instance should contain the resources
 * for one particular widget, feature etc. Those resources can be overridden and formatted.
 *
 * In addition, a {@code FluentBundle} keeps a list of fallback {@code Locale}s.
 * All messages will primarily be formatted using the first locale, unless it lacks some
 * required formatting feature, in which case it will fall back to the rest of the locales
 * until an applicable one is found.
 *
 * The methods of this class delegate to functions of the corresponding Rust type, {@code FluentBundle}.
 * Each Java FluentBundle object is bound to a Rust FluentBundle.
 *
 * This class is thread safe.
 *
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public final class FluentBundle extends RustObject {

  private final List<Locale> locales;

  private FluentBundle(List<Locale> locales) {
    this.locales = Collections.unmodifiableList(new ArrayList<>(locales));
    bind(this.locales);
  }

  /**
   * Creates a new bundle based on the given fallback locales.
   *
   * @param locales A variable amount of locales to use.
   * @return a new {@code FluentBundle}.
   * @throws IllegalArgumentException If the locale array or any locale in the array is null or if the array is empty.
   * @throws InvalidLocaleException If a locale cannot be converted to a Rust LanguageIdentifier.
   * @see #create(List)
   */
  public static FluentBundle create(Locale... locales) {
    return create(locales == null ? null : Arrays.asList(locales));
  }

  /**
   * Creates a new bundle based on the given fallback locales.
   *
   * @param locales A list of locales to use.
   * @return A new {@code FluentBundle}.
   * @throws InvalidLocaleException If a locale cannot be converted to a Rust LanguageIdentifier.
   * @throws IllegalArgumentException If the list or any locale in the list is null or if the list is empty.
   */
  public static FluentBundle create(List<Locale> locales) {
    notNull(locales, "Locale list");
    check(!locales.isEmpty(), "Must provide at least one locale");
    check(locales.stream().noneMatch(Objects::isNull), "No locale in the list may be null");
    return new FluentBundle(locales);
  }

  private native void bind(List<Locale> locales);

  /**
   * Adds a resource by consuming all bytes from the given {@code InputStream}
   * and creating a UTF-8 encoded String from them. The given stream will not be closed by this method,
   * so it should be wrapped in a try-with-resources block or similar on the user end.
   *
   * @param resource An {@code InputStream} obtained from somewhere, e.g.
   *                 from {@code getClass().getResourceAsStream()}.
   * @param override Whether the resource should override messages when encountering a message id
   *                 that already exists.
   * @throws IllegalArgumentException If the InputStream is {@code null}.
   * @throws IOException If an I/O problem with the InputStream occurs.
   * @throws ParseException If the String created from the InputStream is invalid FTL.
   * @throws OverrideException If {@code override} is {@code false} and the resource contains
   *                           one or more messages that already exist.
   * @see #addResource(String, boolean)
   */
  public void addResource(@Nonnull InputStream resource, boolean override) throws IOException {
    notNull(resource, "Resource InputStream");
    ByteArrayOutputStream bytes = new ByteArrayOutputStream();
    int next;
    while ((next = resource.read()) != -1) {
      bytes.write(next);
    }
    addResource(new String(bytes.toByteArray(), StandardCharsets.UTF_8), override);
  }

  /**
   * Adds a resource by consuming all bytes from the given resource file and creating a
   * UTF-8 encoded String from them.
   *
   * @param resource A {@code java.nio.file.Path} pointing to a file that contains valid FTL text.
   * @param override Whether the resource should override messages when encountering a message id
   *                 that already exists.
   * @throws IllegalArgumentException If the Path is {@code null}.
   * @throws IOException If an I/O problem with the file occurs
   * @throws ParseException If the file content is invalid FTL.
   * @throws OverrideException If {@code override} is {@code false} and the resource contains
   *                           one or more messages that already exist.
   * @see java.nio.file.Paths#get(String, String...)
   * @see #addResource(String, boolean)
   */
  public void addResource(@Nonnull Path resource, boolean override) throws IOException {
    addResource(new String(
        Files.readAllBytes(notNull(resource, "Resource Path")),
        StandardCharsets.UTF_8
    ), override);
  }

  /**
   * Adds a resource by parsing the given String to an FTL-Resource.
   *
   * @param resource An String whose content is valid FTL.
   * @param override Whether the resource should override messages when encountering a message id
   *                 that already exists.
   * @throws IllegalArgumentException If the String is {@code null}.
   * @throws ParseException If the String is invalid FTL.
   * @throws OverrideException If {@code override} is {@code false} and the resource contains
   *                           one or more messages that already exist.
   * @see https://projectfluent.org/fluent/guide/
   */
  public void addResource(@Nonnull String resource, boolean override) {
    addResourceRs(notNull(resource, "FTL string"), override);
  }

  private native void addResourceRs(String resource, boolean override);

  /**
   * Returns whether a message with the given id exists in this resource.
   *
   * @param id The message id to check.
   * @return {@code true} if this bundle contains the message, {@code false} if not.
   * @throws IllegalArgumentException If the message id is {@code null}.
   */
  public boolean hasMessage(@Nonnull String id) {
    return hasMessageRs(notNull(id, "Message id"));
  }

  private native boolean hasMessageRs(String id);

  /**
   * Formats a message from this bundle by its id with the given arguments.
   *
   * @param id The identifier of the message to be formatted.
   * @param arguments The arguments for that message. A {@code Map} of variable name -> value.
   * @return An Optional containing the formatted message value or an empty Optional if no
   *         message with the given id could be found.
   * @throws IllegalArgumentException If the id or the argument map are {@code null} or
   *                                  if any keys or values in the map are null.
   * @throws MessageFormatException If the message could not be formatted
   *                                (e.g. because of missing arguments)
   * @see FluentArgs
   */
  @Nonnull
  public Optional<String> formatMessage(@Nonnull String id, @Nonnull Map<String, ? extends FluentValue> arguments) {
    notNull(arguments, "Arguments");
    check(arguments.entrySet().stream().noneMatch((entry) -> entry.getKey() == null || entry.getValue() == null),
        "Argument map must not contain null keys or values");
    return Optional.ofNullable(formatMessageRs(notNull(id, "Message id"), arguments));
  }

  private native String formatMessageRs(String id, Map<String, ? extends FluentValue> arguments);

  /**
   * Returns the fallback locales of this bundle.
   *
   * @return An unmodifiable list view of the fallback locales.
   */
  public List<Locale> getLocales() {
    return locales;
  }

}
