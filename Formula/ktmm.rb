class Ktmm < Formula
  desc "Keep That Mouse Moving - Prevents system sleep by making periodic mouse movements"
  homepage "https://github.com/ao/ktmm"
  url "https://github.com/ao/ktmm/archive/refs/tags/v0.5.2.tar.gz"
  sha256 "5962df6852d5d1b78ace3248f26d5737fee131dda80f2f8b53bbdf1319e85b76"  # source
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "ktmm 0.5.2", shell_output("#{bin}/ktmm --version")
  end

  def caveats
    <<~EOS
      KTMM requires accessibility permissions on macOS.
      After installation, go to:
      System Preferences > Security & Privacy > Privacy > Accessibility
      and add KTMM to the list of allowed applications.
    EOS
  end
end