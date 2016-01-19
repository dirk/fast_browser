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

    test_browser = lambda do
      TIMES.times do
        b = Browser.new user_agent: ua
        assert_equal b.firefox?, true
        assert_equal "40", b.version
      end
    end

    test_fast_browser = lambda do
      TIMES.times do
        b = FastBrowser.new ua
        assert_equal b.firefox?, true
        assert_equal 40, b.major_version
      end
    end

    {
      'browser' => test_browser,
      'fast_browser' => test_fast_browser
    }.each do |(name, test_block)|
      time    = benchmark_time &test_block
      objects = benchmark_memory &test_block

      print "#{name.ljust(15)}%10.5f seconds\t%d objects\n" % [time, objects]
    end
  end

  private

  def profile(&block)
    result = RubyProf.profile &block
    RubyProf::FlatPrinter.new(result).print(STDOUT)
  end

  def benchmark_time &block
    Benchmark.realtime &block
  end

  def benchmark_memory &block
    GC.start

    before_allocated = GC.stat[:total_allocated_objects]
    GC.disable

    block.call

    after_allocated = GC.stat[:total_allocated_objects]
    GC.enable

    return after_allocated - before_allocated
  end
end
