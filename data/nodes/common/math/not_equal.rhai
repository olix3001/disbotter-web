const id = "_builtin_math_cmp_neq";
const title = "Not equal";
const description = "Check if two values are not equal.";
const category = "Math";

const pure;

const inputs = #{
    a: #{
        name: "a",
        type: "any",
        index: 0
    },
    b: #{
        name: "b",
        type: "any",
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
    out result = `(${inv a} != ${inv b})`
}