use dns_example::Dns;

fn main() {
    let dns_a = Dns::new()
        .website("classroom.google.com", "172.16.17.32")
        .build();

    let dns_b = Dns::new()
        .website("docs.google.com", "172.27.12.34")
        .contact(&dns_a)
        .build();

    println!("{:#?}", dns_b.lookup("classroom.google.com").unwrap())
}
