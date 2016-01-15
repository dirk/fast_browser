require 'spec_helper'

describe FastBrowser do
  it 'parses Firefox versions' do
    browser = FastBrowser.new 'Mozilla/5.0 (Windows NT 6.1; WOW64; rv:40.0) Gecko/20100101 Firefox/40.1'

    expect(browser.major_version).to eq 40
    expect(browser.minor_version).to eq 1
  end
end
