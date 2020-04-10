package io.github.javidaloca;

import java.util.HashMap;
import java.util.Map;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public class FluentArgs {

  // Cache so that the contents don't get garbage collected
  private final Map<String, FluentValue> map;

  private FluentArgs() {
    this.map = new HashMap<>();
    bind();
  }

  private native void bind();

  public FluentArgs put(String parameter, double value) {
    return put(parameter, FluentDouble.of(value));
  }

  public FluentArgs put(String parameter, int value) {
    return put(parameter, FluentInt.of(value));
  }

  public FluentArgs put(String parameter, long value) {
    return put(parameter, FluentLong.of(value));
  }

  public FluentArgs put(String parameter, String value) {
    return put(parameter, FluentString.of(value));
  }

  public FluentArgs put(String parameter, FluentValue value) {
    map.put(parameter, value);
    insert(parameter, value);
    return this;
  }

  private native void insert(String parameter, FluentValue value);

  public static FluentArgs of(Map<String, ? extends FluentValue> args) {
    FluentArgs fluentArgs = new FluentArgs();
    args.forEach(fluentArgs::put);
    return fluentArgs;
  }

  public static FluentArgs create() {
    return new FluentArgs();
  }

}
