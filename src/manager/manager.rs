use rs_docker::Docker;

pub fn run_pipe() {

    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
    	Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };

    let containers = match docker.get_containers(true) {
        Ok(containers) => containers,
        Err(e) => { panic!("{}", e); }
    };

    for container in containers {
        println!("{:?}", container.Created);
    }
}