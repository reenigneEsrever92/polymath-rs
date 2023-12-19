package eu.reverseengineer.polymath;

import com.sun.jna.Library;
import com.sun.jna.Native;

import java.io.IOException;

public class Polymath {
  public interface AsciiMathSO extends Library {
    String to_math_ml(String asciimath);
  }

  private static AsciiMathSO INSTANCE;

  public static Polymath of(String asciimath) {
    return new Polymath(asciimath);
  }

  private final String asciimath;

  private Polymath(String asciimath) {
    this.asciimath = asciimath;
  }

  private void init() {
    try {
      INSTANCE = Native.load(
          Native.extractFromResourcePath("polymath_c").getAbsolutePath(),
          AsciiMathSO.class);
    } catch (IOException e) {
      throw new RuntimeException(e);
    }
  }

  public String to_math_ml() {
    if (INSTANCE == null) {
      init();
    }

    return INSTANCE.to_math_ml(asciimath);
  }
}
