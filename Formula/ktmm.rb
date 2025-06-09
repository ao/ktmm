class Ktmm < Formula
  desc "Keep That Mouse Moving - Prevents system sleep by making periodic mouse movements"
  homepage "https://github.com/ao/ktmm"
  version "0.1.0"
  license "MIT"

  if Hardware::CPU.arm?
    url "https://github.com/ao/ktmm/releases/download/v#{version}/ktmm-macos-arm64"
    sha256 "REPLACE_WITH_ARM64_CHECKSUM_AFTER_FIRST_RELEASE"
  else
    url "https://github.com/ao/ktmm/releases/download/v#{version}/ktmm-macos-x86_64"
    sha256 "REPLACE_WITH_X86_64_CHECKSUM_AFTER_FIRST_RELEASE"
  end

  def install
    if Hardware::CPU.arm?
      bin.install "ktmm-macos-arm64" => "ktmm"
    else
      bin.install "ktmm-macos-x86_64" => "ktmm"
    end
  end

  test do
    system "#{bin}/ktmm", "--version"
  end

  # Add a note about accessibility permissions
  def caveats
    <<~EOS
      KTMM requires accessibility permissions on macOS.
      After installation, go to:
      System Preferences > Security & Privacy > Privacy > Accessibility
      and add KTMM to the list of allowed applications.
    EOS
  end
end