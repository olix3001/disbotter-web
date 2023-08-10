const id = "_builtin_random_boolean";
const title = "Random Boolean";
const description = "Generates a random boolean value.";
const category = "Interaction";

const inputs = #{
    true_chance: #{
        name: "True Chance",
        type: "number",
        start_value: 0.5,
    }
};
const outputs = #{
    result: #{
        name: "Result",
        type: "boolean",
    }
};

fn action(builder) {
    
}