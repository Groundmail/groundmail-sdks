# Groundmail Go SDK

Official Go client for the [Groundmail](https://groundmail.dev?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks) transactional email API.

```bash
go get github.com/Groundmail/groundmail-sdks/go@v0.8.2
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

Docs: https://docs.groundmail.dev/sdks/go?utm_source=github&utm_medium=readme&utm_campaign=groundmail-sdks
