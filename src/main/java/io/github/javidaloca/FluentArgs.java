package io.github.javidaloca;

import javax.annotation.Nonnull;
import java.util.*;

import static io.github.javidaloca.Checks.notNull;

/**
 * A utility class used to facilitate the creation of arguments for
 * {@link FluentBundle#formatMessage(String, Map)}.
 *
 * This class implements {@code Map<String, FluentValue>}.
 * Only insertion is supported and insertion may only be done via
 * one of the {@code insert} methods or {@link #put(String, FluentValue)}.
 *
 * This map does not permit null keys or values.
 *
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public final class FluentArgs extends AbstractMap<String, FluentValue> {

  private final Map<String, FluentValue> map;

  private FluentArgs() {
    this.map = new HashMap<>();
  }

  /**
   * Creates a new instance of FluentArgs.
   *
   * @return The new instance.
   */
  @Nonnull
  public static FluentArgs create() {
    return new FluentArgs();
  }

  /**
   * Inserts a floating point number value with the default {@link FluentNumber.Options number options}.
   *
   * @param parameter The name of the parameter in the message.
   * @param value The value to be associated with the parameter.
   * @return this, for chaining
   * @throws IllegalArgumentException If the parameter name is {@code null}.
   * @see FluentNumber#DEFAULT_OPTIONS
   * @see FluentNumber
   */
  @Nonnull
  public FluentArgs insert(@Nonnull String parameter, double value) {
    return insert(parameter, value, FluentNumber.DEFAULT_OPTIONS);
  }

  /**
   * Inserts a floating point number value with the provided {@link FluentNumber.Options number options}.
   *
   * @param parameter The name of the parameter in the message.
   * @param value The value to be associated with the parameter.
   * @param options The options to be used for the value.
   * @return this, for chaining
   * @throws IllegalArgumentException If the parameter name or the options are {@code null}.
   * @see FluentNumber
   */
  @Nonnull
  public FluentArgs insert(@Nonnull String parameter, double value, @Nonnull FluentNumber.Options options) {
    return insert(parameter, FluentNumber.of(value, options));
  }

  /**
   * Inserts a String value.
   *
   * @param parameter The name of the parameter in the message.
   * @param value The value to be associated with the parameter.
   * @return this, for chaining
   * @throws IllegalArgumentException If the parameter name or the value is {@code null}.
   * @see FluentString
   */
  @Nonnull
  public FluentArgs insert(@Nonnull String parameter, @Nonnull String value) {
    return insert(parameter, FluentString.of(value));
  }

  /**
   * Inserts a value.
   *
   * @param parameter The name of the parameter in the message.
   * @param value The value to be associated with the parameter.
   * @return this, for chaining
   * @throws IllegalArgumentException If the parameter name or the value is {@code null}.
   */
  @Nonnull
  public FluentArgs insert(@Nonnull String parameter, @Nonnull FluentValue value) {
    put(parameter, value);
    return this;
  }

  @Override
  public FluentValue put(@Nonnull String key, @Nonnull FluentValue value) {
    return map.put(notNull(key, "Parameter name"), notNull(value, "Value"));
  }

  @Override
  public FluentValue get(Object key) {
    return map.get(key);
  }

  @Override
  public boolean containsKey(Object key) {
    return map.containsKey(key);
  }

  @Override
  public Set<String> keySet() {
    return Collections.unmodifiableSet(map.keySet());
  }

  @Override
  public Collection<FluentValue> values() {
    return Collections.unmodifiableCollection(map.values());
  }

  @Override
  public Set<Entry<String, FluentValue>> entrySet() {
    return Collections.unmodifiableSet(map.entrySet());
  }

}
