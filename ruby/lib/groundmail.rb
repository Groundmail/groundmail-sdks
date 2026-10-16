# frozen_string_literal: true

require_relative "groundmail/version"
require_relative "groundmail/error"
require_relative "groundmail/http"
require_relative "groundmail/resources"
require_relative "groundmail/client"

module Groundmail
end

begin
  require "base64"
  require "mail"
rescue LoadError
  # base64/mail are optional; required for ActionMailer delivery method.
else
  require_relative "groundmail/message_mapper"
  require_relative "groundmail/delivery_method"
end

begin
  require "rails"
rescue LoadError
  # Rails is optional.
else
  require_relative "groundmail/railtie"
end
