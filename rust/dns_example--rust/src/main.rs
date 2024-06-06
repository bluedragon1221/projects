use maplit::hashmap;
use net_literals::ipv4;

use dns_example::Dns;

fn main() {
    let dns_a = Dns::new()
        .lookup_table(hashmap![
            "drive.google.com" => ipv4!["142.250.114.138"],
            "classroom.google.com" => ipv4!["142.250.115.100"]
        ])
        .build();

    let dns_b = Dns::new()
        .website("docs.google.com", ipv4!["142.250.115.100"])
        .contact(&dns_a)
        .build();

    let dns_c = Dns::new().contact(&dns_b).build();

    println!(
        "{:#?}",
        dns_b
            .lookup("classroom.google.com")
            .expect("Could not find url")
    );

    println!(
        "{:#?}",
        dns_c
            .lookup("drive.google.com")
            .expect("Could not find url")
    );
}
