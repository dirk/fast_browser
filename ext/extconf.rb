require 'mkmf'

def sys(cmd, &block)
  block = ->(f) { f.gets } if block.nil?

  ret = IO.popen(cmd, &block)

  if $?.to_i != 0
    puts "=> Failed!"
    raise "Command failed: #{cmd}"
  else
    ret
  end
end

ROOT      = File.expand_path '..', File.dirname(__FILE__)
RUST_ROOT = File.join ROOT, 'rust'

puts ' - Checking Rust compiler'
rustc = sys "cd #{RUST_ROOT}; rustc --version"

allowed_versions = [
  'rustc 1.6.0',
  'rustc 1.7.0',
  'rustc 1.8.0',
]

if allowed_versions.none? {|v| rustc.include? v }
  puts "=> Bad Rust compiler version: #{rustc}"
  puts '   One of the following versions is required:'
  allowed_versions.each do |v|
    puts "     - #{v}"
  end

  raise "Invalid Rust compiler version"
end

# Create an empty makefile with an empty `install` task
puts ' - Creating Makefile'
File.open('Makefile', 'w') do |f|
  body = [
    "install:",
    "\tcd #{RUST_ROOT}; cargo build --release",
    "\tmkdir -p #{ROOT}/ext/fast_browser",
    "\tcp #{RUST_ROOT}/target/release/libfast_browser.* #{ROOT}/ext/fast_browser"
  ]
  f.puts(body.join("\n") + "\n")
end
