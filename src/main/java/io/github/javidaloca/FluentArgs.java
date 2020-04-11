package io.github.javidaloca;

import java.util.*;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public final class FluentArgs extends AbstractMap<String, FluentValue> {

  private final Map<String, FluentValue> map;

  private FluentArgs() {
    this.map = new HashMap<>();
  }

  public FluentArgs insert(String parameter, double value) {
    return insert(parameter, FluentDouble.of(value));
  }

  public FluentArgs insert(String parameter, int value) {
    return insert(parameter, FluentInt.of(value));
  }

  public FluentArgs insert(String parameter, long value) {
    return insert(parameter, FluentLong.of(value));
  }

  public FluentArgs insert(String parameter, String value) {
    return insert(parameter, FluentString.of(value));
  }

  public FluentArgs insert(String parameter, FluentValue value) {
    put(parameter, value);
    return this;
  }

  @Override
  public FluentValue put(String key, FluentValue value) {
    return map.put(key, value);
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
    return map.keySet();
  }

  @Override
  public Collection<FluentValue> values() {
    return map.values();
  }

  @Override
  public Set<Entry<String, FluentValue>> entrySet() {
    return map.entrySet();
  }

  public static FluentArgs create() {
    return new FluentArgs();
  }

}
