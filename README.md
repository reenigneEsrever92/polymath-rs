# polymath-rs

Polymath is an Asciimath implementation with bindings for c, java and javascript written in rust.

It can be used to turn Asciimath into mathml so it can be viewed in a browser for example.

## Quick Start

### Rust

```bash
cargo add polymath-rs
```

```rust
fn main() {
    println!("{}", polymath-rs::to_math_ml("sum_(i=1)^n i^3=((n(n+1))/2)^2"));
}
```

### Javascript - Node / Web

```bash
npm i --save polymath-web
```

```javascript
import * as wasm from "polymath-web";

console.log(wasm.asciimath_to_mathml("int_0^1 f(x)dx"));
```

### Java

```xml
<dependency>
    <groupId>eu.reverseengineer</groupId>
    <artifactId>polymath-java</artifactId>
    <version>0.1.1</version>
</dependency>
```

```java
import eu.reverseengineer.asciimath.AsciiMath;

public class App {
  public static void main(String[] args) {
    System.out.println(AsciiMath.of("obrace(ubrace(t)_(a))^ba").toMathMl());
  }
}
```

