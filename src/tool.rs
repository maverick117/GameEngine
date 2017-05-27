use cgmath::*;

// pub trait ToArray<S> {
// 	fn getArray(&self) -> [[f32; 4]; 4];
// }

// impl<S> ToArray<S> for Matrix4<S> where S: BaseFloat {
// 	fn getArray(&self) -> [[f32; 4]; 4] {
// 		let result: [[f32; 4]; 4]; 
// 		for i in 0..4 {
//             for j in 0..4 {
//                 result[i][j] = self[i][j] as f32;
//             }
//         }
//         result
// 	}
// }

pub trait ToArray {
	fn getArray(&self) -> [[f32; 4]; 4];
}

impl ToArray for Matrix4<f32> where {
	fn getArray(&self) -> [[f32; 4]; 4] {
		let mut result: [[f32; 4]; 4] = 
		                         [[0.0, 0.0, 0.0, 0.0],
                                 [0.0, 0.0, 0.0, 0.0],
                                 [0.0, 0.0, 0.0, 0.0],
                                 [0.0, 0.0, 0.0, 0.0]];
		for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self[i][j];
            }
        }
        result
	}
}