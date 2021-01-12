use crate::utils;
use hc_utils::WrappedAgentPubKey;
use hdk3::prelude::link::Link;
use hdk3::prelude::*;
use std::convert::{TryFrom, TryInto};

#[hdk_entry(id = "persona", visibility = "private")]
#[derive(Clone)]
pub struct Persona {
    pub name: String,
    pub agent_pub_key: AgentPubKey,
}

#[hdk_entry(id = "personadata", visibility = "private")]
#[derive(Clone)]
pub struct PersonaData {
    persona: AnyDhtHash,
    fields: vec<PersonaField>
}
#[hdk_entry(id = "personafield", visibility = "private")]
#[derive(Clone)]
pub struct PersonaField {
	name: String,
	data: String
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct AgentPersona {
    pub name: String,
    pub agent_pub_key: WrappedAgentPubKey,
}

/** functions **/

//gets the default persona
pub fn get_persona(_:()) -> ExternResult<AgentPersona> {
    let agent_info = agent_info!()?;
    let agent_pub_key = WrappedAgentPubKey(agent_info.agent_initial_pubkey)[0]; //wrapped_agent_pub_key.0.clone();
    let agent_address: AnyDhtHash = agent_pub_key.clone().into();
    let links = get_links!(agent_address.into(), link_tag("persona")?)?;
    let inner_links = links.into_inner();
    if inner_links.len() == 0 {
        let agent_persona = create_persona();
        return Ok(agent_persona);
    } else {
        let link = inner_links[0].clone();
        let persona: Persona = utils::try_get_and_convert(link.target)?;
    }
    let agent_persona = AgentPersona {
        name: persona.name,
        agent_pub_key
    };
    Ok(agent_persona)
}

pub fn add_field(field: PersonaField) -> ExternResult<Entryhash> {
    let newfield = PersonaField { 
        name: field.name,
        data: field.data
    };
    create_entry!(newfield.clone())?;
    let agent_info = agent_info!()?;
    let newfield_hash = hash_entry!(newfield.clone())?;
    let agent_pub_key = WrappedAgentPubKey(agent_info.agent_initial_pubkey.clone());
    let agent_address: AnyDhtHash = agent_info.agent_initial_pubkey.clone().into();


    create_link!(
        agent_address.into(),
        persona_hash.clone(),
        link_tag("persona")?
    )?;
}

pub fn get_persona_profile(app_hash:WrappedDnaHash, agent_pub_key: WrappedAgentPubKey) -> ExternResult<Option<AgentPersonaProfile>> {
    let path = Path::from(format!("all_personas.{}",agent_pub_key)); 
    path.ensure()?;

    let links = get_links!(path.hash()?, pub_key_to_tag(app_hash.clone())?)?;
    let inner_links = links.into_inner();
    if inner_links.len() == 0 {
        return Ok(None);
    }
    let link = inner_links[0].clone();

    let persona_profile: PersonaProfile = utils::try_get_and_convert(link.target)?;

    let agent_persona_profile = AgentPersonaProfile {
        agent_pub_key,
        persona_profile
    };
    Ok(Some(agent_persona_profile))
}

/**pub fn get_persona(_:()) -> ExternResult<AgentPersona> {
    let path = Path::from("all_personas"); //should be a declared const 
    path.ensure()?;
    //should take the active agent key
    let agent_info = agent_info!()?;
    let agent_pub_key = WrappedAgentPubKey(agent_info.agent_initial_pubkey);
    
    let links = get_links!(path.hash()?, pub_key_to_tag(agent_pub_key.clone())?)?;
    let inner_links = links.into_inner();
    if inner_links.len() == 0 {
        let agent_persona = create_persona();
        return Ok(agent_persona);
    }
    let link = inner_links[0].clone();
    
    let persona: Persona = utils::try_get_and_convert(link.target)?;
    let agent_persona = AgentPersona {
        name: persona.name,
        agent_pub_key
    };

    Ok(Some(agent_persona))
}**/


pub fn get_all_personas() -> ExternResult<Vec<AgentPersona>> {
    let path = Path::from("all_personas");
    path.ensure()?;

    let links = get_links!(path.hash()?)?;

    links
        .into_inner()
        .into_iter()
        .map(|link| get_agent_profile_from_link(link))
        .collect()
}


/** private helpers */

fn create_persona() -> AgentPersona {
    let agent_info = agent_info!()?;
    let persona = Persona { 
        name: "default".into(),
        agent_pub_key: agent_info.agent_initial_pubkey.clone()
    };
    create_entry!(persona.clone())?;
    let persona_hash = hash_entry!(persona.clone())?;
    let agent_pub_key = WrappedAgentPubKey(agent_info.agent_initial_pubkey.clone());
    let agent_address: AnyDhtHash = agent_info.agent_initial_pubkey.clone().into();


    create_link!(
        agent_address.into(),
        persona_hash.clone(),
        link_tag("persona")?
    )?;

    AgentPersona {
        name: "default".into(),
        agent_pub_key
    };

/*

    let path_all =  Path::from("all_personas"); //should be a declared const 
    let path_persona = Path::from(format!("all_personas.{}", agent_pub_key.clone()));
    path.ensure()?;
    

    create_link!(
        path_all.hash()?,
        persona_hash.clone(),
        pub_key_to_tag(agent_pub_key.clone())?
    )?;
    create_link!(
        path_all.hash()?,
        persona_hash.clone(),
        pub_key_to_tag(agent_pub_key.clone())?
    )?;

    AgentPersona {
        name: "default".into(),
        agent_pub_key
    };
    */
}

fn get_agent_persona_from_link(link: Link) -> ExternResult<AgentPersona> {
    let persona_hash = link.target;

    let persona: Persona = utils::try_get_and_convert(persona_hash)?;
    let agent_pub_key = tag_to_pub_key(link.tag)?;

    let agent_persona = AgentPersona {
        agent_pub_key,
        profile,
    };

    Ok(agent_profile)
}

fn pub_key_to_tag(agent_pub_key: WrappedAgentPubKey) -> ExternResult<LinkTag> {
    let sb: SerializedBytes = agent_pub_key.try_into()?;

    Ok(LinkTag(sb.bytes().clone()))
}

fn tag_to_pub_key(tag: LinkTag) -> ExternResult<WrappedAgentPubKey> {
    let sb = SerializedBytes::from(UnsafeBytes::from(tag.0));

    let pub_key = WrappedAgentPubKey::try_from(sb)?;

    Ok(pub_key)
}

#[derive(Serialize, Deserialize, SerializedBytes)]
struct StringLinkTag(String);
pub fn link_tag(tag: &str) -> ExternResult<LinkTag> {
    let sb: SerializedBytes = StringLinkTag(tag.into()).try_into()?;
    Ok(LinkTag(sb.bytes().clone()))
}
















pub fn create_profile(persona: Persona) -> ExternResult<AgentPersona> {
    let agent_info = agent_info!()?;

    create_entry!(profile.clone())?;

    let profile_hash = hash_entry!(profile.clone())?;

    let path = all_profiles_path();

    path.ensure()?;

    let wrapped_agent_pub_key = WrappedAgentPubKey(agent_info.agent_initial_pubkey.clone());

    create_link!(
        path.hash()?,
        profile_hash.clone(),
        pub_key_to_tag(wrapped_agent_pub_key)?
    )?;

    let agent_profile = AgentProfile {
        agent_pub_key: WrappedAgentPubKey(agent_info.agent_initial_pubkey),
        profile
    };

    Ok(agent_profile)
}


pub fn get_all_profiles() -> ExternResult<Vec<AgentProfile>> {
    let path = all_profiles_path();

    let links = get_links!(path.hash()?)?;

    links
        .into_inner()
        .into_iter()
        .map(|link| get_agent_profile_from_link(link))
        .collect()
}

pub fn get_agent_profile(agent_pub_key: WrappedAgentPubKey) -> ExternResult<Option<AgentProfile>> {
    let path = all_profiles_path();

    let links = get_links!(path.hash()?, pub_key_to_tag(agent_pub_key.clone())?)?;

    let inner_links = links.into_inner();

    if inner_links.len() == 0 {
        return Ok(None);
    }

    let link = inner_links[0].clone();

    let profile: Profile = utils::try_get_and_convert(link.target)?;

    let agent_profile = AgentProfile {
        agent_pub_key,
        profile
    };

    Ok(Some(agent_profile))
}

/** Private helpers */

fn all_profiles_path() -> Path {
    Path::from("all_profiles")
}

fn get_agent_profile_from_link(link: Link) -> ExternResult<AgentProfile> {
    let profile_hash = link.target;

    let profile: Profile = utils::try_get_and_convert(profile_hash)?;
    let agent_pub_key = tag_to_pub_key(link.tag)?;

    let agent_profile = AgentProfile {
        agent_pub_key,
        profile,
    };

    Ok(agent_profile)
}

fn pub_key_to_tag(agent_pub_key: WrappedAgentPubKey) -> ExternResult<LinkTag> {
    let sb: SerializedBytes = agent_pub_key.try_into()?;

    Ok(LinkTag(sb.bytes().clone()))
}

fn tag_to_pub_key(tag: LinkTag) -> ExternResult<WrappedAgentPubKey> {
    let sb = SerializedBytes::from(UnsafeBytes::from(tag.0));

    let pub_key = WrappedAgentPubKey::try_from(sb)?;

    Ok(pub_key)
}
