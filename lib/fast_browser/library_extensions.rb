class FastBrowser
  module LibraryExtensions
    def attach_string_returning_function(name, arg_types)
      private_name = "_#{name}".to_sym

      attach_function private_name, name, arg_types, :strptr

      class_eval <<-BODY
        def self.#{name}(*args)
          call_and_free_string :#{private_name}, *args
        end
      BODY
    end
  end
end
