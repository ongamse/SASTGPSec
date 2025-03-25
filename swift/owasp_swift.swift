import Foundation
import SQLite3

class VulnerableSwift {
    

    func sqlInjection(userInput: String) {
        let dbPath = "test.db"
        var db: OpaquePointer?
        
        if sqlite3_open(dbPath, &db) == SQLITE_OK {
            let query = "SELECT * FROM users WHERE username = '\(userInput)'"
            var stmt: OpaquePointer?
            
            if sqlite3_prepare_v2(db, query, -1, &stmt, nil) == SQLITE_OK {
                sqlite3_step(stmt)
            }
            sqlite3_finalize(stmt)
            sqlite3_close(db)
        }
    }
    

    func brokenAuth(username: String, password: String) -> Bool {
        return username == "adminuser" && password == "xchzdhkrltu"
    }


    func storeSensitiveData() {
        let password = "admin:Passwo#d@&1957"
        let filePath = FileManager.default.temporaryDirectory.appendingPathComponent("passwords.txt")
        
        do {
            try password.write(to: filePath, atomically: true, encoding: .utf8)
        } catch {
            print("Error storing password")
        }
    }


    func parseXML(xmlData: String) {
        let parser = XMLParser(data: xmlData.data(using: .utf8)!)
        parser.parse()
    }


    func accessControl(role: String) {
        if role == "admin" {
            print("Welcome Admin!")
        } else {
            print("Access Denied!")
        }
    }


    func securityMisconfig() {
        print("X-Powered-By: Swift")
    }


    func xssVulnerability(userInput: String) {
        let response = "<html><body>\(userInput)</body></html>"
        print(response)
    }


    func insecureDeserialization(data: Data) {
        if let object = try? NSKeyedUnarchiver.unarchiveTopLevelObjectWithData(data) {
            print("Deserialized object: \(object)")
        }
    }
}

