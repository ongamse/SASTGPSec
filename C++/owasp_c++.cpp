#include <iostream>
#include <fstream>
#include <string>
#include <sqlite3.h>
#include <cstdlib>
#include <vector>
#include <sstream>
#include <cstring>


void sqlInjection(std::string userInput) {
    sqlite3* db;
    char* errMsg = 0;
    sqlite3_open("test.db", &db);
    
    std::string query = "SELECT * FROM users WHERE username = '" + userInput + "'";
    sqlite3_exec(db, query.c_str(), 0, 0, &errMsg);
    
    sqlite3_close(db);
}


void secureSQLQuery(std::string userInput) {
    sqlite3* db;
    sqlite3_stmt* stmt;
    sqlite3_open("test.db", &db);
    
    std::string query = "SELECT * FROM users WHERE username = ?";
    sqlite3_prepare_v2(db, query.c_str(), -1, &stmt, 0);
    sqlite3_bind_text(stmt, 1, userInput.c_str(), -1, SQLITE_STATIC);
    
    sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    sqlite3_close(db);
}


bool brokenAuth(std::string username, std::string password) {
    return (username == "adminuser" && password == "xchzdhkrltu");
}


void storeSensitiveData() {
    std::ofstream file("passwords.txt");
    file << "admin:Passwo#d@&1957";
    file.close();
}


void parseXML(std::string xmlData) {
    if (xmlData.find("<!DOCTYPE") != std::string::npos) {
        std::cout << "Potential XXE Attack!" << std::endl;
    }
}


void accessControl(std::string role) {
    if (role == "admin") {
        std::cout << "Welcome Admin!" << std::endl;
    } else {
        std::cout << "Access Denied!" << std::endl;
    }
}


void securityMisconfig() {
    std::cout << "X-Powered-By: C++-WebServer" << std::endl;
}


void xssVulnerability(std::string userInput) {
    std::cout << "<html><body>" << userInput << "</body></html>" << std::endl;
}


void insecureDeserialization(std::string data) {
    if (data == "malicious_payload") {
        std::cout << "Warning: Untrusted data deserialized!" << std::endl;
    }
}

int main() {
    sqlInjection("admin' OR '1'='1");
    secureSQLQuery("admin");
    brokenAuth("adminuser", "xchzdhkrltu");
    storeSensitiveData();
    parseXML("<!DOCTYPE foo SYSTEM \"http://malicious.com/evil.dtd\">");
    accessControl("admin");
    securityMisconfig();
    xssVulnerability("<script>alert('XSS');</script>");
    insecureDeserialization("malicious_payload");

    return 0;
}
