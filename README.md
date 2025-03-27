
## Overview
SAST-Benchmark-Suite is an open-source repository designed to help security professionals, developers, and organizations evaluate Static Application Security Testing (SAST) tools. This repository contains intentionally vulnerable code in multiple programming languages, covering a wide range of security flaws that align with OWASP Top 10 and CWE (Common Weakness Enumeration) categories.

By leveraging this repository, users can test and compare the effectiveness of various SAST solutions in detecting vulnerabilities, assessing false positives/negatives, and improving their security scanning workflows.

## Purpose
Many security engineers and organizations struggle to determine which SAST tool best suits their needs. This repository serves as a benchmarking ground for testing the accuracy, efficiency, and capability of different SAST vendors in detecting real-world security vulnerabilities across various languages.

Users can:

✅ Test how well a SAST tool detects vulnerabilities in different languages.

✅ Evaluate a tool’s ability to identify OWASP Top 10 vulnerabilities and logic flaws.

✅ Compare the results from different SAST vendors to select the best solution.

✅ Use this as a training resource for developers and security teams.

## Languages & Vulnerabilities Covered
This repository contains vulnerable implementations in multiple languages, including:

- Python
- Java
- C++
- C#
- JavaScript (Node.js)
- Go
- Rust
- PHP
- Ruby
- Swift
- Perl

#### Each language-specific implementation contains deliberate security flaws, such as:

- SQL Injection (SQLi)
- Cross-Site Scripting (XSS)
- Insecure Deserialization
- Broken Authentication & Hardcoded Credentials
- Security Misconfigurations
- XML External Entity Injection (XXE)
- Improper Access Control
- Insecure File Storage & Exposure
- Sensitive Data Leakage

#### Each vulnerable implementation comes with a detailed vulnerability report that outlines:

- Vulnerability name
- Severity
- Description
- Impact
- Remediation steps
- Line number & file location

## Who Should Use This?

🔹 Security Engineers – Benchmark different SAST tools for better selection.

🔹 Developers – Learn about secure coding practices.

🔹 DevSecOps Teams – Integrate SAST tools into CI/CD pipelines.

🔹 Researchers & Students – Study real-world vulnerabilities and their impact.
