# Groundmail Go SDK

Official Go client for the [Groundmail](https://groundmail.dev?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks) API.

```bash
go get github.com/Groundmail/groundmail-sdks/go@v0.10.0
```

```go
package main

import (
	"fmt"
	"log"

	groundmail "github.com/Groundmail/groundmail-sdks/go"
)

func main() {
	client, err := groundmail.New("gm_your_key_here")
	if err != nil {
		log.Fatal(err)
	}

	result, err := client.Emails.Send(map[string]any{
		"from":    "you@yourdomain.com",
		"to":      "user@example.com",
		"subject": "Hello",
		"html":    "<p>It works.</p>",
	})
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(result["id"], result["status"])
}
```

## Validate an address

Check an address before you accept it, for example on a signup form. It's free and sends nothing:

```go
result, err := client.Validate("ada@gmial.com", nil)
if err != nil {
	log.Fatal(err)
}
fmt.Println(result["verdict"], result["reasons"], result["suggestion"])
// risky [disposable possible_typo] ada@gmail.com
```

`verdict` is `valid`, `risky` or `invalid`. Pass `&groundmail.ValidateOptions{MailboxCheck: true}` to also confirm the mailbox exists ($0.01 per answer marked `billed`), and use `client.ValidateBatch` for up to 100 addresses at once. See https://docs.groundmail.dev/validation?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks

Docs: https://docs.groundmail.dev/sdks/go?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks
