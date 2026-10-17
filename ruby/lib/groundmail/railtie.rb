# frozen_string_literal: true

require "rails/railtie"
require_relative "delivery_method"

module Groundmail
  class Railtie < ::Rails::Railtie
    # Runs before Action Mailer applies config.action_mailer.*: registering the
    # delivery method resets groundmail_settings, so doing it afterwards would
    # drop the settings from the app config.
    initializer "groundmail.add_delivery_method", before: "action_mailer.set_configs" do
      ActiveSupport.on_load(:action_mailer) do
        ActionMailer::Base.add_delivery_method :groundmail, Groundmail::DeliveryMethod
      end
    end
  end
end
