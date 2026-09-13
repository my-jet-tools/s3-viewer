use ci_utils::ci_generator::CiGenerator;

// Regenerates ./Dockerfile, ./.github/workflows/release.yaml and ./.github/workflows/test.yml on
// every build, so hand-edits to them are silently reverted: change them here.
//
// The release workflow builds only this Rust server. The UI is NOT built in CI: `wwwroot/` is
// committed (rebuilt with ./build-ui.sh) and copied next to the binary, so `./wwwroot` resolves
// from the container's working directory exactly as it does from the repo root.
fn main() {
    CiGenerator::new(env!("CARGO_PKG_NAME"))
        .as_basic_service()
        .add_docker_copy_file("./wwwroot", "./wwwroot")
        .generate_github_ci_file()
        .with_ci_test()
        .build();
}
