use std::future::Future;

use renderer::IRuntime;

struct TokioRutime {
    runtime: tokio::runtime::Runtime,
}

impl TokioRutime {
    pub fn new() -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
        Self { runtime }
    }
}

impl IRuntime for TokioRutime {
    fn execute<F>(&self, func: F)
    where
        F: Future<Output = ()> + 'static,
    {
        self.runtime.block_on(func);
    }
}

fn main() {
    let runtime = TokioRutime::new();
    renderer::run(runtime);
}
