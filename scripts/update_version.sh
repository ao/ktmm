#!/bin/bash

# Script to update version numbers across all project files
# Usage: ./scripts/update_version.sh 0.5.0

set -e

if [ $# -ne 1 ]; then
  echo "Usage: $0 <new_version>"
  echo "Example: $0 0.5.0"
  exit 1
fi

NEW_VERSION=$1
echo "Updating version to $NEW_VERSION"

# Check if the version format is valid (x.y.z)
if ! [[ $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "Error: Version must be in format x.y.z (e.g., 0.5.0)"
  exit 1
fi

# Update version in Cargo.toml
echo "Updating Cargo.toml..."
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Update version in Formula/ktmm.rb
echo "Updating Formula/ktmm.rb..."
sed -i.bak "s/version \".*\"/version \"$NEW_VERSION\"/" Formula/ktmm.rb
rm Formula/ktmm.rb.bak

# Update version in Formula/ktmm_cask_style.rb
if [ -f Formula/ktmm_cask_style.rb ]; then
  echo "Updating Formula/ktmm_cask_style.rb..."
  # Update URL
  sed -i.bak "s|url \".*\/v.*\.tar\.gz\"|url \"https://github.com/ao/ktmm/archive/refs/tags/v$NEW_VERSION.tar.gz\"|" Formula/ktmm_cask_style.rb
  # Update test assertion
  sed -i.bak "s/assert_match \"ktmm .*\"/assert_match \"ktmm $NEW_VERSION\"/" Formula/ktmm_cask_style.rb
  rm Formula/ktmm_cask_style.rb.bak
fi

echo "Version updated to $NEW_VERSION in all files"
echo ""
echo "Next steps:"
echo "1. Commit the changes: git commit -am \"Bump version to $NEW_VERSION\""
echo "2. Create a tag: git tag -a v$NEW_VERSION -m \"Release v$NEW_VERSION\""
echo "3. Push changes: git push origin main && git push origin v$NEW_VERSION"
echo ""
echo "The GitHub Actions workflow will automatically:"
echo "- Build binaries for all platforms"
echo "- Create a GitHub release"
echo "- Update formula files with correct checksums"