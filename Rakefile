# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("nightingale.gemspec")

RbSys::ExtensionTask.new("nightingale", GEMSPEC) do |ext|
  ext.lib_dir = "lib/nightingale"
end

task default: %i[compile spec]
