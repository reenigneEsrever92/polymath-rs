import * as polymath from "polymath-web";

var input = document.getElementById("input");
var output = document.getElementById("output");

input.oninput = function() {
  output.innerHTML = polymath.asciimath_to_mathml(input.value);
}
