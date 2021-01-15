extern crate hc_utils;

use hc_utils::WrappedAgentPubKey;
use hc_utils::WrappedDnaHash;
use hdk3::prelude::*;

mod persona;
mod persona_profile;
mod utils;

use persona::{AgentPersona, Persona};
use persona_profile::{AgentPersonaProfile, PersonaProfile};


pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}

entry_defs![Path::entry_def(), persona::Persona::entry_def(), persona::PersonaProfile::entrydef()];


//temp hack for bridging
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    // grant unrestricted access to accept_cap_claim so other agents can send us claims
    let mut functions: GrantedFunctions = HashSet::new();
    functions.insert((zome_info!()?.zome_name, "get_all_personas".into()));
    // functions.insert((zome_info!()?.zome_name, "needs_cap_claim".into()));
    create_cap_grant!(
        CapGrantEntry {
            tag: "".into(),
            // empty access converts to unrestricted
            access: ().into(),
            functions,
        }
    )?;
    Ok(InitCallbackResult::Pass)
}

/** persona structs **/

//#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
//pub struct GetAgentPersonaOutput(Option<AgentPersona>);

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct GetAllPersonasOutput(Vec<AgentPersona>);

//#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
//pub struct GetAgentPersonaProfileOutput(Option<AgentPersonaProfile>);

/** persona functions **/

/** 
// pub create_persona(persona:Persona)
cap get_persona()
cap set_persona_name(name:string)
cap get_all_personas()

cap get_persona_profiles()
pub get_persona_profile(app_hash:DnaHash)
pub create_persona_profile(persona_profile:PersonaProfile)
cap remove_persona_profile(profile_hash: hashstring)

**/


#[hdk_extern]
pub fn who_am_i(_: ()) -> ExternResult<WrappedAgentPubKey> {
    let agent_info = agent_info!()?;
    Ok(WrappedAgentPubKey(agent_info.agent_initial_pubkey))    
}

#[hdk_extern]
pub fn get_persona(_: ()) -> ExternResult<AgentPersona> {
    let agent_persona = persona::get_persona()?;
    Ok(agent_persona)
}

#[hdk_extern]
fn get_all_personas(_: ()) -> ExternResult<GetAllPersonasOutput> {
    let agent_personas = persona::get_all_personas()?;
    Ok(GetAllPersonasOutput(agent_personas))
}

pub fn get_persona_profile(app_hash:WrappedDnaHash) -> ExternResult<Option<AgentPersonaProfile>> {
    let agent_key = who_am_i();
    let agent_persona_profile = persona::get_persona_profile(app_hash,agent_key)?;
    Ok(agent_persona_profile)
}

#[hdk_extern]
pub fn create_persona(_: ()) -> ExternResult<AgentPersona> {
    let agent_persona = persona::get_persona()?;
    Ok(agent_persona)
}

/** 

#[hdk_extern]
pub fn get_agent_profile(agent_pub_key: WrappedAgentPubKey) -> ExternResult<GetAgentProfileOutput> {
    let agent_profile = profile::get_agent_profile(agent_pub_key)?;

    Ok(GetAgentProfileOutput(agent_profile))
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct GetAllProfilesOutput(Vec<AgentProfile>);
#[hdk_extern]
pub fn get_all_profiles(_: ()) -> ExternResult<GetAllProfilesOutput> {
    let agent_profiles = profile::get_all_profiles()?;

    Ok(GetAllProfilesOutput(agent_profiles))
}
**/


