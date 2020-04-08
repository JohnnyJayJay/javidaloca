package io.github.javidaloca;

import java.lang.ref.Cleaner;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public abstract class RustObject {

  private static final Cleaner CLEANER = Cleaner.create();

  private final RustPointer pointer;

  protected RustObject() {
    RustPointer pointer = new RustPointer();
    CLEANER.register(this, pointer::surrender);
    this.pointer = pointer;
  }

  private static class RustPointer {

    long pointer;

    native void surrender();

  }

}
