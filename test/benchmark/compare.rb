require_relative '../test_helper'

require 'benchmark'
require 'minitest/autorun'

require 'browser'
require 'ruby-prof'

class TestCompare < Minitest::Test
  TIMES = 10_000

  FIREFOX_40 = 'Mozilla/5.0 (Windows NT 6.1; WOW64; rv:40.0) Gecko/20100101 Firefox/40.1'
  SAFARI_7   = 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_3) AppleWebKit/537.75.14 (KHTML, like Gecko) Version/7.0.3 Safari/7046A194A'

  def test_compare_with_browser
    ua1 = FIREFOX_40
    ua2 = SAFARI_7
    result = nil

    # Make sure the Rust lib is hot
    _ = FastBrowser.new ua1
    _ = FastBrowser.new ua2

    test_browser = lambda do
      TIMES.times do
        b1 = Browser.new user_agent: ua1
        assert_equal b1.firefox?, true
        assert_equal "40", b1.version

        b2 = Browser.new user_agent: ua2
        assert_equal b2.safari?, true
        assert_equal "7", b2.version
      end
    end

    test_fast_browser = lambda do
      TIMES.times do
        b1 = FastBrowser.new ua1
        assert_equal b1.firefox?, true
        assert_equal 40, b1.major_version

        b2 = FastBrowser.new ua2
        assert_equal b2.safari?, true
        assert_equal 7, b2.major_version
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
