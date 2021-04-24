//extern crate hc_utils;

//use hc_utils::WrappedDnaHash;
use holo_hash::AgentPubKeyB64;

use hdk::prelude::*;

mod profile;
mod utils;

use profile::{ProfileData, ProfileSpec}; //Profile, ProfileField,

pub fn err(reason: &str) -> WasmError {
    WasmError::Guest(String::from(reason))
}

entry_defs![Path::entry_def(), profile::Profile::entry_def(), profile::ProfileField::entry_def()];

/** profile functions  **/
#[hdk_extern]
pub fn who_am_i(_: ()) -> ExternResult<AgentPubKeyB64> {
    let agent_info = agent_info()?;
    Ok(AgentPubKeyB64::from(agent_info.agent_initial_pubkey))    
}

#[hdk_extern]
pub fn get_profile(meta: ProfileSpec) -> ExternResult<Option<ProfileData>> {
    let profile_output = profile::get_profile(meta)?;
    Ok(profile_output)
}
 
#[hdk_extern]
pub fn create_profile(profile: ProfileSpec) -> ExternResult<EntryHash> {
    profile::create_profile(profile)
}



/*#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GetPersonasOutput(Vec<String>);
#[hdk_extern]
pub fn get_personas(_: ()) -> ExternResult<GetPersonasOutput> {
    let personas = profile::get_persona_names()?;

    Ok(GetPersonasOutput(personas))
}







#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GetAllProfilesOutput(Vec<AgentProfile>);
#[hdk_extern]
pub fn get_all_profiles(_: ()) -> ExternResult<GetAllProfilesOutput> {
    let agent_profiles = profile::get_all_profiles()?;

    Ok(GetAllProfilesOutput(agent_profiles))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GetAgentProfileOutput(Option<AgentProfile>);
#[hdk_extern]
pub fn get_agent_profile(agent_pub_key: AgentPubKeyB64) -> ExternResult<GetAgentProfileOutput> {
    let agent_profile = profile::get_agent_profile(agent_pub_key)?;

    Ok(GetAgentProfileOutput(agent_profile))
}

#[hdk_extern]
pub fn get_my_profile(_: ()) -> ExternResult<GetAgentProfileOutput> {
    let agent_info = agent_info()?;

    let agent_profile =
        profile::get_agent_profile(AgentPubKeyB64(agent_info.agent_initial_pubkey))?;

    Ok(GetAgentProfileOutput(agent_profile))
}
*/