package io.github.javidaloca;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.Charset;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;

/**
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public final class FluentResource extends RustObject {

  private FluentResource() {}

  public static FluentResource from(Path file) throws IOException {
    return from(file, StandardCharsets.UTF_8);
  }

  public static FluentResource from(Path file, Charset charset) throws IOException {
    return from(Files.readAllBytes(file), charset);
  }

  public static FluentResource from(InputStream inputStream) throws IOException {
    return from(inputStream, StandardCharsets.UTF_8);
  }

  public static FluentResource from(InputStream inputStream, Charset charset) throws IOException {
    ByteArrayOutputStream bytes = new ByteArrayOutputStream();
    int next;
    while ((next = inputStream.read()) != -1) {
      bytes.write(next);
    }
    return from(bytes.toByteArray(), charset);
  }

  private static FluentResource from(byte[] bytes, Charset charset) {
    return from(new String(bytes, charset));
  }

  public static FluentResource from(String source) {
    return tryNew(source);
  }

  private static native FluentResource tryNew(String source);

}
