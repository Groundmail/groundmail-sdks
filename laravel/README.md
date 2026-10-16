# Groundmail Laravel package

Laravel Mail transport and integration for the [Groundmail](https://groundmail.dev) transactional email API.

```bash
composer require groundmail/laravel
```

Set in `.env`:

```bash
GROUNDMAIL_API_KEY=gm_your_key_here
MAIL_MAILER=groundmail
```

Add to `config/mail.php`:

```php
'groundmail' => [
    'transport' => 'groundmail',
],
```

Then send as usual:

```php
Mail::to($user)->send(new WelcomeMail($user));
```

Optional: use the Facade / PHP client for direct API calls:

```php
use Groundmail\Laravel\Facades\Groundmail;

Groundmail::client()->emails->send([
    'from' => 'you@yourdomain.com',
    'to' => 'user@example.com',
    'subject' => 'Hello',
    'html' => '<p>It works.</p>',
]);
```

Docs: https://docs.groundmail.dev/frameworks/laravel
