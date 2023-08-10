const id = "_builtin_get_command_interaction";
const title = "Get Interaction";
const description = "Gets the interaction that triggered the command.";
const category = "Interaction";

const pure;

const inputs = #{};
const outputs = #{
    interaction: #{
        name: "interaction",
        type: "struct",
        struct_tags: ["interaction", "replyable"]
    }
};

fn action(builder) {
    builder.bind_io("___interaction", "interaction");
}