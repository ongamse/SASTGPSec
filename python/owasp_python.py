import sqlite3
import xml.etree.ElementTree as ET
import pickle
import os
from flask import Flask, request, make_response

app = Flask(__name__)


def sql_injection(user_input):
    conn = sqlite3.connect("test.db")
    cursor = conn.cursor()
    query = f"SELECT * FROM users WHERE username = '{user_input}'"
    cursor.execute(query)
    result = cursor.fetchall()
    conn.close()
    return result


def broken_auth(username, password):
    return username == "adminuser" and password == "xchzdhkrltu"


def store_sensitive_data():
    with open("passwords.txt", "w") as f:
        f.write("admin:Passwo#d@&1957")


def parse_xml(xml_data):
    tree = ET.ElementTree(ET.fromstring(xml_data))
    return tree


@app.route('/access_control', methods=['GET'])
def access_control():
    role = request.args.get('role')
    if role == "admin":
        return "Welcome Admin!"
    return "Access Denied!"

@app.route('/security_misconfig', methods=['GET'])
def security_misconfig():
    response = make_response("Security Misconfiguration Example")
    response.headers['X-Powered-By'] = "Python-Flask"
    return response


@app.route('/xss', methods=['GET'])
def xss_vulnerability():
    user_input = request.args.get('input')
    return f"<html><body>{user_input}</body></html>"


def insecure_deserialization(serialized_data):
    return pickle.loads(serialized_data)

if __name__ == "__main__":
    app.run(debug=True)
