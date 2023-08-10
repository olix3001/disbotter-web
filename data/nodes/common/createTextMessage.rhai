const id = "_builtin_create_text_message";
const title = "Create Text Tessage";
const description = "Create a message from the given text.";
const category = "Interaction";

const pure;

const inputs = #{
    text: #{
        name: "Text",
        type: "text",
    },
};
const outputs = #{
    message: #{
        name: "Message",
        type: "struct",
        struct_type: "sendable_message"
    },
};

fn action(builder) {
    
}