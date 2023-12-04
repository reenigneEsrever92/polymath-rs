package eu.reverseengineer.asciimath;

import java.util.stream.Stream;

import org.assertj.core.api.Assertions;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

public class AsciiMathTest {

    @ParameterizedTest
    @MethodSource("testCases")
    public void test_to_math_ml(String asciimath, String mathml) {
        Assertions.assertThat(AsciiMath.of(asciimath).to_math_ml()).isEqualTo(mathml);
    }

    private static Stream<Arguments> testCases() {
        return Stream.of(
                Arguments.of(
                        "obrace(ubrace(t)_(a))^ba",
                        "<math display=\"block\"><mover><mover><mover><mrow><munder><munder><munder><mrow><mi>t</mi></mrow><mo>&#x23DF;</mo></munder></munder><mrow><mi>a</mi></mrow></munder></mrow><mo>&#x23DE;</mo></mover></mover><mi>b</mi></mover><mi>a</mi></math>"));
    }

}
