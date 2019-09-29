use mimalloc_rs::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let v = vec![1, 2, 3];
}
