package io.github.javidaloca;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.util.Collections;
import java.util.Locale;
import java.util.Optional;

import static org.junit.jupiter.api.Assertions.*;

class FluentBundleTest {

  private FluentBundle bundle;

  @BeforeEach
  void setUp() {
    bundle = FluentBundle.create(Locale.US);
    bundle.addResource("key = { $one } and { $two }!", false);
  }

  @Test
  void addResource() {
    assertThrows(OverrideException.class,
        () -> bundle.addResource("key = new value!!", false),
        "FluentBundle did not throw on override");
    assertDoesNotThrow(() -> bundle.addResource("key = new value!!", true),
        "FluentBundle threw on override when override was allowed");
  }

  @Test
  void hasMessage() {
    assertTrue(bundle.hasMessage("key"), "FluentBundle did not have message");
  }

  // FIXME fluent-rs issue #172
  @Test
  void formatMessage() {
    Optional<String> message = bundle.formatMessage("key", FluentArgs.create()
        .insert("one", "test")
        .insert("two",3.141));
    assertTrue(message.isPresent(),
        "Message was not formatted although id existed");
    assertEquals("test and 3.141!", message.get(),
        "Message was not formatted correctly");
  }

  @Test
  void formatAbsentMessage() {
    Optional<String> message = bundle.formatMessage("foo", Collections.emptyMap());
    assertFalse(message.isPresent(),
        "Message was formatted although id did not exist");
  }

  @Test
  void formatException() {
    assertThrows(MessageFormatException.class,
        () -> bundle.formatMessage("key", Collections.emptyMap()),
        "FluentBundle did not throw on invalid formatting");
  }

  @Test
  void parseException() {
    assertThrows(ParseException.class,
        () -> bundle.addResource("I have not read the syntax guide, please help me :(", false),
        "Parser did not throw on syntax errors");
  }
}