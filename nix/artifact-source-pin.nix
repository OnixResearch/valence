{ lib }:

let
  expectedRevision = "5f3962d7471d1547e044f57daeda72f56621fd52";
  rollbackRevision = "799459346d5416fbd7b9f55840a7371441b55afa";
  historicalRadicleRepository = "https://git.onix.computer/z4JGYYW7WsesXUq7MXVdx16Fawu2f.git";
  mismatchedRevision = "0000000000000000000000000000000000000000";
  expectedProfileRepository = "https://github.com/OnixResearch/onix-artifact";
  expectedSshRepository = "ssh://git@github.com/OnixResearch/onix-artifact.git";
  expectedNixNarHash = "sha256-Xr7ocFkPN15XYb6NA4bv3I9Q42NvoSYtXWfsrj7I3WY=";
  expectedLicense = "AGPL-3.0-or-later";
  incompatibleLicense = "LicenseRef-unreviewed";
  expectedVersion = "0.1.0";
  corePackage = "artifact-auth-core";
  ed25519Package = "artifact-auth-ed25519";
  transferPackage = "artifact-transfer-core";
  bindingPackage = "artifact-binding-core";
  consumerPackageNames = [
    corePackage
    ed25519Package
  ];
  sourcePackageNames = consumerPackageNames ++ [ transferPackage bindingPackage ];
  expectedWorkspaceMembers = map (packageName: "crates/${packageName}") sourcePackageNames;
  expectedConsumerPackageCount = builtins.length consumerPackageNames;
  expectedCargoSource = "git+${expectedSshRepository}?rev=${expectedRevision}#${expectedRevision}";
  nixInputNode = "artifact";

  valueAt =
    path: fallback: value:
    lib.attrByPath path fallback value;
  issueUnless = condition: code: lib.optional (!condition) code;
  sorted = values: builtins.sort builtins.lessThan values;

  dependencyIssues =
    cargoWorkspace: packageName:
    let
      dependency = valueAt [ "workspace" "dependencies" packageName ] { } cargoWorkspace;
    in
    issueUnless (dependency ? git && dependency.git == expectedSshRepository) "cargo.${packageName}.git"
    ++ issueUnless (
      dependency ? rev && dependency.rev == expectedRevision
    ) "cargo.${packageName}.revision"
    ++ issueUnless (!(dependency ? path)) "cargo.${packageName}.workspace-relative"
    ++ issueUnless (!(dependency ? branch) && !(dependency ? tag)) "cargo.${packageName}.floating";

  lockPackageIssues =
    cargoLock: packageName:
    let
      matching = builtins.filter (package: (package.name or null) == packageName) (
        cargoLock.package or [ ]
      );
      package = if builtins.length matching == 1 then builtins.head matching else { };
    in
    issueUnless (builtins.length matching == 1) "cargo-lock.${packageName}.unique"
    ++ issueUnless ((package.version or null) == expectedVersion) "cargo-lock.${packageName}.version"
    ++ issueUnless ((package.source or null) == expectedCargoSource) "cargo-lock.${packageName}.source";

  consumerLockSetIssues =
    cargoLock:
    let
      sourcePackages = builtins.filter (package: (package.source or null) == expectedCargoSource) (
        cargoLock.package or [ ]
      );
      sourcePackageNamesObserved = map (package: package.name or "") sourcePackages;
    in
    issueUnless (
      sorted sourcePackageNamesObserved == sorted consumerPackageNames
    ) "cargo-lock.consumer-package-set";

  workspaceUseIssues =
    input:
    issueUnless (valueAt [
      "dependencies"
      corePackage
      "workspace"
    ] false input.coreManifest) "valence-core.workspace-dependency"
    ++ issueUnless (valueAt [
      "dependencies"
      ed25519Package
      "workspace"
    ] false input.coreManifest) "valence-core.ed25519-workspace-dependency"
    ++ issueUnless (valueAt [
      "dependencies"
      corePackage
      "workspace"
    ] false input.radicleManifest) "valence-radicle.core-workspace-dependency"
    ++ issueUnless (valueAt [
      "dependencies"
      ed25519Package
      "workspace"
    ] false input.radicleManifest) "valence-radicle.ed25519-workspace-dependency";

  nixLockIssues =
    input:
    let
      rootInput = valueAt [ "nodes" "root" "inputs" nixInputNode ] null input.flakeLock;
      node = valueAt [ "nodes" nixInputNode ] { } input.flakeLock;
      locked = node.locked or { };
      original = node.original or { };
    in
    issueUnless (rootInput == nixInputNode) "nix.root-input"
    ++ issueUnless ((node.flake or null) == false) "nix.input.flake-enabled"
    ++ issueUnless ((locked.type or null) == "git") "nix.locked.type"
    ++ issueUnless ((locked.url or null) == expectedSshRepository) "nix.locked.repository"
    ++ issueUnless ((locked.rev or null) == expectedRevision) "nix.locked.revision"
    ++ issueUnless ((locked.narHash or null) == expectedNixNarHash) "nix.locked.nar-hash"
    ++ issueUnless ((original.type or null) == "git") "nix.original.type"
    ++ issueUnless ((original.url or null) == expectedSshRepository) "nix.original.repository"
    ++ issueUnless ((original.rev or null) == expectedRevision) "nix.original.revision"
    ++ issueUnless (!(original ? path) && !(original ? ref)) "nix.original.floating-or-path"
    ++ issueUnless (input.resolvedNixRevision == expectedRevision) "nix.resolved.revision"
    ++ issueUnless (input.resolvedNixNarHash == expectedNixNarHash) "nix.resolved.nar-hash";

  sourcePackageIssues =
    workspacePackage: manifest: packageName:
    let
      package = manifest.package or { };
    in
    issueUnless ((package.name or null) == packageName) "source.${packageName}.name"
    ++ issueUnless (
      (package.publish or null) == false
      || (
        (package.publish or null) == { workspace = true; }
        && (workspacePackage.publish or null) == false
      )
    ) "source.${packageName}.publish"
    ++ issueUnless (valueAt [
      "license"
      "workspace"
    ] false package) "source.${packageName}.license-inheritance";

  sourceReleaseIssues =
    input:
    let
      workspacePackage = valueAt [ "workspace" "package" ] { } input.sourceWorkspaceManifest;
      workspaceMembers = valueAt [ "workspace" "members" ] [ ] input.sourceWorkspaceManifest;
      profilePackages = input.releaseProfile.packages or [ ];
      distribution = input.releaseProfile.distribution or { };
    in
    issueUnless ((workspacePackage.version or null) == expectedVersion) "source.workspace.version"
    ++ issueUnless ((workspacePackage.license or null) == expectedLicense) "source.workspace.license"
    ++ issueUnless ((input.releaseProfile.license or null) == expectedLicense) "source.profile.license"
    ++ issueUnless (
      (input.releaseProfile.repository or null) == expectedProfileRepository
    ) "source.profile.repository"
    ++ issueUnless (
      sorted workspaceMembers == sorted expectedWorkspaceMembers
    ) "source.workspace.members"
    ++ issueUnless (sorted profilePackages == sorted sourcePackageNames) "source.profile.packages"
    ++ issueUnless ((distribution.channel or null) == "immutable-git-revision") "source.profile.channel"
    ++ issueUnless ((distribution.crates_io or null) == false) "source.profile.crates-io"
    ++ issueUnless ((distribution.floating_revisions or null) == false) "source.profile.floating"
    ++ issueUnless ((distribution.sibling_paths or null) == false) "source.profile.sibling-paths"
    ++ sourcePackageIssues workspacePackage input.sourceCoreManifest corePackage
    ++ sourcePackageIssues workspacePackage input.sourceEd25519Manifest ed25519Package
    ++ sourcePackageIssues workspacePackage input.sourceTransferManifest transferPackage
    ++ sourcePackageIssues workspacePackage input.sourceBindingManifest bindingPackage;

  validate =
    input:
    let
      issues =
        dependencyIssues input.cargoWorkspace corePackage
        ++ dependencyIssues input.cargoWorkspace ed25519Package
        ++ workspaceUseIssues input
        ++ lockPackageIssues input.cargoLock corePackage
        ++ lockPackageIssues input.cargoLock ed25519Package
        ++ consumerLockSetIssues input.cargoLock
        ++ nixLockIssues input
        ++ sourceReleaseIssues input;
    in
    {
      valid = issues == [ ];
      inherit issues;
      package_count = expectedConsumerPackageCount;
      revision = expectedRevision;
    };

  replaceWorkspaceDependency =
    input: packageName: dependency:
    input
    // {
      cargoWorkspace = input.cargoWorkspace // {
        workspace = input.cargoWorkspace.workspace // {
          dependencies = input.cargoWorkspace.workspace.dependencies // {
            "${packageName}" = dependency;
          };
        };
      };
    };

  expectRejected =
    expectedIssue: input:
    let
      result = validate input;
    in
    issueUnless (
      !result.valid && builtins.elem expectedIssue result.issues
    ) "self-test.expected-${expectedIssue}";

  selfTest =
    input:
    let
      coreLockPackage = builtins.head (
        builtins.filter (package: (package.name or null) == corePackage) (input.cargoLock.package or [ ])
      );
      transferLockPackage = coreLockPackage // {
        name = transferPackage;
      };
      bindingLockPackage = coreLockPackage // {
        name = bindingPackage;
      };
      floatingInput = replaceWorkspaceDependency input corePackage {
        git = expectedSshRepository;
        branch = "main";
      };
      historicalCargoInput = replaceWorkspaceDependency input corePackage {
        git = historicalRadicleRepository;
        rev = rollbackRevision;
        version = expectedVersion;
      };
      historicalNixInput = input // {
        flakeLock = lib.recursiveUpdate input.flakeLock {
          nodes."${nixInputNode}" = {
            locked = {
              url = historicalRadicleRepository;
              rev = rollbackRevision;
            };
            original = {
              url = historicalRadicleRepository;
              rev = rollbackRevision;
            };
          };
        };
      };
      mismatchedInput = input // {
        flakeLock = lib.recursiveUpdate input.flakeLock {
          nodes."${nixInputNode}".locked.rev = mismatchedRevision;
        };
      };
      duplicateInput = input // {
        cargoLock = input.cargoLock // {
          package = (input.cargoLock.package or [ ]) ++ [ coreLockPackage ];
        };
      };
      widenedConsumerGraphInput = input // {
        cargoLock = input.cargoLock // {
          package = (input.cargoLock.package or [ ]) ++ [ transferLockPackage ];
        };
      };
      bindingConsumerGraphInput = input // {
        cargoLock = input.cargoLock // {
          package = (input.cargoLock.package or [ ]) ++ [ bindingLockPackage ];
        };
      };
      missingBindingSourceProfileInput = input // {
        releaseProfile = input.releaseProfile // {
          packages = consumerPackageNames ++ [ transferPackage ];
        };
      };
      bindingLicenseInput = input // {
        sourceBindingManifest = lib.recursiveUpdate input.sourceBindingManifest {
          package.license = incompatibleLicense;
        };
      };
      transferPublishInput = input // {
        sourceTransferManifest = lib.recursiveUpdate input.sourceTransferManifest {
          package.publish = true;
        };
      };
      workspacePublishInput = input // {
        sourceWorkspaceManifest = lib.recursiveUpdate input.sourceWorkspaceManifest {
          workspace.package.publish = true;
        };
      };
      incompleteSourceProfileInput = input // {
        releaseProfile = input.releaseProfile // {
          packages = consumerPackageNames;
        };
      };
      incompleteSourceWorkspaceInput = input // {
        sourceWorkspaceManifest = lib.recursiveUpdate input.sourceWorkspaceManifest {
          workspace.members = map (packageName: "crates/${packageName}") consumerPackageNames;
        };
      };
      incompatibleLicenseInput = input // {
        releaseProfile = input.releaseProfile // {
          license = incompatibleLicense;
        };
        sourceWorkspaceManifest = lib.recursiveUpdate input.sourceWorkspaceManifest {
          workspace.package.license = incompatibleLicense;
        };
      };
      workspaceRelativeInput = replaceWorkspaceDependency input corePackage {
        path = "../artifact/crates/artifact-auth-core";
      };
    in
    expectRejected "cargo.${corePackage}.revision" floatingInput
    ++ expectRejected "cargo.${corePackage}.git" historicalCargoInput
    ++ expectRejected "nix.locked.repository" historicalNixInput
    ++ expectRejected "nix.locked.revision" mismatchedInput
    ++ expectRejected "cargo-lock.${corePackage}.unique" duplicateInput
    ++ expectRejected "cargo-lock.consumer-package-set" widenedConsumerGraphInput
    ++ expectRejected "cargo-lock.consumer-package-set" bindingConsumerGraphInput
    ++ expectRejected "source.profile.packages" missingBindingSourceProfileInput
    ++ expectRejected "source.${bindingPackage}.license-inheritance" bindingLicenseInput
    ++ expectRejected "source.${transferPackage}.publish" transferPublishInput
    ++ expectRejected "source.${bindingPackage}.publish" workspacePublishInput
    ++ expectRejected "source.profile.packages" incompleteSourceProfileInput
    ++ expectRejected "source.workspace.members" incompleteSourceWorkspaceInput
    ++ expectRejected "source.workspace.license" incompatibleLicenseInput
    ++ expectRejected "cargo.${corePackage}.workspace-relative" workspaceRelativeInput;
in
{
  inherit selfTest validate;
  expected = {
    inherit expectedCargoSource expectedLicense expectedNixNarHash;
    profile_repository = expectedProfileRepository;
    ssh_repository = expectedSshRepository;
    packages = consumerPackageNames;
    source_packages = sourcePackageNames;
    revision = expectedRevision;
    rollback_revision = rollbackRevision;
    version = expectedVersion;
  };
}
