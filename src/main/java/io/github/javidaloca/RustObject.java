package io.github.javidaloca;

import java.util.Objects;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public abstract class RustObject {

  private long pointer;

  private native void surrender();

  @Override
  protected void finalize() {
    surrender();
  }
}
