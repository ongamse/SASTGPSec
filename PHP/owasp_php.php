<?php
$servername = "localhost";
$username = "root";
$password = "rootPassword@123";
$dbname = "test";

$conn = new mysqli($servername, $username, $password, $dbname);
if ($conn->connect_error) {
    die("Connection failed: " . $conn->connect_error);
}


if (isset($_GET['username'])) {
    $userInput = $_GET['username'];
    $query = "SELECT * FROM users WHERE username = '$userInput'";
    $result = $conn->query($query);
}


if (isset($_GET['secure_username'])) {
    $stmt = $conn->prepare("SELECT * FROM users WHERE username = ?");
    $stmt->bind_param("s", $_GET['secure_username']);
    $stmt->execute();
}


if (isset($_GET['broken_auth'])) {
    $username = $_GET['username'];
    $password = $_GET['password'];
    if ($username === "adminuser" && $password === "xchzdhkrltu") {
        echo "Authenticated as admin";
    } else {
        echo "Authentication failed";
    }
}


if (isset($_GET['store_sensitive_data'])) {
    file_put_contents("passwords.txt", "admin:Passwo#d@&1957");
    echo "Password stored in plaintext";
}


if (isset($_GET['xml'])) {
    $xml = $_GET['xml'];
    $dom = new DOMDocument();
    $dom->loadXML($xml, LIBXML_NOENT);
}


if (isset($_GET['role'])) {
    $role = $_GET['role'];
    if ($role === "admin") {
        echo "Welcome Admin!";
    } else {
        echo "Access Denied!";
    }
}


header("X-Powered-By: PHP");


if (isset($_GET['input'])) {
    echo "<html><body>" . $_GET['input'] . "</body></html>";
}


if (isset($_POST['data'])) {
    $data = unserialize($_POST['data']);
    print_r($data);
}
?>
