const id = "_builtin_member_break";
const title = "Break Member";
const description = "Break member into its components.";
const category = "Utility";

const pure;

const inputs = #{
    member: #{
        name: "member",
        type: "struct",
        structTags: ["member"]
    }
};
const outputs = #{
    bannable: #{
        name: "bannable",
        type: "boolean",
        index: 0
    },
    kickable: #{
        name: "kickable",
        type: "boolean",
        index: 1
    },
    dm_channel: #{
        name: "DM channel",
        type: "struct",
        structTags: ["channel", "dm_channel"],
        index: 2
    },
    guild: #{
        name: "guild",
        type: "struct",
        structTags: ["guild"],
        index: 3
    },
    user: #{
        name: "user",
        type: "struct",
        structTags: ["user"],
        index: 4
    },
    id: #{
        name: "id",
        type: "text",
        index: 5
    },
    nickname: #{
        name: "nickname",
        type: "text",
        index: 6
    },
};

fn action(builder) {
    let member = inv member;
    out bannable = `${member}.bannable`;
    out kickable = `${member}.kickable`;
    out dm_channel = `${member}.dmChannel`;
    out guild = `${member}.guild`;
    out id = `${member}.id`;
    out nickname = `${member}.nickname`;
    out user = `${member}.user`;
}