const id = "_builtin_channel_break";
const title = "Break Text Channel";
const description = "Break text channel into its component parts.";
const category = "Utility";

const pure;

const inputs = #{
    user: #{
        name: "channel",
        type: "struct",
        struct_tags: ["text_channel", "channel"],
    }
};
const outputs = #{
    guild: #{
        name: "guild",
        type: "struct",
        struct_tags: ["guild"],
        index: 0,
    },
    id: #{
        name: "id",
        type: "string",
        struct_tags: ["id"],
        index: 1
    },
    name: #{
        name: "name",
        type: "string",
        struct_tags: ["name"],
        index: 2
    },
    nsfw: #{
        name: "nsfw",
        type: "boolean",
        struct_tags: ["nsfw"],
        index: 3
    },
};

fn action(builder) {
    let channel = inv channel;

    out guild = channel.guild;
    out id = channel.id;
    out name = channel.name;
    out nsfw = channel.nsfw;
}