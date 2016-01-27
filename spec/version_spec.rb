require 'spec_helper'

describe FastBrowser do
  let(:chrome)     { 'Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2228.0 Safari/537.36' }
  let(:firefox)    { 'Mozilla/5.0 (Windows NT 6.1; WOW64; rv:40.0) Gecko/20100101 Firefox/40.1' }
  let(:opera_mini) { 'Opera/9.80 (J2ME/MIDP; Opera Mini/9.80 (S60; SymbOS; Opera Mobi/23.348; U; en) Presto/2.5.25 Version/10.54' }

  it 'parses Chrome versions' do
    browser = FastBrowser.new chrome

    expect(browser.major_version).to eq 41
    expect(browser.minor_version).to eq 0
  end

  it 'parses Firefox versions' do
    browser = FastBrowser.new firefox

    expect(browser.major_version).to eq 40
    expect(browser.minor_version).to eq 1
  end

  it 'parses Opera Mini versions' do
    browser = FastBrowser.new opera_mini

    expect(browser.major_version).to eq 9
    expect(browser.minor_version).to eq 80
  end
end
