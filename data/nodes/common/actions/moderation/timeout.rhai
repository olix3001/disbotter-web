const id = "_builtin_action_timeout";
const title = "Timeout Member";
const description = "Timeout a member from the server. (duration is in milliseconds)";
const category = "Actions";

const inputs = #{
    duration: #{
        name: "duration",
        type: "number",
        start_value: 3_600_000,
        index: 0
    },
    member: #{
        name: "member",
        type: "struct",
        struct_tags: ["member"],
        index: 1
    },
    reason: #{
        name: "reason",
        type: "text",
        start_value: "No reason provided",
        index: 2
    }
};
const outputs = #{};

fn action(builder) {
    let duration = inv duration;
    let member = inv member;
    -> `await ${member}.timeout(${duration}, ${inv reason});`;
}