package io.github.javidaloca;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public class MessageFormatException extends RuntimeException {

  private final List<String> errors;

  public MessageFormatException(String messageId, List<String> errors) {
    super("Formatting " + messageId + " failed:\n\n" + String.join("\n- ", errors));
    this.errors = Collections.unmodifiableList(new ArrayList<>(errors));
  }

  public List<String> errors() {
    return errors;
  }
}
