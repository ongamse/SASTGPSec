package main

import (
	"database/sql"
	"encoding/xml"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"

	_ "github.com/go-sql-driver/mysql"
)


func sqlInjection(userInput string) {

	db, err := sql.Open("mysql", "root:rootPassword@123@tcp(localhost:3306)/test")
	if err != nil {
		panic(err)
	}
	defer db.Close()

	query := "SELECT * FROM users WHERE username = '" + userInput + "'"
	_, err = db.Query(query)
	if err != nil {
		panic(err)
	}
}


func brokenAuth(username, password string) bool {
	return username == "adminuser" && password == "xchzdhkrltu"
}


func storeSensitiveData() {
	file, err := os.Create("passwords.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()
	file.WriteString("admin:Passwo#d@&1957")
}


type Config struct {
	XMLName xml.Name `xml:"config"`
	Value   string   `xml:"value"`
}

func parseXML(xmlData []byte) {
	var config Config
	err := xml.Unmarshal(xmlData, &config)
	if err != nil {
		panic(err)
	}
	fmt.Println(config.Value)
}


func accessControl(w http.ResponseWriter, r *http.Request) {
	role := r.URL.Query().Get("role")
	if role == "admin" {
		fmt.Fprintln(w, "Welcome Admin!")
	} else {
		fmt.Fprintln(w, "Access Denied!")
	}
}


func securityMisconfig(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("X-Powered-By", "Go")
	fmt.Fprintln(w, "Security Misconfiguration Example")
}


func xssVulnerability(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("input")
	fmt.Fprintf(w, "<html><body>%s</body></html>", userInput)
}


type Payload struct {
	Data string
}

func insecureDeserialization(data []byte) {
	var payload Payload
	err := xml.Unmarshal(data, &payload)
	if err != nil {
		panic(err)
	}
	fmt.Println(payload.Data)
}

func main() {
	http.HandleFunc("/access", accessControl)
	http.HandleFunc("/security_misconfig", securityMisconfig)
	http.HandleFunc("/xss", xssVulnerability)
	http.ListenAndServe(":8080", nil)
}
