mod ex_module;

fn main(){
    println!("Multi File Example");
    println!("{:?}", mylib::really_complicated_code(2, 3));

    ex_module::show_ex_module();
}