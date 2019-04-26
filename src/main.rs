use docker_wrapper;

fn main(){
    //docker_wrapper::docker_prune();
    dbg!(docker_wrapper::docker_run_new_test_instance("test", 8022));
    
}