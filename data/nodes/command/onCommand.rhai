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
        struct_type: "discord_interaction"
    }
};

fn action(builder) {
    
}