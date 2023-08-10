const id = "_builtin_math_bool_invert";
const title = "Invert Boolean";
const description = "Invert a boolean value.";
const category = "Math";

const pure;

const inputs = #{
    value: #{
        name: "value",
        type: "boolean",
    },
};
const outputs = #{
    result: #{
        name: "result",
        type: "boolean",
    }
};

fn action(builder) {
    out result = `!${inv a}`
}