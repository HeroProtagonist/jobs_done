require 'fileutils'

version = ARGV[0]

raise "Version required" if version.nil?

`gh release create #{version}`

targets = {
  "x86_64-pc-windows-gnu" => "windows-64-bit",
  "x86_64-apple-darwin" => "macos-64-bit",
  "aarch64-apple-darwin" => "macos-arm",
  "x86_64-unknown-linux-gnu" => "linux-64-bit",
}

targets.each do |target, name|
  `cross build --target #{target} --release`

  extension = ""
  extension = ".exe" if target.match?(/pc-windows/)

  FileUtils.mkdir_p(name)
  FileUtils.cp("target/#{target}/release/jobs_done#{extension}", name)

  zip_file = "#{name}.zip"

  FileUtils.cd(name) do
    `tar czf ../#{zip_file} *`
  end

  FileUtils.remove_entry(name)

  `gh release upload #{version} #{zip_file}`

  FileUtils.remove_entry(zip_file)
end
