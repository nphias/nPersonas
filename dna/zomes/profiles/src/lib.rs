extern crate hc_utils;

use hc_utils::WrappedDnaHash;
use hc_utils::WrappedAgentPubKey;
use hdk3::prelude::*;

mod profile;
mod utils;

use profile::{ProfileInit, ProfileOut}; //Profile, ProfileField,

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}

entry_defs![Path::entry_def(), profile::Profile::entry_def(), profile::ProfileField::entry_def()];

/** profile functions  **/
#[hdk_extern]
pub fn who_am_i(_: ()) -> ExternResult<WrappedAgentPubKey> {
    let agent_info = agent_info()?;
    Ok(WrappedAgentPubKey(agent_info.agent_initial_pubkey))    
}
#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct GetProfileOutput(Option<ProfileOut>);
#[hdk_extern]
pub fn get_profile(app: WrappedDnaHash) -> ExternResult<GetProfileOutput> {
    let profile_output = profile::get_profile(app)?;
    Ok(GetProfileOutput(profile_output))
}

#[hdk_extern]
pub fn create_profile(profile: ProfileInit) -> ExternResult<ProfileOut> {
    profile::create_profile(profile)
}

/*#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct GetPersonasOutput(Vec<String>);
#[hdk_extern]
pub fn get_personas(_: ()) -> ExternResult<GetPersonasOutput> {
    let personas = profile::get_persona_names()?;

    Ok(GetPersonasOutput(personas))
}







#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct GetAllProfilesOutput(Vec<AgentProfile>);
#[hdk_extern]
pub fn get_all_profiles(_: ()) -> ExternResult<GetAllProfilesOutput> {
    let agent_profiles = profile::get_all_profiles()?;

    Ok(GetAllProfilesOutput(agent_profiles))
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct GetAgentProfileOutput(Option<AgentProfile>);
#[hdk_extern]
pub fn get_agent_profile(agent_pub_key: WrappedAgentPubKey) -> ExternResult<GetAgentProfileOutput> {
    let agent_profile = profile::get_agent_profile(agent_pub_key)?;

    Ok(GetAgentProfileOutput(agent_profile))
}

#[hdk_extern]
pub fn get_my_profile(_: ()) -> ExternResult<GetAgentProfileOutput> {
    let agent_info = agent_info()?;

    let agent_profile =
        profile::get_agent_profile(WrappedAgentPubKey(agent_info.agent_initial_pubkey))?;

    Ok(GetAgentProfileOutput(agent_profile))
}
*/