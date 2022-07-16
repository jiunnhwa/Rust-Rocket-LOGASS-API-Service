package main

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"strings"
	"sync"
	"time"
)

func main() {
	var wg sync.WaitGroup
	start := time.Now()
	for i := 0; i < 99; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			url := "http://localhost:8000/log"
			payload := strings.NewReader("\"master rust\"")
			req, _ := http.NewRequest("POST", url, payload)
			req.Header.Add("Content-Type", "application/json")
			res, _ := http.DefaultClient.Do(req)
			defer res.Body.Close()
			body, _ := ioutil.ReadAll(res.Body)
			fmt.Println(res)
			fmt.Println(string(body))
		}()
	}
	wg.Wait()
	duration := time.Since(start)
	fmt.Println("Duration:", duration.Seconds())
}

// func post() {

// 	url := "http://localhost:8000/log"

// 	payload := strings.NewReader("\"master rust\"")
// 	req, _ := http.NewRequest("POST", url, payload)
// 	req.Header.Add("Content-Type", "application/json")
// 	res, _ := http.DefaultClient.Do(req)
// 	defer res.Body.Close()
// 	body, _ := ioutil.ReadAll(res.Body)
// 	fmt.Println(res)
// 	fmt.Println(string(body))
// }
