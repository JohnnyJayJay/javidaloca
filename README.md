# javidaloca
WIP Java bindings for the Project Fluent Rust implementation.

## TODO 
- [x] Mirror basic API
- [ ] Add Java API for custom types
- [ ] Add Java API for custom functions
- [ ] Setup proper gradle/cargo workflow
- [ ] Write javadoc

## Usage
```java
FluentBundle bundle = FluentBundle.create(Locale.US);
bundle.addResource("score = { $name } scores { $points } points! Yay!", false);
FluentArgs args = FluentArgs.create()
    .insert("name", "Johnny")
    .insert("points", 27.5);
String message = bundle.formatMessage("score", args).orElse("Message not found...");
System.out.println(message); // => Johnny scores 27.5 points! Yay!
```

## See 
- [Project Fluent](https://projectfluent.org)
- [Rust implementation](https://github.com/projectfluent/fluent-rs)