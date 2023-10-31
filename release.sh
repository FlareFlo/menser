cargo set-version --version || echo "installing cargo-edit" && cargo install cargo-edit # Validate cargo-edit installation
cargo get --version || echo "installing cargo-get" && cargo install cargo-get # Validate cargo-get installation

# Bump version
cargo set-version --bump patch
version=$(cargo get package.version)
git tag $version
git add Cargo.lock
git add Cargo.toml
git commit -m "Releasing version $version"

# publish tag to git
read -p "Automatically push tag $version to git? " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]
then
  git push origin $version
fi

# publish to crates.io
read -p "Automatically publish $version to crates.io? " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]
then
  cargo publish
fi
