const id = "_builtin_interaction_reply";
const title = "Reply";
const description = "Reply to an interaction with a message or embed.";
const category = "Interaction";

const inputs = #{
    target: #{
        name: "Target",
        type: "struct",
        struct_type: "discord_interaction"
    },
    reply: #{
        name: "Reply",
        type: "struct",
        struct_type: "sendable_message"
    },
};
const outputs = #{};

fn action(builder) {
    
}