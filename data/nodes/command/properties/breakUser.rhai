const id = "_builtin_user_break";
const title = "Break User";
const description = "Break user into its component parts.";
const category = "Utility";

const pure;

const inputs = #{
    user: #{
        name: "user",
        type: "struct",
        struct_tags: ["user"],
    }
};
const outputs = #{
    bot: #{
        name: "bot",
        type: "boolean",
        index: 0
    },
    discriminator: #{
        name: "discriminator",
        type: "text",
        index: 1
    },
    id: #{
        name: "id",
        type: "text",
        index: 2
    },
    username: #{
        name: "username",
        type: "text",
        index: 3
    },
    tag: #{
        name: "tag",
        type: "text",
        index: 4
    },
    dm_channel: #{
        name: "DM channel",
        type: "struct",
        struct_tags: ["channel", "dm_channel"],
        index: 5
    },
};

fn action(builder) {
    let user = inv user;

    out bot = `${user}.bot`;
    out discriminator = `${user}.discriminator`;
    out id = `${user}.id`;
    out username = `${user}.username`;
    out tag = `${user}.tag`;
    out dm_channel = `${user}.dmChannel`;
}