require 'spec_helper'

describe FastBrowser do
  let(:firefox)       { 'Mozilla/5.0 (Windows NT 6.1; WOW64; rv:40.0) Gecko/20100101 Firefox/40.1' }
  let(:mobile_safari) { 'Mozilla/5.0 (iPad; CPU OS 6_0 like Mac OS X) AppleWebKit/536.26 (KHTML, like Gecko) Version/6.0 Mobile/10A5355d Safari/8536.25' }

  it 'parses Firefox name' do
    browser = FastBrowser.new firefox

    expect(browser.family).to eq 'Firefox'
  end

  it 'parses Chrome name' do
    browser = FastBrowser.new 'Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2228.0 Safari/537.36'

    expect(browser.family).to eq 'Chrome'
  end

  it 'returns Other if not parses matches' do
    browser = FastBrowser.new 'Mozilla/5.0 Unknwon/1.2'

    expect(browser.family).to eq 'Other'
  end

  it 'returns true if it is a mobile browser' do
    browser = FastBrowser.new mobile_safari

    expect(browser.mobile?).to eq true
  end

  it "returns false if it isn't a mobile browser" do
    browser = FastBrowser.new firefox

    expect(browser.mobile?).to eq false
  end

  describe '#user_agent' do
    it 'returns the original user agent' do
      nonsense = 'abc123'
      browser  = FastBrowser.new nonsense

      expect(browser.user_agent).to eq nonsense
    end
  end
end
