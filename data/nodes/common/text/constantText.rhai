const id = "_builtin_text_constant";
const title = "Constant text";
const description = "Constant text.";
const category = "Text";

const pure;

const inputs = #{
    text: #{
        name: "text",
        type: "text",
    }
};
const outputs = #{
    result: #{
        name: "value",
        type: "text",
    }
};

fn action(builder) {
    out result = (inv text);
}