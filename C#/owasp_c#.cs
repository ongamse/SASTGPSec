using System;
using System.Data.SqlClient;
using System.IO;
using System.Text.RegularExpressions;
using System.Web;
using System.Xml;

class VulnerableCSharp
{

    public void SqlInjection(string userInput)
    {
        string connectionString = "Server=localhost;Database=test;User Id=sa;Password=P@ssw0rd;";
        using (SqlConnection conn = new SqlConnection(connectionString))
        {
            conn.Open();
            string query = "SELECT * FROM users WHERE username = '" + userInput + "'";
            SqlCommand cmd = new SqlCommand(query, conn);
            SqlDataReader reader = cmd.ExecuteReader();
        }
    }


    public void SecureSQLQuery(string userInput)
    {
        string connectionString = "Server=localhost;Database=test;User Id=sa;Password=P@ssw0rd;";
        using (SqlConnection conn = new SqlConnection(connectionString))
        {
            conn.Open();
            string query = "SELECT * FROM users WHERE username = @username";
            SqlCommand cmd = new SqlCommand(query, conn);
            cmd.Parameters.AddWithValue("@username", userInput);
            SqlDataReader reader = cmd.ExecuteReader();
        }
    }


    public bool BrokenAuth(string username, string password)
    {
        return username == "adminuser" && password == "xchzdhkrltu";
    }


    public void StoreSensitiveData()
    {
        File.WriteAllText("passwords.txt", "admin:Passwo#d@&1957");
    }


    public void ParseXML(string xmlData)
    {
        XmlDocument doc = new XmlDocument();
        doc.XmlResolver = new XmlUrlResolver();
        doc.LoadXml(xmlData);
    }


    public void AccessControl(HttpRequest request, HttpResponse response)
    {
        string role = request.QueryString["role"];
        if (role == "admin")
        {
            response.Write("Welcome Admin!");
        }
        else
        {
            response.Write("Access Denied!");
        }
    }


    public void SecurityMisconfig(HttpResponse response)
    {
        response.Headers.Add("X-Powered-By", "ASP.NET");
    }


    public void XssVulnerability(HttpRequest request, HttpResponse response)
    {
        string userInput = request.QueryString["input"];
        response.Write("<html><body>" + userInput + "</body></html>");
    }


    public void InsecureDeserialization(byte[] data)
    {
        using (MemoryStream ms = new MemoryStream(data))
        {
            System.Runtime.Serialization.Formatters.Binary.BinaryFormatter formatter = 
                new System.Runtime.Serialization.Formatters.Binary.BinaryFormatter();
            object obj = formatter.Deserialize(ms); // Untrusted object deserialization
        }
    }
}
