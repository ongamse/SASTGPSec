import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.ResultSet;
import java.sql.Statement;
import java.io.*;
import java.net.*;
import javax.servlet.http.*;

public class VulnerableJava {

    public void sqlInjection(String userInput) {
        try {
            Connection conn = DriverManager.getConnection("jdbc:mysql://localhost:3306/test", "root", "rootPassword@123");
            Statement stmt = conn.createStatement();
            String query = "SELECT * FROM users WHERE username = '" + userInput + "'";
            ResultSet rs = stmt.executeQuery(query);
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public void secureSQLQuery(String userInput) {
        try {
            Connection conn = DriverManager.getConnection("jdbc:mysql://localhost:3306/test", "root", "rootPassword@123");
            String query = "SELECT * FROM users WHERE username = ?";
            PreparedStatement pstmt = conn.prepareStatement(query);
            pstmt.setString(1, userInput);
            ResultSet rs = pstmt.executeQuery();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public boolean brokenAuth(String username, String password) {
        return username.equals("adminuser") && password.equals("xchzdhkrltu");
    }

    public void storeSensitiveData() {
        try {
            FileWriter fw = new FileWriter("passwords.txt");
            fw.write("admin:Passwo#d@&1957");
            fw.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public void parseXML(String xmlData) {
        try {
            javax.xml.parsers.DocumentBuilderFactory factory = javax.xml.parsers.DocumentBuilderFactory.newInstance();
            factory.setFeature("http://apache.org/xml/features/disallow-doctype-decl", false);
            javax.xml.parsers.DocumentBuilder builder = factory.newDocumentBuilder();
            builder.parse(new ByteArrayInputStream(xmlData.getBytes()));
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public void accessControl(HttpServletRequest request, HttpServletResponse response) throws IOException {
        String role = request.getParameter("role");
        if (role.equals("admin")) {
            response.getWriter().println("Welcome Admin!");
        } else {
            response.getWriter().println("Access Denied!");
        }
    }

    public void securityMisconfig(HttpServletResponse response) {
        response.setHeader("X-Powered-By", "Java");
    }

    public void xssVulnerability(HttpServletRequest request, HttpServletResponse response) throws IOException {
        String userInput = request.getParameter("input");
        response.getWriter().println("<html><body>" + userInput + "</body></html>");
    }

    public void insecureDeserialization(byte[] data) {
        try {
            ObjectInputStream in = new ObjectInputStream(new ByteArrayInputStream(data));
            Object obj = in.readObject();
            in.close();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
