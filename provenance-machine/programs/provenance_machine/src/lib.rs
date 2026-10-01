use anchor_lang::prelude::*;

declare_id!("ProveNance11111111111111111111111111111111");

#[program]
pub mod provenance_machine {
    use super::*;

    pub fn register_artifact(
        ctx: Context<RegisterArtifact>,
        artifact_hash: [u8; 32],
        version: u64,
    ) -> Result<()> {
        let record = &mut ctx.accounts.provenance_record;
        record.owner = ctx.accounts.owner.key();
        record.artifact_hash = artifact_hash;
        record.version = version;
        record.created_at = Clock::get()?.unix_timestamp;
        record.bump = ctx.bumps.provenance_record;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(artifact_hash: [u8; 32], version: u64)]
pub struct RegisterArtifact<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + ProvenanceRecord::INIT_SPACE,
        seeds = [b"provenance", owner.key().as_ref(), &artifact_hash],
        bump
    )]
    pub provenance_record: Account<'info, ProvenanceRecord>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct ProvenanceRecord {
    pub owner: Pubkey,
    pub artifact_hash: [u8; 32],
    pub version: u64,
    pub created_at: i64,
    pub bump: u8,
}
