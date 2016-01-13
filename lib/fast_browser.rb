require 'ffi'

class FastBrowser
  module RustLib
    extend FFI::Library
    ffi_lib File.expand_path('../../rust/target/debug/libfast_browser.dylib', __FILE__)

    attach_function :parse_browser, [:string], :pointer

    %w(chrome edge opera).each do |tester|
      attach_function "is_#{tester}".to_sym, [:pointer], :bool
    end
  end

  def initialize(string)
    @pointer = RustLib.parse_browser(string)
  end

  def opera?;  RustLib.is_opera(@pointer)  end
  def chrome?; RustLib.is_chrome(@pointer) end
  def edge?;   RustLib.is_edge(@pointer)   end
end
