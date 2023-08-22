const id = "_builtin_interaction_reply";
const title = "Reply";
const description = "Reply to an interaction with a message or embed.";
const category = "Interaction";

const inputs = #{
    target: #{
        name: "target",
        type: "struct",
        struct_tags: ["replyable"],
        index: 0
    },
    reply: #{
        name: "reply",
        type: "struct",
        struct_tags: ["sendable"],
        index: 1
    },
    ephemeral: #{
        name: "ephemeral",
        type: "boolean",
        start_value: false,
        index: 2
    }
};
const outputs = #{};

fn action(builder) {
    let target = inv target;
    let reply = inv reply;
    let ephemeral = inv ephemeral;

    let type = builder.get_comp_time_data("reply", "type");

    if type == "text" {
        -> `await ${target}.reply({ content: ${reply}, ephemeral: ${ephemeral}});`
    }
}