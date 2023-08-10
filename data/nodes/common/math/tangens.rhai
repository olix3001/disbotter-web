const id = "_builtin_math_tangens";
const title = "Tangens";
const description = "Computes the tangens of a number. The number is in radians, not degrees.";
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
        name: "tan(x)",
        type: "number",
    }
};

fn action(builder) {
    out result = `Math.tan(${inv x})`;
}