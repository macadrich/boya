use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("program_id_here");

#[program]
pub mod boya {
    use super::*;

    pub fn init_boya(ctx: Context<Initialize>) -> Result<()> {
        let boya = &mut ctx.accounts.boya;
        boya.authority = *ctx.accounts.authority.key;

        Ok(())
    }

    pub fn create_post(ctx: Context<CreateBoya>, title: String, content: String) -> Result<()> {
        let boya = &mut ctx.accounts.boya;
        let post = &mut ctx.accounts.post;

        post.previous = boya.latest;
        post.blog = boya.key();
        post.title = title;
        post.content = content;
        post.timestamp = Clock::get().unwrap().unix_timestamp;

        boya.latest = post.key();
        boya.posts += 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, 
        seeds = [b"boya", authority.key().as_ref()], 
        bump, 
        payer = authority, 
        space = 8 + size_of::<Boya>())
    ]
    pub boya: Account<'info, Boya>,

    pub system_program: Program<'info, System>
}

#[account]
#[derive(Default)]
pub struct Boya {
    authority: Pubkey,
    latest: Pubkey,
    posts: u64
}

#[account]
#[derive(Default)]
pub struct Post {
    title: String,
    content: String,
    timestamp: i64,
    blog: Pubkey,
    previous: Pubkey
}

#[derive(Accounts)]
#[instruction(title: String, content: String)]
pub struct CreateBoya<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, has_one = authority)]
    pub boya: Account<'info, Boya>,

    #[account(init, 
        seeds = [b"post", boya.key().as_ref(), &boya.posts.to_be_bytes()], 
        bump, 
        payer = authority,
        space = 8 + size_of::<Post>() + title.as_bytes().len() - 40)]
    pub post: Account<'info, Post>,

    pub system_program: Program<'info, System>,
}
