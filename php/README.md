# Groundmail PHP SDK

Official PHP client for the [Groundmail](https://groundmail.dev) transactional email API.

```bash
composer require groundmail/groundmail
```

```php
<?php

use Groundmail\Client;

$tx = new Client('gm_your_key_here');

$result = $tx->emails->send([
    'from' => 'you@yourdomain.com',
    'to' => 'user@example.com',
    'subject' => 'Hello',
    'html' => '<p>It works.</p>',
]);

echo $result['id'], ' ', $result['status'], PHP_EOL;
```

Docs: https://docs.groundmail.dev/sdks/php
