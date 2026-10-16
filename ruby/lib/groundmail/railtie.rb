# frozen_string_literal: true

require "rails/railtie"
require_relative "delivery_method"

module Groundmail
  class Railtie < ::Rails::Railtie
    initializer "groundmail.add_delivery_method" do
      ActiveSupport.on_load(:action_mailer) do
        ActionMailer::Base.add_delivery_method :groundmail, Groundmail::DeliveryMethod
      end
    end
  end
end
