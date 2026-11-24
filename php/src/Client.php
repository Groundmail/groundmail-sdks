<?php

declare(strict_types=1);

namespace Groundmail;

final class Client
{
    private readonly HttpClient $http;

    public readonly EmailsResource $emails;
    public readonly DomainsResource $domains;
    public readonly WebhooksResource $webhooks;
    public readonly TemplatesResource $templates;
    public readonly SuppressionsResource $suppressions;

    /** @param callable(string, string, ?array<string, mixed>, array<string, string>): array<string, mixed>|null $transport */
    public function __construct(
        string $apiKey,
        string $baseUrl = HttpClient::DEFAULT_API_BASE_URL,
        mixed $transport = null,
    ) {
        $this->http = new HttpClient($apiKey, $baseUrl, $transport);
        $this->emails = new EmailsResource($this->http);
        $this->domains = new DomainsResource($this->http);
        $this->webhooks = new WebhooksResource($this->http);
        $this->templates = new TemplatesResource($this->http);
        $this->suppressions = new SuppressionsResource($this->http);
    }

    /**
     * @param array<string, mixed>|null $body
     * @param array<string, string> $headers
     *
     * @return array<string, mixed>
     */
    public function request(string $method, string $path, ?array $body = null, array $headers = []): array
    {
        return $this->http->request($method, $path, $body, $headers);
    }

    /**
     * Check one address before you accept it (for example on a signup form). Free, and
     * sends nothing: syntax, mail server, disposable and role addresses, recent hard
     * bounces, and a typo suggestion. Returns `verdict` (`valid`, `risky` or `invalid`),
     * `reasons` and `suggestion`.
     *
     * `$mailboxCheck` also confirms the mailbox exists: $0.01 per answer marked `billed`
     * (pay as you go or Dedicated; needs an `emails:send` key). `$tenant` attributes
     * mailbox checks to one of your tenants.
     *
     * @return array<string, mixed>
     */
    public function validate(string $email, bool $mailboxCheck = false, ?string $tenant = null): array
    {
        return $this->http->request('POST', '/validate', ['email' => $email] + self::validateOptions($mailboxCheck, $tenant));
    }

    /**
     * Check up to 100 addresses in one request, each answered as validate() would.
     * Returns `data`, one result per address in order.
     *
     * @param list<string> $emails
     *
     * @return array<string, mixed>
     */
    public function validateBatch(array $emails, bool $mailboxCheck = false, ?string $tenant = null): array
    {
        return $this->http->request('POST', '/validate/batch', ['emails' => $emails] + self::validateOptions($mailboxCheck, $tenant));
    }

    /** @return array<string, mixed> */
    private static function validateOptions(bool $mailboxCheck, ?string $tenant): array
    {
        $options = [];
        if ($mailboxCheck) {
            $options['mailbox_check'] = true;
        }
        if ($tenant !== null && $tenant !== '') {
            $options['tenant'] = $tenant;
        }

        return $options;
    }
}
