require 'ffi'

require 'fast_browser/version'
require 'fast_browser/library_extensions'

class FastBrowser
  module RustLib
    extend FFI::Library
    extend LibraryExtensions

    lib_file = "libfast_browser.#{FFI::Platform::LIBSUFFIX}"
    ffi_lib File.expand_path("../../ext/fast_browser/#{lib_file}", __FILE__)

    %w(chrome edge firefox opera safari bot).each do |tester|
      attach_function "is_#{tester}".to_sym, [:pointer], :bool
    end

    attach_function :get_browser_minor_version, [:pointer], :int8
    attach_function :get_browser_major_version, [:pointer], :int8
    attach_function :is_mobile, [:pointer], :bool

    attach_string_returning_function :get_bot_name, [:pointer]
    attach_string_returning_function :get_browser_family, [:pointer]
    attach_string_returning_function :get_user_agent, [:pointer]
    attach_string_returning_function :get_version, []

    # Private Rust methods; don't call these directly!
    attach_function :_parse_user_agent, :parse_user_agent, [:string], :pointer
    attach_function :_free_user_agent,  :free_user_agent, [:pointer], :void
    attach_function :_free_string,      :free_string, [:pointer], :void

    # Sends the given method name (`method`) to self, copies the returned
    # string into a Ruby string and then calls `.free_string` to deallocate
    # the original returned string.
    def self.call_and_free_string method, *args
      string, ptr = send method, *args
      _free_string ptr
      string
    end

    def self.parse_user_agent string
      FFI::AutoPointer.new(
        self._parse_user_agent(string),
        self.method(:_free_user_agent)
      )
    end
  end

  def initialize(string)
    @pointer = RustLib.parse_user_agent(string)
  end

  # Boolean methods
  def bot?;     RustLib.is_bot(@pointer)     end
  def chrome?;  RustLib.is_chrome(@pointer)  end
  def edge?;    RustLib.is_edge(@pointer)    end
  def firefox?; RustLib.is_firefox(@pointer) end
  def opera?;   RustLib.is_opera(@pointer)   end
  def safari?;  RustLib.is_safari(@pointer)  end
  def mobile?;  RustLib.is_mobile(@pointer)  end

  # General methods
  def bot_name;   RustLib.get_bot_name(@pointer)   end
  def user_agent; RustLib.get_user_agent(@pointer) end

  # Browser-related methods
  def browser_family;        RustLib.get_browser_family(@pointer)        end
  def browser_major_version; RustLib.get_browser_major_version(@pointer) end
  def browser_minor_version; RustLib.get_browser_minor_version(@pointer) end

  alias_method :family,        :browser_family
  alias_method :major_version, :browser_major_version
  alias_method :minor_version, :browser_minor_version
end

if FastBrowser::RustLib.get_version != FastBrowser::VERSION
  e = FastBrowser::VERSION
  g = FastBrowser::RustLib.get_version
  raise "Rust library version doesn't match Ruby gem version (expected #{e}, got #{g})"
end
