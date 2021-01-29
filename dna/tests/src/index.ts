import {
  Orchestrator,
  Config,
  InstallAgentsHapps,
  TransportConfigType,
} from "@holochain/tryorama";
import { v4 as uuidv4 } from "uuid";
import path from "path";
import { exit } from "process";

const network = {
  transport_pool: [
    {
      type: TransportConfigType.Quic,
    },
  ]
};

//bootstrap_service: "https://bootstrap.holo.host",
const conductorConfig = Config.gen({ network });

// Construct proper paths for your DNAs
const profilesDna = path.join(__dirname, "../../profiles.dna.gz");

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [profilesDna],
  ],
];

const sleep = (ms) =>
  new Promise((resolve) => setTimeout(() => resolve(null), ms));

const orchestrator = new Orchestrator();

orchestrator.registerScenario("try to retieve a profile that doesnt exist, get fields needed, create one and try again", async (s, t) => {
  const [alice] = await s.players([conductorConfig, conductorConfig]);

  // install your happs into the coductors and destructuring the returned happ data using the same
  // array structure as you created in your installation array.
  const [[alice_test]] = await alice.installAgentsHapps(installation);
  //console.log(alice_test)
  const alice_profiles = await alice_test.cells
  //console.log(alice_profiles)
  //console.log(alice_profiles[0])


  const app_UUID = uuidv4()

  const appInfo = {
    uuid: app_UUID, //required
    appName: "calendar",
    appHash: "QmXpCHbuYVtQqpTaevX5y4Ed8Nnr7i4q6RFpMzNfs3W7ms", //required
    appVersion: "3.1",
    expiry: 0,
    fields: [{
      name: "username",
      displayName: "Username",
      required: true,
      description: "calendar profile name",
      access: "PUBLIC",
      schema: "{'context':'profile'}",  //this should be a BTREEMAP not a string on rust side
    },
    {
      name: "email",
      displayName: "Email",
      required: true,
      description: "calendar profile email",
      access: "PERSONAL",
      schema: "{'schematype':'profile'}",
    }]
    //fields: ['name', 'email']
  }
  const field_set:string[] = appInfo.fields.map(f=>{ return f.name})
 // console.log("Test data:")
 // console.log("--------------------------------------------------------")
 // console.log("fields: ",field_set)
 // console.log("appInfo: ",appInfo.fields)
 // console.log("--------------------------------------------------------")
 
 console.log("TEST1: get profile with app_info (app to zome call):",appInfo)
 try {
    let profile = await alice_profiles[0].call(
      "profiles",
      "get_profile",
      appInfo
    );
    console.log("app profile does not exist... redirecting to personas")
    t.notOk(profile);
    console.log("\n")
  } catch (e){
    console.log(e)
  }
 
  console.log("TEST2: get persona data (called for each persona to present persona-chooser in profiles UI):")
  try {
    let persona = await alice_profiles[0].call(
      "personas",
      "get_persona"
    );
    console.log(persona,"\n persona default chosen")
    t.ok(persona);
    console.log("\n")
  } catch (e){
    console.log(e)
  }
  

  // get fields -  returns type vector PersonaField - could go via profiles
  console.log("TEST3: pre-populate form with existing fields and higlight missing:")
  try {
    let non_existing_Fields = await alice_profiles[0].call(
      "personas",
      "get_fields",      
      { 
        fields:field_set
      }
    )
    console.log(non_existing_Fields)
    t.equal(non_existing_Fields.length,0)
    console.log("Redirecting to Personas UI to enter new field (that was missing)\n")
  } catch (e){
    console.log(e)
  }

//returns personaField
  console.log("TEST4: set persona data with first missing field:")
  try{
    let pFields = await alice_profiles[0].call(
      "personas",
      "add_field",
      {
        key:"email", value:"thomas@thomas.th"
      }
    )
    console.log(pFields)
    t.ok(pFields)
    console.log("redirecting back to Profiles UI\n")
  } catch (e){
    console.log(e)
    exit()
  }
  //await sleep(500);

  console.log("TEST5: set persona data with second missing field, no pause:")
  try{
    let pFields = await alice_profiles[0].call(
      "personas",
      "add_field",
      {
        key:"username", value:"thomascal"
      }
    )
    console.log(pFields)
    t.ok(pFields)
    console.log("redirecting back to Profiles UI\n")
  } catch (e){
    console.log(e)
    exit()
  }
  await sleep(500);

  
  // get fields -  - skipping - go to test 7
  /*console.log("TEST6: confirm all fields are stored:")
  try {
    let existing_Fields = await alice_profiles[0].call(
      "personas",
      "get_fields", //returns type vector PersonaField       
      { 
        fields:field_set
      }
    )
    console.log(existing_Fields)
    t.ok(existing_Fields.length > 0)
    console.log("Redirecting to Personas UI to enter new field (that was missing)\n")
  } catch (e){
    console.log(e)
  }
  await sleep(500);
*/

  //return to profiles UI
  //save complete profile - uses profileSpec, returns a hash
  console.log("TEST6: save profile data:")
  let profileHash = await alice_profiles[0].call(
    "profiles",
    "create_profile",
    appInfo
  );
  console.log("result from creation hash: ",profileHash)
  t.ok(profileHash);
  console.log("\n")
  //await sleep(500);



 //repeat test1 to check that profile exists  
console.log("TEST7: get profile data with values included (interzome personas) :")
try {
  let profile = await alice_profiles[0].call(
    "profiles",
    "get_profile",
    appInfo
  );
  console.log(profile)
  t.ok(profile);
  } catch (e){
    console.log(e)
  }
  await sleep(500);





  /*  IGNORE TESTING REFERENCE


  t.equal(profiles.length, 0);

  profiles = await bob_profiles.cells[0].call("profiles", "search_profiles", {
    username_prefix: "alic",
  });
  t.equal(profiles.length, 1);
  t.ok(profiles[0].agent_pub_key);
  t.equal(profiles[0].profile.username, "alice");

  profiles = await bob_profiles.cells[0].call("profiles", "search_profiles", {
    username_prefix: "ali",
  });
  t.equal(profiles.length, 1);
  t.ok(profiles[0].agent_pub_key);
  t.equal(profiles[0].profile.username, "alice");
  t.equal(profiles[0].profile.fields.avatar, "aliceavatar");

  profiles = await bob_profiles.cells[0].call("profiles", "search_profiles", {
    username_prefix: "alice",
  });
  t.equal(profiles.length, 1);
  t.ok(profiles[0].agent_pub_key);
  t.equal(profiles[0].profile.username, "alice");

  profiles = await bob_profiles.cells[0].call("profiles", "search_profiles", {
    username_prefix: "bob",
  });
  t.equal(profiles.length, 1);
  t.ok(profiles[0].agent_pub_key);
  t.equal(profiles[0].profile.username, "bobbo");
  t.equal(profiles[0].profile.fields.avatar, "bobboavatar");*/
});

orchestrator.run();
