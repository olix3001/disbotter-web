const id = "_builtin_math_multiply_numbers";
const title = "Multiply Numbers";
const description = "Multiply two numbers together. (A * B)";
const category = "Math";

const pure;

const inputs = #{
    a: #{
        name: "A",
        type: "number",
        index: 0
    },
    b: #{
        name: "B",
        type: "number",
        index: 1
    }
};
const outputs = #{
    result: #{
        name: "result",
        type: "number",
    }
};

fn action(builder) {
    out result = `(${inv a} * ${inv b})`;
}