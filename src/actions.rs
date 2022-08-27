use super::command;

fn extract_server_domain(servers_out: Vec<&str>) -> Vec<&str> {
    let mut servers = Vec::new();

    for s in servers_out {
        let out: Vec<&str> = s.split(" ").collect();

        servers.push(out[out.len() - 1]);
    }

    return servers;
}

pub fn check_zone_transfer(host: &str) {
    let host_servers_out = command::run(&["host", "-t", "ns", host]);

    let mut host_servers: Vec<&str> = host_servers_out.split("\n").collect();
    host_servers = extract_server_domain(host_servers);

    if host_servers.len() <= 0 {
        panic!("There's no name servers on the host: {}", host);
    }

    let mut outputs = Vec::new();

    for server in host_servers {
        let zone_transfer_out = command::run(&["host", "-t", "axfr", host, server]);
        if !zone_transfer_out.contains("; Transfer failed") {
            let prev_output_len = outputs.len();
            outputs.push(zone_transfer_out);
            assert_eq!(outputs.len(), prev_output_len + 1);
        }
    }

    if outputs.len() > 0 {
        println!("{}", outputs.join("\n"));
    } else {
        println!("Zone transfer is not available for the host: {}", host);
    }
}
