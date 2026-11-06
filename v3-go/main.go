package main

import (
	"encoding/json"
	"log"
	"net/http"
)

const mod = 1000000

type fibRequest struct {
	N int64 `json:"n"`
}

type fibResponse struct {
	Result int64 `json:"result"`
	N      int64 `json:"n"`
}

func fibonacci(n int64) int64 {
	if n <= 0 {
		return 0
	}
	if n == 1 {
		return 1
	}
	var a, b int64 = 0, 1
	for i := int64(2); i <= n; i++ {
		c := (a + b) % mod
		a = b
		b = c
	}
	return b
}

func writeJSON(w http.ResponseWriter, status int, v any) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	_ = json.NewEncoder(w).Encode(v)
}

func handleFibonacci(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		writeJSON(w, http.StatusMethodNotAllowed, map[string]string{"error": "method not allowed"})
		return
	}
	var req fibRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		writeJSON(w, http.StatusBadRequest, map[string]string{"error": "invalid request body"})
		return
	}
	if req.N < 0 {
		writeJSON(w, http.StatusBadRequest, map[string]string{"error": "n must be non-negative"})
		return
	}
	writeJSON(w, http.StatusOK, fibResponse{Result: fibonacci(req.N), N: req.N})
}

func handleHealth(w http.ResponseWriter, r *http.Request) {
	writeJSON(w, http.StatusOK, map[string]bool{"ok": true})
}

func main() {
	http.HandleFunc("/health", handleHealth)
	http.HandleFunc("/fibonacci", handleFibonacci)
	port := "8081"
	log.Printf("v3-go HTTP service listening on :%s", port)
	log.Fatal(http.ListenAndServe(":"+port, nil))
}