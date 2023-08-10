const id = "_builtin_math_cmp_gt";
const title = "Greater than";
const description = "Compare two values and return true if the first is greater than the second.";
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
    out result = `(${inv a} > ${inv b})`
}