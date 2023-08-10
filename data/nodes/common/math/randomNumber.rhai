const id = "_builtin_math_random_number";
const title = "Random Number";
const description = "Generate a random number in the specified range.";
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
        start_value: 1,
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
    out result = `(Math.random() * (${max} - ${min}) + ${min})`
}