use compute_shader_001::run;

fn main() {
    pollster::block_on(run());
}