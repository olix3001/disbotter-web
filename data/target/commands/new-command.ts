import {Command, LocalizedTranslations} from "disbotter"
import {CommandInteraction, SlashCommandBuilder} from "discord.js"
export default class extends Command {
   public readonly builder = new SlashCommandBuilder()
       .setName("new-command")
       .setDescription("Description");

   public async handle(__TRANSLATIONS__: LocalizedTranslations, __INTERACTION__: CommandInteraction): Promise<void> {
        const __io_N_builtin_interaction_break_Oguild_52a2242052a22420 = __INTERACTION__.guild
        const __io_N_builtin_interaction_break_Ochannel_7f0300d47f0300d4 = __INTERACTION__.channel
        const __io_N_builtin_interaction_break_Omember_212e90e0212e90e0 = __INTERACTION__.member
        const __io_N_builtin_interaction_break_Ouser_06cc64bf06cc64bf = __INTERACTION__.user
        const __io_N_builtin_interaction_break_Oephemeral_fe623f7afe623f7a = __INTERACTION__.ephemeral
        const __io_N_builtin_interaction_break_Odeferred_4bc62c6f4bc62c6f = __INTERACTION__.deferred
        const __io_N_builtin_interaction_break_Oreplied_5e4c357e5e4c357e = __INTERACTION__.replied
        const __io_N_builtin_user_break_Obot_e7878770e7878770 = __io_N_builtin_interaction_break_Ouser_06cc64bf06cc64bf.bot
        const __io_N_builtin_user_break_Odiscriminator_b189f958b189f958 = __io_N_builtin_interaction_break_Ouser_06cc64bf06cc64bf.discriminator
        const __io_N_builtin_user_break_Ousername_f66d915ef66d915e = __io_N_builtin_interaction_break_Ouser_06cc64bf06cc64bf.username
        const __io_N_builtin_user_break_Otag_489c37d8489c37d8 = __io_N_builtin_interaction_break_Ouser_06cc64bf06cc64bf.tag
        const __io_N_builtin_user_break_Odm_channel_91faef6f91faef6f = __io_N_builtin_interaction_break_Ouser_06cc64bf06cc64bf.dmChannel
        const __io_N_builtin_text_combine_Oresult_2f3d2d0e2f3d2d0e = ("Hello" + __io_N_builtin_user_break_Ousername_f66d915ef66d915e)
        const __io_N_builtin_create_text_message_Omessage_ab577eeeab577eee = __io_N_builtin_text_combine_Oresult_2f3d2d0e2f3d2d0e
        await __INTERACTION__.reply(__io_N_builtin_create_text_message_Omessage_ab577eeeab577eee);
   }
}
