require_relative '../test_helper'

require 'benchmark'
require 'minitest/autorun'

require 'browser'
require 'ruby-prof'

class TestCompare < Minitest::Test
  TIMES = 50_000

  FIREFOX_40 = 'Mozilla/5.0 (Windows NT 6.1; WOW64; rv:40.0) Gecko/20100101 Firefox/40.1'

  def test_compare_with_browser
    ua = FIREFOX_40
    result = nil

    # Make sure the Rust lib is hot
    _ = FastBrowser.new ua

    Benchmark.bm(12) do |x|
      x.report('browser:') {
        TIMES.times do
          b = Browser.new user_agent: ua
          assert_equal b.firefox?, true
          assert_equal "40", b.version
        end
      }
      x.report('fast_browser:') {
        TIMES.times do
          b = FastBrowser.new ua
          assert_equal b.firefox?, true
          assert_equal 40, b.major_version
        end
      }
    end
  end

  private

  def profile(&block)
    result = RubyProf.profile &block
    RubyProf::FlatPrinter.new(result).print(STDOUT)
  end
end
