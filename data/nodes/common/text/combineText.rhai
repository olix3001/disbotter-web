const id = "_builtin_text_combine";
const title = "Combine Text";
const description = "Combines two text values into one.";
const category = "Text";

const pure;

const inputs = #{
    a: #{
        name: "text A",
        type: "text",
        index: 0
    },
    b: #{
        name: "text B",
        type: "text",
        index: 1
    }
};
const outputs = #{
    result: #{
        name: "result",
        type: "text"
    }
};

fn action(builder) {
    out result = `(${inv a} + ${inv b})`;
}