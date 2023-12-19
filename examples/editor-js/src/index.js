import 'file-loader?name=[name].[ext]!../index.html';

import("polymath-web").then(asciimath => {
  var input = document.getElementById("input");
  var output = document.getElementById("output");

  input.oninput = function() {
    output.innerHTML = asciimath.asciimath_to_mathml(input.value);
  }
});
