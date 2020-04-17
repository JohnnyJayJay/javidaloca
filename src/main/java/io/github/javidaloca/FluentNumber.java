package io.github.javidaloca;

import javax.annotation.Nonnull;
import javax.annotation.Nullable;

import java.util.Optional;

import static io.github.javidaloca.Checks.notNull;

/**
 * A numeric value. Corresponds to {@code FluentValue::Number} in
 * {@link https://github.com/projectfluent/fluent-rs fluent-rs}.
 *
 * A {@code FluentNumber} consists of two components: the actual value (as a {@code double}) and
 * a bunch of formatting {@link Options options}.
 *
 * @see #of(double, Options)
 * @see Options
 * @author Johnny_JayJay (https://www.github.com/JohnnyJayJay)
 */
public final class FluentNumber extends FluentValue {

  public static final Options DEFAULT_OPTIONS = new Options.Builder().build();

  private final double value;
  private final Options options;

  private FluentNumber(double value, Options options) {
    this.value = value;
    this.options = options;
    bind(value, options);
  }

  private native void bind(double value, Options options);

  /**
   * Creates a new instance of FluentNumber based on the given value and options.
   *
   * @param value The value, as a double.
   * @param options The options for this FluentNumber.
   * @return The new instance.
   * @throws IllegalArgumentException If the options are {@code null}.
   */
  @Nonnull
  public static FluentNumber of(double value, @Nonnull Options options) {
    return new FluentNumber(value, notNull(options, "Options"));
  }

  /**
   * Creates a new instance of FluentNumber based on the given value and the
   * {@link #DEFAULT_OPTIONS default options}.
   *
   * @param value The value, as a double.
   * @return The new instance.
   * @see #DEFAULT_OPTIONS
   * @see #of(double, Options)
   */
  @Nonnull
  public static FluentNumber of(double value) {
    return new FluentNumber(value, DEFAULT_OPTIONS);
  }

  public double getValue() {
    return value;
  }

  @Nonnull
  public Options getOptions() {
    return options;
  }

  @Nonnull
  public static Options.Builder options() {
    return new Options.Builder();
  }

  public static final class Options {

    private final Style style;
    private final String currency;
    private final CurrencyDisplayStyle currencyDisplay;
    private final boolean useGrouping;
    private final long minimumIntegerDigits;
    private final long minimumFractionDigits;
    private final long maximumFractionDigits;
    private final long minimumSignificantDigits;
    private final long maximumSignificantDigits;

    private Options(Style style, String currency, CurrencyDisplayStyle currencyDisplay, boolean useGrouping, long minimumIntegerDigits, long minimumFractionDigits, long maximumFractionDigits, long minimumSignificantDigits, long maximumSignificantDigits) {
      this.style = style;
      this.currency = currency;
      this.currencyDisplay = currencyDisplay;
      this.useGrouping = useGrouping;
      this.minimumIntegerDigits = minimumIntegerDigits;
      this.minimumFractionDigits = minimumFractionDigits;
      this.maximumFractionDigits = maximumFractionDigits;
      this.minimumSignificantDigits = minimumSignificantDigits;
      this.maximumSignificantDigits = maximumSignificantDigits;
    }

    @Nonnull
    public Style getStyle() {
      return style;
    }

    @Nonnull
    public Optional<String> getCurrency() {
      return Optional.ofNullable(currency);
    }

    @Nonnull
    public CurrencyDisplayStyle getCurrencyDisplay() {
      return currencyDisplay;
    }

    public boolean isUseGrouping() {
      return useGrouping;
    }

    public long getMinimumIntegerDigits() {
      return minimumIntegerDigits;
    }

    public long getMinimumFractionDigits() {
      return minimumFractionDigits;
    }

    public long getMaximumFractionDigits() {
      return maximumFractionDigits;
    }

    public long getMinimumSignificantDigits() {
      return minimumSignificantDigits;
    }

    public long getMaximumSignificantDigits() {
      return maximumSignificantDigits;
    }

    public static final class Builder {
      private Style style = Style.DECIMAL;
      private String currency = null;
      private CurrencyDisplayStyle currencyDisplay = CurrencyDisplayStyle.SYMBOL;
      private boolean useGrouping = true;
      private long minimumIntegerDigits = -1;
      private long minimumFractionDigits = -1;
      private long maximumFractionDigits = -1;
      private long minimumSignificantDigits = -1;
      private long maximumSignificantDigits = -1;

      private Builder() {}

      @Nonnull
      public Builder style(@Nonnull Style style) {
        this.style = notNull(style, "Number style");
        return this;
      }

      @Nonnull
      public Builder currency(@Nullable String currency) {
        this.currency = currency;
        return this;
      }

      @Nonnull
      public Builder currencyDisplay(@Nonnull CurrencyDisplayStyle currencyDisplay) {
        this.currencyDisplay = notNull(currencyDisplay, "Currency display style");
        return this;
      }

      @Nonnull
      public Builder useGrouping(boolean useGrouping) {
        this.useGrouping = useGrouping;
        return this;
      }

      @Nonnull
      public Builder minimumIntegerDigits(long minimumIntegerDigits) {
        this.minimumIntegerDigits = minimumIntegerDigits;
        return this;
      }

      @Nonnull
      public Builder minimumFractionDigits(long minimumFractionDigits) {
        this.minimumFractionDigits = minimumFractionDigits;
        return this;
      }

      @Nonnull
      public Builder maximumFractionDigits(long maximumFractionDigits) {
        this.maximumFractionDigits = maximumFractionDigits;
        return this;
      }

      @Nonnull
      public Builder minimumSignificantDigits(long minimumSignificantDigits) {
        this.minimumSignificantDigits = minimumSignificantDigits;
        return this;
      }

      @Nonnull
      public Builder maximumSignificantDigits(long maximumSignificantDigits) {
        this.maximumSignificantDigits = maximumSignificantDigits;
        return this;
      }

      @Nonnull
      public Options build() {
        return new Options(
            style, currency, currencyDisplay, useGrouping, minimumIntegerDigits,
            minimumFractionDigits, maximumFractionDigits, minimumSignificantDigits,
            maximumSignificantDigits
        );
      }
    }
  }

  public enum Style {
    DECIMAL, CURRENCY, PERCENT
  }

  public enum CurrencyDisplayStyle {
    SYMBOL, CODE, NAME
  }
}
