const id = "_builtin_math_logic_and";
const title = "And";
const description = "Returns true if both inputs are true, false otherwise.";
const category = "Math";

const pure;

const inputs = #{
    a: #{
        name: "a",
        type: "boolean",
        index: 0
    },
    b: #{
        name: "b",
        type: "boolean",
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
    out result = `(${inv a} && ${inv b})`
}