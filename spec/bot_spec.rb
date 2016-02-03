require 'spec_helper'

describe FastBrowser do
  let(:googlebot) { 'Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)' }

  describe '#bot?' do
    it 'returns true for Googlebot' do
      browser = FastBrowser.new googlebot

      expect(browser.bot?).to eq true
    end
  end

  describe '#botname' do
    it 'returns "Googlebot" for Googlebot' do
      browser = FastBrowser.new googlebot

      expect(browser.bot_name).to eq 'Googlebot'
    end
  end
end
