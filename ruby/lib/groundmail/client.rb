# frozen_string_literal: true

module Groundmail
  class Client
    attr_reader :emails, :domains, :webhooks, :templates, :suppressions

    def initialize(api_key, base_url: HttpClient::DEFAULT_API_BASE_URL, transport: nil)
      http = HttpClient.new(api_key, base_url: base_url, transport: transport)
      @emails = EmailsResource.new(http)
      @domains = DomainsResource.new(http)
      @webhooks = WebhooksResource.new(http)
      @templates = TemplatesResource.new(http)
      @suppressions = SuppressionsResource.new(http)
      @http = http
    end

    def request(method, path, body: nil, headers: {})
      @http.request(method, path, body: body, headers: headers)
    end

    # Check one address before you accept it (for example on a signup form). Free,
    # and sends nothing: syntax, mail server, disposable and role addresses, recent
    # hard bounces, and a typo suggestion. Returns "verdict" ("valid", "risky" or
    # "invalid"), "reasons" and "suggestion".
    #
    # mailbox_check: true also confirms the mailbox exists: $0.01 per answer marked
    # "billed" (pay as you go or Dedicated; needs an emails:send key). tenant:
    # attributes mailbox checks to one of your tenants.
    def validate(email, mailbox_check: false, tenant: nil)
      @http.request("POST", "/validate", body: { "email" => email }.merge(validate_options(mailbox_check, tenant)))
    end

    # Check up to 100 addresses in one request, each answered as validate would.
    # Returns "data", one result per address in order.
    def validate_batch(emails, mailbox_check: false, tenant: nil)
      @http.request("POST", "/validate/batch", body: { "emails" => emails }.merge(validate_options(mailbox_check, tenant)))
    end

    private

    def validate_options(mailbox_check, tenant)
      options = {}
      options["mailbox_check"] = true if mailbox_check
      options["tenant"] = tenant if tenant && !tenant.empty?
      options
    end
  end
end
