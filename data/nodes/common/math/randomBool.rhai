const id = "_builtin_math_random_boolean";
const title = "Random Boolean";
const description = "Generates a random boolean value.";
const category = "Math";

const inputs = #{
    true_chance: #{
        name: "True Chance",
        type: "number",
        start_value: 0.5,
    }
};
const outputs = #{
    result: #{
        name: "result",
        type: "boolean",
    }
};

fn action(builder) {
    let true_chance = inv true_chance;
    out result = `Math.random() < ${true_chance}`;
}