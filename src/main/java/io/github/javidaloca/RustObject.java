package io.github.javidaloca;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public abstract class RustObject {

  static {
    System.loadLibrary("fluentbindings");
  }

  private long pointer;

  private native void surrender();

  @Override
  protected final void finalize() {
    surrender();
  }
}
