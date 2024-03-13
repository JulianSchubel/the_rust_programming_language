mod some_module;
mod other_modules;

fn main() {
    some_module::hello();
    other_modules::some_module::hello()
}
