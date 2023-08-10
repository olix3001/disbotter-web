const id = "_builtin_math_cosine";
const title = "Cosine";
const description = "Compute the cosine of a number. This uses radians, not degrees.";
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
        name: "cos(x)",
        type: "number",
    }
};

fn action(builder) {
    out result = `Math.cos(${inv x})`;
}