#!/usr/bin/perl
use strict;
use warnings;
use CGI;
use DBI;
use XML::Simple;
use IO::File;

my $cgi = CGI->new;


sub sql_injection {
    my $user_input = $cgi->param('username');
    my $dbh = DBI->connect("DBI:mysql:database=test;host=localhost", "root", "rootPassword@123", { RaiseError => 1, AutoCommit => 1 });
    my $query = "SELECT * FROM users WHERE username = '$user_input'";
    my $sth   = $dbh->prepare($query);
    $sth->execute();
}


sub broken_auth {
    my ($username, $password) = @_;
    return ($username eq "adminuser" && $password eq "xchzdhkrltu");
}


sub store_sensitive_data {
    open(my $fh, '>', 'passwords.txt') or die "Could not open file: $!";
    print $fh "admin:Passwo#d@&1957";
    close $fh;
}


sub parse_xml {
    my $xml_data = $cgi->param('xmlData');
    my $xml = XML::Simple->new();
    my $data = $xml->XMLin($xml_data);
}


sub access_control {
    my $role = $cgi->param('role');

    if ($role eq "admin") {
        print "Welcome Admin!";
    } else {
        print "Access Denied!";
    }
}


sub security_misconfig {
    print "X-Powered-By: Perl CGI";
}


sub xss_vulnerability {
    my $user_input = $cgi->param('input');
    print "<html><body>$user_input</body></html>";
}


sub insecure_deserialization {
    my $data = $cgi->param('data');
    my $eval_data = eval $data;
}

print "Content-Type: text/html\n\n";
print "<html><body>Vulnerable Perl Application</body></html>";
