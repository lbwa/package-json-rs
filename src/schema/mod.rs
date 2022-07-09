mod default;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// `package.json` schema from [official npm documentation](https://docs.npmjs.com/cli/v8/configuring-npm/package-json), see also [json-schemas repo](https://github.com/SchemaStore/schemastore/blob/master/src/schemas/json/package.json) and [json-schemas online](https://json.schemastore.org/package)
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
  /// The [name](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#name) for npm package
  name: String,
  /// The [version](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#version) for npm package
  version: String,
  /// The [description](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#description-1) helps people discover your package, as it's listed in `npm search`.
  description: Option<String>,
  /// The [keywords](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#keywords) helps people discover your package as it's listed in `npm search`.
  keywords: Option<Vec<String>>,
  /// The url to the project [homepage](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#homepage).
  homepage: Option<String>,
  /// The url to your project's issue tracker and / or the email address to which issues should be reported.
  /// These are helpful for people who encounter issues with your package.
  ///
  /// More details: <https://docs.npmjs.com/cli/v8/configuring-npm/package-json#bugs>
  bugs: Option<PackageBugs>,
  /// The [license](https://spdx.org/licenses/) of the project.
  license: Option<String>,
  author: Option<PackagePeople>,
  /// A list of [people](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#people-fields-author-contributors) who contributed to this package.
  contributors: Option<Vec<PackagePeople>>,
  /// A list of [people](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#people-fields-author-contributors) who maintains this package.
  maintainers: Option<Vec<PackagePeople>>,
  funding: Option<Vec<PackageFunding>>,
  files: Option<Vec<String>>,
  /// The [main](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#main) field is a module ID that is the primary entry point to your program. That is, if your package is named foo, and a user installs it, and then does require("foo"), then your main module's exports object will be returned
  ///
  /// This should be a module relative to the root of your package folder.
  ///
  /// For most modules, it makes the most sense to have a main script and often not much else.
  ///
  /// If main is not set it defaults to index.js in the package's root folder.
  #[serde(default = "default::main")]
  main: String,
  /// If your module is meant to be used client-side the [browser](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#browser) field should be used instead of the main field. This is helpful to hint users that it might rely on primitives that aren't available in Node.js modules. (e.g. window)
  browser: Option<String>,
  /// A lot of packages have one or more executable files that they'd like to install into the PATH. npm makes this pretty easy (in fact, it uses this feature to install the "npm" executable.)
  ///
  /// To use this, supply a [bin](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#bin) field in your package.json which is a map of command name to local file name. When this package is installed globally, that file will be linked where global bins go so it is available to run by name. When this package is installed as a dependency in another package, the file will be linked where it will be available to that package either directly by npm exec or by name in other scripts when invoking them via npm run-script.
  bin: Option<PackageBin>,
  /// Specify either a single file or an array of filenames to put in place for the man program to find.
  ///
  /// If only a single file is provided, then it's installed such that it is the result from man <pkgname>, regardless of its actual filename.
  ///
  /// More details: <https://docs.npmjs.com/cli/v8/configuring-npm/package-json#man>
  man: Option<PackageMan>,
  /// The CommonJS Packages spec details a few ways that you can indicate the structure of your package using a directories object. If you look at npm's package.json, you'll see that it has directories for doc, lib, and man.
  ///
  /// More details: <https://docs.npmjs.com/cli/v8/configuring-npm/package-json#directories>
  directories: Option<PackageDirectories>,
  /// Specify the place where your code lives. This is helpful for people who want to contribute. If the git repo is on GitHub, then the npm docs command will be able to find you.
  ///
  /// More details: <https://docs.npmjs.com/cli/v8/configuring-npm/package-json#repository>
  repository: Option<PackageRepository>,
  /// The "[scripts](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#scripts)" property is a dictionary containing script commands that are run at various times in the lifecycle of your package. The key is the lifecycle event, and the value is the command to run at that point.
  #[serde(default = "default::scripts")]
  scripts: HashMap<String, String>,
  /// A "[config](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#config)" object can be used to set configuration parameters used in package scripts that persist across upgrades.
  config: Option<HashMap<String, String>>,
  /// [Dependencies](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#dependencies) are specified in a simple object that maps a package name to a version range. The version range is a string which has one or more space-separated descriptors. Dependencies can also be identified with a tarball or git URL.
  ///
  /// Please do not put test harnesses or transpilers or other "development" time tools in your dependencies object. See devDependencies.
  dependencies: Option<PackageDependencies>,
  /// If someone is planning on downloading and using your module in their program, then they probably don't want or need to download and build the external test or documentation framework that you use.
  ///
  /// In this case, it's best to map these additional items in a [devDependencies](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#devdependencies) object.
  dev_dependencies: Option<PackageDependencies>,
  /// In some cases, you want to express the compatibility of your package with a host tool or library, while not necessarily doing a require of this host. This is usually referred to as a plugin. Notably, your module may be exposing a specific interface, expected and specified by the host documentation.
  ///
  /// More details: <https://docs.npmjs.com/cli/v8/configuring-npm/package-json#peerdependencies>
  peer_dependencies: Option<PackageDependencies>,
  /// When a user installs your package, npm will emit warnings if packages specified in peerDependencies are not already installed. The [peerDependenciesMeta](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#peerdependenciesmeta) field serves to provide npm more information on how your peer dependencies are to be used. Specifically, it allows peer dependencies to be marked as optional.
  peer_dependencies_meta: Option<HashMap<String, HashMap<String, bool>>>,
  /// This defines an array of package names that will be bundled when publishing the package.
  ///
  /// More details: <https://docs.npmjs.com/cli/v8/configuring-npm/package-json#bundleddependencies>
  bundled_dependencies: Option<Vec<String>>,
  /// If a dependency can be used, but you would like npm to proceed if it cannot be found or fails to install, then you may put it in the [optionalDependencies](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#optionaldependencies) object.
  optional_dependencies: Option<PackageDependencies>,
  /// If you need to make specific changes to dependencies of your dependencies, for example replacing the version of a dependency with a known security issue, replacing an existing dependency with a fork, or making sure that the same version of a package is used everywhere, then you may add an override.
  ///
  /// [Overrides](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#overrides) provide a way to replace a package in your dependency tree with another version, or another package entirely. These changes can be scoped as specific or as vague as desired.
  overrides: Option<HashMap<String, String>>,
  /// Specify which [engines](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#engines) your module will run on.
  engines: Option<HashMap<String, String>>,
  /// Specify which [operating systems](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#os) your module will run on.
  os: Option<Vec<String>>,
  /// Specify which [cpu](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#cpu) your module will run on.
  cpu: Option<Vec<String>>,
  /// If set to true, then npm will refuse to publish it.
  ///
  /// More details: <https://docs.npmjs.com/cli/v8/configuring-npm/package-json#private>
  #[serde(default)]
  private: bool,
  /// This is a set of config values that will be used at publish-time. It's
  /// especially handy if you want to set the tag, registry or access, so that
  /// you can ensure that a given package is not tagged with "latest", published
  /// to the global public registry or that a scoped module is private by default.
  ///
  /// see also [official site](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#publishconfig)
  publish_config: Option<HashMap<String, String>>,
  /// The optional [workspace](https://docs.npmjs.com/cli/v8/configuring-npm/package-json#workspaces)s
  /// field is an array of file patterns that describes locations within the local
  /// file system that the install client should look up to find each workspace
  /// that needs to be symlinked to the top level node_modules folder.
  workspaces: Option<Vec<String>>,
  /// When set to "module", the type field allows a package to specify all .js files within are ES modules. If the "type" field is omitted or set to "commonjs", all .js files are treated as CommonJS.
  #[serde(default = "default::r#type")]
  r#type: String,
  /// Set the types property to point to your bundled declaration file. This is useful for packages that have a large number of types, but only a few of which are used.
  types: Option<String>,
  /// Note that the "typings" field is synonymous with "types", and could be used as well.
  typings: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
enum PackageBugs {
  Url(String),
  Record(PackageBugsRecord),
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct PackageBugsRecord {
  url: String,
  email: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum PackagePeople {
  Literal(String),
  Record(PackagePeopleRecord),
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct PackagePeopleRecord {
  name: String,
  email: Option<String>,
  url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
enum PackageFunding {
  Url(String),
  Record(PackageFundingRecord),
  Slice(Vec<PackageFundingRecord>),
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct PackageFundingRecord {
  r#type: String,
  url: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum PackageBin {
  Literal(String),
  Record(HashMap<String, String>),
}

#[derive(Serialize, Deserialize, Debug)]
enum PackageMan {
  Literal(String),
  Slice(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct PackageDirectories {
  bin: Option<String>,
  man: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct PackageRepository {
  r#type: String,
  url: String,
}

type PackageDependencies = HashMap<String, String>;

#[test]
fn test_package_json() {
  use self::default;
  let package_json_raw = r#"
    {
      "name": "test",
      "version": "1.0.0",
      "description": "test",
      "devDependencies": {
        "typescript": "*"
      }
    }
  "#;

  let json = serde_json::from_str::<PackageJson>(package_json_raw).unwrap();
  // test actual values
  assert_eq!(json.name, "test");
  assert_eq!(json.version, "1.0.0");
  assert_eq!(json.description, Some("test".to_owned()));
  assert_eq!(json.license, None);
  assert_eq!(json.dependencies, None);
  assert_eq!(
    json.dev_dependencies,
    Some(HashMap::from([("typescript".to_owned(), "*".to_owned())]))
  );
  assert_eq!(json.bundled_dependencies, None);

  // test default values
  assert_eq!(json.private, false);
  assert_eq!(json.scripts, default::scripts());
  assert_eq!(json.main, default::main());
  assert_eq!(json.r#type, default::r#type());
}
