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

puts ' - Checking Rust compiler'
rustc = sys 'cd rust; rustc --version'

if !(rustc.include?('rustc 1.6.0') || rustc.include?('rustc 1.7.0'))
  puts "=> Bad Rust compiler version: #{rustc}"
  puts '   Version 1.6.0 or 1.7.0 is required.'

  raise "Invalid Rust compiler version"
end

# Create an empty makefile with an empty `install` task
puts ' - Creating Makefile'
File.open('Makefile', 'w') do |f|
  body = [
    "install:",
    "\tcd rust; cargo build --release",
    "\tmkdir -p ext/fast_browser",
    "\tcp rust/target/release/libfast_browser.* ext/fast_browser"
  ]
  f.puts(body.join("\n") + "\n")
end
