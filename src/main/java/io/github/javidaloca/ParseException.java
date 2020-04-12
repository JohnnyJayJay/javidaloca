package io.github.javidaloca;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public class ParseException extends RuntimeException {

  private final String source;
  private final List<Error> errors;

  public ParseException(String source, List<Error> errors) {
    super("Failed to parse FluentResource:\n\n" + errors.stream()
        .map((error) -> "Error: " + error.message + " at \"" + source.substring(error.from, error.to) + "\"")
        .collect(Collectors.joining("\n----------------\n")));
    this.source = source;
    this.errors = Collections.unmodifiableList(new ArrayList<>(errors));
  }

  public String getSource() {
    return source;
  }

  public List<Error> getErrors() {
    return errors;
  }

  public static class Error {
    private final int from;
    private final int to;
    private final String message;

    public Error(int from, int to, String message) {
      this.from = from;
      this.to = to;
      this.message = message;
    }

    public int getFrom() {
      return from;
    }

    public int getTo() {
      return to;
    }

    public String getMessage() {
      return message;
    }
  }
}
