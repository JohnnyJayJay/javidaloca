package io.github.javidaloca;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
abstract class RustObject {

  static {
    try {
      System.loadLibrary("fluentbindings");
    } catch (UnsatisfiedLinkError e) {
      // TODO load from jar
    }
  }

  private long pointer;

  private native void surrender();

  @Override
  protected final void finalize() {
    surrender();
  }
}
