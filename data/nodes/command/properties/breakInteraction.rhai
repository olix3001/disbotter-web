const id = "_builtin_interaction_break";
const title = "Break Interaction";
const description = "Break up an interaction into its components.";
const category = "Utility";

const pure;

const inputs = #{
    interaction: #{
        name: "interaction",
        type: "struct",
        struct_tags: ["interaction"],
    },
};
const outputs = #{
    guild: #{
        name: "guild",
        type: "struct",
        struct_tags: ["guild"],
        index: 0
    },
    channel: #{
        name: "channel",
        type: "struct",
        struct_tags: ["channel", "guild_channel", "message_sendable", "text_channel"],
        index: 1
    },
    member: #{
        name: "member",
        type: "struct",
        struct_tags: ["member", "user_resolvable"],
        index: 2
    },
    user: #{
        name: "user",
        type: "struct",
        struct_tags: ["user", "user_resolvable"],
        index: 3
    },
    ephemeral: #{
        name: "ephemeral",
        type: "boolean",
        index: 4
    },
    deferred: #{
        name: "deferred",
        type: "boolean",
        index: 5
    },
    replied: #{
        name: "replied",
        type: "boolean",
        index: 6
    },
};

fn action(builder) {
    let interaction = inv interaction;

    out guild = `${interaction}.guild`;
    out channel = `${interaction}.channel`;
    out member = `${interaction}.member`;
    out user = `${interaction}.user`;
    out ephemeral = `${interaction}.ephemeral`;
    out deferred = `${interaction}.deferred`;
    out replied = `${interaction}.replied`;
}