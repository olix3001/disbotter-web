const id = "_builtin_math_subtract_numbers";
const title = "Subtract Numbers";
const description = "Subtract one number from another. (A - B)";
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
    out result = `(${inv a} - ${inv b})`;
}