require 'spec_helper'
require 'get_process_mem'

describe FastBrowser do
  let(:firefox) { 'Mozilla/5.0 (Windows NT 6.1; WOW64; rv:40.0) Gecko/20100101 Firefox/40.1' }
  let(:chrome)  { 'Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2228.0 Safari/537.36' }

  it 'seems to be memory-safe' do
    b1 = FastBrowser.new firefox
    expect(b1.firefox?).to eq true
    expect(b1.chrome?).to eq false

    b2 = FastBrowser.new chrome
    expect(b2.firefox?).to eq false
    expect(b2.chrome?).to eq true

    # Then check each against to make sure they didn't overwrite each other
    # or something

    expect(b1.firefox?).to eq true
    expect(b1.chrome?).to eq false

    expect(b2.firefox?).to eq false
    expect(b2.chrome?).to eq true
  end

  ITERATIONS = 1000

  def get_process_kb
    GC.start
    GetProcessMem.new.kb
  end

  it "doesn't leak memory" do
    # Collect kilobyte differences
    sample = 20.times.map do
      before_kb = get_process_kb

      ITERATIONS.times do
        FastBrowser.new firefox
        FastBrowser.new chrome
      end

      get_process_kb - before_kb
    end

    # At least one of the sample runs needs to be memory-constant
    expect(sample).to include(0.0)

    # Sanity check that at least a few iterations caused allocations
    expect(sample.select { |s| s > 0.0 }).not_to be_empty
  end
end
