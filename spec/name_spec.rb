require 'spec_helper'

describe FastBrowser do
  it 'parses Firefox name' do
    browser = FastBrowser.new 'Mozilla/5.0 (Windows NT 6.1; WOW64; rv:40.0) Gecko/20100101 Firefox/40.1'

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
end
