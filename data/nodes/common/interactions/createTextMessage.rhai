const id = "_builtin_create_text_message";
const title = "Create Text Message";
const description = "Create a message from the given text.";
const category = "Interaction";

const pure;

const inputs = #{
    text: #{
        name: "text",
        type: "text",
    },
};
const outputs = #{
    message: #{
        name: "message",
        type: "struct",
        struct_tags: ["sendable"]
    },
};

fn action(builder) {
    let text = inv text;
    out message = text;
    builder.set_comp_time_data("message", "type", "text");
}