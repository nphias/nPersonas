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

orchestrator.registerScenario("create a profile and get it", async (s, t) => {
  const [alice, bob] = await s.players([conductorConfig, conductorConfig]);

  // install your happs into the coductors and destructuring the returned happ data using the same
  // array structure as you created in your installation array.
  const [[alice_profiles]] = await alice.installAgentsHapps(installation);
  const [[bob_profiles]] = await bob.installAgentsHapps(installation);

  const app_UUID = uuidv4()
  const profileMeta = {
    uuid: app_UUID,
    applicationName: "calendar",
    appHash: "QmXpCHbuYVtQqpTaevX5y4Ed8Nnr7i4q6RFpMzNfs3W7ms",
    appVersion: 3.1,
    fields: ["name", "email"],
  }

  let profileOut1 = await alice_profiles.cells[0].call(
    "profiles",
    "get_profile",
    profileMeta
  );
  console.log(profileOut1)
  t.notOk(profileOut1);

  let profileOut2 = await alice_profiles.cells[0].call(
    "profiles",
    "init_profile",
    profileMeta
  );
  console.log(profileOut2)
  t.notOk(profileOut2);

  await sleep(500);

  let profileHash = await alice_profiles.cells[0].call(
    "profiles",
    "create_profile",
    {
      applicationName: "calendar",
      appHash: "QmXpCHbuYVtQqpTaevX5y4Ed8Nnr7i4q6RFpMzNfs3W7ms",
      fields: [{
        name: "name",
        displayName: "Name",
        required: true,
        description: "calendar profile name",
        access: "PERSONAL",
        schema: "",
        value: "Thomas"
      }],
    }
  );
  console.log(profileHash)
  t.ok(profileHash);

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
