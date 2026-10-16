# frozen_string_literal: true

require_relative "lib/groundmail/version"

Gem::Specification.new do |spec|
  spec.name = "groundmail"
  spec.version = Groundmail::VERSION
  spec.authors = ["Groundmail"]
  spec.email = ["support@groundmail.dev"]

  spec.summary = "Groundmail transactional email API client for Ruby"
  spec.description = "Official Ruby client for the Groundmail REST API."
  spec.homepage = "https://groundmail.dev"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.1.0"

  spec.metadata = {
    "homepage_uri" => spec.homepage,
    "source_code_uri" => "https://github.com/Groundmail/groundmail-sdks/tree/main/ruby",
    "documentation_uri" => "https://docs.groundmail.dev/sdks/ruby",
    "rubygems_mfa_required" => "true"
  }

  spec.files = Dir.chdir(__dir__) do
    `git ls-files -z`.split("\x0").reject { |f| f.start_with?("spec/") }
  rescue StandardError
    Dir.glob("{lib}/**/*", base: __dir__) + ["README.md", "groundmail.gemspec"]
  end

  spec.require_paths = ["lib"]

  spec.add_dependency "base64", ">= 0.2"
  spec.add_development_dependency "mail", ">= 2.8"
  spec.add_development_dependency "minitest", "~> 5.25"
end
