use burn::backend::Wgpu;
use burn::backend::wgpu::WgpuDevice;
use burn::tensor::{Data, Shape, Tensor};
use wasm_bindgen_test::wasm_bindgen_test_configure;

wasm_bindgen_test_configure!(run_in_browser);

pub fn create_tensor() -> Tensor<Wgpu, 1> {
    Tensor::from_data(Data::new(vec![1.0], Shape::new([1])), &WgpuDevice::BestAvailable)
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;
    use super::*;

    #[wasm_bindgen_test]
    fn it_works() {
        let result = create_tensor();
        assert_eq!(result.shape().dims, [1]);
    }
}
