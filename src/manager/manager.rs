use rs_docker::Docker;


pub fn run_pipe() {

    let docker = match Docker::connect("unix:///var/run/docker.sock") {
    	Ok(docker) => {
            println!("{:?}", docker)
        },
        Err(e) => { panic!("{}", e); }
    };

    let networks = match docker.get_networks() {
        Ok(networks) => {
            println!("{:?}", networks)
        },
        Err(e) => { panic!("{}", e); }
    };


    let containers = match docker.get_containers(false) {
        Ok(containers) => {
            println!("{:?}", containers)
        },
        Err(e) => { panic!("{}", e); }
    };
}