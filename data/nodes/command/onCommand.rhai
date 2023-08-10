const id = "_builtin_on_command";
const title = "On Command";
const description = "Triggered when a command is executed.";
const category = "Events";

const noFlowIn;

const inputs = #{};
const outputs = #{
    interaction: #{
        name: "Interaction",
        type: "struct",
        struct_tags: ["interaction", "replyable"]
    }
};

fn action(builder) {
    builder.bind_io("___interaction", "interaction");
}