const id = "_builtin_math_random_integer";
const title = "Random Integer";
const description = "Generate a random integer between two values.";
const category = "Math";

const inputs = #{
    min: #{
        name: "minimum",
        type: "number",
        start_value: 0,
        index: 0
    },
    max: #{
        name: "maximum",
        type: "number",
        start_value: 100,
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
    let min = inv min;
    let max = inv max;
    out result = `Math.floor(Math.random() * (${max} - ${min} + 1)) + ${min}`;
}