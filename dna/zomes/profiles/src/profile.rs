use crate::utils;
//use hc_utils::wrappers::*;
//use hdk3::prelude::link::Link;
use hdk3::prelude::*;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;


#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub enum AccessType {
    PUBLIC,
    PERSONAL,
    PRIVATE
}

impl FromStr for AccessType {

    type Err = ();

    fn from_str(input: &str) -> Result<AccessType, Self::Err> {
        match input {
            "PUBLIC"  => Ok(AccessType::PUBLIC),
            "PERSONAL"  => Ok(AccessType::PERSONAL),
            "PRIVATE"  => Ok(AccessType::PRIVATE),
            _      => Err(()),
        }
    }
}

#[hdk_entry(id = "profile", visibility = "private")]
#[derive(Clone)]
pub struct Profile {
    uuid: String,
    app_name: String,
    app_hash: String,
    app_version: String,
    expiry: u32,
    enabled: bool,
    fields: Vec<ProfileField>
}

#[hdk_entry(id = "profilefield", visibility = "private")]
#[derive(Clone)]
pub struct ProfileField {
    name: String,
    display_name: String,
    required: bool,
    description: String,
    access: AccessType,
    schema: String,
    mapping:  Option<String> //Option<AnyDhtHash>
}

// DTO - Data Transfer Objects Input

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    uuid: String,
    app_name: String,
    app_hash: String,
    app_version: String,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSpec {
    uuid: String,
    app_name: String,
    app_hash: String,
    app_version: String,
    expiry: u32,
    fields: Vec<ProfileFieldSpec>
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct ProfileFieldSpec {
    pub name: String,
    pub display_name: String,
    pub required: bool,
    pub description: String,
    pub access: String,
    pub schema: String,
}

// DTO - Data Transfer Objects Output

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct ProfileData {
    id: EntryHash,
    uuid: String,
    app_name: String,
    app_hash: String,
    app_version: String,
    expiry: u32,
    enabled: bool,
    fields: Vec<ProfileFieldData>
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct ProfileFieldData {
    pub name: String,
    pub display_name: String,
    pub required: bool,
    pub description: String,
    pub access: AccessType,
    pub schema: String,
    pub value: Option<String>
}

//#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
//pub struct FieldMapping {
//    persona: AnyDhtHash,
//    persona_field: AnyDhtHash
//}

//DTO to get data from the persona zome
#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct PersonaField {
    persona_id: Option<AnyDhtHash>,
    field_id: Option<AnyDhtHash>,
    name: String,
    value: Option<String>
}


pub fn create_profile(spec: ProfileSpec) -> ExternResult<EntryHash> {
    //let search_fields = get_name_fields(&profileInit.fields.clone());
    //let mapped_fields = get_persona_fields(search_fields);   //here we call personas and get back vec<PersonaField>
    let pf = PersonaField {persona_id: None, field_id: None, name:"name".into(), value: Some(String::from("Thomas"))};
    let mapped_fields = vec![pf];
    let profile = Profile {
        uuid: spec.uuid,
        app_name: spec.app_name,
        app_hash: spec.app_hash,
        app_version: spec.app_version,
        expiry: 0,
        enabled: true,
        fields: map_profile_spec_fields(mapped_fields.clone(), spec.fields)
    };
    create_entry(&profile.clone())?;
    let profile_hash = hash_entry(&profile.clone())?;
    //let app_key = profile.app_hash.0.clone().to_string();//.0.clone();
    let app_key = profile.uuid.clone();
    let path = Path::from(format!("all_applications.{}",app_key.as_str()));
    path.ensure()?;

    create_link(
        path.hash()?,
        profile_hash.clone(),
        link_tag(profile.app_hash.as_str().clone())?
    )?;
    Ok(profile_hash)
}

//one profile per persona, per app version
pub fn get_profile(app_info: AppInfo) -> ExternResult<Option<ProfileData>> {
    //check if profile exists.
    let mapped_fields = Vec::new();

    //let app_key = wrapped_app_dna.0.clone().to_string();//.0.clone();
    let path = Path::from(format!("all_applications.{}",app_info.uuid.as_str()));
    path.ensure()?;
    //let app_address: AnyDhtHash = app_key.into_hash(); 
    let links = get_links(path.hash()?,Some(link_tag(&app_info.app_hash.as_str())?))?;//, tag_to_app_key(app.clone())?)?;
    let inner_links = links.into_inner();

    if inner_links.len() == 0 {
        return Ok(None);
    }
    let link = inner_links[0].clone();

    let profile: Profile = utils::try_get_and_convert(link.target)?;
    let profile_hash = hash_entry(&profile.clone())?;

    let profile = ProfileData {
        id: profile_hash,
        uuid: profile.uuid,
        app_name: profile.app_name,
        app_hash: profile.app_hash,
        app_version: profile.app_version,
        expiry: profile.expiry,
        enabled: profile.enabled,
        fields: map_to_output_fields(mapped_fields, profile.fields)
    };
    Ok(Some(profile))

}



//Helpers

//DTO preparation
/*
fn get_name_fields(fields: &Vec<ProfileFieldSpec>) -> Vec<PersonaField> {
    return fields.iter().map(|f| {
        return PersonaField {
            persona_id: None,
            field_id: None,
            name: f.name.clone().into(),
            value: None
        }
    }).collect();  
}*/

fn map_profile_spec_fields(mapped_fields: Vec<PersonaField>, field_data: Vec<ProfileFieldSpec> ) -> Vec<ProfileField> {
    return field_data.iter().map(|fd| {
        return ProfileField {
            name: fd.name.clone(),
            display_name: fd.display_name.clone(),
            required: fd.required.clone(),
            description: fd.description.clone(),
            access: AccessType::from_str(&fd.access).unwrap(),
            schema: fd.schema.clone(),
            mapping: get_mapping_data_in(fd.name.clone(),mapped_fields.clone())
        }
    }).collect(); 
}

fn get_mapping_data_in(name:String, mapped_fields:Vec<PersonaField>) -> Option<String> {
    //return AnyDhtHash::from_raw_36_and_type(
     //   b"000000000000000000000000000000000000".to_vec(),
     //   hash_type::AnyDht::Header,
    return mapped_fields.iter().filter(|mf| mf.name == name).map(|r| {
        return r.value.clone();
    }).collect();
}

fn map_to_output_fields(mapped_fields: Vec<PersonaField>, field_data: Vec<ProfileField>) -> Vec<ProfileFieldData> {
    return field_data.iter().map(|fd| {
        return ProfileFieldData {
            name: fd.name.clone(),
            display_name: fd.display_name.clone(),
            required: fd.required.clone(),
            description: fd.description.clone(),
            access: fd.access.clone(),
            schema: fd.schema.clone(),
            value: get_mapping_data_out(fd.name.clone(),mapped_fields.clone())
        }
    }).collect();   
}

fn get_mapping_data_out(name:String, mapped_fields:Vec<PersonaField>) -> Option<String> {
    return mapped_fields.iter().filter(|mf| mf.name == name).map(|r| {
        return r.value.clone()
    }).collect();
}

/*pub fn get_persona_names() -> Vec<String> {
    let function_name = zome::FunctionName("get_links_from_foo".to_owned());
    match call_remote!(
        agent, 
        "zomeone".into(),
        function_name, 
        None,
        ().try_into()?
    )? {
        ZomeCallResponse::Ok(output) => {
            let sb = output.into_inner();
            let links: Links = sb.try_into()?;
            Ok(links)
        },
        ZomeCallResponse::Unauthorized => {
            Err(HdkError::Wasm(WasmError::Zome(
                "this agent has no proper authorization".to_owned()
            )))
        },
    }

}




#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct AgentProfile {
    pub agent_pub_key: WrappedAgentPubKey,
    pub profile: Profile,
}
pub fn get_all_profiles() -> ExternResult<Vec<AgentProfile>> {
    let path = all_profiles_path();

    let links = get_links(path.hash()?)?;

    links
        .into_inner()
        .into_iter()
        .map(|link| get_agent_profile_from_link(link))
        .collect()
}

pub fn get_agent_profile(agent_pub_key: WrappedAgentPubKey) -> ExternResult<Option<AgentProfile>> {
    let path = all_profiles_path();

    let links = get_links(path.hash()?, pub_key_to_tag(agent_pub_key.clone())?)?;

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

fn app_key_to_tag(app_key: WrappedDnaHash) -> ExternResult<LinkTag> {
    let sb: SerializedBytes = app_key.try_into()?;
    Ok(LinkTag(sb.bytes().clone()))
}

fn tag_to_app_key(tag: LinkTag) -> ExternResult<WrappedDnaHash> {
    let sb = SerializedBytes::from(UnsafeBytes::from(tag.0));
    let app_key = WrappedDnaHash::try_from(sb)?;
    Ok(app_key)
}

fn pub_key_to_tag(agent_pub_key: WrappedAgentPubKey) -> ExternResult<LinkTag> {
    let sb: SerializedBytes = agent_pub_key.try_into()?;

    Ok(LinkTag(sb.bytes().clone()))
}

fn tag_to_pub_key(tag: LinkTag) -> ExternResult<WrappedAgentPubKey> {
    let sb = SerializedBytes::from(UnsafeBytes::from(tag.0));

    let pub_key = WrappedAgentPubKey::try_from(sb)?;

    Ok(pub_key)
}*/

#[derive(Serialize, Deserialize, SerializedBytes)]
struct StringLinkTag(String);
pub fn link_tag(tag: &str) -> ExternResult<LinkTag> {
    let sb: SerializedBytes = StringLinkTag(tag.into()).try_into()?;
    Ok(LinkTag(sb.bytes().clone()))
}
