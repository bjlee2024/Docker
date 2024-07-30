package main

import (
	"fmt"
	"runtime"
)

func main() {
	fmt.Printf(
		"This is running on %s/%s\n",
		runtime.GOOS,
		runtime.GOARCH,
	)
}
