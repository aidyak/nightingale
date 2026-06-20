# frozen_string_literal: true

RSpec.describe Nightingale do
  it "has a version number" do
    expect(Nightingale::VERSION).not_to be nil
  end

  it "can call into Rust" do
    result = Nightingale.hello("world")

    expect(result).to be("Hello earth, from Rust!")
  end
end
