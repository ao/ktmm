class Ktmm < Formula
  desc "Keep That Mouse Moving - Prevents system sleep by making periodic mouse movements"
  homepage "https://github.com/ao/ktmm"
  version "0.2.0"
  license "MIT"

  if Hardware::CPU.arm?
    url "https://github.com/ao/ktmm/releases/download/v#{version}/ktmm-macos-arm64"
    sha256 "2e6986970b21679dfe6deb06a43a9b40659b493e002ee875f4c581620bf613aa"
  else
    url "https://github.com/ao/ktmm/releases/download/v#{version}/ktmm-macos-x86_64"
    sha256 "dc37da38d8f80a4e577338e587c5087893c9d21af5d12dcc69924f5437b5df16"
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