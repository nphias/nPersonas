use crate::utils;
use hc_utils::WrappedAgentPubKey;
//use hdk3::prelude::link::Link;
use hdk3::prelude::*;
use std::convert::{TryFrom, TryInto};

#[hdk_entry(id = "persona", visibility = "private")]
#[derive(Clone)]
pub struct Persona {
    pub name: String,
    pub agent_pub_key: AgentPubKey,
}

/*#[hdk_entry(id = "personadata", visibility = "private")]
#[derive(Clone)]
pub struct PersonaData {
    persona: AnyDhtHash,
    fields: vec<PersonaField>
}*/

#[hdk_entry(id = "personaData", visibility = "private")]
#[derive(Clone)]
pub struct PersonaData {
	aliases: Vec<String>,
	data: String
}

//DTOs

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct PersonaField {
    persona_id: WrappedAgentPubKey,
    data_id: EntryHash,
    key: String,
    value: Option<String>,
    aliases: Vec<String>
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct FieldNames {
 pub fields: Vec<String> 
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct FieldData {
 pub key: String,
 pub value: String
}


#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct AgentPersona {
    pub name: String,
    pub agent_pub_key: WrappedAgentPubKey,
}

/** functions **/

//gets the persona info ..  first time use creates the Persona... later only that agent can use the zome
pub fn get_persona(_:()) -> ExternResult<Option<AgentPersona>> {
    //let links_agent = get_links(agent_info()?.agent_latest_pubkey.into(), Some(LinkTag::new("username")))?;
    let pub_key = agent_info()?.agent_latest_pubkey.clone();
    //let agent_address: AnyDhtHash = pub_key.into();//agent_info()?.agent_initial_pubkey.clone().into();

    let path = Path::from("owner");
    path.ensure()?;
    //let app_address: AnyDhtHash = app_key.into_hash(); 
    let links = get_links(path.hash()?, None)?;//, tag_to_app_key(app.clone())?)?;
    let inner_links = links.into_inner();

    if inner_links.len() == 0 {
        let agent_persona = create_persona();
        return Ok(Some(agent_persona?));
    }
    let link = inner_links[0].clone();
    let persona: Persona = utils::try_get_and_convert(link.target)?;
    if persona.agent_pub_key != pub_key {
        return Ok(None); // should return a no permission error
    }
    let agent_persona = AgentPersona {
        name: persona.name,
        agent_pub_key: WrappedAgentPubKey(pub_key)
    };
    Ok(Some(agent_persona))
}

//gets the default persona
pub fn get_fields(fieldnames:FieldNames) -> ExternResult<Vec<PersonaField>> {

    //first verify agent
    let pub_key = agent_info()?.agent_latest_pubkey.clone();
    //let agent_pub_key = WrappedAgentPubKey(agent_info.agent_initial_pubkey); //wrapped_agent_pub_key.0.clone();
    //let agent_address: AnyDhtHash = pub_key.into();
    let mut result:Vec<PersonaField> = Vec::new(); 
    for name in fieldnames.fields { 
        let path = Path::from(format!("all_data.{}",name)); 
        path.ensure()?;
        let links = get_links(path.hash()?, None)?;//, tag_to_app_key(app.clone())?)?;
        let inner_links = links.into_inner();
        if inner_links.len() > 0 {
            let link = inner_links[0].clone();
            let pd: PersonaData = utils::try_get_and_convert(link.target)?;
            let pd_hash = hash_entry(&pd.clone())?;
            result.push( PersonaField {
                persona_id: WrappedAgentPubKey(pub_key.clone()),
                data_id: pd_hash,
                key: name,
                value: Some(pd.data),
                aliases: pd.aliases
            })
        }
    }
    Ok(result)

}



pub fn add_field(field: FieldData) -> ExternResult<PersonaField> {

    //first verify agent
    let pub_key = agent_info()?.agent_latest_pubkey.clone();
    //let agent_pub_key = WrappedAgentPubKey(agent_info.agent_initial_pubkey); //wrapped_agent_pub_key.0.clone();
    //let agent_address: AnyDhtHash = pub_key.into();
    let path = Path::from(format!("all_data.{}",&field.key)); 
        path.ensure()?;
        let links = get_links(path.hash()?, None)?;//, tag_to_app_key(app.clone())?)?;
        let inner_links = links.into_inner();
        if inner_links.len() > 0 {
            let link = inner_links[0].clone();
            let pd: PersonaData = utils::try_get_and_convert(link.target)?;
            let pd_hash = hash_entry(&pd.clone())?;
            Ok(PersonaField {
                persona_id: WrappedAgentPubKey(pub_key.clone()),
                data_id: pd_hash,
                key: field.key.clone(),
                value: Some(pd.data),
                aliases: pd.aliases
            })
        } else {
            let newdata = PersonaData { 
                aliases: Vec::new(),
                data: field.value.clone()
            };
            create_entry(&newdata);
            let newdata_hash = hash_entry(&newdata)?;
            create_link(
                path.hash()?,
                newdata_hash.clone(),
                LinkTag("persona".into())
            )?;
            Ok( PersonaField {
                persona_id: WrappedAgentPubKey(pub_key.clone()),
                data_id: newdata_hash.clone(),
                key: field.key.clone(),
                value: Some(newdata.data),
                aliases: newdata.aliases
            })
        }
}


/* 
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
}*/

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


/*pub fn get_all_personas() -> ExternResult<Vec<AgentPersona>> {
    let path = Path::from("all_personas");
    path.ensure()?;

    let links = get_links!(path.hash()?)?;

    links
        .into_inner()
        .into_iter()
        .map(|link| get_agent_profile_from_link(link))
        .collect()
}*/


/** private helpers */

fn create_persona() -> ExternResult<AgentPersona> {
    let pub_key: AgentPubKey = agent_info()?.agent_initial_pubkey.clone().into();
    let persona = Persona { 
        name: "default".into(),
        agent_pub_key: pub_key
    };
    create_entry(&persona.clone());
    let persona_hash = hash_entry(&persona.clone());
    let path = Path::from("owner");
    path.ensure();
    create_link(
        path.hash()?,
        persona_hash.clone()?,
        LinkTag("persona".into())
    )?;
    let pub_key = agent_info()?.agent_initial_pubkey.clone().into();
    let persona = AgentPersona {
        name: "default".into(),
        agent_pub_key: WrappedAgentPubKey(pub_key)
    };
    Ok(persona)
}

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
/*
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

*/













/* 
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
*/
