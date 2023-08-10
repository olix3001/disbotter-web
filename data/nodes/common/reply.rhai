const id = "_builtin_interaction_reply";
const title = "Reply";
const description = "Reply to an interaction with a message or embed.";
const category = "Interaction";

const inputs = #{
    target: #{
        name: "Target",
        type: "struct",
        struct_tags: ["replyable"],
        index: 0
    },
    reply: #{
        name: "Reply",
        type: "struct",
        struct_tags: ["sendable"],
        index: 1
    },
};
const outputs = #{};

fn action(builder) {
    let target = inv target;
    let reply = inv reply;

    -> `await ${target}.reply(${reply});`
}