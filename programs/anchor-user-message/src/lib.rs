use anchor_lang::prelude::*;

declare_id!("GxZvwLovQwcaTKFjyo37mp1KNqbzZPVNFNjbEaWacCES");

const ANCHOR_DISCRIMINATOR: usize = 8;
const PUBKEY_SIZE: usize = 32;
const STRING_PREFIX_LENGTH: usize = 4;
const MAX_NAME_LENGTH: usize = 10;
const MAX_MESSAGE_LENGTH: usize = 20;

#[program]
pub mod anchor_user_message {
    use super::*;

    pub fn add_user_message(
        ctx: Context<AddUserMessage>,
        name: String,
        message: String,
    ) -> Result<()> {
        require!(name.len() <= MAX_NAME_LENGTH, UserMessageError::NameTooLong);
        require!(
            message.len() < MAX_MESSAGE_LENGTH,
            UserMessageError::MessageTooLong
        );

        let user_message = &mut ctx.accounts.user_message;

        user_message.name = name;
        user_message.message = message;

        Ok(())
    }

    pub fn update_user_message(
        ctx: Context<UpdateUserMessage>,
        name: String,
        message: String,
    ) -> Result<()> {
        let user_message = &mut ctx.accounts.user_message;

        user_message.name = name;
        user_message.message = message;

        Ok(())
    }

    pub fn delete_user_message(_ctx: Context<DeleteUserMessage>, name: String) -> Result<()> {
        msg!("Deleted the user {} Message", name);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String, message: String)]
pub struct AddUserMessage<'info> {
    #[account(
        init,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = UserMessageState::INIT_SPACE + name.len() + message.len(),
    )]
    pub user_message: Account<'info, UserMessageState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String, message: String)]
pub struct UpdateUserMessage<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = UserMessageState::INIT_SPACE + name.len() + message.len(),
        realloc::payer = initializer,
        realloc::zero = true,
    )]
    pub user_message: Account<'info, UserMessageState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name:String)]
pub struct DeleteUserMessage<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        close = initializer,
    )]
    pub user_message: Account<'info, UserMessageState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserMessageState {
    pub name: String,    // 4 + len()
    pub message: String, // 4 + len()
}

impl Space for UserMessageState {
    const INIT_SPACE: usize =
        ANCHOR_DISCRIMINATOR + PUBKEY_SIZE + STRING_PREFIX_LENGTH + STRING_PREFIX_LENGTH;
}

#[error_code]
enum UserMessageError {
    #[msg("User name too long")]
    NameTooLong,
    #[msg("User Message too long")]
    MessageTooLong,
}
