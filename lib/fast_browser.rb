require 'ffi'

class FastBrowser
  module RustLib
    extend FFI::Library

    lib_file = "libfast_browser.#{FFI::Platform::LIBSUFFIX}"
    ffi_lib File.expand_path("../../rust/target/debug/#{lib_file}", __FILE__)

    attach_function :parse_user_agent, [:string], :pointer

    %w(chrome edge firefox opera safari).each do |tester|
      attach_function "is_#{tester}".to_sym, [:pointer], :bool
    end

    attach_function :get_browser_minor_version, [:pointer], :int8
    attach_function :get_browser_major_version, [:pointer], :int8
    attach_function :get_browser_family, [:pointer], :strptr
    attach_function :free_string, [:pointer], :void

    # Sends the given method name (`method`) to self, copies the returned
    # string into a Ruby string and then calls `.free_string` to deallocate
    # the original returned string.
    def self.call_and_free_string method, *args
      string, ptr = self.send method, *args
      self.free_string ptr
      string
    end
  end

  def initialize(string)
    @pointer = RustLib.parse_user_agent(string)
  end

  def chrome?;  RustLib.is_chrome(@pointer)  end
  def edge?;    RustLib.is_edge(@pointer)    end
  def firefox?; RustLib.is_firefox(@pointer) end
  def opera?;   RustLib.is_opera(@pointer)   end
  def safari?;  RustLib.is_safari(@pointer)  end

  def major_version; RustLib.get_browser_major_version(@pointer) end
  def minor_version; RustLib.get_browser_minor_version(@pointer) end

  def family
    RustLib.call_and_free_string :get_browser_family, @pointer
  end
end
