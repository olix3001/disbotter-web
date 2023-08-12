const id = "_builtin_message_send";
const title = "Send Message";
const description = "Send a message to specified target.";
const category = "Interaction";

const inputs = #{
    target: #{
        name: "target",
        type: "struct",
        struct_tags: ["replyable"],
        index: 0
    },
    message: #{
        name: "message",
        type: "struct",
        struct_tags: ["sendable"],
        index: 1
    },
};
const outputs = #{};

fn action(builder) {
    let target = inv target;
    let reply = inv reply;

    let type = builder.get_comp_time_data("message", "type");

    if type == "text" {
        -> `await ${target}.reply(${reply});`
    }
}