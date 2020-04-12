package io.github.javidaloca;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public class OverrideException extends RuntimeException {

  private final List<Override> overrides;

  public OverrideException(List<Override> overrides) {
    super("The messages "
        + overrides.stream().map(Override::getId).collect(Collectors.joining(", "))
        + " were overridden by adding the resource.");
    this.overrides = Collections.unmodifiableList(new ArrayList<>(overrides));
  }

  public List<Override> getOverrides() {
    return overrides;
  }

  public static class Override {
    private final String kind;
    private final String id;

    public Override(String kind, String id) {
      this.kind = kind;
      this.id = id;
    }

    public String getKind() {
      return kind;
    }

    public String getId() {
      return id;
    }
  }

}
