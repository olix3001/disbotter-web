const id = "_builtin_math_cmp_ge";
const title = "Greater or Equal";
const description = "Check if first value is greater than or equal to second value.";
const category = "Math";

const pure;

const inputs = #{
    a: #{
        name: "a",
        type: "number",
        index: 0
    },
    b: #{
        name: "b",
        type: "number",
        index: 1
    }
};
const outputs = #{
    result: #{
        name: "result",
        type: "boolean",
    }
};

fn action(builder) {
    out result = `(${inv a} >= ${inv b})`
}