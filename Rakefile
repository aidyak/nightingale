# frozen_string_literal: true

require "date"
require "bundler/gem_tasks"
require "open3"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("nightingale-play.gemspec")

RbSys::ExtensionTask.new("nightingale-play", GEMSPEC) do |ext|
  ext.lib_dir = "lib/nightingale_play"
end

module ReleaseNotes
  module_function

  CATEGORY_RULES = {
    "Added" => [
      /\Afeat(?:\(.+\))?!?:/i,
      /\Aadd\b/i,
      /\Acreate\b/i,
      /\Aenable\b/i
    ],
    "Fixed" => [
      /\Afix(?:\(.+\))?!?:/i,
      /\Afix\b/i
    ],
    "Removed" => [
      /\Aremove\b/i,
      /\Adelete\b/i,
      /\Adrop\b/i
    ],
    "Changed" => [
      /\Achange\b/i,
      /\Aedit\b/i,
      /\Amake\b/i,
      /\Aupdate\b/i,
      /\Ause\b/i,
      /\Abump\b/i,
      /\Arefactor\b/i,
      /\Aimprove\b/i
    ]
  }.freeze

  CATEGORY_ORDER = ["Added", "Changed", "Fixed", "Removed"].freeze

  def commits(from_ref:, to_ref:)
    range = from_ref && !from_ref.empty? ? "#{from_ref}..#{to_ref}" : to_ref
    output, status = Open3.capture2("git", "log", range, "--pretty=format:%s")
    raise "git log failed" unless status.success?

    output.lines(chomp: true).reject(&:empty?)
  end

  def latest_tag
    tag, status = Open3.capture2("git", "describe", "--tags", "--abbrev=0", err: File::NULL)
    status.success? ? tag.strip : nil
  end

  def categorize(messages)
    grouped = Hash.new { |hash, key| hash[key] = [] }

    messages.each do |message|
      category = CATEGORY_RULES.find { |_, patterns| patterns.any? { |pattern| pattern.match?(message) } }&.first || "Changed"
      grouped[category] << normalize(message)
    end

    grouped
  end

  def normalize(message)
    normalized = message.dup
    normalized.sub!(/\A(?:feat|fix|docs|chore|refactor)(?:\(.+\))?!?:\s*/i, "")
    normalized = normalized.strip
    normalized[0] = normalized[0].upcase if normalized[0]
    normalized
  end

  def render(version:, date:, from_ref:, to_ref:)
    messages = commits(from_ref:, to_ref:)
    raise "No commits found for release notes" if messages.empty?

    grouped = categorize(messages)
    lines = []
    lines << "## [#{version}] - #{date}"
    lines << ""

    CATEGORY_ORDER.each do |category|
      next if grouped[category].empty?

      lines << "### #{category}"
      grouped[category].each do |message|
        lines << "- #{message}"
      end
      lines << ""
    end

    lines.join("\n").rstrip + "\n"
  end

  def update_changelog!(path:, version:, date:, from_ref:, to_ref:)
    release_notes = render(version:, date:, from_ref:, to_ref:)
    content = File.read(path)
    raise "Version #{version} already exists in #{path}" if content.include?("## [#{version}]")
    raise "Missing Unreleased section in #{path}" unless content.include?("## [Unreleased]")

    updated = content.sub("## [Unreleased]\n", "## [Unreleased]\n\n#{release_notes}\n")
    File.write(path, updated)
  end
end

namespace :release do
  desc "Print release notes from the previous tag to HEAD"
  task :notes do
    version = ENV["VERSION"] || NightingalePlay::VERSION
    date = ENV["DATE"] || Date.today.iso8601
    from_ref = ENV["FROM"] || ReleaseNotes.latest_tag
    to_ref = ENV["TO"] || "HEAD"

    puts ReleaseNotes.render(version:, date:, from_ref:, to_ref:)
  end

  desc "Write release notes into CHANGELOG.md under Unreleased"
  task :changelog do
    version = ENV.fetch("VERSION")
    date = ENV["DATE"] || Date.today.iso8601
    from_ref = ENV["FROM"] || ReleaseNotes.latest_tag
    to_ref = ENV["TO"] || "HEAD"

    ReleaseNotes.update_changelog!(
      path: "CHANGELOG.md",
      version:,
      date:,
      from_ref:,
      to_ref:
    )
  end
end

task default: %i[compile spec]
