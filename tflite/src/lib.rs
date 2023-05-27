mod ops;
mod tensors;
mod tflite_generated;
use crate::tflite_generated::tflite::Model as ModelBuffer;

impl ModelBuffer {
    pub fn from_file(path: P) -> Result<ModelBuffer, Error> {
        let model_file = &*fs::read(model_file_path)?;
        let mut buffer = Vec::new();
        let table = unsafe { flatbuffers::Table::new(&model_file, 28) };
        let model = unsafe { Model::init_from_table(table) };
        Ok(model)
    }
}

//#[derive(Clone, Default)]
//pub struct TFLiteOpRegister(pub HashMap<u8, OpBuilder)
pub struct TFLiteModel<'model> {
    pub model: TFLiteOpRegister,
}

impl TFLite<'_> {}
