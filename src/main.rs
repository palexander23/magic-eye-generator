use magic_eye_generator::run_window;

fn main() {
    pollster::block_on(run_window());
}
