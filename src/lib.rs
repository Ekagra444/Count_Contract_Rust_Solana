use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    entrypoint,
    msg, pubkey::Pubkey
};
#[derive(BorshDeserialize,BorshSerialize)]
enum Instruction_type{
    Increament(u32),
    Decreament(u32)
}
#[derive(BorshDeserialize,BorshSerialize)]
struct Counter{
    count:u32,
}
entrypoint!(counter_contract);
pub fn counter_contract(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data: &[u8]
)->ProgramResult{
    let acc=next_account_info(&mut accounts.iter())?;
    let instruction_type = Instruction_type::try_from_slice(instruction_data)?;
    let mut counter=Counter::try_from_slice(&acc.data.borrow())?;
    match instruction_type{
        Instruction_type::Increament(val)=>{
            counter.count+=val;
        },
        Instruction_type::Decreament(val)=>{
            counter.count-=val;
        }
    }   
    counter.serialize(&mut *acc.data.borrow_mut())?;
    ProgramResult::Ok(())
}