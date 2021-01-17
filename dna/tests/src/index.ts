import {
  Orchestrator,
  Config,
  InstallAgentsHapps,
  TransportConfigType,
} from "@holochain/tryorama";
import { v4 as uuidv4 } from "uuid";
import path from "path";

const network = {
  transport_pool: [
    {
      type: TransportConfigType.Quic,
    },
  ],
  bootstrap_service: "https://bootstrap.holo.host",
};
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
  console.log(alice_test)
  const alice_profiles = await alice_test.cells
  console.log(alice_profiles)
  //console.log(alice_profiles[0])


  const app_UUID = uuidv4()

  const appInfo = {
    uuid: app_UUID, //required
    appName: "calendar",
    appHash: "QmXpCHbuYVtQqpTaevX5y4Ed8Nnr7i4q6RFpMzNfs3W7ms", //required
    appVersion: "3.1"
  }
  
 console.log("TEST1: get profile with app_info:",appInfo)
 try {
  let profile = await alice_profiles[0].call(
    "profiles",
    "get_profile",
    appInfo
  );
  t.notOk(false);
  } catch (e){
    console.log(e)
  }
  console.log("\n")
  await sleep(500);


  console.log("TEST2: get persona data:")
  try {
  let persona = await alice_profiles[0].call(
    "personas",
    "get_persona"
  );
  console.log(persona)
  t.ok(persona);
  } catch (e){
    console.log(e)
  }
  console.log("\n")
  await sleep(500);
  

  // get fields -  returns type vector PersonaField
  console.log("TEST3: get persona data with specified fields:")
  let personaFields = await alice_profiles[0].call(
    "personas",
    "get_fields",
    {
    fields: ["name", "email"]
    }
  )
  console.log(personaFields)
  t.ok(personaFields)
  console.log("\n")
  await sleep(500);

  //save complete profile - uses profileSpec, returns a hash
  console.log("TEST4: save profile data:")
  let profileHash = await alice_profiles[0].call(
    "profiles",
    "create_profile",
    {
      uuid: app_UUID, //required
      appName: "calendar",
      appHash: "QmXpCHbuYVtQqpTaevX5y4Ed8Nnr7i4q6RFpMzNfs3W7ms", //required
      appVersion: "3.1",
      expiry: 23,
      fields: [{
        name: "name",
        displayName: "Name",
        required: true,
        description: "calendar profile name",
        access: "PERSONAL",
        schema: "",
      }],
    }
  );
  console.log("result from creatiion hash: ",profileHash)
  t.ok(profileHash);
  console.log("\n")
  await sleep(500);

  console.log("TEST5: get profile data with appInfo:")
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

  //get profile for app

  /*try {
    profileHash = await bob_profiles.cells[0].call(
      "profiles",
      "create_profile",
      {
        username: "alice",
        fields: {
          avatar: "avatar",
        },
      }
    );
    t.ok(false);
  } catch (e) {}

  profileHash = await bob_profiles.cells[0].call("profiles", "create_profile", {
    username: "bobbo",
    fields: {
      avatar: "bobboavatar",
    },
  });
  t.ok(profileHash);

  await sleep(10);

  myProfile = await alice_profiles.cells[0].call(
    "profiles",
    "get_my_profile",
    null
  );
  t.ok(myProfile.agent_pub_key);
  t.equal(myProfile.profile.username, "alice");

  let profiles = await bob_profiles.cells[0].call(
    "profiles",
    "search_profiles",
    {
      username_prefix: "sdf",
    }
  );
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
