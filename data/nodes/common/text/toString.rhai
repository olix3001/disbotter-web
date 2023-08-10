const id = "_builtin_any_to_text";
const title = "To Text";
const description = "Converts any value to text.";
const category = "Text";

const pure;

const inputs = #{
    value: #{
        name: "value",
        type: "any"
    },
};
const outputs = #{
    text: #{
        name: "text",
        type: "text"
    }
};

fn action(builder) {
    out text = `(${inv value}).toString()`;
}