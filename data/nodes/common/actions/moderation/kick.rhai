const id = "_builtin_action_kick";
const title = "Kick Member";
const description = "Kick a member from the server.";
const category = "Actions";

const inputs = #{
    member: #{
        name: "user",
        type: "struct",
        struct_tags: ["user_resolvable"],
        index: 0
    },
    reason: #{
        name: "reason",
        type: "text",
        start_value: "No reason provided.",
        index: 1
    }
};
const outputs = #{};

fn action(builder) {
    let member = inv member;
    -> `await ${inv ___guild}.members.kick(${member}, ${reason});`;
}