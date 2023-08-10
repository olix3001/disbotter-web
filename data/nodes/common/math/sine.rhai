const id = "_builtin_math_sine";
const title = "Sine";
const description = "Compute the sine of a number. This uses radians, not degrees.";
const category = "Math";

const pure;

const inputs = #{
    x: #{
        name: "x",
        type: "number",
    },
};
const outputs = #{
    result: #{
        name: "sin(x)",
        type: "number",
    }
};

fn action(builder) {
    out result = `Math.sin(${inv x})`;
}